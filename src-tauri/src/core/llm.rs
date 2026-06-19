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

/// One line of a streaming `/api/generate` response. Ollama emits newline-
/// delimited JSON objects; each carries an incremental `response` token and the
/// final object has `done: true` (and may also carry an `error`).
#[derive(Debug, Deserialize)]
struct OllamaStreamChunk {
    #[serde(default)]
    response: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    error: Option<String>,
}

/// Default per-generation timeout. CPU-only inference of a multi-paragraph draft
/// on a 7-9B model routinely exceeds a minute, so this is deliberately generous
/// (ENG-M4/QA-M4). Override at runtime with `CIVICNEWS_LLM_TIMEOUT_SECS` (set it
/// to `0` to disable the timeout entirely in favor of a cancel control).
const DEFAULT_LLM_TIMEOUT_SECS: u64 = 600;

/// Resolve the generation timeout: `CIVICNEWS_LLM_TIMEOUT_SECS` if set and
/// parseable (`0` => no timeout), otherwise [`DEFAULT_LLM_TIMEOUT_SECS`].
fn generation_timeout() -> Option<Duration> {
    match std::env::var("CIVICNEWS_LLM_TIMEOUT_SECS") {
        Ok(v) => match v.trim().parse::<u64>() {
            Ok(0) => None,
            Ok(secs) => Some(Duration::from_secs(secs)),
            Err(_) => Some(Duration::from_secs(DEFAULT_LLM_TIMEOUT_SECS)),
        },
        Err(_) => Some(Duration::from_secs(DEFAULT_LLM_TIMEOUT_SECS)),
    }
}

/// A user-surfaceable error from a local LLM generation. The timeout variant is
/// distinguished so the UI can say "this is just slow on CPU" rather than
/// reporting a generic failure (ENG-M4/QA-M4).
#[derive(Debug)]
pub enum LlmError {
    /// Generation exceeded the configured timeout.
    Timeout(u64),
    /// Ollama is unreachable (sidecar down / connection refused).
    Unreachable(String),
    /// Ollama returned a non-success HTTP status (e.g. model not installed).
    Api(String),
    /// Transport / decode error while streaming the response.
    Stream(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::Timeout(secs) => write!(
                f,
                "The AI took longer than {}s to respond. Local generation on CPU can be slow — try a shorter format or a smaller model, or raise CIVICNEWS_LLM_TIMEOUT_SECS.",
                secs
            ),
            LlmError::Unreachable(e) => write!(
                f,
                "Could not reach the local AI (Ollama). Make sure the AI is running. ({})",
                e
            ),
            LlmError::Api(e) => write!(f, "The local AI returned an error: {}", e),
            LlmError::Stream(e) => write!(f, "The AI response stream failed: {}", e),
        }
    }
}

impl std::error::Error for LlmError {}

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

/// Stream a generation from the local Ollama, accumulating tokens as they
/// arrive. `on_token` is invoked with each incremental chunk so a caller can
/// surface partial output to the UI; pass a no-op closure if you only want the
/// final text.
///
/// ENG-M4/QA-M4: switched from a fixed 60s non-streaming call to a STREAMING
/// call with a generous, configurable timeout that bounds the *whole* stream.
/// Timeouts are reported distinctly from other failures (see [`LlmError`]).
pub async fn call_local_ollama_streaming(
    model: &str,
    prompt: &str,
    system: &str,
    mut on_token: impl FnMut(&str),
) -> Result<String, LlmError> {
    // No reqwest client-level timeout: we bound the whole stream with our own
    // tokio timeout below so a slow-but-progressing CPU generation isn't killed
    // by reqwest's idle/read timeout.
    let client = Client::builder()
        .build()
        .map_err(|e| LlmError::Unreachable(e.to_string()))?;

    let req_payload = OllamaGenerateRequest {
        model,
        prompt,
        system,
        stream: true,
    };

    let timeout = generation_timeout();

    let fut = async {
        let resp = client
            .post("http://127.0.0.1:11434/api/generate")
            .json(&req_payload)
            .send()
            .await
            .map_err(|e| LlmError::Unreachable(e.to_string()))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(LlmError::Api(err_text));
        }

        let mut resp = resp;
        let mut accumulated = String::new();
        let mut buf = String::new();

        // Ollama streams newline-delimited JSON objects. Chunks can split a line,
        // so buffer until we see a newline before parsing.
        while let Some(chunk) = resp
            .chunk()
            .await
            .map_err(|e| LlmError::Stream(e.to_string()))?
        {
            buf.push_str(&String::from_utf8_lossy(&chunk));
            while let Some(nl) = buf.find('\n') {
                let line = buf[..nl].trim().to_string();
                buf.drain(..=nl);
                if line.is_empty() {
                    continue;
                }
                if let Ok(parsed) = serde_json::from_str::<OllamaStreamChunk>(&line) {
                    if let Some(err) = parsed.error {
                        return Err(LlmError::Api(err));
                    }
                    if !parsed.response.is_empty() {
                        on_token(&parsed.response);
                        accumulated.push_str(&parsed.response);
                    }
                    if parsed.done {
                        return Ok(accumulated);
                    }
                }
            }
        }

        // Stream ended without an explicit done:true — parse any trailing line.
        let tail = buf.trim();
        if !tail.is_empty() {
            if let Ok(parsed) = serde_json::from_str::<OllamaStreamChunk>(tail) {
                if let Some(err) = parsed.error {
                    return Err(LlmError::Api(err));
                }
                if !parsed.response.is_empty() {
                    on_token(&parsed.response);
                    accumulated.push_str(&parsed.response);
                }
            }
        }
        Ok(accumulated)
    };

    match timeout {
        Some(d) => match tokio::time::timeout(d, fut).await {
            Ok(res) => res,
            Err(_) => Err(LlmError::Timeout(d.as_secs())),
        },
        None => fut.await,
    }
}

