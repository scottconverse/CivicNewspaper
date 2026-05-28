// core/server.rs
use super::db::DbConn;
use super::guardrails::run_guardrails_check;
// Removed call_local_ollama
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
    pub pair_attempts: Arc<Mutex<HashMap<String, (usize, Instant)>>>,
    pub llm_client: Arc<dyn crate::core::llm::LlmClient>,
}

// JSON request/response models
#[derive(Deserialize)]
struct PairRequest {
    pin: String,
}

#[derive(Serialize)]
struct PairResponse {
    token: String,
}

#[derive(Serialize)]
struct QueueResponse {
    leads: Vec<super::db::Lead>,
    drafts: Vec<super::db::Draft>,
}

#[derive(Deserialize)]
struct CreateDraftRequest {
    lead_id: Option<i32>,
    format: String,
    title: String,
    content: String,
    verification_checklist: Option<String>,
}

#[derive(Serialize)]
struct CreateDraftResponse {
    id: i32,
}

#[derive(Deserialize)]
struct LlmTaskRequest {
    prompt: String,
    system: String,
}

#[derive(Serialize)]
struct LlmTaskResponse {
    result: String,
}

#[derive(Deserialize)]
struct GuardrailsRequest {
    draft_id: i32,
}

pub fn build_app(app_state: AppState) -> axum::Router {
    let api_routes = Router::new()
        .route("/queue", get(get_queue_handler))
        .route("/evidence/:lead_id", get(get_evidence_handler))
        .route("/drafts", post(create_draft_handler))
        .route("/llm/task", post(llm_task_handler))
        .route("/guardrails/check", post(guardrails_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            super::auth::auth_middleware,
        ));

    Router::new()
        .route("/api/pair", post(pair_handler))
        .nest("/api", api_routes)
        .with_state(app_state)
}

pub async fn start_server(db: DbConn) -> Result<(), Box<dyn Error + Send + Sync>> {
    let app_state = AppState {
        db: db.clone(),
        pair_attempts: Arc::new(Mutex::new(HashMap::new())),
        llm_client: Arc::new(crate::core::llm::OllamaClient),
    };

    let app = build_app(app_state);

    // Bind strictly to loopback interface 127.0.0.1
    let addr = SocketAddr::from(([127, 0, 0, 1], 12053));
    println!("Core API server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

// Handlers

async fn pair_handler(
    State(state): State<AppState>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,
    Json(payload): Json<PairRequest>,
) -> Result<Json<PairResponse>, StatusCode> {
    let ip = addr.ip().to_string();
    {
        let mut attempts = state.pair_attempts.lock().unwrap();
        if let Some(&(count, time)) = attempts.get(&ip) {
            if count >= 5 && time.elapsed().as_secs() < 1800 {
                return Err(StatusCode::TOO_MANY_REQUESTS);
            } else if time.elapsed().as_secs() >= 1800 {
                attempts.remove(&ip);
            }
        }
    }

    let conn = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut hasher = Sha256::new();
    hasher.update(payload.pin.as_bytes());
    let hashed_pin = hex::encode(hasher.finalize());

    match super::db::confirm_pairing(&conn, &hashed_pin) {
        Ok(Some(token)) => {
            let mut attempts = state.pair_attempts.lock().unwrap();
            attempts.remove(&ip);
            Ok(Json(PairResponse { token }))
        }
        Ok(None) => {
            let mut attempts = state.pair_attempts.lock().unwrap();
            let entry = attempts.entry(ip).or_insert((0, Instant::now()));
            entry.0 += 1;
            entry.1 = Instant::now();
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_queue_handler(
    State(state): State<AppState>,
) -> Result<Json<QueueResponse>, StatusCode> {
    let conn = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let leads = super::db::list_leads(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let drafts = super::db::list_drafts(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QueueResponse { leads, drafts }))
}

async fn get_evidence_handler(
    State(state): State<AppState>,
    Path(lead_id): Path<i32>,
) -> Result<Json<Vec<super::db::EvidenceItem>>, StatusCode> {
    let conn = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items = super::db::get_evidence_by_lead(&conn, lead_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(items))
}

async fn create_draft_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDraftRequest>,
) -> Result<Json<CreateDraftResponse>, StatusCode> {
    let conn = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let now = chrono::Utc::now().to_rfc3339();
    let draft = super::db::Draft {
        id: None,
        lead_id: payload.lead_id,
        format: payload.format,
        title: payload.title,
        content: payload.content,
        status: "draft_generated".to_string(), // Forced to draft state
        verification_checklist: payload
            .verification_checklist
            .unwrap_or_else(|| "[]".to_string()),
        missing_evidence_notes: None,
        correction_note: None,
        created_at: now.clone(),
        updated_at: now,
    };

    match super::db::insert_draft(&conn, &draft) {
        Ok(id) => Ok(Json(CreateDraftResponse { id })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn llm_task_handler(
    State(state): State<AppState>,
    Json(payload): Json<LlmTaskRequest>,
) -> Result<Json<LlmTaskResponse>, StatusCode> {
    let model = {
        let conn = state
            .db
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut stmt = conn
            .prepare("SELECT value FROM settings WHERE key = 'model.selected'")
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let val: Result<String, _> = stmt.query_row([], |row| row.get(0));
        match val {
            Ok(v) => v,
            Err(_) => "phi3:mini".to_string(),
        }
    };

    match state
        .llm_client
        .call(&model, &payload.prompt, &payload.system)
        .await
    {
        Ok(result) => Ok(Json(LlmTaskResponse { result })),
        Err(e) => {
            eprintln!("Ollama task execution failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn guardrails_handler(
    State(state): State<AppState>,
    Json(payload): Json<GuardrailsRequest>,
) -> Result<Json<super::guardrails::GuardrailsReport>, StatusCode> {
    let conn = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match run_guardrails_check(&conn, payload.draft_id) {
        Ok(report) => Ok(Json(report)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
