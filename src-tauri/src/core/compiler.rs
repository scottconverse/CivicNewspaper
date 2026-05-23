// core/compiler.rs
use rusqlite::{Connection, params};
use std::error::Error;
use std::fs;
use std::path::Path;
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use pulldown_cmark::{html, Options, Parser};
use super::db::{get_evidence_by_lead, insert_published_post, list_drafts, PublishedPost};

const INDEX_TEMPLATE: &str = include_str!("../../../templates/index.html");
const POST_TEMPLATE: &str = include_str!("../../../templates/post.html");
const STYLES_CSS: &str = include_str!("../../../templates/styles.css");
const PRINT_CSS: &str = include_str!("../../../templates/print.css");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerProfile {
    pub site_title: String,
    pub site_subtitle: String,
    pub about_text: String,
    pub ethics_text: String,
    pub how_we_report_text: String,
}

impl Default for CompilerProfile {
    fn default() -> Self {
        CompilerProfile {
            site_title: "CivicNews Observer".to_string(),
            site_subtitle: "Transparent Local Public Records & Evidence".to_string(),
            about_text: "We report on local government activities using raw public records.".to_string(),
            ethics_text: "Our core ethics: evidence, not rumor. We link every fact to primary documentation.".to_string(),
            how_we_report_text: "We collect agendas, minutes, and documents directly from municipal feeds.".to_string(),
        }
    }
}

