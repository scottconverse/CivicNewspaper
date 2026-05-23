// core/llm.rs
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};

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

pub async fn pull_ollama_model(model: &str) -> Result<reqwest::Response, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    
    #[derive(Debug, Serialize)]
    struct PullRequest<'a> {
        name: &'a str,
        stream: bool,
    }
    
    // We stream the pull progress to display the progress bar in UI
    let resp = client
        .post("http://127.0.0.1:11434/api/pull")
        .json(&PullRequest { name: model, stream: true })
        .send()
        .await?;
        
    if !resp.status().is_success() {
        return Err(format!("Failed to start pulling model: status {}", resp.status()).into());
    }
    
    Ok(resp)
}
