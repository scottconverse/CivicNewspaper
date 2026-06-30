use crate::core::db::DbConn;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::LazyLock;
use sysinfo::System;

static WINDOWS_USER_PATH_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?i)C:\\Users\\[^\\\s]+").unwrap());
static TOKEN_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)\b(token|password|secret|api[_-]?key)\s*[:=]\s*[^\s,;]+").unwrap()
});
static BEARER_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?i)bearer\s+[A-Za-z0-9._~+/-]+").unwrap());

#[derive(Serialize, Deserialize)]
pub struct Diagnostics {
    pub app_version: String,
    pub os_name: String,
    pub os_version: String,
    pub tauri_version: String,
    pub ollama_reachable: bool,
    pub ollama_models: Vec<String>,
    pub db_schema_version: i64,
    pub evidence_count: i64,
    pub leads_count: i64,
    pub drafts_count: i64,
    pub published_posts_count: i64,
    pub panic_log_tail: Vec<String>,
}

pub(crate) fn redact_diagnostic_line(line: &str) -> String {
    let redacted = WINDOWS_USER_PATH_RE.replace_all(line, r"C:\Users\[redacted]");
    let redacted = BEARER_RE.replace_all(&redacted, "Bearer [redacted]");
    TOKEN_RE.replace_all(&redacted, "$1=[redacted]").to_string()
}

pub async fn gather_diagnostics(db: &DbConn, app_data_dir: PathBuf) -> Result<Diagnostics, String> {
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let tauri_version = tauri::VERSION.to_string();

    let ollama_state = crate::tauri_cmds::ollama_health()
        .await
        .unwrap_or_else(|_| crate::tauri_cmds::OllamaState {
            reachable: false,
            models: vec![],
            version: None,
        });

    let (db_schema_version, evidence_count, leads_count, drafts_count, published_posts_count) = {
        let conn = db.lock().map_err(|_| "Failed to lock DB".to_string())?;

        let schema_ver: i64 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap_or(0);
        let ev_cnt: i64 = conn
            .query_row("SELECT COUNT(*) FROM evidence_items", [], |row| row.get(0))
            .unwrap_or(0);
        let ld_cnt: i64 = conn
            .query_row("SELECT COUNT(*) FROM leads", [], |row| row.get(0))
            .unwrap_or(0);
        let dr_cnt: i64 = conn
            .query_row("SELECT COUNT(*) FROM drafts", [], |row| row.get(0))
            .unwrap_or(0);
        let pub_cnt: i64 = conn
            .query_row("SELECT COUNT(*) FROM published_posts", [], |row| row.get(0))
            .unwrap_or(0);

        (schema_ver, ev_cnt, ld_cnt, dr_cnt, pub_cnt)
    };

    let log_path = app_data_dir.join("logs").join("civicnews.log");
    let mut panic_log_tail = Vec::new();
    if let Ok(file) = File::open(&log_path) {
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
        let start = if lines.len() > 100 {
            lines.len() - 100
        } else {
            0
        };
        panic_log_tail = lines[start..]
            .iter()
            .map(|line| redact_diagnostic_line(line))
            .collect();
    }

    Ok(Diagnostics {
        app_version,
        os_name,
        os_version,
        tauri_version,
        ollama_reachable: ollama_state.reachable,
        ollama_models: ollama_state.models,
        db_schema_version,
        evidence_count,
        leads_count,
        drafts_count,
        published_posts_count,
        panic_log_tail,
    })
}
