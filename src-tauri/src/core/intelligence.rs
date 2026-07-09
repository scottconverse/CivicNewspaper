use super::db::{EvidenceItem, Source};
use chrono::Utc;
use regex::Regex;
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicObservation {
    pub id: Option<i32>,
    pub observation_type: String,
    pub source_id: Option<i32>,
    pub evidence_id: Option<i32>,
    pub title: String,
    pub summary: String,
    pub url: Option<String>,
    pub observed_at: String,
    pub content_hash: Option<String>,
    pub previous_hash: Option<String>,
    pub diff_summary: Option<String>,
    pub metadata_json: String,
    pub tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicEntity {
    pub id: Option<i32>,
    pub entity_type: String,
    pub name: String,
    pub normalized_name: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub mention_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePerformanceScore {
    pub source_id: i32,
    pub source_name: String,
    pub fetch_successes: i32,
    pub fetch_failures: i32,
    pub new_items: i32,
    pub changed_items: i32,
    pub entity_hits: i32,
    pub dark_signal_hits: i32,
    pub reliability_score: f64,
    pub usefulness_score: f64,
    pub last_fetch_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkSignal {
    pub id: Option<i32>,
    pub observation_id: Option<i32>,
    pub source_id: Option<i32>,
    pub title: String,
    pub summary: String,
    pub origin: String,
    pub risk_level: String,
    pub rank_score: f64,
    pub tier: String,
    pub evidence_policy: String,
    pub why_it_matters: String,
    pub verification_path: String,
    pub publication_status: String,
    pub created_at: String,
    pub updated_at: String,
    pub entities: Vec<CivicEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicIntelligenceSnapshot {
    pub observations: Vec<CivicObservation>,
    pub entities: Vec<CivicEntity>,
    pub source_scores: Vec<SourcePerformanceScore>,
    pub dark_signals: Vec<DarkSignal>,
}

#[derive(Debug, Clone)]
pub struct ExtractedEntity {
    pub entity_type: String,
    pub name: String,
}

static RE_MONEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\$[0-9][0-9,]*(?:\.[0-9]{2})?").unwrap());
static RE_PARCEL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:parcel|apn|pin)\s*#?:?\s*([A-Z0-9-]{5,})\b").unwrap());
static RE_ADDRESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b\d{2,6}\s+[A-Z][A-Za-z0-9.'-]*(?:\s+[A-Z][A-Za-z0-9.'-]*){0,4}\s+(?:St|Street|Ave|Avenue|Rd|Road|Dr|Drive|Blvd|Boulevard|Ln|Lane|Ct|Court|Way|Pkwy|Parkway)\b").unwrap()
});
static RE_AGENCY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Z][A-Za-z&.'-]*(?:\s+[A-Z][A-Za-z&.'-]*){0,5}\s+(?:City Council|Town Council|Board|Commission|Committee|Department|Agency|District|Authority|Office)\b").unwrap()
});
static RE_COMPANY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Z][A-Za-z0-9&.'-]*(?:\s+[A-Z][A-Za-z0-9&.'-]*){0,5}\s+(?:LLC|Inc|Corp|Corporation|Company|Co\.|LP|LLP|Holdings|Partners|Group|Development|Developers)\b").unwrap()
});
static RE_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:Mayor|Councilmember|Commissioner|Chief|Sheriff|Director|Manager|Attorney)\s+[A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2}\b").unwrap()
});

pub fn extract_civic_entities(text: &str) -> Vec<ExtractedEntity> {
    let mut entities = Vec::new();
    push_matches(&mut entities, "money", &RE_MONEY, text);
    for cap in RE_PARCEL.captures_iter(text) {
        if let Some(value) = cap.get(1) {
            entities.push(ExtractedEntity {
                entity_type: "parcel".to_string(),
                name: value.as_str().to_string(),
            });
        }
    }
    push_matches(&mut entities, "address", &RE_ADDRESS, text);
    push_matches(&mut entities, "agency", &RE_AGENCY, text);
    push_matches(&mut entities, "company", &RE_COMPANY, text);
    push_matches(&mut entities, "vendor", &RE_COMPANY, text);
    push_matches(&mut entities, "person", &RE_PERSON, text);

    entities.sort_by(|a, b| {
        (a.entity_type.as_str(), normalize_entity_name(&a.name))
            .cmp(&(b.entity_type.as_str(), normalize_entity_name(&b.name)))
    });
    entities.dedup_by(|a, b| {
        a.entity_type == b.entity_type
            && normalize_entity_name(&a.name) == normalize_entity_name(&b.name)
    });
    entities
}

