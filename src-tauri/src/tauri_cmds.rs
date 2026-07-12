// src/tauri_cmds.rs
use crate::core::backups;
use crate::core::compiler;
use crate::core::db::{self, DbConn, Draft, EvidenceItem, Lead, PairedClient, Source, Subscriber};
use crate::core::detectors;
use crate::core::discovery::{self, DiscoveredSourceCategory};
use crate::core::guardrails::{self, GuardrailsReport};
use crate::core::intelligence::{self, CivicIntelligenceSnapshot, DarkSignal};
use crate::core::llm;
use crate::core::publisher;
use crate::core::scraper;
use crate::core::source_grounding;
use crate::core::verification::{self, VerificationQueueSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;
use tauri::Emitter;
use tauri::Manager;

const SOURCE_IMPORT_MAX_FILE_BYTES: u64 = 25 * 1024 * 1024;
const SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES: u64 = 5 * 1024 * 1024;
const SOURCE_IMPORT_MAX_ZIP_TOTAL_BYTES: u64 = 12 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityProfile {
    pub site_title: String,
    pub site_subtitle: String,
    pub about_text: String,
    pub ethics_text: String,
    pub how_we_report_text: String,
    #[serde(default = "default_organization_type")]
    pub organization_type: String,
    #[serde(default)]
    pub footer_text: String,
    #[serde(default)]
    pub logo_url: String,
    #[serde(default = "default_accent_color")]
    pub accent_color: String,
    #[serde(default = "default_layout_style")]
    pub layout_style: String,
    #[serde(default = "default_first_amendment_advisor_enabled")]
    pub first_amendment_advisor_enabled: bool,
    pub money_threshold: f64,
    pub watchlist: Vec<String>,
    #[serde(default = "default_city")]
    pub city: String,
    #[serde(default = "default_state")]
    pub state: String,
}

fn default_city() -> String {
    String::new()
}
fn default_state() -> String {
    String::new()
}
fn default_organization_type() -> String {
    "single_person".to_string()
}
fn default_accent_color() -> String {
    "#5a1818".to_string()
}
fn default_layout_style() -> String {
    "classic".to_string()
}
fn default_first_amendment_advisor_enabled() -> bool {
    true
}

pub(crate) fn curated_ollama_model_ids() -> Result<HashSet<String>, String> {
    let config: serde_json::Value = serde_json::from_str(include_str!("../../src/models.json"))
        .map_err(|e| format!("Could not read bundled model list: {e}"))?;
    let mut ids = HashSet::new();
    for key in ["high", "medium", "low"] {
        if let Some(value) = config.get(key).and_then(|value| value.as_str()) {
            ids.insert(value.to_string());
        }
    }
    if let Some(sizes) = config.get("sizes").and_then(|value| value.as_object()) {
        ids.extend(sizes.keys().cloned());
    }
    Ok(ids)
}

pub(crate) fn is_allowed_ollama_pull_model(model_id: &str) -> bool {
    let model_id = model_id.trim();
    !model_id.is_empty()
        && curated_ollama_model_ids()
            .map(|ids| ids.contains(model_id))
            .unwrap_or(false)
}

fn keep_main_window_visible<R: tauri::Runtime>(app: &tauri::AppHandle<R>, focus: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.center();
        let _ = window.show();
        if focus {
            let _ = window.set_focus();
        }
    }
}

#[tauri::command]
pub fn reveal_main_window_for_setup<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    keep_main_window_visible(&app, true);
    Ok(())
}

#[tauri::command]
pub fn get_resolved_app_data_dir<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<String, String> {
    crate::core::app_paths::app_data_dir(&app).map(|path| path.to_string_lossy().to_string())
}

fn default_community_profile() -> CommunityProfile {
    CommunityProfile {
        site_title: "My Local Publication".to_string(),
        site_subtitle: "Local news and community information.".to_string(),
        about_text: "A locally edited publication for this community.".to_string(),
        ethics_text:
            "Editorial standards are set by the publisher. Corrections are published when needed."
                .to_string(),
        how_we_report_text:
            "Stories, sources, and publication decisions are reviewed by the editor before publication."
                .to_string(),
        organization_type: default_organization_type(),
        footer_text: String::new(),
        logo_url: String::new(),
        accent_color: default_accent_color(),
        layout_style: default_layout_style(),
        first_amendment_advisor_enabled: default_first_amendment_advisor_enabled(),
        money_threshold: 250000.0,
        watchlist: Vec::new(),
        city: String::new(),
        state: String::new(),
    }
}

fn normalize_legacy_profile(mut profile: CommunityProfile) -> CommunityProfile {
    if matches!(
        profile.site_title.as_str(),
        "The Civic Desk" | "Longmont Civic Desk" | "Longmont Local News"
    ) {
        profile.site_title = "My Local Publication".to_string();
    }
    if matches!(
        profile.site_subtitle.as_str(),
        "Transparent Local Public Records & Evidence"
            | "Transparent Local Public Records"
            | "Evidence-backed local public records"
    ) {
        profile.site_subtitle = "Local news and community information.".to_string();
    }
    if profile.about_text.contains("public records newsroom")
        || profile.about_text.contains("raw public records")
    {
        profile.about_text = "A locally edited publication for this community.".to_string();
    }
    if profile.ethics_text.contains("zero ads")
        || profile.ethics_text.contains("public evidence records")
        || profile.ethics_text.contains("not outrage or rumors")
    {
        profile.ethics_text =
            "Editorial standards are set by the publisher. Corrections are published when needed."
                .to_string();
    }
    if profile
        .how_we_report_text
        .contains("on-device AI assistance")
        || profile.how_we_report_text.contains("public agendas")
    {
        profile.how_we_report_text =
            "Stories, sources, and publication decisions are reviewed by the editor before publication."
                .to_string();
    }
    profile
}

/// Normalize an Ollama model tag for EXACT comparison. Ollama treats an untagged
/// name as `:latest`, so `qwen2.5` and `qwen2.5:latest` are the same model. QA-mn1:
/// match exact tags (with this `:latest` normalization) instead of loose
/// substring/`starts_with`/`contains` matching, which could select the wrong
/// model (e.g. `qwen3:4b` selected vs `qwen3:14b` installed).
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
    installed.iter().any(|m| normalize_model_tag(m) == want)
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

    // Default to the scan-safe model verified by the local bakeoff. This
    // fallback only runs when no model is saved in settings.
    let default_m = "phi4-mini:latest".to_string();
    let mut model = default_m.clone();
    if let Ok(resp) = reqwest::get(format!("{}/api/tags", llm::ollama_base_url())).await {
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
                    let names: Vec<String> = tags.models.iter().map(|m| m.name.clone()).collect();
                    // QA-mn1: prefer the default model only if it is EXACTLY
                    // installed (with :latest normalization), then fall back to
                    // known scan-capable families in bakeoff order, then the first model.
                    if model_is_installed(&default_m, &names) {
                        model = default_m;
                    } else if let Some(m) = tags.models.iter().find(|m| {
                        // Match by model FAMILY on the tag's base name (the part
                        // before ':'), not a loose whole-string contains.
                        let base = m.name.split(':').next().unwrap_or("");
                        base == "phi4-mini" || base == "gemma4" || base == "llama3.2"
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
    #[cfg(test)]
    if let Ok(models) = std::env::var("CIVICNEWS_TEST_INSTALLED_MODELS") {
        return models
            .split(',')
            .map(str::trim)
            .filter(|model| !model.is_empty())
            .map(str::to_string)
            .collect();
    }

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
    {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    if let Ok(resp) = client
        .get(format!("{}/api/tags", llm::ollama_base_url()))
        .send()
        .await
    {
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

#[cfg(test)]
mod installed_model_transport_tests {
    use super::list_installed_models;
    use crate::test_support::TestEnv;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn list_installed_models_reads_ollama_tags_response() {
        let mut env = TestEnv::new();
        env.remove("CIVICNEWS_TEST_INSTALLED_MODELS");
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        listener.set_nonblocking(true).unwrap();
        env.set("CIVICNEWS_OLLAMA_BASE_URL", format!("http://{address}"));
        let server = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(3);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error)
                        if error.kind() == std::io::ErrorKind::WouldBlock
                            && Instant::now() < deadline =>
                    {
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(error) => {
                        panic!("model tags test server did not accept a request: {error}")
                    }
                }
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = Vec::new();
            let mut chunk = [0_u8; 1024];
            while request.len() < 8192 && !request.windows(4).any(|part| part == b"\r\n\r\n") {
                let bytes_read = stream.read(&mut chunk).unwrap();
                if bytes_read == 0 {
                    break;
                }
                request.extend_from_slice(&chunk[..bytes_read]);
            }
            assert!(String::from_utf8_lossy(&request).starts_with("GET /api/tags "));
            let body = r#"{"models":[{"name":"phi4-mini:latest"},{"name":"test-model:latest"}]}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(), body
            );
            stream.write_all(response.as_bytes()).unwrap();
        });

        let models = list_installed_models().await;
        server.join().unwrap();
        assert_eq!(models, ["phi4-mini:latest", "test-model:latest"]);
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueueData {
    pub leads: Vec<Lead>,
    pub drafts: Vec<Draft>,
}

fn get_config_path<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<std::path::PathBuf, String> {
    let app_data = crate::core::app_paths::app_data_dir(app)?;
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
    add_source_inner(&db, name, url, r#type, tier)
}

fn normalize_source_url(url: &str) -> String {
    let mut value = url
        .trim()
        .trim_matches(|ch| matches!(ch, '"' | '\'' | '`'))
        .trim()
        .trim_end_matches(['.', ',', ';', ':', '!', '?'])
        .to_string();

    loop {
        let Some(last) = value.chars().last() else {
            break;
        };
        let Some((open, close)) = (match last {
            ')' => Some(('(', ')')),
            ']' => Some(('[', ']')),
            '}' => Some(('{', '}')),
            _ => None,
        }) else {
            break;
        };
        let opens = value.chars().filter(|ch| *ch == open).count();
        let closes = value.chars().filter(|ch| *ch == close).count();
        if closes <= opens {
            break;
        }
        value.pop();
        value = value
            .trim_end_matches(['.', ',', ';', ':', '!', '?'])
            .to_string();
    }

    value
}

/// The storage chokepoint for every source-ingestion path (manual add, discovery
/// auto-import, bulk import — all funnel through the `add_source` command). This
/// is the single place the tier allow-list and the SSRF storage gate are enforced
/// before a source is persisted; it is factored out of the `#[tauri::command]`
/// wrapper so it can be tested directly without a Tauri `AppHandle` (C-6 /
/// CRIT-1). Both gates MUST run before `insert_source`, and a rejected source MUST
/// NOT be inserted — the tests in `tests.rs` pin exactly that.
pub(crate) fn add_source_inner(
    db: &DbConn,
    name: String,
    url: String,
    r#type: String,
    tier: String,
) -> Result<i32, String> {
    if tier != "official_record" && tier != "news_reporting" && tier != "community_signal" {
        return Err("Invalid tier".to_string());
    }
    let url = normalize_source_url(&url);
    // SSRF defense-in-depth: reject non-http(s) schemes and blocked-IP literals
    // at the storage boundary so a discovered/auto-imported URL can never point
    // the scraper at loopback/private/link-local hosts. DNS-based hosts are
    // re-validated (with resolution) at scrape time.
    scraper::validate_source_url(&url)?;
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    if let Ok(existing_id) = conn.query_row(
        "SELECT id FROM sources WHERE lower(trim(url)) = lower(trim(?1)) LIMIT 1",
        [&url],
        |row| row.get::<_, i32>(0),
    ) {
        return Ok(existing_id);
    }
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
pub fn get_civic_intelligence(
    db: tauri::State<'_, DbConn>,
) -> Result<CivicIntelligenceSnapshot, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    intelligence::intelligence_snapshot(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_dark_signals(db: tauri::State<'_, DbConn>) -> Result<Vec<DarkSignal>, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    intelligence::list_dark_signals(&conn, 100).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_verification_queue(
    db: tauri::State<'_, DbConn>,
) -> Result<VerificationQueueSnapshot, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    verification::verification_queue_snapshot(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_verification_task_status(
    db: tauri::State<'_, DbConn>,
    id: i32,
    status: String,
    result_summary: Option<String>,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    verification::update_task_status(&conn, id, &status, result_summary).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_lead_from_dark_signal(
    db: tauri::State<'_, DbConn>,
    dark_signal_id: i32,
) -> Result<i32, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    verification::create_lead_from_dark_signal(&conn, dark_signal_id).map_err(|e| e.to_string())
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
pub fn get_browser_extension_path(app: tauri::AppHandle) -> Result<String, String> {
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Could not resolve app resource directory: {e}"))?;
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|parent| parent.to_path_buf()));

    let mut candidates = vec![
        resource_dir.join("browser-extension").join("chromium"),
        resource_dir
            .join("_up_")
            .join("browser-extension")
            .join("chromium"),
    ];
    if let Some(dir) = exe_dir {
        candidates.push(dir.join("browser-extension").join("chromium"));
        candidates.push(dir.join("_up_").join("browser-extension").join("chromium"));
    }

    let dev_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "Could not resolve project root".to_string())?
        .join("browser-extension")
        .join("chromium");
    candidates.push(dev_path);

    for path in &candidates {
        if path.join("manifest.json").exists() {
            return Ok(path.to_string_lossy().to_string());
        }
    }

    let checked = candidates
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("; ");
    Err(format!(
        "Browser extension folder was not found. Checked: {}",
        checked
    ))
}

#[cfg(test)]
mod extension_path_tests {
    #[test]
    fn development_browser_extension_folder_exists() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("browser-extension")
            .join("chromium")
            .join("manifest.json");
        assert!(
            path.exists(),
            "Expected development extension manifest at {}",
            path.display()
        );
    }

    #[test]
    fn tauri_config_bundles_browser_extension() {
        let config_path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json");
        let config = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|e| panic!("Could not read {}: {e}", config_path.display()));
        assert!(
            config.contains("../browser-extension/chromium/*"),
            "tauri.conf.json must bundle browser extension resources"
        );
    }
}

fn spawn_platform_opener(target: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = std::process::Command::new("explorer.exe");
        command.arg(target);
        command
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = std::process::Command::new("open");
        command.arg(target);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(target);
        command
    };

    command
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Could not open requested item: {}", e))
}

#[tauri::command]
pub fn open_local_path<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    path: String,
) -> Result<(), String> {
    let path = std::path::PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        return Err("No path was provided".to_string());
    }
    if !path.exists() {
        if let Ok(app_data) = crate::core::app_paths::app_data_dir(&app) {
            if crate::core::app_paths::is_standard_site_path(&app_data, &path) {
                std::fs::create_dir_all(&path)
                    .map_err(|e| format!("Could not create the publish output folder: {}", e))?;
            }
        }
    }
    if !path.exists() {
        return Err(format!(
            "The folder or file does not exist: {}",
            path.display()
        ));
    }
    let canonical = std::fs::canonicalize(&path).map_err(|e| e.to_string())?;
    spawn_platform_opener(&canonical.to_string_lossy())
}

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url.trim()).map_err(|e| format!("Invalid URL: {}", e))?;
    match parsed.scheme() {
        "http" | "https" => spawn_platform_opener(parsed.as_str()),
        scheme => Err(format!("Unsupported URL scheme: {}", scheme)),
    }
}

