// core/scraper.rs
use super::db::{
    get_evidence_by_hash, insert_evidence_item, update_source_status, DbConn, EvidenceItem, Source,
};
use chrono::Utc;
use feed_rs::parser;
use reqwest::Client;
use serde_json;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::time::sleep;

// SSRF defense-in-depth: a source URL can arrive from manual entry OR from the
// discovery -> auto-import chain (third-party search results the user never
// typed). Without validation the scraper would fetch and store the body of
// `http://127.0.0.1:11434` (local Ollama), `http://169.254.169.254` (cloud
// metadata), or internal-LAN hosts. `validate_source_url` is the cheap,
// network-free gate used when a source is *stored* (rejects bad schemes and
// blocked IP literals); `validate_source_url_resolved` additionally resolves
// DNS and is used at *scrape* time, where the network is in play anyway and DNS
// may have changed since the source was added.
pub fn validate_source_url(url: &str) -> Result<reqwest::Url, String> {
    let parsed = reqwest::Url::parse(url.trim()).map_err(|e| format!("Invalid URL: {}", e))?;

    match parsed.scheme() {
        "http" | "https" => {}
        other => {
            return Err(format!(
                "Unsupported URL scheme '{}': only http and https sources are allowed",
                other
            ))
        }
    }

    let host = parsed
        .host_str()
        .ok_or_else(|| "URL has no host".to_string())?;

    // If the host is a literal IP, check it now without touching DNS.
    // `host_str()` keeps the brackets on IPv6 literals (`[::1]`), so strip them
    // before parsing.
    if let Ok(ip) = host_as_ip(host) {
        if is_blocked_ip(&ip) {
            return Err(format!(
                "URL host {} is a blocked address: loopback, private, and link-local destinations are not allowed",
                ip
            ));
        }
    }

    Ok(parsed)
}

fn host_as_ip(host: &str) -> Result<IpAddr, std::net::AddrParseError> {
    host.trim_start_matches('[')
        .trim_end_matches(']')
        .parse::<IpAddr>()
}

pub fn validate_source_url_resolved(url: &str) -> Result<(), String> {
    let parsed = validate_source_url(url)?;
    let host = parsed
        .host_str()
        .ok_or_else(|| "URL has no host".to_string())?;

    // Literal IPs were already vetted by validate_source_url; only domains need
    // resolution.
    if host_as_ip(host).is_ok() {
        return Ok(());
    }

    let port = parsed.port_or_known_default().unwrap_or(80);
    let addrs = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("Could not resolve host '{}': {}", host, e))?;

    let mut resolved_any = false;
    for addr in addrs {
        resolved_any = true;
        if is_blocked_ip(&addr.ip()) {
            return Err(format!(
                "URL host '{}' resolves to a blocked address ({}): loopback, private, and link-local destinations are not allowed",
                host, addr.ip()
            ));
        }
    }
    if !resolved_any {
        return Err(format!("Host '{}' did not resolve to any address", host));
    }
    Ok(())
}

fn is_blocked_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local() // 169.254.0.0/16 — covers the 169.254.169.254 metadata IP
                || v4.is_unspecified()
                || v4.is_broadcast()
                // Carrier-grade NAT 100.64.0.0/10
                || (v4.octets()[0] == 100 && (v4.octets()[1] & 0xc0) == 64)
        }
        IpAddr::V6(v6) => {
            // Unwrap IPv4-mapped addresses (::ffff:a.b.c.d) and check as IPv4.
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_blocked_ip(&IpAddr::V4(v4));
            }
            v6.is_loopback()
                || v6.is_unspecified()
                || (v6.segments()[0] & 0xffc0) == 0xfe80 // link-local fe80::/10
                || (v6.segments()[0] & 0xfe00) == 0xfc00 // unique-local fc00::/7
        }
    }
}

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
        // Disable reqwest's automatic redirect following. The default policy
        // follows up to 10 redirects, but it would only have validated the
        // *original* URL's resolved IP — a public feed that 302s to
        // `http://169.254.169.254/` (cloud metadata) or an internal-LAN host
        // would be followed straight past the SSRF gate. We follow redirects
        // manually in `fetch_validated`, re-running the resolved-IP check on
        // every hop.
        .redirect(reqwest::redirect::Policy::none())
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

