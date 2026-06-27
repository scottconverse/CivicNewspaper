use async_trait::async_trait;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const KEYRING_SERVICE: &str = "The Civic Desk Publisher";
const USER_AGENT: &str = "The Civic Desk Publisher";
const HERENOW_API_BASE: &str = "https://here.now";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublisherProvider {
    HereNow,
    GithubPages,
    Netlify,
    CloudflarePages,
    Substack,
    Wordpress,
    Other,
}

impl PublisherProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            PublisherProvider::HereNow => "here_now",
            PublisherProvider::GithubPages => "github_pages",
            PublisherProvider::Netlify => "netlify",
            PublisherProvider::CloudflarePages => "cloudflare_pages",
            PublisherProvider::Substack => "substack",
            PublisherProvider::Wordpress => "wordpress",
            PublisherProvider::Other => "other",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "here_now" => Some(PublisherProvider::HereNow),
            "github_pages" => Some(PublisherProvider::GithubPages),
            "netlify" => Some(PublisherProvider::Netlify),
            "cloudflare_pages" => Some(PublisherProvider::CloudflarePages),
            "substack" => Some(PublisherProvider::Substack),
            "wordpress" => Some(PublisherProvider::Wordpress),
            "other" => Some(PublisherProvider::Other),
            _ => None,
        }
    }

    pub fn requires_credential(&self) -> bool {
        matches!(
            self,
            PublisherProvider::GithubPages
                | PublisherProvider::Netlify
                | PublisherProvider::CloudflarePages
                | PublisherProvider::Wordpress
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherConfig {
    pub provider: String,
    pub display_name: String,
    pub site_url: Option<String>,
    pub project_hint: Option<String>,
    #[serde(default)]
    pub site_id: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub path_prefix: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    pub has_credential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherConfigInput {
    pub provider: String,
    pub display_name: String,
    pub site_url: Option<String>,
    pub project_hint: Option<String>,
    #[serde(default)]
    pub site_id: Option<String>,
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub path_prefix: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    pub credential: Option<String>,
    pub clear_credential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherTestResult {
    pub provider: String,
    pub ok: bool,
    pub message: String,
    pub credential_checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherPublishRequest {
    pub output_dir: String,
    pub provider: String,
    pub published_url: Option<String>,
    pub deployment_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherPublishResult {
    pub provider: String,
    pub published_url: String,
    pub deployment_id: Option<String>,
    pub message: String,
}

#[async_trait]
pub trait Publisher: Send + Sync {
    fn validate_config(&self, config: &PublisherConfig) -> Result<(), String>;
    async fn test_connection(&self, config: &PublisherConfig) -> PublisherTestResult;
    async fn publish_folder(
        &self,
        config: &PublisherConfig,
        request: &PublisherPublishRequest,
    ) -> Result<PublisherPublishResult, String>;
}

struct HttpPublisher {
    provider: PublisherProvider,
}

impl HttpPublisher {
    fn new(provider: PublisherProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl Publisher for HttpPublisher {
    fn validate_config(&self, config: &PublisherConfig) -> Result<(), String> {
        validate_common_config(&self.provider, config)?;
        match self.provider {
            PublisherProvider::HereNow => {
                if let Some(slug) = config.site_id.as_deref() {
                    validate_herenow_slug(slug)?;
                }
            }
            PublisherProvider::Netlify => {
                required_field("Netlify site ID", config.site_id.as_deref())?;
            }
            PublisherProvider::GithubPages => {
                validate_repo(config.repo.as_deref())?;
                required_field("GitHub branch", config.branch.as_deref())?;
                validate_github_pages_path(config.path_prefix.as_deref())?;
            }
            PublisherProvider::CloudflarePages => {
                required_field("Cloudflare account ID", config.account_id.as_deref())?;
                required_field("Cloudflare Pages project name", config.site_id.as_deref())?;
            }
            PublisherProvider::Wordpress => {
                required_field("WordPress site URL", config.site_url.as_deref())?;
                required_field("WordPress username", config.username.as_deref())?;
            }
            PublisherProvider::Substack => {
                return Err(
                    "Substack does not provide a supported public publishing API. Use the generated Substack draft for assisted publishing."
                        .to_string(),
                );
            }
            PublisherProvider::Other => {}
        }
        Ok(())
    }

    async fn test_connection(&self, config: &PublisherConfig) -> PublisherTestResult {
        let credential_checked = self.provider.requires_credential();
        if let Err(message) = self.validate_config(config) {
            return test_result(self.provider.as_str(), false, message, credential_checked);
        }
        if credential_checked && !has_provider_credential(self.provider.as_str()) {
            return test_result(
                self.provider.as_str(),
                false,
                "Save the required provider credential before testing this connector.".to_string(),
                true,
            );
        }

        let result = match self.provider {
            PublisherProvider::HereNow => test_herenow(config).await,
            PublisherProvider::Netlify => test_netlify(config).await,
            PublisherProvider::GithubPages => test_github(config).await,
            PublisherProvider::CloudflarePages => test_cloudflare(config).await,
            PublisherProvider::Wordpress => test_wordpress(config).await,
            PublisherProvider::Substack | PublisherProvider::Other => {
                Ok("Connector settings are valid for assisted publishing.".to_string())
            }
        };

        match result {
            Ok(message) => test_result(self.provider.as_str(), true, message, credential_checked),
            Err(message) => test_result(self.provider.as_str(), false, message, credential_checked),
        }
    }

    async fn publish_folder(
        &self,
        config: &PublisherConfig,
        request: &PublisherPublishRequest,
    ) -> Result<PublisherPublishResult, String> {
        if request.provider != self.provider.as_str() {
            return Err("Publisher request provider does not match connector.".to_string());
        }
        self.validate_config(config)?;
        validate_publish_artifacts(&request.output_dir)?;
        match self.provider {
            PublisherProvider::HereNow => publish_herenow(config, request).await,
            PublisherProvider::Netlify => publish_netlify(config, request).await,
            PublisherProvider::GithubPages => publish_github(config, request).await,
            PublisherProvider::CloudflarePages => publish_cloudflare(config, request).await,
            PublisherProvider::Wordpress => publish_wordpress(config, request).await,
            PublisherProvider::Substack => Err(
                "Substack does not provide a supported public publishing API. Open the generated Substack draft and paste it into Substack."
                    .to_string(),
            ),
            PublisherProvider::Other => publish_assisted(config, request),
        }
    }
}

pub fn publisher_for(provider: &str) -> Result<Box<dyn Publisher>, String> {
    let provider = PublisherProvider::from_str(provider)
        .ok_or_else(|| "Unsupported publishing provider.".to_string())?;
    Ok(Box::new(HttpPublisher::new(provider)))
}

fn test_result(
    provider: &str,
    ok: bool,
    message: String,
    credential_checked: bool,
) -> PublisherTestResult {
    PublisherTestResult {
        provider: provider.to_string(),
        ok,
        message,
        credential_checked,
    }
}

fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| format!("Could not create HTTP client: {e}"))
}

fn required_field(label: &str, value: Option<&str>) -> Result<String, String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{label} is required for this connector."))
}

fn validate_common_config(
    provider: &PublisherProvider,
    config: &PublisherConfig,
) -> Result<(), String> {
    if PublisherProvider::from_str(&config.provider) != Some(provider.clone()) {
        return Err("Publisher config provider does not match connector.".to_string());
    }
    if config.display_name.trim().is_empty() {
        return Err("Connector name cannot be empty.".to_string());
    }
    if let Some(url) = config.site_url.as_deref() {
        validate_public_url(url)?;
    }
    Ok(())
}

fn validate_repo(value: Option<&str>) -> Result<String, String> {
    let repo = required_field("GitHub repository", value)?;
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 || parts.iter().any(|part| part.trim().is_empty()) {
        return Err("GitHub repository must be in owner/repo format.".to_string());
    }
    Ok(repo)
}

fn credential(provider: PublisherProvider) -> Result<String, String> {
    get_provider_credential(provider.as_str())?.ok_or_else(|| {
        format!(
            "No credential is saved for {}.",
            provider.as_str().replace('_', " ")
        )
    })
}

pub fn validate_public_url(url: &str) -> Result<String, String> {
    let parsed =
        reqwest::Url::parse(url.trim()).map_err(|e| format!("Invalid public URL: {}", e))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err("Public URL must start with http:// or https://.".to_string());
    }
    Ok(parsed.as_str().trim_end_matches('/').to_string())
}

pub fn validate_publish_artifacts(output_dir: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(output_dir.trim());
    if !path.join("publish-manifest.json").exists() {
        return Err("Compile the site before publishing or recording a public URL.".to_string());
    }
    if !path.join("site-package.zip").exists() {
        return Err("The hosting ZIP package is missing. Compile the site again.".to_string());
    }
    Ok(path)
}

fn fallback_url(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<String, String> {
    if let Some(url) = request
        .published_url
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        return validate_public_url(url);
    }
    if let Some(url) = config.site_url.as_deref() {
        return validate_public_url(url);
    }
    Err(
        "A public URL is required for this connector or could not be returned by the provider."
            .to_string(),
    )
}

async fn test_netlify(config: &PublisherConfig) -> Result<String, String> {
    let token = credential(PublisherProvider::Netlify)?;
    let site_id = required_field("Netlify site ID", config.site_id.as_deref())?;
    let url = format!("https://api.netlify.com/api/v1/sites/{site_id}");
    let response = http_client()?
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Netlify connection failed: {e}"))?;
    if response.status().is_success() {
        Ok("Netlify accepted the site ID and API token.".to_string())
    } else {
        Err(format!(
            "Netlify rejected the connection test with status {}.",
            response.status()
        ))
    }
}

async fn publish_netlify(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    let token = credential(PublisherProvider::Netlify)?;
    let site_id = required_field("Netlify site ID", config.site_id.as_deref())?;
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let zip_path = output_dir.join("site-package.zip");
    let zip =
        std::fs::read(&zip_path).map_err(|e| format!("Could not read hosting ZIP package: {e}"))?;
    let url = format!("https://api.netlify.com/api/v1/sites/{site_id}/deploys");
    let response = http_client()?
        .post(url)
        .bearer_auth(token)
        .header(reqwest::header::CONTENT_TYPE, "application/zip")
        .body(zip)
        .send()
        .await
        .map_err(|e| format!("Netlify deploy failed: {e}"))?;
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!(
            "Netlify deploy failed with status {status}: {body}"
        ));
    }
    #[derive(Deserialize)]
    struct NetlifyDeploy {
        id: Option<String>,
        deploy_url: Option<String>,
        ssl_url: Option<String>,
        url: Option<String>,
    }
    let deploy: NetlifyDeploy =
        serde_json::from_str(&body).map_err(|e| format!("Could not read Netlify response: {e}"))?;
    let published_url = deploy
        .ssl_url
        .or(deploy.url)
        .or(deploy.deploy_url)
        .or_else(|| request.published_url.clone())
        .or_else(|| config.site_url.clone())
        .ok_or_else(|| "Netlify did not return a published URL.".to_string())?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::Netlify.as_str().to_string(),
        published_url: validate_public_url(&published_url)?,
        deployment_id: deploy.id.or_else(|| request.deployment_id.clone()),
        message: "Uploaded the site ZIP to Netlify.".to_string(),
    })
}

