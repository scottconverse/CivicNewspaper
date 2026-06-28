// core/guardrails.rs
use super::db::{get_draft, get_evidence_by_lead};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;

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

// Editor-editable guardrails: these are only the built-in STARTING lists. A
// newsroom edits them via Settings (persisted under the `guardrails.terms`
// setting). Matching any word raises a WARNING by default; only words an editor
// explicitly marks as blocking escalate to a publish-blocking ERROR — so the
// machine never imposes a stop the editor didn't choose.
pub const DEFAULT_ACCUSATORY: &[&str] = &[
    "corrupt",
    "stole",
    "illegal",
    "fraud",
    "embezzle",
    "bribe",
    "scam",
    "theft",
    "criminal",
    "guilty",
    "conspiracy",
    "extortion",
    "misconduct",
    "kickback",
    "laundering",
    "arrested",
    "charged",
    "indicted",
    "convicted",
    "prosecuted",
];
pub const DEFAULT_LEGAL: &[&str] = &[
    "arrested",
    "charged",
    "indicted",
    "accused",
    "suspect",
    "theft",
    "embezzle",
    "fraud",
    "misconduct",
];

const GUARDRAIL_SETTINGS_KEY: &str = "guardrails.terms";

/// Editor-editable guardrail configuration (per newsroom).
/// - `accusatory`: words that, used without a source link, raise an
///   "Accusatory Language" issue.
/// - `legal`: charge/legal words that, used without "alleged", raise a
///   "Legal Naming" (presumption-of-innocence) issue.
/// - `blocking`: the subset of words (case-insensitive, from either list) that
///   escalate a raised issue from a warning to a publish-blocking error. Empty
///   by default => warn-only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailConfig {
    pub accusatory: Vec<String>,
    pub legal: Vec<String>,
    pub blocking: Vec<String>,
}

impl Default for GuardrailConfig {
    fn default() -> Self {
        GuardrailConfig {
            accusatory: DEFAULT_ACCUSATORY.iter().map(|s| s.to_string()).collect(),
            legal: DEFAULT_LEGAL.iter().map(|s| s.to_string()).collect(),
            blocking: Vec::new(),
        }
    }
}

/// Load the newsroom's guardrail config from settings, falling back to the
/// built-in defaults when unset or unparseable.
pub fn load_guardrail_config(conn: &Connection) -> GuardrailConfig {
    let raw: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [GUARDRAIL_SETTINGS_KEY],
            |r| r.get::<_, String>(0),
        )
        .ok();
    match raw {
        Some(json) => serde_json::from_str(&json).unwrap_or_default(),
        None => GuardrailConfig::default(),
    }
}

