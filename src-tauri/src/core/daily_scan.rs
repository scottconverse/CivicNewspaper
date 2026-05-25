// core/daily_scan.rs
use crate::core::db::{self, DbConn, DailyScanRun, DailyScanLead};
use crate::core::llm;
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

pub fn parse_and_save_scan_response(conn: &rusqlite::Connection, run_id: i32, json_response: &str) -> Result<usize, String> {
    let result: ScanResult = serde_json::from_str(json_response).map_err(|e| e.to_string())?;
    let mut saved = 0;
    for item in result.leads {
        let lead = DailyScanLead {
            id: None,
            scan_id: run_id,
            title: item.title,
            summary: item.summary,
            source_id: 1, // Assume 1 for simplicity in parsing unless extraction is sophisticated
            original_url: item.original_url,
        };
        if db::insert_daily_scan_lead(conn, &lead).is_ok() {
            saved += 1;
        }
    }
    Ok(saved)
}

pub async fn run_daily_scan(
    db: &DbConn,
    app: &tauri::AppHandle,
    city: &str,
    state: &str,
    since_hours: u32,
) -> Result<i32, String> {
    // Validate bounds
    if since_hours == 0 || since_hours > 168 {
        return Err("since_hours must be between 1 and 168".to_string());
    }
    
    let re = regex::Regex::new(r"^[A-Za-z][A-Za-z .'-]{0,63}$").map_err(|e| e.to_string())?;
    if !re.is_match(city) || !re.is_match(state) {
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
        db::list_all_evidence(&conn).unwrap_or_default()
    };
    
    let mut context = String::new();
    for item in evidence_items.iter().take(20) {
        context.push_str(&format!("Source ID: {}\nExcerpt: {}\n\n", item.source_id, item.excerpt));
    }
    
    let final_prompt = format!("City: {}, State: {}\n\nEvidence Context:\n{}", city, state, context);

    let llm_res = llm::call_local_ollama("gemma2:9b", &final_prompt, &prompt_template).await;
    
    let mut conn = db.lock().map_err(|_| "Failed to lock db")?;
    let mut updated_run = run.clone();
    updated_run.id = Some(run_id);
    updated_run.completed_at = Some(Utc::now().to_rfc3339());
    
    match llm_res {
        Ok(json_response) => {
            let _ = parse_and_save_scan_response(&conn, run_id, &json_response);
            updated_run.run_status = "completed".to_string();
            let _ = db::update_daily_scan_run(&conn, &updated_run);
            Ok(run_id)
        },
        Err(e) => {
            updated_run.run_status = "failed".to_string();
            let _ = db::update_daily_scan_run(&conn, &updated_run);
            Err(e.to_string())
        }
    }
}
