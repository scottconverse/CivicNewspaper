use std::collections::HashSet;

use super::db::EvidenceItem;

const STOPWORDS: &[&str] = &[
    "about",
    "after",
    "again",
    "against",
    "also",
    "before",
    "being",
    "between",
    "could",
    "during",
    "editor",
    "event",
    "events",
    "first",
    "from",
    "have",
    "into",
    "local",
    "longmont",
    "more",
    "original",
    "public",
    "reader",
    "residents",
    "review",
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
    "were",
    "where",
    "which",
    "while",
    "will",
    "with",
    "would",
];

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
        lead_token_count.min(2).max(1)
    } else {
        ((lead_token_count + 2) / 3).clamp(2, 4)
    }
}

pub fn evidence_matches_topic(lead_text: &str, evidence_text: &str) -> bool {
    let lead_tokens = grounding_tokens(lead_text);
    if lead_tokens.is_empty() {
        return false;
    }
    let evidence_tokens = grounding_tokens(evidence_text);
    let overlap = lead_tokens.intersection(&evidence_tokens).count();
    overlap >= required_overlap(lead_tokens.len())
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
    let paragraph_tokens = grounding_tokens(paragraph);
    if paragraph_tokens.len() < 4 {
        return true;
    }
    let source_tokens = grounding_tokens(source_text);
    let overlap = paragraph_tokens.intersection(&source_tokens).count();
    let required = ((paragraph_tokens.len() + 3) / 4).clamp(2, 6);
    overlap >= required
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
    fn cited_paragraph_with_many_unsupported_claims_does_not_align() {
        let paragraph = "Council members will vote on a roof repair project, increase capacity by 40 percent, and spend ten million dollars over ten years while residents attend Road Pony.";
        let source = "Summer Concert Series: Road Pony 7pm - 8:30pm Longmont Museum Creative District Event.";

        assert!(!paragraph_is_source_aligned(paragraph, source));
    }
}
