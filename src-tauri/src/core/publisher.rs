use async_trait::async_trait;
use base64::Engine;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const KEYRING_SERVICE: &str = "The Civic Desk Publisher";
const USER_AGENT: &str = "The Civic Desk Publisher";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublisherProvider {
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
            PublisherProvider::Netlify => {
                required_field("Netlify site ID", config.site_id.as_deref())?;
            }
            PublisherProvider::GithubPages => {
                validate_repo(config.repo.as_deref())?;
                required_field("GitHub branch", config.branch.as_deref())?;
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

async fn test_github(config: &PublisherConfig) -> Result<String, String> {
    let token = credential(PublisherProvider::GithubPages)?;
    let repo = validate_repo(config.repo.as_deref())?;
    let branch = required_field("GitHub branch", config.branch.as_deref())?;
    let response = http_client()?
        .get(format!(
            "https://api.github.com/repos/{repo}/branches/{branch}"
        ))
        .bearer_auth(token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .map_err(|e| format!("GitHub connection failed: {e}"))?;
    if response.status().is_success() {
        Ok("GitHub accepted the repository, branch, and token.".to_string())
    } else {
        Err(format!(
            "GitHub rejected the connection test with status {}. Confirm the branch exists and the token can write repository contents.",
            response.status()
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
    let prefix = normalize_prefix(config.path_prefix.as_deref());
    let output_dir = validate_publish_artifacts(&request.output_dir)?;
    let files = static_site_files(&output_dir)?;
    let client = http_client()?;
    let mut uploaded = 0usize;

    for file in files {
        let relative = file
            .strip_prefix(&output_dir)
            .map_err(|e| format!("Could not resolve generated file path: {e}"))?;
        let remote_path = remote_path(&prefix, relative);
        put_github_file(&client, &token, &repo, &branch, &remote_path, &file).await?;
        uploaded += 1;
    }

    let published_url =
        fallback_url(config, request).or_else(|_| derive_github_pages_url(&repo))?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::GithubPages.as_str().to_string(),
        published_url,
        deployment_id: Some(format!("{branch}:{uploaded}-files")),
        message: format!("Uploaded {uploaded} generated files to GitHub Pages."),
    })
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
    let existing = client
        .get(&api)
        .bearer_auth(token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
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
    let response = client
        .put(api)
        .bearer_auth(token)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
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
    let markdown_path = output_dir.join("substack.md");
    let markdown = std::fs::read_to_string(&markdown_path)
        .map_err(|e| format!("Could not read generated article package: {e}"))?;
    let title = markdown
        .lines()
        .find_map(|line| line.strip_prefix("# "))
        .unwrap_or("Civic Desk issue")
        .trim()
        .to_string();
    let parser = Parser::new_ext(&markdown, Options::all());
    let mut content = String::new();
    html::push_html(&mut content, parser);
    #[derive(Serialize)]
    struct PostPayload {
        title: String,
        content: String,
        status: &'static str,
    }
    let response = http_client()?
        .post(format!(
            "{}/wp-json/wp/v2/posts",
            site_url.trim_end_matches('/')
        ))
        .basic_auth(username, Some(password))
        .json(&PostPayload {
            title,
            content,
            status: "publish",
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
    #[derive(Deserialize)]
    struct WordpressPost {
        id: Option<u64>,
        link: Option<String>,
    }
    let post: WordpressPost = serde_json::from_str(&body)
        .map_err(|e| format!("Could not read WordPress response: {e}"))?;
    let published_url = post
        .link
        .or_else(|| request.published_url.clone())
        .ok_or_else(|| "WordPress did not return a public post URL.".to_string())?;
    Ok(PublisherPublishResult {
        provider: PublisherProvider::Wordpress.as_str().to_string(),
        published_url: validate_public_url(&published_url)?,
        deployment_id: post.id.map(|id| id.to_string()),
        message: "Published the issue as a WordPress post.".to_string(),
    })
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
