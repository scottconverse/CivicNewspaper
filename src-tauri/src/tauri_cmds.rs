// src/tauri_cmds.rs
use crate::core::backups;
use crate::core::compiler;
use crate::core::db::{self, DbConn, Draft, EvidenceItem, Lead, PairedClient, Source};
use crate::core::detectors;
use crate::core::discovery::{self, DiscoveredSourceCategory};
use crate::core::guardrails::{self, GuardrailsReport};
use crate::core::llm;
use crate::core::scraper;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityProfile {
    pub site_title: String,
    pub site_subtitle: String,
    pub about_text: String,
    pub ethics_text: String,
    pub how_we_report_text: String,
    pub money_threshold: f64,
    pub watchlist: Vec<String>,
    #[serde(default = "default_city")]
    pub city: String,
    #[serde(default = "default_state")]
    pub state: String,
}

fn default_city() -> String {
    "Brighton".to_string()
}
fn default_state() -> String {
    "CO".to_string()
}

/// Normalize an Ollama model tag for EXACT comparison. Ollama treats an untagged
/// name as `:latest`, so `phi3` and `phi3:latest` are the same model. QA-mn1:
/// match exact tags (with this `:latest` normalization) instead of loose
/// substring/`starts_with`/`contains` matching, which could select the wrong
/// model (e.g. `phi3:mini` selected vs `phi3:medium` installed).
pub(crate) fn normalize_model_tag(tag: &str) -> String {
    let t = tag.trim();
    if t.contains(':') {
        t.to_string()
    } else {
        format!("{}:latest", t)
    }
}

/// True if `selected` exactly matches one of the `installed` tags (after
/// `:latest` normalization on both sides). QA-mn1.
pub(crate) fn model_is_installed(selected: &str, installed: &[String]) -> bool {
    let want = normalize_model_tag(selected);
    installed
        .iter()
        .any(|m| normalize_model_tag(m) == want)
}