fn validate_herenow_slug(value: &str) -> Result<String, String> {
    let slug = value.trim();
    if slug.is_empty() {
        return Ok(String::new());
    }
    let valid = slug
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        && !slug.starts_with('-')
        && !slug.ends_with('-');
    if valid {
        Ok(slug.to_string())
    } else {
        Err("here.now slug must use lowercase letters, numbers, and hyphens.".to_string())
    }
}

async fn test_herenow(_config: &PublisherConfig) -> Result<String, String> {
    if let Some(token) = get_provider_credential(PublisherProvider::HereNow.as_str())? {
        let response = http_client()?
            .get(format!("{HERENOW_API_BASE}/api/v1/publishes"))
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("here.now connection failed: {e}"))?;
        if response.status().is_success() {
            Ok("here.now accepted the saved API key. Publishes will create permanent account-owned sites.".to_string())
        } else {
            Err(format!(
                "here.now rejected the connection test with status {}.",
                response.status()
            ))
        }
    } else {
        Ok("here.now is ready for temporary preview publishing. Save an API key for permanent sites.".to_string())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HereNowFileSpec {
    path: String,
    size: u64,
    content_type: String,
    hash: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HereNowPublishRequest {
    files: Vec<HereNowFileSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer: Option<HereNowViewer>,
}

#[derive(Serialize)]
struct HereNowViewer {
    title: String,
    description: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HereNowCreateResponse {
    slug: String,
    site_url: String,
    expires_at: Option<String>,
    anonymous: Option<bool>,
    upload: HereNowUpload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HereNowUpload {
    version_id: String,
    uploads: Vec<HereNowUploadTarget>,
    finalize_url: String,
}

#[derive(Deserialize)]
struct HereNowUploadTarget {
    path: String,
    method: String,
    url: String,
    headers: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HereNowFinalizeResponse {
    slug: String,
    site_url: String,
    current_version_id: String,
}

async fn publish_herenow(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let files = static_site_files(&output_dir)?;
    let mut file_specs = Vec::new();
    let mut file_bytes = std::collections::HashMap::new();
    for file in files {
        let relative = file
            .strip_prefix(&output_dir)
            .map_err(|e| format!("Could not resolve generated file path: {e}"))?;
        let relative_path = relative.to_string_lossy().replace('\\', "/");
        let bytes = std::fs::read(&file)
            .map_err(|e| format!("Could not read generated file {}: {e}", file.display()))?;
        let hash = format!("{:x}", sha2::Sha256::digest(&bytes));
        file_specs.push(HereNowFileSpec {
            path: relative_path.clone(),
            size: bytes.len() as u64,
            content_type: content_type_for_path(&relative_path).to_string(),
            hash,
        });
        file_bytes.insert(relative_path, bytes);
    }

    let display_name = Some(config.display_name.clone()).filter(|value| !value.trim().is_empty());
    let display_description = config
        .project_hint
        .clone()
        .or_else(|| Some("Published by The Civic Desk.".to_string()));
    let payload = HereNowPublishRequest {
        files: file_specs,
        display_name: display_name.clone(),
        display_description: display_description.clone(),
        viewer: display_name.map(|title| HereNowViewer {
            title,
            description: display_description
                .clone()
                .unwrap_or_else(|| "Local civic newspaper issue.".to_string()),
        }),
    };
    let client = http_client()?;
    let token = get_provider_credential(PublisherProvider::HereNow.as_str())?;
    let slug = config
        .site_id
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    if let Some(slug) = slug {
        validate_herenow_slug(slug)?;
    }
    let mut create_request = if let Some(slug) = slug {
        client.put(format!("{HERENOW_API_BASE}/api/v1/publish/{slug}"))
    } else {
        client.post(format!("{HERENOW_API_BASE}/api/v1/publish"))
    }
    .header("X-HereNow-Client", "civicnewspaper/publisher")
    .json(&payload);
    if let Some(token) = token.as_deref() {
        create_request = create_request.bearer_auth(token);
    }
    let create_response = create_request
        .send()
        .await
        .map_err(|e| format!("here.now publish could not start: {e}"))?;
    let created: HereNowCreateResponse =
        json_response(create_response, "here.now publish create").await?;

    for upload in &created.upload.uploads {
        let bytes = file_bytes
            .get(&upload.path)
            .ok_or_else(|| format!("here.now requested unknown file upload: {}", upload.path))?;
        let method = upload.method.parse::<reqwest::Method>().map_err(|e| {
            format!(
                "here.now returned unsupported upload method {}: {e}",
                upload.method
            )
        })?;
        let mut upload_request = client.request(method, &upload.url);
        for (key, value) in &upload.headers {
            upload_request = upload_request.header(key, value);
        }
        let upload_response = upload_request
            .body(bytes.clone())
            .send()
            .await
            .map_err(|e| format!("here.now upload failed for {}: {e}", upload.path))?;
        if !upload_response.status().is_success() {
            let status = upload_response.status();
            let body = upload_response.text().await.unwrap_or_default();
            return Err(format!(
                "here.now upload failed for {} with status {status}: {body}",
                upload.path
            ));
        }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct FinalizePayload<'a> {
        version_id: &'a str,
    }
    let mut finalize_request = client
        .post(&created.upload.finalize_url)
        .header("X-HereNow-Client", "civicnewspaper/publisher")
        .json(&FinalizePayload {
            version_id: &created.upload.version_id,
        });
    if let Some(token) = token.as_deref() {
        finalize_request = finalize_request.bearer_auth(token);
    }
    let finalized: HereNowFinalizeResponse = json_response(
        finalize_request
            .send()
            .await
            .map_err(|e| format!("here.now publish could not finalize: {e}"))?,
        "here.now publish finalize",
    )
    .await?;

    let mut message = if created.anonymous.unwrap_or(false) {
        "Published a temporary here.now preview. Save an API key for permanent sites.".to_string()
    } else {
        "Published a permanent here.now site.".to_string()
    };
    if let Some(expires_at) = created.expires_at {
        message.push_str(&format!(" Expires at {expires_at}."));
    }

    Ok(PublisherPublishResult {
        provider: PublisherProvider::HereNow.as_str().to_string(),
        published_url: validate_public_url(&finalized.site_url)
            .or_else(|_| validate_public_url(&created.site_url))?,
        deployment_id: Some(format!(
            "slug={};version={};created_slug={}",
            finalized.slug, finalized.current_version_id, created.slug
        )),
        message,
    })
}

async fn json_response<T: for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    label: &str,
) -> Result<T, String> {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!("{label} failed with status {status}: {body}"));
    }
    serde_json::from_str(&body).map_err(|e| format!("Could not parse {label} response: {e}"))
}

fn content_type_for_path(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "xml" | "rss" => "application/rss+xml; charset=utf-8",
        "txt" => "text/plain; charset=utf-8",
        "md" => "text/markdown; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    }
}

async fn test_github(config: &PublisherConfig) -> Result<String, String> {
    let token = credential(PublisherProvider::GithubPages)?;
    let repo = validate_repo(config.repo.as_deref())?;
    let branch = required_field("GitHub branch", config.branch.as_deref())?;
    let client = http_client()?;
    let repo_info = github_get_json(&client, &token, &format!("/repos/{repo}")).await?;
    if let Some(permissions) = repo_info.get("permissions") {
        let can_push = permissions
            .get("push")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let is_admin = permissions
            .get("admin")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        if !can_push && !is_admin {
            return Err(
                "GitHub accepted the token, but it does not appear to have repository contents write permission."
                    .to_string(),
            );
        }
    }
    let response = github_request(
        &client,
        reqwest::Method::GET,
        &token,
        &format!("/repos/{repo}/branches/{branch}"),
    )
    .send()
    .await
    .map_err(|e| format!("GitHub connection failed: {e}"))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(
            "GitHub accepted the repository and token. The publish branch will be created on first publish."
                .to_string(),
        );
    }
    if response.status().is_success() {
        Ok("GitHub accepted the repository, branch, and token.".to_string())
    } else {
        Err(format!(
            "GitHub rejected the connection test with status {}. Confirm the token can write repository contents.",
            response.status()
        ))
    }
}

fn validate_github_pages_path(value: Option<&str>) -> Result<(), String> {
    let path = normalize_prefix(value);
    if path.is_empty() || path == "docs" {
        Ok(())
    } else {
        Err("GitHub Pages can publish from the repository root or /docs. Leave Folder path blank or use docs.".to_string())
    }
}

fn github_request(
    client: &reqwest::Client,
    method: reqwest::Method,
    token: &str,
    path: &str,
) -> reqwest::RequestBuilder {
    client
        .request(method, format!("https://api.github.com{path}"))
        .bearer_auth(token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
}

async fn github_get_json(
    client: &reqwest::Client,
    token: &str,
    path: &str,
) -> Result<serde_json::Value, String> {
    let response = github_request(client, reqwest::Method::GET, token, path)
        .send()
        .await
        .map_err(|e| format!("GitHub connection failed: {e}"))?;
    if response.status().is_success() {
        response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Could not read GitHub response: {e}"))
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!(
            "GitHub request failed with status {status}: {body}"
        ))
    }
}

async fn publish_github(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    let token = credential(PublisherProvider::GithubPages)?;
    let repo = validate_repo(config.repo.as_deref())?;
    let branch = required_field("GitHub branch", config.branch.as_deref())?;
    validate_github_pages_path(config.path_prefix.as_deref())?;
    let prefix = normalize_prefix(config.path_prefix.as_deref());
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let files = static_site_files(&output_dir)?;
    let client = http_client()?;
    ensure_github_branch(&client, &token, &repo, &branch).await?;
    let stale_files = previous_github_generated_files(&client, &token, &repo, &branch, &prefix)
        .await
        .unwrap_or_default();
    let mut uploaded = 0usize;
    let mut current_remote_paths = std::collections::HashSet::new();

    for file in files {
        let relative = file
            .strip_prefix(&output_dir)
            .map_err(|e| format!("Could not resolve generated file path: {e}"))?;
        let remote_path = remote_path(&prefix, relative);
        current_remote_paths.insert(remote_path.clone());
        put_github_file(&client, &token, &repo, &branch, &remote_path, &file).await?;
        uploaded += 1;
    }
    let mut deleted = 0usize;
    for remote_path in stale_files {
        if !current_remote_paths.contains(&remote_path) && !preserve_github_path(&remote_path) {
            delete_github_file(&client, &token, &repo, &branch, &remote_path).await?;
            deleted += 1;
        }
    }
    ensure_github_pages(&client, &token, &repo, &branch, &prefix).await?;

    let published_url =
        fallback_url(config, request).or_else(|_| derive_github_pages_url(&repo))?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::GithubPages.as_str().to_string(),
        published_url,
        deployment_id: Some(format!("{branch}:{uploaded}-files")),
        message: format!(
            "Uploaded {uploaded} generated files to GitHub Pages and removed {deleted} stale generated file(s)."
        ),
    })
}

