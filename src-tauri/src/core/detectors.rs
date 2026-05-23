// core/detectors.rs
use rusqlite::Connection;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDateTime};
use super::db::{insert_lead, get_evidence_by_lead, Lead, EvidenceItem, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub money_threshold: Option<f64>,
    pub watchlist: Option<Vec<String>>,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        ProfileConfig {
            money_threshold: Some(250000.0),
            watchlist: Some(Vec::new()),
        }
    }
}

pub fn parse_profile_config(config_json: &str) -> ProfileConfig {
    serde_json::from_str(config_json).unwrap_or_default()
}

pub fn run_detectors(
    conn: &Connection,
    new_evidence_ids: &[i32],
    profile_json: &str,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let config = parse_profile_config(profile_json);
    let money_threshold = config.money_threshold.unwrap_or(250000.0);
    let watchlist = config.watchlist.unwrap_or_default();
    
    let mut new_lead_ids = Vec::new();
    
    // Fetch all the source details for mapping
    let sources = super::db::list_sources(conn)?;
    
    // Pre-compile Regexes
    let re_money = Regex::new(r"\$([0-9,]+)(?:\.[0-9]+)?").unwrap();
    let re_vote = Regex::new(r"(?i)\b(unanimously|voted|approved|resolved|passed|carried|denied|motion|adopted|rejected)\b").unwrap();
    let re_personnel = Regex::new(r"(?i)\b(appoint|resign|retire|terminate|hire|employ|vacancy|successor|resignation|appointment|fired|promoted)\b").unwrap();
    let re_meeting = Regex::new(r"(?i)\b(public hearing|special meeting|session will be held|meeting scheduled|council chamber|town hall|public meeting)\b").unwrap();
    let re_deadline = Regex::new(r"(?i)\b(deadline|submit by|due date|public comment period|rfp|bid due|applications close)\b").unwrap();
    
    // High-risk legal keywords raising warning levels
    let re_high_risk = Regex::new(r"(?i)\b(arrested|charged|indicted|felony|misdemeanor|prosecute|lawsuit|alleged|defendant|plaintiff|litigation)\b").unwrap();

    // 1. Check "Source went quiet" detector (Source-level)
    for source in &sources {
        if let (Some(last_success), Some(last_scrape)) = (&source.last_success_at, &source.last_scraped) {
            if let (Ok(success_dt), Ok(scrape_dt)) = (
                DateTime::parse_from_rfc3339(last_success),
                DateTime::parse_from_rfc3339(last_scrape)
            ) {
                let duration = scrape_dt.signed_duration_since(success_dt);
                if duration.num_days() >= 7 {
                    let why = format!("The source '{}' has not successfully fetched in {} days (Last success: {}). This may indicate a posting delay or structural feed change.", source.name, duration.num_days(), last_success);
                    let lead = Lead {
                        id: None,
                        detector_name: "Source Went Quiet".to_string(),
                        why,
                        confidence: "high".to_string(),
                        risk_level: "low".to_string(),
                        confirmation_checklist: serde_json::to_string(&vec![
                            "Verify if the agency's website URL has changed",
                            "Check if public postings have been paused for holidays/recess",
                            "Ping the web administrator if the feed remains offline"
                        ])?,
                        created_at: Utc::now().to_rfc3339(),
                    };
                    
                    // Check if lead already exists to avoid duplication
                    let exists = lead_exists(conn, "Source Went Quiet", &lead.why)?;
                    if !exists {
                        let lid = insert_lead(conn, &lead, &[])?;
                        new_lead_ids.push(lid);
                    }
                }
            }
        }
    }

    // Process each new evidence item
    for &evidence_id in new_evidence_ids {
        // Fetch evidence text
        let mut stmt = conn.prepare("SELECT excerpt, source_id, url FROM evidence_items WHERE id = ?1")?;
        let mut rows = stmt.query([evidence_id])?;
        if let Some(row) = rows.next()? {
            let excerpt: String = row.get(0)?;
            let source_id: i32 = row.get(1)?;
            let evidence_url: Option<String> = row.get(2)?;
            
            let source = sources.iter().find(|s| s.id == Some(source_id));
            let source_name = source.map(|s| s.name.as_str()).unwrap_or("Unknown Source");
            let source_type = source.map(|s| s.r#type.as_str()).unwrap_or("primary_record");

            // Evaluate risk level of this specific evidence text
            let is_high_risk = re_high_risk.is_match(&excerpt);
            let risk_level = if is_high_risk { "high" } else { "low" };

            // 2. New Primary Record Detector
            if source_type == "primary_record" || source_type == "official_comm" {
                let why = format!("A new official primary document from '{}' was fetched today ({}).", source_name, evidence_url.clone().unwrap_or_default());
                let checklist = vec![
                    "Read the source document thoroughly for hidden agendas",
                    "Verify matching records from previous weeks",
                    "Cross-reference this record with local news announcements"
                ];
                let lead = Lead {
                    id: None,
                    detector_name: "New Primary Record".to_string(),
                    why,
                    confidence: "high".to_string(),
                    risk_level: risk_level.to_string(),
                    confirmation_checklist: serde_json::to_string(&checklist)?,
                    created_at: Utc::now().to_rfc3339(),
                };
                let exists = lead_exists(conn, "New Primary Record", &lead.why)?;
                if !exists {
                    let lid = insert_lead(conn, &lead, &[evidence_id])?;
                    new_lead_ids.push(lid);
                }
            }

            // 3. Money Threshold Detector
            for mat in re_money.find_iter(&excerpt) {
                let amount_str = mat.as_str().replace("$", "").replace(",", "");
                if let Ok(amount) = amount_str.parse::<f64>() {
                    if amount >= money_threshold {
                        let why = format!("Found a transaction/budget amount of ${} in '{}' evidence. This exceeds your configured threshold of ${}.", format_money(amount), source_name, format_money(money_threshold));
                        let checklist = vec![
                            "Verify that this amount is an actual expenditure, not a general projection",
                            "Identify the payee, vendor, or department receiving the funds",
                            "Look up previous contracts with this vendor to see if costs have ballooned"
                        ];
                        let lead = Lead {
                            id: None,
                            detector_name: "Money Threshold".to_string(),
                            why,
                            confidence: "high".to_string(),
                            risk_level: risk_level.to_string(),
                            confirmation_checklist: serde_json::to_string(&checklist)?,
                            created_at: Utc::now().to_rfc3339(),
                        };
                        let exists = lead_exists(conn, "Money Threshold", &lead.why)?;
                        if !exists {
                            let lid = insert_lead(conn, &lead, &[evidence_id])?;
                            new_lead_ids.push(lid);
                        }
                    }
                }
            }

            // 4. Decision / Vote Detector
            if re_vote.is_match(&excerpt) {
                let matched_words: Vec<&str> = re_vote.find_iter(&excerpt).map(|m| m.as_str()).collect();
                let why = format!("VOTING SIGNAL: Found vote/decision keywords ({:?}) in '{}' record, indicating an official board action has taken place.", matched_words, source_name);
                let checklist = vec![
                    "Record the exact vote tally (e.g. 5-2, unanimous)",
                    "Identify which board members voted 'No' or abstained, and find out why",
                    "Determine what immediate local impact this decision has on the community"
                ];
                let lead = Lead {
                    id: None,
                    detector_name: "Decision / Vote".to_string(),
                    why,
                    confidence: "medium".to_string(),
                    risk_level: risk_level.to_string(),
                    confirmation_checklist: serde_json::to_string(&checklist)?,
                    created_at: Utc::now().to_rfc3339(),
                };
                let exists = lead_exists(conn, "Decision / Vote", &lead.why)?;
                if !exists {
                    let lid = insert_lead(conn, &lead, &[evidence_id])?;
                    new_lead_ids.push(lid);
                }
            }

            // 5. Personnel Change Detector
            if re_personnel.is_match(&excerpt) {
                let matched_words: Vec<&str> = re_personnel.find_iter(&excerpt).map(|m| m.as_str()).collect();
                let why = format!("PERSONNEL SIGNAL: Detected appointment or resignation keywords ({:?}) in '{}' record, suggesting staff transitions.", matched_words, source_name);
                let checklist = vec![
                    "Confirm the name and title of the departing/entering official",
                    "Verify the official reason given for the departure or appointment",
                    "Check if a salary change or severance package was approved"
                ];
                let lead = Lead {
                    id: None,
                    detector_name: "Personnel Change".to_string(),
                    why,
                    confidence: "medium".to_string(),
                    risk_level: risk_level.to_string(),
                    confirmation_checklist: serde_json::to_string(&checklist)?,
                    created_at: Utc::now().to_rfc3339(),
                };
                let exists = lead_exists(conn, "Personnel Change", &lead.why)?;
                if !exists {
                    let lid = insert_lead(conn, &lead, &[evidence_id])?;
                    new_lead_ids.push(lid);
                }
            }

            // 6. Public Meeting Detector
            if re_meeting.is_match(&excerpt) {
                let why = format!("MEETING SCHEDULED: Meeting notification detected in '{}' record. Public attendance or comment may be permitted.", source_name);
                let checklist = vec![
                    "Verify the date, time, and physical/virtual address of the meeting",
                    "Download the meeting agenda when it becomes available",
                    "Identify the public comment signup rules (e.g. register 24 hours in advance)"
                ];
                let lead = Lead {
                    id: None,
                    detector_name: "Public Meeting Scheduled".to_string(),
                    why,
                    confidence: "medium".to_string(),
                    risk_level: "low".to_string(),
                    confirmation_checklist: serde_json::to_string(&checklist)?,
                    created_at: Utc::now().to_rfc3339(),
                };
                let exists = lead_exists(conn, "Public Meeting Scheduled", &lead.why)?;
                if !exists {
                    let lid = insert_lead(conn, &lead, &[evidence_id])?;
                    new_lead_ids.push(lid);
                }
            }

            // 7. Deadline Detector
            if re_deadline.is_match(&excerpt) {
                let why = format!("DEADLINE SIGNAL: Found deadline/due-date language in '{}' record, indicating key timelines for submissions or responses.", source_name);
                let checklist = vec![
                    "Pinpoint the exact submission deadline date and time",
                    "Outline the required criteria for submitting a public response",
                    "Determine who receives the applications or proposals"
                ];
                let lead = Lead {
                    id: None,
                    detector_name: "Deadline".to_string(),
                    why,
                    confidence: "medium".to_string(),
                    risk_level: "low".to_string(),
                    confirmation_checklist: serde_json::to_string(&checklist)?,
                    created_at: Utc::now().to_rfc3339(),
                };
                let exists = lead_exists(conn, "Deadline", &lead.why)?;
                if !exists {
                    let lid = insert_lead(conn, &lead, &[evidence_id])?;
                    new_lead_ids.push(lid);
                }
            }

            // 8. Watchlist Hit Detector
            for term in &watchlist {
                let term_escaped = regex::escape(term);
                let re_term = Regex::new(&format!(r"(?i)\b{}\b", term_escaped)).unwrap();
                if re_term.is_match(&excerpt) {
                    let why = format!("WATCHLIST HIT: The tracked keyword '{}' was found in a new record from '{}'.", term, source_name);
                    let checklist = vec![
                        format!("Examine how '{}' is related to the other entities in the document", term),
                        "Check if this hit indicates a potential conflict of interest".to_string(),
                        "Look up historical minutes to find previous references to this item".to_string()
                    ];
                    let lead = Lead {
                        id: None,
                        detector_name: "Watchlist Hit".to_string(),
                        why,
                        confidence: "high".to_string(),
                        risk_level: risk_level.to_string(),
                        confirmation_checklist: serde_json::to_string(&checklist)?,
                        created_at: Utc::now().to_rfc3339(),
                    };
                    let exists = lead_exists(conn, "Watchlist Hit", &lead.why)?;
                    if !exists {
                        let lid = insert_lead(conn, &lead, &[evidence_id])?;
                        new_lead_ids.push(lid);
                    }
                }
            }
        }
    }
    
    Ok(new_lead_ids)
}

fn lead_exists(conn: &Connection, detector_name: &str, why: &str) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT count(*) FROM leads WHERE detector_name = ?1 AND why = ?2")?;
    let count: i32 = stmt.query_row([detector_name, why], |row| row.get(0))?;
    Ok(count > 0)
}

fn format_money(val: f64) -> String {
    let s = format!("{:.2}", val);
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = parts[1];
    
    let mut result = String::new();
    let num_bytes = integer_part.len();
    for (i, c) in integer_part.chars().enumerate() {
        if i > 0 && (num_bytes - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.push('.');
    result.push_str(decimal_part);
    result
}
