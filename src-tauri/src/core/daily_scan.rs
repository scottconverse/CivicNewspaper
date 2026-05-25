// core/daily_scan.rs
use crate::core::db::{DbConn, DailyScanRun, DailyScanLead, insert_daily_scan_run, insert_daily_scan_lead};
use crate::core::prompts::load_prompt;
use crate::core::llm::call_local_ollama;
use chrono::{Utc, Duration};

pub fn parse_daily_scan_leads(raw: &str) -> Vec<DailyScanLead> {
    let mut leads = Vec::new();
    let mut current_lead: Option<DailyScanLead> = None;
    
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        if line.starts_with("Headline:") || line.starts_with("**Headline:**") {
            if let Some(lead) = current_lead.take() {
                leads.push(lead);
            }
            let val = line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default();
            current_lead = Some(DailyScanLead {
                id: None,
                run_id: 0,
                rank: leads.len() as i32 + 1,
                tier: "news_reporting".to_string(),
                headline: val,
                details: String::new(),
                source: None,
                url: None,
                confidence: None,
                action: None,
                beat: None,
            });
        } else if let Some(ref mut lead) = current_lead {
            if line.starts_with("Tier:") || line.starts_with("**Tier:**") {
                lead.tier = line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default();
            } else if line.starts_with("Source:") || line.starts_with("**Source:**") {
                lead.source = Some(line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default());
            } else if line.starts_with("URL:") || line.starts_with("**URL:**") {
                lead.url = Some(line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default());
            } else if line.starts_with("Confidence:") || line.starts_with("**Confidence:**") {
                lead.confidence = Some(line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default());
            } else if line.starts_with("Action:") || line.starts_with("**Action:**") {
                lead.action = Some(line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default());
            } else if line.starts_with("Beat:") || line.starts_with("**Beat:**") {
                lead.beat = Some(line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default());
            } else if line.starts_with("Details:") || line.starts_with("**Details:**") {
                lead.details = line.split_once(':').map(|s| s.1.trim().replace("**", "")).unwrap_or_default();
            }
        }
    }
    
    if let Some(lead) = current_lead {
        leads.push(lead);
    }
    
    leads
}

pub async fn run_daily_scan_logic(
    db: &DbConn,
    app: Option<&tauri::AppHandle>,
    city: String,
    state: String,
    since_hours: u32,
) -> Result<(DailyScanRun, Vec<DailyScanLead>), String> {
    if since_hours == 0 || since_hours > 168 {
        return Err("since_hours must be between 1 and 168".to_string());
    }

    let prompt_template = load_prompt(app, "aggregator/01-daily-scan")?;
    let prompt = prompt_template
        .replace("[YOUR_CITY]", &city)
        .replace("[YOUR_STATE]", &state)
        .replace("[YOUR_CITY_AGENDA_PORTAL_URL]", "http://local.gov");

    let items = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        let cutoff = (Utc::now() - Duration::hours(since_hours as i64)).to_rfc3339();
        let mut stmt = conn.prepare("SELECT id, source_id, url, fetched_at, excerpt, content_hash, entities FROM evidence_items WHERE fetched_at >= ?1 LIMIT 100")
            .map_err(|e| e.to_string())?;
        
        let iter = stmt.query_map([cutoff], |row| {
            Ok(crate::core::db::EvidenceItem {
                id: Some(row.get(0)?),
                source_id: row.get(1)?,
                url: row.get(2)?,
                fetched_at: row.get(3)?,
                excerpt: row.get(4)?,
                content_hash: row.get(5)?,
                entities: row.get(6)?,
            })
        }).map_err(|e| e.to_string())?;
        
        let mut items = Vec::new();
        for i in iter {
            if let Ok(item) = i { items.push(item); }
        }
        items
    };

    let mut context = String::new();
    for item in items {
        let excerpt = if item.excerpt.len() > 300 {
            &item.excerpt[..300]
        } else {
            &item.excerpt
        };
        context.push_str(&format!("Evidence: {}\n\n", excerpt));
        if context.len() > 32000 { break; }
    }

    let model = "gemma2:9b";
    let raw_response = call_local_ollama(model, &context, &prompt).await.map_err(|e| e.to_string())?;

    let mut leads = parse_daily_scan_leads(&raw_response);

    let run = DailyScanRun {
        id: None,
        run_date: Utc::now().to_rfc3339(),
        city,
        state,
        model_used: model.to_string(),
        prompt_id: "aggregator/01-daily-scan".to_string(),
        raw_response,
        created_at: Utc::now().to_rfc3339(),
    };

    let (run_out, leads_out) = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        let run_id = insert_daily_scan_run(&conn, &run).map_err(|e| e.to_string())?;
        let mut out_run = run.clone();
        out_run.id = Some(run_id);
        
        let mut out_leads = Vec::new();
        for mut lead in leads {
            lead.run_id = run_id;
            let lead_id = insert_daily_scan_lead(&conn, &lead).map_err(|e| e.to_string())?;
            lead.id = Some(lead_id);
            out_leads.push(lead);
        }
        (out_run, out_leads)
    };

    Ok((run_out, leads_out))
}
