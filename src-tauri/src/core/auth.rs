use super::db::{get_paired_client_by_token, record_paired_client_use};
use super::server::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

// Validate the Host header to defeat DNS rebinding attacks.
pub fn is_valid_host(host: &str) -> bool {
    host.trim() == "127.0.0.1:12053"
}

// Validate browser-extension origins. "null" is never trusted.
pub fn is_valid_origin(origin: &str) -> bool {
    let origin_clean = origin.trim().to_lowercase();
    if origin_clean == "null" {
        return false;
    }
    origin_clean.starts_with("chrome-extension://")
}

fn validate_host_and_origin(request: &Request<Body>) -> Result<(), StatusCode> {
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

    match request.headers().get(header::ORIGIN) {
        Some(origin_header) => match origin_header.to_str() {
            Ok(origin_str) if is_valid_origin(origin_str) => Ok(()),
            Ok(origin_str) => {
                if cfg!(debug_assertions) {
                    eprintln!("Auth Block: Untrusted Origin header: '{}'", origin_str);
                }
                Err(StatusCode::FORBIDDEN)
            }
            Err(_) => {
                if cfg!(debug_assertions) {
                    eprintln!("Auth Block: Unparseable Origin header");
                }
                Err(StatusCode::FORBIDDEN)
            }
        },
        None => {
            let is_pair_route = request.uri().path().ends_with("/pair");
            if is_pair_route && !request.headers().contains_key("x-civicnews-pair") {
                if cfg!(debug_assertions) {
                    eprintln!("Auth Block: Pair route missing Origin or x-civicnews-pair");
                }
                return Err(StatusCode::FORBIDDEN);
            }
            Ok(())
        }
    }
}

// Host/Origin boundary for the pairing route. /api/pair intentionally has no
// bearer-token check because it is where clients exchange the user-visible PIN.
pub async fn host_origin_middleware(
    State(_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    validate_host_and_origin(&request)?;
    Ok(next.run(request).await)
}

// Middleware for authenticated API routes.
pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // A present Origin must parse and be on the allowlist. A missing Origin is
    // allowed for non-browser local callers only because every protected route
    // still has to pass this bearer-token check.
    validate_host_and_origin(&request)?;

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

    let is_valid = {
        let conn = state
            .db
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        match get_paired_client_by_token(&conn, token) {
            Ok(Some(_client)) => {
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
