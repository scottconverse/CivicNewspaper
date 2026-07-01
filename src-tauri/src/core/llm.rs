// core/llm.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_shell::process::CommandChild;
#[cfg(not(test))]
use tauri_plugin_shell::ShellExt;
use tokio::sync::watch;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<serde_json::Value>,
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
const RUNTIME_DOWNLOAD_CONNECT_TIMEOUT_SECS: u64 = 30;
const RUNTIME_DOWNLOAD_IDLE_TIMEOUT_SECS: u64 = 120;
const DEFAULT_OLLAMA_BASE_URL: &str = "http://127.0.0.1:11434";

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

pub fn ollama_base_url() -> String {
    std::env::var("CIVICNEWS_OLLAMA_BASE_URL")
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| {
            value.starts_with("http://127.0.0.1:")
                || value.starts_with("http://localhost:")
                || value.starts_with("http://[::1]:")
        })
        .unwrap_or_else(|| DEFAULT_OLLAMA_BASE_URL.to_string())
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
                "The AI took longer than {}s to respond. Local generation on CPU can be slow - try a shorter format or a smaller model, or raise CIVICNEWS_LLM_TIMEOUT_SECS.",
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

    match client
        .get(format!("{}/api/tags", ollama_base_url()))
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// Outcome of feeding one buffered NDJSON line to the streaming parser. Pulling
/// this out of the network loop makes the fragile line-by-line parsing logic a
/// pure, unit-testable function (NEW-Nit1 / ENG-Tests).
#[derive(Debug, PartialEq)]
pub(crate) enum StreamLineOutcome {
    /// A token to append to the accumulated output.
    Token(String),
    /// The stream's terminal `done: true` object was seen — stop reading.
    Done,
    /// An `error` field was present mid-stream — abort with this message.
    Error(String),
    /// A blank/unparseable line that carries nothing — skip it.
    Skip,
}

/// Parse a single (already newline-stripped) NDJSON line from Ollama's
/// `/api/generate` stream into a [`StreamLineOutcome`]. Pure: no I/O, no state.
/// Blank lines and lines that don't parse as a chunk are [`StreamLineOutcome::Skip`]
/// (matching the original loop's permissive behavior). `error` takes precedence
/// over `done`, and an empty `response` on a non-done line is skipped.
pub(crate) fn parse_stream_line(line: &str) -> StreamLineOutcome {
    let line = line.trim();
    if line.is_empty() {
        return StreamLineOutcome::Skip;
    }
    match serde_json::from_str::<OllamaStreamChunk>(line) {
        Ok(parsed) => {
            if let Some(err) = parsed.error {
                return StreamLineOutcome::Error(err);
            }
            if !parsed.response.is_empty() {
                // A token may co-occur with done:true; emit the token and let the
                // caller append it before checking `done` at the call site. To keep
                // this a single-outcome function we prioritize the token here and
                // rely on a following Done line; Ollama emits the final token and
                // the done marker such that the accumulated text is complete either
                // way (the terminal object's `response` is empty in practice).
                return StreamLineOutcome::Token(parsed.response);
            }
            if parsed.done {
                return StreamLineOutcome::Done;
            }
            StreamLineOutcome::Skip
        }
        Err(_) => StreamLineOutcome::Skip,
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
    call_local_ollama_streaming_with_format(model, prompt, system, None, &mut on_token).await
}

pub async fn call_local_ollama_json(
    model: &str,
    prompt: &str,
    system: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    call_local_ollama_streaming_with_format(
        model,
        prompt,
        system,
        Some(serde_json::json!("json")),
        &mut |_| {},
    )
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
}

async fn call_local_ollama_streaming_with_format(
    model: &str,
    prompt: &str,
    system: &str,
    format: Option<serde_json::Value>,
    on_token: &mut impl FnMut(&str),
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
        format,
    };

    let timeout = generation_timeout();

    let fut = async {
        let resp = client
            .post(format!("{}/api/generate", ollama_base_url()))
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
                let line = buf[..nl].to_string();
                buf.drain(..=nl);
                match parse_stream_line(&line) {
                    StreamLineOutcome::Token(t) => {
                        on_token(&t);
                        accumulated.push_str(&t);
                    }
                    StreamLineOutcome::Done => return Ok(accumulated),
                    StreamLineOutcome::Error(err) => return Err(LlmError::Api(err)),
                    StreamLineOutcome::Skip => {}
                }
            }
        }

        // Stream ended without an explicit done:true — parse any trailing line.
        match parse_stream_line(&buf) {
            StreamLineOutcome::Token(t) => {
                on_token(&t);
                accumulated.push_str(&t);
            }
            StreamLineOutcome::Error(err) => return Err(LlmError::Api(err)),
            StreamLineOutcome::Done | StreamLineOutcome::Skip => {}
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

    async fn call_json(&self, model: &str, prompt: &str, system: &str) -> Result<String, String> {
        self.call(model, prompt, system).await
    }

    /// Typed variant of [`call`](LlmClient::call) that preserves the [`LlmError`]
    /// variant up to the caller, so an HTTP boundary can classify a timeout
    /// (`LlmError::Timeout` → 504) distinctly from other failures (→ 503)
    /// WITHOUT substring-matching the Display string (ENG-Nit-R1). The default
    /// implementation falls back to [`call`](LlmClient::call) and wraps any
    /// error as [`LlmError::Api`]; the real Ollama client overrides it to carry
    /// the genuine variant.
    async fn call_typed(
        &self,
        model: &str,
        prompt: &str,
        system: &str,
    ) -> Result<String, LlmError> {
        self.call(model, prompt, system)
            .await
            .map_err(LlmError::Api)
    }
}