// Resolve a redirect's `Location` (which may be relative) against the URL that
// produced it. Kept network-free and pure so the redirect-handling logic is
// unit-testable without a live HTTP server.
fn resolve_redirect_target(current: &str, location: &str) -> Result<String, Box<dyn Error>> {
    let base = reqwest::Url::parse(current)?;
    let next = base.join(location)?;
    Ok(next.to_string())
}

// Fetch a URL, following redirects manually so that *every* hop is re-validated
// with DNS resolution before we connect to it. The client must be built with
// `redirect::Policy::none()`; otherwise reqwest would follow redirects itself,
// having validated only the original URL — a redirect-SSRF gap (a public feed
// 302-ing to an internal/metadata address).
async fn fetch_validated(
    client: &Client,
    initial_url: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    const MAX_REDIRECTS: usize = 10;
    let mut current = initial_url.to_string();

    for _ in 0..=MAX_REDIRECTS {
        // Re-validate (with DNS resolution) before each fetch. A stored URL may
        // predate this check, a host's DNS can change between add and scrape,
        // and a redirect target is attacker-influenced. Run the blocking
        // resolver off the async runtime.
        let url_for_check = current.clone();
        tokio::task::spawn_blocking(move || validate_source_url_resolved(&url_for_check)).await??;

        let response = client.get(&current).send().await?;

        if response.status().is_redirection() {
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .ok_or("Redirect response missing a valid Location header")?;
            current = resolve_redirect_target(&current, location)?;
            continue;
        }

        return Ok(response);
    }

    Err(format!(
        "Too many redirects (>{}) while fetching {}",
        MAX_REDIRECTS, initial_url
    )
    .into())
}