async fn ensure_github_branch(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
) -> Result<(), String> {
    let branch_path = format!("/repos/{repo}/branches/{branch}");
    let response = github_request(client, reqwest::Method::GET, token, &branch_path)
        .send()
        .await
        .map_err(|e| format!("Could not check GitHub branch: {e}"))?;
    if response.status().is_success() {
        return Ok(());
    }
    if response.status() != reqwest::StatusCode::NOT_FOUND {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!(
            "GitHub branch check failed with status {status}: {body}"
        ));
    }

    let repo_info = github_get_json(client, token, &format!("/repos/{repo}")).await?;
    let default_branch = repo_info
        .get("default_branch")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "GitHub did not return a default branch.".to_string())?;
    let default_ref = github_get_json(
        client,
        token,
        &format!("/repos/{repo}/git/ref/heads/{default_branch}"),
    )
    .await?;
    let sha = default_ref
        .get("object")
        .and_then(|object| object.get("sha"))
        .and_then(|value| value.as_str())
        .ok_or_else(|| "GitHub did not return a default branch commit SHA.".to_string())?;
    #[derive(Serialize)]
    struct CreateRef<'a> {
        r#ref: String,
        sha: &'a str,
    }
    let response = github_request(
        client,
        reqwest::Method::POST,
        token,
        &format!("/repos/{repo}/git/refs"),
    )
    .json(&CreateRef {
        r#ref: format!("refs/heads/{branch}"),
        sha,
    })
    .send()
    .await
    .map_err(|e| format!("Could not create GitHub publish branch: {e}"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!(
            "GitHub publish branch creation failed with status {status}: {body}"
        ))
    }
}

