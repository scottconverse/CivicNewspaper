// core/daily_scan.rs
use crate::core::db::{self, DailyScanLead, DailyScanRun, DbConn, EvidenceItem, Lead};
use crate::core::llm::LlmClient;
use crate::core::{intelligence, source_grounding, verification};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const MIN_READER_FACING_SCAN_LEADS: usize = 5;

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
    #[serde(default)]
    pub eligible_evidence_count: usize,
    #[serde(default)]
    pub truncated_evidence_count: usize,
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
            recurrence_count: None,
            recurrence_note: None,
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
        eligible_evidence_count: 0,
        truncated_evidence_count: 0,
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
         If an excerpt contains a dated Latest News, alert, notice, or events list, split it into separate leads for the specific dated items. Do not summarize the parent page as one generic lead unless the parent page itself changed in a newsworthy way.\n\
         Prefer distinct topics for an issue. Do not create several separate leads that are only different phrasings of the same meeting-process, public-participation, archive, or general-access information.\n\
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
            recurrence_count: None,
            recurrence_note: None,
        };
        if save_daily_scan_lead_for_queue(conn, &lead).unwrap_or(0) > 0 {
            saved += 1;
        }
    }
    saved
}

fn save_quality_rescue_leads(
    conn: &rusqlite::Connection,
    run_id: i32,
    evidence_items: &[EvidenceItem],
) -> usize {
    let existing_reader_facing = count_reader_facing_scan_leads(conn, run_id);
    if existing_reader_facing >= MIN_READER_FACING_SCAN_LEADS {
        return 0;
    }

    let mut saved = 0;
    let mut reader_facing_saved = 0;
    let target_reader_facing = MIN_READER_FACING_SCAN_LEADS.saturating_sub(existing_reader_facing);
    for item in evidence_items {
        if reader_facing_saved >= target_reader_facing {
            break;
        }
        let Some(lead) = rescue_lead_from_evidence(conn, run_id, item) else {
            continue;
        };
        if scan_lead_url_exists(conn, run_id, &lead.original_url) {
            continue;
        }
        let reader_facing = is_reader_facing_scan_lead(&lead);
        if save_daily_scan_lead_for_queue(conn, &lead).unwrap_or(0) > 0 {
            saved += 1;
            if reader_facing {
                reader_facing_saved += 1;
            }
        }
    }
    saved
}

fn count_reader_facing_scan_leads(conn: &rusqlite::Connection, run_id: i32) -> usize {
    conn.query_row(
        "SELECT COUNT(*) FROM daily_scan_leads
         WHERE scan_id = ?1
           AND COALESCE(story_type, '') IN ('story', 'brief')
           AND COALESCE(disposition, '') IN ('ready_to_draft', 'review')",
        [run_id],
        |row| row.get::<_, i64>(0),
    )
    .unwrap_or(0)
    .max(0) as usize
}

fn is_reader_facing_scan_lead(lead: &DailyScanLead) -> bool {
    matches!(lead.story_type.as_deref(), Some("story") | Some("brief"))
        && matches!(
            lead.disposition.as_deref(),
            Some("ready_to_draft") | Some("review")
        )
}

fn scan_lead_url_exists(conn: &rusqlite::Connection, run_id: i32, original_url: &str) -> bool {
    let original_url = original_url.trim();
    if original_url.is_empty() {
        return false;
    }
    conn.query_row(
        "SELECT COUNT(*) FROM daily_scan_leads WHERE scan_id = ?1 AND original_url = ?2",
        rusqlite::params![run_id, original_url],
        |row| row.get::<_, i64>(0),
    )
    .unwrap_or(0)
        > 0
}

