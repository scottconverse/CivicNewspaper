# Security Policy

CivicNewspaper is public-beta software with no formal security review. The threat surface is small (the app runs entirely on a single user's machine, no cloud) but non-trivial (it opens a localhost HTTP server, executes a local LLM, and writes user-controlled markdown into compiled HTML).

## Reporting a vulnerability

Please **do not** open public GitHub issues for security reports.

Email: `sconverse@gmail.com`.

You may also use GitHub's private vulnerability reporting (the "Report a vulnerability" button under the repository's Security tab) if you prefer a tracked, confidential channel.

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
| `src-tauri/src/core/auth.rs` | Host-header validation, Origin whitelist, token pairing (one-time PIN exchanged for a SHA-256-hashed bearer token), bearer-token enforcement. |
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
- **Offline Capable:** The entire AI pipeline operates without an internet connection once your selected model (e.g., `qwen3:8b`) is downloaded.

## Local Sidecar Attack Surface

CivicNewspaper bundles Ollama as an external binary sidecar process to perform local AI inference. This introduces specific security aspects:
- **Bundled Binary**: The Ollama executable is packaged inside the application bundle. It is fetched and verified against a pinned SHA256 by `scripts/fetch-ollama-binaries.sh`, which aborts on a checksum mismatch; that script is run before every official release build (and is the required first step for from-source builds — see README "Building from source"). The checksum guarantee therefore holds for any build that provisions the binary through that script, including all release binaries.
- **Process Lifecycle**: The backend Rust core spawns and manages the Ollama sidecar process, binding it to the default port `11434`. The process is terminated automatically when the main desktop app exits.
- **Tauri Capability `args` Config**: The sidecar execution argument list is explicitly restricted by Tauri's capability policies. Arbitrary command-line arguments are not permitted.
- **Renderer-Compromise Implications**: If the frontend renderer is compromised (e.g., via stored XSS in a scraped feed), the attacker cannot execute arbitrary commands or access the host filesystem directly through the sidecar, but they could perform unauthorized local inference or query local models via the loopback port `11434`.

## Diagnostic reports

The application allows you to manually export a diagnostic JSON report via the System Status panel to assist with troubleshooting.

**There is no automatic upload or telemetery.** The report is generated only when you click "Export Diagnostic Report", and you choose exactly where on your local machine the JSON file is saved. You can inspect it before sharing it with anyone.

The diagnostic report captures the following fields:
- `app_version`: The current version of The Civic Desk
- `os_name` / `os_version`: Your operating system details
- `tauri_version`: The underlying Tauri framework version
- `ollama_reachable` / `ollama_models`: Local AI inference status and available models
- `db_schema_version`: Internal SQLite database schema version
- `evidence_count`, `leads_count`, `drafts_count`, `published_posts_count`: Counts of items in your local database
- `panic_log_tail`: The last 100 lines of the application's panic log (if any crashes occurred)

## Known weak spots (acknowledged, not fixed)

These are documented so reporters don't burn time finding them:

- **Other local processes can reach `127.0.0.1:12053`.** The bearer token is the only gate. Any process running as the same user can attempt to brute-force the pairing token, but it is 16 random bytes of `OsRng` encoded as a 22-char base64 string, making an online brute-force during the 5-minute pairing window virtually impossible. A malicious local process would have to read the token directly out of the SQLite file or intercept IPC.
- **Any local process running as you can use a paired token.** This is by design: the loopback API exists so IDE coding agents (which run as your user) can drive the app. Once a client pairs, its token is written to a config file (`%APPDATA%\civicnews-token.json` on Windows, `~/Library/Application Support/` or `~/.config/` on macOS). That file is restricted to your user (mode `0600` on Unix; per-user ACL on `%APPDATA%`), but the trust boundary is the user account, not the process: anything you can run can read that file and act as a paired client. The `/api/llm/task` endpoint in particular lets a paired client run arbitrary local-LLM prompts. We do not defend against malware already executing as you — see "Physical access" and "Social engineering" under Out of scope. Revoke a leaked token from the desktop app's "Browser Pairing" tab.
- **No code signing.** Built binaries will be flagged by Windows SmartScreen and macOS Gatekeeper. There is no way for a user to verify their build hasn't been tampered with downstream other than building from source.

## Disclosure timing

We follow coordinated disclosure. Reporters and maintainer agree on a public-disclosure date once a fix is available. If a vulnerability is already being exploited in the wild, that timeline compresses.