fn xml_text_with_breaks(xml: &str) -> String {
    let mut out = String::new();
    let mut tag = String::new();
    let mut in_tag = false;

    for ch in xml.chars() {
        match ch {
            '<' => {
                in_tag = true;
                tag.clear();
            }
            '>' if in_tag => {
                let tag_name = tag
                    .trim()
                    .trim_start_matches('/')
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                if matches!(
                    tag_name,
                    "w:p" | "w:tr" | "w:br" | "w:tab" | "row" | "c" | "si"
                ) {
                    out.push('\n');
                }
                in_tag = false;
            }
            _ if in_tag => tag.push(ch),
            _ => out.push(ch),
        }
    }

    normalize_extracted_text(&html_escape::decode_html_entities(&out))
}

fn normalize_extracted_text(text: &str) -> String {
    text.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn xml_text_values(xml: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut remaining = xml;
    loop {
        let Some(start) = remaining.find("<t") else {
            break;
        };
        let after_start = &remaining[start..];
        let Some(close) = after_start.find('>') else {
            break;
        };
        let content_start = start + close + 1;
        let after_content = &remaining[content_start..];
        let Some(end) = after_content.find("</t>") else {
            break;
        };
        let raw = &remaining[content_start..content_start + end];
        let value = html_escape::decode_html_entities(raw)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !value.is_empty() {
            values.push(value);
        }
        remaining = &remaining[content_start + end + "</t>".len()..];
    }
    values
}

fn read_zip_entry_text<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    name: &str,
    total_read: &mut u64,
) -> Result<Option<String>, String> {
    match archive.by_name(name) {
        Ok(mut file) => {
            if file.size() > SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES {
                return Err(format!(
                    "{name} is too large after decompression. Export a smaller source list or paste the URLs directly."
                ));
            }
            let mut text = String::new();
            let read = file
                .by_ref()
                .take(SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES + 1)
                .read_to_string(&mut text)
                .map_err(|e| format!("Could not read {name}: {e}"))?;
            if read as u64 > SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES {
                return Err(format!(
                    "{name} is too large after decompression. Export a smaller source list or paste the URLs directly."
                ));
            }
            *total_read += read as u64;
            if *total_read > SOURCE_IMPORT_MAX_ZIP_TOTAL_BYTES {
                return Err(
                    "This Office source-list file expands to too much text. Export a smaller list or paste the URLs directly."
                        .to_string(),
                );
            }
            Ok(Some(text))
        }
        Err(zip::result::ZipError::FileNotFound) => Ok(None),
        Err(e) => Err(format!("Could not read {name}: {e}")),
    }
}

fn extract_docx_text(path: &std::path::Path) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Could not open Word document: {e}"))?;
    let mut total_read = 0;
    let xml = read_zip_entry_text(&mut archive, "word/document.xml", &mut total_read)?
        .ok_or_else(|| "This Word document does not contain readable document text.".to_string())?;
    let text = xml_text_with_breaks(&xml);
    if text.trim().is_empty() {
        Err("No readable text was found in this Word document.".to_string())
    } else {
        Ok(text)
    }
}

fn extract_xlsx_text(path: &std::path::Path) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Could not open spreadsheet: {e}"))?;
    let mut total_read = 0;
    let shared_strings =
        read_zip_entry_text(&mut archive, "xl/sharedStrings.xml", &mut total_read)?
            .map(|shared| xml_text_values(&shared))
            .unwrap_or_default();
    let cell_re = regex::Regex::new(
        r#"(?s)<c\b(?P<attrs>[^>]*)>\s*(?:<v>(?P<v>.*?)</v>|<is>(?P<is>.*?)</is>)"#,
    )
    .map_err(|e| e.to_string())?;
    let row_re =
        regex::Regex::new(r#"(?s)<row\b[^>]*>(?P<body>.*?)</row>"#).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for index in 1..=50 {
        let sheet_name = format!("xl/worksheets/sheet{index}.xml");
        if let Some(sheet) = read_zip_entry_text(&mut archive, &sheet_name, &mut total_read)? {
            for row_cap in row_re.captures_iter(&sheet) {
                let body = row_cap.name("body").map(|m| m.as_str()).unwrap_or("");
                let mut cells = Vec::new();
                for cell_cap in cell_re.captures_iter(body) {
                    let attrs = cell_cap.name("attrs").map(|m| m.as_str()).unwrap_or("");
                    let cell_type = if attrs.contains(r#"t="s""#) {
                        "s"
                    } else if attrs.contains(r#"t="inlineStr""#) {
                        "inlineStr"
                    } else {
                        ""
                    };
                    let raw_value = cell_cap
                        .name("v")
                        .or_else(|| cell_cap.name("is"))
                        .map(|m| m.as_str())
                        .unwrap_or("");
                    let value = if cell_type == "s" {
                        raw_value
                            .trim()
                            .parse::<usize>()
                            .ok()
                            .and_then(|shared_index| shared_strings.get(shared_index).cloned())
                            .unwrap_or_default()
                    } else if cell_type == "inlineStr" {
                        xml_text_with_breaks(raw_value).replace('\n', " ")
                    } else {
                        html_escape::decode_html_entities(raw_value)
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                    };
                    if !value.trim().is_empty() {
                        cells.push(value);
                    }
                }
                if !cells.is_empty() {
                    rows.push(cells.join("\t"));
                }
            }
        }
    }

    let text = rows.join("\n");
    if text.trim().is_empty() {
        Err("No readable text was found in this spreadsheet.".to_string())
    } else {
        Ok(text)
    }
}

#[tauri::command]
pub fn extract_source_import_text(path: String) -> Result<String, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("Choose a file to import first.".to_string());
    }
    let path = std::path::PathBuf::from(trimmed);
    if !path.exists() {
        return Err(format!("The file does not exist: {}", path.display()));
    }
    if !path.is_file() {
        return Err("Choose a source-list file, not a folder.".to_string());
    }
    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    if metadata.len() > SOURCE_IMPORT_MAX_FILE_BYTES {
        return Err(
            "This source-list file is too large. Export a smaller list or paste the URLs directly."
                .to_string(),
        );
    }

    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "txt" | "csv" | "tsv" | "md" | "markdown" | "html" | "htm" | "json" => {
            std::fs::read_to_string(&path).map_err(|e| format!("Could not read file as text: {e}"))
        }
        "docx" => extract_docx_text(&path),
        "xlsx" => extract_xlsx_text(&path),
        "pdf" => Err("PDF source-list import is disabled in this public beta until hardened PDF parsing is available. Convert the PDF to TXT/CSV/DOCX/XLSX or paste the source URLs directly.".to_string()),
        _ => Err("Unsupported source-list file type. Use CSV, TSV, TXT, DOCX, XLSX, or paste URLs directly.".to_string()),
    }
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
pub fn get_community_profile<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<CommunityProfile, String> {
    let path = get_config_path(&app)?;
    if !path.exists() {
        return Ok(default_community_profile());
    }
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let parsed: CommunityProfile = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let normalized = normalize_legacy_profile(parsed.clone());
    if normalized != parsed {
        let content = serde_json::to_string_pretty(&normalized).map_err(|e| e.to_string())?;
        std::fs::write(get_config_path(&app)?, content).map_err(|e| e.to_string())?;
    }
    Ok(normalized)
}

#[tauri::command]
pub fn save_community_profile(
    app: tauri::AppHandle,
    db: tauri::State<'_, DbConn>,
    profile: CommunityProfile,
) -> Result<(), String> {
    let path = get_config_path(&app)?;
    let content = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())?;
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    for (key, value) in [
        ("identity.newsroom_name", profile.site_title.as_str()),
        (
            "identity.organization_type",
            profile.organization_type.as_str(),
        ),
        ("identity.city", profile.city.as_str()),
        ("identity.state", profile.state.as_str()),
    ] {
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn import_logo_asset(path: String) -> Result<String, String> {
    use base64::Engine;

    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("Choose a logo image first.".to_string());
    }
    let path = std::path::PathBuf::from(trimmed);
    if !path.exists() {
        return Err(format!("The logo file does not exist: {}", path.display()));
    }
    if !path.is_file() {
        return Err("Choose an image file, not a folder.".to_string());
    }
    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    if metadata.len() > 2 * 1024 * 1024 {
        return Err(
            "Logo image is too large. Use a PNG, JPG, GIF, or WebP under 2 MB.".to_string(),
        );
    }

    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => return Err("Unsupported logo type. Use a PNG, JPG, GIF, or WebP image.".to_string()),
    };

    let bytes = std::fs::read(&path).map_err(|e| format!("Could not read logo image: {e}"))?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:{mime};base64,{encoded}"))
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
    recover_in_progress_scans_with_saved_leads(&conn).map_err(|e| e.to_string())?;
    db::remediate_legacy_quality_issues(&conn).map_err(|e| e.to_string())?;
    let leads = db::list_leads(&conn).map_err(|e| e.to_string())?;
    let drafts = db::list_drafts(&conn).map_err(|e| e.to_string())?;
    Ok(QueueData { leads, drafts })
}

