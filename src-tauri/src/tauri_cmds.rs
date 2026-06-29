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
use crate::core::verification::{self, VerificationQueueSnapshot};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Read;
use tauri::Emitter;
use tauri::Manager;

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
    "Brighton".to_string()
}
fn default_state() -> String {
    "CO".to_string()
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
        city: "Brighton".to_string(),
        state: "CO".to_string(),
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
    let default_m = "qwen2.5:7b".to_string();
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
                    let names: Vec<String> = tags.models.iter().map(|m| m.name.clone()).collect();
                    // QA-mn1: prefer the default model only if it is EXACTLY
                    // installed (with :latest normalization), then fall back to
                    // a known scan-capable family, then the first model.
                    if model_is_installed(&default_m, &names) {
                        model = default_m;
                    } else if let Some(m) = tags.models.iter().find(|m| {
                        // Match by model FAMILY on the tag's base name (the part
                        // before ':'), not a loose whole-string contains.
                        let base = m.name.split(':').next().unwrap_or("");
                        base == "qwen2.5" || base == "llama3.2"
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
    add_source_inner(&db, name, url, r#type, tier)
}

fn normalize_source_url(url: &str) -> String {
    let mut value = url
        .trim()
        .trim_matches(|ch| matches!(ch, '"' | '\'' | '`'))
        .trim()
        .trim_end_matches(|ch| matches!(ch, '.' | ',' | ';' | ':' | '!' | '?'))
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
            .trim_end_matches(|ch| matches!(ch, '.' | ',' | ';' | ':' | '!' | '?'))
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
pub fn open_local_path(path: String) -> Result<(), String> {
    let path = std::path::PathBuf::from(path.trim());
    if path.as_os_str().is_empty() {
        return Err("No path was provided".to_string());
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
) -> Result<Option<String>, String> {
    match archive.by_name(name) {
        Ok(mut file) => {
            let mut text = String::new();
            file.read_to_string(&mut text)
                .map_err(|e| format!("Could not read {name}: {e}"))?;
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
    let xml = read_zip_entry_text(&mut archive, "word/document.xml")?
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
    let shared_strings = read_zip_entry_text(&mut archive, "xl/sharedStrings.xml")?
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
        if let Some(sheet) = read_zip_entry_text(&mut archive, &sheet_name)? {
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

fn extract_pdf_text(path: &std::path::Path) -> Result<String, String> {
    let text = pdf_extract::extract_text(path)
        .map_err(|e| format!("Could not extract readable text from this PDF: {e}"))?;
    let normalized = normalize_extracted_text(&text);
    if normalized.trim().is_empty() {
        Err(
            "No readable text was found in this PDF. It may be scanned image-only and require OCR."
                .to_string(),
        )
    } else {
        Ok(normalized)
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
    if metadata.len() > 25 * 1024 * 1024 {
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
        "pdf" => extract_pdf_text(&path),
        _ => Err("Unsupported source-list file type. Use CSV, TSV, TXT, DOCX, XLSX, PDF, or paste URLs directly.".to_string()),
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
pub fn get_community_profile(app: tauri::AppHandle) -> Result<CommunityProfile, String> {
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
    profile: CommunityProfile,
) -> Result<(), String> {
    let path = get_config_path(&app)?;
    let content = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
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
/// publish/hold/kill decision; software only warns and records.
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
    Ok(())
}

#[tauri::command]
pub fn story_decision(
    db: tauri::State<'_, DbConn>,
    id: i32,
    decision: String,
    override_reason: Option<String>,
) -> Result<(), String> {
    const PUBLISH_STATES: [&str; 3] = ["ready_to_publish", "published", "corrected"];
    let conn = db
        .lock()
        .map_err(|_| "Failed to lock database".to_string())?;
    if PUBLISH_STATES.contains(&decision.as_str()) {
        if let Some(draft) = db::get_draft(&conn, id).map_err(|e| e.to_string())? {
            if draft.status == "killed" {
                return Err(
                    "This story is killed. Move it back to Hold before approving it for publish."
                        .to_string(),
                );
            }
        }
    }
    enforce_publish_gate(&conn, id, &decision, override_reason.as_deref())?;
    db::update_draft_status(&conn, id, &decision).map_err(|e| e.to_string())
}

fn sanitize_unlinked_evidence_citations(text: &str, allowed_ids: &HashSet<i32>) -> String {
    if !text.contains("evidence:") {
        return text.to_string();
    }

    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    let mut removed_ids = Vec::new();

    while let Some(pos) = rest.find("evidence:") {
        output.push_str(&rest[..pos]);
        let after_prefix = &rest[pos + "evidence:".len()..];
        let digit_len = after_prefix
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .map(char::len_utf8)
            .sum::<usize>();

        if digit_len == 0 {
            output.push_str("evidence:");
            rest = after_prefix;
            continue;
        }

        let id_text = &after_prefix[..digit_len];
        let id = id_text.parse::<i32>().unwrap_or_default();
        if allowed_ids.contains(&id) {
            output.push_str("evidence:");
            output.push_str(id_text);
        } else {
            removed_ids.push(id);
            output.push_str("unlinked-evidence-");
            output.push_str(id_text);
        }
        rest = &after_prefix[digit_len..];
    }

    output.push_str(rest);
    removed_ids.sort_unstable();
    removed_ids.dedup();

    if removed_ids.is_empty() {
        output
    } else {
        format!(
            "{output}\n\n> Editor note: The AI draft referenced unlinked evidence ID(s) {}. Those citation markers were disabled automatically. Verify the claim against the linked sources before publishing.",
            removed_ids
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
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
        assert!(sanitized.contains("Editor note"));
        assert!(sanitized.contains("224"));
    }

    #[test]
    fn disables_all_model_citations_when_no_evidence_is_linked() {
        let allowed = HashSet::new();
        let draft = "This unlinked draft cites [a source](evidence:12).";

        let sanitized = sanitize_unlinked_evidence_citations(draft, &allowed);

        assert!(!sanitized.contains("](evidence:12)"));
        assert!(sanitized.contains("unlinked-evidence-12"));
        assert!(sanitized.contains("Editor note"));
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

    let mut evidence_context = String::new();
    for item in &evidence_items {
        let item_id = item.id.unwrap_or(0);
        evidence_context.push_str(&format!(
            "Evidence Citation ID: {}\nExcerpt: {}\n\n",
            item_id, item.excerpt
        ));
    }

    let prompt = if evidence_items.is_empty() {
        format!(
            "Lead topic: {}\n\nNo source documents are attached to this lead yet. Please draft a '{}' working draft for an editor to review.\n\nWrite a short verification memo, not a finished article. Include: a headline, a one-paragraph summary of the lead, a clearly labeled 'Needs verification' section, and a 'Next reporting steps' section. Do not include evidence citations. Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details. If a detail is not in the lead topic, say it needs verification instead of filling it in.",
            lead_why, format
        )
    } else if evidence_items.len() < 2 {
        format!(
            "Lead topic: {}\n\nHere is the only attached source material:\n{}\nPlease draft a report in '{}' format.\n\nBecause there is only one linked source, write this as a brief or watchlist item, not a complete reported story. Use only the attached excerpt. Include: a headline, a 2-4 sentence factual brief, what is known, what remains unclear, and specific next reporting steps. Use only the attached Evidence Citation ID in citations like [Source](evidence:ID). Do not cite any other evidence ID. Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details. If the excerpt is thin, say so plainly instead of padding.",
            lead_why, evidence_context, format
        )
    } else {
        format!(
            "Lead topic: {}\n\nHere is the attached source material:\n{}\nPlease draft a report in '{}' format.\n\nWrite for Longmont residents. Include: a headline, a short nut graf, 3-5 factual paragraphs or brief sections, what is known, what remains unclear, and specific next reporting steps. Use only the listed Evidence Citation IDs in citations like [Source](evidence:ID). Do not cite any other evidence ID. Use a citation for every factual claim drawn from the source material. Do not invent dates, durations, dollar amounts, causes, officials, quotes, project history, impacts, community reaction, or technical details. If the evidence is thin, make that visible instead of padding. Any claim without an evidence citation must be framed as a question or verification task, not as fact.",
            lead_why, evidence_context, format
        )
    };

    let sys = system_prompt.unwrap_or_else(|| "You are an assistant for a local publication editor. You help prepare careful working drafts. Do not decide what is publishable; warn about uncertainty and leave final judgment to the human editor.".to_string());

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
    let draft = llm_client.call(&model, &prompt, &sys).await?;
    let allowed_ids = evidence_items
        .iter()
        .filter_map(|item| item.id)
        .collect::<HashSet<_>>();
    Ok(sanitize_unlinked_evidence_citations(&draft, &allowed_ids))
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
    let profile_json = {
        let path = get_config_path(&app)?;
        if path.exists() {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            r#"{"site_title": "My Local Publication", "site_subtitle": "Local news and community information.", "about_text": "", "ethics_text": "", "how_we_report_text": "", "organization_type": "single_person"}"#.to_string()
        }
    };

    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    compiler::compile_static_site(&conn, &output_dir, &profile_json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_publish_destination(
    db: tauri::State<'_, DbConn>,
    output_dir: String,
    provider: String,
    published_url: String,
    deployment_id: Option<String>,
) -> Result<compiler::CompileStaticSiteResult, String> {
    let provider = provider.trim();
    let _connector = publisher::publisher_for(provider)?;
    let normalized_url = publisher::validate_public_url(&published_url)?;
    let output_path = publisher::validate_publish_artifacts(&output_dir)?;

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
    if config.clear_credential {
        publisher::delete_provider_credential(&provider)?;
    }
    if let Some(credential) = config.credential.as_deref() {
        if !credential.trim().is_empty() {
            publisher::set_provider_credential(&provider, credential.trim())?;
        }
    }
    let mut sanitized = publisher::sanitize_config(config)?;
    sanitized.has_credential = publisher::has_provider_credential(&sanitized.provider);
    let value = serde_json::to_string(&sanitized).map_err(|e| e.to_string())?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![
            publisher::provider_config_setting_key(&sanitized.provider),
            value
        ],
    )
    .map_err(|e| e.to_string())?;
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
    let display_name = std::path::Path::new(output_dir)
        .file_name()
        .and_then(|name| name.to_str())
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
pub fn export_subscribers_csv(db: tauri::State<'_, DbConn>, path: String) -> Result<(), String> {
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
    std::fs::write(path.trim(), out).map_err(|e| e.to_string())
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
    output_dir: String,
    path: String,
) -> Result<(), String> {
    let newsletter_path = publish_artifact_path(&output_dir, "newsletter.md")?;
    let body = std::fs::read_to_string(newsletter_path).map_err(|e| e.to_string())?;
    let conn = db.lock().map_err(|_| "Failed to lock database")?;
    let active_count = db::list_subscribers(&conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|subscriber| subscriber.status == "active")
        .count();
    let subject = format!(
        "Subject: Your Civic Desk issue is ready\nPreheader: {} subscriber(s) on the local list\n\n{}",
        active_count, body
    );
    std::fs::write(path.trim(), subject).map_err(|e| e.to_string())
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
    use super::{extract_source_import_text, import_logo_asset};
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

        assert!(
            err.contains("Could not extract readable text") || err.contains("No readable text")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn extracts_text_based_pdf_source_list() {
        let dir = temp_dir("pdf-readable");
        let path = dir.join("sources.pdf");
        let stream =
            "BT\n/F1 12 Tf\n72 720 Td\n(Longmont Council https://www.longmontcolorado.gov/city-council) Tj\nET\n";
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_string(),
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
            format!("<< /Length {} >>\nstream\n{}endstream", stream.len(), stream),
        ];
        let mut pdf = String::from("%PDF-1.4\n");
        let mut offsets = vec![0usize];
        for (idx, object) in objects.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.push_str(&format!("{} 0 obj\n{}\nendobj\n", idx + 1, object));
        }
        let xref_start = pdf.len();
        pdf.push_str("xref\n0 6\n0000000000 65535 f \n");
        for offset in offsets.iter().skip(1) {
            pdf.push_str(&format!("{offset:010} 00000 n \n"));
        }
        pdf.push_str(&format!(
            "trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref_start}\n%%EOF"
        ));
        std::fs::write(&path, pdf).unwrap();

        let text = extract_source_import_text(path.to_string_lossy().to_string()).unwrap();

        assert!(text.contains("Longmont Council"));
        assert!(text.contains("https://www.longmontcolorado.gov/city-council"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    #[ignore = "local release-smoke fixture gate; set CIVICNEWS_IMPORT_FIXTURE_DIR"]
    fn local_source_import_fixtures_extract_reviewable_text() {
        let fixture_dir = std::env::var("CIVICNEWS_IMPORT_FIXTURE_DIR")
            .expect("set CIVICNEWS_IMPORT_FIXTURE_DIR to the fixture folder");
        let fixture_dir = PathBuf::from(fixture_dir);
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
            ("colorado-source-list-exported.pdf", 30usize, true),
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
                        err.contains("OCR") || err.contains("readable text"),
                        "{name} should fail with OCR/readable-text guidance, got: {err}"
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