async fn previous_github_generated_files(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    prefix: &str,
) -> Result<Vec<String>, String> {
    let manifest_path = remote_path(prefix, Path::new("publish-manifest.json"));
    let Some(contents) = get_github_file(client, token, repo, branch, &manifest_path).await? else {
        return Ok(Vec::new());
    };
    let manifest: serde_json::Value = serde_json::from_slice(&contents)
        .map_err(|e| format!("Could not read previous publish manifest: {e}"))?;
    let generated = manifest
        .get("generated_files")
        .and_then(|value| value.as_array())
        .ok_or_else(|| "Previous publish manifest does not list generated files.".to_string())?;
    Ok(generated
        .iter()
        .filter_map(|value| value.as_str())
        .map(|path| remote_path(prefix, Path::new(path)))
        .collect())
}

async fn get_github_file(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    remote_path: &str,
) -> Result<Option<Vec<u8>>, String> {
    let response = github_request(
        client,
        reqwest::Method::GET,
        token,
        &format!("/repos/{repo}/contents/{remote_path}"),
    )
    .query(&[("ref", branch)])
    .send()
    .await
    .map_err(|e| format!("Could not read GitHub file {remote_path}: {e}"))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!(
            "GitHub file read failed for {remote_path} with status {status}: {body}"
        ));
    }
    #[derive(Deserialize)]
    struct ExistingFile {
        content: String,
        encoding: String,
    }
    let file = response
        .json::<ExistingFile>()
        .await
        .map_err(|e| format!("Could not read GitHub file metadata: {e}"))?;
    if file.encoding != "base64" {
        return Err(format!(
            "GitHub returned unsupported content encoding for {remote_path}."
        ));
    }
    let normalized = file.content.replace(['\n', '\r'], "");
    base64::engine::general_purpose::STANDARD
        .decode(normalized)
        .map(Some)
        .map_err(|e| format!("Could not decode GitHub file {remote_path}: {e}"))
}