pub struct OllamaClient;

#[async_trait::async_trait]
impl LlmClient for OllamaClient {
    async fn call(&self, model: &str, prompt: &str, system: &str) -> Result<String, String> {
        call_local_ollama(model, prompt, system)
            .await
            .map_err(|e| e.to_string())
    }

    async fn call_json(&self, model: &str, prompt: &str, system: &str) -> Result<String, String> {
        call_local_ollama_json(model, prompt, system)
            .await
            .map_err(|e| e.to_string())
    }

    async fn call_typed(
        &self,
        model: &str,
        prompt: &str,
        system: &str,
    ) -> Result<String, LlmError> {
        call_local_ollama_streaming(model, prompt, system, |_| {}).await
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

pub fn build_press_freedom_review_prompt(
    title: &str,
    content: &str,
    draft_format: &str,
    evidence_context: &str,
) -> (String, String) {
    let system = "You are a press-freedom and newsroom legal-risk advisor for a local editor. You are not the publisher, not a lawyer, and not a censor. Give practical risk spotting, verification paths, and wording options. Never say the story must be published, must be killed, or is legally approved. Never hide information from the editor. The human editor always decides what to investigate, edit, hold, or publish.".to_string();
    let prompt = format!(
        "Run an advisory press-freedom / legal-risk review on this draft.\n\n\
Return Markdown with these sections:\n\
1. Editorial summary\n\
2. Legal-risk flags to consider\n\
3. Press-freedom / public-interest factors\n\
4. Paragraph-level notes\n\
5. Verification tasks\n\
6. Safer wording options\n\
7. Unknowns or ask-a-lawyer questions\n\n\
Check for issues such as defamation risk, public official/public figure/private person status, opinion vs fact, fair-report privilege, privacy and identifying details, minors, active legal matters, prior-restraint threats, records-access issues, anti-SLAPP/state-law uncertainty, and whether claims are supported by sources. If evidence is missing, say what to verify. Do not make a publish/no-publish recommendation.\n\n\
Draft format: {draft_format}\n\
Title: {title}\n\n\
Draft:\n{content}\n\n\
Linked evidence:\n{evidence_context}",
    );
    (prompt, system)
}

pub async fn press_freedom_legal_review(
    llm_client: &std::sync::Arc<dyn LlmClient>,
    model: &str,
    title: &str,
    content: &str,
    draft_format: &str,
    evidence_context: &str,
) -> Result<String, String> {
    let (prompt, system) =
        build_press_freedom_review_prompt(title, content, draft_format, evidence_context);
    llm_client.call(model, &prompt, &system).await
}

#[cfg(test)]
mod press_freedom_review_tests {
    use super::build_press_freedom_review_prompt;

    #[test]
    fn prompt_is_advisory_and_never_a_publish_veto() {
        let (prompt, system) = build_press_freedom_review_prompt(
            "Council approves contract",
            "The council approved a contract.",
            "watch",
            "Evidence ID: 1\nURL: https://example.test\nExcerpt: agenda item",
        );

        let combined = format!("{system}\n{prompt}").to_lowercase();
        assert!(combined.contains("advisory"));
        assert!(combined.contains("never say the story must be published"));
        assert!(combined.contains("never hide information from the editor"));
        assert!(combined.contains("do not make a publish/no-publish recommendation"));
        assert!(combined.contains("verification tasks"));
        assert!(combined.contains("safer wording options"));
    }
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
    Std(Child),
}

impl SidecarChild {
    #[allow(dead_code)]
    pub fn pid(&self) -> u32 {
        match self {
            Self::Tauri(c) => c.pid(),
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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
                SidecarChild::Std(mut c) => {
                    c.kill().map_err(|e| format!("Kill error: {}", e))?;
                    c.wait().map_err(|e| format!("Wait error: {}", e))?;
                }
            }
        }
        Ok(())
    }
}

pub const OLLAMA_RUNTIME_VERSION: &str = "v0.30.11";
pub const OLLAMA_WINDOWS_AMD64_URL: &str =
    "https://github.com/ollama/ollama/releases/download/v0.30.11/ollama-windows-amd64.zip";
pub const OLLAMA_WINDOWS_AMD64_SHA256: &str =
    "43d534c10040ea676c99af19836377a315daa8cb3bb6c3d9d609b4c23dd37b88";

#[derive(Debug, Serialize, Clone)]
pub struct RuntimeInstallProgress {
    pub stage: String,
    pub message: String,
    pub completed: Option<u64>,
    pub total: Option<u64>,
}

pub trait RuntimeInstallSink: Send + Sync + 'static {
    fn progress(&self, payload: RuntimeInstallProgress);
}

