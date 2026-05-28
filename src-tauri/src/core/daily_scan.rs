// core/daily_scan.rs
use crate::core::db::{self, DailyScanLead, DailyScanRun, DbConn};
use crate::core::llm::LlmClient;
use chrono::Utc;
use serde::{Deserialize, Serialize};

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
    let result: ScanResult = serde_json::from_str(json_response).map_err(|e| e.to_string())?;
    let mut saved = 0;
    for item in result.leads {
        let lead = DailyScanLead {
            id: None,
            scan_id: run_id,
            title: item.title,
            summary: item.summary,
            source_id: None, // Assume None, aggregated logic (D5)
            original_url: item.original_url,
        };
        match db::insert_daily_scan_lead(conn, &lead) {
            Ok(_) => saved += 1,
            Err(e) => eprintln!("Failed to insert daily scan lead: {}", e), // P4-009 log insert error
        }
    }
    Ok(saved)
}

pub async fn run_daily_scan<R: tauri::Runtime>(
    db: &DbConn,
    app: &tauri::AppHandle<R>,
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

    let prompt_template = crate::core::prompts::get_prompt(app, "aggregator")?;

    let run = DailyScanRun {
        id: None,
        started_at: Utc::now().to_rfc3339(),
        completed_at: None,
        run_status: "in_progress".to_string(),
    };

    let run_id = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        db::insert_daily_scan_run(&conn, &run).map_err(|e| e.to_string())?
    };

    let evidence_items = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        db::list_evidence_since(&conn, since_hours).unwrap_or_default()
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

    let model = {
        let conn = db.lock().map_err(|_| "Failed to lock db")?;
        let mut stmt = conn
            .prepare("SELECT value FROM settings WHERE key = 'model.selected'")
            .map_err(|e| e.to_string())?;
        let val: Result<String, _> = stmt.query_row([], |row| row.get(0));
        match val {
            Ok(v) => v,
            Err(_) => "phi3:mini".to_string(), // Fallback if not set
        }
    };

    use tauri::Manager;
    let llm_client = app.state::<std::sync::Arc<dyn LlmClient>>();
    let llm_res = llm_client
        .call(&model, &final_prompt, &prompt_template)
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
            if let Err(e) = parse_and_save_scan_response(&conn, run_id, &json_response) {
                eprintln!("Failed to parse and save scan response: {}", e);
                updated_run.run_status = "failed".to_string();
            } else {
                updated_run.run_status = "completed".to_string();
            }
            if let Err(e) = db::update_daily_scan_run(&conn, &updated_run) {
                eprintln!("Failed to update daily scan run status: {}", e);
            }
            Ok(run_id)
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
