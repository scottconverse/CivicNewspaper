# Security Policy

CivicNewspaper is public-beta software with no formal security review. The threat surface is small because the app runs on one user's machine, but it is still real: it opens a localhost HTTP server, executes a local LLM runtime, reads untrusted source material, and writes user-controlled Markdown into compiled HTML.

## Reporting a Vulnerability

Please do not open public GitHub issues for security reports.

Email: `sconverse@gmail.com`.

You may also use GitHub's private vulnerability reporting from the repository Security tab if you prefer a tracked confidential channel.

Include:
- A description of the vulnerability.
- Steps to reproduce, ideally with a minimal proof-of-concept.
- The version or commit you reproduced against.
- Your assessment of impact.

Expect an acknowledgement within seven days. A fix timeline depends on severity and maintainer availability; there is no full-time security team.

## In Scope

| Component | Notes |
|---|---|
| `src-tauri/src/core/server.rs` | Axum loopback server on `127.0.0.1:12053`. |
| `src-tauri/src/core/auth.rs` | Host-header validation, Origin allowlist, token pairing, and bearer-token enforcement. |
| `src-tauri/src/core/db.rs` | SQL injection, schema integrity, backup/restore handling. |
| `src-tauri/src/core/backups.rs` | Atomic backup/restore, file overwrite handling, and path traversal. |
| `src-tauri/src/core/compiler.rs` | Markdown-to-HTML compilation and stored-XSS reports in compiled output. |
| `src-tauri/src/core/llm.rs` | Ollama runtime download, startup, local HTTP calls, and model-pull allowlist. |
| `browser-extension/chromium/` | Manifest v3 extension permissions and content-script behavior. |
| Tauri IPC commands in `src-tauri/src/tauri_cmds.rs` | Anything a compromised WebView could invoke. |

## Out Of Scope

- Cloud / SaaS AI services: CivicNewspaper does not send story content to external AI APIs.
- Physical access to the user's unlocked machine.
- Vulnerabilities in Ollama, Tauri, Rust, Node, or other dependencies; report those upstream. We will update dependency pins when CVEs are published.
- Social engineering of the user, such as tricking them into pasting a malicious feed URL.
- Performance / DoS by a feed publishing huge documents; file those as normal bugs.
- Editorial disagreements with guardrail words or story judgments; file those as product issues.

## Local-LLM Privacy

CivicNewspaper is designed with a local-LLM architecture. Draft generation, social media pack creation, and plain-language rewriting run on the user's machine through Ollama.

- **No external AI API keys:** The app does not accept or use OpenAI, Anthropic, Google, or similar AI service keys.
- **No story-content upload:** Drafts, evidence, and prompts are not sent to a vendor AI service by the app.
- **Offline capable after setup:** Once the runtime and selected model are installed, AI work can run without an internet connection.

## Local AI Runtime Attack Surface

CivicNewspaper does not commit an Ollama executable into this repository, and the v0.3.x release path does not depend on the legacy bundled sidecar-fetch script. On Windows, first-run setup can download the pinned Ollama runtime, verify its SHA256, extract it under the user's CivicNewspaper app-data directory, and start it locally.

- **Downloaded runtime:** `install_ollama_runtime` downloads the pinned Windows runtime from Ollama and compares it to the SHA256 in `src-tauri/src/core/llm.rs` before extraction.
- **Process lifecycle:** When a verified downloaded runtime exists, the Rust backend starts `ollama.exe serve` from the app-data runtime folder and keeps the child process handle so it can stop it on app exit or panic. If something is already listening on `127.0.0.1:11434`, the app attaches to that local service instead of starting another process.
- **Tauri capability policy:** The active default capability file does not grant renderer-accessible shell execute/spawn permissions for an Ollama sidecar. The active runtime start path uses Rust backend process control, not renderer shell IPC.
- **Renderer-compromise implications:** A renderer compromise should not gain arbitrary shell execution through Tauri capabilities. It may still be able to trigger exposed app commands or local inference within the app's allowed command surface, so command validation and output escaping remain important.

## Loopback API And Pairing

The browser/agent API listens on `127.0.0.1:12053`.

- Requests must use the expected Host header.
- Browser-origin requests must come from a `chrome-extension://` Origin.
- The pairing endpoint accepts no-Origin local pairing only when the explicit `x-civicnews-pair` header is present, and it still requires the one-time PIN and rate-limit checks.
- All non-pair API routes require a bearer token created by pairing.

## Diagnostic Reports

The app can manually export a diagnostic JSON report from System Status.

There is no automatic upload or telemetry. The report is generated only when the user clicks "Export Diagnostic Report", and the user chooses where the JSON file is saved.

The diagnostic report may include:
- `app_version`
- `os_name` / `os_version`
- `tauri_version`
- `ollama_reachable` / `ollama_models`
- `db_schema_version`
- counts for evidence, leads, drafts, and published posts
- the last lines of the local panic log, if any crashes occurred

## Known Weak Spots

- **Other local processes can reach `127.0.0.1:12053`.** The bearer token is the main gate after pairing. A malicious process running as the same user could try to read local app data or intercept local traffic.
- **Any local process running as the user can use a paired token.** This is by design for local IDE/coding-agent workflows. Revoke leaked tokens from Browser Pairing.
- **Installer provenance.** Users should download only from the official release page, verify the published checksum, and avoid third-party mirrors. Release evidence must include Authenticode signature verification for Windows installers.

## Disclosure Timing

We follow coordinated disclosure. Reporters and maintainer agree on a public-disclosure date once a fix is available. If a vulnerability is already being exploited, that timeline compresses.
