// core/llm.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandChild;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}

pub async fn check_ollama_status() -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    match client.get("http://127.0.0.1:11434/api/tags").send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

pub async fn call_local_ollama(
    model: &str,
    prompt: &str,
    system: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(60)) // Long timeout for generation on slower CPU
        .build()?;

    let req_payload = OllamaGenerateRequest {
        model,
        prompt,
        system,
        stream: false,
    };

    let resp = client
        .post("http://127.0.0.1:11434/api/generate")
        .json(&req_payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("Ollama API returned error: {}", err_text).into());
    }

    let body: OllamaGenerateResponse = resp.json().await?;
    Ok(body.response)
}

#[async_trait::async_trait]
pub trait LlmClient: Send + Sync {
    async fn call(&self, model: &str, prompt: &str, system: &str) -> Result<String, String>;
}

pub struct OllamaClient;

#[async_trait::async_trait]
impl LlmClient for OllamaClient {
    async fn call(&self, model: &str, prompt: &str, system: &str) -> Result<String, String> {
        call_local_ollama(model, prompt, system)
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn pull_ollama_model(
    model: &str,
) -> Result<reqwest::Response, Box<dyn Error + Send + Sync>> {
    let client = Client::new();

    #[derive(Debug, Serialize)]
    struct PullRequest<'a> {
        name: &'a str,
        stream: bool,
    }

    // We stream the pull progress to display the progress bar in UI
    let resp = client
        .post("http://127.0.0.1:11434/api/pull")
        .json(&PullRequest {
            name: model,
            stream: true,
        })
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("Failed to start pulling model: status {}", resp.status()).into());
    }

    Ok(resp)
}

pub struct OllamaSidecar {
    pub(crate) child: Mutex<Option<CommandChild>>,
}

impl OllamaSidecar {
    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }

    pub fn start(&self, app: &AppHandle) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if guard.is_some() {
            return Ok(());
        }

        let binary_name = if cfg!(test) {
            "test-ollama-fixture"
        } else {
            "ollama"
        };

        let sidecar_command = app
            .shell()
            .sidecar(binary_name)
            .map_err(|e| format!("Sidecar error: {}", e))?
            .args(["serve"]);

        let (_rx, child_proc) = sidecar_command
            .spawn()
            .map_err(|e| format!("Spawn error: {}", e))?;

        *guard = Some(child_proc);
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if let Some(child_proc) = guard.take() {
            child_proc
                .kill()
                .map_err(|e| format!("Kill error: {}", e))?;
        }
        Ok(())
    }
}

impl Drop for OllamaSidecar {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
