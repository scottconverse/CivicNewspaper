use crate::core::compiler;

pub fn normalize_public_text(value: &str) -> String {
    let once = html_escape::decode_html_entities(value).to_string();
    let twice = html_escape::decode_html_entities(&once).to_string();
    compiler::repair_common_mojibake(&twice)
        .replace(['\u{00a0}', '\u{2022}'], " ")
        .replace("-->", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub fn clean_multiline_public_text(value: &str) -> String {
    value
        .lines()
        .map(normalize_public_text)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn public_quality_issue(values: &[&str]) -> Option<&'static str> {
    let raw = values.join(" ");
    let normalized = normalize_public_text(&raw);
    let lower = normalized.to_lowercase();

    if raw.contains("&#")
        || raw.contains("&amp;#")
        || raw.contains("-->")
        || raw.contains('\u{fffd}')
        || raw.contains('â')
        || raw.contains('Ã')
        || lower.contains("<script")
        || lower.contains("<nav")
    {
        return Some("Text contains encoded HTML, mojibake, or page-markup debris.");
    }

    if looks_like_navigation_or_index(&lower) {
        return Some("Text appears to summarize source navigation, category lists, or an index page instead of one reportable item.");
    }

    if looks_like_multi_item_event_listing(&lower) {
        return Some("Text appears to combine multiple calendar or event-listing items; split and verify one specific civic item before drafting.");
    }

    None
}

pub fn looks_like_multi_item_event_listing(lower: &str) -> bool {
    let month_hits = [
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
    ]
    .iter()
    .map(|month| lower.matches(month).count())
    .sum::<usize>();
    let event_hits = [
        "concert",
        "festival",
        "artwalk",
        "farmers market",
        "events",
        "things to do",
        "weekend",
        "calendar",
        "workshop",
        "class",
        "library closed",
        "free fitness",
        "symphony",
        "storytime",
        "loteria mexicana",
        "loteria",
        "bilingual",
        "independence",
    ]
    .iter()
    .map(|term| lower.matches(*term).count())
    .sum::<usize>();
    let time_hits = regex::Regex::new(r"(?i)\b\d{1,2}\s*(am|pm)\b")
        .expect("valid event time regex")
        .find_iter(lower)
        .count();
    let date_range_hits = lower.matches(" - ").count() + lower.matches(" to ").count();
    (lower.len() > 220 && month_hits >= 3 && event_hits >= 3)
        || (lower.len() > 360 && month_hits >= 1 && event_hits >= 4)
        || (lower.len() > 360 && time_hits >= 4 && event_hits >= 3)
        || (lower.len() > 420 && date_range_hits >= 4 && event_hits >= 3)
}

fn looks_like_navigation_or_index(lower: &str) -> bool {
    let nav_hits = [
        "skip to main content",
        "search",
        "menu",
        "departments",
        "services",
        "quick links",
        "about the city",
        "contact us",
    ]
    .iter()
    .filter(|term| lower.contains(**term))
    .count();
    let short_link_like_lines = lower
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && trimmed.len() <= 48
                && trimmed.split_whitespace().count() <= 5
                && !trimmed.ends_with('.')
                && !trimmed.ends_with('?')
                && !trimmed.ends_with('!')
        })
        .count();
    nav_hits >= 3 || (lower.len() > 300 && nav_hits >= 2 && short_link_like_lines >= 8)
}

pub fn append_quality_note(existing: Option<String>, reason: &str) -> String {
    let gate_note = format!("Quality gate: {reason}");
    match existing.map(|value| value.trim().to_string()) {
        Some(value) if !value.is_empty() && !value.contains(&gate_note) => {
            format!("{}. {gate_note}", value.trim_end_matches('.'))
        }
        Some(value) if !value.is_empty() => value,
        _ => gate_note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repairs_entities_and_mojibake_markers() {
        let raw = "Friday, July 3 \u{00e2}\u{20ac}\u{00a2} 6 pm &#8211; 10 pm -->";
        let cleaned = normalize_public_text(raw);

        assert_eq!(cleaned, "Friday, July 3 - 6 pm - 10 pm");
    }

    #[test]
    fn detects_broad_encoded_calendar_rollups() {
        let raw = "Independence Weekend Free Concert Friday, July 3 \u{00e2}\u{20ac}\u{00a2} 6 pm &#8211; 10 pm. LIBRARY CLOSED Friday, July 3 \u{00e2}\u{20ac}\u{00a2} 6 pm &#8211; 10 pm. Free Fitness in the Park Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 8 am &#8211; 9 am. July 4th Symphony Concert Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 11 am &#8211; 12 pm. Independence Weekend Festival Saturday, July 4 \u{00e2}\u{20ac}\u{00a2} 4 pm &#8211; 10 pm.";

        assert!(public_quality_issue(&[raw]).unwrap().contains("encoded"));
        let normalized = normalize_public_text(raw).to_lowercase();
        assert!(looks_like_multi_item_event_listing(&normalized));
    }
}