pub(crate) async fn get_selected_model_or_fallback(db: &DbConn) -> String {
    let saved = {
        if let Ok(conn) = db.lock() {
            if let Ok(mut stmt) =
                conn.prepare("SELECT value FROM settings WHERE key = 'model.selected'")
            {
                stmt.query_row([], |row| row.get::<_, String>(0)).ok()
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(m) = saved {
        return m;
    }

    let default_m = "phi3:mini".to_string();
    let mut model = default_m.clone();
    if let Ok(resp) = reqwest::get("http://127.0.0.1:11434/api/tags").await {
        if resp.status().is_success() {
            #[derive(Deserialize)]
            struct ModelItem {
                name: String,
            }
            #[derive(Deserialize)]
            struct TagsResp {
                models: Vec<ModelItem>,
            }
            if let Ok(tags) = resp.json::<TagsResp>().await {
                if !tags.models.is_empty() {
                    let names: Vec<String> =
                        tags.models.iter().map(|m| m.name.clone()).collect();
                    // QA-mn1: prefer the default model only if it is EXACTLY
                    // installed (with :latest normalization), then fall back to
                    // any preferred-family exact-ish tag, then the first model.
                    if model_is_installed(&default_m, &names) {
                        model = default_m;
                    } else if let Some(m) = tags.models.iter().find(|m| {
                        // Match by model FAMILY on the tag's base name (the part
                        // before ':'), not a loose whole-string contains.
                        let base = m.name.split(':').next().unwrap_or("");
                        base.starts_with("gemma")
                            || base.starts_with("llama")
                            || base.starts_with("phi")
                    }) {
                        model = m.name.clone();
                    } else {
                        model = tags.models[0].name.clone();
                    }
                }
            }
        }
    }
    model
}

/// Fetch the list of installed model tags from the local Ollama. Returns an
/// empty vec if Ollama is unreachable or returns no models.
pub(crate) async fn list_installed_models() -> Vec<String> {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
    {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    if let Ok(resp) = client.get("http://127.0.0.1:11434/api/tags").send().await {
        if resp.status().is_success() {
            #[derive(Deserialize)]
            struct ModelItem {
                name: String,
            }
            #[derive(Deserialize)]
            struct TagsResp {
                models: Vec<ModelItem>,
            }
            if let Ok(tags) = resp.json::<TagsResp>().await {
                return tags.models.into_iter().map(|m| m.name).collect();
            }
        }
    }
    Vec::new()
}

#[derive(Serialize, Deserialize)]
pub struct QueueData {
    pub leads: Vec<Lead>,
    pub drafts: Vec<Draft>,
}

fn get_config_path(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.join("community_profile.json"))
}

#[tauri::command]
pub fn get_sources(db: tauri::State<'_, DbConn>) -> Result<Vec<Source>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::list_sources(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_source(
    db: tauri::State<'_, DbConn>,
    name: String,
    url: String,
    r#type: String,
    tier: String,
) -> Result<i32, String> {
    if tier != "official_record" && tier != "news_reporting" && tier != "community_signal" {
        return Err("Invalid tier".to_string());
    }
    // SSRF defense-in-depth: reject non-http(s) schemes and blocked-IP literals
    // at the storage boundary so a discovered/auto-imported URL can never point
    // the scraper at loopback/private/link-local hosts. DNS-based hosts are
    // re-validated (with resolution) at scrape time.
    scraper::validate_source_url(&url)?;
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let source = Source {
        id: None,
        name,
        url,
        r#type,
        status: "online".to_string(),
        tier,
        last_success_at: None,
        last_failed_at: None,
        last_scraped: None,
    };
    db::insert_source(&conn, &source).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_source(db: tauri::State<'_, DbConn>, id: i32) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::delete_source(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_pairing_pin(db: tauri::State<'_, DbConn>, label: String) -> Result<String, String> {
    use base64::Engine;
    use rand::RngCore;
    use sha2::{Digest, Sha256};

    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;

    let mut bytes = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let raw_pin = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

    let mut hasher = Sha256::new();
    hasher.update(raw_pin.as_bytes());
    let hashed_pin = hex::encode(hasher.finalize());

    let expires_at = (chrono::Utc::now() + chrono::Duration::minutes(5)).to_rfc3339();
    db::create_pairing_pin(&conn, &label, &hashed_pin, &expires_at).map_err(|e| e.to_string())?;

    Ok(raw_pin)
}

#[tauri::command]
pub fn list_paired_clients(db: tauri::State<'_, DbConn>) -> Result<Vec<PairedClient>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::list_paired_clients(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn revoke_pairing(db: tauri::State<'_, DbConn>, id: i32) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::revoke_paired_client(&conn, id).map_err(|e| e.to_string())
}

#[allow(dead_code)]
#[tauri::command]
pub fn list_daily_scan_leads(
    db: tauri::State<'_, DbConn>,
    scan_id: i32,
) -> Result<Vec<crate::core::db::DailyScanLead>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::list_daily_scan_leads(&conn, scan_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_community_profile(app: tauri::AppHandle) -> Result<CommunityProfile, String> {
    let path = get_config_path(&app)?;
    if !path.exists() {
        return Ok(CommunityProfile {
            site_title: "CivicNews Observer".to_string(),
            site_subtitle: "Transparent Local Public Records & Evidence".to_string(),
            about_text: "We report on local government activities using raw public records."
                .to_string(),
            ethics_text: "Our ethics standard: transparent evidence, not outrage or rumors."
                .to_string(),
            how_we_report_text:
                "We monitor municipal feeds and index agendas, minutes, and documents.".to_string(),
            money_threshold: 250000.0,
            watchlist: Vec::new(),
            city: "Brighton".to_string(),
            state: "CO".to_string(),
        });
    }
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_community_profile(
    app: tauri::AppHandle,
    profile: CommunityProfile,
) -> Result<(), String> {
    let path = get_config_path(&app)?;
    let content = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ingest(db: tauri::State<'_, DbConn>, app: tauri::AppHandle) -> Result<usize, String> {
    scraper::scrape_all_sources(&db)
        .await
        .map_err(|e| e.to_string())?;

    let unlinked_ids = {
        let conn = db
            .lock()
            .map_err(|_| "Failed to lock database".to_string())?;
        let mut stmt = conn.prepare("SELECT id FROM evidence_items WHERE id NOT IN (SELECT evidence_id FROM lead_evidence)")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, i32>(0))
            .map_err(|e| e.to_string())?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id.map_err(|e| e.to_string())?);
        }
        ids
    };

    let profile_json = {
        let path = get_config_path(&app)?;
        if path.exists() {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            r#"{"money_threshold": 250000.0, "watchlist": []}"#.to_string()
        }
    };

    let new_lead_ids = {
        let conn = db
            .lock()
            .map_err(|_| "Failed to lock database".to_string())?;
        detectors::run_detectors(&conn, &unlinked_ids, &profile_json).map_err(|e| e.to_string())?
    };

    Ok(new_lead_ids.len())
}

#[tauri::command]
pub fn get_queue(db: tauri::State<'_, DbConn>) -> Result<QueueData, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let leads = db::list_leads(&conn).map_err(|e| e.to_string())?;
    let drafts = db::list_drafts(&conn).map_err(|e| e.to_string())?;
    Ok(QueueData { leads, drafts })
}

#[tauri::command]
pub fn get_evidence(
    db: tauri::State<'_, DbConn>,
    lead_id: i32,
) -> Result<Vec<EvidenceItem>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::get_evidence_by_lead(&conn, lead_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_draft(db: tauri::State<'_, DbConn>, draft: Draft) -> Result<i32, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    if let Some(id) = draft.id {
        db::update_draft(&conn, &draft).map_err(|e| e.to_string())?;
        Ok(id)
    } else {
        db::insert_draft(&conn, &draft).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn delete_draft(db: tauri::State<'_, DbConn>, id: i32) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::delete_draft(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn story_decision(
    db: tauri::State<'_, DbConn>,
    id: i32,
    decision: String,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::update_draft_status(&conn, id, &decision).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_draft<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, DbConn>,
    lead_id: i32,
    format: String,
    system_prompt: Option<String>,
) -> Result<String, String> {
    let (lead_why, evidence_items) = {
        let conn = db
            .lock()
            .map_err(|_| "Failed to lock database".to_string())?;
        let mut stmt = conn
            .prepare("SELECT why FROM leads WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        let why: String = stmt
            .query_row([lead_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        let items = db::get_evidence_by_lead(&conn, lead_id).map_err(|e| e.to_string())?;
        (why, items)
    };

    if evidence_items.is_empty() {
        return Err("No evidence items linked to this lead.".to_string());
    }

    let mut evidence_context = String::new();
    for item in &evidence_items {
        let item_id = item.id.unwrap_or(0);
        evidence_context.push_str(&format!(
            "Evidence Citation ID: {}\nExcerpt: {}\n\n",
            item_id, item.excerpt
        ));
    }

    let prompt = format!(
        "Lead topic: {}\n\nHere are the raw public records evidence:\n{}\nPlease draft a report in '{}' format. You MUST use 'evidence:ID' citations inside the text (like [Source](evidence:ID)) when claiming a fact from the records. Keep it objective, professional, and do not make assumptions beyond the text.",
        lead_why, evidence_context, format
    );

    let sys = system_prompt.unwrap_or_else(|| "You are an AI assistant for a local community newspaper reporter. You write factual, objective drafts based strictly on provided records. You always cite evidence IDs.".to_string());

    let model = get_selected_model_or_fallback(&db).await;

    // QA-C1: pre-flight that the resolved model is actually installed. The
    // frontend gates the button on sidecar-reachability only, so a user who
    // skipped the model download in onboarding could reach this with no model.
    // Fail with a clear, typed remedy instead of an opaque "model not found"
    // surfaced from Ollama mid-call. (The frontend will also pre-flight.)
    let installed = list_installed_models().await;
    if !installed.is_empty() && !model_is_installed(&model, &installed) {
        return Err(format!(
            "MODEL_NOT_INSTALLED: The selected AI model '{}' is not installed. Open Setup and download a model before drafting.",
            model
        ));
    }

    let llm_client = app
        .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
        .inner()
        .clone();
    llm_client.call(&model, &prompt, &sys).await
}

#[tauri::command]
pub async fn llm_task<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, DbConn>,
    prompt: String,
    system: String,
) -> Result<String, String> {
    let model = get_selected_model_or_fallback(&db).await;
    let llm_client = app
        .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
        .inner()
        .clone();
    llm_client.call(&model, &prompt, &system).await
}

#[tauri::command]
pub fn guardrails_check(
    db: tauri::State<'_, DbConn>,
    draft_id: i32,
) -> Result<GuardrailsReport, String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    guardrails::run_guardrails_check(&conn, draft_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn publish(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    output_dir: String,
) -> Result<(), String> {
    let profile_json = {
        let path = get_config_path(&app)?;
        if path.exists() {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            r#"{"site_title": "CivicNews Observer", "site_subtitle": "Transparent Local Public Records", "about_text": "", "ethics_text": "", "how_we_report_text": ""}"#.to_string()
        }
    };

    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    compiler::compile_static_site(&conn, &output_dir, &profile_json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn register_correction(
    db: tauri::State<'_, DbConn>,
    draft_id: i32,
    correction_note: String,
) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let mut draft = db::get_draft(&conn, draft_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Draft not found".to_string())?;

    draft.status = "corrected".to_string();
    draft.correction_note = Some(correction_note);
    db::update_draft(&conn, &draft).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn backup_save(db: tauri::State<'_, DbConn>, dest_path: String) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    backups::save_backup(&conn, &dest_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backup_restore(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    backup_path: String,
) -> Result<(), String> {
    let live_db_path = db::get_app_db_path(&app).map_err(|e| e.to_string())?;
    let live_db_path_str = live_db_path
        .to_str()
        .ok_or_else(|| "Invalid database path".to_string())?;
    backups::restore_backup(&db, &backup_path, live_db_path_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_ollama() -> bool {
    llm::check_ollama_status().await
}

#[tauri::command]
pub fn cancel_ollama_pull(model: String) -> Result<(), String> {
    crate::core::llm::cancel_pull(&model);
    Ok(())
}

/// Bridges core pull progress to Tauri's event emitter. Keeps the wire format
/// the frontend expects: `ollama-pull-progress` carries `{model,status,completed,total}`,
/// `ollama-pull-complete` carries null, `ollama-pull-error` carries a string.
struct AppHandlePullSink<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> crate::core::llm::PullProgressSink for AppHandlePullSink<R> {
    fn progress(&self, payload: crate::core::llm::PullProgress) {
        use tauri::Emitter;
        let _ = self.app.emit("ollama-pull-progress", payload);
    }
    fn complete(&self) {
        use tauri::Emitter;
        let _ = self.app.emit("ollama-pull-complete", ());
    }
    fn error(&self, message: String) {
        use tauri::Emitter;
        let _ = self.app.emit("ollama-pull-error", message);
    }
}

#[tauri::command]
pub async fn pull_ollama_model<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    model_id: String,
) -> Result<(), String> {
    let sink = std::sync::Arc::new(AppHandlePullSink { app });
    crate::core::llm::run_ollama_pull(model_id, "http://127.0.0.1:11434", sink).await
}

#[tauri::command]
pub fn get_system_ram() -> Result<u64, String> {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_ram_bytes = sys.total_memory();
    let total_ram_gb = total_ram_bytes / (1024 * 1024 * 1024);
    Ok(total_ram_gb)
}

#[tauri::command]
pub async fn discover_sources(
    city: String,
    state: String,
) -> Result<Vec<DiscoveredSourceCategory>, String> {
    discovery::discover_all_sources(&city, &state)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct OllamaState {
    pub reachable: bool,
    pub models: Vec<String>,
    pub version: Option<String>,
}

#[tauri::command]
pub async fn ollama_health() -> Result<OllamaState, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    match client.get("http://127.0.0.1:11434/api/tags").send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                #[derive(serde::Deserialize)]
                struct ModelItem {
                    name: String,
                }
                #[derive(serde::Deserialize)]
                struct TagsResp {
                    models: Vec<ModelItem>,
                }

                let mut version = None;
                if let Ok(v_resp) = client
                    .get("http://127.0.0.1:11434/api/version")
                    .send()
                    .await
                {
                    #[derive(serde::Deserialize)]
                    struct VersionResp {
                        version: String,
                    }
                    if let Ok(v) = v_resp.json::<VersionResp>().await {
                        version = Some(v.version);
                    }
                }

                if let Ok(tags) = resp.json::<TagsResp>().await {
                    let models = tags.models.into_iter().map(|m| m.name).collect();
                    Ok(OllamaState {
                        reachable: true,
                        models,
                        version,
                    })
                } else {
                    Ok(OllamaState {
                        reachable: true,
                        models: vec![],
                        version,
                    })
                }
            } else {
                Ok(OllamaState {
                    reachable: true,
                    models: vec![],
                    version: None,
                })
            }
        }
        Err(_) => Ok(OllamaState {
            reachable: false,
            models: vec![],
            version: None,
        }),
    }
}

#[tauri::command]
pub fn is_onboarding_complete(db: tauri::State<'_, DbConn>) -> Result<bool, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = 'onboarding_complete'")
        .map_err(|e| e.to_string())?;
    let val: Result<String, _> = stmt.query_row([], |row| row.get(0));
    match val {
        Ok(v) => Ok(v == "1"),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
pub fn set_onboarding_complete(db: tauri::State<'_, DbConn>, value: bool) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let val_str = if value { "1" } else { "0" };
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('onboarding_complete', ?1)",
        [val_str],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn set_setting(db: tauri::State<'_, DbConn>, key: String, value: String) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        [&key, &value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_setting(db: tauri::State<'_, DbConn>, key: String) -> Result<Option<String>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = ?1")
        .map_err(|e| e.to_string())?;
    let val: Result<String, _> = stmt.query_row([key], |row| row.get(0));
    match val {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn validate_export_path(
    app_data_dir: std::path::PathBuf,
    download_dir: std::path::PathBuf,
    path: &str,
) -> Result<std::path::PathBuf, String> {
    let requested = std::path::Path::new(path);
    let parent = requested
        .parent()
        .ok_or_else(|| "Invalid path".to_string())?;

    let canonical_parent =
        std::fs::canonicalize(parent).map_err(|e| format!("Invalid path: {}", e))?;

    let canonical_app_data = std::fs::canonicalize(&app_data_dir).unwrap_or(app_data_dir);
    let canonical_download = std::fs::canonicalize(&download_dir).unwrap_or(download_dir);

    if canonical_parent.starts_with(&canonical_app_data)
        || canonical_parent.starts_with(&canonical_download)
    {
        Ok(canonical_parent.join(
            requested
                .file_name()
                .ok_or_else(|| "No file name".to_string())?,
        ))
    } else {
        Err("Path is outside allowed directories".to_string())
    }
}

pub async fn export_diagnostics_inner(
    db: &DbConn,
    app_data: std::path::PathBuf,
    validated_path: std::path::PathBuf,
) -> Result<(), String> {
    let diags = crate::core::diagnostics::gather_diagnostics(db, app_data).await?;
    let json = serde_json::to_string_pretty(&diags).map_err(|e| e.to_string())?;
    std::fs::write(validated_path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn export_diagnostics(
    db: tauri::State<'_, DbConn>,
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    let app_data = app_handle.path().app_data_dir().unwrap_or_default();
    let download = app_handle.path().download_dir().unwrap_or_default();
    let validated = validate_export_path(app_data.clone(), download, &path)?;
    export_diagnostics_inner(&db, app_data, validated).await
}

#[tauri::command]
pub fn list_prompts() -> Vec<String> {
    crate::core::prompts::list_prompts()
}

#[tauri::command]
pub fn get_prompt(app: tauri::AppHandle, id: String) -> Result<String, String> {
    crate::core::prompts::get_prompt(&app, &id)
}

#[tauri::command]
pub async fn run_daily_scan<R: tauri::Runtime>(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle<R>,
    city: String,
    state: String,
    since_hours: u32,
) -> Result<i32, String> {
    let prompt_template = crate::core::prompts::get_prompt(&app, "aggregator")?;
    let llm_client = app
        .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
        .inner()
        .clone();
    crate::core::daily_scan::run_daily_scan(
        &db,
        &llm_client,
        &prompt_template,
        &city,
        &state,
        since_hours,
    )
    .await
}

#[tauri::command]
pub async fn plain_language_rewrite<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, DbConn>,
    text: String,
    draft_format: String,
) -> Result<String, String> {
    let model = get_selected_model_or_fallback(&db).await;
    let llm_client = app
        .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
        .inner()
        .clone();
    crate::core::llm::plain_language_rewrite(&llm_client, &model, &text, &draft_format).await
}
