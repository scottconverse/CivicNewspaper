use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const KEYRING_SERVICE: &str = "The Civic Desk Publisher";

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
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherConfig {
    pub provider: String,
    pub display_name: String,
    pub site_url: Option<String>,
    pub project_hint: Option<String>,
    pub has_credential: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherConfigInput {
    pub provider: String,
    pub display_name: String,
    pub site_url: Option<String>,
    pub project_hint: Option<String>,
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
    pub published_url: String,
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
        request: &PublisherPublishRequest,
    ) -> Result<PublisherPublishResult, String>;
}

pub struct GuidedPublisher {
    provider: PublisherProvider,
}

impl GuidedPublisher {
    pub fn new(provider: PublisherProvider) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl Publisher for GuidedPublisher {
    fn validate_config(&self, config: &PublisherConfig) -> Result<(), String> {
        if PublisherProvider::from_str(&config.provider) != Some(self.provider.clone()) {
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

    async fn test_connection(&self, config: &PublisherConfig) -> PublisherTestResult {
        let credential_checked = self.provider.requires_credential();
        match self.validate_config(config) {
            Ok(()) => {
                let message = if config.has_credential {
                    "Connector settings are valid, and a credential is stored securely."
                } else {
                    "Connector settings are valid for guided publishing. No API credential is required for this connector mode."
                };
                PublisherTestResult {
                    provider: self.provider.as_str().to_string(),
                    ok: true,
                    message: message.to_string(),
                    credential_checked,
                }
            }
            Err(message) => PublisherTestResult {
                provider: self.provider.as_str().to_string(),
                ok: false,
                message,
                credential_checked,
            },
        }
    }

    async fn publish_folder(
        &self,
        request: &PublisherPublishRequest,
    ) -> Result<PublisherPublishResult, String> {
        validate_publish_artifacts(&request.output_dir)?;
        validate_public_url(&request.published_url)?;
        Ok(PublisherPublishResult {
            provider: self.provider.as_str().to_string(),
            published_url: request.published_url.trim_end_matches('/').to_string(),
            deployment_id: request.deployment_id.clone(),
            message: "Guided publish recorded. Share artifacts now point to the public URL."
                .to_string(),
        })
    }
}

pub fn publisher_for(provider: &str) -> Result<Box<dyn Publisher>, String> {
    let provider = PublisherProvider::from_str(provider)
        .ok_or_else(|| "Unsupported publishing provider.".to_string())?;
    Ok(Box::new(GuidedPublisher::new(provider)))
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
        project_hint: config
            .project_hint
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
        has_credential: has_provider_credential(provider.as_str()),
    })
}
