// core/daily_scan.rs
use crate::core::db::{self, DailyScanLead, DailyScanRun, DbConn, EvidenceItem, Lead};
use crate::core::llm::LlmClient;
use crate::core::{intelligence, verification};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Distinct error signal returned by [`run_daily_scan`] when there is zero
/// evidence in the requested window, so the scan is reported as a real `Err`
/// rather than an empty-but-successful run (QA-M2). The `NO_EVIDENCE:` prefix is
/// a typed marker the frontend's `toUserMessage` (src/ipc.ts) recognizes: it
/// strips the prefix and surfaces the plain "run Scrape & Detect first" guidance
/// instead of a raw "Something went wrong: NO_EVIDENCE: ..." leak.
pub const NO_EVIDENCE_SIGNAL: &str =
    "NO_EVIDENCE: No recent evidence was found after checking sources. Add sources, fix offline sources, or widen the scan window.";

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResultItem {
    pub title: String,
    pub summary: String,
    pub original_url: String,
    #[serde(default)]
    pub why_flagged: Option<String>,
    #[serde(default)]
    pub source_name: Option<String>,
    #[serde(default)]
    pub source_type: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub suggested_next_step: Option<String>,
    #[serde(default)]
    pub story_type: Option<String>,
    #[serde(default)]
    pub what_changed: Option<String>,
    #[serde(default)]
    pub immediacy: Option<u8>,
    #[serde(default)]
    pub impact: Option<u8>,
    #[serde(default)]
    pub conflict: Option<u8>,
    #[serde(default)]
    pub novelty: Option<u8>,
    #[serde(default)]
    pub what_would_make_it_publishable: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResult {
    pub leads: Vec<ScanResultItem>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DailyScanProgress {
    pub stage: String,
    pub message: String,
    pub run_id: Option<i32>,
    pub model: Option<String>,
    pub evidence_count: usize,
    pub batch_index: Option<usize>,
    pub batch_count: Option<usize>,
    pub saved_leads: usize,
}

pub fn parse_and_save_scan_response(
    conn: &rusqlite::Connection,
    run_id: i32,
    json_response: &str,
) -> Result<usize, String> {
    // The model output is frequently raw JSON, but small local models sometimes
    // wrap it in a ```json fence or add prose. Strip a leading/trailing code
    // fence before parsing so a well-formed-but-fenced response still succeeds;
    // anything still unparseable propagates as a real error (QA-M2).
    let cleaned = extract_json_object(strip_json_fence(json_response))?;
    let result: ScanResult = serde_json::from_str(cleaned).map_err(|e| e.to_string())?;
    let mut saved = 0;
    for item in result.leads {
        let story_type = normalize_story_type(item.story_type.clone());
        let what_changed = normalize_optional(item.what_changed.clone());
        let immediacy = clamped_score_i32(item.immediacy);
        let impact = clamped_score_i32(item.impact);
        let conflict = clamped_score_i32(item.conflict);
        let novelty = clamped_score_i32(item.novelty);
        let publishability_note = normalize_optional(item.what_would_make_it_publishable.clone());
        let disposition = classify_disposition(
            story_type.as_deref(),
            what_changed.as_deref(),
            immediacy,
            impact,
            novelty,
        );
        let quality_note = newsworthiness_note(&item);
        let next_step_note = publishability_next_step(&item);
        let why_flagged = append_editor_note(
            normalize_optional(item.why_flagged.clone()).unwrap_or_else(|| {
                "The local scan found language that may deserve an editor's review.".to_string()
            }),
            quality_note,
        );
        let suggested_next_step = append_editor_note(
            normalize_optional(item.suggested_next_step.clone())
                .unwrap_or_else(|| "Open the original source and confirm the key dates, names, and decision points before drafting.".to_string()),
            next_step_note,
        );
        // ENG-Min4: the model-asserted `original_url` is untrusted. Validate it
        // against the same scheme/host allowlist used for real sources before
        // persisting; a hallucinated/poisoned URL must not enter the evidence
        // trail unvetted. Invalid URLs are stored as empty (the lead is kept,
        // but the bogus URL is dropped and logged) so the model's text is not
        // silently treated as a verified link.
        let original_url = match crate::core::scraper::validate_source_url(&item.original_url) {
            Ok(_) => item.original_url,
            Err(e) => {
                eprintln!(
                    "Dropping invalid model-asserted original_url '{}': {}",
                    item.original_url, e
                );
                String::new()
            }
        };
        let lead = DailyScanLead {
            id: None,
            scan_id: run_id,
            title: item.title,
            summary: item.summary,
            source_id: None, // Assume None, aggregated logic (D5)
            original_url,
            why_flagged: Some(why_flagged),
            source_name: normalize_optional(item.source_name)
                .or_else(|| Some("Watched sources".to_string())),
            source_type: normalize_optional(item.source_type),
            priority: normalize_priority(item.priority),
            suggested_next_step: Some(suggested_next_step),
            story_type,
            what_changed,
            immediacy,
            impact,
            conflict,
            novelty,
            publishability_note,
            disposition: Some(disposition),
        };
        match save_daily_scan_lead_for_queue(conn, &lead) {
            Ok(id) if id > 0 => saved += 1,
            Ok(_) => {}
            Err(e) => eprintln!("Failed to insert daily scan lead: {}", e), // P4-009 log insert error
        }
    }
    Ok(saved)
}

fn append_editor_note(base: String, note: Option<String>) -> String {
    match note {
        Some(note) if !note.trim().is_empty() => {
            format!("{}. {}", base.trim_end_matches('.'), note)
        }
        _ => base,
    }
}

fn clamped_score(score: Option<u8>) -> Option<u8> {
    score.map(|score| score.clamp(1, 5))
}

fn clamped_score_i32(score: Option<u8>) -> Option<i32> {
    clamped_score(score).map(i32::from)
}

fn normalize_story_type(value: Option<String>) -> Option<String> {
    let normalized = normalize_optional(value)?.to_lowercase().replace('_', " ");
    match normalized.as_str() {
        "story" | "article" | "reported story" | "hard news" => Some("story".to_string()),
        "brief" | "short brief" => Some("brief".to_string()),
        "watch" | "watchlist" | "watch item" => Some("watch".to_string()),
        "background" | "evergreen" | "background note" => Some("background".to_string()),
        "verification" | "needs verification" | "verification assignment" => {
            Some("verification".to_string())
        }
        _ => Some("verification".to_string()),
    }
}

fn classify_disposition(
    story_type: Option<&str>,
    what_changed: Option<&str>,
    immediacy: Option<i32>,
    impact: Option<i32>,
    novelty: Option<i32>,
) -> String {
    let change_text = what_changed.unwrap_or_default().to_lowercase();
    let no_current_change = [
        "no current change",
        "no new fact",
        "not current",
        "background",
        "evergreen",
    ]
    .iter()
    .any(|needle| change_text.contains(needle));

    match story_type.unwrap_or("verification") {
        "background" => "background".to_string(),
        "watch" => "watch".to_string(),
        "verification" => "needs_verification".to_string(),
        "story" | "brief" if no_current_change => "background".to_string(),
        "story" | "brief" => {
            let novelty = novelty.unwrap_or(0);
            let urgency = immediacy.unwrap_or(0) + impact.unwrap_or(0);
            if novelty >= 3 && urgency >= 5 {
                "ready_to_draft".to_string()
            } else {
                "review".to_string()
            }
        }
        _ => "review".to_string(),
    }
}

fn newsworthiness_note(item: &ScanResultItem) -> Option<String> {
    let scores = [
        clamped_score(item.immediacy),
        clamped_score(item.impact),
        clamped_score(item.conflict),
        clamped_score(item.novelty),
    ];
    let total = scores.iter().flatten().copied().sum::<u8>();
    let score_note = if scores.iter().all(Option::is_some) {
        Some(format!(
            "Newsworthiness: {}/20 (immediacy {}, impact {}, conflict {}, novelty {}).",
            total,
            scores[0].unwrap(),
            scores[1].unwrap(),
            scores[2].unwrap(),
            scores[3].unwrap()
        ))
    } else {
        None
    };
    let story_type = normalize_story_type(item.story_type.clone())
        .map(|value| format!("Suggested treatment: {}.", value));
    let what_changed = normalize_optional(item.what_changed.clone())
        .map(|value| format!("Why now: {}.", value.trim_end_matches('.')));

    let parts = [story_type, score_note, what_changed]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn publishability_next_step(item: &ScanResultItem) -> Option<String> {
    normalize_optional(item.what_would_make_it_publishable.clone()).map(|value| {
        format!(
            "What would make this publishable: {}.",
            value.trim_end_matches('.')
        )
    })
}

/// Strip a surrounding markdown code fence (```json ... ``` or ``` ... ```) from
/// a model response, returning the inner JSON. Returns the trimmed input
/// unchanged when there is no fence.
fn strip_json_fence(s: &str) -> &str {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("```") {
        // Drop an optional language tag on the first line (e.g. "json").
        let rest = match rest.find('\n') {
            Some(nl) => &rest[nl + 1..],
            None => rest,
        };
        rest.trim().strip_suffix("```").unwrap_or(rest).trim()
    } else {
        t
    }
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_priority(value: Option<String>) -> Option<String> {
    let normalized = normalize_optional(value)?.to_lowercase();
    match normalized.as_str() {
        "high" | "medium" | "low" => Some(normalized),
        "med" => Some("medium".to_string()),
        _ => Some("review".to_string()),
    }
}

fn extract_json_object(s: &str) -> Result<&str, String> {
    let trimmed = s.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        return Ok(trimmed);
    }

    let start = trimmed
        .find('{')
        .ok_or_else(|| "model response did not contain a JSON object".to_string())?;
    let end = trimmed
        .rfind('}')
        .ok_or_else(|| "model response did not contain a complete JSON object".to_string())?;
    if end <= start {
        return Err("model response did not contain a complete JSON object".to_string());
    }
    Ok(&trimmed[start..=end])
}

fn scan_progress(stage: &str, message: impl Into<String>) -> DailyScanProgress {
    DailyScanProgress {
        stage: stage.to_string(),
        message: message.into(),
        run_id: None,
        model: None,
        evidence_count: 0,
        batch_index: None,
        batch_count: None,
        saved_leads: 0,
    }
}

fn build_batch_prompt(
    city: &str,
    state: &str,
    batch_index: usize,
    batch: &[EvidenceItem],
) -> String {
    let mut context = String::new();
    for (idx, item) in batch.iter().enumerate() {
        context.push_str(&format!(
            "Evidence {}.{}\nSource ID: {}\nOriginal URL: {}\nExcerpt: {}\n\n",
            batch_index + 1,
            idx + 1,
            item.source_id,
            item.url.as_deref().unwrap_or(""),
            item.excerpt
        ));
    }

    format!(
        "City: {city}, State: {state}\n\
         Batch: {batch_number}\n\n\
         Evidence Context:\n{context}\n\
         Return ONLY valid JSON. No markdown. No prose. No code fence.\n\
         Schema: {{\"leads\":[{{\"title\":\"short civic lead title\",\"summary\":\"1-2 evidence-grounded sentences\",\"original_url\":\"source URL from evidence or empty string\",\"why_flagged\":\"plain-language reason this deserves review\",\"source_name\":\"name or short description of source\",\"source_type\":\"agenda, public notice, budget, official update, community signal, or unknown\",\"priority\":\"high, medium, or low\",\"suggested_next_step\":\"specific editor action before drafting\",\"story_type\":\"story, brief, watch, background, or verification\",\"what_changed\":\"specific current fact that makes this timely, or 'no current change found'\",\"immediacy\":1,\"impact\":1,\"conflict\":1,\"novelty\":1,\"what_would_make_it_publishable\":\"specific missing fact, document, interview, vote, deadline, public effect, or cross-check\"}}]}}\n\
         Score immediacy, impact, conflict, and novelty from 1 to 5. A recurring meeting page, archive page, general service page, or newly fetched but unchanged source should score low on immediacy and novelty and should usually be story_type background or watch, not story.\n\
         Include at most 3 leads. Use an empty leads array if nothing deserves an editor's look.\n\
         Explain why each lead matters to a local civic reporter. Avoid vague reasons like 'interesting item'. Do not hide weak leads; label them honestly so the editor can decide.",
        city = city,
        state = state,
        batch_number = batch_index + 1,
        context = context
    )
}

fn repair_prompt(raw: &str) -> String {
    format!(
        "Repair the following model output into ONLY valid JSON matching this schema: \
         {{\"leads\":[{{\"title\":\"...\",\"summary\":\"...\",\"original_url\":\"...\",\"why_flagged\":\"...\",\"source_name\":\"...\",\"source_type\":\"...\",\"priority\":\"high|medium|low\",\"suggested_next_step\":\"...\",\"story_type\":\"story|brief|watch|background|verification\",\"what_changed\":\"...\",\"immediacy\":1,\"impact\":1,\"conflict\":1,\"novelty\":1,\"what_would_make_it_publishable\":\"...\"}}]}}. \
         Do not add markdown or explanation.\n\nOutput to repair:\n{}",
        raw
    )
}

fn save_fallback_leads(
    conn: &rusqlite::Connection,
    run_id: i32,
    evidence_items: &[EvidenceItem],
) -> usize {
    let mut saved = 0;
    for item in evidence_items.iter().take(5) {
        let source = db::get_source(conn, item.source_id).ok().flatten();
        let source_name = source
            .as_ref()
            .map(|source| source.name.clone())
            .unwrap_or_else(|| format!("Source #{}", item.source_id));
        let source_type = source.as_ref().map(|source| source.r#type.clone());
        let excerpt = item.excerpt.replace('\n', " ");
        let summary = if excerpt.chars().count() > 260 {
            format!("{}...", excerpt.chars().take(260).collect::<String>())
        } else {
            excerpt
        };
        let lead = DailyScanLead {
            id: None,
            scan_id: run_id,
            title: format!("Review new source evidence from source #{}", item.source_id),
            summary,
            source_id: Some(item.source_id),
            original_url: item.url.clone().unwrap_or_default(),
            why_flagged: Some("The model did not return usable JSON, so this evidence was preserved for editor review instead of being discarded.".to_string()),
            source_name: Some(source_name),
            source_type,
            priority: Some("review".to_string()),
            suggested_next_step: Some("Open the source, decide whether it contains a reportable civic action, then save or dismiss it from the queue.".to_string()),
            story_type: Some("verification".to_string()),
            what_changed: None,
            immediacy: None,
            impact: None,
            conflict: None,
            novelty: None,
            publishability_note: Some("A specific current fact, document, vote, deadline, impact, or cross-check is needed before publication.".to_string()),
            disposition: Some("needs_verification".to_string()),
        };
        if save_daily_scan_lead_for_queue(conn, &lead).unwrap_or(0) > 0 {
            saved += 1;
        }
    }
    saved
}

fn run_deterministic_intelligence_pass(
    conn: &rusqlite::Connection,
    run_id: i32,
    evidence_items: &[EvidenceItem],
) -> Result<usize, String> {
    let mut evidence_ids = Vec::new();
    for item in evidence_items {
        let Some(evidence_id) = item.id else {
            continue;
        };
        evidence_ids.push(evidence_id);
        let already_processed: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM civic_observations WHERE evidence_id = ?1 AND content_hash = ?2",
                rusqlite::params![evidence_id, item.content_hash],
                |row| row.get(0),
            )
            .unwrap_or(0);
        if already_processed > 0 {
            continue;
        }
        if let Some(source) = db::get_source(conn, item.source_id).map_err(|e| e.to_string())? {
            let previous_hash = intelligence::previous_hash_for_source_url(
                conn,
                item.source_id,
                item.url.as_deref(),
                &item.content_hash,
            )
            .map_err(|e| e.to_string())?;
            intelligence::record_evidence_intelligence(
                conn,
                &source,
                item,
                evidence_id,
                previous_hash,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    verification::generate_verification_tasks(conn).map_err(|e| e.to_string())?;
    save_dark_signal_leads(conn, run_id, &evidence_ids)
}

fn save_dark_signal_leads(
    conn: &rusqlite::Connection,
    run_id: i32,
    evidence_ids: &[i32],
) -> Result<usize, String> {
    let signals = intelligence::list_dark_signals(conn, 8).map_err(|e| e.to_string())?;
    let mut saved = 0;
    for signal in signals
        .into_iter()
        .filter(|signal| signal.publication_status != "dismissed")
        .filter(|signal| {
            signal
                .observation_id
                .and_then(|observation_id| {
                    conn.query_row(
                        "SELECT evidence_id FROM civic_observations WHERE id = ?1",
                        [observation_id],
                        |row| row.get::<_, Option<i32>>(0),
                    )
                    .ok()
                    .flatten()
                })
                .map(|evidence_id| evidence_ids.contains(&evidence_id))
                .unwrap_or(false)
        })
        .take(5)
    {
        let original_url: String = conn
            .query_row(
                "SELECT COALESCE(co.url, s.url, '')
                 FROM dark_signals ds
                 LEFT JOIN civic_observations co ON co.id = ds.observation_id
                 LEFT JOIN sources s ON s.id = ds.source_id
                 WHERE ds.id = ?1",
                [signal.id.unwrap_or_default()],
                |row| row.get(0),
            )
            .unwrap_or_default();
        let lead = DailyScanLead {
            id: None,
            scan_id: run_id,
            title: signal.title,
            summary: signal.summary,
            source_id: signal.source_id,
            original_url,
            why_flagged: Some(signal.why_it_matters),
            source_name: Some(signal.origin),
            source_type: Some(signal.tier),
            priority: Some(
                match signal.risk_level.as_str() {
                    "high" => "high",
                    "medium" => "medium",
                    _ => "low",
                }
                .to_string(),
            ),
            suggested_next_step: Some(signal.verification_path),
            story_type: Some("verification".to_string()),
            what_changed: Some("Dark-signal pattern detected from recent evidence.".to_string()),
            immediacy: Some(2),
            impact: Some(match signal.risk_level.as_str() {
                "high" => 4,
                "medium" => 3,
                _ => 2,
            }),
            conflict: None,
            novelty: None,
            publishability_note: Some("Confirm the signal against source records or a second public source before drafting.".to_string()),
            disposition: Some("needs_verification".to_string()),
        };
        if save_daily_scan_lead_for_queue(conn, &lead).unwrap_or(0) > 0 {
            saved += 1;
        }
    }
    Ok(saved)
}

fn save_daily_scan_lead_for_queue(
    conn: &rusqlite::Connection,
    lead: &DailyScanLead,
) -> rusqlite::Result<i32> {
    if similar_scan_lead_exists(conn, lead)? {
        return Ok(0);
    }
    let recurring_memory = find_recurring_memory(conn, lead)?;
    let lead = lead_with_beat_memory(lead, recurring_memory.as_ref());
    let scan_lead_id = db::insert_daily_scan_lead(conn, &lead)?;
    upsert_beat_memory(conn, scan_lead_id, &lead)?;
    let title = lead.title.trim();
    let summary = lead.summary.trim();
    let mut why = match (title.is_empty(), summary.is_empty()) {
        (false, false) => format!("{title}: {summary}"),
        (false, true) => title.to_string(),
        (true, false) => summary.to_string(),
        (true, true) => "Daily Scan found a lead that needs editor review.".to_string(),
    };
    if let Some(context) = lead
        .why_flagged
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        why.push_str(&format!(" Editor context: {context}"));
    }
    if let Some(next_step) = lead
        .suggested_next_step
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        why.push_str(&format!(" Suggested next step: {next_step}"));
    }
    let priority = lead.priority.as_deref().unwrap_or("review");
    let normalized_level = match priority {
        "high" => "high",
        "medium" | "med" => "med",
        _ => "low",
    };
    let checklist = serde_json::json!([
        lead.suggested_next_step
            .as_deref()
            .unwrap_or("Open the original source and confirm the key facts before drafting."),
        "Confirm dates, names, dollar amounts, and decision points before publication.",
        "Keep unverified or single-source claims labeled for editor review."
    ])
    .to_string();
    let story_lead = Lead {
        id: None,
        detector_name: "daily_scan".to_string(),
        why,
        confidence: normalized_level.to_string(),
        risk_level: normalized_level.to_string(),
        confirmation_checklist: checklist,
        from_scan_lead_id: Some(scan_lead_id),
        story_type: lead.story_type.clone(),
        disposition: lead
            .disposition
            .clone()
            .or_else(|| Some("review".to_string())),
        novelty_score: lead.novelty,
        novelty_reason: lead.what_changed.clone(),
        created_at: String::new(),
    };
    let evidence_ids = evidence_ids_for_scan_lead(conn, &lead)?;
    db::insert_lead(conn, &story_lead, &evidence_ids)?;
    Ok(scan_lead_id)
}

#[derive(Debug)]
struct BeatMemory {
    representative_title: String,
    first_seen_at: String,
    last_seen_at: String,
    seen_count: i32,
}

fn lead_with_beat_memory(lead: &DailyScanLead, memory: Option<&BeatMemory>) -> DailyScanLead {
    let Some(memory) = memory else {
        return lead.clone();
    };

    let mut lead = lead.clone();
    let seen_note = format!(
        "Beat memory: similar topic '{}' was first seen {}, last seen {}, and has appeared {} previous time(s). Treat this as recurring/background unless the source shows a new vote, deadline, dollar amount, filing, outage, meeting item, or public impact.",
        memory.representative_title, memory.first_seen_at, memory.last_seen_at, memory.seen_count
    );
    lead.why_flagged = Some(append_editor_note(
        lead.why_flagged
            .unwrap_or_else(|| "This lead deserves editor review.".to_string()),
        Some(seen_note),
    ));
    lead.suggested_next_step = Some(append_editor_note(
        lead.suggested_next_step.unwrap_or_else(|| {
            "Open the original source and confirm whether anything has changed since the previous scan.".to_string()
        }),
        Some("Compare against beat memory before drafting; write a story only if there is a new reportable fact.".to_string()),
    ));
    if lead.priority.as_deref().unwrap_or("review") != "high"
        && looks_like_background_or_unchanged(&lead)
    {
        lead.priority = Some("low".to_string());
        lead.disposition = Some("background".to_string());
    }
    lead
}

fn looks_like_background_or_unchanged(lead: &DailyScanLead) -> bool {
    let text = format!(
        "{} {} {}",
        lead.why_flagged.as_deref().unwrap_or_default(),
        lead.suggested_next_step.as_deref().unwrap_or_default(),
        lead.summary
    )
    .to_lowercase();
    [
        "no current change found",
        "suggested treatment: background",
        "suggested treatment: watch",
        "recurring",
        "archive page",
        "general service page",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn find_recurring_memory(
    conn: &rusqlite::Connection,
    lead: &DailyScanLead,
) -> rusqlite::Result<Option<BeatMemory>> {
    let mut stmt = conn.prepare(
        "SELECT representative_title, source_url, first_seen_at, last_seen_at, seen_count, last_summary
         FROM beat_memory
         ORDER BY last_seen_at DESC
         LIMIT 200",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i32>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    for row in rows {
        let (
            representative_title,
            source_url,
            first_seen_at,
            last_seen_at,
            seen_count,
            last_summary,
        ) = row?;
        if scan_leads_are_similar(&representative_title, &last_summary, &source_url, lead) {
            return Ok(Some(BeatMemory {
                representative_title,
                first_seen_at,
                last_seen_at,
                seen_count,
            }));
        }
    }

    Ok(None)
}

fn upsert_beat_memory(
    conn: &rusqlite::Connection,
    scan_lead_id: i32,
    lead: &DailyScanLead,
) -> rusqlite::Result<()> {
    let topic_key = memory_topic_key(lead);
    if topic_key.is_empty() {
        return Ok(());
    }
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO beat_memory (
            topic_key, representative_title, source_url, first_seen_at, last_seen_at,
            seen_count, last_scan_lead_id, last_summary
         )
         VALUES (?1, ?2, ?3, ?4, ?4, 1, ?5, ?6)
         ON CONFLICT(topic_key) DO UPDATE SET
            last_seen_at = excluded.last_seen_at,
            seen_count = beat_memory.seen_count + 1,
            last_scan_lead_id = excluded.last_scan_lead_id,
            last_summary = excluded.last_summary",
        rusqlite::params![
            topic_key,
            lead.title.trim(),
            lead.original_url.trim(),
            now,
            scan_lead_id,
            lead.summary.trim()
        ],
    )?;
    Ok(())
}

fn memory_topic_key(lead: &DailyScanLead) -> String {
    let url = lead.original_url.trim().to_lowercase();
    if !url.is_empty() {
        return format!("url:{url}");
    }
    let topic = normalized_topic(&scan_lead_topic_text(
        &lead.title,
        &lead.summary,
        &lead.original_url,
    ));
    if topic.is_empty() {
        String::new()
    } else {
        format!("topic:{topic}")
    }
}

fn normalized_topic(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || ch.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
}

fn topic_tokens(text: &str) -> std::collections::BTreeSet<String> {
    const STOPWORDS: &[&str] = &[
        "a", "an", "and", "are", "as", "at", "be", "by", "can", "for", "from", "in", "into", "is",
        "it", "its", "of", "on", "or", "that", "the", "their", "this", "to", "with",
    ];

    text.to_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch.is_whitespace() {
                ch
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .filter(|token| token.len() > 2)
        .filter(|token| !STOPWORDS.contains(token))
        .map(|token| {
            token
                .trim_end_matches('s')
                .trim_end_matches("ing")
                .to_string()
        })
        .filter(|token| token.len() > 2)
        .collect()
}

fn scan_lead_topic_text(title: &str, summary: &str, original_url: &str) -> String {
    format!("{title} {summary} {original_url}")
}

fn scan_leads_are_similar(
    existing_title: &str,
    existing_summary: &str,
    existing_url: &str,
    lead: &DailyScanLead,
) -> bool {
    let existing_url = existing_url.trim();
    let lead_url = lead.original_url.trim();
    if !existing_url.is_empty() && existing_url.eq_ignore_ascii_case(lead_url) {
        return true;
    }

    let existing_tokens = topic_tokens(&scan_lead_topic_text(
        existing_title,
        existing_summary,
        existing_url,
    ));
    let lead_tokens = topic_tokens(&scan_lead_topic_text(
        &lead.title,
        &lead.summary,
        &lead.original_url,
    ));
    if existing_tokens.len() < 4 || lead_tokens.len() < 4 {
        return normalized_topic(existing_title) == normalized_topic(&lead.title);
    }

    let common = existing_tokens.intersection(&lead_tokens).count();
    let smaller = existing_tokens.len().min(lead_tokens.len());
    common >= 5 && (common as f32 / smaller as f32) >= 0.58
}

fn similar_scan_lead_exists(
    conn: &rusqlite::Connection,
    lead: &DailyScanLead,
) -> rusqlite::Result<bool> {
    let topic = normalized_topic(&lead.title);
    if topic.is_empty() {
        return Ok(false);
    }
    let mut stmt = conn
        .prepare("SELECT title, summary, original_url FROM daily_scan_leads WHERE scan_id = ?1")?;
    let rows = stmt.query_map([lead.scan_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    for row in rows {
        let (title, summary, original_url) = row?;
        if scan_leads_are_similar(&title, &summary, &original_url, lead) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn evidence_ids_for_scan_lead(
    conn: &rusqlite::Connection,
    lead: &DailyScanLead,
) -> rusqlite::Result<Vec<i32>> {
    let mut ids = Vec::new();
    if let Some(source_id) = lead.source_id {
        let mut stmt = conn.prepare(
            "SELECT id FROM evidence_items WHERE source_id = ?1 ORDER BY fetched_at DESC LIMIT 3",
        )?;
        let rows = stmt.query_map([source_id], |row| row.get::<_, i32>(0))?;
        for row in rows {
            ids.push(row?);
        }
    }
    if ids.is_empty() && !lead.original_url.trim().is_empty() {
        let mut stmt = conn.prepare(
            "SELECT id FROM evidence_items WHERE url = ?1 ORDER BY fetched_at DESC LIMIT 3",
        )?;
        let rows = stmt.query_map([lead.original_url.trim()], |row| row.get::<_, i32>(0))?;
        for row in rows {
            ids.push(row?);
        }
    }
    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}

#[cfg(test)]
pub async fn run_daily_scan(
    db: &DbConn,
    llm_client: &std::sync::Arc<dyn LlmClient>,
    prompt_template: &str,
    city: &str,
    state: &str,
    since_hours: u32,
) -> Result<i32, String> {
    run_daily_scan_with_progress(
        db,
        llm_client,
        prompt_template,
        city,
        state,
        since_hours,
        |_| {},
    )
    .await
}

pub async fn run_daily_scan_with_progress<F>(
    db: &DbConn,
    llm_client: &std::sync::Arc<dyn LlmClient>,
    prompt_template: &str,
    city: &str,
    state: &str,
    since_hours: u32,
    mut progress: F,
) -> Result<i32, String>
where
    F: FnMut(DailyScanProgress),
{
    // Validate bounds
    if since_hours == 0 || since_hours > 168 {
        return Err("since_hours must be between 1 and 168".to_string());
    }

    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let regex = RE.get_or_init(|| regex::Regex::new(r"^[A-Za-z][A-Za-z .'-]{0,63}$").unwrap());
    if !regex.is_match(city) || !regex.is_match(state) {
        return Err("Invalid city or state format".to_string());
    }

    progress(scan_progress(
        "preflight",
        "Checking recent evidence for the daily scan.",
    ));

    let run = DailyScanRun {
        id: None,
        started_at: Utc::now().to_rfc3339(),
        completed_at: None,
        run_status: "in_progress".to_string(),
    };

    // QA-M2: short-circuit when there is no evidence in the window BEFORE creating
    // a run row or calling the LLM. A daily scan over zero evidence can only return
    // "no leads," so spending a (potentially minute-long, CPU-bound) LLM round trip
    // to produce nothing is wasted work — and on first run / a fresh DB it is the
    // common case. Return a distinct, recognizable signal so the UI can tell the
    // user to "Scrape & Detect first" instead of reporting an empty success. No run
    // row is created, so this does not pollute the scan history with empty runs.
    let evidence_items = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        db::list_evidence_since(&conn, since_hours).unwrap_or_default()
    };

    if evidence_items.is_empty() {
        return Err(NO_EVIDENCE_SIGNAL.to_string());
    }

    progress(DailyScanProgress {
        evidence_count: evidence_items.len(),
        ..scan_progress(
            "preparing",
            format!(
                "Preparing {} evidence item(s) for local AI review.",
                evidence_items.len()
            ),
        )
    });

    let run_id = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        db::insert_daily_scan_run(&conn, &run).map_err(|e| e.to_string())?
    };

    let model = crate::tauri_cmds::get_selected_model_or_fallback(db).await;
    let scan_items: Vec<EvidenceItem> = evidence_items.into_iter().take(20).collect();
    let deterministic_saved = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        progress(DailyScanProgress {
            run_id: Some(run_id),
            evidence_count: scan_items.len(),
            ..scan_progress(
                "deterministic",
                "Extracting entities, detecting changes, and building verification tasks.",
            )
        });
        run_deterministic_intelligence_pass(&conn, run_id, &scan_items)?
    };
    let batches: Vec<&[EvidenceItem]> = scan_items.chunks(4).collect();
    let batch_count = batches.len();

    progress(DailyScanProgress {
        run_id: Some(run_id),
        model: Some(model.clone()),
        evidence_count: scan_items.len(),
        batch_count: Some(batch_count),
        saved_leads: deterministic_saved,
        ..scan_progress(
            "generating",
            format!(
                "Deterministic pass saved {} lead(s). Starting targeted AI review with {} across {} batch(es).",
                deterministic_saved, model, batch_count
            ),
        )
    });

    let mut saved_total = deterministic_saved;
    let mut parsed_batches = 0usize;
    let mut batch_errors = Vec::new();

    for (batch_index, batch) in batches.iter().enumerate() {
        progress(DailyScanProgress {
            run_id: Some(run_id),
            model: Some(model.clone()),
            evidence_count: scan_items.len(),
            batch_index: Some(batch_index + 1),
            batch_count: Some(batch_count),
            saved_leads: saved_total,
            ..scan_progress(
                "generating",
                format!("Scanning batch {} of {}.", batch_index + 1, batch_count),
            )
        });

        let batch_prompt = build_batch_prompt(city, state, batch_index, batch);
        let llm_res = llm_client
            .call_json(&model, &batch_prompt, prompt_template)
            .await;

        let json_response = match llm_res {
            Ok(response) => response,
            Err(e) => {
                batch_errors.push(format!("batch {} failed: {}", batch_index + 1, e));
                continue;
            }
        };

        let first_parse = {
            let conn = db.lock().map_err(|_| "Failed to lock db")?;
            parse_and_save_scan_response(&conn, run_id, &json_response)
        };

        let saved = match first_parse {
            Ok(saved) => Ok(saved),
            Err(first_err) => {
                progress(DailyScanProgress {
                    run_id: Some(run_id),
                    model: Some(model.clone()),
                    evidence_count: scan_items.len(),
                    batch_index: Some(batch_index + 1),
                    batch_count: Some(batch_count),
                    saved_leads: saved_total,
                    ..scan_progress(
                        "parsing",
                        format!("Repairing JSON for batch {}.", batch_index + 1),
                    )
                });
                match llm_client
                    .call_json(&model, &repair_prompt(&json_response), prompt_template)
                    .await
                {
                    Ok(repaired) => {
                        let conn = db.lock().map_err(|_| "Failed to lock db")?;
                        parse_and_save_scan_response(&conn, run_id, &repaired).map_err(
                            |repair_err| {
                                format!(
                                    "parse failed: {}; repair failed: {}",
                                    first_err, repair_err
                                )
                            },
                        )
                    }
                    Err(repair_call_err) => Err(format!(
                        "parse failed: {}; repair call failed: {}",
                        first_err, repair_call_err
                    )),
                }
            }
        };

        match saved {
            Ok(saved) => {
                parsed_batches += 1;
                saved_total += saved;
                progress(DailyScanProgress {
                    run_id: Some(run_id),
                    model: Some(model.clone()),
                    evidence_count: scan_items.len(),
                    batch_index: Some(batch_index + 1),
                    batch_count: Some(batch_count),
                    saved_leads: saved_total,
                    ..scan_progress(
                        "saving",
                        format!("Saved {} lead(s) from batch {}.", saved, batch_index + 1),
                    )
                });
            }
            Err(e) => batch_errors.push(format!("batch {} {}", batch_index + 1, e)),
        }
    }

    let conn = match db.lock() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to lock db for scan update (orphan run): {}", e);
            return Err("Failed to lock db".to_string());
        }
    };
    let mut updated_run = run.clone();
    updated_run.id = Some(run_id);
    updated_run.completed_at = Some(Utc::now().to_rfc3339());

    let model_returned_usable_json = parsed_batches > 0;
    if saved_total == 0 && !model_returned_usable_json {
        progress(DailyScanProgress {
            run_id: Some(run_id),
            model: Some(model.clone()),
            evidence_count: scan_items.len(),
            batch_count: Some(batch_count),
            saved_leads: saved_total,
            ..scan_progress(
                "fallback",
                "The local model did not return usable leads; building an evidence packet instead.",
            )
        });
        saved_total = save_fallback_leads(&conn, run_id, &scan_items);
    }

    updated_run.run_status = if saved_total > 0 || model_returned_usable_json {
        "completed".to_string()
    } else {
        "failed".to_string()
    };
    if let Err(e) = db::update_daily_scan_run(&conn, &updated_run) {
        eprintln!("Failed to update daily scan run status: {}", e);
    }

    if saved_total > 0 || model_returned_usable_json {
        let complete_message = if saved_total > 0 {
            format!("Daily Scan saved {} lead(s).", saved_total)
        } else {
            "Daily Scan completed with no leads found.".to_string()
        };
        progress(DailyScanProgress {
            run_id: Some(run_id),
            model: Some(model.clone()),
            evidence_count: scan_items.len(),
            batch_count: Some(batch_count),
            saved_leads: saved_total,
            ..scan_progress("complete", complete_message)
        });
        Ok(run_id)
    } else {
        Err(format!(
            "Daily Scan could not produce leads. {}",
            batch_errors.join("; ")
        ))
    }
}

pub async fn run_daily_scan_fetching_sources_with_progress<F>(
    db: &DbConn,
    llm_client: &std::sync::Arc<dyn LlmClient>,
    prompt_template: &str,
    city: &str,
    state: &str,
    since_hours: u32,
    mut progress: F,
) -> Result<i32, String>
where
    F: FnMut(DailyScanProgress),
{
    progress(scan_progress(
        "fetching",
        "Checking watched sources for fresh records before analysis.",
    ));
    crate::core::scraper::scrape_all_sources(db)
        .await
        .map_err(|e| e.to_string())?;
    run_daily_scan_with_progress(
        db,
        llm_client,
        prompt_template,
        city,
        state,
        since_hours,
        progress,
    )
    .await
}
