// core/compiler.rs
use super::db::{
    get_evidence_by_lead, insert_publish_run, insert_published_post, list_drafts, PublishRun,
    PublishedPost,
};
use chrono::{Datelike, Utc};
use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const INDEX_TEMPLATE: &str = include_str!("../../../templates/index.html");
const POST_TEMPLATE: &str = include_str!("../../../templates/post.html");
const STYLES_CSS: &str = include_str!("../../../templates/styles.css");
const PRINT_CSS: &str = include_str!("../../../templates/print.css");

const EVIDENCE_SECTION_TEMPLATE: &str = "<div class=\"citation-list\">\n                <h3 style=\"margin-top: 0; font-family: var(--font-sans); font-size: 0.9rem; text-transform: uppercase; color: var(--accent-color);\">Sources & Notes</h3>\n                <p style=\"margin-bottom: 0.8rem; font-size: 0.85rem; color: var(--muted-color);\">Source links attached by the editor:</p>\n                <ol style=\"margin: 0; padding-left: 1.2rem; font-size: 0.9rem; line-height: 1.5;\">\n                    {{EVIDENCE_CITATIONS}}\n                </ol>\n            </div>";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerProfile {
    pub site_title: String,
    pub site_subtitle: String,
    pub about_text: String,
    pub ethics_text: String,
    pub how_we_report_text: String,
    #[serde(default = "default_organization_type")]
    pub organization_type: String,
    #[serde(default)]
    pub footer_text: String,
    #[serde(default)]
    pub logo_url: String,
    #[serde(default = "default_accent_color")]
    pub accent_color: String,
    #[serde(default = "default_layout_style")]
    pub layout_style: String,
    #[serde(default = "default_first_amendment_advisor_enabled")]
    pub first_amendment_advisor_enabled: bool,
}

fn default_organization_type() -> String {
    "single_person".to_string()
}
fn default_accent_color() -> String {
    "#5a1818".to_string()
}
fn default_layout_style() -> String {
    "classic".to_string()
}
fn default_first_amendment_advisor_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledArticle {
    pub title: String,
    pub format: String,
    pub relative_path: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileStaticSiteResult {
    pub issue_id: String,
    pub output_dir: String,
    pub generated_at: String,
    pub provider: String,
    pub published_url: Option<String>,
    pub deployment_id: Option<String>,
    pub article_count: usize,
    pub skipped_count: usize,
    pub files_written: usize,
    pub generated_files: Vec<String>,
    pub index_path: String,
    pub rss_path: String,
    pub newsletter_path: String,
    pub substack_path: String,
    pub share_package_path: String,
    pub facebook_post_path: String,
    pub subreddit_post_path: String,
    pub nextdoor_post_path: String,
    pub short_link_blurb_path: String,
    pub manifest_path: String,
    pub zip_path: String,
    pub articles: Vec<CompiledArticle>,
}

impl Default for CompilerProfile {
    fn default() -> Self {
        CompilerProfile {
            site_title: "My Local Publication".to_string(),
            site_subtitle: "Local news and community information.".to_string(),
            about_text: "A locally edited publication for this community.".to_string(),
            ethics_text:
                "Editorial standards are set by the publisher. Corrections are published when needed."
                    .to_string(),
            how_we_report_text:
                "Stories, sources, and publication decisions are reviewed by the editor before publication."
                    .to_string(),
            organization_type: default_organization_type(),
            footer_text: String::new(),
            logo_url: String::new(),
            accent_color: default_accent_color(),
            layout_style: default_layout_style(),
            first_amendment_advisor_enabled: default_first_amendment_advisor_enabled(),
        }
    }
}

fn footer_notice(profile: &CompilerProfile) -> String {
    let text = profile.footer_text.trim();
    if text.is_empty() {
        String::new()
    } else {
        format!("<p>{}</p>", html_escape::encode_safe(text))
    }
}

fn layout_class(profile: &CompilerProfile) -> &'static str {
    match profile.layout_style.trim() {
        "compact" => "layout-compact",
        "wide" => "layout-wide",
        "modern" => "layout-modern",
        _ => "layout-classic",
    }
}

fn custom_style(profile: &CompilerProfile) -> String {
    let color = profile.accent_color.trim();
    if regex::Regex::new(r"^#[0-9a-fA-F]{6}$")
        .map(|re| re.is_match(color))
        .unwrap_or(false)
    {
        format!("<style>:root {{ --accent-color: {}; }}</style>", color)
    } else {
        String::new()
    }
}