fn preserve_github_path(remote_path: &str) -> bool {
    matches!(remote_path.rsplit('/').next(), Some("CNAME" | ".nojekyll"))
}

fn derive_github_pages_url(repo: &str) -> Result<String, String> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err("Could not derive GitHub Pages URL from repository.".to_string());
    }
    if parts[1].eq_ignore_ascii_case(&format!("{}.github.io", parts[0])) {
        validate_public_url(&format!("https://{}/", parts[1]))
    } else {
        validate_public_url(&format!("https://{}.github.io/{}/", parts[0], parts[1]))
    }
}

async fn put_github_file(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    remote_path: &str,
    file: &Path,
) -> Result<(), String> {
    let api = format!("https://api.github.com/repos/{repo}/contents/{remote_path}");
    let existing = github_request(
        client,
        reqwest::Method::GET,
        token,
        &format!("/repos/{repo}/contents/{remote_path}"),
    )
    .query(&[("ref", branch)])
    .send()
    .await
    .map_err(|e| format!("Could not check GitHub file {remote_path}: {e}"))?;
    let existing_status = existing.status();
    let existing_body = existing.text().await.unwrap_or_default();
    let sha = if existing_status.is_success() {
        #[derive(Deserialize)]
        struct ExistingFile {
            sha: Option<String>,
        }
        serde_json::from_str::<ExistingFile>(&existing_body)
            .ok()
            .and_then(|value| value.sha)
    } else if existing_status == reqwest::StatusCode::NOT_FOUND {
        None
    } else {
        return Err(format!(
            "GitHub refused file lookup for {remote_path} with status {existing_status}: {existing_body}"
        ));
    };

    let bytes = std::fs::read(file)
        .map_err(|e| format!("Could not read generated file {}: {e}", file.display()))?;
    #[derive(Serialize)]
    struct PutFile<'a> {
        message: &'a str,
        content: String,
        branch: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        sha: Option<String>,
    }
    let payload = PutFile {
        message: "Publish Civic Desk issue",
        content: base64::engine::general_purpose::STANDARD.encode(bytes),
        branch,
        sha,
    };
    let response = github_request(
        client,
        reqwest::Method::PUT,
        token,
        &api.replace("https://api.github.com", ""),
    )
    .json(&payload)
    .send()
    .await
    .map_err(|e| format!("Could not upload GitHub file {remote_path}: {e}"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!(
            "GitHub upload failed for {remote_path} with status {status}: {body}"
        ))
    }
}

