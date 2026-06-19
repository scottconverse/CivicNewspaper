// core/auth.rs
use super::db::{get_paired_client_by_token, record_paired_client_use};
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

// Validate the Host header to defeat DNS rebinding attacks
pub fn is_valid_host(host: &str) -> bool {
    host.trim() == "127.0.0.1:12053"
}

// Validate the Origin header to ensure it matches browser extension origins or is absent (for IDE agents)
pub fn is_valid_origin(origin: &str) -> bool {
    let origin_clean = origin.trim().to_lowercase();
    if origin_clean == "null" {
        return false;
    }
    origin_clean.starts_with("chrome-extension://")
}

use super::server::AppState;
use axum::extract::State;

// Middleware for Axum HTTP routes
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Host Validation
    let host = request
        .headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if !is_valid_host(host) {
        if cfg!(debug_assertions) {
            eprintln!("Auth Block: Invalid Host header: '{}'", host);
        }
        return Err(StatusCode::FORBIDDEN);
    }

    // 2. Origin Validation
    //
    // ENG-M1: a *present* Origin must be on the allowlist (browser-extension
    // origins). A *missing* Origin is NOT treated as trusted: it is allowed to
    // proceed past this step ONLY for the explicit no-origin caller classes
    // below, and every non-pair route still has to pass the mandatory bearer
    // token check in step 4 — token possession is what authorizes a no-origin
    // local client. The classic weakening this guards against is "skip the
    // whole gate when the header is absent"; here the token gate is unconditional
    // for non-pair routes, so an absent Origin can never bypass authorization.
    let path = request.uri().path();
    let is_pair_route = path == "/api/pair";
    // The pairing route may opt out of the Origin allowlist via an explicit
    // pairing header (the IDE/extension pairing handshake doesn't carry a
    // chrome-extension origin yet). This is the only sanctioned no-origin bypass
    // of the allowlist, and /api/pair has its own per-IP rate limit + PIN check.
    let skip_origin = is_pair_route && request.headers().contains_key("x-civicnews-pair");

    if !skip_origin {
        match request.headers().get(header::ORIGIN) {
            Some(origin_header) => {
                // A present Origin must parse and be on the allowlist.
                match origin_header.to_str() {
                    Ok(origin_str) if is_valid_origin(origin_str) => {}
                    Ok(origin_str) => {
                        if cfg!(debug_assertions) {
                            eprintln!("Auth Block: Untrusted Origin header: '{}'", origin_str);
                        }
                        return Err(StatusCode::FORBIDDEN);
                    }
                    Err(_) => {
                        if cfg!(debug_assertions) {
                            eprintln!("Auth Block: Unparseable Origin header");
                        }
                        return Err(StatusCode::FORBIDDEN);
                    }
                }
            }
            None => {
                // No Origin header. We do NOT short-circuit-allow here; the
                // request must still satisfy the bearer-token check below
                // (step 4). Non-browser local callers (curl, IDE agents) are
                // expected to authenticate by token, not by Origin.
            }
        }
    }

    // 3. Skip token check for pairing route
    if is_pair_route {
        return Ok(next.run(request).await);
    }

    // 4. Paired Token Authorization Check
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|a| a.to_str().ok());

    let token = match auth_header {
        Some(auth_val) if auth_val.starts_with("Bearer ") => {
            auth_val.trim_start_matches("Bearer ").trim()
        }
        _ => {
            if cfg!(debug_assertions) {
                eprintln!("Auth Block: Missing or malformed Bearer token");
            }
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Check token in DB
    let is_valid = {
        let conn = state
            .db
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        match get_paired_client_by_token(&conn, token) {
            Ok(Some(_client)) => {
                // Update last used timestamp
                let _ = record_paired_client_use(&conn, token);
                true
            }
            _ => false,
        }
    };

    if !is_valid {
        if cfg!(debug_assertions) {
            eprintln!("Auth Block: Token not found or revoked");
        }
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