pub fn render_markdown(markdown: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(markdown, options).filter(|event| {
        !matches!(event, pulldown_cmark::Event::Html(_) | pulldown_cmark::Event::InlineHtml(_))
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn compile_static_site(
    conn: &Connection,
    output_dir_str: &str,
    profile_json: &str,
) -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new(output_dir_str);
    
    // 1. Create standard output directories
    fs::create_dir_all(output_dir)?;
    let formats = vec!["briefs", "watch", "explainers", "stories", "opinions", "corrections"];
    for fmt in &formats {
        fs::create_dir_all(output_dir.join(fmt))?;
    }

    // 2. Parse Profile Settings
    let profile: CompilerProfile = serde_json::from_str(profile_json).unwrap_or_default();
    let current_year = Utc::now().year().to_string();

    // 3. Copy Stylesheets
    fs::write(output_dir.join("styles.css"), STYLES_CSS)?;
    fs::write(output_dir.join("print.css"), PRINT_CSS)?;

    // 4. Fetch drafts that are published or corrected
    let drafts = list_drafts(conn)?;
    let mut published_drafts = Vec::new();
    for d in drafts {
        if d.status == "published" || d.status == "corrected" {
            published_drafts.push(d);
        }
    }

    let mut article_list_html = String::new();
    let mut corrections_list_html = String::new();

    // RSS Feed Builder
    let mut rss_items = String::new();

    // 5. Compile each published article
    for draft in &published_drafts {
        let draft_id = draft.id.unwrap_or(0);
        let subfolder = match draft.format.as_str() {
            "brief" => "briefs",
            "watch" => "watch",
            "explainer" => "explainers",
            "investigation" => "stories",
            "opinion" => "opinions",
            _ => "stories",
        };

        // Convert Markdown body to HTML
        let raw_html = render_markdown(&draft.content);
        // Replace custom references `href="evidence:123"` with local section anchors `#evidence-123`
        let html_content = raw_html.replace("href=\"evidence:", "href=\"#evidence-");

        // Format Linked Evidence List
        let mut evidence_html = String::new();
        if let Some(lid) = draft.lead_id {
            let items = get_evidence_by_lead(conn, lid)?;
            for item in items {
                let item_id = item.id.unwrap_or(0);
                let source_url = item.url.clone().unwrap_or_else(|| "#".to_string());
                let safe_url = html_escape::encode_safe(&source_url);
                let safe_excerpt = html_escape::encode_safe(&item.excerpt);
                evidence_html.push_str(&format!(
                    "<li id=\"evidence-{}\" style=\"margin-bottom: 0.5rem;\"><strong>[Ref {}]</strong> <a href=\"{}\" target=\"_blank\">Original Document Link</a>: <span style=\"font-style: italic;\">\"{}\"</span></li>\n",
                    item_id, item_id, safe_url, safe_excerpt
                ));
            }
        }
        if evidence_html.is_empty() {
            evidence_html = "<li>No specific evidence items linked to this report.</li>".to_string();
        }

        // Format Correction Banner
        let correction_banner = if draft.status == "corrected" {
            let note = draft.correction_note.clone().unwrap_or_default();
            let safe_note = html_escape::encode_safe(&note);
            format!(
                "<div class=\"correction-banner\" style=\"background-color: #fff3cd; border: 1px solid #ffeeba; color: #856404; padding: 1rem; margin-bottom: 2rem; border-radius: 4px; font-family: var(--font-sans); font-size: 0.95rem;\"><strong>CORRECTION:</strong> {}</div>\n",
                safe_note
            )
        } else {
            String::new()
        };

        let safe_title = html_escape::encode_safe(&draft.title).to_string();
        
        // Generate Post Page
        let mut post_html = POST_TEMPLATE.to_string();
        post_html = post_html.replace("{{SITE_TITLE}}", &profile.site_title);
        post_html = post_html.replace("{{SITE_SUBTITLE}}", &profile.site_subtitle);
        post_html = post_html.replace("{{POST_TITLE}}", &safe_title);
        post_html = post_html.replace("{{POST_DESCRIPTION}}", &safe_title);
        post_html = post_html.replace("{{POST_DATE}}", &draft.updated_at);
        post_html = post_html.replace("{{POST_FORMAT}}", &draft.format);
        post_html = post_html.replace("{{POST_CONTENT}}", &html_content);
        post_html = post_html.replace("{{EVIDENCE_CITATIONS}}", &evidence_html);
        post_html = post_html.replace("{{CORRECTION_BANNER}}", &correction_banner);
        post_html = post_html.replace("{{YEAR}}", &current_year);

        let relative_path = format!("{}/{}.html", subfolder, draft_id);
        let dest_path = output_dir.join(&relative_path);
        fs::write(dest_path, post_html)?;

        // Update database linking
        let published_post = PublishedPost {
            id: None,
            draft_id,
            file_path: relative_path.clone(),
            url: relative_path.clone(),
            published_at: draft.updated_at.clone(),
            correction_history: draft.correction_note.clone().unwrap_or_else(|| "[]".to_string()),
        };
        // Let's insert published post into DB (ignoring if it exists or doing it silently)
        let mut stmt = conn.prepare("SELECT count(*) FROM published_posts WHERE draft_id = ?1")?;
        let exists: i32 = stmt.query_row([draft_id], |row| row.get(0))?;
        if exists == 0 {
            insert_published_post(conn, &published_post)?;
        } else {
            conn.execute(
                "UPDATE published_posts SET published_at = ?1, correction_history = ?2 WHERE draft_id = ?3",
                params![draft.updated_at, published_post.correction_history, draft_id],
            )?;
        }

        // Add to Homepage listing HTML
        article_list_html.push_str(&format!(
            "<article>\n  <h2 class=\"article-title\"><a href=\"{}\">{}</a></h2>\n  <div class=\"article-meta\">\n    <span>{}</span>\n    <span>Format: <span class=\"tag\">{}</span></span>\n  </div>\n</article>\n\n",
            relative_path, safe_title, draft.updated_at, draft.format
        ));

        // Add to RSS feed items
        rss_items.push_str(&format!(
            "    <item>\n      <title>{}</title>\n      <link>{}</link>\n      <guid>{}/{}</guid>\n      <pubDate>{}</pubDate>\n      <description>{}</description>\n    </item>\n",
            safe_title, relative_path, subfolder, draft_id, draft.updated_at, safe_title
        ));

        // Add to Corrections Ledger listing if corrected
        if draft.status == "corrected" {
            let note_str = draft.correction_note.clone().unwrap_or_default();
            let safe_corr_note = html_escape::encode_safe(&note_str);
            corrections_list_html.push_str(&format!(
                "<div style=\"border-bottom: 1px dashed #cccccc; padding: 1.5rem 0;\">\n  <h3 style=\"margin-top: 0;\"><a href=\"{}\">{}</a></h3>\n  <p style=\"font-size: 0.9rem; color: #856404; background-color: #fff3cd; padding: 0.75rem; border-radius: 4px;\"><strong>Correction Notice ({}):</strong> {}</p>\n</div>\n\n",
                relative_path, safe_title, draft.updated_at, safe_corr_note
            ));
        }
    }

    if article_list_html.is_empty() {
        article_list_html = "<p>No observation records published yet.</p>".to_string();
    }
    if corrections_list_html.is_empty() {
        corrections_list_html = "<p>No corrections registered in the public log.</p>".to_string();
    }

    // 6. Build index.html
    let mut index_html = INDEX_TEMPLATE.to_string();
    index_html = index_html.replace("{{SITE_TITLE}}", &profile.site_title);
    index_html = index_html.replace("{{SITE_SUBTITLE}}", &profile.site_subtitle);
    index_html = index_html.replace("{{ARTICLES}}", &article_list_html);
    
    // Sidebar: list of latest 5 posts + profile description
    let mut sidebar_html = format!(
        "<div class=\"sidebar-section\">\n  <h3 class=\"sidebar-title\">About this Site</h3>\n  <p>{}</p>\n</div>\n",
        profile.about_text
    );
    sidebar_html.push_str("<div class=\"sidebar-section\">\n  <h3 class=\"sidebar-title\">Ethics Standards</h3>\n  <p>Every claim published here is strictly bound to public evidence records. We run zero ads.</p>\n</div>");
    index_html = index_html.replace("{{SIDEBAR}}", &sidebar_html);
    index_html = index_html.replace("{{YEAR}}", &current_year);
    fs::write(output_dir.join("index.html"), index_html)?;

    // 7. Build About, Ethics, and How We Report pages
    let compile_info_page = |filename: &str, title: &str, content_md: &str| -> Result<(), Box<dyn Error>> {
        let body_html = render_markdown(content_md);
        let mut page_html = POST_TEMPLATE.to_string();
        page_html = page_html.replace("{{SITE_TITLE}}", &profile.site_title);
        page_html = page_html.replace("{{SITE_SUBTITLE}}", &profile.site_subtitle);
        page_html = page_html.replace("{{POST_TITLE}}", title);
        page_html = page_html.replace("{{POST_DESCRIPTION}}", title);
        page_html = page_html.replace("{{POST_DATE}}", &Utc::now().to_rfc3339());
        page_html = page_html.replace("{{POST_FORMAT}}", "meta");
        page_html = page_html.replace("{{POST_CONTENT}}", &body_html);
        page_html = page_html.replace("{{EVIDENCE_CITATIONS}}", "<!-- No citations for info pages -->");
        page_html = page_html.replace("{{CORRECTION_BANNER}}", "");
        page_html = page_html.replace("{{YEAR}}", &current_year);
        fs::write(output_dir.join(filename), page_html)?;
        Ok(())
    };

    compile_info_page("about.html", "About CivicNews Observer", &profile.about_text)?;
    compile_info_page("ethics.html", "Reporting Ethics & Standards", &profile.ethics_text)?;
    compile_info_page("how-we-report.html", "How We Report", &profile.how_we_report_text)?;

    // 8. Build corrections.html ledger
    let mut corrections_html = POST_TEMPLATE.to_string();
    corrections_html = corrections_html.replace("{{SITE_TITLE}}", &profile.site_title);
    corrections_html = corrections_html.replace("{{SITE_SUBTITLE}}", &profile.site_subtitle);
    corrections_html = corrections_html.replace("{{POST_TITLE}}", "Public Corrections Ledger");
    corrections_html = corrections_html.replace("{{POST_DESCRIPTION}}", "Public Corrections Ledger");
    corrections_html = corrections_html.replace("{{POST_DATE}}", &Utc::now().to_rfc3339());
    corrections_html = corrections_html.replace("{{POST_FORMAT}}", "ledger");
    corrections_html = corrections_html.replace("{{POST_CONTENT}}", &corrections_list_html);
    corrections_html = corrections_html.replace("{{EVIDENCE_CITATIONS}}", "");
    corrections_html = corrections_html.replace("{{CORRECTION_BANNER}}", "");
    corrections_html = corrections_html.replace("{{YEAR}}", &current_year);
    fs::write(output_dir.join("corrections.html"), corrections_html)?;

    // 9. Build RSS feed.xml
    let rss_feed = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<rss version=\"2.0\">\n  <channel>\n    <title>{}</title>\n    <link>index.html</link>\n    <description>{}</description>\n    <language>en-us</language>\n    <pubDate>{}</pubDate>\n    <lastBuildDate>{}</lastBuildDate>\n{}\n  </channel>\n</rss>\n",
        profile.site_title, profile.site_subtitle, Utc::now().to_rfc2822(), Utc::now().to_rfc2822(), rss_items
    );
    fs::write(output_dir.join("feed.xml"), rss_feed)?;

    Ok(())
}
