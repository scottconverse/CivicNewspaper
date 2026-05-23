// core/discovery.rs
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use regex::Regex;
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

        if !title.is_empty() && (real_url.starts_with("http://") || real_url.starts_with("https://")) {
            results.push((title, real_url));
        }
    }

    Ok(results)
}

pub async fn search_duckduckgo(client: &Client, query: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut base_url = reqwest::Url::parse("https://html.duckduckgo.com/html/")?;
    base_url.query_pairs_mut().append_pair("q", query);
    let search_url = base_url.to_string();

    let res = client.get(&search_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("DuckDuckGo HTML search error: {}", res.status()).into());
    }

    let html = res.text().await?;
    parse_duckduckgo_html(&html)
}


pub async fn discover_all_sources(city: &str, state: &str) -> Result<Vec<DiscoveredSourceCategory>, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    let targets = vec![
        ("Municipal Website", "primary_record", format!("{} {} government website", city, state)),
        ("City Council Agenda", "primary_record", format!("{} {} city council agenda", city, state)),
        ("Public & Legal Notices", "primary_record", format!("{} {} public notices", city, state)),
        ("School Board Agenda", "primary_record", format!("{} {} school district board agenda", city, state)),
        ("Police Department Facebook", "official_comm", format!("{} {} police department Facebook", city, state)),
        ("Local Reddit Community", "community_signal", format!("{} {} Reddit", city, state)),
        ("Local Newspaper Headlines", "media_lead", format!("{} {} local newspaper", city, state)),
        ("Chamber of Commerce", "community_signal", format!("{} {} chamber of commerce", city, state)),
        ("Library Events", "community_signal", format!("{} {} library events", city, state)),
    ];

    let mut categories = Vec::new();

    for (cat_name, src_type, query) in targets {
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

        categories.push(DiscoveredSourceCategory {
            category_name: cat_name.to_string(),
            r#type: src_type.to_string(),
            candidates,
        });
    }

    Ok(categories)
}