fn runtime_base_dir<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    super::app_paths::app_data_dir(app).map(|dir| dir.join("ollama-runtime"))
}

pub(crate) fn find_downloaded_ollama_exe_in_base(
    base_dir: &Path,
) -> Result<Option<PathBuf>, String> {
    let canonical = base_dir.join(OLLAMA_RUNTIME_VERSION);
    if let Some(path) = find_file_named(&canonical, "ollama.exe")? {
        return Ok(Some(path));
    }

    if !base_dir.exists() {
        return Ok(None);
    }

    let mut candidates = Vec::new();
    for entry in fs::read_dir(base_dir).map_err(|e| {
        format!(
            "Could not inspect local AI runtime directory {}: {e}",
            base_dir.display()
        )
    })? {
        let entry =
            entry.map_err(|e| format!("Could not read local AI runtime directory entry: {e}"))?;
        let path = entry.path();
        if !path.is_dir() || path == canonical {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if name.starts_with(&format!("{OLLAMA_RUNTIME_VERSION}-")) {
            candidates.push(path);
        }
    }
    candidates.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    for candidate in candidates {
        if let Some(path) = find_file_named(&candidate, "ollama.exe")? {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

pub fn downloaded_ollama_exe<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let base_dir = runtime_base_dir(app)?;
    find_downloaded_ollama_exe_in_base(&base_dir)?
        .ok_or_else(|| "The downloaded local AI runtime is not installed yet.".to_string())
}

pub fn downloaded_runtime_available<R: tauri::Runtime>(app: &AppHandle<R>) -> bool {
    downloaded_ollama_exe(app)
        .map(|path| path.exists())
        .unwrap_or(false)
}

pub fn start_downloaded_ollama<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<Option<SidecarChild>, String> {
    if OllamaSidecar::port_in_use(OllamaSidecar::OLLAMA_LOCAL_ADDR) {
        return Ok(None);
    }
    let exe = downloaded_ollama_exe(app)?;
    if !exe.exists() {
        return Err("The downloaded local AI runtime is not installed yet.".to_string());
    }
    let mut command = Command::new(&exe);
    if let Some(parent) = exe.parent() {
        command.current_dir(parent);
    }
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let child = command
        .arg("serve")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Could not start downloaded local AI runtime: {e}"))?;
    Ok(Some(SidecarChild::Std(child)))
}

pub async fn install_windows_ollama_runtime<R: tauri::Runtime>(
    app: AppHandle<R>,
    sink: std::sync::Arc<dyn RuntimeInstallSink>,
) -> Result<(), String> {
    if !cfg!(target_os = "windows") {
        return Err(
            "Automatic local AI runtime install is currently implemented for Windows.".to_string(),
        );
    }

    let base_dir = runtime_base_dir(&app)?;
    if downloaded_ollama_exe(&app).is_ok() {
        sink.progress(RuntimeInstallProgress {
            stage: "ready".to_string(),
            message: "Local AI runtime is already installed.".to_string(),
            completed: None,
            total: None,
        });
        return Ok(());
    }

    fs::create_dir_all(&base_dir).map_err(|e| {
        format!(
            "Could not create local AI runtime folder {}: {e}",
            base_dir.display()
        )
    })?;
    let install_id = runtime_install_id();
    let install_dir = runtime_install_dir_for_base(&base_dir);
    let zip_path = base_dir.join(format!(
        "ollama-windows-amd64-{OLLAMA_RUNTIME_VERSION}-{install_id}.zip"
    ));

    sink.progress(RuntimeInstallProgress {
        stage: "download".to_string(),
        message: "Downloading the local AI runtime from Ollama.".to_string(),
        completed: Some(0),
        total: None,
    });

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(RUNTIME_DOWNLOAD_CONNECT_TIMEOUT_SECS))
        .build()
        .map_err(|e| e.to_string())?;
    let mut response = client
        .get(OLLAMA_WINDOWS_AMD64_URL)
        .send()
        .await
        .map_err(|e| format!("Could not download local AI runtime: {e}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "Could not download local AI runtime: HTTP {}",
            response.status()
        ));
    }

    let total = response.content_length();
    let mut file = fs::File::create(&zip_path).map_err(|e| {
        format!(
            "Could not create local AI runtime download file {}: {e}",
            zip_path.display()
        )
    })?;
    let mut downloaded = 0u64;
    loop {
        let next_chunk = tokio::time::timeout(
            Duration::from_secs(RUNTIME_DOWNLOAD_IDLE_TIMEOUT_SECS),
            response.chunk(),
        )
        .await
        .map_err(|_| {
            "Runtime download stalled. Check your internet connection and retry setup.".to_string()
        })?
        .map_err(|e| format!("Runtime download failed: {e}"))?;
        let Some(chunk) = next_chunk else { break };
        file.write_all(&chunk).map_err(|e| {
            format!(
                "Could not write local AI runtime download file {}: {e}",
                zip_path.display()
            )
        })?;
        downloaded += chunk.len() as u64;
        sink.progress(RuntimeInstallProgress {
            stage: "download".to_string(),
            message: "Downloading the local AI runtime from Ollama.".to_string(),
            completed: Some(downloaded),
            total,
        });
    }
    drop(file);

    sink.progress(RuntimeInstallProgress {
        stage: "verify".to_string(),
        message: "Verifying the downloaded runtime.".to_string(),
        completed: None,
        total: None,
    });
    let mut hasher = sha2::Sha256::new();
    let mut file = fs::File::open(&zip_path).map_err(|e| {
        format!(
            "Could not reopen local AI runtime download file {} for verification: {e}",
            zip_path.display()
        )
    })?;
    let mut buf = [0u8; 1024 * 1024];
    loop {
        let read = file.read(&mut buf).map_err(|e| {
            format!(
                "Could not read local AI runtime download file {} for verification: {e}",
                zip_path.display()
            )
        })?;
        if read == 0 {
            break;
        }
        hasher.update(&buf[..read]);
    }
    let actual = hex::encode(hasher.finalize());
    if actual != OLLAMA_WINDOWS_AMD64_SHA256 {
        let _ = fs::remove_file(&zip_path);
        return Err(format!(
            "Local AI runtime verification failed. Expected {}, got {}.",
            OLLAMA_WINDOWS_AMD64_SHA256, actual
        ));
    }

    sink.progress(RuntimeInstallProgress {
        stage: "extract".to_string(),
        message: "Installing the local AI runtime.".to_string(),
        completed: None,
        total: None,
    });
    fs::create_dir_all(&install_dir).map_err(|e| {
        format!(
            "Could not create local AI runtime install folder {}: {e}",
            install_dir.display()
        )
    })?;
    extract_zip(&zip_path, &install_dir)?;

    if find_file_named(&install_dir, "ollama.exe")?.is_none() {
        return Err("The local AI runtime archive did not contain ollama.exe.".to_string());
    }
    let _ = fs::remove_file(&zip_path);

    sink.progress(RuntimeInstallProgress {
        stage: "start".to_string(),
        message: "Starting the local AI runtime.".to_string(),
        completed: None,
        total: None,
    });

    if let Some(child) = start_downloaded_ollama(&app)? {
        if let Some(sidecar) = app.try_state::<std::sync::Arc<OllamaSidecar>>() {
            if let Ok(mut guard) = sidecar.child.lock() {
                *guard = Some(child);
            }
        }
    }

    for _ in 0..30 {
        if check_ollama_status().await {
            sink.progress(RuntimeInstallProgress {
                stage: "complete".to_string(),
                message: "Local AI runtime is ready.".to_string(),
                completed: None,
                total: None,
            });
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Err("The local AI runtime was installed but did not become reachable.".to_string())
}

pub(crate) fn runtime_install_dir_for_base(base_dir: &Path) -> PathBuf {
    let canonical = base_dir.join(OLLAMA_RUNTIME_VERSION);
    if canonical.exists() {
        base_dir.join(format!("{OLLAMA_RUNTIME_VERSION}-{}", runtime_install_id()))
    } else {
        canonical
    }
}

fn runtime_install_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{}-{millis}", std::process::id())
}

fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<(), String> {
    let file = fs::File::open(zip_path).map_err(|e| {
        format!(
            "Could not open local AI runtime archive {}: {e}",
            zip_path.display()
        )
    })?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| {
        format!(
            "Could not read local AI runtime archive {}: {e}",
            zip_path.display()
        )
    })?;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("Could not read local AI runtime archive entry {i}: {e}"))?;
        let Some(name) = entry.enclosed_name().map(|p| p.to_owned()) else {
            continue;
        };
        let out = dest_dir.join(name);
        if entry.is_dir() {
            fs::create_dir_all(&out).map_err(|e| {
                format!(
                    "Could not create local AI runtime folder {}: {e}",
                    out.display()
                )
            })?;
            continue;
        }
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Could not create local AI runtime folder {}: {e}",
                    parent.display()
                )
            })?;
        }
        let mut outfile = fs::File::create(&out).map_err(|e| {
            format!(
                "Could not create local AI runtime file {}: {e}",
                out.display()
            )
        })?;
        std::io::copy(&mut entry, &mut outfile).map_err(|e| {
            format!(
                "Could not write local AI runtime file {}: {e}",
                out.display()
            )
        })?;
    }
    Ok(())
}

