// core/db.rs
use chrono::Utc;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use subtle::ConstantTimeEq;

/// Constant-time string equality for auth secrets (bearer tokens, hashed
/// pairing PINs). A plain `==` (or a SQL `WHERE secret = ?`) can leak the length
/// and matching-prefix of a secret through how long the comparison takes. The
/// secrets here are high-entropy (UUID v4 tokens, 16-byte OsRng PINs hashed with
/// SHA-256), so practical exploitation over loopback is close to theoretical —
/// but treating these as a security boundary means comparing them in constant
/// time, and it keeps us safe if a future change ever shortens a secret. See
/// ENG-M2. Length is intentionally compared in variable time first (the length
/// of a hash is not itself secret) so the byte comparison runs over equal-length
/// inputs.
fn secret_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

/// The application's single shared SQLite handle.
///
/// ENG-Min3 — deliberate architecture decision (single writer, not a pool):
/// CivicNewspaper is a single-user, local desktop app. One `Arc<Mutex<Connection>>`
/// is shared by every Tauri command and the loopback Axum server (which only the
/// paired browser extension on the same machine reaches). The DB is opened WAL
/// (`open_conn`), and for a single-user workload a single writer under WAL is the
/// correct shape: there is no multi-client write contention to relieve, and SQLite
/// itself serializes writers anyway. A connection pool (e.g. r2d2_sqlite) would add
/// a dependency, change this type at ~40 call sites and every test constructor, and
/// complicate the backup/restore connection-swap (`backups.rs`) — real regression
/// risk on the data layer for no functional gain at this concurrency level.
///
/// The two genuinely long operations (backup save / restore) hold the lock for the
/// duration of a full-DB copy. They are bounded, complete on their own, and are
/// invoked from explicit user actions with their own UI progress state, so they do
/// not "appear hung" — they are not unbounded or polling. If this app ever grows a
/// real multi-writer workload, revisit with a pool; until then a pool is premature.
pub type DbConn = Arc<Mutex<Connection>>;
const DB_FILE_NAME: &str = "civicdesk.db";
const LEGACY_DB_FILE_NAME: &str = "civicnews.db";
const LEGACY_APP_DATA_DIR: &str = "org.civicnews.app";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<i32>,
    pub name: String,
    pub url: String,
    pub r#type: String, // 'primary_record', 'official_comm', 'community_signal', 'media_lead'
    pub status: String, // 'online', 'offline'
    pub tier: String,   // 'official_record', 'news_reporting', 'community_signal'
    pub last_success_at: Option<String>,
    pub last_failed_at: Option<String>,
    pub last_scraped: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub id: Option<i32>,
    pub source_id: i32,
    pub url: Option<String>,
    pub fetched_at: String,
    pub excerpt: String,
    pub content_hash: String,
    pub entities: String, // JSON array of strings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Option<i32>,
    pub detector_name: String,
    pub why: String,
    pub confidence: String,             // 'low', 'med', 'high'
    pub risk_level: String,             // 'low', 'med', 'high'
    pub confirmation_checklist: String, // JSON array
    pub from_scan_lead_id: Option<i32>,
    #[serde(default)]
    pub story_type: Option<String>,
    #[serde(default)]
    pub disposition: Option<String>,
    #[serde(default)]
    pub novelty_score: Option<i32>,
    #[serde(default)]
    pub novelty_reason: Option<String>,
    #[serde(default)]
    pub recurrence_count: Option<i32>,
    #[serde(default)]
    pub recurrence_note: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyScanRun {
    pub id: Option<i32>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub run_status: String, // 'in_progress', 'completed', 'failed'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyScanLead {
    pub id: Option<i32>,
    pub scan_id: i32,
    pub title: String,
    pub summary: String,
    pub source_id: Option<i32>,
    pub original_url: String,
    pub why_flagged: Option<String>,
    pub source_name: Option<String>,
    pub source_type: Option<String>,
    pub priority: Option<String>,
    pub suggested_next_step: Option<String>,
    #[serde(default)]
    pub story_type: Option<String>,
    #[serde(default)]
    pub what_changed: Option<String>,
    #[serde(default)]
    pub immediacy: Option<i32>,
    #[serde(default)]
    pub impact: Option<i32>,
    #[serde(default)]
    pub conflict: Option<i32>,
    #[serde(default)]
    pub novelty: Option<i32>,
    #[serde(default)]
    pub publishability_note: Option<String>,
    #[serde(default)]
    pub disposition: Option<String>,
    #[serde(default)]
    pub recurrence_count: Option<i32>,
    #[serde(default)]
    pub recurrence_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    pub id: Option<i32>,
    pub lead_id: Option<i32>,
    pub format: String, // 'brief', 'watch', 'explainer', 'investigation', 'opinion'
    pub title: String,
    pub content: String,
    pub status: String, // 'lead', 'draft_generated', 'ready_to_review', ...
    pub verification_checklist: String, // JSON array
    pub missing_evidence_notes: Option<String>,
    pub correction_note: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedPost {
    pub id: Option<i32>,
    pub draft_id: i32,
    pub file_path: String,
    pub url: String,
    pub published_at: String,
    pub correction_history: String, // JSON array
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRun {
    pub id: Option<i32>,
    pub issue_id: String,
    pub output_path: String,
    pub generated_files: String, // JSON array
    pub provider: String,
    pub published_url: Option<String>,
    pub deployment_id: Option<String>,
    pub article_count: i32,
    pub skipped_count: i32,
    pub files_written: i32,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDecisionAudit {
    pub id: Option<i32>,
    pub draft_id: i32,
    pub decision: String,
    pub attested: bool,
    pub guardrail_override_reason: Option<String>,
    pub guardrail_issue_count: i32,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: Option<i32>,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairedClient {
    pub id: Option<i32>,
    pub token: String,
    pub label: String,
    pub pairing_pin: Option<String>,
    pub pin_expires_at: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub revoked: bool,
}

// Database Initialization

/// Open a SQLite connection with the invariant pragmas this app relies on: WAL
/// journaling and foreign-key enforcement. `foreign_keys` is per-connection in
/// SQLite and does NOT persist, so EVERY connection that becomes a live handle
/// must be opened through here — otherwise ON DELETE CASCADE / SET NULL silently
/// stop firing (notably on the backup-restore rollback paths, which previously
/// reopened the live DB without re-enabling foreign keys). See finding C-2.
pub fn open_conn(path: &str) -> Result<Connection, Box<dyn Error + Send + Sync>> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    Ok(conn)
}

pub fn init_db(path: &str) -> Result<Connection, Box<dyn Error + Send + Sync>> {
    let mut conn = open_conn(path)?;
    super::migrations::run_migrations(&mut conn)?;
    Ok(conn)
}

// App Data Path Resolver for Tauri v2.
// ENG-Nit2: returns a `Send + Sync` boxed error so it composes with async/
// multithreaded callers (and the rest of the core error surface) without
// fighting auto-trait bounds.
pub fn get_app_db_path(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let app_data = super::app_paths::app_data_dir(app)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let db_path = app_data.join(DB_FILE_NAME);

    if !db_path.exists() {
        let legacy_same_dir = app_data.join(LEGACY_DB_FILE_NAME);
        let legacy_old_dir = app_data
            .parent()
            .map(|parent| parent.join(LEGACY_APP_DATA_DIR).join(LEGACY_DB_FILE_NAME));
        let legacy_source = if legacy_same_dir.exists() {
            Some(legacy_same_dir)
        } else {
            legacy_old_dir.filter(|path| path.exists())
        };

        if let Some(legacy_path) = legacy_source {
            std::fs::copy(legacy_path, &db_path)?;
        }
    }

    Ok(db_path)
}

// CRUD Operations

// --- Sources ---
pub fn list_sources(conn: &Connection) -> SqlResult<Vec<Source>> {
    let mut stmt = conn.prepare("SELECT id, name, url, type, status, last_success_at, last_failed_at, last_scraped, tier FROM sources")?;
    let source_iter = stmt.query_map([], |row| {
        Ok(Source {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            url: row.get(2)?,
            r#type: row.get(3)?,
            status: row.get(4)?,
            last_success_at: row.get(5)?,
            last_failed_at: row.get(6)?,
            last_scraped: row.get(7)?,
            tier: row.get(8)?,
        })
    })?;

    let mut sources = Vec::new();
    for source in source_iter {
        sources.push(source?);
    }
    Ok(sources)
}

pub fn get_source(conn: &Connection, id: i32) -> SqlResult<Option<Source>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, url, type, status, last_success_at, last_failed_at, last_scraped, tier FROM sources WHERE id = ?1",
    )?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Source {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            url: row.get(2)?,
            r#type: row.get(3)?,
            status: row.get(4)?,
            last_success_at: row.get(5)?,
            last_failed_at: row.get(6)?,
            last_scraped: row.get(7)?,
            tier: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn insert_source(conn: &Connection, source: &Source) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO sources (name, url, type, status, last_success_at, last_failed_at, last_scraped, tier) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![source.name, source.url, source.r#type, source.status, source.last_success_at, source.last_failed_at, source.last_scraped, source.tier],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn update_source_status(
    conn: &Connection,
    id: i32,
    status: &str,
    is_success: bool,
) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    if is_success {
        conn.execute(
            "UPDATE sources SET status = ?1, last_success_at = ?2, last_scraped = ?2 WHERE id = ?3",
            params![status, now, id],
        )?;
    } else {
        conn.execute(
            "UPDATE sources SET status = ?1, last_failed_at = ?2 WHERE id = ?3",
            params![status, now, id],
        )?;
    }
    Ok(())
}

pub fn delete_source(conn: &Connection, id: i32) -> SqlResult<()> {
    conn.execute("DELETE FROM sources WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Evidence Items ---
pub fn insert_evidence_item(conn: &Connection, item: &EvidenceItem) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO evidence_items (source_id, url, fetched_at, excerpt, content_hash, entities) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![item.source_id, item.url, item.fetched_at, item.excerpt, item.content_hash, item.entities],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_evidence_by_hash(conn: &Connection, hash: &str) -> SqlResult<Option<EvidenceItem>> {
    let mut stmt = conn.prepare("SELECT id, source_id, url, fetched_at, excerpt, content_hash, entities FROM evidence_items WHERE content_hash = ?1")?;
    let mut rows = stmt.query(params![hash])?;
    if let Some(row) = rows.next()? {
        Ok(Some(EvidenceItem {
            id: Some(row.get(0)?),
            source_id: row.get(1)?,
            url: row.get(2)?,
            fetched_at: row.get(3)?,
            excerpt: row.get(4)?,
            content_hash: row.get(5)?,
            entities: row.get(6)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_evidence_by_lead(conn: &Connection, lead_id: i32) -> SqlResult<Vec<EvidenceItem>> {
    let mut stmt = conn.prepare(
        "SELECT e.id, e.source_id, e.url, e.fetched_at, e.excerpt, e.content_hash, e.entities 
         FROM evidence_items e
         JOIN lead_evidence le ON e.id = le.evidence_id
         WHERE le.lead_id = ?1
         ORDER BY e.fetched_at DESC, e.id DESC",
    )?;
    let iter = stmt.query_map(params![lead_id], |row| {
        Ok(EvidenceItem {
            id: Some(row.get(0)?),
            source_id: row.get(1)?,
            url: row.get(2)?,
            fetched_at: row.get(3)?,
            excerpt: row.get(4)?,
            content_hash: row.get(5)?,
            entities: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for item in iter {
        items.push(item?);
    }
    Ok(items)
}

#[allow(dead_code)]
pub fn list_all_evidence(conn: &Connection) -> SqlResult<Vec<EvidenceItem>> {
    let mut stmt = conn.prepare("SELECT id, source_id, url, fetched_at, excerpt, content_hash, entities FROM evidence_items")?;
    let iter = stmt.query_map([], |row| {
        Ok(EvidenceItem {
            id: Some(row.get(0)?),
            source_id: row.get(1)?,
            url: row.get(2)?,
            fetched_at: row.get(3)?,
            excerpt: row.get(4)?,
            content_hash: row.get(5)?,
            entities: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for item in iter {
        items.push(item?);
    }
    Ok(items)
}

pub fn list_evidence_since(conn: &Connection, since_hours: u32) -> SqlResult<Vec<EvidenceItem>> {
    let cutoff = Utc::now() - chrono::Duration::hours(since_hours as i64);
    let cutoff_str = cutoff.to_rfc3339();

    let mut stmt = conn.prepare(
        "SELECT id, source_id, url, fetched_at, excerpt, content_hash, entities 
         FROM evidence_items 
         WHERE fetched_at >= ?1
         ORDER BY fetched_at DESC, id DESC",
    )?;
    let iter = stmt.query_map(params![cutoff_str], |row| {
        Ok(EvidenceItem {
            id: Some(row.get(0)?),
            source_id: row.get(1)?,
            url: row.get(2)?,
            fetched_at: row.get(3)?,
            excerpt: row.get(4)?,
            content_hash: row.get(5)?,
            entities: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for item in iter {
        items.push(item?);
    }
    Ok(items)
}

// --- Leads ---
pub fn insert_lead(conn: &Connection, lead: &Lead, evidence_ids: &[i32]) -> SqlResult<i32> {
    let now = Utc::now().to_rfc3339();

    // We execute the insert and linking inside a transaction
    // to keep lead integrity.
    // Note: since rusqlite allows executing on connection directly,
    // and we want this call to be atomic:
    // If the connection is already in a transaction, this might fail, so we use execute block
    conn.execute(
        "INSERT INTO leads (detector_name, why, confidence, risk_level, confirmation_checklist, from_scan_lead_id, story_type, disposition, novelty_score, novelty_reason, recurrence_count, recurrence_note, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            lead.detector_name,
            lead.why,
            lead.confidence,
            lead.risk_level,
            lead.confirmation_checklist,
            lead.from_scan_lead_id,
            lead.story_type,
            lead.disposition.as_deref().unwrap_or("review"),
            lead.novelty_score,
            lead.novelty_reason,
            lead.recurrence_count,
            lead.recurrence_note,
            now
        ],
    )?;
    let lead_id = conn.last_insert_rowid() as i32;
    for &eid in evidence_ids {
        conn.execute(
            "INSERT INTO lead_evidence (lead_id, evidence_id) VALUES (?1, ?2)",
            params![lead_id, eid],
        )?;
    }
    Ok(lead_id)
}

pub fn list_leads(conn: &Connection) -> SqlResult<Vec<Lead>> {
    let mut stmt = conn.prepare("SELECT id, detector_name, why, confidence, risk_level, confirmation_checklist, from_scan_lead_id, story_type, disposition, novelty_score, novelty_reason, recurrence_count, recurrence_note, created_at FROM leads")?;
    let iter = stmt.query_map([], |row| {
        Ok(Lead {
            id: Some(row.get(0)?),
            detector_name: row.get(1)?,
            why: row.get(2)?,
            confidence: row.get(3)?,
            risk_level: row.get(4)?,
            confirmation_checklist: row.get(5)?,
            from_scan_lead_id: row.get(6)?,
            story_type: row.get(7)?,
            disposition: row.get(8)?,
            novelty_score: row.get(9)?,
            novelty_reason: row.get(10)?,
            recurrence_count: row.get(11)?,
            recurrence_note: row.get(12)?,
            created_at: row.get(13)?,
        })
    })?;
    let mut leads = Vec::new();
    for lead in iter {
        leads.push(lead?);
    }
    Ok(leads)
}

// --- Daily Scans ---
pub fn insert_daily_scan_run(conn: &Connection, run: &DailyScanRun) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO daily_scan_runs (started_at, completed_at, run_status) VALUES (?1, ?2, ?3)",
        params![run.started_at, run.completed_at, run.run_status],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn update_daily_scan_run(conn: &Connection, run: &DailyScanRun) -> SqlResult<()> {
    conn.execute(
        "UPDATE daily_scan_runs SET completed_at = ?1, run_status = ?2 WHERE id = ?3",
        params![run.completed_at, run.run_status, run.id],
    )?;
    Ok(())
}

pub fn insert_daily_scan_lead(conn: &Connection, lead: &DailyScanLead) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO daily_scan_leads (scan_id, title, summary, source_id, original_url, why_flagged, source_name, source_type, priority, suggested_next_step, story_type, what_changed, immediacy, impact, conflict, novelty, publishability_note, disposition, recurrence_count, recurrence_note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
        params![
            lead.scan_id,
            lead.title,
            lead.summary,
            lead.source_id,
            lead.original_url,
            lead.why_flagged,
            lead.source_name,
            lead.source_type,
            lead.priority,
            lead.suggested_next_step,
            lead.story_type,
            lead.what_changed,
            lead.immediacy,
            lead.impact,
            lead.conflict,
            lead.novelty,
            lead.publishability_note,
            lead.disposition.as_deref().unwrap_or("review"),
            lead.recurrence_count,
            lead.recurrence_note
        ],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

#[allow(dead_code)]
pub fn list_daily_scan_leads(conn: &Connection, scan_id: i32) -> SqlResult<Vec<DailyScanLead>> {
    let mut stmt = conn.prepare("SELECT id, scan_id, title, summary, source_id, original_url, why_flagged, source_name, source_type, priority, suggested_next_step, story_type, what_changed, immediacy, impact, conflict, novelty, publishability_note, disposition, recurrence_count, recurrence_note FROM daily_scan_leads WHERE scan_id = ?1")?;
    let iter = stmt.query_map(params![scan_id], |row| {
        Ok(DailyScanLead {
            id: Some(row.get(0)?),
            scan_id: row.get(1)?,
            title: row.get(2)?,
            summary: row.get(3)?,
            source_id: row.get(4)?,
            original_url: row.get(5)?,
            why_flagged: row.get(6)?,
            source_name: row.get(7)?,
            source_type: row.get(8)?,
            priority: row.get(9)?,
            suggested_next_step: row.get(10)?,
            story_type: row.get(11)?,
            what_changed: row.get(12)?,
            immediacy: row.get(13)?,
            impact: row.get(14)?,
            conflict: row.get(15)?,
            novelty: row.get(16)?,
            publishability_note: row.get(17)?,
            disposition: row.get(18)?,
            recurrence_count: row.get(19)?,
            recurrence_note: row.get(20)?,
        })
    })?;
    let mut leads = Vec::new();
    for lead in iter {
        leads.push(lead?);
    }
    Ok(leads)
}

// --- Drafts ---
pub fn get_draft(conn: &Connection, id: i32) -> SqlResult<Option<Draft>> {
    let mut stmt = conn.prepare("SELECT id, lead_id, format, title, content, status, verification_checklist, missing_evidence_notes, correction_note, created_at, updated_at FROM drafts WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Draft {
            id: Some(row.get(0)?),
            lead_id: row.get(1)?,
            format: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            status: row.get(5)?,
            verification_checklist: row.get(6)?,
            missing_evidence_notes: row.get(7)?,
            correction_note: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        }))
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
pub fn get_draft_by_lead(conn: &Connection, lead_id: i32) -> SqlResult<Option<Draft>> {
    let mut stmt = conn.prepare("SELECT id, lead_id, format, title, content, status, verification_checklist, missing_evidence_notes, correction_note, created_at, updated_at FROM drafts WHERE lead_id = ?1")?;
    let mut rows = stmt.query(params![lead_id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Draft {
            id: Some(row.get(0)?),
            lead_id: row.get(1)?,
            format: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            status: row.get(5)?,
            verification_checklist: row.get(6)?,
            missing_evidence_notes: row.get(7)?,
            correction_note: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn insert_draft(conn: &Connection, draft: &Draft) -> SqlResult<i32> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO drafts (lead_id, format, title, content, status, verification_checklist, missing_evidence_notes, correction_note, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        params![draft.lead_id, draft.format, draft.title, draft.content, draft.status, draft.verification_checklist, draft.missing_evidence_notes, draft.correction_note, now],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn update_draft(conn: &Connection, draft: &Draft) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE drafts SET format = ?1, title = ?2, content = ?3, status = ?4, verification_checklist = ?5, missing_evidence_notes = ?6, correction_note = ?7, updated_at = ?8 WHERE id = ?9",
        params![draft.format, draft.title, draft.content, draft.status, draft.verification_checklist, draft.missing_evidence_notes, draft.correction_note, now, draft.id],
    )?;
    Ok(())
}

pub fn update_draft_status(conn: &Connection, id: i32, status: &str) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE drafts SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, id],
    )?;
    Ok(())
}

/// GG-C1: record that a human reviewed this draft and accepts responsibility.
/// Stamped before a draft may be approved for publishing (see `story_decision`).
pub fn attest_draft(conn: &Connection, id: i32, editor: &str) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE drafts SET attested_by = ?1, attested_at = ?2, updated_at = ?2 WHERE id = ?3",
        params![editor, now, id],
    )?;
    Ok(())
}

/// GG-B2: record an explicit, logged decision to publish despite error-severity
/// guardrail issues, for the audit trail.
pub fn record_guardrail_override(conn: &Connection, id: i32, reason: &str) -> SqlResult<()> {
    conn.execute(
        "UPDATE drafts SET guardrail_override_reason = ?1 WHERE id = ?2",
        params![reason, id],
    )?;
    Ok(())
}

/// GG-B2 / GG-C1: read the publish-gate state for a draft as
/// `(attested_at, guardrail_override_reason)`. Kept separate from `get_draft`
/// so the `Draft` struct and its many SELECT sites stay unchanged.
pub fn get_draft_publish_gate(
    conn: &Connection,
    id: i32,
) -> SqlResult<(Option<String>, Option<String>)> {
    conn.query_row(
        "SELECT attested_at, guardrail_override_reason FROM drafts WHERE id = ?1",
        params![id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
}

pub fn record_publish_decision_audit(
    conn: &Connection,
    audit: &PublishDecisionAudit,
) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO publish_decision_audits (draft_id, decision, attested, guardrail_override_reason, guardrail_issue_count, note, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            audit.draft_id,
            audit.decision,
            if audit.attested { 1 } else { 0 },
            audit.guardrail_override_reason.as_deref(),
            audit.guardrail_issue_count,
            audit.note,
            now
        ],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn list_publish_decision_audits(
    conn: &Connection,
    draft_id: i32,
) -> SqlResult<Vec<PublishDecisionAudit>> {
    let mut stmt = conn.prepare(
        "SELECT id, draft_id, decision, attested, guardrail_override_reason, guardrail_issue_count, note, created_at FROM publish_decision_audits WHERE draft_id = ?1 ORDER BY id ASC",
    )?;
    let iter = stmt.query_map(params![draft_id], |row| {
        let attested: i32 = row.get(3)?;
        Ok(PublishDecisionAudit {
            id: Some(row.get(0)?),
            draft_id: row.get(1)?,
            decision: row.get(2)?,
            attested: attested != 0,
            guardrail_override_reason: row.get(4)?,
            guardrail_issue_count: row.get(5)?,
            note: row.get(6)?,
            created_at: row.get(7)?,
        })
    })?;
    let mut rows = Vec::new();
    for row in iter {
        rows.push(row?);
    }
    Ok(rows)
}

pub fn list_drafts(conn: &Connection) -> SqlResult<Vec<Draft>> {
    let mut stmt = conn.prepare("SELECT id, lead_id, format, title, content, status, verification_checklist, missing_evidence_notes, correction_note, created_at, updated_at FROM drafts ORDER BY updated_at DESC")?;
    let iter = stmt.query_map([], |row| {
        Ok(Draft {
            id: Some(row.get(0)?),
            lead_id: row.get(1)?,
            format: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            status: row.get(5)?,
            verification_checklist: row.get(6)?,
            missing_evidence_notes: row.get(7)?,
            correction_note: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    let mut drafts = Vec::new();
    for d in iter {
        drafts.push(d?);
    }
    Ok(drafts)
}

pub fn delete_draft(conn: &Connection, id: i32) -> SqlResult<()> {
    conn.execute("DELETE FROM drafts WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Published Posts ---
pub fn insert_published_post(conn: &Connection, post: &PublishedPost) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO published_posts (draft_id, file_path, url, published_at, correction_history) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![post.draft_id, post.file_path, post.url, post.published_at, post.correction_history],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

#[allow(dead_code)]
pub fn list_published_posts(conn: &Connection) -> SqlResult<Vec<PublishedPost>> {
    let mut stmt = conn.prepare("SELECT id, draft_id, file_path, url, published_at, correction_history FROM published_posts ORDER BY published_at DESC")?;
    let iter = stmt.query_map([], |row| {
        Ok(PublishedPost {
            id: Some(row.get(0)?),
            draft_id: row.get(1)?,
            file_path: row.get(2)?,
            url: row.get(3)?,
            published_at: row.get(4)?,
            correction_history: row.get(5)?,
        })
    })?;
    let mut posts = Vec::new();
    for p in iter {
        posts.push(p?);
    }
    Ok(posts)
}

// --- Publish Runs ---
pub fn insert_publish_run(conn: &Connection, run: &PublishRun) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO publish_runs (issue_id, output_path, generated_files, provider, published_url, deployment_id, article_count, skipped_count, files_written, generated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            run.issue_id,
            run.output_path,
            run.generated_files,
            run.provider,
            run.published_url,
            run.deployment_id,
            run.article_count,
            run.skipped_count,
            run.files_written,
            run.generated_at
        ],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

#[allow(dead_code)]
pub fn list_publish_runs(conn: &Connection) -> SqlResult<Vec<PublishRun>> {
    let mut stmt = conn.prepare("SELECT id, issue_id, output_path, generated_files, provider, published_url, deployment_id, article_count, skipped_count, files_written, generated_at FROM publish_runs ORDER BY generated_at DESC")?;
    let iter = stmt.query_map([], |row| {
        Ok(PublishRun {
            id: Some(row.get(0)?),
            issue_id: row.get(1)?,
            output_path: row.get(2)?,
            generated_files: row.get(3)?,
            provider: row.get(4)?,
            published_url: row.get(5)?,
            deployment_id: row.get(6)?,
            article_count: row.get(7)?,
            skipped_count: row.get(8)?,
            files_written: row.get(9)?,
            generated_at: row.get(10)?,
        })
    })?;
    let mut runs = Vec::new();
    for run in iter {
        runs.push(run?);
    }
    Ok(runs)
}

pub fn update_latest_publish_run_destination(
    conn: &Connection,
    output_path: &str,
    provider: &str,
    published_url: &str,
    deployment_id: Option<&str>,
) -> SqlResult<PublishRun> {
    conn.execute(
        "UPDATE publish_runs
         SET provider = ?1, published_url = ?2, deployment_id = ?3
         WHERE id = (
            SELECT id FROM publish_runs
            WHERE output_path = ?4
            ORDER BY generated_at DESC, id DESC
            LIMIT 1
         )",
        params![provider, published_url, deployment_id, output_path],
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, issue_id, output_path, generated_files, provider, published_url, deployment_id, article_count, skipped_count, files_written, generated_at
         FROM publish_runs
         WHERE output_path = ?1
         ORDER BY generated_at DESC, id DESC
         LIMIT 1",
    )?;
    stmt.query_row(params![output_path], |row| {
        Ok(PublishRun {
            id: Some(row.get(0)?),
            issue_id: row.get(1)?,
            output_path: row.get(2)?,
            generated_files: row.get(3)?,
            provider: row.get(4)?,
            published_url: row.get(5)?,
            deployment_id: row.get(6)?,
            article_count: row.get(7)?,
            skipped_count: row.get(8)?,
            files_written: row.get(9)?,
            generated_at: row.get(10)?,
        })
    })
}

// --- Subscribers ---
pub fn list_subscribers(conn: &Connection) -> SqlResult<Vec<Subscriber>> {
    let mut stmt = conn.prepare(
        "SELECT id, email, name, status, created_at, updated_at FROM subscribers ORDER BY email",
    )?;
    let iter = stmt.query_map([], |row| {
        Ok(Subscriber {
            id: Some(row.get(0)?),
            email: row.get(1)?,
            name: row.get(2)?,
            status: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    let mut subscribers = Vec::new();
    for subscriber in iter {
        subscribers.push(subscriber?);
    }
    Ok(subscribers)
}

pub fn upsert_subscriber(conn: &Connection, email: &str, name: Option<&str>) -> SqlResult<i32> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO subscribers (email, name, status, created_at, updated_at)
         VALUES (?1, ?2, 'active', ?3, ?3)
         ON CONFLICT(email) DO UPDATE SET
            name = excluded.name,
            status = 'active',
            updated_at = excluded.updated_at",
        params![email, name, now],
    )?;
    conn.query_row(
        "SELECT id FROM subscribers WHERE email = ?1",
        params![email],
        |row| row.get(0),
    )
}

pub fn delete_subscriber(conn: &Connection, id: i32) -> SqlResult<()> {
    conn.execute("DELETE FROM subscribers WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Paired Clients ---
pub fn get_paired_client_by_token(
    conn: &Connection,
    token: &str,
) -> SqlResult<Option<PairedClient>> {
    // ENG-M2: do NOT look the token up with a SQL `WHERE token = ?` equality —
    // SQLite's string comparison is not constant-time. Instead enumerate the
    // active (paired, non-revoked) clients and compare each stored token against
    // the supplied one in constant time. The set of paired browser clients is
    // tiny, so the full scan is negligible.
    let mut stmt = conn.prepare("SELECT id, token, label, pairing_pin, pin_expires_at, created_at, last_used_at, revoked FROM paired_clients WHERE revoked = 0 AND pairing_pin IS NULL")?;
    let iter = stmt.query_map([], |row| {
        Ok(PairedClient {
            id: Some(row.get(0)?),
            token: row.get(1)?,
            label: row.get(2)?,
            pairing_pin: row.get(3)?,
            pin_expires_at: row.get(4)?,
            created_at: row.get(5)?,
            last_used_at: row.get(6)?,
            revoked: row.get::<_, i32>(7)? == 1,
        })
    })?;
    for client in iter {
        let client = client?;
        if secret_eq(&client.token, token) {
            return Ok(Some(client));
        }
    }
    Ok(None)
}

#[allow(dead_code)]
pub fn get_paired_client_by_pin(conn: &Connection, pin: &str) -> SqlResult<Option<PairedClient>> {
    // ENG-M2: constant-time PIN comparison (see `secret_eq` / `confirm_pairing`).
    let mut stmt = conn.prepare("SELECT id, token, label, pairing_pin, pin_expires_at, created_at, last_used_at, revoked FROM paired_clients WHERE revoked = 0 AND pairing_pin IS NOT NULL")?;
    let iter = stmt.query_map([], |row| {
        Ok(PairedClient {
            id: Some(row.get(0)?),
            token: row.get(1)?,
            label: row.get(2)?,
            pairing_pin: row.get(3)?,
            pin_expires_at: row.get(4)?,
            created_at: row.get(5)?,
            last_used_at: row.get(6)?,
            revoked: row.get::<_, i32>(7)? == 1,
        })
    })?;
    for client in iter {
        let client = client?;
        if let Some(stored) = client.pairing_pin.as_deref() {
            if secret_eq(stored, pin) {
                return Ok(Some(client));
            }
        }
    }
    Ok(None)
}

pub fn create_pairing_pin(
    conn: &Connection,
    label: &str,
    pin: &str,
    expires_at: &str,
) -> SqlResult<String> {
    let now = Utc::now().to_rfc3339();
    // Create a temporary token that is inactive until paired
    let token = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO paired_clients (token, label, pairing_pin, pin_expires_at, created_at, revoked) VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        params![token, label, pin, expires_at, now],
    )?;
    Ok(token)
}

pub fn confirm_pairing(conn: &Connection, pin: &str) -> SqlResult<Option<String>> {
    let now = Utc::now().to_rfc3339();
    // ENG-M2: enumerate candidate clients with a pending PIN and compare the
    // hashed PIN in constant time, rather than a SQL `WHERE pairing_pin = ?`
    // equality (not constant-time). The candidate set is tiny (one pending pair
    // at a time in practice).
    let mut stmt = conn.prepare(
        "SELECT id, token, pairing_pin, pin_expires_at FROM paired_clients WHERE revoked = 0 AND pairing_pin IS NOT NULL",
    )?;
    let rows: Vec<(i32, String, String, String)> = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?
        .collect::<SqlResult<Vec<_>>>()?;

    for (id, token, stored_pin, expires_at) in rows {
        if secret_eq(&stored_pin, pin) {
            if expires_at > now {
                // Pair successful! Clear PIN and set last_used_at
                conn.execute(
                    "UPDATE paired_clients SET pairing_pin = NULL, pin_expires_at = NULL, last_used_at = ?1 WHERE id = ?2",
                    params![now, id],
                )?;
                return Ok(Some(token));
            }
            // Matched but expired — stop (the PIN is unique per pending pair).
            return Ok(None);
        }
    }
    Ok(None)
}

pub fn list_paired_clients(conn: &Connection) -> SqlResult<Vec<PairedClient>> {
    let mut stmt = conn.prepare("SELECT id, token, label, pairing_pin, pin_expires_at, created_at, last_used_at, revoked FROM paired_clients WHERE revoked = 0 AND pairing_pin IS NULL")?;
    let iter = stmt.query_map([], |row| {
        Ok(PairedClient {
            id: Some(row.get(0)?),
            token: row.get(1)?,
            label: row.get(2)?,
            pairing_pin: row.get(3)?,
            pin_expires_at: row.get(4)?,
            created_at: row.get(5)?,
            last_used_at: row.get(6)?,
            revoked: row.get::<_, i32>(7)? == 1,
        })
    })?;
    let mut clients = Vec::new();
    for c in iter {
        clients.push(c?);
    }
    Ok(clients)
}

pub fn revoke_paired_client(conn: &Connection, id: i32) -> SqlResult<()> {
    conn.execute(
        "UPDATE paired_clients SET revoked = 1 WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

pub fn record_paired_client_use(conn: &Connection, token: &str) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE paired_clients SET last_used_at = ?1 WHERE token = ?2",
        params![now, token],
    )?;
    Ok(())
}