async fn scrape_source(
    client: &Client,
    db: &DbConn,
    source: &Source,
) -> Result<(), Box<dyn Error>> {
    // Fetch through a manual redirect loop that re-validates (with DNS
    // resolution) on every hop. See `fetch_validated` for why automatic
    // redirect following is unsafe here.
    let response = fetch_validated(client, &source.url).await?;
    if !response.status().is_success() {
        return Err(format!("HTTP error status: {}", response.status()).into());
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|c| c.to_str().ok())
        .unwrap_or("")
        .to_string();

    let body_bytes = response.bytes().await?;

    // Detect if RSS feed or raw HTML
    let is_feed = content_type.contains("xml")
        || content_type.contains("rss")
        || content_type.contains("atom")
        || body_bytes.starts_with(b"<?xml")
        || body_bytes.starts_with(b"<rss")
        || body_bytes.starts_with(b"<feed");

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
                let entities_json =
                    serde_json::to_string(&entities).unwrap_or_else(|_| "[]".to_string());

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
                    let entities_json =
                        serde_json::to_string(&entities).unwrap_or_else(|_| "[]".to_string());

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
    let re_script_style =
        regex::Regex::new(r"(?s)<(script|style)[^>]*>.*?</(script|style)>").unwrap();
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
        if current_chunk.len() + p_trimmed.len() > chunk_size && !current_chunk.is_empty() {
            chunks.push(current_chunk.clone());
            current_chunk.clear();
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

#[cfg(test)]
mod url_validation_tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn rejects_non_http_schemes() {
        for url in [
            "file:///etc/passwd",
            "ftp://example.com/feed.xml",
            "gopher://example.com",
            "data:text/html,<script>alert(1)</script>",
            "javascript:alert(1)",
        ] {
            let res = validate_source_url(url);
            assert!(res.is_err(), "scheme should be rejected: {}", url);
        }
    }

    #[test]
    fn rejects_blocked_ip_literals() {
        // loopback, RFC1918, link-local (incl. metadata), unspecified, CGNAT,
        // and IPv6 loopback / IPv4-mapped loopback.
        for url in [
            "http://127.0.0.1/feed",
            "http://127.0.0.1:11434/api/tags",
            "http://10.0.0.5/x",
            "http://172.16.4.2/x",
            "http://192.168.1.1/x",
            "http://169.254.169.254/latest/meta-data/",
            "http://0.0.0.0/x",
            "http://100.64.0.1/x",
            "http://[::1]/x",
            "http://[::ffff:127.0.0.1]/x",
        ] {
            let res = validate_source_url(url);
            assert!(res.is_err(), "blocked IP literal should be rejected: {}", url);
        }
    }

    #[test]
    fn accepts_public_http_and_https() {
        // Public hostnames pass the network-free storage check (no DNS performed
        // here); a literal public IP also passes.
        for url in [
            "http://example.com/feed.xml",
            "https://www.brightoncolorado.gov/rss",
            "https://203.0.113.10/feed", // TEST-NET-3, a routable-looking literal
        ] {
            assert!(validate_source_url(url).is_ok(), "should be accepted: {}", url);
        }
    }

    #[test]
    fn resolved_check_rejects_blocked_literal_without_dns() {
        // The resolving variant must still reject IP literals (it short-circuits
        // before DNS for literals) — no network required for this assertion.
        assert!(validate_source_url_resolved("http://127.0.0.1/feed").is_err());
        assert!(validate_source_url_resolved("http://169.254.169.254/").is_err());
    }

    #[test]
    fn is_blocked_ip_classifies_ranges() {
        let blocked = [
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(10, 1, 2, 3)),
            IpAddr::V4(Ipv4Addr::new(172, 31, 255, 255)),
            IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254)),
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1)),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::UNSPECIFIED),
            IpAddr::V6("fe80::1".parse().unwrap()),
            IpAddr::V6("fc00::1".parse().unwrap()),
        ];
        for ip in blocked {
            assert!(is_blocked_ip(&ip), "{} should be blocked", ip);
        }

        let allowed = [
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            IpAddr::V4(Ipv4Addr::new(203, 0, 113, 10)),
            IpAddr::V6("2606:4700:4700::1111".parse().unwrap()),
        ];
        for ip in allowed {
            assert!(!is_blocked_ip(&ip), "{} should be allowed", ip);
        }
    }

    #[test]
    fn resolve_redirect_target_handles_absolute_and_relative() {
        // Absolute Location replaces the URL entirely.
        assert_eq!(
            resolve_redirect_target("http://example.com/feed", "https://other.org/x").unwrap(),
            "https://other.org/x"
        );
        // Root-relative Location is joined against the origin.
        assert_eq!(
            resolve_redirect_target("http://example.com/a/b", "/c").unwrap(),
            "http://example.com/c"
        );
        // Path-relative Location is joined against the current directory.
        assert_eq!(
            resolve_redirect_target("http://example.com/a/b", "c").unwrap(),
            "http://example.com/a/c"
        );
    }

    #[test]
    fn redirect_target_to_internal_address_is_rejected_by_validator() {
        // The redirect loop validates every hop via validate_source_url_resolved.
        // A public feed that redirects to the cloud-metadata IP (literal, so no
        // DNS needed) must be rejected — this is the redirect-SSRF gap the manual
        // redirect loop closes.
        let next =
            resolve_redirect_target("http://example.com/feed", "http://169.254.169.254/latest/")
                .unwrap();
        assert_eq!(next, "http://169.254.169.254/latest/");
        assert!(
            validate_source_url_resolved(&next).is_err(),
            "redirect to metadata IP must be rejected"
        );
    }
}
