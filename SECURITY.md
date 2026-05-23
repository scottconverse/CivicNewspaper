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

## Known weak spots (acknowledged, not fixed)

These are documented so reporters don't burn time finding them:

- **Other local processes can reach `127.0.0.1:12053`.** The bearer token is the only gate. Any process running as the same user can attempt to brute-force a 6-digit PIN during the 5-minute window or read the token out of the SQLite file.
- **The PIN is 6 decimal digits.** That is ~10^6 = 1M space. Online brute-force during a 5-minute pairing window is conceivable; the code should rate-limit `POST /api/pair`. If it doesn't, that's a real bug worth reporting.
- **Markdown compiled to HTML is not sanitized in this audit's read of `compiler.rs`.** `pulldown-cmark` by default allows raw HTML. If your authoring workflow pulls evidence excerpts from untrusted feeds, those excerpts could contain `<script>` tags that survive into your published static site. Treat all evidence excerpts as untrusted text and sanitize before output. (This is a real, fixable bug — reports welcome.)
- **No code signing.** Built binaries will be flagged by Windows SmartScreen and macOS Gatekeeper. There is no way for a user to verify their build hasn't been tampered with downstream other than building from source.

## Disclosure timing

We follow coordinated disclosure. Reporters and maintainer agree on a public-disclosure date once a fix is available. If a vulnerability is already being exploited in the wild, that timeline compresses.