fn logo_html(profile: &CompilerProfile) -> String {
    let url = profile.logo_url.trim();
    if url.is_empty() || !is_safe_logo_src(url) {
        return String::new();
    }
    format!(
        "<img class=\"site-logo\" src=\"{}\" alt=\"{} logo\">",
        html_escape::encode_safe(url),
        html_escape::encode_safe(profile.site_title.trim())
    )
}

fn write_site_file(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
    files_written: &mut usize,
) -> Result<(), Box<dyn Error>> {
    fs::write(path, contents)?;
    *files_written += 1;
    Ok(())
}

pub(crate) fn repair_common_mojibake(text: &str) -> String {
    let mut repaired = text.to_string();
    for (bad, good) in [
        ("Ã¢â‚¬â„¢", "'"),
        ("Ã¢â‚¬Ëœ", "'"),
        ("Ã¢â‚¬Å“", "\""),
        ("Ã¢â‚¬Â", "\""),
        ("Ã¢â‚¬â€œ", "-"),
        ("Ã¢â‚¬â€", "-"),
        ("Ã¢â‚¬Â¦", "..."),
        ("â€™", "'"),
        ("â€˜", "'"),
        ("â€œ", "\""),
        ("â€�", "\""),
        ("â€“", "-"),
        ("â€”", "-"),
        ("â€¦", "..."),
        ("â€", "\""),
        ("Â ", " "),
        ("Â", ""),
    ] {
        repaired = repaired.replace(bad, good);
    }
    repaired
}