fn rescue_lead_from_evidence(
    conn: &rusqlite::Connection,
    run_id: i32,
    item: &EvidenceItem,
) -> Option<DailyScanLead> {
    let excerpt = clean_inline(&item.excerpt);
    if !evidence_looks_actionable(&excerpt) {
        return None;
    }

    let source = db::get_source(conn, item.source_id).ok().flatten();
    let source_name = source
        .as_ref()
        .map(|source| source.name.clone())
        .unwrap_or_else(|| format!("Source #{}", item.source_id));
    let source_type = source.as_ref().map(|source| source.r#type.clone());
    let title = rescue_title(&source_name, &excerpt);
    let summary = truncate_chars(&excerpt, 280);
    let story_type = if evidence_has_strong_current_action(&excerpt) {
        "brief"
    } else {
        "watch"
    };
    let immediacy = if evidence_has_calendar_signal(&excerpt) {
        4
    } else {
        3
    };
    let impact = if evidence_has_public_impact_signal(&excerpt) {
        4
    } else {
        3
    };
    let novelty = if evidence_has_strong_current_action(&excerpt) {
        4
    } else {
        3
    };
    let what_changed = format!(
        "Recent source evidence contains a dated or actionable civic item: {}",
        truncate_chars(&first_sentence(&excerpt), 180).trim_end_matches('.')
    );
    let disposition = classify_disposition(
        Some(story_type),
        Some(&what_changed),
        Some(immediacy),
        Some(impact),
        Some(novelty),
    );

    Some(DailyScanLead {
        id: None,
        scan_id: run_id,
        title,
        summary,
        source_id: Some(item.source_id),
        original_url: item.url.clone().unwrap_or_default(),
        why_flagged: Some("The scan found actionable source evidence after the local model produced too few draftable leads. This is an editor lead, not an automatic publication decision.".to_string()),
        source_name: Some(source_name),
        source_type,
        priority: Some(if story_type == "brief" { "medium" } else { "low" }.to_string()),
        suggested_next_step: Some("Open the source, confirm the date or action, and look for a second public source before approving reader-facing copy.".to_string()),
        story_type: Some(story_type.to_string()),
        what_changed: Some(what_changed),
        immediacy: Some(immediacy),
        impact: Some(impact),
        conflict: if evidence_has_conflict_signal(&excerpt) { Some(3) } else { Some(1) },
        novelty: Some(novelty),
        publishability_note: Some("Confirm the source date, names, decision point, and at least one corroborating source if available.".to_string()),
        disposition: Some(disposition),
        recurrence_count: None,
        recurrence_note: None,
    })
}

fn clean_inline(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

fn truncate_chars(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    format!("{}...", value.chars().take(max_chars).collect::<String>())
}

fn first_sentence(value: &str) -> String {
    value
        .split_terminator(['.', '!', '?'])
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(value)
        .to_string()
}

fn rescue_title(source_name: &str, excerpt: &str) -> String {
    let sentence = first_sentence(excerpt);
    let sentence = sentence
        .strip_prefix("Headline:")
        .unwrap_or(&sentence)
        .trim();
    format!("{}: {}", source_name, truncate_chars(sentence, 86))
}

fn evidence_looks_actionable(excerpt: &str) -> bool {
    let text = excerpt.to_lowercase();
    if looks_static_or_evergreen(&text) && !evidence_has_strong_current_action(excerpt) {
        return false;
    }
    evidence_has_calendar_signal(excerpt)
        || evidence_has_strong_current_action(excerpt)
        || evidence_has_public_impact_signal(excerpt)
}

fn evidence_has_calendar_signal(excerpt: &str) -> bool {
    let text = excerpt.to_lowercase();
    const MONTHS: &[&str] = &[
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
    ];
    MONTHS.iter().any(|month| text.contains(month))
        || text.contains("tonight")
        || text.contains("tomorrow")
        || text.contains("deadline")
        || text.contains("public hearing")
        || text.contains("agenda")
        || text.contains("meeting")
}

fn evidence_has_strong_current_action(excerpt: &str) -> bool {
    let text = excerpt.to_lowercase();
    [
        "approved",
        "adopted",
        "filed",
        "opened",
        "closed",
        "canceled",
        "cancelled",
        "rescheduled",
        "deadline",
        "vote",
        "hearing",
        "outage",
        "contract",
        "lawsuit",
        "permit",
        "budget",
        "grant",
        "bid",
        "application",
        "applications",
        "public comment",
        "notice",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn evidence_has_public_impact_signal(excerpt: &str) -> bool {
    let text = excerpt.to_lowercase();
    [
        "residents",
        "businesses",
        "tax",
        "fee",
        "road",
        "housing",
        "water",
        "police",
        "fire",
        "school",
        "library",
        "construction",
        "downtown",
        "public",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn evidence_has_conflict_signal(excerpt: &str) -> bool {
    let text = excerpt.to_lowercase();
    [
        "lawsuit",
        "appeal",
        "opposition",
        "controvers",
        "complaint",
        "denied",
        "violation",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn looks_static_or_evergreen(text: &str) -> bool {
    [
        "about us",
        "contact us",
        "archive",
        "video archive",
        "how to",
        "available online",
        "regularly held",
        "general information",
        "service page",
        "department page",
    ]
    .iter()
    .any(|needle| text.contains(needle))
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
            recurrence_count: None,
            recurrence_note: None,
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
    let mut lead = lead_with_beat_memory(lead, recurring_memory.as_ref());
    let evidence_ids = evidence_ids_for_scan_lead(conn, &lead)?;
    if evidence_ids.is_empty() && scan_lead_requires_linked_evidence(&lead) {
        lead = downgrade_scan_lead_without_evidence(lead);
    }
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
        recurrence_count: lead.recurrence_count,
        recurrence_note: lead.recurrence_note.clone(),
        created_at: String::new(),
    };
    db::insert_lead(conn, &story_lead, &evidence_ids)?;
    Ok(scan_lead_id)
}

fn scan_lead_requires_linked_evidence(lead: &DailyScanLead) -> bool {
    matches!(lead.story_type.as_deref(), Some("story") | Some("brief"))
        && matches!(
            lead.disposition.as_deref(),
            Some("ready_to_draft") | Some("review")
        )
}

fn scan_lead_grounding_text(lead: &DailyScanLead) -> String {
    [lead.title.as_str(), lead.summary.as_str()].join(" ")
}

fn downgrade_scan_lead_without_evidence(mut lead: DailyScanLead) -> DailyScanLead {
    lead.disposition = Some("needs_verification".to_string());
    let evidence_note = "No source documents could be linked to this model-suggested lead. Attach or verify public source material before drafting reader-facing copy.";
    lead.publishability_note = Some(match lead.publishability_note {
        Some(note) if !note.trim().is_empty() => {
            format!("{}. {}", note.trim_end_matches('.'), evidence_note)
        }
        _ => evidence_note.to_string(),
    });
    lead.suggested_next_step = Some(match lead.suggested_next_step {
        Some(step) if !step.trim().is_empty() => format!(
            "{}. Then attach or cite the source document before drafting.",
            step.trim_end_matches('.')
        ),
        _ => "Attach or cite the public source document before drafting.".to_string(),
    });
    lead
}

#[derive(Debug)]
struct BeatMemory {
    representative_title: String,
    first_seen_at: String,
    last_seen_at: String,
    seen_count: i32,
    topic_key: String,
}

fn lead_with_beat_memory(lead: &DailyScanLead, memory: Option<&BeatMemory>) -> DailyScanLead {
    let Some(memory) = memory else {
        return lead.clone();
    };

    let mut lead = lead.clone();
    let seen_note = format!(
        "Beat memory: structured match '{}' for topic '{}' was first seen {}, last seen {}, and has appeared {} previous time(s). Treat this as recurring/background unless the source shows a new vote, deadline, dollar amount, filing, outage, meeting item, or public impact.",
        memory.topic_key, memory.representative_title, memory.first_seen_at, memory.last_seen_at, memory.seen_count
    );
    let display_note = format!(
        "Similar topic '{}' was first seen {}, last seen {}, and has appeared {} previous time(s). Match: {}.",
        memory.representative_title, memory.first_seen_at, memory.last_seen_at, memory.seen_count, memory.topic_key
    );
    lead.recurrence_count = Some(memory.seen_count);
    lead.recurrence_note = Some(display_note);
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
    if !has_new_reportable_fact(&lead) || looks_like_background_or_unchanged(&lead) {
        lead.priority = Some("low".to_string());
        lead.story_type = Some("background".to_string());
        lead.disposition = Some("background".to_string());
        lead.what_changed = Some(
            "Beat memory found this topic before, and this scan did not identify a concrete new reportable fact."
                .to_string(),
        );
    }
    lead
}

fn has_new_reportable_fact(lead: &DailyScanLead) -> bool {
    let what_changed = lead
        .what_changed
        .as_deref()
        .unwrap_or_default()
        .to_lowercase();
    if what_changed.trim().is_empty()
        || what_changed.contains("no current change")
        || what_changed.contains("not found")
        || what_changed.contains("unchanged")
        || what_changed.contains("background")
    {
        return false;
    }

    if lead.novelty.unwrap_or(0) >= 4 && lead.immediacy.unwrap_or(0) >= 3 {
        return true;
    }

    [
        "approved",
        "adopted",
        "filed",
        "opened",
        "closed",
        "deadline",
        "vote",
        "hearing",
        "outage",
        "contract",
        "lawsuit",
        "permit",
        "budget",
        "grant",
        "meeting item",
        "public impact",
    ]
    .iter()
    .any(|needle| what_changed.contains(needle))
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
        "SELECT topic_key, representative_title, source_url, first_seen_at, last_seen_at, seen_count, last_summary
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
            row.get::<_, String>(4)?,
            row.get::<_, i32>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    for row in rows {
        let (
            topic_key,
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
                topic_key,
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
    let signature = topic_signature(
        &lead.title,
        &lead.summary,
        lead.original_url.as_str(),
        lead.source_name.as_deref(),
    );
    if signature.key.is_empty() {
        String::new()
    } else {
        format!("topic:{}", signature.key)
    }
}

#[derive(Debug)]
struct TopicSignature {
    key: String,
    entity: Option<String>,
    action: Option<String>,
    terms: BTreeSet<String>,
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

fn topic_tokens(text: &str) -> BTreeSet<String> {
    const STOPWORDS: &[&str] = &[
        "a",
        "an",
        "and",
        "are",
        "as",
        "at",
        "be",
        "by",
        "can",
        "for",
        "from",
        "in",
        "into",
        "is",
        "it",
        "its",
        "of",
        "on",
        "or",
        "that",
        "the",
        "their",
        "this",
        "to",
        "with",
        "city",
        "civic",
        "public",
        "local",
        "residents",
        "resident",
        "longmont",
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

fn topic_signature(
    title: &str,
    summary: &str,
    original_url: &str,
    source_name: Option<&str>,
) -> TopicSignature {
    let topic_text = format!(
        "{} {} {} {}",
        source_name.unwrap_or_default(),
        title,
        summary,
        original_url
    );
    let text_without_url = format!("{} {} {}", source_name.unwrap_or_default(), title, summary);
    let combined = topic_text;
    let lower = combined.to_lowercase();
    let entity = detect_topic_entity(&lower);
    let action = detect_topic_action(&lower);
    let terms = topic_tokens(&text_without_url);
    let mut key_terms = topic_key_terms(&format!(
        "{} {} {}",
        source_name.unwrap_or_default(),
        title,
        summary
    ));
    if key_terms.is_empty() {
        key_terms = terms.iter().take(8).cloned().collect();
    }
    let term_part = key_terms.into_iter().take(8).collect::<Vec<_>>().join("-");
    let mut key_parts = Vec::new();
    if let Some(entity) = entity.as_deref() {
        key_parts.push(entity.to_string());
    }
    if let Some(action) = action.as_deref() {
        key_parts.push(action.to_string());
    }
    if !term_part.is_empty() && topic_key_needs_term_discriminator(action.as_deref()) {
        key_parts.push(term_part);
    }
    TopicSignature {
        key: key_parts.join(":"),
        entity,
        action,
        terms,
    }
}

fn topic_key_needs_term_discriminator(action: Option<&str>) -> bool {
    !matches!(action, Some("archive-access"))
}

fn topic_key_terms(text: &str) -> Vec<String> {
    const KEY_STOPWORDS: &[&str] = &[
        "action", "actions", "approved", "adopted", "award", "bid", "brief", "city", "contract",
        "council", "decision", "longmont", "meeting", "public", "regular", "source", "vote",
        "votes",
    ];
    let mut seen = BTreeSet::new();
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
        .map(|token| {
            token
                .trim_end_matches('s')
                .trim_end_matches("ing")
                .to_string()
        })
        .filter(|token| token.len() > 2)
        .filter(|token| !KEY_STOPWORDS.contains(&token.as_str()))
        .filter(|token| seen.insert(token.clone()))
        .collect()
}

fn detect_topic_entity(text: &str) -> Option<String> {
    let candidates = [
        (
            "city-council",
            &["city council", "council meeting", "council agenda"][..],
        ),
        ("library", &["library", "public library"][..]),
        (
            "building-services",
            &[
                "building services",
                "permitting portal",
                "permit portal",
                "building permit",
            ][..],
        ),
        (
            "planning-zoning",
            &["planning", "zoning", "development review"][..],
        ),
        (
            "parks-recreation",
            &["parks", "recreation", "trail", "open space"][..],
        ),
        ("youth-center", &["youth center", "youth services"][..]),
        (
            "public-safety",
            &["police", "fire rescue", "public safety"][..],
        ),
        ("finance-budget", &["budget", "tax", "fee", "bond"][..]),
        (
            "utilities",
            &["water", "electric", "utility", "utilities", "stormwater"][..],
        ),
        (
            "transportation",
            &["road", "street", "traffic", "transit", "transportation"][..],
        ),
        ("housing", &["housing", "apartment", "affordable"][..]),
        (
            "community-event",
            &["event", "workshop", "class", "club"][..],
        ),
    ];
    candidates.iter().find_map(|(entity, needles)| {
        needles
            .iter()
            .any(|needle| text.contains(needle))
            .then(|| (*entity).to_string())
    })
}

fn detect_topic_action(text: &str) -> Option<String> {
    let candidates = [
        (
            "service-outage",
            &["outage", "down", "technical issues", "unavailable"][..],
        ),
        (
            "schedule-change",
            &[
                "canceled",
                "cancelled",
                "rescheduled",
                "postponed",
                "closed",
            ][..],
        ),
        (
            "decision-contract",
            &["approved", "adopted", "contract", "bid", "award"][..],
        ),
        (
            "archive-access",
            &["archive", "video archive", "livestream", "live stream"][..],
        ),
        (
            "public-hearing",
            &["public hearing", "public comment", "comment period"][..],
        ),
        (
            "deadline-notice",
            &["deadline", "notice", "application", "applications due"][..],
        ),
        (
            "meeting-agenda",
            &["agenda", "meeting item", "meeting", "minutes"][..],
        ),
        (
            "public-event",
            &[
                "event",
                "workshop",
                "class",
                "club",
                "chess",
                "conversation",
            ][..],
        ),
        ("finance", &["budget", "grant", "tax", "fee", "funding"][..]),
    ];
    candidates.iter().find_map(|(action, needles)| {
        needles
            .iter()
            .any(|needle| text.contains(needle))
            .then(|| (*action).to_string())
    })
}

fn signatures_compatible(existing: &TopicSignature, lead: &TopicSignature) -> bool {
    if !existing.key.is_empty() && existing.key == lead.key {
        return true;
    }

    if existing.entity.is_some() && lead.entity.is_some() && existing.entity != lead.entity {
        return false;
    }
    if existing.action.is_some() && lead.action.is_some() && existing.action != lead.action {
        return false;
    }

    let common = existing.terms.intersection(&lead.terms).count();
    let smaller = existing.terms.len().min(lead.terms.len());
    let overlap = smaller > 0 && common >= 4 && (common as f32 / smaller as f32) >= 0.62;
    if existing.entity.is_some()
        && existing.entity == lead.entity
        && existing.action.is_some()
        && existing.action == lead.action
        && topic_key_needs_term_discriminator(existing.action.as_deref())
    {
        return overlap && common >= 5;
    }

    match (
        existing.entity.as_ref(),
        lead.entity.as_ref(),
        existing.action.as_ref(),
        lead.action.as_ref(),
    ) {
        (Some(_), Some(_), Some(_), Some(_)) => overlap || common >= 3,
        (Some(_), Some(_), _, _) | (_, _, Some(_), Some(_)) => overlap,
        _ => overlap && common >= 5,
    }
}

fn scan_leads_are_similar(
    existing_title: &str,
    existing_summary: &str,
    existing_url: &str,
    lead: &DailyScanLead,
) -> bool {
    let existing_url = existing_url.trim();
    let lead_url = lead.original_url.trim();
    let existing_signature = topic_signature(existing_title, existing_summary, existing_url, None);
    let lead_signature = topic_signature(
        &lead.title,
        &lead.summary,
        &lead.original_url,
        lead.source_name.as_deref(),
    );
    if !existing_url.is_empty()
        && existing_url.eq_ignore_ascii_case(lead_url)
        && existing_signature.action.is_some()
        && existing_signature.action == lead_signature.action
        && !matches!(
            existing_signature.action.as_deref(),
            Some("public-event")
                | Some("meeting-agenda")
                | Some("public-hearing")
                | Some("deadline-notice")
                | Some("decision-contract")
                | Some("schedule-change")
                | Some("service-outage")
                | Some("finance")
        )
    {
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
    if !existing_url.is_empty()
        && existing_url.eq_ignore_ascii_case(lead_url)
        && signatures_compatible(&existing_signature, &lead_signature)
    {
        return true;
    }
    if !signatures_compatible(&existing_signature, &lead_signature) {
        return false;
    }
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
    let mut candidates = Vec::new();
    if let Some(source_id) = lead.source_id {
        let mut stmt = conn.prepare(
            "SELECT id, excerpt FROM evidence_items WHERE source_id = ?1 ORDER BY fetched_at DESC LIMIT 8",
        )?;
        let rows = stmt.query_map([source_id], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            candidates.push(row?);
        }
    }
    if candidates.is_empty() && !lead.original_url.trim().is_empty() {
        let mut stmt = conn.prepare(
            "SELECT id, excerpt FROM evidence_items WHERE url = ?1 ORDER BY fetched_at DESC LIMIT 8",
        )?;
        let rows = stmt.query_map([lead.original_url.trim()], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            candidates.push(row?);
        }
    }
    let topic = scan_lead_grounding_text(lead);
    let mut ids: Vec<i32> = candidates
        .into_iter()
        .filter(|(_, excerpt)| source_grounding::evidence_matches_topic(&topic, excerpt))
        .map(|(id, _)| id)
        .take(3)
        .collect();
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
        eligible_evidence_count: evidence_items.len(),
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
    let eligible_evidence_count = evidence_items.len();
    let scan_items: Vec<EvidenceItem> = evidence_items.into_iter().take(20).collect();
    let truncated_evidence_count = eligible_evidence_count.saturating_sub(scan_items.len());
    let deterministic_saved = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        progress(DailyScanProgress {
            run_id: Some(run_id),
            evidence_count: scan_items.len(),
            eligible_evidence_count,
            truncated_evidence_count,
            ..scan_progress(
                "deterministic",
                if truncated_evidence_count > 0 {
                    format!(
                        "Reviewing the newest {} of {} eligible evidence item(s); {} older item(s) were left for a later scan.",
                        scan_items.len(),
                        eligible_evidence_count,
                        truncated_evidence_count
                    )
                } else {
                    "Extracting entities, detecting changes, and building verification tasks."
                        .to_string()
                },
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
        eligible_evidence_count,
        truncated_evidence_count,
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
            eligible_evidence_count,
            truncated_evidence_count,
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
                    eligible_evidence_count,
                    truncated_evidence_count,
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
                    eligible_evidence_count,
                    truncated_evidence_count,
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

    let model_returned_usable_json = parsed_batches > 0;
    let reader_facing_total = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        count_reader_facing_scan_leads(&conn, run_id)
    };
    if model_returned_usable_json && reader_facing_total < MIN_READER_FACING_SCAN_LEADS {
        let conn = match db.lock() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to lock db for quality rescue pass: {}", e);
                return Err("Failed to lock db".to_string());
            }
        };
        progress(DailyScanProgress {
            run_id: Some(run_id),
            model: Some(model.clone()),
            evidence_count: scan_items.len(),
            eligible_evidence_count,
            truncated_evidence_count,
            batch_count: Some(batch_count),
            saved_leads: saved_total,
            ..scan_progress(
                "quality_rescue",
                format!(
                    "Saved {} reader-facing candidate(s); checking source evidence for additional dated or actionable story/brief leads.",
                    reader_facing_total
                ),
            )
        });
        let rescue_saved = save_quality_rescue_leads(&conn, run_id, &scan_items);
        saved_total += rescue_saved;
        if rescue_saved > 0 {
            progress(DailyScanProgress {
                run_id: Some(run_id),
                model: Some(model.clone()),
                evidence_count: scan_items.len(),
                eligible_evidence_count,
                truncated_evidence_count,
                batch_count: Some(batch_count),
                saved_leads: saved_total,
                ..scan_progress(
                    "quality_rescue",
                    format!("Added {} source-backed editor lead(s).", rescue_saved),
                )
            });
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

    if saved_total == 0 && !model_returned_usable_json {
        progress(DailyScanProgress {
            run_id: Some(run_id),
            model: Some(model.clone()),
            evidence_count: scan_items.len(),
            eligible_evidence_count,
            truncated_evidence_count,
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
            eligible_evidence_count,
            truncated_evidence_count,
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

#[cfg(test)]
mod prompt_tests {
    use super::*;

    #[test]
    fn batch_prompt_keeps_latest_news_items_distinct() {
        let evidence = vec![EvidenceItem {
            id: Some(1),
            source_id: 2,
            url: Some("https://longmontcolorado.gov/public-information/".to_string()),
            excerpt: "Latest News: June 28 road closure. June 30 human services funding deadline."
                .to_string(),
            content_hash: "latest-news-list".to_string(),
            entities: "[]".to_string(),
            fetched_at: "2026-06-30T00:00:00Z".to_string(),
        }];

        let prompt = build_batch_prompt("Longmont", "CO", 0, &evidence);

        assert!(prompt.contains("split it into separate leads"));
        assert!(prompt.contains("Do not summarize the parent page as one generic lead"));
        assert!(prompt.contains("Do not create several separate leads"));
    }
}
