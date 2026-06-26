// core/discovery.rs
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSource {
    pub name: String,
    pub url: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSourceCategory {
    pub category_name: String,
    pub r#type: String,
    pub candidates: Vec<DiscoveredSource>,
}

pub fn parse_duckduckgo_html(html: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    // DuckDuckGo HTML result anchors look like:
    // <a class="result__a" href="//duckduckgo.com/l/?uddg=URL...">Title</a>
    let re = Regex::new(r#"(?s)<a\s+class="result__a"\s+href="([^"]+)"[^>]*>(.*?)</a>"#)?;

    let mut results = Vec::new();
    let title_re = Regex::new(r"<[^>]*>")?;
    for cap in re.captures_iter(html) {
        let href = &cap[1];
        let raw_title = &cap[2];

        // Strip any nested HTML tags from the title
        let title = title_re.replace_all(raw_title, "").trim().to_string();

        let mut real_url = href.to_string();
        if href.contains("uddg=") {
            let full_url = if href.starts_with("//") {
                format!("https:{}", href)
            } else if href.starts_with("/") {
                format!("https://duckduckgo.com{}", href)
            } else {
                href.to_string()
            };
            if let Ok(parsed) = reqwest::Url::parse(&full_url) {
                for (key, val) in parsed.query_pairs() {
                    if key == "uddg" {
                        real_url = val.into_owned();
                        break;
                    }
                }
            }
        } else if href.starts_with("//") {
            real_url = format!("https:{}", href);
        } else if href.starts_with("/") {
            real_url = format!("https://duckduckgo.com{}", href);
        }

        if !title.is_empty()
            && (real_url.starts_with("http://") || real_url.starts_with("https://"))
        {
            results.push((title, real_url));
        }
    }

    Ok(results)
}

fn is_duckduckgo_challenge_page(html: &str) -> bool {
    html.contains("anomaly-modal")
        || html.contains("Unfortunately, bots use DuckDuckGo too")
        || html.contains("challenge-form")
}

fn slugify_city(city: &str, separator: &str) -> String {
    let mut parts = Vec::new();
    let mut current = String::new();

    for ch in city.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            parts.push(current);
            current = String::new();
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts.join(separator)
}

fn search_url(query: &str) -> String {
    let mut url = reqwest::Url::parse("https://www.google.com/search").expect("static URL is valid");
    url.query_pairs_mut().append_pair("q", query);
    url.to_string()
}

fn fallback_candidates_for_category(
    category_name: &str,
    src_type: &str,
    city: &str,
    state: &str,
) -> Vec<DiscoveredSource> {
    let city_dash = slugify_city(city, "-");
    let city_compact = slugify_city(city, "");
    let state_lower = state.trim().to_ascii_lowercase();
    let city_label = city.trim();
    let state_label = state.trim().to_ascii_uppercase();

    let mut urls: Vec<(String, String)> = match category_name {
        "Municipal Website" => vec![
            (
                format!("{city_label} official website"),
                format!("https://www.{city_compact}{state_lower}.gov/"),
            ),
            (
                format!("{city_label} city website"),
                format!("https://www.{city_dash}-{state_lower}.gov/"),
            ),
            (
                format!("{city_label} official website search"),
                search_url(&format!("{city_label} {state_label} official city website")),
            ),
        ],
        "City Council Agenda" => vec![
            (
                format!("{city_label} council agendas"),
                format!("https://www.{city_compact}{state_lower}.gov/agendas"),
            ),
            (
                format!("{city_label} agenda center"),
                format!("https://www.{city_compact}{state_lower}.gov/agendacenter"),
            ),
            (
                format!("{city_label} city council agenda search"),
                search_url(&format!("{city_label} {state_label} city council agenda")),
            ),
        ],
        "Public & Legal Notices" => vec![
            (
                format!("{city_label} public notices"),
                format!("https://www.{city_compact}{state_lower}.gov/public-notices"),
            ),
            (
                format!("{city_label} legal notices search"),
                search_url(&format!("{city_label} {state_label} public legal notices")),
            ),
        ],
        "School Board Agenda" => vec![
            (
                format!("{city_label} school board agendas"),
                search_url(&format!(
                    "{city_label} {state_label} school district board agenda"
                )),
            ),
            (
                format!("{city_label} board of education meetings"),
                search_url(&format!(
                    "{city_label} {state_label} board of education meetings"
                )),
            ),
        ],
        "Police Department Facebook" => vec![
            (
                format!("{city_label} police department"),
                format!("https://www.facebook.com/search/pages/?q={city_dash}%20police%20department%20{state_lower}"),
            ),
            (
                format!("{city_label} police department search"),
                search_url(&format!(
                    "{city_label} {state_label} police department Facebook"
                )),
            ),
        ],
        "Local Reddit Community" => vec![
            (
                format!("{city_label} Reddit search"),
                format!("https://www.reddit.com/search/?q={city_dash}%20{state_lower}"),
            ),
            (
                format!("r/{city_compact}"),
                format!("https://www.reddit.com/r/{city_compact}/"),
            ),
        ],
        "Local Newspaper Headlines" => vec![
            (
                format!("{city_label} local news search"),
                search_url(&format!("{city_label} {state_label} local newspaper")),
            ),
            (
                format!("{city_label} news"),
                search_url(&format!("{city_label} {state_label} news")),
            ),
        ],
        "Chamber of Commerce" => vec![
            (
                format!("{city_label} chamber of commerce"),
                format!("https://www.{city_compact}chamber.org/"),
            ),
            (
                format!("{city_label} chamber search"),
                search_url(&format!("{city_label} {state_label} chamber of commerce")),
            ),
        ],
        "Library Events" => vec![
            (
                format!("{city_label} library events"),
                format!("https://www.{city_compact}library.org/events"),
            ),
            (
                format!("{city_label} library calendar search"),
                search_url(&format!("{city_label} {state_label} library events calendar")),
            ),
        ],
        _ => vec![(
            format!("{city_label} {category_name} search"),
            search_url(&format!("{city_label} {state_label} {category_name}")),
        )],
    };

    urls.dedup_by(|a, b| a.1 == b.1);
    urls.into_iter()
        .map(|(name, url)| DiscoveredSource {
            name,
            url,
            r#type: src_type.to_string(),
        })
        .collect()
}

#[cfg(test)]
fn fallback_discovery_categories(city: &str, state: &str) -> Vec<DiscoveredSourceCategory> {
    discovery_targets(city, state)
        .into_iter()
        .map(|(cat_name, src_type, _query)| DiscoveredSourceCategory {
            category_name: cat_name.to_string(),
            r#type: src_type.to_string(),
            candidates: fallback_candidates_for_category(cat_name, src_type, city, state),
        })
        .collect()
}

pub async fn search_duckduckgo(
    client: &Client,
    query: &str,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut base_url = reqwest::Url::parse("https://html.duckduckgo.com/html/")?;
    base_url.query_pairs_mut().append_pair("q", query);
    let search_url = base_url.to_string();

    let res = client.get(&search_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("Source search returned an error: {}", res.status()).into());
    }

    let html = res.text().await?;

    if is_duckduckgo_challenge_page(&html) {
        return Err("Search engine asked for human verification".into());
    }

    parse_duckduckgo_html(&html)
}

fn discovery_targets(city: &str, state: &str) -> Vec<(&'static str, &'static str, String)> {
    vec![
        (
            "Municipal Website",
            "primary_record",
            format!("{} {} government website", city, state),
        ),
        (
            "City Council Agenda",
            "primary_record",
            format!("{} {} city council agenda", city, state),
        ),
        (
            "Public & Legal Notices",
            "primary_record",
            format!("{} {} public notices", city, state),
        ),
        (
            "School Board Agenda",
            "primary_record",
            format!("{} {} school district board agenda", city, state),
        ),
        (
            "Police Department Facebook",
            "official_comm",
            format!("{} {} police department Facebook", city, state),
        ),
        (
            "Local Reddit Community",
            "community_signal",
            format!("{} {} Reddit", city, state),
        ),
        (
            "Local Newspaper Headlines",
            "media_lead",
            format!("{} {} local newspaper", city, state),
        ),
        (
            "Chamber of Commerce",
            "community_signal",
            format!("{} {} chamber of commerce", city, state),
        ),
        (
            "Library Events",
            "community_signal",
            format!("{} {} library events", city, state),
        ),
    ]
}

pub async fn discover_all_sources(
    city: &str,
    state: &str,
) -> Result<Vec<DiscoveredSourceCategory>, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    let mut categories = Vec::new();

    for (cat_name, src_type, query) in discovery_targets(city, state) {
        // Politeness delay of 500ms between requests to prevent rate-limiting
        sleep(Duration::from_millis(500)).await;

        let mut candidates = Vec::new();
        match search_duckduckgo(&client, &query).await {
            Ok(results) => {
                for (title, url) in results.into_iter().take(3) {
                    candidates.push(DiscoveredSource {
                        name: title,
                        url,
                        r#type: src_type.to_string(),
                    });
                }
            }
            Err(e) => {
                eprintln!("Discovery search failed for query '{}': {}", query, e);
            }
        }

        if candidates.is_empty() {
            candidates = fallback_candidates_for_category(cat_name, src_type, city, state);
        }

        categories.push(DiscoveredSourceCategory {
            category_name: cat_name.to_string(),
            r#type: src_type.to_string(),
            candidates,
        });
    }

    Ok(categories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_duckduckgo_challenge_page() {
        let html = r#"
            <form id="challenge-form">
              <div class="anomaly-modal__title">
                Unfortunately, bots use DuckDuckGo too.
              </div>
            </form>
        "#;

        assert!(is_duckduckgo_challenge_page(html));
    }

    #[test]
    fn fallback_discovery_returns_actionable_civic_candidates() {
        let categories = fallback_discovery_categories("Brighton", "CO");

        assert_eq!(categories.len(), 9);
        assert!(categories.iter().all(|category| !category.candidates.is_empty()));

        let council = categories
            .iter()
            .find(|category| category.category_name == "City Council Agenda")
            .expect("city council category");
        assert!(council
            .candidates
            .iter()
            .any(|candidate| candidate.url.contains("brightonco.gov/agendas")));

        let newspaper = categories
            .iter()
            .find(|category| category.category_name == "Local Newspaper Headlines")
            .expect("newspaper category");
        assert!(newspaper
            .candidates
            .iter()
            .any(|candidate| candidate.url.starts_with("https://www.google.com/search")));
    }
}
