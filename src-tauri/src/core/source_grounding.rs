use std::collections::HashSet;

use super::db::EvidenceItem;

const STOPWORDS: &[&str] = &[
    "about",
    "after",
    "again",
    "against",
    "also",
    "alerts",
    "before",
    "being",
    "between",
    "could",
    "city",
    "community",
    "contact",
    "department",
    "departments",
    "during",
    "editor",
    "event",
    "events",
    "explore",
    "first",
    "from",
    "have",
    "government",
    "information",
    "into",
    "latest",
    "local",
    "longmont",
    "more",
    "news",
    "newsletter",
    "program",
    "programs",
    "original",
    "public",
    "reader",
    "residents",
    "review",
    "services",
    "should",
    "source",
    "sources",
    "story",
    "suggested",
    "that",
    "their",
    "there",
    "these",
    "this",
    "through",
    "under",
    "updates",
    "were",
    "where",
    "which",
    "while",
    "will",
    "with",
    "would",
];

fn specific_topic_text(text: &str) -> String {
    let first_line = text.lines().next().unwrap_or(text).trim();
    let before_metadata = first_line
        .split(" Editor context:")
        .next()
        .unwrap_or(first_line)
        .split(" Suggested treatment:")
        .next()
        .unwrap_or(first_line);
    let before_summary = before_metadata
        .split(':')
        .next()
        .unwrap_or(before_metadata)
        .trim();
    if grounding_tokens(before_summary).len() >= 3 {
        before_summary.to_string()
    } else if let Some((_, after_summary)) = before_metadata.split_once(':') {
        let after_summary = after_summary.trim();
        if grounding_tokens(after_summary).len() >= 3 {
            return after_summary.to_string();
        }
        before_metadata.to_string()
    } else {
        before_metadata.to_string()
    }
}

pub fn grounding_tokens(text: &str) -> HashSet<String> {
    let stop: HashSet<&str> = STOPWORDS.iter().copied().collect();
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
        .filter_map(|token| {
            let token = token.trim();
            if token.len() < 4
                || stop.contains(token)
                || token.chars().all(|ch| ch.is_ascii_digit())
            {
                return None;
            }
            Some(light_stem(token))
        })
        .filter(|token| token.len() >= 4 && !stop.contains(token.as_str()))
        .collect()
}

fn light_stem(token: &str) -> String {
    if token.len() > 6 && token.ends_with("ies") {
        format!("{}y", &token[..token.len() - 3])
    } else if token.len() > 5 && token.ends_with("ing") {
        token[..token.len() - 3].to_string()
    } else if token.len() > 5 && token.ends_with("ed") {
        token[..token.len() - 2].to_string()
    } else if token.len() > 5 && token.ends_with('s') {
        token[..token.len() - 1].to_string()
    } else {
        token.to_string()
    }
}

fn required_overlap(lead_token_count: usize) -> usize {
    if lead_token_count <= 3 {
        lead_token_count.clamp(1, 2)
    } else if lead_token_count <= 5 {
        3
    } else {
        lead_token_count.div_ceil(3).clamp(4, 6)
    }
}

pub fn evidence_matches_topic(lead_text: &str, evidence_text: &str) -> bool {
    let topic = specific_topic_text(lead_text);
    let lead_tokens = grounding_tokens(&topic);
    if lead_tokens.is_empty() {
        return false;
    }
    let evidence_tokens = grounding_tokens(evidence_text);
    let overlap = lead_tokens.intersection(&evidence_tokens).count();
    let required = required_overlap(lead_tokens.len());
    overlap >= required && (overlap as f32 / lead_tokens.len() as f32) >= 0.45
}

pub fn filter_topic_matched_evidence(
    lead_text: &str,
    evidence_items: &[EvidenceItem],
) -> Vec<EvidenceItem> {
    evidence_items
        .iter()
        .filter(|item| evidence_matches_topic(lead_text, &item.excerpt))
        .cloned()
        .collect()
}

pub fn evidence_alignment_issue(
    lead_text: &str,
    evidence_items: &[EvidenceItem],
) -> Option<String> {
    if evidence_items.is_empty() {
        return None;
    }
    if evidence_items
        .iter()
        .any(|item| evidence_matches_topic(lead_text, &item.excerpt))
    {
        return None;
    }
    Some(
        "Linked source documents do not appear to match this lead topic. Attach the correct source material before drafting or publishing reader-facing copy."
            .to_string(),
    )
}

pub fn paragraph_is_source_aligned(paragraph: &str, source_text: &str) -> bool {
    let paragraph = strip_evidence_citation_syntax(paragraph);
    let paragraph_tokens = grounding_tokens(&paragraph);
    if paragraph_tokens.len() < 4 {
        return true;
    }
    if !specific_factual_anchors_are_supported(&paragraph, source_text) {
        return false;
    }
    let source_tokens = grounding_tokens(source_text);
    let overlap = paragraph_tokens.intersection(&source_tokens).count();
    let required = paragraph_tokens.len().div_ceil(4).clamp(2, 6);
    overlap >= required
}

fn strip_evidence_citation_syntax(text: &str) -> String {
    let citation_re = regex::Regex::new(
        r"(?i)\[[^\]]+\]\(\s*evidence:\s*(?://)?\s*\d+\s*\)|evidence:\s*(?://)?\s*\d+",
    )
    .expect("valid evidence citation regex");
    citation_re.replace_all(text, " ").to_string()
}

