#[cfg(test)]
mod tests {
    use crate::core::db::{create_pairing_pin, DbConn};
    use crate::core::migrations::run_migrations;
    use crate::core::server::{build_app, AppState};
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
        Router,
    };
    use http_body_util::BodyExt;
    use rusqlite::Connection;
    use serde_json::json;
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt; // for `oneshot` // for `collect` on Body

    // Helper to setup app state and router
    fn setup_app() -> (Router, DbConn) {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let db: DbConn = Arc::new(Mutex::new(conn));
        let attempts = Arc::new(Mutex::new(HashMap::new()));

        let state = AppState {
            db: db.clone(),
            pair_attempts: attempts,
            llm_client: Arc::new(crate::core::llm::OllamaClient),
        };

        let app = build_app(state);
        (app, db)
    }

    fn make_req(
        uri: &str,
        method: axum::http::Method,
        body: Option<serde_json::Value>,
    ) -> Request<Body> {
        let mut req = Request::builder().uri(uri).method(method);

        let b = if let Some(j) = body {
            req = req.header(header::CONTENT_TYPE, "application/json");
            Body::from(serde_json::to_string(&j).unwrap())
        } else {
            Body::empty()
        };

        let mut request = req.body(b).unwrap();
        request
            .extensions_mut()
            .insert(axum::extract::ConnectInfo(SocketAddr::from((
                [127, 0, 0, 1],
                12053,
            ))));
        request
    }

    #[tokio::test]
    async fn test_auth_middleware_missing_origin() {
        let (app, _) = setup_app();
        let mut req = make_req("/api/queue", axum::http::Method::GET, None);
        req.headers_mut()
            .insert(header::HOST, "127.0.0.1:12053".parse().unwrap());

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_auth_middleware_invalid_host() {
        let (app, _) = setup_app();
        let mut req = make_req("/api/queue", axum::http::Method::GET, None);
        req.headers_mut()
            .insert(header::HOST, "invalidhost.com:12053".parse().unwrap());
        req.headers_mut()
            .insert(header::ORIGIN, "chrome-extension://someid".parse().unwrap());

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_pair_and_roundtrip() {
        let (app, db) = setup_app();

        // 1. Create a raw pin and its hash in the db
        use sha2::{Digest, Sha256};
        let raw_pin = "fake-base64-token-16bytes-1234";
        let mut hasher = Sha256::new();
        hasher.update(raw_pin.as_bytes());
        let hashed_pin = hex::encode(hasher.finalize());
        let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(5)).to_rfc3339();

        {
            let conn = db.lock().unwrap();
            create_pairing_pin(&conn, "test-client", &hashed_pin, &expires_at).unwrap();
        }

        // 2. Call /api/pair
        let req = make_req(
            "/api/pair",
            axum::http::Method::POST,
            Some(json!({ "pin": raw_pin })),
        );
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        use axum::body::Bytes;
        let body_bytes: Bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let token = body["token"].as_str().unwrap().to_string();

        // 3. Test roundtrip to protected endpoint
        let mut req2 = make_req("/api/queue", axum::http::Method::GET, None);
        req2.headers_mut()
            .insert(header::HOST, "127.0.0.1:12053".parse().unwrap());
        req2.headers_mut()
            .insert(header::ORIGIN, "chrome-extension://someid".parse().unwrap());
        req2.headers_mut().insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        let res2 = app.clone().oneshot(req2).await.unwrap();
        assert_eq!(res2.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_pair_rate_limit() {
        let (app, _) = setup_app();

        // Try pairing 6 times with bad PINs
        for _ in 0..5 {
            let req = make_req(
                "/api/pair",
                axum::http::Method::POST,
                Some(json!({ "pin": "bad-pin" })),
            );
            let res = app.clone().oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        }

        // 6th time should be rate limited
        let req = make_req(
            "/api/pair",
            axum::http::Method::POST,
            Some(json!({ "pin": "bad-pin" })),
        );
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_revoked_token_rejected() {
        let (app, db) = setup_app();

        use sha2::{Digest, Sha256};
        let raw_pin = "fake-token-revoked";
        let mut hasher = Sha256::new();
        hasher.update(raw_pin.as_bytes());
        let hashed_pin = hex::encode(hasher.finalize());
        let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(5)).to_rfc3339();

        let _token_uuid = {
            let conn = db.lock().unwrap();
            create_pairing_pin(&conn, "revoked-client", &hashed_pin, &expires_at).unwrap()
        };

        // Call /api/pair to pair it
        let req = make_req(
            "/api/pair",
            axum::http::Method::POST,
            Some(json!({ "pin": raw_pin })),
        );
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        use axum::body::Bytes;
        let body_bytes: Bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let token = body["token"].as_str().unwrap().to_string();

        // Revoke the token manually
        {
            let conn = db.lock().unwrap();
            crate::core::db::revoke_paired_client(&conn, 1).unwrap();
        }

        // Test with revoked token
        let mut req2 = make_req("/api/queue", axum::http::Method::GET, None);
        req2.headers_mut()
            .insert(header::HOST, "127.0.0.1:12053".parse().unwrap());
        req2.headers_mut()
            .insert(header::ORIGIN, "chrome-extension://someid".parse().unwrap());
        req2.headers_mut().insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );

        let res2 = app.clone().oneshot(req2).await.unwrap();
        assert_eq!(res2.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_expired_pairing_pin_rejected() {
        let (app, db) = setup_app();

        use sha2::{Digest, Sha256};
        let raw_pin = "fake-token-expired";
        let mut hasher = Sha256::new();
        hasher.update(raw_pin.as_bytes());
        let hashed_pin = hex::encode(hasher.finalize());
        let expires_at = (chrono::Utc::now() - chrono::Duration::minutes(5)).to_rfc3339(); // Past

        {
            let conn = db.lock().unwrap();
            create_pairing_pin(&conn, "expired-client", &hashed_pin, &expires_at).unwrap();
        }

        // Call /api/pair to pair it
        let req = make_req(
            "/api/pair",
            axum::http::Method::POST,
            Some(json!({ "pin": raw_pin })),
        );
        let res = app.clone().oneshot(req).await.unwrap();
        // The confirm_pairing query uses pin_expires_at > current time, so it won't find it.
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