pub async fn call_local_ollama(
    model: &str,
    prompt: &str,
    system: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    call_local_ollama_streaming(model, prompt, system, |_| {})
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
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
    /// The default loopback address Ollama listens on. `start()` probes this to
    /// decide whether to coexist with an already-running instance.
    const OLLAMA_LOCAL_ADDR: &'static str = "127.0.0.1:11434";

    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
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

    /// Shared, testable control flow for bringing up the sidecar. The only
    /// production-vs-test divergence is the `spawn` closure: production spawns the
    /// shell-plugin `ollama` sidecar (which needs an `AppHandle`), tests spawn the
    /// bundled fixture. Everything else lives here so it is exercised identically
    /// by the shipping `start()` and by `start_for_test`.
    ///
    /// ENG-004: we deliberately do **not** sweep/kill other `ollama serve`
    /// processes. Coexistence with a user-managed Ollama is delivered solely by
    /// the port-collision early-return below — a previous version enumerated
    /// processes and force-killed anything matching `ollama ... serve`, which
    /// could reap a user's deliberate instance running on a non-default port (or
    /// one still mid-startup). The upside of reaping a truly-orphaned child is
    /// small next to the cost of killing a process we did not spawn.
    fn start_internal(
        &self,
        probe_addr: &str,
        spawn: impl FnOnce() -> Result<SidecarChild, Box<dyn Error + Send + Sync>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // If something is already listening on the port, attach to it rather than
        // starting anything new.
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
        *guard = Some(spawn()?);
        Ok(())
    }

    pub fn start<R: tauri::Runtime>(
        &self,
        app: &AppHandle<R>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        #[cfg(test)]
        {
            let _app = app;
            self.start_internal(Self::OLLAMA_LOCAL_ADDR, Self::spawn_test_fixture)
        }

        #[cfg(not(test))]
        {
            // The shell-plugin spawn is the one branch with no unit coverage — it
            // needs a real Tauri runtime. All of the testable control flow lives
            // in start_internal, verified cross-platform via start_for_test
            // (TEST-002).
            self.start_internal(Self::OLLAMA_LOCAL_ADDR, || {
                let sidecar_command = app
                    .shell()
                    .sidecar("ollama")
                    .map_err(|e| format!("Sidecar error: {}", e))?
                    .args(["serve"]);

                let (_rx, child_proc) = sidecar_command
                    .spawn()
                    .map_err(|e| format!("Spawn error: {}", e))?;

                Ok(SidecarChild::Tauri(child_proc))
            })
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

    /// Test-only entry that drives the exact same control flow as `start()` (both
    /// route through `start_internal`) but takes an injectable probe address, so
    /// the skip / spawn / already-running behavior is verified cross-platform
    /// (including Windows) without constructing a Tauri `AppHandle` — which
    /// `start()` only needs for the production shell-plugin sidecar lookup.
    ///
    /// Post-ENG-004 there is no orphan sweep, so this is now a faithful mirror of
    /// `start()`'s entire testable path; the only thing it does not exercise is
    /// the production shell-plugin spawn, which requires a real Tauri runtime.
    #[cfg(test)]
    pub(crate) fn start_for_test(
        &self,
        probe_addr: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.start_internal(probe_addr, Self::spawn_test_fixture)
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut guard = self
            .child
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        Self::kill_guarded(guard.take())
    }

    /// Best-effort, non-blocking stop for use inside the panic hook (ENG-Min2).
    ///
    /// The panic hook runs `stop()` while the process is unwinding. If a panic
    /// ever occurred while the `child` mutex was already held, a blocking
    /// `lock()` inside the hook would deadlock (or, with a poisoned lock, hang
    /// the very crash-logging the hook exists to perform). `try_lock` makes the
    /// hook give up rather than block: failing to reap the child is far less bad
    /// than a hung panic handler. Acquiring a poisoned lock is also handled (we
    /// reap through the poison).
    pub fn stop_best_effort(&self) {
        match self.child.try_lock() {
            Ok(mut guard) => {
                let _ = Self::kill_guarded(guard.take());
            }
            Err(std::sync::TryLockError::Poisoned(poisoned)) => {
                let mut guard = poisoned.into_inner();
                let _ = Self::kill_guarded(guard.take());
            }
            Err(std::sync::TryLockError::WouldBlock) => {
                // Lock held elsewhere; skip rather than block the panic hook.
            }
        }
    }

    fn kill_guarded(child: Option<SidecarChild>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(child_proc) = child {
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