fn find_file_named(dir: &Path, filename: &str) -> Result<Option<PathBuf>, String> {
    if !dir.exists() {
        return Ok(None);
    }
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file()
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.eq_ignore_ascii_case(filename))
                .unwrap_or(false)
        {
            return Ok(Some(path));
        }
        if path.is_dir() {
            if let Some(found) = find_file_named(&path, filename)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

impl Drop for OllamaSidecar {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[cfg(test)]
mod stream_parser_tests {
    use super::*;

    // --- parse_stream_line: the pure NDJSON-per-line parser (NEW-Nit1) ---

    #[test]
    fn parses_a_token_line() {
        assert_eq!(
            parse_stream_line(r#"{"response":"Hello","done":false}"#),
            StreamLineOutcome::Token("Hello".to_string())
        );
    }

    #[test]
    fn done_terminator_line_returns_done() {
        // The terminal object in practice carries an empty `response` + done:true.
        assert_eq!(
            parse_stream_line(r#"{"response":"","done":true}"#),
            StreamLineOutcome::Done
        );
    }

    #[test]
    fn error_object_takes_precedence_over_everything() {
        assert_eq!(
            parse_stream_line(r#"{"error":"model 'qwen3:8b' not found","done":true}"#),
            StreamLineOutcome::Error("model 'qwen3:8b' not found".to_string())
        );
    }

    #[test]
    fn blank_and_garbage_lines_are_skipped() {
        assert_eq!(parse_stream_line(""), StreamLineOutcome::Skip);
        assert_eq!(parse_stream_line("   "), StreamLineOutcome::Skip);
        assert_eq!(
            parse_stream_line("not json at all"),
            StreamLineOutcome::Skip
        );
        // Whitespace around a valid line is tolerated.
        assert_eq!(
            parse_stream_line("  {\"response\":\"x\",\"done\":false}\r"),
            StreamLineOutcome::Token("x".to_string())
        );
    }

    /// Drive the same parse/accumulate logic the streaming loop uses, but over an
    /// in-memory buffer fed in arbitrary chunks. This pins the multi-chunk line
    /// buffering (a token split across two pushes) and the `done` terminator
    /// without needing a live socket (the real call hardcodes 127.0.0.1:11434).
    fn accumulate_chunks(chunks: &[&str]) -> Result<String, String> {
        let mut buf = String::new();
        let mut accumulated = String::new();
        for chunk in chunks {
            buf.push_str(chunk);
            while let Some(nl) = buf.find('\n') {
                let line = buf[..nl].to_string();
                buf.drain(..=nl);
                match parse_stream_line(&line) {
                    StreamLineOutcome::Token(t) => accumulated.push_str(&t),
                    StreamLineOutcome::Done => return Ok(accumulated),
                    StreamLineOutcome::Error(e) => return Err(e),
                    StreamLineOutcome::Skip => {}
                }
            }
        }
        match parse_stream_line(&buf) {
            StreamLineOutcome::Token(t) => accumulated.push_str(&t),
            StreamLineOutcome::Error(e) => return Err(e),
            StreamLineOutcome::Done | StreamLineOutcome::Skip => {}
        }
        Ok(accumulated)
    }

    #[test]
    fn multi_chunk_line_buffering_reassembles_split_lines() {
        // A single JSON object's newline arrives in a later chunk; a second
        // object is split across the chunk boundary mid-object.
        let out = accumulate_chunks(&[
            "{\"response\":\"Hel",
            "lo \",\"done\":false}\n{\"response\":\"World\",\"done\":false}\n",
            "{\"response\":\"\",\"done\":true}\n",
        ])
        .unwrap();
        assert_eq!(out, "Hello World");
    }

    #[test]
    fn error_in_stream_aborts_accumulation() {
        let err = accumulate_chunks(&[
            "{\"response\":\"partial \",\"done\":false}\n",
            "{\"error\":\"context length exceeded\"}\n",
            "{\"response\":\"never seen\",\"done\":true}\n",
        ])
        .unwrap_err();
        assert_eq!(err, "context length exceeded");
    }

    #[test]
    fn done_terminator_stops_reading_remaining_lines() {
        let out = accumulate_chunks(&[
            "{\"response\":\"A\",\"done\":false}\n{\"response\":\"\",\"done\":true}\n{\"response\":\"B\",\"done\":false}\n",
        ])
        .unwrap();
        assert_eq!(out, "A");
    }

    #[test]
    fn trailing_line_without_newline_is_parsed() {
        // Stream ends with a final object and no trailing newline.
        let out = accumulate_chunks(&["{\"response\":\"tail\",\"done\":false}"]).unwrap();
        assert_eq!(out, "tail");
    }

    // --- generation_timeout: env-var resolution (NEW-Nit1) ---

    #[test]
    fn generation_timeout_resolves_env_values() {
        // Serialized within one test to avoid cross-test env-var races.
        let key = "CIVICNEWS_LLM_TIMEOUT_SECS";
        let prev = std::env::var(key).ok();

        std::env::remove_var(key);
        assert_eq!(
            generation_timeout(),
            Some(Duration::from_secs(DEFAULT_LLM_TIMEOUT_SECS)),
            "unset → default"
        );

        std::env::set_var(key, "120");
        assert_eq!(
            generation_timeout(),
            Some(Duration::from_secs(120)),
            "valid integer → that many seconds"
        );

        std::env::set_var(key, "0");
        assert_eq!(generation_timeout(), None, "0 → no timeout");

        std::env::set_var(key, "  90 ");
        assert_eq!(
            generation_timeout(),
            Some(Duration::from_secs(90)),
            "whitespace is trimmed"
        );

        std::env::set_var(key, "garbage");
        assert_eq!(
            generation_timeout(),
            Some(Duration::from_secs(DEFAULT_LLM_TIMEOUT_SECS)),
            "unparseable → default"
        );

        match prev {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }

    #[test]
    fn ollama_base_url_accepts_only_loopback_overrides() {
        let key = "CIVICNEWS_OLLAMA_BASE_URL";
        let prev = std::env::var(key).ok();

        std::env::remove_var(key);
        assert_eq!(ollama_base_url(), DEFAULT_OLLAMA_BASE_URL);

        std::env::set_var(key, "http://127.0.0.1:65534/");
        assert_eq!(ollama_base_url(), "http://127.0.0.1:65534");

        std::env::set_var(key, "http://localhost:11435");
        assert_eq!(ollama_base_url(), "http://localhost:11435");

        std::env::set_var(key, "https://example.com");
        assert_eq!(ollama_base_url(), DEFAULT_OLLAMA_BASE_URL);

        match prev {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }
}
