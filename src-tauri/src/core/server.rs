// core/server.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use super::db::{DbConn, confirm_pairing, list_leads, list_drafts, get_evidence_by_lead, insert_draft, Draft};
use super::guardrails::run_guardrails_check;
use super::llm::call_local_ollama;

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

pub async fn start_server(db: DbConn) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Shared state between routes
    let state_db = db.clone();

    // Routes requiring token authentication
    let api_routes = Router::new()
        .route("/queue", get(get_queue_handler))
        .route("/evidence/:lead_id", get(get_evidence_handler))
        .route("/drafts", post(create_draft_handler))
        .route("/llm/task", post(llm_task_handler))
        .route("/guardrails/check", post(guardrails_handler))
        .layer(middleware::from_fn_with_state(state_db.clone(), super::auth::auth_middleware));

    // Outer app router (includes pairing route which doesn't check tokens)
    let app = Router::new()
        .route("/api/pair", post(pair_handler))
        .nest("/api", api_routes)
        .with_state(state_db);

    // Bind strictly to loopback interface 127.0.0.1
    let addr = SocketAddr::from(([127, 0, 0, 1], 12053));
    println!("Core API server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Handlers

async fn pair_handler(
    State(db): State<DbConn>,
    Json(payload): Json<PairRequest>,
) -> Result<Json<PairResponse>, StatusCode> {
    let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match confirm_pairing(&conn, &payload.pin) {
        Ok(Some(token)) => Ok(Json(PairResponse { token })),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_queue_handler(
    State(db): State<DbConn>,
) -> Result<Json<QueueResponse>, StatusCode> {
    let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let leads = list_leads(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let drafts = list_drafts(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(QueueResponse { leads, drafts }))
}

async fn get_evidence_handler(
    State(db): State<DbConn>,
    Path(lead_id): Path<i32>,
) -> Result<Json<Vec<super::db::EvidenceItem>>, StatusCode> {
    let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let items = get_evidence_by_lead(&conn, lead_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(items))
}

async fn create_draft_handler(
    State(db): State<DbConn>,
    Json(payload): Json<CreateDraftRequest>,
) -> Result<Json<CreateDraftResponse>, StatusCode> {
    let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let now = chrono::Utc::now().to_rfc3339();
    let draft = Draft {
        id: None,
        lead_id: payload.lead_id,
        format: payload.format,
        title: payload.title,
        content: payload.content,
        status: "draft_generated".to_string(), // Forced to draft state
        verification_checklist: payload.verification_checklist.unwrap_or_else(|| "[]".to_string()),
        missing_evidence_notes: None,
        correction_note: None,
        created_at: now.clone(),
        updated_at: now,
    };
    
    match insert_draft(&conn, &draft) {
        Ok(id) => Ok(Json(CreateDraftResponse { id })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn llm_task_handler(
    Json(payload): Json<LlmTaskRequest>,
) -> Result<Json<LlmTaskResponse>, StatusCode> {
    // Load default model for tasks. Default to gemma2:9b as per spec
    let model = "gemma2:9b";
    
    match call_local_ollama(model, &payload.prompt, &payload.system).await {
        Ok(result) => Ok(Json(LlmTaskResponse { result })),
        Err(e) => {
            eprintln!("Ollama task execution failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn guardrails_handler(
    State(db): State<DbConn>,
    Json(payload): Json<GuardrailsRequest>,
) -> Result<Json<super::guardrails::GuardrailsReport>, StatusCode> {
    let conn = db.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match run_guardrails_check(&conn, payload.draft_id) {
        Ok(report) => Ok(Json(report)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
