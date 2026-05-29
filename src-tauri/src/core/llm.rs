// core/llm.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandChild;
#[cfg(not(test))]
use tauri_plugin_shell::ShellExt;
use tokio::sync::watch;

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

/// Rewrites `text` into plain language at an 8th-grade level in `draft_format`.
/// Pure business logic: takes an injected `LlmClient` so it can be tested with a
/// fake client without a Tauri `AppHandle`.
pub async fn plain_language_rewrite(
    llm_client: &std::sync::Arc<dyn LlmClient>,
    model: &str,
    text: &str,
    draft_format: &str,
) -> Result<String, String> {
    let system = format!("You are a plain language summarizer. Rewrite the following text to an 8th-grade reading level in the '{}' format. Remove jargon. Keep the core facts.", draft_format);
    let prompt = format!("Rewrite this:\n{}", text);
    llm_client.call(model, &prompt, &system).await
}

/// Per-model cancellation senders for in-flight `run_ollama_pull` tasks. Keyed
/// by model id so cancelling one model's pull never disturbs another's (the
/// per-model isolation guarantee). At most one pull per model is in flight: a
/// duplicate same-model pull is rejected before insert, so an entry is never
/// overwritten and a finishing pull never removes a sibling's sender. Lives
/// here, decoupled from any Tauri `AppHandle`, so the pull/cancel behavior is
/// testable cross-platform.
pub(crate) static CANCEL_PULL_MAP: LazyLock<Mutex<HashMap<String, watch::Sender<bool>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Signals the in-flight pull for `model` to cancel, if one is running.
pub fn cancel_pull(model: &str) {
    let map = CANCEL_PULL_MAP.lock().unwrap();
    if let Some(tx) = map.get(model) {
        let _ = tx.send(true);
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PullProgress {
    pub model: String,
    pub status: String,
    pub completed: Option<f64>,
    pub total: Option<f64>,
}

/// Sink for pull progress events. Production wires this to Tauri's event
/// emitter; tests pass a no-op (or recording) implementation so the streaming
/// loop can be exercised without a Tauri `AppHandle`.
pub trait PullProgressSink: Send + Sync + 'static {
    fn progress(&self, payload: PullProgress);
    fn complete(&self);
    fn error(&self, message: String);
}

/// Streams an `ollama pull` for `model_id`, reporting progress through `sink`.
/// Registers a cancellation channel in [`CANCEL_PULL_MAP`] keyed by model id and
/// removes it when the stream finishes, errors, or is cancelled. Returns `Err`
/// (and leaves nothing registered) if the pull cannot be started, or if a pull
/// for the same model is already in flight.
pub async fn run_ollama_pull(
    model_id: String,
    base_url: &str,
    sink: std::sync::Arc<dyn PullProgressSink>,
) -> Result<(), String> {
    let model = model_id.clone();

    let (tx, mut rx) = watch::channel(false);
    {
        let mut map = CANCEL_PULL_MAP.lock().unwrap();
        if map.contains_key(&model_id) {
            return Err(format!(
                "A pull for model '{}' is already in progress",
                model_id
            ));
        }
        map.insert(model_id.clone(), tx);
    }

    // Trim a trailing slash so a base_url like "http://host:11434/" doesn't
    // produce a double-slashed "http://host:11434//api/pull".
    let base_url = base_url.trim_end_matches('/');
    let client = Client::new();
    let resp_res = client
        .post(format!("{}/api/pull", base_url))
        .json(&serde_json::json!({ "name": model_id, "stream": true }))
        .send()
        .await;

    let mut resp = match resp_res {
        Ok(r) => r,
        Err(e) => {
            let mut map = CANCEL_PULL_MAP.lock().unwrap();
            map.remove(&model);
            return Err(e.to_string());
        }
    };

    if !resp.status().is_success() {
        let mut map = CANCEL_PULL_MAP.lock().unwrap();
        map.remove(&model);
        return Err(format!(
            "Failed to start pulling model: status {}",
            resp.status()
        ));
    }

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::select! {
                _ = rx.changed() => {
                    if *rx.borrow() {
                        sink.progress(PullProgress {
                            model: model.clone(),
                            status: "cancelled".to_string(),
                            completed: None,
                            total: None,
                        });
                        break;
                    }
                }
                chunk_res = resp.chunk() => {
                    match chunk_res {
                        Ok(Some(chunk)) => {
                            let text = String::from_utf8_lossy(&chunk);
                            for line in text.lines() {
                                if line.trim().is_empty() {
                                    continue;
                                }
                                #[derive(Deserialize)]
                                struct PullMsg {
                                    status: String,
                                    completed: Option<f64>,
                                    total: Option<f64>,
                                }
                                if let Ok(msg) = serde_json::from_str::<PullMsg>(line) {
                                    sink.progress(PullProgress {
                                        model: model.clone(),
                                        status: msg.status,
                                        completed: msg.completed,
                                        total: msg.total,
                                    });
                                }
                            }
                        }
                        Ok(None) => {
                            sink.complete();
                            break;
                        }
                        Err(e) => {
                            sink.error(e.to_string());
                            break;
                        }
                    }
                }
            }
        }

        let mut map = CANCEL_PULL_MAP.lock().unwrap();
        map.remove(&model);
    });

    Ok(())
}

#[allow(dead_code)]
pub enum SidecarChild {
    Tauri(CommandChild),
    #[cfg(test)]
    Std(std::process::Child),
}