fn push_matches(out: &mut Vec<ExtractedEntity>, entity_type: &str, re: &Regex, text: &str) {
    for mat in re.find_iter(text) {
        out.push(ExtractedEntity {
            entity_type: entity_type.to_string(),
            name: mat.as_str().trim().to_string(),
        });
    }
}

fn normalize_entity_name(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn record_source_fetch(
    conn: &Connection,
    source: &Source,
    success: bool,
    message: &str,
) -> SqlResult<()> {
    let now = Utc::now().to_rfc3339();
    let source_id = source.id.unwrap_or_default();
    let obs = CivicObservation {
        id: None,
        observation_type: "source_fetched".to_string(),
        source_id: source.id,
        evidence_id: None,
        title: if success {
            format!("Fetched {}", source.name)
        } else {
            format!("Could not fetch {}", source.name)
        },
        summary: message.to_string(),
        url: Some(source.url.clone()),
        observed_at: now.clone(),
        content_hash: None,
        previous_hash: None,
        diff_summary: None,
        metadata_json: format!(r#"{{"success":{}}}"#, success),
        tier: source.tier.clone(),
    };
    insert_observation(conn, &obs)?;
    update_source_score(
        conn,
        SourceScoreUpdate {
            source_id,
            fetch_success: success,
            new_items: 0,
            changed_items: 0,
            entity_hits: 0,
            dark_signal_hits: 0,
            last_fetch_at: Some(&now),
        },
    )
}

pub fn record_evidence_intelligence(
    conn: &Connection,
    source: &Source,
    item: &EvidenceItem,
    evidence_id: i32,
    previous_hash: Option<String>,
) -> SqlResult<()> {
    let changed = previous_hash
        .as_deref()
        .map(|prev| prev != item.content_hash)
        .unwrap_or(false);
    let observation_type = classify_observation_type(source, &item.excerpt, changed);
    let diff_summary = previous_hash
        .as_ref()
        .map(|prev| summarize_change(prev, &item.content_hash, &item.excerpt));
    let title = observation_title(&observation_type, source, &item.excerpt);
    let summary = item
        .excerpt
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or(&item.excerpt)
        .chars()
        .take(260)
        .collect::<String>();
    let obs = CivicObservation {
        id: None,
        observation_type: observation_type.clone(),
        source_id: source.id,
        evidence_id: Some(evidence_id),
        title,
        summary,
        url: item.url.clone(),
        observed_at: item.fetched_at.clone(),
        content_hash: Some(item.content_hash.clone()),
        previous_hash,
        diff_summary,
        metadata_json: "{}".to_string(),
        tier: source.tier.clone(),
    };
    let observation_id = insert_observation(conn, &obs)?;
    let extracted = extract_civic_entities(&item.excerpt);
    for entity in &extracted {
        let entity_id = upsert_entity(conn, entity, &item.fetched_at)?;
        link_observation_entity(
            conn,
            observation_id,
            entity_id,
            Some(&context_for(&entity.name, &item.excerpt)),
        )?;
        let entity_obs = CivicObservation {
            id: None,
            observation_type: "entity_detected".to_string(),
            source_id: source.id,
            evidence_id: Some(evidence_id),
            title: format!(
                "{} detected: {}",
                label_entity_type(&entity.entity_type),
                entity.name
            ),
            summary: context_for(&entity.name, &item.excerpt),
            url: item.url.clone(),
            observed_at: item.fetched_at.clone(),
            content_hash: Some(item.content_hash.clone()),
            previous_hash: None,
            diff_summary: None,
            metadata_json: format!(r#"{{"entity_type":"{}"}}"#, entity.entity_type),
            tier: source.tier.clone(),
        };
        let entity_observation_id = insert_observation(conn, &entity_obs)?;
        link_observation_entity(
            conn,
            entity_observation_id,
            entity_id,
            Some(&entity_obs.summary),
        )?;
    }

    let mut dark_hits = 0;
    if should_create_dark_signal(source, &item.excerpt, &observation_type) {
        insert_dark_signal(conn, observation_id, source, item, &extracted)?;
        dark_hits = 1;
    }

    update_source_score(
        conn,
        SourceScoreUpdate {
            source_id: source.id.unwrap_or_default(),
            fetch_success: true,
            new_items: 1,
            changed_items: i32::from(changed),
            entity_hits: extracted.len() as i32,
            dark_signal_hits: dark_hits,
            last_fetch_at: Some(&item.fetched_at),
        },
    )
}

fn classify_observation_type(source: &Source, text: &str, changed: bool) -> String {
    let lower = text.to_ascii_lowercase();
    if source.r#type == "community_signal" || source.tier == "community_signal" {
        return "social_signal_found".to_string();
    }
    if lower.contains("youtube.com") || lower.contains("youtu.be") || lower.contains("video") {
        return "video_posted".to_string();
    }
    if lower.contains("agenda")
        || lower.contains("public hearing")
        || lower.contains("ordinance")
        || lower.contains("resolution")
    {
        return "agenda_item_found".to_string();
    }
    if changed {
        return "document_changed".to_string();
    }
    "source_fetched".to_string()
}

fn observation_title(kind: &str, source: &Source, text: &str) -> String {
    let first_line = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim();
    let short = first_line.chars().take(80).collect::<String>();
    match kind {
        "agenda_item_found" => format!("Agenda item found in {}", source.name),
        "document_changed" => format!("Document changed at {}", source.name),
        "video_posted" => format!("Video posted from {}", source.name),
        "social_signal_found" => format!("Community signal from {}", source.name),
        _ if !short.is_empty() => short,
        _ => format!("Observation from {}", source.name),
    }
}

fn summarize_change(previous_hash: &str, current_hash: &str, text: &str) -> String {
    let preview = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .chars()
        .take(180)
        .collect::<String>();
    format!(
        "Content hash changed from {} to {}. Current excerpt starts: {}",
        &previous_hash[..previous_hash.len().min(12)],
        &current_hash[..current_hash.len().min(12)],
        preview
    )
}

fn should_create_dark_signal(source: &Source, text: &str, observation_type: &str) -> bool {
    if observation_type == "social_signal_found" {
        return true;
    }
    let lower = text.to_ascii_lowercase();
    source.tier == "community_signal"
        || lower.contains("rumor")
        || lower.contains("anonymous")
        || lower.contains("datacenter")
        || lower.contains("data center")
        || lower.contains("out of state")
        || lower.contains("shell company")
        || lower.contains("quietly")
        || lower.contains("abuse")
        || lower.contains("retaliation")
}

fn insert_dark_signal(
    conn: &Connection,
    observation_id: i32,
    source: &Source,
    item: &EvidenceItem,
    entities: &[ExtractedEntity],
) -> SqlResult<i32> {
    let risk = dark_signal_risk(&item.excerpt, entities);
    let score = dark_signal_score(&risk, entities.len(), source);
    let now = Utc::now().to_rfc3339();
    let title = format!("Review community signal from {}", source.name);
    let summary = item
        .excerpt
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(4)
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(300)
        .collect::<String>();
    let verification_path = "Check for matching official records, agenda packets, land records, business registrations, meeting video, and at least one independent local source before drafting.".to_string();
    conn.execute(
        "INSERT INTO dark_signals (observation_id, source_id, title, summary, origin, risk_level, rank_score, tier, evidence_policy, why_it_matters, verification_path, publication_status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'editor_review_only', ?9, ?10, 'review', ?11, ?11)",
        params![
            observation_id,
            source.id,
            title,
            summary,
            source.name,
            risk,
            score,
            source.tier,
            "Community or weakly verified signals can reveal early civic risk, but they must be verified before they become publishable evidence.",
            verification_path,
            now
        ],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

fn dark_signal_risk(text: &str, entities: &[ExtractedEntity]) -> String {
    let lower = text.to_ascii_lowercase();
    if lower.contains("abuse")
        || lower.contains("retaliation")
        || lower.contains("shell company")
        || lower.contains("out of state")
        || lower.contains("datacenter")
        || lower.contains("data center")
        || entities
            .iter()
            .any(|e| e.entity_type == "parcel" || e.entity_type == "company")
    {
        "high".to_string()
    } else if entities.len() >= 2 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

fn dark_signal_score(risk: &str, entity_count: usize, source: &Source) -> f64 {
    let base = match risk {
        "high" => 80.0,
        "medium" => 55.0,
        _ => 30.0,
    };
    let tier_bonus = if source.tier == "community_signal" {
        5.0
    } else {
        0.0
    };
    (base + (entity_count.min(6) as f64 * 3.0) + tier_bonus).min(100.0)
}

fn insert_observation(conn: &Connection, obs: &CivicObservation) -> SqlResult<i32> {
    conn.execute(
        "INSERT INTO civic_observations (observation_type, source_id, evidence_id, title, summary, url, observed_at, content_hash, previous_hash, diff_summary, metadata_json, tier)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            obs.observation_type,
            obs.source_id,
            obs.evidence_id,
            obs.title,
            obs.summary,
            obs.url,
            obs.observed_at,
            obs.content_hash,
            obs.previous_hash,
            obs.diff_summary,
            obs.metadata_json,
            obs.tier
        ],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

fn upsert_entity(conn: &Connection, entity: &ExtractedEntity, seen_at: &str) -> SqlResult<i32> {
    let normalized = normalize_entity_name(&entity.name);
    conn.execute(
        "INSERT INTO civic_entities (entity_type, name, normalized_name, first_seen_at, last_seen_at, mention_count)
         VALUES (?1, ?2, ?3, ?4, ?4, 1)
         ON CONFLICT(entity_type, normalized_name) DO UPDATE SET
            name = excluded.name,
            last_seen_at = excluded.last_seen_at,
            mention_count = mention_count + 1",
        params![entity.entity_type, entity.name, normalized, seen_at],
    )?;
    conn.query_row(
        "SELECT id FROM civic_entities WHERE entity_type = ?1 AND normalized_name = ?2",
        params![entity.entity_type, normalized],
        |row| row.get(0),
    )
}

fn link_observation_entity(
    conn: &Connection,
    observation_id: i32,
    entity_id: i32,
    context: Option<&str>,
) -> SqlResult<()> {
    conn.execute(
        "INSERT OR IGNORE INTO civic_observation_entities (observation_id, entity_id, context)
         VALUES (?1, ?2, ?3)",
        params![observation_id, entity_id, context],
    )?;
    Ok(())
}

fn context_for(name: &str, text: &str) -> String {
    let needle = name.to_ascii_lowercase();
    text.lines()
        .find(|line| line.to_ascii_lowercase().contains(&needle))
        .unwrap_or(text)
        .chars()
        .take(220)
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn label_entity_type(value: &str) -> &'static str {
    match value {
        "person" => "Person",
        "company" => "Company",
        "parcel" => "Parcel",
        "address" => "Address",
        "vendor" => "Vendor",
        "agency" => "Agency",
        "money" => "Money amount",
        _ => "Entity",
    }
}

struct SourceScoreUpdate<'a> {
    source_id: i32,
    fetch_success: bool,
    new_items: i32,
    changed_items: i32,
    entity_hits: i32,
    dark_signal_hits: i32,
    last_fetch_at: Option<&'a str>,
}

fn update_source_score(conn: &Connection, update: SourceScoreUpdate<'_>) -> SqlResult<()> {
    if update.source_id == 0 {
        return Ok(());
    }
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO source_performance_scores (source_id, updated_at)
         VALUES (?1, ?2)
         ON CONFLICT(source_id) DO NOTHING",
        params![update.source_id, now],
    )?;
    conn.execute(
        "UPDATE source_performance_scores
         SET fetch_successes = fetch_successes + ?1,
             fetch_failures = fetch_failures + ?2,
             new_items = new_items + ?3,
             changed_items = changed_items + ?4,
             entity_hits = entity_hits + ?5,
             dark_signal_hits = dark_signal_hits + ?6,
             last_fetch_at = COALESCE(?7, last_fetch_at),
             updated_at = ?8
        WHERE source_id = ?9",
        params![
            i32::from(update.fetch_success),
            i32::from(!update.fetch_success),
            update.new_items,
            update.changed_items,
            update.entity_hits,
            update.dark_signal_hits,
            update.last_fetch_at,
            now,
            update.source_id
        ],
    )?;
    recalculate_source_score(conn, update.source_id)
}

fn recalculate_source_score(conn: &Connection, source_id: i32) -> SqlResult<()> {
    let (success, fail, new_items, changed, entities, dark): (i32, i32, i32, i32, i32, i32) =
        conn.query_row(
            "SELECT fetch_successes, fetch_failures, new_items, changed_items, entity_hits, dark_signal_hits
             FROM source_performance_scores WHERE source_id = ?1",
            params![source_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )?;
    let total = (success + fail).max(1) as f64;
    let reliability = ((success as f64 / total) * 100.0).round();
    let usefulness = ((new_items * 4 + changed * 6 + entities + dark * 10) as f64).min(100.0);
    conn.execute(
        "UPDATE source_performance_scores SET reliability_score = ?1, usefulness_score = ?2 WHERE source_id = ?3",
        params![reliability, usefulness, source_id],
    )?;
    Ok(())
}

pub fn previous_hash_for_source_url(
    conn: &Connection,
    source_id: i32,
    url: Option<&str>,
    current_hash: &str,
) -> SqlResult<Option<String>> {
    let Some(url) = url else {
        return Ok(None);
    };
    conn.query_row(
        "SELECT content_hash FROM evidence_items
         WHERE source_id = ?1 AND url = ?2 AND content_hash <> ?3
         ORDER BY fetched_at DESC, id DESC LIMIT 1",
        params![source_id, url, current_hash],
        |row| row.get(0),
    )
    .optional()
}

pub fn list_recent_observations(
    conn: &Connection,
    limit: usize,
) -> SqlResult<Vec<CivicObservation>> {
    let mut stmt = conn.prepare(
        "SELECT id, observation_type, source_id, evidence_id, title, summary, url, observed_at, content_hash, previous_hash, diff_summary, metadata_json, tier
         FROM civic_observations ORDER BY observed_at DESC, id DESC LIMIT ?1",
    )?;
    let iter = stmt.query_map(params![limit as i64], observation_from_row)?;
    iter.collect()
}

pub fn list_top_entities(conn: &Connection, limit: usize) -> SqlResult<Vec<CivicEntity>> {
    let mut stmt = conn.prepare(
        "SELECT id, entity_type, name, normalized_name, first_seen_at, last_seen_at, mention_count
         FROM civic_entities ORDER BY mention_count DESC, last_seen_at DESC LIMIT ?1",
    )?;
    let iter = stmt.query_map(params![limit as i64], entity_from_row)?;
    iter.collect()
}

pub fn list_source_scores(conn: &Connection) -> SqlResult<Vec<SourcePerformanceScore>> {
    let mut stmt = conn.prepare(
        "SELECT sps.source_id, COALESCE(s.name, 'Unknown source'), sps.fetch_successes, sps.fetch_failures,
                sps.new_items, sps.changed_items, sps.entity_hits, sps.dark_signal_hits,
                sps.reliability_score, sps.usefulness_score, sps.last_fetch_at, sps.updated_at
         FROM source_performance_scores sps
         LEFT JOIN sources s ON s.id = sps.source_id
         ORDER BY sps.usefulness_score DESC, sps.reliability_score DESC",
    )?;
    let iter = stmt.query_map([], |row| {
        Ok(SourcePerformanceScore {
            source_id: row.get(0)?,
            source_name: row.get(1)?,
            fetch_successes: row.get(2)?,
            fetch_failures: row.get(3)?,
            new_items: row.get(4)?,
            changed_items: row.get(5)?,
            entity_hits: row.get(6)?,
            dark_signal_hits: row.get(7)?,
            reliability_score: row.get(8)?,
            usefulness_score: row.get(9)?,
            last_fetch_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?;
    iter.collect()
}

pub fn list_dark_signals(conn: &Connection, limit: usize) -> SqlResult<Vec<DarkSignal>> {
    let mut stmt = conn.prepare(
        "SELECT id, observation_id, source_id, title, summary, origin, risk_level, rank_score, tier, evidence_policy, why_it_matters, verification_path, publication_status, created_at, updated_at
         FROM dark_signals
         ORDER BY rank_score DESC, created_at DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(DarkSignal {
            id: Some(row.get(0)?),
            observation_id: row.get(1)?,
            source_id: row.get(2)?,
            title: row.get(3)?,
            summary: row.get(4)?,
            origin: row.get(5)?,
            risk_level: row.get(6)?,
            rank_score: row.get(7)?,
            tier: row.get(8)?,
            evidence_policy: row.get(9)?,
            why_it_matters: row.get(10)?,
            verification_path: row.get(11)?,
            publication_status: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            entities: Vec::new(),
        })
    })?;
    let mut signals = Vec::new();
    for row in rows {
        let mut signal = row?;
        if let Some(observation_id) = signal.observation_id {
            signal.entities = entities_for_observation(conn, observation_id)?;
        }
        signals.push(signal);
    }
    Ok(signals)
}

fn entities_for_observation(conn: &Connection, observation_id: i32) -> SqlResult<Vec<CivicEntity>> {
    let mut stmt = conn.prepare(
        "SELECT e.id, e.entity_type, e.name, e.normalized_name, e.first_seen_at, e.last_seen_at, e.mention_count
         FROM civic_entities e
         JOIN civic_observation_entities oe ON oe.entity_id = e.id
         WHERE oe.observation_id = ?1
         ORDER BY e.entity_type, e.name",
    )?;
    let iter = stmt.query_map(params![observation_id], entity_from_row)?;
    iter.collect()
}

pub fn intelligence_snapshot(conn: &Connection) -> SqlResult<CivicIntelligenceSnapshot> {
    Ok(CivicIntelligenceSnapshot {
        observations: list_recent_observations(conn, 40)?,
        entities: list_top_entities(conn, 40)?,
        source_scores: list_source_scores(conn)?,
        dark_signals: list_dark_signals(conn, 50)?,
    })
}

fn observation_from_row(row: &rusqlite::Row<'_>) -> SqlResult<CivicObservation> {
    Ok(CivicObservation {
        id: Some(row.get(0)?),
        observation_type: row.get(1)?,
        source_id: row.get(2)?,
        evidence_id: row.get(3)?,
        title: row.get(4)?,
        summary: row.get(5)?,
        url: row.get(6)?,
        observed_at: row.get(7)?,
        content_hash: row.get(8)?,
        previous_hash: row.get(9)?,
        diff_summary: row.get(10)?,
        metadata_json: row.get(11)?,
        tier: row.get(12)?,
    })
}

fn entity_from_row(row: &rusqlite::Row<'_>) -> SqlResult<CivicEntity> {
    Ok(CivicEntity {
        id: Some(row.get(0)?),
        entity_type: row.get(1)?,
        name: row.get(2)?,
        normalized_name: row.get(3)?,
        first_seen_at: row.get(4)?,
        last_seen_at: row.get(5)?,
        mention_count: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn extracts_civic_entity_types() {
        let text = "Councilmember Jane Smith discussed parcel APN 123-456-789 near 1200 Main St with Acme Development LLC and the Planning Commission for $450,000.";
        let entities = extract_civic_entities(text);
        assert!(entities.iter().any(|e| e.entity_type == "person"));
        assert!(entities.iter().any(|e| e.entity_type == "parcel"));
        assert!(entities.iter().any(|e| e.entity_type == "address"));
        assert!(entities.iter().any(|e| e.entity_type == "company"));
        assert!(entities.iter().any(|e| e.entity_type == "agency"));
        assert!(entities.iter().any(|e| e.entity_type == "money"));
    }

    #[test]
    fn records_dark_signal_without_hiding_it() {
        let mut conn = Connection::open_in_memory().unwrap();
        crate::core::migrations::run_migrations(&mut conn).unwrap();
        let source = Source {
            id: Some(
                crate::core::db::insert_source(
                    &conn,
                    &Source {
                        id: None,
                        name: "Town subreddit".to_string(),
                        url: "https://reddit.com/r/town".to_string(),
                        r#type: "community_signal".to_string(),
                        status: "online".to_string(),
                        tier: "community_signal".to_string(),
                        last_success_at: None,
                        last_failed_at: None,
                        last_scraped: None,
                    },
                )
                .unwrap(),
            ),
            name: "Town subreddit".to_string(),
            url: "https://reddit.com/r/town".to_string(),
            r#type: "community_signal".to_string(),
            status: "online".to_string(),
            tier: "community_signal".to_string(),
            last_success_at: None,
            last_failed_at: None,
            last_scraped: None,
        };
        let item = EvidenceItem {
            id: None,
            source_id: source.id.unwrap(),
            url: Some(source.url.clone()),
            fetched_at: Utc::now().to_rfc3339(),
            excerpt: "Rumor says out of state shell company Acme Development LLC bought parcel APN 123-456-789 quietly.".to_string(),
            content_hash: "hash-dark".to_string(),
            entities: "[]".to_string(),
        };
        let evidence_id = crate::core::db::insert_evidence_item(&conn, &item).unwrap();
        record_evidence_intelligence(&conn, &source, &item, evidence_id, None).unwrap();

        let signals = list_dark_signals(&conn, 10).unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].evidence_policy, "editor_review_only");
        assert_eq!(signals[0].publication_status, "review");
        assert!(signals[0].rank_score > 0.0);
        assert!(!signals[0].entities.is_empty());
    }
}
