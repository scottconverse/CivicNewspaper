// core/guardrails.rs
use rusqlite::Connection;
use std::error::Error;
use serde::{Deserialize, Serialize};
use super::db::{get_draft, get_evidence_by_lead};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailsIssue {
    pub category: String, // "Citation Coverage", "Accusatory Language", "Verbatim Overlap", "Legal Naming"
    pub message: String,
    pub severity: String, // "warning", "error"
    pub paragraph_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailsReport {
    pub is_clean: bool,
    pub issues: Vec<GuardrailsIssue>,
}

pub fn run_guardrails_check(
    conn: &Connection,
    draft_id: i32,
) -> Result<GuardrailsReport, Box<dyn Error>> {
    let mut issues = Vec::new();
    
    let draft = match get_draft(conn, draft_id)? {
        Some(d) => d,
        None => return Err(format!("Draft ID {} not found", draft_id).into()),
    };

    let _title = draft.title;
    let content = draft.content;
    let lead_id = draft.lead_id;

    // Fetch linked evidence items to check for verbatim overlap
    let evidence_items = if let Some(lid) = lead_id {
        get_evidence_by_lead(conn, lid)?
    } else {
        Vec::new()
    };

    // Split content into paragraphs
    let paragraphs: Vec<&str> = content
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    let accusatory_words = vec![
        "corrupt", "stole", "illegal", "fraud", "embezzle", "bribe", 
        "scam", "theft", "criminal", "guilty", "conspiracy", "extortion", 
        "misconduct", "kickback", "laundering", "arrested", "charged", 
        "indicted", "convicted", "prosecuted"
    ];

    for (p_idx, paragraph) in paragraphs.iter().enumerate() {
        // Skip headers and markdown code blocks
        if paragraph.starts_with("#") || paragraph.starts_with("```") {
            continue;
        }

        // 1. Citation Coverage Check
        // Every factual claim paragraph must reference at least one `evidence:` citation
        let has_citation = paragraph.contains("evidence:") || paragraph.contains("evidence://");
        
        // Let's check if the paragraph length is significant enough to count as a factual claim
        if paragraph.len() > 30 && !has_citation {
            issues.push(GuardrailsIssue {
                category: "Citation Coverage".to_string(),
                message: "Paragraph contains factual claims but is missing an evidence citation link (e.g. '(evidence:101)').".to_string(),
                severity: "warning".to_string(),
                paragraph_index: p_idx,
            });
        }

        // 2. Accusatory Language & Citation Link Check
        let lower_p = paragraph.to_lowercase();
        let mut found_accusatory = Vec::new();
        for &word in &accusatory_words {
            if lower_p.contains(word) {
                found_accusatory.push(word);
            }
        }

        if !found_accusatory.is_empty() {
            if !has_citation {
                issues.push(GuardrailsIssue {
                    category: "Accusatory Language".to_string(),
                    message: format!(
                        "Accusatory term(s) {:?} used without a supporting evidence citation link. Add evidence link to substantiate.",
                        found_accusatory
                    ),
                    severity: "error".to_string(),
                    paragraph_index: p_idx,
                });
            }

            // 3. Presumption of Innocence / Legal Naming Rule
            // If legal/accusatory charge terms are present, the word 'alleged' or 'allegedly' must also be present
            let legal_terms = vec!["arrested", "charged", "indicted", "accused", "suspect", "theft", "embezzle", "fraud", "misconduct"];
            let contains_legal = legal_terms.iter().any(|&term| lower_p.contains(term));
            
            if contains_legal && !lower_p.contains("alleged") && !lower_p.contains("allegedly") {
                issues.push(GuardrailsIssue {
                    category: "Legal Naming".to_string(),
                    message: "Presumption of innocence safeguard: Accusatory/charge words are used, but the modifier 'alleged' or 'allegedly' is missing. Please rephrase to clarify these are indicators/accusations under review.".to_string(),
                    severity: "error".to_string(),
                    paragraph_index: p_idx,
                });
            }
        }

        // 4. Verbatim Source Overlap Check (7+ words sequence matching)
        for evidence in &evidence_items {
            let matches = find_verbatim_overlap(paragraph, &evidence.excerpt, 7);
            for overlap in matches {
                issues.push(GuardrailsIssue {
                    category: "Verbatim Overlap".to_string(),
                    message: format!(
                        "Verbatim sequence of 7+ words copied directly from evidence ID {} (Source URL: {}): '{}'. Please rewrite in your own words or format as a blockquote.",
                        evidence.id.unwrap_or(0),
                        evidence.url.clone().unwrap_or_else(|| "N/A".to_string()),
                        overlap
                    ),
                    severity: "warning".to_string(),
                    paragraph_index: p_idx,
                });
            }
        }
    }

    let is_clean = !issues.iter().any(|i| i.severity == "error");

    Ok(GuardrailsReport { is_clean, issues })
}

// Find verbatim sequences of length N matching between a paragraph and an evidence text
fn find_verbatim_overlap(paragraph: &str, excerpt: &str, min_words: usize) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Normalize and split into words
    let p_words = tokenize(paragraph);
    let e_words = tokenize(excerpt);
    
    if p_words.len() < min_words || e_words.len() < min_words {
        return matches;
    }

    // Check sliding windows of size `min_words`
    let mut i = 0;
    while i <= p_words.len() - min_words {
        let window = &p_words[i .. i + min_words];
        
        // Look for this window in excerpt words
        let mut found = false;
        let mut j = 0;
        while j <= e_words.len() - min_words {
            if &e_words[j .. j + min_words] == window {
                // Found match! Let's extend it as long as it matches
                let mut match_len = min_words;
                while i + match_len < p_words.len() && j + match_len < e_words.len() {
                    if p_words[i + match_len] == e_words[j + match_len] {
                        match_len += 1;
                    } else {
                        break;
                    }
                }
                
                let match_str = p_words[i .. i + match_len].join(" ");
                matches.push(match_str);
                
                i += match_len - 1; // Advance outer index
                found = true;
                break;
            }
            j += 1;
        }
        
        if found {
            // Re-evaluate at new advanced i
        }
        i += 1;
    }
    
    // Deduplicate and filter nested matches
    matches.sort_by_key(|b| std::cmp::Reverse(b.len()));
    let mut filtered: Vec<String> = Vec::new();
    for m in matches {
        if !filtered.iter().any(|f| f.contains(&m)) {
            filtered.push(m);
        }
    }
    
    filtered
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c.is_whitespace() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