impl SidecarChild {
    #[allow(dead_code)]
    pub fn pid(&self) -> u32 {
        match self {
            Self::Tauri(c) => c.pid(),
            #[cfg(test)]
            Self::Std(c) => c.id(),
        }
    }
}

pub struct OllamaSidecar {
    pub(crate) child: Mutex<Option<SidecarChild>>,
}

impl OllamaSidecar {
    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }

    /// Returns true if something is already listening on the Ollama port (11434).
    pub(crate) fn ollama_port_in_use() -> bool {
        Self::port_in_use("127.0.0.1:11434")
    }

    /// Returns true if a TCP listener is reachable at `addr`. Extracted and
    /// parameterized so the collision-skip behavior can be tested cross-platform
    /// against an OS-assigned ephemeral port — without a Tauri app handle and
    /// without depending on whether real Ollama happens to occupy 11434.
    pub(crate) fn port_in_use(addr: &str) -> bool {
        match addr.parse::<std::net::SocketAddr>() {
            Ok(socket) => {
                std::net::TcpStream::connect_timeout(&socket, std::time::Duration::from_millis(50))
                    .is_ok()
            }
            Err(_) => false,
        }
    }

    /// Decides whether a process — identified by its `name` and full `cmd` line —
    /// is an orphaned `ollama serve` the startup sweep should reap. Extracted as a
    /// pure function so the matching rule is unit-testable with synthetic inputs
    /// without enumerating real processes (which would be non-deterministic and,
    /// worse, could match a developer's intentional `ollama serve`). A process
    /// qualifies only when it both references ollama (by binary name or command)
    /// and is running the `serve` subcommand.
    pub(crate) fn is_orphan_ollama_serve(name: &str, cmd: &str) -> bool {
        (name.contains("ollama") || cmd.contains("ollama")) && cmd.contains("serve")
    }

    pub fn start<R: tauri::Runtime>(
        &self,
        app: &AppHandle<R>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Port 11434 collision check: graceful coexistence
        if Self::ollama_port_in_use() {
            return Ok(());
        }

        // Startup sweep for orphan ollama processes
        let sys = sysinfo::System::new_all();
        for process in sys.processes().values() {
            let name = process.name();
            let cmd = process.cmd().join(" ");
            if Self::is_orphan_ollama_serve(name, &cmd) {
                let _ = process.kill();
            }
        }

        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if guard.is_some() {
            return Ok(());
        }

        #[cfg(test)]
        {
            let _app = app;
            *guard = Some(Self::spawn_test_fixture()?);
            Ok(())
        }

        #[cfg(not(test))]
        {
            let sidecar_command = app
                .shell()
                .sidecar("ollama")
                .map_err(|e| format!("Sidecar error: {}", e))?
                .args(["serve"]);

            let (_rx, child_proc) = sidecar_command
                .spawn()
                .map_err(|e| format!("Spawn error: {}", e))?;

            *guard = Some(SidecarChild::Tauri(child_proc));
            Ok(())
        }
    }

    /// Spawns the bundled test fixture binary (a stand-in for the real `ollama`
    /// sidecar) and wraps it in a `SidecarChild`. Shared by `start()`'s test
    /// branch and `start_for_test()` so the platform-specific binary selection
    /// lives in one place.
    #[cfg(test)]
    fn spawn_test_fixture() -> Result<SidecarChild, Box<dyn Error + Send + Sync>> {
        let mut base_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base_path.push("tests");
        base_path.push("fixtures");

        let binary_name = if cfg!(target_os = "windows") {
            "test-ollama-fixture-x86_64-pc-windows-msvc.exe"
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "test-ollama-fixture-aarch64-apple-darwin"
            } else {
                "test-ollama-fixture-x86_64-apple-darwin"
            }
        } else {
            "test-ollama-fixture-x86_64-unknown-linux-gnu"
        };
        base_path.push(binary_name);

        let child_proc = std::process::Command::new(base_path)
            .arg("serve")
            .spawn()
            .map_err(|e| format!("Spawn error: {}", e))?;

        Ok(SidecarChild::Std(child_proc))
    }

    /// Test-only mirror of `start()`'s control flow with an injectable address
    /// to probe for collisions. Spawns the bundled test fixture in place of the
    /// real `ollama` sidecar, so the spawn / skip / drop behavior can be verified
    /// cross-platform (including Windows) without constructing a Tauri
    /// `AppHandle` — which `start()` only needs for the production shell-plugin
    /// sidecar lookup.
    ///
    /// Deliberately omits the orphan-process sweep that `start()` performs so
    /// running the suite never kills a developer's real local `ollama serve`.
    #[cfg(test)]
    pub(crate) fn start_for_test(
        &self,
        probe_addr: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if Self::port_in_use(probe_addr) {
            return Ok(());
        }
        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if guard.is_some() {
            return Ok(());
        }
        *guard = Some(Self::spawn_test_fixture()?);
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if let Some(child_proc) = guard.take() {
            match child_proc {
                SidecarChild::Tauri(c) => {
                    c.kill().map_err(|e| format!("Kill error: {}", e))?;
                }
                #[cfg(test)]
                SidecarChild::Std(mut c) => {
                    c.kill().map_err(|e| format!("Kill error: {}", e))?;
                }
            }
        }
        Ok(())
    }
}

impl Drop for OllamaSidecar {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
