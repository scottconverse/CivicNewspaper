// core/auth.rs
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use super::db::{get_paired_client_by_token, record_paired_client_use};

// Validate the Host header to defeat DNS rebinding attacks
pub fn is_valid_host(host: &str) -> bool {
    let host_clean = host.trim().to_lowercase();
    host_clean == "127.0.0.1:12053" || host_clean == "localhost:12053"
}

// Validate the Origin header to ensure it matches browser extension origins or is absent (for IDE agents)
pub fn is_valid_origin(origin: &str) -> bool {
    let origin_clean = origin.trim().to_lowercase();
    // Allow extensions (chrome-extension://... or safari-extension://...)
    origin_clean.starts_with("chrome-extension://") || origin_clean.starts_with("safari-extension://") || origin_clean == "null"
}

use axum::extract::State;

// Middleware for Axum HTTP routes
pub async fn auth_middleware(
    State(db): State<Arc<Mutex<Connection>>>,
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
        println!("Auth Block: Invalid Host header: '{}'", host);
        return Err(StatusCode::BAD_REQUEST);
    }

    // 2. Origin Validation (if Origin header exists)
    if let Some(origin_header) = request.headers().get(header::ORIGIN) {
        if let Ok(origin_str) = origin_header.to_str() {
            if !is_valid_origin(origin_str) {
                println!("Auth Block: Untrusted Origin header: '{}'", origin_str);
                return Err(StatusCode::FORBIDDEN);
            }
        }
    }

    // 3. Skip token check for pairing route
    let path = request.uri().path();
    if path == "/api/pair" {
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
            println!("Auth Block: Missing or malformed Bearer token");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Check token in DB
    let is_valid = {
        let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
        println!("Auth Block: Token not found or revoked");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
