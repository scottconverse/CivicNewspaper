// core/daily_scan.rs
use crate::core::db::{self, DailyScanLead, DailyScanRun, DbConn, EvidenceItem};
use crate::core::llm::LlmClient;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Distinct error signal returned by [`run_daily_scan`] when there is zero
/// evidence in the requested window, so the scan is reported as a real `Err`
/// rather than an empty-but-successful run (QA-M2). The `NO_EVIDENCE:` prefix is
/// a typed marker the frontend's `toUserMessage` (src/ipc.ts) recognizes: it
/// strips the prefix and surfaces the plain "run Scrape & Detect first" guidance
/// instead of a raw "Something went wrong: NO_EVIDENCE: …" leak.
pub const NO_EVIDENCE_SIGNAL: &str =
    "NO_EVIDENCE: There is no evidence in the selected window. Run Scrape & Detect first.";

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResultItem {
    pub title: String,
    pub summary: String,
    pub original_url: String,
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
        };
        match db::insert_daily_scan_lead(conn, &lead) {
            Ok(_) => saved += 1,
            Err(e) => eprintln!("Failed to insert daily scan lead: {}", e), // P4-009 log insert error
        }
    }
    Ok(saved)
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

fn scan_progress(
    stage: &str,
    message: impl Into<String>,
) -> DailyScanProgress {
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

fn build_batch_prompt(city: &str, state: &str, batch_index: usize, batch: &[EvidenceItem]) -> String {
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
         Schema: {{\"leads\":[{{\"title\":\"short civic lead title\",\"summary\":\"1-2 evidence-grounded sentences\",\"original_url\":\"source URL from evidence or empty string\"}}]}}\n\
         Include at most 3 leads. Use an empty leads array if nothing deserves an editor's look.",
        city = city,
        state = state,
        batch_number = batch_index + 1,
        context = context
    )
}

fn repair_prompt(raw: &str) -> String {
    format!(
        "Repair the following model output into ONLY valid JSON matching this schema: \
         {{\"leads\":[{{\"title\":\"...\",\"summary\":\"...\",\"original_url\":\"...\"}}]}}. \
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
        };
        if db::insert_daily_scan_lead(conn, &lead).is_ok() {
            saved += 1;
        }
    }
    saved
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
    let batches: Vec<&[EvidenceItem]> = scan_items.chunks(4).collect();
    let batch_count = batches.len();

    progress(DailyScanProgress {
        run_id: Some(run_id),
        model: Some(model.clone()),
        evidence_count: scan_items.len(),
        batch_count: Some(batch_count),
        ..scan_progress(
            "generating",
            format!("Starting local scan with {} across {} batch(es).", model, batch_count),
        )
    });

    let mut saved_total = 0usize;
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