async fn delete_github_file(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    remote_path: &str,
) -> Result<(), String> {
    let existing = github_request(
        client,
        reqwest::Method::GET,
        token,
        &format!("/repos/{repo}/contents/{remote_path}"),
    )
    .query(&[("ref", branch)])
    .send()
    .await
    .map_err(|e| format!("Could not check stale GitHub file {remote_path}: {e}"))?;
    if existing.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(());
    }
    if !existing.status().is_success() {
        let status = existing.status();
        let body = existing.text().await.unwrap_or_default();
        return Err(format!(
            "GitHub refused stale file lookup for {remote_path} with status {status}: {body}"
        ));
    }
    #[derive(Deserialize)]
    struct ExistingFile {
        sha: String,
    }
    let existing_file = existing
        .json::<ExistingFile>()
        .await
        .map_err(|e| format!("Could not read stale GitHub file metadata: {e}"))?;
    #[derive(Serialize)]
    struct DeleteFile<'a> {
        message: &'a str,
        sha: &'a str,
        branch: &'a str,
    }
    let response = github_request(
        client,
        reqwest::Method::DELETE,
        token,
        &format!("/repos/{repo}/contents/{remote_path}"),
    )
    .json(&DeleteFile {
        message: "Remove stale Civic Desk generated file",
        sha: &existing_file.sha,
        branch,
    })
    .send()
    .await
    .map_err(|e| format!("Could not delete stale GitHub file {remote_path}: {e}"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!(
            "GitHub stale file deletion failed for {remote_path} with status {status}: {body}"
        ))
    }
}

async fn ensure_github_pages(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
    branch: &str,
    prefix: &str,
) -> Result<(), String> {
    let pages_path = if prefix == "docs" { "/docs" } else { "/" };
    #[derive(Serialize)]
    struct PagesSource<'a> {
        branch: &'a str,
        path: &'a str,
    }
    #[derive(Serialize)]
    struct PagesPayload<'a> {
        source: PagesSource<'a>,
    }
    let payload = PagesPayload {
        source: PagesSource {
            branch,
            path: pages_path,
        },
    };
    let get = github_request(
        client,
        reqwest::Method::GET,
        token,
        &format!("/repos/{repo}/pages"),
    )
    .send()
    .await
    .map_err(|e| format!("Could not check GitHub Pages settings: {e}"))?;
    let method = if get.status() == reqwest::StatusCode::NOT_FOUND {
        reqwest::Method::POST
    } else if get.status().is_success() {
        reqwest::Method::PUT
    } else {
        let status = get.status();
        let body = get.text().await.unwrap_or_default();
        return Err(format!(
            "GitHub Pages settings check failed with status {status}: {body}"
        ));
    };
    let response = github_request(client, method, token, &format!("/repos/{repo}/pages"))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Could not configure GitHub Pages: {e}"))?;
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!(
            "GitHub Pages configuration failed with status {status}: {body}"
        ))
    }
}