fn clean_generated_site_output(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    for subdir in [
        "briefs",
        "watch",
        "explainers",
        "stories",
        "opinions",
        "corrections",
    ] {
        let path = output_dir.join(subdir);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
    }

    for filename in [
        "index.html",
        "about.html",
        "ethics.html",
        "how-we-report.html",
        "corrections.html",
        "feed.xml",
        "newsletter.md",
        "substack.md",
        "share-package.md",
        "facebook-post.txt",
        "subreddit-post.md",
        "nextdoor-post.txt",
        "short-link-blurb.txt",
        "publish-manifest.json",
        "site-package.zip",
        "styles.css",
        "print.css",
    ] {
        let path = output_dir.join(filename);
        if path.exists() {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn absolute_site_url(base_url: &str, relative_path: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let rel = relative_path.trim_start_matches('/');
    if rel.is_empty() {
        base.to_string()
    } else {
        format!("{}/{}", base, rel)
    }
}

fn replace_share_placeholders(contents: String, result: &CompileStaticSiteResult) -> String {
    let Some(public_url) = result.published_url.as_deref() else {
        return contents;
    };
    let public_url = public_url.trim_end_matches('/');
    let rss_url = absolute_site_url(public_url, "feed.xml");
    let mut updated = contents
        .replace("[add public URL]", public_url)
        .replace("[short link]", public_url)
        .replace(
            "Website home: index.html",
            &format!("Website home: {}", public_url),
        )
        .replace("RSS feed: feed.xml", &format!("RSS feed: {}", rss_url));

    for article in &result.articles {
        updated = updated.replace(
            &article.relative_path,
            &absolute_site_url(public_url, &article.relative_path),
        );
    }
    updated
}

pub fn record_publish_destination_files(
    output_dir: impl AsRef<Path>,
    provider: &str,
    published_url: &str,
    deployment_id: Option<&str>,
) -> Result<CompileStaticSiteResult, Box<dyn Error>> {
    let output_dir = output_dir.as_ref();
    let manifest_path = output_dir.join("publish-manifest.json");
    let manifest = fs::read_to_string(&manifest_path)?;
    let mut result: CompileStaticSiteResult = serde_json::from_str(&manifest)?;
    result.provider = provider.to_string();
    result.published_url = Some(published_url.to_string());
    result.deployment_id = deployment_id.map(|value| value.to_string());

    for relative_path in [
        &result.newsletter_path,
        &result.substack_path,
        &result.share_package_path,
        &result.facebook_post_path,
        &result.subreddit_post_path,
        &result.nextdoor_post_path,
        &result.short_link_blurb_path,
    ] {
        let path = output_dir.join(relative_path);
        if path.exists() {
            let contents = fs::read_to_string(&path)?;
            fs::write(&path, replace_share_placeholders(contents, &result))?;
        }
    }

    fs::write(&manifest_path, serde_json::to_string_pretty(&result)?)?;
    write_zip_package(output_dir, &output_dir.join(&result.zip_path))?;
    Ok(result)
}

fn path_for_manifest(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn add_zip_file(
    zip: &mut zip::ZipWriter<File>,
    base_dir: &Path,
    path: &Path,
    zip_path: &Path,
) -> Result<(), Box<dyn Error>> {
    if path == zip_path {
        return Ok(());
    }
    let relative = path.strip_prefix(base_dir)?;
    let name = path_for_manifest(relative);
    zip.start_file(name, zip::write::SimpleFileOptions::default())?;
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    Ok(())
}

fn add_zip_dir(
    zip: &mut zip::ZipWriter<File>,
    base_dir: &Path,
    current_dir: &Path,
    zip_path: &Path,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            add_zip_dir(zip, base_dir, &path, zip_path)?;
        } else {
            add_zip_file(zip, base_dir, &path, zip_path)?;
        }
    }
    Ok(())
}

fn write_zip_package(output_dir: &Path, zip_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    add_zip_dir(&mut zip, output_dir, output_dir, zip_path)?;
    zip.finish()?;
    Ok(())
}

/// SEC (GG-B1): allow-list URL schemes for any href/src that reaches the
/// published static site. A markdown link/image whose scheme is not safe is a
/// stored-XSS vector (`javascript:`/`data:`/`vbscript:`). URLs with no scheme
/// (relative paths, `#fragment`, `?query`, protocol-relative `//host`) are safe.
/// `evidence` is allowed because the compiler rewrites `href="evidence:..."` to
/// local `#evidence-N` anchors AFTER rendering.
pub(crate) fn is_safe_url_scheme(dest: &str) -> bool {
    let d = dest.trim_start();
    let mut scheme_end = None;
    for (i, c) in d.char_indices() {
        match c {
            ':' => {
                scheme_end = Some(i);
                break;
            }
            // Reached the path/query/fragment before any ':' => no scheme.
            '/' | '?' | '#' => break,
            _ => {
                let valid = if i == 0 {
                    c.is_ascii_alphabetic()
                } else {
                    c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')
                };
                if !valid {
                    break;
                }
            }
        }
    }
    match scheme_end {
        None => true,
        Some(i) => matches!(
            d[..i].to_ascii_lowercase().as_str(),
            "http" | "https" | "mailto" | "evidence"
        ),
    }
}

fn is_safe_logo_src(dest: &str) -> bool {
    let trimmed = dest.trim_start();
    if trimmed.len() >= 5 && trimmed[..5].eq_ignore_ascii_case("data:") {
        let lower = trimmed.to_ascii_lowercase();
        return lower.starts_with("data:image/png;base64,")
            || lower.starts_with("data:image/jpeg;base64,")
            || lower.starts_with("data:image/gif;base64,")
            || lower.starts_with("data:image/webp;base64,");
    }
    is_safe_url_scheme(dest)
}

/// Replace a link/image destination with an inert `#` when its scheme is not
/// allow-listed, so dangerous URIs cannot reach the rendered HTML.
fn sanitize_dest(dest: CowStr<'_>) -> CowStr<'_> {
    if is_safe_url_scheme(&dest) {
        dest
    } else {
        CowStr::Borrowed("#")
    }
}

pub fn render_markdown(markdown: &str) -> String {
    let markdown = repair_common_mojibake(markdown);
    let options = Options::empty();
    // SEC (GG-B1): strip raw HTML events AND neutralize dangerous URI schemes on
    // markdown-syntax links/images. pulldown-cmark otherwise emits link/image
    // destinations verbatim into href/src, so `[x](javascript:...)` /
    // `![x](data:...)` in a draft (LLM-authored or pasted) would become live
    // script vectors on the public static site.
    let parser = Parser::new_ext(&markdown, options).filter_map(|event| match event {
        Event::Html(_) | Event::InlineHtml(_) => None,
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => Some(Event::Start(Tag::Link {
            link_type,
            dest_url: sanitize_dest(dest_url),
            title,
            id,
        })),
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => Some(Event::Start(Tag::Image {
            link_type,
            dest_url: sanitize_dest(dest_url),
            title,
            id,
        })),
        other => Some(other),
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn compile_static_site(
    conn: &Connection,
    output_dir_str: &str,
    profile_json: &str,
) -> Result<CompileStaticSiteResult, Box<dyn Error>> {
    let output_dir = Path::new(output_dir_str);
    let generated_at_time = Utc::now();
    let generated_at = generated_at_time.to_rfc3339();
    let issue_id = format!(
        "issue-{}-{:09}",
        generated_at_time.format("%Y%m%d-%H%M%S"),
        generated_at_time.timestamp_subsec_nanos()
    );
    let mut files_written = 0usize;
    let mut generated_files: Vec<String> = Vec::new();
    let skipped_count = 0usize;
    let mut compiled_articles: Vec<CompiledArticle> = Vec::new();

    // 1. Create standard output directories
    fs::create_dir_all(output_dir)?;
    clean_generated_site_output(output_dir)?;
    let formats = vec![
        "briefs",
        "watch",
        "explainers",
        "stories",
        "opinions",
        "corrections",
    ];
    for fmt in &formats {
        fs::create_dir_all(output_dir.join(fmt))?;
    }

    // 2. Parse Profile Settings
    let profile: CompilerProfile = serde_json::from_str(profile_json).unwrap_or_default();
    let current_year = Utc::now().year().to_string();

    // ENG-002: profile fields are author-controlled local config, but the realistic
    // trigger is an author pasting boilerplate (a city blurb, an LLM tagline) that
    // contains markup. Escape them once here so every HTML and RSS sink emits them
    // as inert text instead of live script. encode_safe escapes &<>"' which is
    // sufficient for both the HTML element contexts and the RSS/XML text nodes
    // below (matching how titles/excerpts are already escaped).
    let site_title = repair_common_mojibake(&profile.site_title);
    let site_subtitle = repair_common_mojibake(&profile.site_subtitle);
    let about_text = repair_common_mojibake(&profile.about_text);
    let ethics_text = repair_common_mojibake(&profile.ethics_text);
    let how_we_report_text = repair_common_mojibake(&profile.how_we_report_text);
    let safe_site_title = html_escape::encode_safe(&site_title).to_string();
    let safe_site_subtitle = html_escape::encode_safe(&site_subtitle).to_string();
    let safe_about_text = html_escape::encode_safe(&about_text).to_string();
    let safe_ethics_text = html_escape::encode_safe(&ethics_text).to_string();
    let footer_html = footer_notice(&profile);
    let logo_html = logo_html(&profile);
    let custom_style = custom_style(&profile);
    let layout_class = layout_class(&profile);

    // 3. Copy Stylesheets
    write_site_file(
        output_dir.join("styles.css"),
        STYLES_CSS,
        &mut files_written,
    )?;
    generated_files.push("styles.css".to_string());
    write_site_file(output_dir.join("print.css"), PRINT_CSS, &mut files_written)?;
    generated_files.push("print.css".to_string());

    // 4. Fetch drafts that are published, corrected, or ready_to_publish
    let drafts = list_drafts(conn)?;
    let mut published_drafts = Vec::new();
    for d in drafts {
        if d.status == "published" || d.status == "corrected" || d.status == "ready_to_publish" {
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
        let title = repair_common_mojibake(&draft.title);
        let content = repair_common_mojibake(&draft.content);

        // Editorial review is advisory at compile time: do not silently filter a
        // story out of the public package. If a draft reached a publishable
        // status by any path, compile it and preserve any review concerns as a
        // visible note for the publisher to resolve or intentionally keep.
        let mut editorial_notes: Vec<String> = Vec::new();
        let (attested, overridden) = match super::db::get_draft_publish_gate(conn, draft_id) {
            Ok((att, ovr)) => (
                att.as_deref()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false),
                ovr.as_deref()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false),
            ),
            Err(_) => (false, false),
        };
        if !attested {
            editorial_notes.push(
                "No human review attestation was recorded before this package was compiled."
                    .to_string(),
            );
        }
        if !overridden {
            match super::guardrails::run_guardrails_check(conn, draft_id) {
                Ok(report) if report.is_clean => {}
                Ok(report) => {
                    let errs = report
                        .issues
                        .iter()
                        .filter(|i| i.severity == "error")
                        .count();
                    if errs > 0 {
                        editorial_notes.push(format!(
                            "{} editor-configured sensitive issue(s) were still present when this package was compiled.",
                            errs
                        ));
                    }
                }
                Err(e) => {
                    editorial_notes.push(format!(
                        "The pre-publication review check could not run: {}.",
                        e
                    ));
                }
            }
        }

        let subfolder = match draft.format.as_str() {
            "brief" => "briefs",
            "watch" => "watch",
            "explainer" => "explainers",
            "investigation" => "stories",
            "opinion" => "opinions",
            _ => "stories",
        };

        // Convert Markdown body to HTML
        let raw_html = render_markdown(&content);
        // Replace custom references `href="evidence:123"` or `href="evidence://123"` with local section anchors `#evidence-123`
        let mut html_content = raw_html.replace("href=\"evidence://", "href=\"#evidence-");
        html_content = html_content.replace("href=\"evidence:", "href=\"#evidence-");

        // Format Linked Evidence List
        let mut evidence_html = String::new();
        if let Some(lid) = draft.lead_id {
            let items = get_evidence_by_lead(conn, lid)?;
            for item in items {
                let item_id = item.id.unwrap_or(0);
                let raw_url = item.url.clone().unwrap_or_else(|| "#".to_string());
                // SEC (GG-B1 / QA-Min1): defense-in-depth at the publish sink —
                // neutralize non-allowlisted schemes even though ingest validates
                // source URLs upstream. encode_safe only escapes characters; it
                // does not stop a `javascript:` scheme from being a live href.
                let source_url = if is_safe_url_scheme(&raw_url) {
                    raw_url
                } else {
                    "#".to_string()
                };
                let safe_url = html_escape::encode_safe(&source_url);
                let excerpt = repair_common_mojibake(&item.excerpt);
                let safe_excerpt = html_escape::encode_safe(&excerpt);
                evidence_html.push_str(&format!(
                    "<li id=\"evidence-{}\" style=\"margin-bottom: 0.5rem;\"><strong>[Ref {}]</strong> <a href=\"{}\" target=\"_blank\">Original Document Link</a>: <span style=\"font-style: italic;\">\"{}\"</span></li>\n",
                    item_id, item_id, safe_url, safe_excerpt
                ));
            }
        }
        if evidence_html.is_empty() {
            evidence_html = "<li>No source links were attached to this article.</li>".to_string();
        }

        // Format Correction Banner
        let correction_banner = if draft.status == "corrected" {
            let note = repair_common_mojibake(&draft.correction_note.clone().unwrap_or_default());
            let safe_note = html_escape::encode_safe(&note);
            format!(
                "<div class=\"correction-banner\" style=\"background-color: #fff3cd; border: 1px solid #ffeeba; color: #856404; padding: 1rem; margin-bottom: 2rem; border-radius: 4px; font-family: var(--font-sans); font-size: 0.95rem;\"><strong>CORRECTION:</strong> {}</div>\n",
                safe_note
            )
        } else {
            String::new()
        };
        let editorial_note_banner = if editorial_notes.is_empty() {
            String::new()
        } else {
            let notes = editorial_notes
                .iter()
                .map(|note| format!("<li>{}</li>", html_escape::encode_safe(note)))
                .collect::<Vec<_>>()
                .join("");
            format!(
                "<div class=\"correction-banner\" style=\"background-color: #fff8e1; border: 1px solid #f6d365; color: #6b4b00; padding: 1rem; margin-bottom: 2rem; border-radius: 4px; font-family: var(--font-sans); font-size: 0.95rem;\"><strong>EDITOR REVIEW NOTE:</strong><ul style=\"margin: 0.5rem 0 0 1rem; padding: 0;\">{}</ul></div>\n",
                notes
            )
        };

        let safe_title = html_escape::encode_safe(&title).to_string();

        // Generate Post Page
        let mut post_html = POST_TEMPLATE.to_string();
        post_html = post_html.replace("{{SITE_TITLE}}", &safe_site_title);
        post_html = post_html.replace("{{SITE_SUBTITLE}}", &safe_site_subtitle);
        post_html = post_html.replace("{{LOGO_HTML}}", &logo_html);
        post_html = post_html.replace("{{CUSTOM_STYLE}}", &custom_style);
        post_html = post_html.replace("{{LAYOUT_CLASS}}", layout_class);
        post_html = post_html.replace("{{POST_TITLE}}", &safe_title);
        post_html = post_html.replace("{{POST_DESCRIPTION}}", &safe_title);
        post_html = post_html.replace("{{POST_DATE}}", &draft.updated_at);
        post_html = post_html.replace("{{POST_FORMAT}}", &draft.format);
        post_html = post_html.replace("{{POST_CONTENT}}", &html_content);
        post_html = post_html.replace(
            "{{EVIDENCE_SECTION}}",
            &EVIDENCE_SECTION_TEMPLATE.replace("{{EVIDENCE_CITATIONS}}", &evidence_html),
        );
        post_html = post_html.replace(
            "{{CORRECTION_BANNER}}",
            &(correction_banner + &editorial_note_banner),
        );
        post_html = post_html.replace("{{YEAR}}", &current_year);
        post_html = post_html.replace("{{FOOTER_NOTICE}}", &footer_html);

        // Prepend "../" to relative assets and links since post page is in a subfolder
        post_html = post_html.replace("href=\"styles.css\"", "href=\"../styles.css\"");
        post_html = post_html.replace("href=\"print.css\"", "href=\"../print.css\"");
        post_html = post_html.replace("href=\"index.html\"", "href=\"../index.html\"");
        post_html = post_html.replace("href=\"about.html\"", "href=\"../about.html\"");
        post_html = post_html.replace("href=\"ethics.html\"", "href=\"../ethics.html\"");
        post_html = post_html.replace(
            "href=\"how-we-report.html\"",
            "href=\"../how-we-report.html\"",
        );
        post_html = post_html.replace("href=\"corrections.html\"", "href=\"../corrections.html\"");
        post_html = post_html.replace("href=\"feed.xml\"", "href=\"../feed.xml\"");

        let relative_path = format!("{}/{}.html", subfolder, draft_id);
        let dest_path = output_dir.join(&relative_path);
        write_site_file(dest_path, post_html, &mut files_written)?;
        generated_files.push(relative_path.clone());
        compiled_articles.push(CompiledArticle {
            title: title.clone(),
            format: draft.format.clone(),
            relative_path: relative_path.clone(),
            updated_at: draft.updated_at.clone(),
        });

        // Update database linking
        let published_post = PublishedPost {
            id: None,
            draft_id,
            file_path: relative_path.clone(),
            url: relative_path.clone(),
            published_at: draft.updated_at.clone(),
            correction_history: draft
                .correction_note
                .clone()
                .unwrap_or_else(|| "[]".to_string()),
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
            let note_str =
                repair_common_mojibake(&draft.correction_note.clone().unwrap_or_default());
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
    index_html = index_html.replace("{{SITE_TITLE}}", &safe_site_title);
    index_html = index_html.replace("{{SITE_SUBTITLE}}", &safe_site_subtitle);
    index_html = index_html.replace("{{LOGO_HTML}}", &logo_html);
    index_html = index_html.replace("{{CUSTOM_STYLE}}", &custom_style);
    index_html = index_html.replace("{{LAYOUT_CLASS}}", layout_class);
    index_html = index_html.replace("{{ARTICLES}}", &article_list_html);

    // Sidebar: list of latest 5 posts + profile description
    let mut sidebar_html = format!(
        "<div class=\"sidebar-section\">\n  <h3 class=\"sidebar-title\">About this Site</h3>\n  <p>{}</p>\n</div>\n",
        safe_about_text
    );
    if !safe_ethics_text.trim().is_empty() {
        sidebar_html.push_str(&format!(
            "<div class=\"sidebar-section\">\n  <h3 class=\"sidebar-title\">Ethics Standards</h3>\n  <p>{}</p>\n</div>",
            safe_ethics_text
        ));
    }
    index_html = index_html.replace("{{SIDEBAR}}", &sidebar_html);
    index_html = index_html.replace("{{YEAR}}", &current_year);
    index_html = index_html.replace("{{FOOTER_NOTICE}}", &footer_html);
    write_site_file(
        output_dir.join("index.html"),
        index_html,
        &mut files_written,
    )?;
    generated_files.push("index.html".to_string());

    // 7. Build About, Ethics, and How We Report pages
    let mut compile_info_page =
        |filename: &str, title: &str, content_md: &str| -> Result<(), Box<dyn Error>> {
            let body_html = render_markdown(content_md);
            let mut page_html = POST_TEMPLATE.to_string();
            page_html = page_html.replace("{{SITE_TITLE}}", &safe_site_title);
            page_html = page_html.replace("{{SITE_SUBTITLE}}", &safe_site_subtitle);
            page_html = page_html.replace("{{LOGO_HTML}}", &logo_html);
            page_html = page_html.replace("{{CUSTOM_STYLE}}", &custom_style);
            page_html = page_html.replace("{{LAYOUT_CLASS}}", layout_class);
            page_html = page_html.replace("{{POST_TITLE}}", title);
            page_html = page_html.replace("{{POST_DESCRIPTION}}", title);
            page_html = page_html.replace("{{POST_DATE}}", &Utc::now().to_rfc3339());
            page_html = page_html.replace("{{POST_FORMAT}}", "meta");
            page_html = page_html.replace("{{POST_CONTENT}}", &body_html);
            page_html = page_html.replace("{{EVIDENCE_SECTION}}", "");
            page_html = page_html.replace("{{CORRECTION_BANNER}}", "");
            page_html = page_html.replace("{{YEAR}}", &current_year);
            page_html = page_html.replace("{{FOOTER_NOTICE}}", &footer_html);
            write_site_file(output_dir.join(filename), page_html, &mut files_written)?;
            generated_files.push(filename.to_string());
            Ok(())
        };

    compile_info_page("about.html", "About The Civic Desk", &about_text)?;
    compile_info_page("ethics.html", "Reporting Ethics & Standards", &ethics_text)?;
    compile_info_page("how-we-report.html", "How We Report", &how_we_report_text)?;

    // 8. Build corrections.html ledger
    let mut corrections_html = POST_TEMPLATE.to_string();
    corrections_html = corrections_html.replace("{{SITE_TITLE}}", &safe_site_title);
    corrections_html = corrections_html.replace("{{SITE_SUBTITLE}}", &safe_site_subtitle);
    corrections_html = corrections_html.replace("{{LOGO_HTML}}", &logo_html);
    corrections_html = corrections_html.replace("{{CUSTOM_STYLE}}", &custom_style);
    corrections_html = corrections_html.replace("{{LAYOUT_CLASS}}", layout_class);
    corrections_html = corrections_html.replace("{{POST_TITLE}}", "Public Corrections Ledger");
    corrections_html =
        corrections_html.replace("{{POST_DESCRIPTION}}", "Public Corrections Ledger");
    corrections_html = corrections_html.replace("{{POST_DATE}}", &Utc::now().to_rfc3339());
    corrections_html = corrections_html.replace("{{POST_FORMAT}}", "ledger");
    corrections_html = corrections_html.replace("{{POST_CONTENT}}", &corrections_list_html);
    corrections_html = corrections_html.replace("{{EVIDENCE_SECTION}}", "");
    corrections_html = corrections_html.replace("{{CORRECTION_BANNER}}", "");
    corrections_html = corrections_html.replace("{{YEAR}}", &current_year);
    corrections_html = corrections_html.replace("{{FOOTER_NOTICE}}", &footer_html);
    write_site_file(
        output_dir.join("corrections.html"),
        corrections_html,
        &mut files_written,
    )?;
    generated_files.push("corrections.html".to_string());

    // 9. Build RSS feed.xml
    let rss_feed = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<rss version=\"2.0\">\n  <channel>\n    <title>{}</title>\n    <link>index.html</link>\n    <description>{}</description>\n    <language>en-us</language>\n    <pubDate>{}</pubDate>\n    <lastBuildDate>{}</lastBuildDate>\n{}\n  </channel>\n</rss>\n",
        safe_site_title, safe_site_subtitle, Utc::now().to_rfc2822(), Utc::now().to_rfc2822(), rss_items
    );
    write_site_file(output_dir.join("feed.xml"), rss_feed, &mut files_written)?;
    generated_files.push("feed.xml".to_string());

    let mut newsletter = format!(
        "# {}\n\n{}\n\nGenerated: {}\n\n",
        site_title, site_subtitle, generated_at
    );
    if compiled_articles.is_empty() {
        newsletter.push_str("No approved stories were included in this package.\n");
    } else {
        newsletter.push_str("## This Issue\n\n");
        for article in &compiled_articles {
            newsletter.push_str(&format!(
                "- [{}]({}) - {} - {}\n",
                article.title, article.relative_path, article.format, article.updated_at
            ));
        }
    }
    newsletter.push_str("\n## Links\n\n- Website home: index.html\n- RSS feed: feed.xml\n");
    let newsletter_path = output_dir.join("newsletter.md");
    write_site_file(&newsletter_path, newsletter, &mut files_written)?;
    generated_files.push("newsletter.md".to_string());

    let mut substack = format!(
        "# {}\n\n{}\n\n_Publication package generated by The Civic Desk on {}._\n\n",
        site_title, site_subtitle, generated_at
    );
    if compiled_articles.is_empty() {
        substack.push_str("No approved stories were included in this package.\n");
    } else {
        for article in &compiled_articles {
            substack.push_str(&format!(
                "## {}\n\nRead the full story on the public site: {}\n\n",
                article.title, article.relative_path
            ));
        }
    }
    substack.push_str("---\n\nReview links and formatting after pasting into Substack.\n");
    write_site_file(output_dir.join("substack.md"), substack, &mut files_written)?;
    generated_files.push("substack.md".to_string());

    let mut share_package = format!(
        "# Share Package\n\nGenerated: {}\n\nWebsite home: index.html\nRSS feed: feed.xml\n\n",
        generated_at
    );
    if compiled_articles.is_empty() {
        share_package.push_str(
            "No stories are ready to share yet. Approve at least one story, then compile again.\n",
        );
    } else {
        share_package.push_str("## Suggested Community Posts\n\n");
        for article in &compiled_articles {
            share_package.push_str(&format!(
                "### {}\n\nLocal update: {}. Read it in the latest issue: {}\n\n",
                article.title, article.title, article.relative_path
            ));
        }
    }
    share_package.push_str("## Hosting Notes\n\nPublish instantly with here.now. Use GitHub Pages if you want a durable public archive in your own repository. Cloudflare Pages and Netlify remain good technical-hosting options.\n");
    let share_package_path = output_dir.join("share-package.md");
    write_site_file(&share_package_path, share_package, &mut files_written)?;
    generated_files.push("share-package.md".to_string());

    let headline = compiled_articles
        .first()
        .map(|article| article.title.as_str())
        .unwrap_or("New local civic issue");
    let article_count = compiled_articles.len();
    let facebook_post = format!(
        "{} is live. {} local update(s), with links and an RSS feed for following future issues.\n\nRead it here: [add public URL]\n",
        site_title, article_count
    );
    write_site_file(
        output_dir.join("facebook-post.txt"),
        facebook_post,
        &mut files_written,
    )?;
    generated_files.push("facebook-post.txt".to_string());

    let subreddit_post = format!(
        "# {}\n\n{} local update(s).\n\nTop item: {}\n\nRead the issue: [add public URL]\n\nSources and corrections policy are included on the site.\n",
        site_title, article_count, headline
    );
    write_site_file(
        output_dir.join("subreddit-post.md"),
        subreddit_post,
        &mut files_written,
    )?;
    generated_files.push("subreddit-post.md".to_string());

    let nextdoor_post = format!(
        "A new {} issue is ready with {} local update(s). Top item: {}. Read it here: [add public URL]",
        site_title, article_count, headline
    );
    write_site_file(
        output_dir.join("nextdoor-post.txt"),
        nextdoor_post,
        &mut files_written,
    )?;
    generated_files.push("nextdoor-post.txt".to_string());

    let short_link_blurb = format!(
        "{}: {} local update(s). Read: [short link]",
        site_title, article_count
    );
    write_site_file(
        output_dir.join("short-link-blurb.txt"),
        short_link_blurb,
        &mut files_written,
    )?;
    generated_files.push("short-link-blurb.txt".to_string());

    generated_files.push("publish-manifest.json".to_string());
    generated_files.push("site-package.zip".to_string());
    let mut result = CompileStaticSiteResult {
        issue_id,
        output_dir: path_for_manifest(output_dir),
        generated_at,
        provider: "local_export".to_string(),
        published_url: None,
        deployment_id: None,
        article_count: compiled_articles.len(),
        skipped_count,
        files_written,
        generated_files,
        index_path: path_for_manifest(&PathBuf::from("index.html")),
        rss_path: path_for_manifest(&PathBuf::from("feed.xml")),
        newsletter_path: path_for_manifest(&PathBuf::from("newsletter.md")),
        substack_path: path_for_manifest(&PathBuf::from("substack.md")),
        share_package_path: path_for_manifest(&PathBuf::from("share-package.md")),
        facebook_post_path: path_for_manifest(&PathBuf::from("facebook-post.txt")),
        subreddit_post_path: path_for_manifest(&PathBuf::from("subreddit-post.md")),
        nextdoor_post_path: path_for_manifest(&PathBuf::from("nextdoor-post.txt")),
        short_link_blurb_path: path_for_manifest(&PathBuf::from("short-link-blurb.txt")),
        manifest_path: path_for_manifest(&PathBuf::from("publish-manifest.json")),
        zip_path: path_for_manifest(&PathBuf::from("site-package.zip")),
        articles: compiled_articles,
    };

    result.files_written = files_written + 2;
    let manifest = serde_json::to_string_pretty(&result)?;
    write_site_file(
        output_dir.join("publish-manifest.json"),
        manifest,
        &mut files_written,
    )?;

    let zip_path = output_dir.join("site-package.zip");
    write_zip_package(output_dir, &zip_path)?;
    files_written += 1;
    result.files_written = files_written;

    insert_publish_run(
        conn,
        &PublishRun {
            id: None,
            issue_id: result.issue_id.clone(),
            output_path: result.output_dir.clone(),
            generated_files: serde_json::to_string(&result.generated_files)?,
            provider: result.provider.clone(),
            published_url: result.published_url.clone(),
            deployment_id: result.deployment_id.clone(),
            article_count: result.article_count as i32,
            skipped_count: result.skipped_count as i32,
            files_written: result.files_written as i32,
            generated_at: result.generated_at.clone(),
        },
    )?;

    Ok(result)
}
