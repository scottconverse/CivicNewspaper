// core/scraper.rs
use rusqlite::Connection;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use feed_rs::parser;
use sha2::{Sha256, Digest};
use chrono::Utc;
use serde_json;
use super::db::{update_source_status, insert_evidence_item, get_evidence_by_hash, Source, EvidenceItem, DbConn};

// Generate content hash to deduplicate evidence items
pub fn compute_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Simple entity extraction using regex/keywords for our evidence entities list
pub fn extract_entities(text: &str) -> Vec<String> {
    let mut entities = Vec::new();
    let re_dollar = regex::Regex::new(r"\$[0-9,]+(?:\.[0-9]+)?").unwrap();
    let re_org = regex::Regex::new(r"\b[A-Z][a-zA-Z0-9&]+(?:\s+[A-Z][a-zA-Z0-9&]+)*\s+(?:Board|Council|Committee|Department|District|Commission|Agency|Association|Corp|Inc|LLC)\b").unwrap();
    
    // Extract dollar amounts
    for mat in re_dollar.find_iter(text) {
        entities.push(mat.as_str().to_string());
    }
    
    // Extract formal organizations
    for mat in re_org.find_iter(text) {
        entities.push(mat.as_str().to_string());
    }

    entities.sort();
    entities.dedup();
    entities
}

pub async fn scrape_all_sources(db: &DbConn) -> Result<(), Box<dyn Error>> {
    let sources = {
        let conn = db.lock().map_err(|_| "Failed to lock database")?;
        super::db::list_sources(&conn)?
    };
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("CivicNewsScraper/1.0 (+http://127.0.0.1:12053)")
        .build()?;

    for source in sources {
        // Enforce 3-second politeness delay between source scraping
        sleep(Duration::from_secs(3)).await;
        
        let source_id = source.id.unwrap_or(0);
        println!("Scraping source: {} ({})", source.name, source.url);
        
        match scrape_source(&client, db, &source).await {
            Ok(_) => {
                println!("Scraped source successfully: {}", source.name);
                let conn = db.lock().map_err(|_| "Failed to lock database")?;
                let _ = update_source_status(&conn, source_id, "online", true);
            }
            Err(e) => {
                eprintln!("Scrape failed for {}: {}", source.name, e);
                let conn = db.lock().map_err(|_| "Failed to lock database")?;
                let _ = update_source_status(&conn, source_id, "offline", false);
            }
        }
    }
    Ok(())
}

async fn scrape_source(client: &Client, db: &DbConn, source: &Source) -> Result<(), Box<dyn Error>> {
    let response = client.get(&source.url).send().await?;
    if !response.status().is_success() {
        return Err(format!("HTTP error status: {}", response.status()).into());
    }
    
    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|c| c.to_str().ok())
        .unwrap_or("")
        .to_string();
        
    let body_bytes = response.bytes().await?;
    
    // Detect if RSS feed or raw HTML
    let is_feed = content_type.contains("xml") || content_type.contains("rss") || content_type.contains("atom") 
                  || body_bytes.starts_with(b"<?xml") || body_bytes.starts_with(b"<rss") || body_bytes.starts_with(b"<feed");

    if is_feed {
        let feed = parser::parse(&body_bytes[..])?;
        for entry in feed.entries {
            let title = entry.title.map(|t| t.content).unwrap_or_default();
            let description = entry.summary.map(|s| s.content).unwrap_or_default();
            let url = entry.links.first().map(|l| l.href.clone());
            
            // Build excerpt based on source type constraints
            let excerpt = if source.r#type == "media_lead" {
                // Media headlines: NEVER store body text. Title/Headline only.
                format!("Headline: {}", title)
            } else {
                format!("Title: {}\nDescription: {}", title, description)
            };

            if excerpt.trim().is_empty() {
                continue;
            }

            let hash = compute_hash(&excerpt);
            
            // Check for duplicates
            let is_new = {
                let conn = db.lock().map_err(|_| "Failed to lock database")?;
                get_evidence_by_hash(&conn, &hash)?.is_none()
            };
            
            if is_new {
                let entities = extract_entities(&excerpt);
                let entities_json = serde_json::to_string(&entities).unwrap_or_else(|_| "[]".to_string());
                
                let item = EvidenceItem {
                    id: None,
                    source_id: source.id.unwrap(),
                    url,
                    fetched_at: Utc::now().to_rfc3339(),
                    excerpt,
                    content_hash: hash,
                    entities: entities_json,
                };
                let conn = db.lock().map_err(|_| "Failed to lock database")?;
                insert_evidence_item(&conn, &item)?;
            }
        }
    } else {
        // Raw HTML Webpage
        let html_text = String::from_utf8_lossy(&body_bytes);
        // Simple HTML text extractor (strips scripts, styles, tags)
        let text_content = clean_html(&html_text);
        
        let excerpt = if source.r#type == "media_lead" {
            // Store only first line/title if it's media
            let first_line = text_content.lines().next().unwrap_or("Media Headline");
            format!("Headline: {}", first_line)
        } else {
            // For agenda/public record, split into reasonable paragraphs and store as multiple items
            text_content
        };

        if !excerpt.trim().is_empty() {
            // Split large text blocks into chunks to prevent database bloating
            for chunk in chunk_text(&excerpt, 2000) {
                let hash = compute_hash(&chunk);
                let is_new = {
                    let conn = db.lock().map_err(|_| "Failed to lock database")?;
                    get_evidence_by_hash(&conn, &hash)?.is_none()
                };
                if is_new {
                    let entities = extract_entities(&chunk);
                    let entities_json = serde_json::to_string(&entities).unwrap_or_else(|_| "[]".to_string());
                    
                    let item = EvidenceItem {
                        id: None,
                        source_id: source.id.unwrap(),
                        url: Some(source.url.clone()),
                        fetched_at: Utc::now().to_rfc3339(),
                        excerpt: chunk,
                        content_hash: hash,
                        entities: entities_json,
                    };
                    let conn = db.lock().map_err(|_| "Failed to lock database")?;
                    insert_evidence_item(&conn, &item)?;
                }
            }
        }
    }

    Ok(())
}

fn clean_html(html: &str) -> String {
    let re_script_style = regex::Regex::new(r"(?s)<(script|style)[^>]*>.*?</\1>").unwrap();
    let re_tags = regex::Regex::new(r"<[^>]*>").unwrap();
    
    let step1 = re_script_style.replace_all(html, "");
    let step2 = re_tags.replace_all(&step1, " ");
    
    // Normalize whitespace
    let mut cleaned = String::new();
    for line in step2.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            cleaned.push_str(trimmed);
            cleaned.push('\n');
        }
    }
    cleaned
}

fn chunk_text(text: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    
    for paragraph in text.split("\n") {
        let p_trimmed = paragraph.trim();
        if p_trimmed.is_empty() {
            continue;
        }
        if current_chunk.len() + p_trimmed.len() > chunk_size {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
        }
        current_chunk.push_str(p_trimmed);
        current_chunk.push('\n');
    }
    
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }
    
    if chunks.is_empty() && !text.trim().is_empty() {
        chunks.push(text.trim().to_string());
    }
    
    chunks
}