fn static_site_files(output_dir: &Path) -> Result<Vec<PathBuf>, String> {
    fn visit(base: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        for entry in std::fs::read_dir(dir).map_err(|e| format!("Could not read folder: {e}"))? {
            let entry = entry.map_err(|e| format!("Could not read folder item: {e}"))?;
            let path = entry.path();
            if path.is_dir() {
                visit(base, &path, files)?;
            } else {
                let relative = path.strip_prefix(base).unwrap_or(&path);
                if relative == Path::new("site-package.zip") {
                    continue;
                }
                files.push(path);
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    visit(output_dir, output_dir, &mut files)?;
    Ok(files)
}

fn normalize_prefix(value: Option<&str>) -> String {
    value
        .unwrap_or("")
        .trim()
        .trim_matches('/')
        .replace('\\', "/")
}

fn remote_path(prefix: &str, relative: &Path) -> String {
    let relative = relative.to_string_lossy().replace('\\', "/");
    if prefix.is_empty() {
        relative
    } else {
        format!("{prefix}/{relative}")
    }
}

async fn test_cloudflare(config: &PublisherConfig) -> Result<String, String> {
    credential(PublisherProvider::CloudflarePages)?;
    required_field("Cloudflare account ID", config.account_id.as_deref())?;
    required_field("Cloudflare Pages project name", config.site_id.as_deref())?;
    let status = Command::new("npx")
        .args(["--yes", "wrangler@latest", "--version"])
        .status()
        .map_err(|e| format!("Could not run Wrangler through npx: {e}"))?;
    if status.success() {
        Ok("Wrangler is available for Cloudflare Pages deployments.".to_string())
    } else {
        Err("Wrangler did not start successfully through npx.".to_string())
    }
}

async fn publish_cloudflare(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    let token = credential(PublisherProvider::CloudflarePages)?;
    let account_id = required_field("Cloudflare account ID", config.account_id.as_deref())?;
    let project = required_field("Cloudflare Pages project name", config.site_id.as_deref())?;
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let mut command = Command::new("npx");
    command
        .args([
            "--yes",
            "wrangler@latest",
            "pages",
            "deploy",
            output_dir.to_string_lossy().as_ref(),
            "--project-name",
            &project,
            "--branch",
            config.branch.as_deref().unwrap_or("main"),
        ])
        .env("CLOUDFLARE_API_TOKEN", token)
        .env("CLOUDFLARE_ACCOUNT_ID", account_id);
    let output = command
        .output()
        .map_err(|e| format!("Cloudflare Pages deployment could not start: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "Cloudflare Pages deployment failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let published_url = find_first_url(&stdout)
        .or_else(|| request.published_url.clone())
        .or_else(|| config.site_url.clone())
        .ok_or_else(|| "Cloudflare deployed, but no public URL was returned.".to_string())?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::CloudflarePages.as_str().to_string(),
        published_url: validate_public_url(&published_url)?,
        deployment_id: request.deployment_id.clone(),
        message: "Published with Wrangler to Cloudflare Pages.".to_string(),
    })
}

fn find_first_url(text: &str) -> Option<String> {
    let re = regex::Regex::new(r"https://[^\s)]+").ok()?;
    re.find(text)
        .map(|m| m.as_str().trim_end_matches('.').to_string())
}

async fn test_wordpress(config: &PublisherConfig) -> Result<String, String> {
    let password = credential(PublisherProvider::Wordpress)?;
    let site_url = required_field("WordPress site URL", config.site_url.as_deref())?;
    let username = required_field("WordPress username", config.username.as_deref())?;
    let response = http_client()?
        .get(format!(
            "{}/wp-json/wp/v2/users/me",
            site_url.trim_end_matches('/')
        ))
        .basic_auth(username, Some(password))
        .send()
        .await
        .map_err(|e| format!("WordPress connection failed: {e}"))?;
    if response.status().is_success() {
        Ok("WordPress accepted the username and application password.".to_string())
    } else {
        Err(format!(
            "WordPress rejected the connection test with status {}.",
            response.status()
        ))
    }
}

async fn publish_wordpress(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    let password = credential(PublisherProvider::Wordpress)?;
    let site_url = required_field("WordPress site URL", config.site_url.as_deref())?;
    let username = required_field("WordPress username", config.username.as_deref())?;
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let manifest = read_publish_manifest(&output_dir)?;
    let index_html = std::fs::read_to_string(output_dir.join("index.html"))
        .map_err(|e| format!("Could not read generated issue homepage: {e}"))?;
    let index_content = wordpress_content_fragment(&index_html);
    let issue_title = format!("Civic Desk issue {}", manifest.issue_id);
    let issue_page = create_wordpress_page(
        &site_url,
        &username,
        &password,
        &issue_title,
        &index_content,
        None,
    )
    .await?;

    let mut article_count = 0usize;
    for article in &manifest.articles {
        let path = output_dir.join(&article.relative_path);
        let article_html = std::fs::read_to_string(&path).map_err(|e| {
            format!(
                "Could not read generated article page {}: {e}",
                article.relative_path
            )
        })?;
        let content = wordpress_content_fragment(&article_html);
        create_wordpress_page(
            &site_url,
            &username,
            &password,
            &article.title,
            &content,
            issue_page.id,
        )
        .await?;
        article_count += 1;
    }

    Ok(PublisherPublishResult {
        provider: PublisherProvider::Wordpress.as_str().to_string(),
        published_url: validate_public_url(&issue_page.link)?,
        deployment_id: issue_page.id.map(|id| id.to_string()),
        message: format!("Published one WordPress issue page and {article_count} article page(s)."),
    })
}

#[derive(Deserialize)]
struct WordpressPage {
    id: Option<u64>,
    link: String,
}

async fn create_wordpress_page(
    site_url: &str,
    username: &str,
    password: &str,
    title: &str,
    content: &str,
    parent: Option<u64>,
) -> Result<WordpressPage, String> {
    #[derive(Serialize)]
    struct PagePayload<'a> {
        title: &'a str,
        content: &'a str,
        status: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<u64>,
    }
    let response = http_client()?
        .post(format!(
            "{}/wp-json/wp/v2/pages",
            site_url.trim_end_matches('/')
        ))
        .basic_auth(username, Some(password))
        .json(&PagePayload {
            title,
            content,
            status: "publish",
            parent,
        })
        .send()
        .await
        .map_err(|e| format!("WordPress publish failed: {e}"))?;
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!(
            "WordPress publish failed with status {status}: {body}"
        ));
    }
    serde_json::from_str::<WordpressPage>(&body)
        .map_err(|e| format!("Could not read WordPress response: {e}"))
}

