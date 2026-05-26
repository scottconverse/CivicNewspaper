# Security Policy

CivicNewspaper is pre-alpha software with no formal security review. The threat surface is small (the app runs entirely on a single user's machine, no cloud) but non-trivial (it opens a localhost HTTP server, executes a local LLM, and writes user-controlled markdown into compiled HTML).

## Reporting a vulnerability

Please **do not** open public GitHub issues for security reports.

Email: `security@example.invalid` *(maintainer: replace with a real address before publishing)*.

Include:
- A description of the vulnerability.
- Steps to reproduce, ideally with a minimal proof-of-concept.
- The version / commit you reproduced against.
- Your assessment of impact.

Expect an acknowledgement within seven days. A fix timeline depends on severity and on the maintainer's availability — there is no full-time security team.

## In scope

The following components are in scope for security reports:

| Component | Notes |
|---|---|
| `src-tauri/src/core/server.rs` | Axum loopback server on `127.0.0.1:12053`. |
| `src-tauri/src/core/auth.rs` | Host-header validation, Origin whitelist, PIN pairing, bearer-token enforcement. |
| `src-tauri/src/core/db.rs` | SQL injection, schema integrity, backup/restore handling. |
| `src-tauri/src/core/backups.rs` | Atomic backup/restore — file-overwrite, path traversal. |
| `src-tauri/src/core/compiler.rs` | Markdown-to-HTML compilation. Reports of stored-XSS in compiled output (e.g. via raw HTML in evidence excerpts) are in scope. |
| `src-tauri/src/core/llm.rs` | Ollama HTTP client. Reports of prompt-injection that bypass guardrails are in scope but treated as bugs, not critical security issues, because the user is the only consumer. |
| `browser-extension/chromium/` | Manifest v3 extension. Permission overreach, content-script injection issues. |
| Tauri IPC commands (`tauri_cmds.rs`) | Anything an unprivileged WebView page can do that it shouldn't. |

## Out of scope

- Cloud / SaaS: there is none.
- Physical access to the user's machine: not a defended threat. If an attacker has your laptop and unlocks it, they have your CivicNewspaper data.
- Vulnerabilities in Ollama, Tauri, Rust, Node, or other dependencies — report those upstream. We will update dependency pins promptly when CVEs are published.
- Social engineering of the user (e.g. tricking them into pasting a malicious feed URL). The app trusts its own user.
- Performance / DoS by a feed publishing huge documents — file as a normal bug.
- Editorial-content disagreements with the guardrail keyword list — file as a normal issue.

## Local-LLM Only Privacy

CivicNewspaper is designed with a strict "local-LLM only" architecture. All AI tasks—draft generation, social media pack creation, and plain-language rewriting—are executed on your local machine using Ollama.

- **No API Keys:** The application does not accept or use API keys for external services like OpenAI, Anthropic, or Google.
- **No Data Exfiltration:** Your drafts, evidence, and prompts never leave your local machine.
- **Offline Capable:** The entire AI pipeline operates without an internet connection once your selected model (e.g., `gemma2:9b`) is downloaded.

## Diagnostic reports

The application allows you to manually export a diagnostic JSON report via the System Status panel to assist with troubleshooting.

**There is no automatic upload or telemetery.** The report is generated only when you click "Export Diagnostic Report", and you choose exactly where on your local machine the JSON file is saved. You can inspect it before sharing it with anyone.

The diagnostic report captures the following fields:
- `app_version`: The current version of CivicNews
- `os_name` / `os_version`: Your operating system details
- `tauri_version`: The underlying Tauri framework version
- `ollama_reachable` / `ollama_models`: Local AI inference status and available models
- `db_schema_version`: Internal SQLite database schema version
- `evidence_count`, `leads_count`, `drafts_count`, `published_posts_count`: Counts of items in your local database
- `panic_log_tail`: The last 100 lines of the application's panic log (if any crashes occurred)

## Known weak spots (acknowledged, not fixed)

These are documented so reporters don't burn time finding them:

- **Other local processes can reach `127.0.0.1:12053`.** The bearer token is the only gate. Any process running as the same user can attempt to brute-force the pairing token, but it is 16 random bytes of `OsRng` encoded as a 22-char base64 string, making an online brute-force during the 5-minute pairing window virtually impossible. A malicious local process would have to read the token directly out of the SQLite file or intercept IPC.
- **No code signing.** Built binaries will be flagged by Windows SmartScreen and macOS Gatekeeper. There is no way for a user to verify their build hasn't been tampered with downstream other than building from source.

## Disclosure timing

We follow coordinated disclosure. Reporters and maintainer agree on a public-disclosure date once a fix is available. If a vulnerability is already being exploited in the wild, that timeline compresses.