/// Persist the newsroom's guardrail config to settings.
pub fn save_guardrail_config(
    conn: &Connection,
    config: &GuardrailConfig,
) -> Result<(), Box<dyn Error>> {
    // RE-AUDIT NEW-4: a blocking word that isn't in the accusatory/legal lists can
    // never fire, so keep the persisted config self-consistent by dropping any
    // blocking word not present in either list (case-insensitive).
    let known: HashSet<String> = config
        .accusatory
        .iter()
        .chain(config.legal.iter())
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();
    let normalized = GuardrailConfig {
        accusatory: config.accusatory.clone(),
        legal: config.legal.clone(),
        blocking: config
            .blocking
            .iter()
            .filter(|w| known.contains(&w.trim().to_lowercase()))
            .cloned()
            .collect(),
    };
    let json = serde_json::to_string(&normalized)?;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params![GUARDRAIL_SETTINGS_KEY, json],
    )?;
    Ok(())
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

    // Editor-editable lists. Words warn by default; only words in `blocking`
    // (which the editor opts in via Settings) escalate an issue to a publish-
    // blocking error.
    let config = load_guardrail_config(conn);
    let blocking: HashSet<String> = config
        .blocking
        .iter()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();
    let accusatory_words: Vec<String> = config
        .accusatory
        .iter()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();
    let legal_terms: Vec<String> = config
        .legal
        .iter()
        .map(|w| w.trim().to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();

    for (p_idx, paragraph) in paragraphs.iter().enumerate() {
        // Skip headers and markdown code blocks
        if paragraph.starts_with("#") || paragraph.starts_with("```") {
            continue;
        }

        // 1. Source-link coverage check. This is advisory by default: it helps
        // editors see unsupported factual claims without deciding what they may
        // publish.
        let has_citation = paragraph.contains("evidence:") || paragraph.contains("evidence://");

        // Let's check if the paragraph length is significant enough to count as a factual claim
        if paragraph.len() > 30 && !has_citation {
            issues.push(GuardrailsIssue {
                category: "Citation Coverage".to_string(),
                message: "Paragraph may contain factual claims without a linked source."
                    .to_string(),
                severity: "warning".to_string(),
                paragraph_index: p_idx,
            });
        }

        // 2. Accusatory Language & Citation Link Check
        let lower_p = paragraph.to_lowercase();
        // RE-AUDIT NEW-3: match on whole words (with common inflections) rather
        // than raw substrings, so "charged" no longer fires inside "surcharged"
        // and "scam" no longer fires inside "scampi", while "embezzle" still
        // matches "embezzled"/"embezzlement".
        let tokens = tokenize(paragraph);
        let found_accusatory: Vec<&String> = accusatory_words
            .iter()
            .filter(|word| term_in_tokens(&tokens, word.as_str()))
            .collect();

        if !found_accusatory.is_empty() {
            if !has_citation {
                // Warn by default; escalate to a publish-blocking error only when a
                // matched word was explicitly marked blocking by the editor.
                let severity = if found_accusatory
                    .iter()
                    .any(|w| blocking.contains(w.as_str()))
                {
                    "error"
                } else {
                    "warning"
                };
                issues.push(GuardrailsIssue {
                    category: "Accusatory Language".to_string(),
                    message: format!(
                        "Accusatory term(s) {:?} used without a supporting source link. Add a source, attribute carefully, or approve with editorial judgment.",
                        found_accusatory
                    ),
                    severity: severity.to_string(),
                    paragraph_index: p_idx,
                });
            }

            // 3. Presumption of Innocence / Legal Naming Rule
            // If legal/charge terms are present, 'alleged'/'allegedly' must also appear.
            let found_legal: Vec<&String> = legal_terms
                .iter()
                .filter(|term| term_in_tokens(&tokens, term.as_str()))
                .collect();

            if !found_legal.is_empty()
                && !lower_p.contains("alleged")
                && !lower_p.contains("allegedly")
            {
                let severity = if found_legal.iter().any(|w| blocking.contains(w.as_str())) {
                    "error"
                } else {
                    "warning"
                };
                issues.push(GuardrailsIssue {
                    category: "Legal Naming".to_string(),
                    message: "Presumption of innocence safeguard: Accusatory/charge words are used, but the modifier 'alleged' or 'allegedly' is missing. Please rephrase to clarify these are indicators/accusations under review.".to_string(),
                    severity: severity.to_string(),
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
        let window = &p_words[i..i + min_words];

        // Look for this window in excerpt words
        let mut found = false;
        let mut j = 0;
        while j <= e_words.len() - min_words {
            if &e_words[j..j + min_words] == window {
                // Found match! Let's extend it as long as it matches
                let mut match_len = min_words;
                while i + match_len < p_words.len() && j + match_len < e_words.len() {
                    if p_words[i + match_len] == e_words[j + match_len] {
                        match_len += 1;
                    } else {
                        break;
                    }
                }

                let match_str = p_words[i..i + match_len].join(" ");
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

/// Common English inflection suffixes used to match a guard term against a token
/// without over-matching unrelated words. "" = exact match. This deliberately
/// favours precision (fewer false positives) over recall — the editor owns the
/// word list and can add inflected forms explicitly (RE-AUDIT NEW-3).
const INFLECTION_SUFFIXES: &[&str] = &[
    "", "s", "es", "d", "ed", "ing", "ings", "ment", "ments", "er", "ers", "ion", "ions",
];

/// True if `token` is `term` or a common inflection of it (prefix + allow-listed
/// suffix). Both must already be lower-case (tokenize lower-cases).
fn token_matches_term(token: &str, term: &str) -> bool {
    token
        .strip_prefix(term)
        .map(|suffix| INFLECTION_SUFFIXES.contains(&suffix))
        .unwrap_or(false)
}

/// True if any token in the paragraph matches `term` as a whole word/inflection.
fn term_in_tokens(tokens: &[String], term: &str) -> bool {
    tokens.iter().any(|t| token_matches_term(t, term))
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