fn read_publish_manifest(
    output_dir: &Path,
) -> Result<crate::core::compiler::CompileStaticSiteResult, String> {
    let manifest = std::fs::read_to_string(output_dir.join("publish-manifest.json"))
        .map_err(|e| format!("Could not read publish manifest: {e}"))?;
    serde_json::from_str(&manifest).map_err(|e| format!("Could not parse publish manifest: {e}"))
}

fn wordpress_content_fragment(html: &str) -> String {
    let body = html
        .split_once("<body>")
        .and_then(|(_, rest)| rest.split_once("</body>").map(|(body, _)| body))
        .unwrap_or(html);
    body.replace("<script", "&lt;script")
}

fn publish_assisted(
    config: &PublisherConfig,
    request: &PublisherPublishRequest,
) -> Result<PublisherPublishResult, String> {
    validate_publish_artifacts(&request.output_dir)?;
    let published_url = fallback_url(config, request)?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::Other.as_str().to_string(),
        published_url,
        deployment_id: request.deployment_id.clone(),
        message: "Assisted publish recorded. Share artifacts now point to the public URL."
            .to_string(),
    })
}

pub fn credential_key(provider: &str) -> String {
    format!("publisher:{}", provider)
}

pub fn set_provider_credential(provider: &str, credential: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, &credential_key(provider))
        .map_err(|e| format!("Could not open secure credential storage: {}", e))?;
    entry
        .set_password(credential)
        .map_err(|e| format!("Could not save provider credential: {}", e))
}

pub fn get_provider_credential(provider: &str) -> Result<Option<String>, String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, &credential_key(provider))
        .map_err(|e| format!("Could not open secure credential storage: {}", e))?;
    match entry.get_password() {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Could not read provider credential: {}", e)),
    }
}

pub fn delete_provider_credential(provider: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, &credential_key(provider))
        .map_err(|e| format!("Could not open secure credential storage: {}", e))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Could not delete provider credential: {}", e)),
    }
}

pub fn has_provider_credential(provider: &str) -> bool {
    get_provider_credential(provider)
        .map(|value| value.is_some())
        .unwrap_or(false)
}

pub fn provider_config_setting_key(provider: &str) -> String {
    format!("publisher.config.{}", provider)
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn sanitize_config(config: PublisherConfigInput) -> Result<PublisherConfig, String> {
    let provider = PublisherProvider::from_str(config.provider.trim())
        .ok_or_else(|| "Unsupported publishing provider.".to_string())?;
    let site_url = match config.site_url {
        Some(value) if !value.trim().is_empty() => Some(validate_public_url(&value)?),
        _ => None,
    };
    Ok(PublisherConfig {
        provider: provider.as_str().to_string(),
        display_name: if config.display_name.trim().is_empty() {
            provider.as_str().replace('_', " ")
        } else {
            config.display_name.trim().to_string()
        },
        site_url,
        project_hint: clean_optional(config.project_hint),
        site_id: clean_optional(config.site_id),
        account_id: clean_optional(config.account_id),
        repo: clean_optional(config.repo),
        branch: clean_optional(config.branch).or_else(|| {
            if provider == PublisherProvider::GithubPages {
                Some("gh-pages".to_string())
            } else {
                None
            }
        }),
        path_prefix: clean_optional(config.path_prefix),
        username: clean_optional(config.username),
        has_credential: has_provider_credential(provider.as_str()),
    })
}
