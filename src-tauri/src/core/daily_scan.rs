// core/daily_scan.rs
use crate::core::db::{self, DailyScanLead, DailyScanRun, DbConn};
use crate::core::llm::LlmClient;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Distinct error signal returned by [`run_daily_scan`] when there is zero
/// evidence in the requested window. The frontend matches on this to show a
/// "no evidence — run Scrape & Detect first" message rather than treating the
/// scan as an empty-but-successful run (QA-M2).
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

pub fn parse_and_save_scan_response(
    conn: &rusqlite::Connection,
    run_id: i32,
    json_response: &str,
) -> Result<usize, String> {
    // The model output is frequently raw JSON, but small local models sometimes
    // wrap it in a ```json fence or add prose. Strip a leading/trailing code
    // fence before parsing so a well-formed-but-fenced response still succeeds;
    // anything still unparseable propagates as a real error (QA-M2).
    let cleaned = strip_json_fence(json_response);
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

pub async fn run_daily_scan(
    db: &DbConn,
    llm_client: &std::sync::Arc<dyn LlmClient>,
    prompt_template: &str,
    city: &str,
    state: &str,
    since_hours: u32,
) -> Result<i32, String> {
    // Validate bounds
    if since_hours == 0 || since_hours > 168 {
        return Err("since_hours must be between 1 and 168".to_string());
    }

    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let regex = RE.get_or_init(|| regex::Regex::new(r"^[A-Za-z][A-Za-z .'-]{0,63}$").unwrap());
    if !regex.is_match(city) || !regex.is_match(state) {
        return Err("Invalid city or state format".to_string());
    }

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

    let run_id = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        db::insert_daily_scan_run(&conn, &run).map_err(|e| e.to_string())?
    };

    let mut context = String::new();
    for item in evidence_items.iter().take(20) {
        context.push_str(&format!(
            "Source ID: {}\nExcerpt: {}\n\n",
            item.source_id, item.excerpt
        ));
    }

    let final_prompt = format!(
        "City: {}, State: {}\n\nEvidence Context:\n{}",
        city, state, context
    );

    let model = crate::tauri_cmds::get_selected_model_or_fallback(db).await;

    let llm_res = llm_client
        .call(&model, &final_prompt, prompt_template)
        .await;

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

    match llm_res {
        Ok(json_response) => {
            // QA-M2: a model that returns non-JSON (or fenced/garbage output)
            // must NOT be treated as a successful empty scan. Mark the run failed
            // AND propagate the parse failure as a real Err so the UI can
            // distinguish "0 leads found" from "the AI returned an unreadable
            // response." (A valid JSON response with an empty leads array still
            // counts as a successful, completed scan.)
            match parse_and_save_scan_response(&conn, run_id, &json_response) {
                Ok(_) => {
                    updated_run.run_status = "completed".to_string();
                    if let Err(e) = db::update_daily_scan_run(&conn, &updated_run) {
                        eprintln!("Failed to update daily scan run status: {}", e);
                    }
                    Ok(run_id)
                }
                Err(e) => {
                    eprintln!("Failed to parse and save scan response: {}", e);
                    updated_run.run_status = "failed".to_string();
                    if let Err(e2) = db::update_daily_scan_run(&conn, &updated_run) {
                        eprintln!("Failed to update daily scan run status: {}", e2);
                    }
                    Err(format!(
                        "The AI returned an unreadable response (could not parse scan results): {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            updated_run.run_status = "failed".to_string();
            if let Err(e2) = db::update_daily_scan_run(&conn, &updated_run) {
                eprintln!(
                    "Failed to update daily scan run status (error-of-error): {}",
                    e2
                );
            }
            Err(e.to_string())
        }
    }
}