fn specific_factual_anchors_are_supported(paragraph: &str, source_text: &str) -> bool {
    let paragraph_lower = paragraph.to_lowercase();
    let source_lower = source_text.to_lowercase();
    factual_anchor_tokens(&paragraph_lower)
        .into_iter()
        .all(|anchor| source_lower.contains(&anchor))
}

fn factual_anchor_tokens(text: &str) -> HashSet<String> {
    let mut anchors = HashSet::new();
    collect_numeric_anchors(text, &mut anchors);
    collect_calendar_anchors(text, &mut anchors);
    collect_money_word_anchors(text, &mut anchors);
    collect_high_risk_action_anchors(text, &mut anchors);
    anchors
}

fn collect_numeric_anchors(text: &str, anchors: &mut HashSet<String>) {
    for token in text
        .split(|ch: char| {
            !(ch.is_ascii_alphanumeric() || ch == '$' || ch == '%' || ch == '.' || ch == ',')
        })
        .map(str::trim)
        .filter(|token| token.chars().any(|ch| ch.is_ascii_digit()))
    {
        let normalized = token.trim_matches(|ch: char| ch == ',' || ch == '.');
        if normalized.len() >= 2 || normalized.starts_with('$') || normalized.ends_with('%') {
            anchors.insert(normalized.to_string());
        }
    }
}

fn collect_calendar_anchors(text: &str, anchors: &mut HashSet<String>) {
    const TERMS: &[&str] = &[
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
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    for term in TERMS {
        if text.contains(term) {
            anchors.insert((*term).to_string());
        }
    }
}

fn collect_money_word_anchors(text: &str, anchors: &mut HashSet<String>) {
    const TERMS: &[&str] = &[
        "dollar", "dollars", "million", "billion", "grant", "budget", "fee", "fees", "tax", "taxes",
    ];
    for term in TERMS {
        if text.contains(term) {
            anchors.insert((*term).to_string());
        }
    }
}

fn collect_high_risk_action_anchors(text: &str, anchors: &mut HashSet<String>) {
    const TERMS: &[&str] = &[
        "approved",
        "adopted",
        "rejected",
        "denied",
        "voted",
        "passed",
        "closed",
        "closing",
        "opened",
        "launched",
        "awarded",
        "canceled",
        "cancelled",
        "suspended",
        "delayed",
        "increased",
        "decreased",
        "requires",
        "required",
    ];
    for term in TERMS {
        if text.contains(term) {
            anchors.insert((*term).to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn council_roof_lead_does_not_match_event_calendar() {
        let lead = "Council vote on library roof contract and rooftop repair project for Downtown Longmont Public Library.";
        let evidence = "Summer Concert Series: Road Pony 7pm - 8:30pm Longmont Museum Creative District Event.";

        assert!(!evidence_matches_topic(lead, evidence));
    }

    #[test]
    fn library_program_lead_matches_library_program_evidence() {
        let lead = "Teen Temporary Tattoo Studio Launch at Longmont Public Library";
        let evidence = "Teen Temporary Tattoo Studio Wednesday July 1 Longmont Public Library.";

        assert!(evidence_matches_topic(lead, evidence));
    }

    #[test]
    fn summer_reading_lead_does_not_match_broad_events_calendar() {
        let lead = "Summer Reading Challenge Starts at Longmont Public Library: The 2026 Summer Reading Challenge is starting on Wednesday, May 21 with activities running through July 31.";
        let evidence = "Wednesday, July 1 6 pm - 7 pm Longmont Public Library A free gaming club for 3rd-5th graders. Yoga Storytime. Summer Science Series. Longmont Museum.";

        assert!(!evidence_matches_topic(lead, evidence));
    }

    #[test]
    fn summer_reading_lead_matches_specific_challenge_evidence() {
        let lead = "Summer Reading Challenge Starts at Longmont Public Library: The 2026 Summer Reading Challenge is starting on Wednesday, May 21 with activities running through July 31.";
        let evidence = "Summer Reading Challenge starts at Longmont Public Library on May 21 and runs through July 31 with prize opportunities.";

        assert!(evidence_matches_topic(lead, evidence));
    }

    #[test]
    fn rescue_source_label_lead_matches_its_own_evidence_sentence() {
        let lead = "Longmont source bundle: Housing office approved a July application deadline for a new affordable housing grant Housing office approved a July application deadline for a new affordable housing grant.";
        let evidence = "Housing office approved a July application deadline for a new affordable housing grant.";

        assert!(evidence_matches_topic(lead, evidence));
    }

    #[test]
    fn cited_paragraph_with_many_unsupported_claims_does_not_align() {
        let paragraph = "Council members will vote on a roof repair project, increase capacity by 40 percent, and spend ten million dollars over ten years while residents attend Road Pony.";
        let source = "Summer Concert Series: Road Pony 7pm - 8:30pm Longmont Museum Creative District Event.";

        assert!(!paragraph_is_source_aligned(paragraph, source));
    }

    #[test]
    fn cited_paragraph_cannot_claim_approval_when_source_only_says_review() {
        let paragraph = "Council approved a $10 million library roof contract that will close the building for two years.";
        let source = "City Council will review a library roof contract at its next public meeting.";

        assert!(!paragraph_is_source_aligned(paragraph, source));
    }

    #[test]
    fn cited_paragraph_with_supported_action_and_amount_aligns() {
        let paragraph =
            "Council approved a $10 million library roof contract [Source](evidence:1).";
        let source =
            "City Council approved a $10 million library roof contract after public discussion.";

        assert!(paragraph_is_source_aligned(paragraph, source));
    }
}