fn recover_in_progress_scans_with_saved_leads(
    conn: &rusqlite::Connection,
) -> rusqlite::Result<usize> {
    conn.execute(
        "UPDATE daily_scan_runs
         SET completed_at = COALESCE(completed_at, ?1), run_status = 'completed'
         WHERE run_status = 'in_progress'
           AND id IN (SELECT DISTINCT scan_id FROM daily_scan_leads)",
        [chrono::Utc::now().to_rfc3339()],
    )
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
pub fn save_draft(db: tauri::State<'_, DbConn>, mut draft: Draft) -> Result<i32, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    draft.title = compiler::repair_common_mojibake(&draft.title);
    draft.content = compiler::repair_common_mojibake(&draft.content);
    draft.correction_note = draft
        .correction_note
        .map(|note| compiler::repair_common_mojibake(&note));
    draft.missing_evidence_notes = draft
        .missing_evidence_notes
        .map(|note| compiler::repair_common_mojibake(&note));
    let now = chrono::Utc::now().to_rfc3339();
    if draft.created_at.trim().is_empty() {
        draft.created_at = now.clone();
    }
    if draft.updated_at.trim().is_empty() {
        draft.updated_at = now;
    }
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

/// Advisory publish review hook. It records an editor note when one is supplied,
/// but it must never veto a publish-advancing decision. The editor owns the
/// publish/hold/cut decision; software only warns and records.
pub(crate) fn enforce_publish_gate(
    conn: &rusqlite::Connection,
    id: i32,
    decision: &str,
    override_reason: Option<&str>,
) -> Result<(), String> {
    const PUBLISH_STATES: [&str; 3] = ["ready_to_publish", "published", "corrected"];
    if !PUBLISH_STATES.contains(&decision) {
        return Ok(());
    }

    if let Some(reason) = override_reason.map(str::trim).filter(|s| !s.is_empty()) {
        let _ = db::record_guardrail_override(conn, id, reason);
    }

    let (attested_at, stored_override) = db::get_draft_publish_gate(conn, id)
        .map_err(|e| format!("Could not read publish decision state: {e}"))?;
    let guardrails = guardrails::run_guardrails_check(conn, id)
        .map_err(|e| format!("Could not record publish decision audit: {e}"))?;
    let attested = attested_at
        .as_deref()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    let issue_count = guardrails.issues.len() as i32;
    let supplied_reason = override_reason
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or(stored_override);
    let note = match (attested, issue_count, supplied_reason.as_deref()) {
        (false, 0, _) => "Publish decision recorded without a prior editor attestation.",
        (false, _, Some(_)) => {
            "Publish decision recorded without a prior editor attestation; guardrail warnings had an editor note."
        }
        (false, _, None) => {
            "Publish decision recorded without a prior editor attestation and with guardrail warnings."
        }
        (true, 0, _) => "Publish decision recorded after editor attestation.",
        (true, _, Some(_)) => {
            "Publish decision recorded after editor attestation; guardrail warnings had an editor note."
        }
        (true, _, None) => {
            "Publish decision recorded after editor attestation and with guardrail warnings."
        }
    };

    db::record_publish_decision_audit(
        conn,
        &db::PublishDecisionAudit {
            id: None,
            draft_id: id,
            decision: decision.to_string(),
            attested,
            guardrail_override_reason: supplied_reason,
            guardrail_issue_count: issue_count,
            note: note.to_string(),
            created_at: String::new(),
        },
    )
    .map_err(|e| format!("Could not record publish decision audit: {e}"))?;
    Ok(())
}

pub(crate) fn story_decision_with_conn(
    conn: &rusqlite::Connection,
    id: i32,
    decision: &str,
    override_reason: Option<&str>,
) -> Result<(), String> {
    const PUBLISH_STATES: [&str; 3] = ["ready_to_publish", "published", "corrected"];
    if PUBLISH_STATES.contains(&decision) {
        if let Some(draft) = db::get_draft(conn, id).map_err(|e| e.to_string())? {
            if draft.status == "killed" {
                return Err(
                    "This story is cut from the issue. Restore it before approving it for publish."
                        .to_string(),
                );
            }
            if let Some(lead_id) = draft.lead_id {
                let linked_ids: HashSet<i32> = db::get_evidence_by_lead(conn, lead_id)
                    .map_err(|e| e.to_string())?
                    .iter()
                    .filter_map(|item| item.id)
                    .collect();
                let citation_re = regex::Regex::new("(?i)evidence:\\s*(?://)?\\s*(\\d+)")
                    .expect("valid citation regex");
                let mut unlinked_ids: Vec<i32> = citation_re
                    .captures_iter(&draft.content)
                    .filter_map(|caps| caps.get(1))
                    .filter_map(|m| m.as_str().parse::<i32>().ok())
                    .filter(|evidence_id| !linked_ids.contains(evidence_id))
                    .collect();
                unlinked_ids.sort_unstable();
                unlinked_ids.dedup();
                if !unlinked_ids.is_empty() {
                    let ids = unlinked_ids
                        .iter()
                        .map(i32::to_string)
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(format!(
                        "This draft cites evidence ID(s) {ids} that are not linked to this lead. Use the linked source citation buttons or attach the correct source before approving it for publish."
                    ));
                }
            }
        }
    }
    if let Err(err) = enforce_publish_gate(conn, id, decision, override_reason) {
        eprintln!("Publish decision audit failed without vetoing editor decision: {err}");
    }
    let workflow_note = match decision {
        "needs_verification" | "hold" => override_reason,
        _ => None,
    };
    db::update_draft_status_with_note(conn, id, decision, workflow_note).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn story_decision(
    db: tauri::State<'_, DbConn>,
    id: i32,
    decision: String,
    override_reason: Option<String>,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    story_decision_with_conn(&conn, id, &decision, override_reason.as_deref())
}

fn sanitize_unlinked_evidence_citations(text: &str, allowed_ids: &HashSet<i32>) -> String {
    let citation_re =
        regex::Regex::new("(?i)evidence:\\s*(?://)?\\s*(\\d+)").expect("valid citation regex");
    if !citation_re.is_match(text) {
        return text.to_string();
    }

    let mut removed_ids = Vec::new();
    let output = citation_re
        .replace_all(text, |caps: &regex::Captures<'_>| {
            let marker = caps.get(0).map(|m| m.as_str()).unwrap_or_default();
            let id_text = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
            let id = id_text.parse::<i32>().unwrap_or_default();
            if allowed_ids.contains(&id) {
                marker.to_string()
            } else {
                removed_ids.push(id);
                format!("unlinked-evidence-{id_text}")
            }
        })
        .to_string();
    removed_ids.sort_unstable();
    removed_ids.dedup();

    if removed_ids.is_empty() {
        output
    } else {
        format!(
            "{output}\n\n> Source check: The AI draft referenced unlinked evidence ID(s) {}. Those citation markers were disabled automatically. Verify the claim against the linked sources before publishing.",
            removed_ids
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn normalize_model_evidence_citations(text: &str) -> String {
    let malformed_source =
        regex::Regex::new("(?i)\\[\\s*source\\s*\\(\\s*evidence\\s*:\\s*(\\d+)\\s*\\)\\s*\\]")
            .expect("valid malformed source evidence regex");
    let repaired = malformed_source
        .replace_all(text, |caps: &regex::Captures<'_>| {
            format!("[Source](evidence:{})", caps.get(1).unwrap().as_str())
        })
        .to_string();
    let bracketed = regex::Regex::new("(?i)\\[\\s*evidence\\s*:\\s*(\\d+)\\s*\\]\\([^)]*\\)")
        .expect("valid bracketed evidence regex");
    let normalized = bracketed
        .replace_all(&repaired, |caps: &regex::Captures<'_>| {
            format!("[Source](evidence:{})", caps.get(1).unwrap().as_str())
        })
        .to_string();
    let spaced = regex::Regex::new("(?i)evidence:\\s+(\\d+)").expect("valid spaced evidence regex");
    spaced
        .replace_all(&normalized, |caps: &regex::Captures<'_>| {
            format!("evidence:{}", caps.get(1).unwrap().as_str())
        })
        .to_string()
}

fn line_has_forbidden_draft_marker(line: &str) -> bool {
    let trimmed = line.trim();
    let lower = trimmed.to_lowercase();
    lower.starts_with("editor_note:")
        || lower.starts_with("editor note:")
        || lower.starts_with("[editor_note:")
        || lower.starts_with("[editor note:")
        || lower.starts_with("tester edit:")
        || lower == "[source needed]"
        || lower == "[verification needed]"
        || lower == "[end of report]"
        || lower == "end of report"
}

fn strip_bracketed_insert_placeholders(line: &str) -> String {
    let mut output = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find('[') {
        output.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find(']') else {
            output.push_str(&rest[start..]);
            return output;
        };
        let inside = &after[..end];
        let lower = inside.trim().to_lowercase();
        if lower.starts_with("insert ")
            || lower.contains(" if available")
            || lower.contains("source needed")
            || lower.contains("verification needed")
        {
            rest = &after[end + 1..];
        } else {
            output.push('[');
            output.push_str(inside);
            output.push(']');
            rest = &after[end + 1..];
        }
    }
    output.push_str(rest);
    let mut compacted = output.split_whitespace().collect::<Vec<_>>().join(" ");
    for punct in [".", ",", ";", ":", "!", "?"] {
        compacted = compacted.replace(&format!(" {punct}"), punct);
    }
    compacted
}

fn normalize_draft_source_text(value: &str) -> String {
    crate::core::content_quality::normalize_public_text(value)
}

fn clean_generated_draft_for_workbench(text: &str) -> String {
    let mut cleaned = Vec::new();
    let mut skipping_reporting_steps = false;

    for raw_line in text.lines() {
        let line = normalize_draft_source_text(raw_line);
        let trimmed = line.trim();
        let plain = trimmed.trim_matches('*').trim();
        let lower = plain.to_lowercase();

        if line_has_forbidden_draft_marker(plain) {
            skipping_reporting_steps = false;
            continue;
        }

        if lower.starts_with("reporting steps:") || lower.starts_with("next reporting steps:") {
            skipping_reporting_steps = true;
            continue;
        }

        if skipping_reporting_steps {
            if trimmed.is_empty()
                || trimmed.starts_with('-')
                || trimmed
                    .chars()
                    .next()
                    .map(|ch| ch.is_ascii_digit())
                    .unwrap_or(false)
                || trimmed.ends_with('?')
            {
                continue;
            }
            skipping_reporting_steps = false;
        }

        let line = strip_bracketed_insert_placeholders(&line);
        if !line.trim().is_empty()
            || cleaned
                .last()
                .map(|last: &String| !last.is_empty())
                .unwrap_or(false)
        {
            cleaned.push(line);
        }
    }

    cleaned.join("\n").trim().to_string()
}

fn normalize_generated_draft_parts(raw: &str, fallback_title: &str) -> (String, String) {
    let repaired = raw.replace("\r\n", "\n");
    let mut title = fallback_title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut title_line_index: Option<usize> = None;
    let headline_re = regex::Regex::new(
        r"(?i)^\s*(?:#{1,2}\s+|(?:\*\*)?\s*(?:headline|title)\s*:\s*)(.+?)(?:\*\*)?\s*$",
    )
    .expect("valid headline regex");
    let lines = repaired.lines().collect::<Vec<_>>();

    for (idx, line) in lines.iter().take(8).enumerate() {
        if let Some(captures) = headline_re.captures(line) {
            if let Some(value) = captures.get(1).map(|m| m.as_str().trim()) {
                if !value.is_empty() {
                    title = value.replace("**", "").trim().to_string();
                    title_line_index = Some(idx);
                    break;
                }
            }
        }
    }

    let mut cleaned_lines = Vec::new();
    let mut skipping_reporting_steps = false;
    let label_re =
        regex::Regex::new(r"(?i)^\s*(?:\*\*)?\s*(?:nut graf|lede)\s*:\s*(?:\*\*)?\s*(.*)$")
            .expect("valid draft label regex");
    let placeholder_re = regex::Regex::new(
        r"(?i)\[(?:insert [^\]]+|[^\]]+ if available|source needed|verification needed)\]",
    )
    .expect("valid placeholder regex");
    let draft_marker_re = regex::Regex::new(
        r"(?i)^\s*\[?\s*(source needed|verification needed|end of report)\s*\]?\s*$",
    )
    .expect("valid draft marker regex");

    for (idx, line) in lines.iter().enumerate() {
        if title_line_index == Some(idx) {
            continue;
        }
        let plain = line.trim().trim_matches('*').trim();
        let lower = plain.to_lowercase();
        if line_has_forbidden_draft_marker(plain)
            || lower.starts_with("tester edit:")
            || draft_marker_re.is_match(plain)
        {
            skipping_reporting_steps = false;
            continue;
        }
        if lower.starts_with("reporting steps:") || lower.starts_with("next reporting steps:") {
            skipping_reporting_steps = true;
            continue;
        }
        if skipping_reporting_steps {
            if plain.is_empty()
                || plain.starts_with('-')
                || plain.starts_with('*')
                || plain
                    .chars()
                    .next()
                    .map(|ch| ch.is_ascii_digit())
                    .unwrap_or(false)
                || plain.ends_with('?')
            {
                continue;
            }
            skipping_reporting_steps = false;
        }

        let labeled = label_re
            .captures(line)
            .and_then(|captures| captures.get(1).map(|m| m.as_str()))
            .unwrap_or(line);
        let normalized = placeholder_re
            .replace_all(labeled, "")
            .replace("  ", " ")
            .trim_end()
            .to_string();
        cleaned_lines.push(normalized);
    }

    let content = cleaned_lines
        .join("\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    (title, content)
}

fn first_public_sentence(text: &str) -> String {
    let cleaned = normalize_draft_source_text(text);
    let sentence_end = cleaned
        .char_indices()
        .find_map(|(idx, ch)| matches!(ch, '.' | '!' | '?').then_some(idx + ch.len_utf8()));
    let mut sentence = sentence_end
        .and_then(|idx| cleaned.get(..idx))
        .unwrap_or(&cleaned)
        .trim()
        .to_string();
    if sentence.len() > 260 {
        sentence.truncate(260);
        sentence = sentence
            .rsplit_once(' ')
            .map(|(left, _)| left.to_string())
            .unwrap_or(sentence);
        sentence.push('.');
    }
    if sentence.is_empty() {
        "The linked source has a civic update that needs editor review.".to_string()
    } else {
        sentence
    }
}

fn lead_needs_draft_caution(
    story_type: Option<&str>,
    disposition: Option<&str>,
    novelty_score: Option<i32>,
    recurrence_count: Option<i32>,
) -> bool {
    let story_type = story_type.unwrap_or_default().to_lowercase();
    let disposition = disposition.unwrap_or("review").to_lowercase();
    let novelty = novelty_score.unwrap_or(0);
    recurrence_count.unwrap_or(0) > 0
        || matches!(story_type.as_str(), "background" | "watch" | "verification")
        || matches!(
            disposition.as_str(),
            "background" | "watch" | "needs_verification"
        )
        || (novelty > 0 && novelty <= 2)
}

fn draft_has_publishable_linked_source_shape(content: &str, linked_evidence_count: usize) -> bool {
    linked_evidence_count > 0
        && regex::Regex::new("(?i)evidence:\\s*(?://)?\\s*\\d+")
            .expect("valid evidence regex")
            .is_match(content)
        && content.to_lowercase().contains("according to")
}

#[allow(clippy::too_many_arguments)]
fn build_persistable_draft(
    lead_id: i32,
    lead_why: &str,
    story_type: Option<&str>,
    disposition: Option<&str>,
    novelty_score: Option<i32>,
    recurrence_count: Option<i32>,
    generated_text: &str,
    evidence_count: usize,
    format: &str,
) -> Draft {
    let fallback_title = source_bound_headline(lead_why);
    let (title, content) = normalize_generated_draft_parts(generated_text, &fallback_title);
    let cautious =
        lead_needs_draft_caution(story_type, disposition, novelty_score, recurrence_count);
    let publishable_linked_shape =
        draft_has_publishable_linked_source_shape(&content, evidence_count);
    let force_verification = evidence_count == 0 || (cautious && !publishable_linked_shape);

    Draft {
        id: None,
        lead_id: Some(lead_id),
        format: format.to_string(),
        title,
        content,
        status: if force_verification {
            "needs_verification".to_string()
        } else {
            "draft_generated".to_string()
        },
        verification_checklist: "[]".to_string(),
        missing_evidence_notes: if evidence_count == 0 {
            Some("No source documents are linked to this lead yet. Treat this as a verification assignment until public source material is attached or cited.".to_string())
        } else {
            None
        },
        correction_note: None,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

fn is_source_backed_draftable_lead(story_type: Option<&str>, disposition: Option<&str>) -> bool {
    let story_type = story_type.unwrap_or_default().to_lowercase();
    let disposition = disposition.unwrap_or_default().to_lowercase();
    matches!(story_type.as_str(), "brief" | "story")
        && matches!(disposition.as_str(), "ready_to_draft" | "draftable")
}

fn draft_evidence_for_lead(
    lead_why: &str,
    story_type: Option<&str>,
    disposition: Option<&str>,
    linked_items: Vec<EvidenceItem>,
) -> Vec<EvidenceItem> {
    let topic_matched = source_grounding::filter_topic_matched_evidence(lead_why, &linked_items);
    if topic_matched.is_empty()
        && !linked_items.is_empty()
        && is_source_backed_draftable_lead(story_type, disposition)
    {
        linked_items
    } else {
        topic_matched
    }
}

fn source_bound_headline(lead_why: &str) -> String {
    let cleaned = lead_why
        .replace('\n', " ")
        .split("Editor context:")
        .next()
        .unwrap_or(lead_why)
        .split("Suggested treatment:")
        .next()
        .unwrap_or(lead_why)
        .split("Suggested next step:")
        .next()
        .unwrap_or(lead_why)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut title = cleaned
        .trim_matches(|ch: char| ch == '"' || ch == '\'' || ch == ':' || ch.is_whitespace())
        .to_string();
    if title.len() > 96 {
        title.truncate(96);
        title = title
            .rsplit_once(' ')
            .map(|(left, _)| left.to_string())
            .unwrap_or(title);
    }
    if title.is_empty() {
        "Civic item needs review".to_string()
    } else {
        title
    }
}

fn audience_from_profile(profile: &CommunityProfile) -> String {
    let city = profile.city.trim();
    if city.is_empty() {
        "local readers".to_string()
    } else {
        format!("{city} readers")
    }
}

fn source_bound_fallback_draft(
    lead_why: &str,
    evidence_items: &[EvidenceItem],
    audience: &str,
    format: &str,
) -> String {
    let headline = source_bound_headline(lead_why);
    if !evidence_items.iter().any(|item| item.id.is_some()) {
        return format!(
            "Headline: {headline}\n\nNo source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material."
        );
    }
    let linked_sentences = evidence_items
        .iter()
        .filter_map(|item| item.id.map(|id| (id, first_public_sentence(&item.excerpt))))
        .take(3)
        .collect::<Vec<_>>();
    let first_paragraph = linked_sentences
        .first()
        .map(|(id, sentence)| {
            format!("According to the linked source, {sentence} [Source](evidence:{id}).")
        })
        .unwrap_or_else(|| {
            "No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.".to_string()
        });
    let extra_sources = linked_sentences
        .iter()
        .skip(1)
        .map(|(id, sentence)| {
            format!("A second linked source says {sentence} [Source](evidence:{id}).")
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let closing = if matches!(format, "brief" | "story" | "article") {
        format!("Taken together, the linked records give {audience} a source-backed civic brief. Keep the story limited to these records until an editor confirms any public impact, cost, decision date, or agency response not shown in the linked sources.")
    } else {
        format!("This is a watch brief for {audience}. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.")
    };
    if extra_sources.is_empty() {
        format!("Headline: {headline}\n\n{first_paragraph}\n\n{closing}")
    } else {
        format!("Headline: {headline}\n\n{first_paragraph}\n\n{extra_sources}\n\n{closing}")
    }
}

fn repeated_phrase_count(text: &str, phrase: &str) -> usize {
    text.to_lowercase().matches(phrase).count()
}

fn generated_draft_quality_issue(
    draft: &str,
    evidence_text: &str,
    evidence_count: usize,
    format: &str,
) -> Option<String> {
    let lower = draft.to_lowercase();
    let evidence_lower = evidence_text.to_lowercase();
    let reader_facing_format = matches!(format, "brief" | "story" | "article");
    if draft.contains("&#") || draft.contains("&amp;#") || draft.contains("-->") {
        return Some("model output retained encoded HTML or page markup debris".to_string());
    }
    if looks_like_multi_item_event_listing_draft(&lower) {
        return Some("model output copied a broad multi-item event listing".to_string());
    }
    if draft.len() > 8_000 {
        return Some("model output was far too long for a draft".to_string());
    }
    if repeated_phrase_count(draft, "reader-facing watch item") > 2
        || repeated_phrase_count(draft, "further fact-checking") > 3
        || lower.contains("proof of research required")
    {
        return Some("model output repeated template/junk text".to_string());
    }
    if lower.contains("evidence:none") || lower.contains("evidence: none") {
        return Some("model cited missing evidence".to_string());
    }
    if regex::Regex::new("(?i)\\[\\s*source\\s*\\(\\s*evidence\\s*:\\s*\\d+\\s*\\)\\s*\\]")
        .expect("valid malformed source evidence regex")
        .is_match(draft)
    {
        return Some("model output used malformed evidence citation syntax".to_string());
    }
    if evidence_count > 0
        && !regex::Regex::new("(?i)evidence:\\s*(?://)?\\s*\\d+")
            .expect("valid evidence regex")
            .is_match(draft)
    {
        return Some("model output did not include inline evidence citations".to_string());
    }
    if evidence_count > 0 && !lower.contains("according to") {
        return Some("model output did not clearly attribute sourced claims".to_string());
    }
    if reader_facing_format && lower.contains("this is a watch brief") {
        return Some(
            "model output used watch-brief scaffolding for a reader-facing brief".to_string(),
        );
    }
    if reader_facing_format && evidence_count > 1 {
        let paragraph_count = draft
            .split("\n\n")
            .filter(|part| {
                let trimmed = part.trim();
                !trimmed.is_empty() && !trimmed.to_lowercase().starts_with("headline:")
            })
            .count();
        if paragraph_count < 2 {
            return Some(
                "model output was too thin for a linked-evidence reader-facing brief".to_string(),
            );
        }
    }
    for term in [
        "cancel",
        "canceled",
        "cancelled",
        "cancellation",
        "covid",
        "pandemic",
        "funding cut",
        "funding cuts",
        "at risk",
        "selected vendor",
        "vendor",
        "contractor",
    ] {
        if lower.contains(term) && !evidence_lower.contains(term) {
            return Some(format!(
                "model output introduced unsupported high-risk claim term `{term}`"
            ));
        }
    }
    for term in [
        "school district 2",
        "burlington public schools",
        "senior volunteers",
        "abc news",
        "kmgh",
        "sam mims",
    ] {
        if lower.contains(term) && !evidence_lower.contains(term) {
            return Some(format!(
                "model output introduced unsupported named source or entity `{term}`"
            ));
        }
    }
    None
}

fn looks_like_multi_item_event_listing_draft(lower: &str) -> bool {
    let month_hits = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ]
    .iter()
    .map(|month| lower.matches(month).count())
    .sum::<usize>();
    let event_hits = [
        "concert",
        "festival",
        "calendar",
        "library closed",
        "free fitness",
        "symphony",
        "storytime",
        "loteria mexicana",
        "lotería mexicana",
        "bilingual",
        "workshop",
        "class",
    ]
    .iter()
    .map(|term| lower.matches(*term).count())
    .sum::<usize>();
    let time_hits = regex::Regex::new(r"(?i)\b\d{1,2}\s*(am|pm)\b")
        .expect("valid event time regex")
        .find_iter(lower)
        .count();
    (lower.len() > 360 && month_hits >= 1 && event_hits >= 4)
        || (lower.len() > 360 && time_hits >= 4 && event_hits >= 3)
}

#[cfg(test)]
mod draft_citation_tests {
    use super::sanitize_unlinked_evidence_citations;
    use std::collections::HashSet;

    #[test]
    fn disables_citations_that_are_not_linked_to_the_lead() {
        let allowed = HashSet::from([3, 9]);
        let draft = "Known from [the agenda](evidence:3). Unsupported [claim](evidence:224).";

        let sanitized = sanitize_unlinked_evidence_citations(draft, &allowed);

        assert!(sanitized.contains("evidence:3"));
        assert!(sanitized.contains("unlinked-evidence-224"));
        assert!(sanitized.contains("Source check"));
        assert!(sanitized.contains("224"));
    }

    #[test]
    fn normalizes_model_evidence_citation_shapes() {
        let draft =
            "Claim one [evidence: 67](17). Claim two [Source](evidence: 66). Claim three [Source(evidence:15)].";

        let normalized = super::normalize_model_evidence_citations(draft);

        assert!(normalized.contains("[Source](evidence:67)"));
        assert!(normalized.contains("[Source](evidence:66)"));
        assert!(normalized.contains("[Source](evidence:15)"));
    }

    #[test]
    fn disables_all_model_citations_when_no_evidence_is_linked() {
        let allowed = HashSet::new();
        let draft = "This unlinked draft cites [a source](evidence:12), [another](Evidence://13), and [one more](EVIDENCE:14).";

        let sanitized = sanitize_unlinked_evidence_citations(draft, &allowed);

        assert!(!sanitized.contains("](evidence:12)"));
        assert!(!sanitized.contains("Evidence://13"));
        assert!(!sanitized.contains("EVIDENCE:14"));
        assert!(sanitized.contains("unlinked-evidence-12"));
        assert!(sanitized.contains("unlinked-evidence-13"));
        assert!(sanitized.contains("unlinked-evidence-14"));
        assert!(sanitized.contains("Source check"));
    }

    #[test]
    fn cleans_model_reporter_markers_before_workbench() {
        let draft = "Headline: Council to review traffic safety\n\nEDITOR_NOTE: weak source\n\nThe council will review traffic safety at an upcoming meeting [insert date if available].\n\nReporting Steps:\n- Call the clerk.\n- Verify date.\n\nResidents can follow the agenda for the next posted meeting. [Source](evidence:4)\n\n[End of Report]";

        let cleaned = super::clean_generated_draft_for_workbench(draft);

        assert!(!cleaned.to_lowercase().contains("editor_note"));
        assert!(!cleaned.to_lowercase().contains("reporting steps"));
        assert!(!cleaned.to_lowercase().contains("[insert"));
        assert!(!cleaned.to_lowercase().contains("end of report"));
        assert!(cleaned.contains("Headline: Council to review traffic safety"));
        assert!(cleaned.contains("Residents can follow the agenda"));
    }

    #[test]
    fn falls_back_when_model_output_is_junk_or_unsupported() {
        let item = crate::core::db::EvidenceItem {
            id: Some(66),
            source_id: 1,
            url: Some("https://example.test/events".to_string()),
            fetched_at: "2026-06-30T00:00:00Z".to_string(),
            excerpt: "The city calendar lists snacks and antojitos at the youth center."
                .to_string(),
            content_hash: "events".to_string(),
            entities: "[]".to_string(),
        };
        let draft = "According to the source, the program was canceled because of COVID funding cuts. [Source](evidence:66)";
        let evidence_text = format!("Evidence Citation ID: 66\nExcerpt: {}\n\n", item.excerpt);

        let issue = super::generated_draft_quality_issue(draft, &evidence_text, 1, "watch")
            .expect("unsupported claims should be rejected");
        let fallback = super::source_bound_fallback_draft(
            "Snacks and Antojitos (Summer Program)",
            &[item],
            "local readers",
            "watch",
        );

        assert!(issue.contains("unsupported"));
        assert!(fallback.contains("Headline: Snacks and Antojitos"));
        assert!(fallback.contains("[Source](evidence:66)"));
        assert!(fallback.contains("local readers"));
        assert!(!fallback.to_lowercase().contains("covid"));
        assert!(!fallback.to_lowercase().contains("funding cut"));
        assert!(!fallback.contains("Longmont readers"));
    }

    #[test]
    fn falls_back_when_model_output_lacks_attribution_or_adds_named_claims() {
        let evidence_text = "Evidence Citation ID: 15\nExcerpt: St. Vrain Valley Schools announced direct admission offers through CU Denver.\n\n";
        let unattributed = "Students have a direct admission opportunity [Source](evidence:15).";
        let named_drift = "School District 2 and Burlington Public Schools will hold meetings according to the source [Source](evidence:15).";

        assert!(
            super::generated_draft_quality_issue(unattributed, evidence_text, 1, "watch")
                .unwrap()
                .contains("attribute")
        );
        assert!(
            super::generated_draft_quality_issue(named_drift, evidence_text, 1, "watch")
                .unwrap()
                .contains("unsupported")
        );
    }

    #[test]
    fn rejects_encoded_multi_item_event_listing_drafts() {
        let draft = "Headline: Independence Weekend Events\n\nAccording to the source, Friday, July 3 \u{00e2}\u{20ac}\u{00a2} 6 pm &#8211; 10 pm brings a free concert. LIBRARY CLOSED Friday, July 3 \u{00e2}\u{20ac}\u{00a2} 6 pm &#8211; 10 pm. Free Fitness in the Park Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 8 am &#8211; 9 am. July 4th Longmont Symphony Concert Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 11 am &#8211; 12 pm. Independence Weekend Festival Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 4 pm &#8211; 10 pm. Loter\u{00c3}\u{00ad}a Mexicana Monday, July 6 \u{00e2}\u{20ac}\u{00a2} 5 pm &#8211; 6 pm. Bilingual Storytime Tuesday, July 7 \u{00e2}\u{20ac}\u{00a2} 10 am &#8211; 11 am. [Source](evidence:9)";
        let cleaned = super::clean_generated_draft_for_workbench(draft);

        assert!(!cleaned.contains("&#8211;"));
        assert!(!cleaned.contains("\u{00e2}\u{20ac}\u{00a2}"));
        assert!(cleaned.contains("6 pm - 10 pm"));
        assert!(super::generated_draft_quality_issue(
            &cleaned,
            "Evidence Citation ID: 9\nExcerpt: Events calendar.\n\n",
            1,
            "watch",
        )
        .unwrap()
        .contains("event listing"));
    }

    #[test]
    fn rejects_watch_fragment_for_reader_facing_brief() {
        let evidence_text = "Evidence Citation ID: 15\nExcerpt: St. Vrain Valley Schools announced an academic program update.\n\nEvidence Citation ID: 16\nExcerpt: The district listed a board meeting date for the program.\n\n";
        let draft = "Headline: Academic Excellence By Design Initiative\n\nAccording to the linked source, August 1, 2026 - Aug 12 Board of Education Regular Meeting [Source](evidence:15).\n\nThis is a watch brief for residents.";

        let issue = super::generated_draft_quality_issue(draft, evidence_text, 2, "brief")
            .expect("watch fragments should not satisfy a brief lead");

        assert!(issue.contains("watch-brief scaffolding"));
    }

    #[test]
    fn source_bound_fallback_is_attributed_and_reader_facing() {
        let item = crate::core::db::EvidenceItem {
            id: Some(15),
            source_id: 1,
            url: Some("https://example.test/schools".to_string()),
            fetched_at: "2026-07-02T00:00:00Z".to_string(),
            excerpt:
                "St. Vrain Valley Schools announced direct admission offers through CU Denver."
                    .to_string(),
            content_hash: "schools".to_string(),
            entities: "[]".to_string(),
        };

        let fallback = super::source_bound_fallback_draft(
            "St. Vrain direct admission offer",
            &[item],
            "local readers",
            "watch",
        );

        assert!(fallback.contains("According to the linked source"));
        assert!(fallback.contains("[Source](evidence:15)"));
        assert!(!fallback.contains("Burlington Public Schools"));
    }

    #[test]
    fn source_bound_brief_fallback_does_not_use_watch_brief_copy() {
        let item = crate::core::db::EvidenceItem {
            id: Some(25),
            source_id: 1,
            url: Some("https://longmontcolorado.gov/events/".to_string()),
            fetched_at: "2026-07-09T00:00:00Z".to_string(),
            excerpt: "Longmont posted a utility update for residents with a public meeting date, city department contact, and transportation service information.".to_string(),
            content_hash: "utility".to_string(),
            entities: "[]".to_string(),
        };

        let fallback = super::source_bound_fallback_draft(
            "Review community signal from Longmont city events",
            &[item],
            "Longmont readers",
            "brief",
        );

        assert!(fallback.contains("source-backed civic brief"));
        assert!(!fallback.to_lowercase().contains("watch brief"));
        assert!(fallback.contains("[Source](evidence:25)"));
    }

    #[test]
    fn no_evidence_fallback_stays_a_verification_assignment() {
        let fallback = super::source_bound_fallback_draft(
            "City of Longmont to Perform Chip Seal on Francis Street July 6-15",
            &[],
            "Longmont readers",
            "watch",
        );
        let lower = fallback.to_lowercase();

        assert!(fallback.contains("No source documents are linked"));
        assert!(fallback.contains("verification assignment"));
        assert!(!lower.contains("abc news"));
        assert!(!lower.contains("kmgh"));
        assert!(!lower.contains("sam mims"));
    }

    #[test]
    fn durable_generated_brief_draft_is_persistable_from_linked_evidence() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        conn.execute(
            "INSERT INTO leads (id, detector_name, why, confidence, risk_level, confirmation_checklist, from_scan_lead_id, story_type, disposition, novelty_score, novelty_reason, created_at)
             VALUES (4, 'daily_scan', 'Review community signal from Longmont city events: Public Safety Sustainability Transportation Utilities', 'med', 'med', '[]', 4, 'brief', 'ready_to_draft', 4, 'Specific source-backed civic item detected from recent evidence.', '2026-07-09T00:00:00Z')",
            [],
        )
        .unwrap();
        let generated = "Headline: Longmont reviews utility update\n\nAccording to the linked source, Longmont posted a utility update for residents. [Source](evidence:25)\n\nThe brief should stay limited to the linked source.";

        let draft = super::build_persistable_draft(
            4,
            "Review community signal from Longmont city events: Public Safety Sustainability Transportation Utilities",
            Some("brief"),
            Some("ready_to_draft"),
            Some(4),
            None,
            generated,
            1,
            "brief",
        );

        assert_eq!(draft.lead_id, Some(4));
        assert_eq!(draft.format, "brief");
        assert_eq!(draft.status, "draft_generated");
        assert_eq!(draft.missing_evidence_notes, None);
        assert_eq!(draft.title, "Longmont reviews utility update");
        assert!(!draft.content.contains("Headline:"));
        assert!(draft.content.contains("According to the linked source"));

        let draft_id = crate::core::db::insert_draft(&conn, &draft).unwrap();
        let persisted = crate::core::db::get_draft(&conn, draft_id)
            .unwrap()
            .unwrap();
        assert_eq!(persisted.id, Some(draft_id));
        assert_eq!(persisted.lead_id, Some(4));
        assert_eq!(persisted.status, "draft_generated");
        assert_eq!(persisted.title, "Longmont reviews utility update");
    }

    #[test]
    fn draftable_brief_keeps_linked_evidence_when_topic_filter_is_too_strict() {
        let linked = vec![crate::core::db::EvidenceItem {
            id: Some(25),
            source_id: 1,
            url: Some("https://longmontcolorado.gov/events/".to_string()),
            fetched_at: "2026-07-09T00:00:00Z".to_string(),
            excerpt: "Longmont posted a utility update for residents with a public meeting date, city department contact, and transportation service information.".to_string(),
            content_hash: "utility".to_string(),
            entities: "[]".to_string(),
        }];

        let evidence = super::draft_evidence_for_lead(
            "Review community signal from Longmont city events: Public Safety Sustainability Transportation Utilities",
            Some("brief"),
            Some("ready_to_draft"),
            linked,
        );

        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].id, Some(25));
    }
}

#[cfg(test)]
mod queue_recovery_tests {
    use chrono::Utc;
    use rusqlite::Connection;

    #[test]
    fn recovers_in_progress_scan_when_saved_leads_exist() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();

        conn.execute(
            "INSERT INTO daily_scan_runs (started_at, run_status) VALUES (?1, 'in_progress')",
            [Utc::now().to_rfc3339()],
        )
        .unwrap();
        let scan_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO daily_scan_leads (scan_id, title, summary, original_url)
             VALUES (?1, 'Saved lead', 'Saved summary', 'https://example.gov/notice')",
            [scan_id],
        )
        .unwrap();

        let changed = super::recover_in_progress_scans_with_saved_leads(&conn).unwrap();
        assert_eq!(changed, 1);

        let (status, completed_at): (String, Option<String>) = conn
            .query_row(
                "SELECT run_status, completed_at FROM daily_scan_runs WHERE id = ?1",
                [scan_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(status, "completed");
        assert!(completed_at.is_some());
    }
}

fn story_template_guidance(
    conn: &rusqlite::Connection,
    story_type: Option<&str>,
) -> Option<String> {
    let key = story_type
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("verification");
    conn.query_row(
        "SELECT prompt_guidance FROM story_templates WHERE key = ?1",
        [key],
        |row| row.get::<_, String>(0),
    )
    .ok()
}

#[allow(clippy::too_many_arguments)]
fn build_draft_prompt(
    lead_why: &str,
    story_type: Option<&str>,
    disposition: Option<&str>,
    novelty_score: Option<i32>,
    novelty_reason: Option<&str>,
    recurrence_count: Option<i32>,
    recurrence_note: Option<&str>,
    template_guidance: Option<&str>,
    evidence_context: &str,
    evidence_count: usize,
    format: &str,
    audience: &str,
) -> String {
    let lead_quality_context = format!(
        "Story type: {}\nEditorial disposition: {}\nNovelty score: {}\nNovelty reason: {}\nBeat recurrence: {}\nBeat recurrence note: {}\nStory template guidance: {}\n",
        story_type.unwrap_or("unspecified"),
        disposition.unwrap_or("review"),
        novelty_score
            .map(|score| format!("{score}/5"))
            .unwrap_or_else(|| "unspecified".to_string()),
        novelty_reason.unwrap_or("unspecified"),
        recurrence_count
            .map(|count| format!("{count} previous appearance(s)"))
            .unwrap_or_else(|| "not known".to_string()),
        recurrence_note.unwrap_or("none"),
        template_guidance
            .unwrap_or("Use the editor's selected format and keep the draft evidence-bound.")
    );
    let advisory = "If editorial disposition is background, watch, needs_verification, low novelty, no current change found, or recurring beat memory with no new fact, write a short reader-facing watch item or background brief instead of inflating it into a full story. Do not use internal editor labels, placeholders, or private newsroom notes in the draft body.";
    let forbidden_output = "Never include EDITOR_NOTE, Editor Note, TESTER EDIT, Nut graf, Reporting Steps, Source needed, Verification needed, End of Report, Body:, or bracketed placeholders such as [insert date]. If a fact is not in the source, omit it or say only what the source actually says.";

    if evidence_count == 0 {
        format!(
            "Lead topic: {}\n\n{}\nNo source documents are attached to this lead yet. Prepare a short '{}' watch item for an editor to expand later.\n\n{}\n\nReturn clean Markdown. First line must be `Headline: ...`. After the headline, write reader-facing copy only: what is known, why a resident might watch it, and what should be checked next. {} Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details.",
            lead_why, lead_quality_context, format, advisory, forbidden_output
        )
    } else if evidence_count < 2 {
        format!(
            "Lead topic: {}\n\n{}\nHere is the only attached source material:\n{}\nPlease draft a '{}' item.\n\n{}\n\nReturn clean Markdown for an editor. First line must be `Headline: ...`. Because there is only one linked source, write this as a short brief or watch item unless the evidence shows a current, specific development. If the excerpt includes dated current items, choose the strongest dated item rather than describing the whole page. Use only the attached excerpt and cite it as [Source](evidence:ID). {} Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details. If the excerpt is evergreen/background material, write a reader-facing background brief with a final sentence on what residents should watch for next.",
            lead_why, lead_quality_context, evidence_context, format, advisory, forbidden_output
        )
    } else {
        format!(
            "Lead topic: {}\n\n{}\nHere is the attached source material:\n{}\nPlease draft a '{}' item.\n\n{}\n\nWrite for {} in clean Markdown. First line must be `Headline: ...`. After the headline, write reader-facing article copy only: a clear lede, 3-5 factual paragraphs or short sections, and a concise explanation of what remains uncertain when needed. If the excerpts include dated current items, choose the strongest dated/current item rather than describing the source page itself. Use only the listed Evidence Citation IDs in citations like [Source](evidence:ID). Do not cite any other evidence ID. Use a citation for every factual claim drawn from the source material. {} Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details. If the evidence is evergreen/background material or does not show a current development, write a reader-facing watch/background brief rather than a full news story.",
            lead_why, lead_quality_context, evidence_context, format, advisory, audience, forbidden_output
        )
    }
}

#[cfg(test)]
mod draft_prompt_tests {
    use super::{build_draft_prompt, story_template_guidance};

    #[test]
    fn background_leads_include_quality_context_and_template_guidance() {
        let prompt = build_draft_prompt(
            "Council video archive: no current change found.",
            Some("background"),
            Some("background"),
            Some(1),
            Some("no current change found"),
            Some(2),
            Some("Similar topic was seen on earlier scans."),
            Some("Do not frame this as news. State what new fact would make it publishable."),
            "Evidence Citation ID: 7\nExcerpt: Archived council meetings are available.\n\n",
            1,
            "story",
            "local readers",
        );

        assert!(prompt.contains("Story type: background"));
        assert!(prompt.contains("Editorial disposition: background"));
        assert!(prompt.contains("Novelty score: 1/5"));
        assert!(prompt.contains("Novelty reason: no current change found"));
        assert!(prompt.contains("Beat recurrence: 2 previous appearance(s)"));
        assert!(prompt.contains("Similar topic was seen on earlier scans."));
        assert!(prompt.contains("reader-facing watch item or background brief"));
        assert!(prompt.contains("Do not use internal editor labels"));
        assert!(prompt.contains("Never include EDITOR_NOTE"));
    }

    #[test]
    fn story_template_guidance_reads_seeded_templates() {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();

        let guidance = story_template_guidance(&conn, Some("background")).unwrap();

        assert!(guidance.contains("reader-facing background brief"));
        assert!(guidance.contains("private editor-note labels"));
    }

    #[test]
    fn multi_source_prompt_uses_configured_audience_without_hardcoded_city() {
        let prompt = build_draft_prompt(
            "Council vote on library roof contract.",
            Some("story"),
            Some("ready_to_draft"),
            Some(4),
            Some("new agenda item"),
            Some(0),
            Some("none"),
            Some("Write a short civic news story."),
            "Evidence Citation ID: 7\nExcerpt: Agenda item.\n\nEvidence Citation ID: 8\nExcerpt: Staff memo.\n\n",
            2,
            "story",
            "Pueblo readers",
        );

        assert!(prompt.contains("Write for Pueblo readers"));
        assert!(!prompt.contains("Longmont residents"));
        assert!(!prompt.contains("Brighton"));
    }
}

/// Record that a human reviewed this draft and accepts responsibility.
#[tauri::command]
pub fn attest_draft(db: tauri::State<'_, DbConn>, id: i32, editor: String) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    db::attest_draft(&conn, id, &editor).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_and_save_draft<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, DbConn>,
    lead_id: i32,
    format: String,
    system_prompt: Option<String>,
) -> Result<Draft, String> {
    let (
        lead_why,
        story_type,
        disposition,
        novelty_score,
        novelty_reason,
        recurrence_count,
        recurrence_note,
        template_guidance,
        evidence_items,
    ) = {
        let conn = db
            .lock()
            .map_err(|_| "Failed to lock database".to_string())?;
        let mut stmt = conn
            .prepare("SELECT why, story_type, disposition, novelty_score, novelty_reason, recurrence_count, recurrence_note FROM leads WHERE id = ?1")
            .map_err(|e| e.to_string())?;
        type LeadPromptRow = (
            String,
            Option<String>,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<i32>,
            Option<String>,
        );

        let (
            why,
            story_type,
            disposition,
            novelty_score,
            novelty_reason,
            recurrence_count,
            recurrence_note,
        ): LeadPromptRow = stmt
            .query_row([lead_id], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            })
            .map_err(|e| e.to_string())?;
        let template_guidance = story_template_guidance(&conn, story_type.as_deref());
        let linked_items = db::get_evidence_by_lead(&conn, lead_id).map_err(|e| e.to_string())?;
        let evidence_items = draft_evidence_for_lead(
            &why,
            story_type.as_deref(),
            disposition.as_deref(),
            linked_items,
        );
        (
            why,
            story_type,
            disposition,
            novelty_score,
            novelty_reason,
            recurrence_count,
            recurrence_note,
            template_guidance,
            evidence_items,
        )
    };

    let mut evidence_context = String::new();
    for item in &evidence_items {
        let item_id = item.id.unwrap_or(0);
        evidence_context.push_str(&format!(
            "Evidence Citation ID: {}\nExcerpt: {}\n\n",
            item_id,
            normalize_draft_source_text(&item.excerpt)
        ));
    }

    let profile = get_community_profile(app.clone())?;
    let audience = audience_from_profile(&profile);
    let generated_text = if evidence_items.is_empty() {
        source_bound_fallback_draft(&lead_why, &evidence_items, &audience, &format)
    } else {
        let prompt = build_draft_prompt(
            &lead_why,
            story_type.as_deref(),
            disposition.as_deref(),
            novelty_score,
            novelty_reason.as_deref(),
            recurrence_count,
            recurrence_note.as_deref(),
            template_guidance.as_deref(),
            &evidence_context,
            evidence_items.len(),
            &format,
            &audience,
        );

        let sys = system_prompt.unwrap_or_else(|| "You are an assistant for a local publication editor. You help prepare careful working drafts. Do not decide what is publishable; warn about uncertainty and leave final judgment to the human editor.".to_string());
        let model = get_selected_model_or_fallback(&db).await;
        let installed = list_installed_models().await;
        if !model_is_installed(&model, &installed) {
            return Err(format!(
                "MODEL_NOT_INSTALLED: The selected AI model '{}' is not installed. Open Setup and download a model before drafting.",
                model
            ));
        }

        let llm_client = app
            .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
            .inner()
            .clone();
        let draft = llm_client.call(&model, &prompt, &sys).await?;
        let allowed_ids = evidence_items
            .iter()
            .filter_map(|item| item.id)
            .collect::<HashSet<_>>();
        let normalized_citations = normalize_model_evidence_citations(&draft);
        let citation_safe =
            sanitize_unlinked_evidence_citations(&normalized_citations, &allowed_ids);
        let cleaned = clean_generated_draft_for_workbench(&citation_safe);
        if let Some(issue) = generated_draft_quality_issue(
            &cleaned,
            &evidence_context,
            evidence_items.len(),
            &format,
        ) {
            eprintln!(
                "Generated draft for lead {} replaced with source-bound fallback: {}",
                lead_id, issue
            );
            source_bound_fallback_draft(&lead_why, &evidence_items, &audience, &format)
        } else {
            cleaned
        }
    };

    let mut draft = build_persistable_draft(
        lead_id,
        &lead_why,
        story_type.as_deref(),
        disposition.as_deref(),
        novelty_score,
        recurrence_count,
        &generated_text,
        evidence_items.len(),
        &format,
    );

    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    let draft_id = db::insert_draft(&conn, &draft).map_err(|e| e.to_string())?;
    draft.id = Some(draft_id);
    db::get_draft(&conn, draft_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Draft was saved but could not be reloaded.".to_string())
}

#[cfg(test)]
mod generate_and_save_draft_ipc_tests {
    use super::*;
    use crate::core::llm::LlmClient;
    use crate::test_support::TestEnv;
    use rusqlite::Connection;
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    struct FakeDraftLlm {
        evidence_id: i32,
        calls: Mutex<Vec<(String, String)>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for FakeDraftLlm {
        async fn call(&self, model: &str, prompt: &str, _system: &str) -> Result<String, String> {
            self.calls
                .lock()
                .unwrap()
                .push((model.to_string(), prompt.to_string()));
            Ok(format!(
                "Headline: Council approves a street-safety grant\n\nAccording to the linked source, the city council approved a street-safety grant for the coming fiscal year [Source](evidence:{}).",
                self.evidence_id
            ))
        }
    }

    #[tokio::test]
    async fn registered_ipc_command_generates_and_reloads_linked_draft() {
        let mut env = TestEnv::new();
        let app_data = tempdir().unwrap();
        env.set(
            crate::core::app_paths::APP_DATA_OVERRIDE_ENV,
            app_data.path(),
        );
        env.set("CIVICNEWS_TEST_INSTALLED_MODELS", "test-model:latest");

        let mut connection = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut connection).unwrap();
        let source_id = crate::core::db::insert_source(
            &connection,
            &Source {
                id: None,
                name: "City council agenda".to_string(),
                url: "https://city.example/agenda".to_string(),
                r#type: "primary_record".to_string(),
                status: "online".to_string(),
                tier: "official_record".to_string(),
                last_success_at: None,
                last_failed_at: None,
                last_scraped: None,
            },
        )
        .unwrap();
        let evidence_id = crate::core::db::insert_evidence_item(
            &connection,
            &EvidenceItem {
                id: None,
                source_id,
                url: Some("https://city.example/agenda#grant".to_string()),
                fetched_at: "2026-07-09T12:00:00Z".to_string(),
                excerpt:
                    "The city council approved a street-safety grant for the coming fiscal year."
                        .to_string(),
                content_hash: "test-hash".to_string(),
                entities: "[]".to_string(),
            },
        )
        .unwrap();
        let lead_id = crate::core::db::insert_lead(
            &connection,
            &Lead {
                id: None,
                detector_name: "Test detector".to_string(),
                why: "Council approves a street-safety grant".to_string(),
                confidence: "high".to_string(),
                risk_level: "low".to_string(),
                confirmation_checklist: "[]".to_string(),
                from_scan_lead_id: None,
                story_type: Some("story".to_string()),
                disposition: Some("ready_to_draft".to_string()),
                novelty_score: Some(5),
                novelty_reason: Some("Current approval".to_string()),
                recurrence_count: Some(0),
                recurrence_note: None,
                created_at: "2026-07-09T12:00:00Z".to_string(),
            },
            &[evidence_id],
        )
        .unwrap();
        connection
            .execute(
                "INSERT INTO settings (key, value) VALUES ('model.selected', 'test-model:latest')",
                [],
            )
            .unwrap();

        let db = Arc::new(Mutex::new(connection));
        let fake_llm = Arc::new(FakeDraftLlm {
            evidence_id,
            calls: Mutex::new(Vec::new()),
        });
        let app = tauri::test::mock_builder()
            .manage(db.clone())
            .manage(fake_llm.clone() as Arc<dyn LlmClient>)
            .invoke_handler(tauri::generate_handler![generate_and_save_draft])
            .build(tauri::test::mock_context(tauri::test::noop_assets()))
            .unwrap();
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        let bundled_origin = if cfg!(windows) {
            "http://tauri.localhost"
        } else {
            "tauri://localhost"
        };
        let request = tauri::webview::InvokeRequest {
            cmd: "generate_and_save_draft".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: bundled_origin.parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(serde_json::json!({
                "leadId": lead_id,
                "format": "brief",
            })),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        };

        let response = tauri::test::get_ipc_response(&webview, request).unwrap();
        let draft: Draft = response.deserialize().unwrap();
        assert_eq!(draft.lead_id, Some(lead_id));
        assert_eq!(draft.status, "draft_generated");
        assert!(draft.content.contains(&format!("evidence:{evidence_id}")));

        let persisted = crate::core::db::get_draft(&db.lock().unwrap(), draft.id.unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(persisted.content, draft.content);
        assert_eq!(persisted.title, "Council approves a street-safety grant");
        let calls = fake_llm.calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "test-model:latest");
        assert!(calls[0]
            .1
            .contains(&format!("Evidence Citation ID: {evidence_id}")));

        drop(webview);
        drop(app);
    }
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

/// GG (editor-editable guardrails): return the newsroom's guardrail word lists,
/// seeded with the built-in defaults when unset.
#[tauri::command]
pub fn get_guardrail_terms(
    db: tauri::State<'_, DbConn>,
) -> Result<guardrails::GuardrailConfig, String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    Ok(guardrails::load_guardrail_config(&conn))
}

/// GG (editor-editable guardrails): persist the newsroom's guardrail word lists.
/// `blocking` is the subset of words that become high-concern warnings; everything else
/// only warns.
#[tauri::command]
pub fn set_guardrail_terms(
    db: tauri::State<'_, DbConn>,
    config: guardrails::GuardrailConfig,
) -> Result<(), String> {
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    guardrails::save_guardrail_config(&conn, &config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn publish(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    output_dir: String,
) -> Result<compiler::CompileStaticSiteResult, String> {
    let output_dir = validate_app_write_destination(&app, &output_dir)?;
    let profile_json = {
        let path = get_config_path(&app)?;
        if path.exists() {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            r#"{"site_title": "My Local Publication", "site_subtitle": "Local news and community information.", "about_text": "", "ethics_text": "", "how_we_report_text": "", "organization_type": "single_person"}"#.to_string()
        }
    };

    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    ensure_publishable_story_exists(&conn)?;
    compiler::compile_static_site(&conn, &output_dir.to_string_lossy(), &profile_json)
        .map_err(|e| e.to_string())
}

fn ensure_publishable_story_exists(conn: &rusqlite::Connection) -> Result<(), String> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*)
             FROM drafts
             WHERE status IN ('published', 'corrected', 'ready_to_publish')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("Could not inspect approved stories before publishing: {e}"))?;
    if count <= 0 {
        return Err(
            "No approved stories are ready to publish. Approve at least one story or brief in Workbench before compiling a public package."
                .to_string(),
        );
    }
    Ok(())
}

#[tauri::command]
pub fn record_publish_destination(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    output_dir: String,
    provider: String,
    published_url: String,
    deployment_id: Option<String>,
) -> Result<compiler::CompileStaticSiteResult, String> {
    let output_dir = validate_app_write_destination(&app, &output_dir)?;
    let provider = provider.trim();
    let _connector = publisher::publisher_for(provider)?;
    let normalized_url = publisher::validate_public_url(&published_url)?;
    let output_path = publisher::validate_publish_artifacts(&output_dir.to_string_lossy())?;

    let result = compiler::record_publish_destination_files(
        &output_path,
        provider,
        &normalized_url,
        deployment_id.as_deref(),
    )
    .map_err(|e| e.to_string())?;

    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    db::update_latest_publish_run_destination(
        &conn,
        &result.output_dir,
        provider,
        &normalized_url,
        deployment_id.as_deref(),
    )
    .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn save_publisher_config(
    db: tauri::State<'_, DbConn>,
    config: publisher::PublisherConfigInput,
) -> Result<publisher::PublisherConfig, String> {
    let provider = publisher::PublisherProvider::from_str(config.provider.trim())
        .ok_or_else(|| "Unsupported publishing provider.".to_string())?;
    let provider = provider.as_str().to_string();
    let clear_credential = config.clear_credential;
    let pending_credential = config
        .credential
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let mut sanitized = publisher::sanitize_config(config)?;
    if pending_credential.is_some() {
        sanitized.has_credential = true;
    } else if clear_credential {
        sanitized.has_credential = false;
    } else {
        sanitized.has_credential = publisher::has_provider_credential(&sanitized.provider);
    }
    let connector = publisher::publisher_for(&sanitized.provider)?;
    connector.validate_config(&sanitized)?;

    if clear_credential {
        publisher::delete_provider_credential(&provider)?;
    }
    let wrote_pending_credential = if let Some(credential) = pending_credential {
        publisher::set_provider_credential(&provider, &credential)?;
        true
    } else {
        false
    };
    sanitized.has_credential = publisher::has_provider_credential(&sanitized.provider);
    let value = serde_json::to_string(&sanitized).map_err(|e| e.to_string())?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    if let Err(e) = conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![
            publisher::provider_config_setting_key(&sanitized.provider),
            value
        ],
    ) {
        if wrote_pending_credential {
            let _ = publisher::delete_provider_credential(&provider);
        }
        return Err(e.to_string());
    }
    Ok(sanitized)
}

#[tauri::command]
pub fn get_publisher_config(
    db: tauri::State<'_, DbConn>,
    provider: String,
) -> Result<Option<publisher::PublisherConfig>, String> {
    let provider = publisher::PublisherProvider::from_str(provider.trim())
        .ok_or_else(|| "Unsupported publishing provider.".to_string())?;
    let key = publisher::provider_config_setting_key(provider.as_str());
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1",
            rusqlite::params![key],
            |row| row.get(0),
        )
        .ok();
    let Some(value) = value else {
        return Ok(None);
    };
    let mut config: publisher::PublisherConfig =
        serde_json::from_str(&value).map_err(|e| e.to_string())?;
    config.has_credential = publisher::has_provider_credential(provider.as_str());
    Ok(Some(config))
}

#[tauri::command]
pub async fn test_publisher_connection(
    db: tauri::State<'_, DbConn>,
    provider: String,
) -> Result<publisher::PublisherTestResult, String> {
    let config =
        get_publisher_config(db, provider.clone())?.unwrap_or(publisher::PublisherConfig {
            provider: provider.trim().to_string(),
            display_name: provider.trim().replace('_', " "),
            site_url: None,
            project_hint: None,
            site_id: None,
            account_id: None,
            repo: None,
            branch: None,
            path_prefix: None,
            username: None,
            has_credential: publisher::has_provider_credential(provider.trim()),
        });
    let connector = publisher::publisher_for(&config.provider)?;
    Ok(connector.test_connection(&config).await)
}

#[tauri::command]
pub async fn publish_with_connector(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    output_dir: String,
    provider: String,
    published_url: Option<String>,
    deployment_id: Option<String>,
) -> Result<compiler::CompileStaticSiteResult, String> {
    let provider = provider.trim().to_string();
    let mut config = get_publisher_config(db.clone(), provider.clone())?
        .or_else(|| default_publish_config_for_unsaved_connector(&provider, &output_dir))
        .ok_or_else(|| "Save this publisher connector before publishing.".to_string())?;
    if provider == publisher::PublisherProvider::HereNow.as_str()
        && config.display_name.trim().is_empty()
    {
        if let Some(default_config) =
            default_publish_config_for_unsaved_connector(&provider, &output_dir)
        {
            config.display_name = default_config.display_name;
        }
    }
    let connector = publisher::publisher_for(&provider)?;
    let request = publisher::PublisherPublishRequest {
        output_dir: output_dir.clone(),
        provider,
        published_url,
        deployment_id,
    };
    let published = connector.publish_folder(&config, &request).await?;
    record_publish_destination(
        db,
        app,
        output_dir,
        published.provider,
        published.published_url,
        published.deployment_id,
    )
}

fn default_publish_config_for_unsaved_connector(
    provider: &str,
    output_dir: &str,
) -> Option<publisher::PublisherConfig> {
    if provider != publisher::PublisherProvider::HereNow.as_str() {
        return None;
    }
    let display_name = output_dir
        .split(['/', '\\'])
        .next_back()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("Civic Newspaper preview")
        .to_string();
    Some(publisher::PublisherConfig {
        provider: provider.to_string(),
        display_name,
        site_url: None,
        project_hint: Some("Temporary civic newspaper preview.".to_string()),
        site_id: None,
        account_id: None,
        repo: None,
        branch: None,
        path_prefix: None,
        username: None,
        has_credential: publisher::has_provider_credential(provider),
    })
}

#[cfg(test)]
mod publish_connector_tests {
    use super::*;

    #[test]
    fn unsaved_here_now_connector_gets_anonymous_preview_config() {
        let config = default_publish_config_for_unsaved_connector(
            "here_now",
            r"C:\Users\tester\Documents\The Longmont Ledger",
        )
        .expect("here.now should support anonymous publishing without a saved config");

        assert_eq!(config.provider, "here_now");
        assert_eq!(config.display_name, "The Longmont Ledger");
        assert!(config.site_id.is_none());
        assert!(config.project_hint.unwrap().contains("Temporary"));
    }

    #[test]
    fn unsaved_credential_connectors_still_require_saved_config() {
        assert!(default_publish_config_for_unsaved_connector("netlify", r"C:\site").is_none());
        assert!(default_publish_config_for_unsaved_connector("github_pages", r"C:\site").is_none());
    }
}

#[tauri::command]
pub fn list_publish_history(db: tauri::State<'_, DbConn>) -> Result<Vec<db::PublishRun>, String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    db::list_publish_runs(&conn).map_err(|e| e.to_string())
}

fn valid_subscriber_email(email: &str) -> bool {
    let email = email.trim();
    email.contains('@') && email.contains('.') && !email.chars().any(char::is_whitespace)
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut chars = line.chars().peekable();
    let mut in_quotes = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                current.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                values.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    values.push(current.trim().to_string());
    values
}

#[tauri::command]
pub fn list_subscribers(db: tauri::State<'_, DbConn>) -> Result<Vec<Subscriber>, String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    db::list_subscribers(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_subscriber(
    db: tauri::State<'_, DbConn>,
    email: String,
    name: Option<String>,
) -> Result<i32, String> {
    let email = email.trim().to_ascii_lowercase();
    if !valid_subscriber_email(&email) {
        return Err("Enter a valid email address.".to_string());
    }
    let clean_name = name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    db::upsert_subscriber(&conn, &email, clean_name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_subscriber(db: tauri::State<'_, DbConn>, id: i32) -> Result<(), String> {
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    db::delete_subscriber(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_subscribers_csv(db: tauri::State<'_, DbConn>, path: String) -> Result<usize, String> {
    let text = std::fs::read_to_string(path.trim()).map_err(|e| e.to_string())?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let mut imported = 0usize;
    for (idx, line) in text.lines().enumerate() {
        let fields = parse_csv_line(line);
        if fields.is_empty() {
            continue;
        }
        let email = fields[0].trim().to_ascii_lowercase();
        if idx == 0 && email == "email" {
            continue;
        }
        if !valid_subscriber_email(&email) {
            continue;
        }
        let name = fields
            .get(1)
            .map(String::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        db::upsert_subscriber(&conn, &email, name).map_err(|e| e.to_string())?;
        imported += 1;
    }
    Ok(imported)
}

#[tauri::command]
pub fn export_subscribers_csv(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    let path = validate_app_write_destination(&app, &path)?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let subscribers = db::list_subscribers(&conn).map_err(|e| e.to_string())?;
    let mut out = String::from("email,name,status\n");
    for subscriber in subscribers {
        out.push_str(&format!(
            "{},{},{}\n",
            csv_escape(&subscriber.email),
            csv_escape(subscriber.name.as_deref().unwrap_or("")),
            csv_escape(&subscriber.status)
        ));
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn publish_artifact_path(
    output_dir: &str,
    relative_path: &str,
) -> Result<std::path::PathBuf, String> {
    let root = std::fs::canonicalize(output_dir.trim()).map_err(|e| e.to_string())?;
    let relative = std::path::Path::new(relative_path.trim());
    if relative.is_absolute()
        || relative
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Invalid publish artifact path.".to_string());
    }
    let file = std::fs::canonicalize(root.join(relative)).map_err(|e| e.to_string())?;
    if !file.starts_with(&root) {
        return Err("Publish artifact path is outside the output folder.".to_string());
    }
    Ok(file)
}

#[tauri::command]
pub fn read_publish_artifact(output_dir: String, relative_path: String) -> Result<String, String> {
    let path = publish_artifact_path(&output_dir, &relative_path)?;
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_issue_email(
    db: tauri::State<'_, DbConn>,
    app: tauri::AppHandle,
    output_dir: String,
    path: String,
) -> Result<(), String> {
    let path = validate_app_write_destination(&app, &path)?;
    let newsletter_path = publish_artifact_path(&output_dir, "newsletter.md")?;
    let body = std::fs::read_to_string(newsletter_path).map_err(|e| e.to_string())?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let public_url = db::list_publish_runs(&conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|run| run.output_path == output_dir && run.published_url.is_some())
        .and_then(|run| run.published_url);
    let body = prepare_issue_email_body(&body, public_url.as_deref());
    let active_count = db::list_subscribers(&conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|subscriber| subscriber.status == "active")
        .count();
    let subject = format!(
        "Subject: Your Civic Desk issue is ready\nPreheader: {} subscriber(s) on the local list\n\n{}",
        active_count, body
    );
    std::fs::write(path, subject).map_err(|e| e.to_string())
}

pub(crate) fn prepare_issue_email_body(body: &str, public_url: Option<&str>) -> String {
    match public_url.map(str::trim).filter(|url| !url.is_empty()) {
        Some(url) => body.replace(
            "https://YOUR-PUBLIC-SITE.example",
            url.trim_end_matches('/'),
        ),
        None => format!(
            "> Before sending: replace `https://YOUR-PUBLIC-SITE.example` with your published site URL.\n\n{body}"
        ),
    }
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
pub async fn backup_save(app: tauri::AppHandle, dest_path: String) -> Result<(), String> {
    let dest_path = validate_app_write_destination(&app, &dest_path)?;
    let live_db_path = db::get_app_db_path(&app).map_err(|e| e.to_string())?;
    let live_db_path = live_db_path.to_string_lossy().into_owned();
    tauri::async_runtime::spawn_blocking(move || {
        let conn = db::open_conn(&live_db_path).map_err(|e| e.to_string())?;
        backups::save_backup(&conn, &dest_path.to_string_lossy()).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Backup worker failed: {e}"))?
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

pub struct RuntimeInstallEventSink<R: tauri::Runtime> {
    app: tauri::AppHandle<R>,
}

impl<R: tauri::Runtime> crate::core::llm::RuntimeInstallSink for RuntimeInstallEventSink<R> {
    fn progress(&self, payload: crate::core::llm::RuntimeInstallProgress) {
        let _ = self.app.emit("ollama-runtime-install-progress", payload);
    }
}

#[tauri::command]
pub async fn install_ollama_runtime<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    let sink = std::sync::Arc::new(RuntimeInstallEventSink { app: app.clone() });
    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::Builder::new()
        .name("civicdesk-ollama-runtime-install".to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| format!("Could not prepare local AI installer runtime: {e}"))
                .and_then(|runtime| {
                    runtime.block_on(crate::core::llm::install_windows_ollama_runtime(app, sink))
                });
            let _ = tx.send(result);
        })
        .map_err(|e| format!("Could not start local AI installer thread: {e}"))?;
    rx.await
        .map_err(|_| "Local AI installer thread stopped before reporting a result.".to_string())?
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
        keep_main_window_visible(&self.app, false);
        let _ = self.app.emit("ollama-pull-progress", payload);
    }
    fn complete(&self) {
        use tauri::Emitter;
        keep_main_window_visible(&self.app, true);
        let _ = self.app.emit("ollama-pull-complete", ());
    }
    fn error(&self, message: String) {
        use tauri::Emitter;
        keep_main_window_visible(&self.app, true);
        let _ = self.app.emit("ollama-pull-error", message);
    }
}

#[tauri::command]
pub async fn pull_ollama_model<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    model_id: String,
) -> Result<(), String> {
    keep_main_window_visible(&app, true);
    if !is_allowed_ollama_pull_model(&model_id) {
        return Err(format!(
            "Model `{}` is not in this CivicNewspaper release's curated local-AI model list.",
            model_id.trim()
        ));
    }
    let sink = std::sync::Arc::new(AppHandlePullSink { app });
    crate::core::llm::run_ollama_pull(model_id, &crate::core::llm::ollama_base_url(), sink).await
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
    discovery::discover_all_sources(&city, &state).await.map_err(|e| {
        eprintln!("Source discovery failed for {city}, {state}: {e}");
        "Source discovery could not run. Check your connection, then try again or add sources manually."
            .to_string()
    })
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

    let base_url = llm::ollama_base_url();
    match client.get(format!("{base_url}/api/tags")).send().await {
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
                if let Ok(v_resp) = client.get(format!("{base_url}/api/version")).send().await {
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
    crate::core::app_paths::validate_write_destination(
        &[app_data_dir, download_dir],
        std::path::Path::new(path),
    )
    .map_err(|_| "Path is outside allowed directories".to_string())
}

fn validate_app_write_destination(
    app: &tauri::AppHandle,
    requested: &str,
) -> Result<std::path::PathBuf, String> {
    let app_data = crate::core::app_paths::app_data_dir(app)?;
    let downloads = app.path().download_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&downloads).map_err(|e| e.to_string())?;
    crate::core::app_paths::validate_write_destination(
        &[app_data, downloads],
        std::path::Path::new(requested.trim()),
    )
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
    let app_data = crate::core::app_paths::app_data_dir(&app_handle).unwrap_or_default();
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
    let progress_app = app.clone();
    crate::core::daily_scan::run_daily_scan_fetching_sources_with_progress(
        &db,
        &llm_client,
        &prompt_template,
        &city,
        &state,
        since_hours,
        move |progress| {
            let _ = progress_app.emit("daily-scan-progress", progress);
        },
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

#[tauri::command]
pub async fn press_freedom_legal_review<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, DbConn>,
    draft_id: i32,
) -> Result<String, String> {
    let (draft, evidence_items) = {
        let conn = db
            .lock()
            .map_err(|_| "Failed to lock database".to_string())?;
        let draft = db::get_draft(&conn, draft_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Draft {} was not found.", draft_id))?;
        let evidence_items = match draft.lead_id {
            Some(lead_id) => db::get_evidence_by_lead(&conn, lead_id).map_err(|e| e.to_string())?,
            None => Vec::new(),
        };
        (draft, evidence_items)
    };

    let evidence_context = if evidence_items.is_empty() {
        "No linked evidence is attached to this draft.".to_string()
    } else {
        evidence_items
            .iter()
            .map(|item| {
                format!(
                    "Evidence ID: {}\nURL: {}\nExcerpt: {}\n",
                    item.id.unwrap_or(0),
                    item.url.as_deref().unwrap_or("unknown"),
                    item.excerpt
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let model = get_selected_model_or_fallback(&db).await;
    let llm_client = app
        .state::<std::sync::Arc<dyn crate::core::llm::LlmClient>>()
        .inner()
        .clone();
    crate::core::llm::press_freedom_legal_review(
        &llm_client,
        &model,
        &draft.title,
        &draft.content,
        &draft.format,
        &evidence_context,
    )
    .await
}

#[cfg(test)]
mod source_import_extraction_tests {
    use super::{extract_source_import_text, import_logo_asset, SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "civicnews-source-import-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_zip(path: &Path, entries: &[(&str, &str)]) {
        let file = std::fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        for (name, content) in entries {
            zip.start_file(name, options).unwrap();
            zip.write_all(content.as_bytes()).unwrap();
        }
        zip.finish().unwrap();
    }

    #[test]
    fn extracts_plain_text_source_list() {
        let dir = temp_dir("txt");
        let path = dir.join("sources.txt");
        std::fs::write(
            &path,
            "Denver Council, https://www.denvergov.org/Government/Agencies-Departments-Offices/Agencies-Departments-Offices-Directory/City-Council\n",
        )
        .unwrap();

        let text = extract_source_import_text(path.to_string_lossy().to_string()).unwrap();

        assert!(text.contains("Denver Council"));
        assert!(text.contains("https://www.denvergov.org/"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn imports_small_logo_as_data_url_and_rejects_svg() {
        let dir = temp_dir("logo");
        let png = dir.join("logo.png");
        std::fs::write(&png, [137u8, 80, 78, 71, 13, 10, 26, 10]).unwrap();
        let data_url = import_logo_asset(png.to_string_lossy().to_string()).unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));

        let svg = dir.join("logo.svg");
        std::fs::write(&svg, "<svg><script>alert(1)</script></svg>").unwrap();
        let err = import_logo_asset(svg.to_string_lossy().to_string()).unwrap_err();
        assert!(err.contains("Unsupported logo type"));
    }

    #[test]
    fn extracts_docx_source_list_text() {
        let dir = temp_dir("docx");
        let path = dir.join("sources.docx");
        write_zip(
            &path,
            &[(
                "word/document.xml",
                r#"<w:document><w:body><w:p><w:r><w:t>Denver Agendas https://denver.legistar.com/Calendar.aspx</w:t></w:r></w:p><w:p><w:r><w:t>Denver Maps https://www.denvergov.org/maps</w:t></w:r></w:p></w:body></w:document>"#,
            )],
        );

        let text = extract_source_import_text(path.to_string_lossy().to_string()).unwrap();

        assert!(text.contains("Denver Agendas"));
        assert!(text.contains("https://denver.legistar.com/Calendar.aspx"));
        assert!(text.contains("Denver Maps"));
        assert!(text.lines().count() >= 2);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn extracts_xlsx_shared_strings_and_sheet_text() {
        let dir = temp_dir("xlsx");
        let path = dir.join("sources.xlsx");
        write_zip(
            &path,
            &[
                (
                    "xl/sharedStrings.xml",
                    r#"<sst><si><t>Denver Open Data</t></si><si><t>https://www.denvergov.org/opendata</t></si><si><t>Denver Maps</t></si><si><t>https://www.denvergov.org/maps</t></si></sst>"#,
                ),
                (
                    "xl/worksheets/sheet1.xml",
                    r#"<worksheet><sheetData><row><c t="s"><v>0</v></c><c t="s"><v>1</v></c></row><row><c t="s"><v>2</v></c><c t="s"><v>3</v></c></row></sheetData></worksheet>"#,
                ),
            ],
        );

        let text = extract_source_import_text(path.to_string_lossy().to_string()).unwrap();

        assert!(text.contains("Denver Open Data"));
        assert!(text.contains("https://www.denvergov.org/opendata"));
        assert!(text.contains("Denver Maps"));
        assert!(text.contains("https://www.denvergov.org/maps"));
        assert!(text.lines().count() >= 2);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn invalid_pdf_source_lists_return_actionable_guidance() {
        let dir = temp_dir("pdf");
        let path = dir.join("sources.pdf");
        std::fs::write(&path, "%PDF-1.4").unwrap();

        let err = extract_source_import_text(path.to_string_lossy().to_string()).unwrap_err();

        assert!(err.contains("PDF source-list import is disabled"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn oversized_source_list_file_is_rejected_before_extraction() {
        let dir = temp_dir("oversized");
        let path = dir.join("too-large.csv");
        let file = std::fs::File::create(&path).unwrap();
        file.set_len(25 * 1024 * 1024 + 1).unwrap();

        let err = extract_source_import_text(path.to_string_lossy().to_string()).unwrap_err();

        assert!(
            err.contains("too large"),
            "oversized source-list files must fail before extraction, got: {err}"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn oversized_docx_xml_entry_is_rejected_after_decompression() {
        let dir = temp_dir("docx-zip-bomb");
        let path = dir.join("sources.docx");
        let large_xml = format!(
            "<w:document><w:body><w:t>{}</w:t></w:body></w:document>",
            "x".repeat(SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES as usize + 1)
        );
        write_zip(&path, &[("word/document.xml", &large_xml)]);

        let err = extract_source_import_text(path.to_string_lossy().to_string()).unwrap_err();
        assert!(
            err.contains("too large after decompression"),
            "oversized DOCX XML entry must be rejected, got: {err}"
        );
    }

    #[test]
    fn oversized_xlsx_xml_entry_is_rejected_after_decompression() {
        let dir = temp_dir("xlsx-zip-bomb");
        let path = dir.join("sources.xlsx");
        let large_xml = format!(
            "<sst><si><t>{}</t></si></sst>",
            "x".repeat(SOURCE_IMPORT_MAX_ZIP_ENTRY_BYTES as usize + 1)
        );
        write_zip(&path, &[("xl/sharedStrings.xml", &large_xml)]);

        let err = extract_source_import_text(path.to_string_lossy().to_string()).unwrap_err();
        assert!(
            err.contains("too large after decompression"),
            "oversized XLSX XML entry must be rejected, got: {err}"
        );
    }

    #[test]
    fn local_source_import_fixtures_extract_reviewable_text() {
        let fixture_dir = std::env::var("CIVICNEWS_IMPORT_FIXTURE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .expect("src-tauri should have a repo parent")
                    .join("tests")
                    .join("fixtures")
                    .join("source-import")
            });
        assert!(
            fixture_dir.join("colorado-source-list-clean.csv").is_file(),
            "source import fixture folder not found at {}; set CIVICNEWS_IMPORT_FIXTURE_DIR to the fixture folder",
            fixture_dir.display()
        );
        let output_dir = std::env::var("CIVICNEWS_IMPORT_EXTRACTED_DIR")
            .ok()
            .map(PathBuf::from);
        if let Some(dir) = &output_dir {
            std::fs::create_dir_all(dir).unwrap();
        }

        let cases = [
            ("colorado-source-list-clean.csv", 35usize, true),
            ("colorado-source-list-messy.xlsx", 25usize, true),
            ("colorado-source-list-human-notes.txt", 25usize, true),
            ("colorado-source-list-briefing.docx", 35usize, true),
            ("colorado-source-list-exported.pdf", 0usize, false),
            ("colorado-source-list-edge-cases.xlsx", 15usize, true),
            ("colorado-source-list-scanned-style.pdf", 0usize, false),
        ];

        let url_re = regex::Regex::new(r#"https?://[^\s<>"')\]]+"#).unwrap();
        let mut report = Vec::new();
        for (name, min_urls, should_extract) in cases {
            let path = fixture_dir.join(name);
            let result = extract_source_import_text(path.to_string_lossy().to_string());
            match (result, should_extract) {
                (Ok(text), true) => {
                    let count = url_re.find_iter(&text).count();
                    if let Some(dir) = &output_dir {
                        let out_name = format!("{name}.txt");
                        std::fs::write(dir.join(out_name), &text).unwrap();
                    }
                    report.push(format!("{name}: extracted {count} URL-like strings"));
                    assert!(
                        count >= min_urls,
                        "{name} extracted {count} URL-like strings; expected at least {min_urls}"
                    );
                }
                (Err(err), false) => {
                    report.push(format!("{name}: expected extraction failure: {err}"));
                    assert!(
                        err.contains("PDF source-list import is disabled"),
                        "{name} should fail with public-beta PDF-disabled guidance, got: {err}"
                    );
                }
                (Ok(text), false) => {
                    panic!(
                        "{name} unexpectedly extracted text from scanned-style fixture: {} chars",
                        text.len()
                    );
                }
                (Err(err), true) => {
                    panic!("{name} should extract reviewable text, got error: {err}");
                }
            }
        }
        eprintln!("{}", report.join("\n"));
    }
}
