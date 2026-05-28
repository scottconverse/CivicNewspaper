# Documentation Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-26
**Role:** Technical Writer
**Scope audited:** README.md, docs/architecture.md, docs/user_manual.md, FAQ.md, docs/install.md, docs/discussion_seeds.md, CONTRIBUTING.md, and SECURITY.md
**Writer mode:** audit+draft
**Auditor posture:** Balanced

---

## TL;DR

CivicNewspaper's primary documentation assets are in solid shape, but several secondary files and diagrams contain outdated information and inaccuracies. The previously documented compiler status mismatch (`"ready_to_publish"`), loopback authorization middleware checks (Origin header check bypass for CLI), and Host header checks have been fully resolved in the codebase. However, there are unresolved documentation issues: the user manual lists incorrect database file system paths based on an old bundle identifier and shows an inaccurate Mermaid diagram for local HTTP routing. Additionally, the contributing guidelines still reference the legacy monolithic `App.tsx` component, and the community discussion templates falsely claim that guardrails block site compilation.

## Severity roll-up (documentation)

| Severity | Count (Before Fixes) | Count (Remaining Doc Gaps) |
|---|---|---|
| Blocker | 2 | 0 |
| Critical | 2 | 1 |
| Major | 3 | 2 |
| Minor | 2 | 1 |
| Nit | 0 | 0 |

## What's working

- **Ollama / Local LLM Hardware Guide** — [FAQ.md, "What hardware do I need?", L27-33] is accurate and matches the automated memory diagnostics implemented in the onboarding wizard.
- **OSINT Regex Specs** — [FAQ.md, "What does the OSINT Detector Engine actually do?", L85-98] perfectly lists the eight regular expressions and accurately documents the case-insensitivity and exact-string deduplication logic.
- **Tauri Installer Workarounds** — [docs/install.md] details clean operating system installation instructions, and accurately guides non-technical users on how to bypass unsigned binary warnings (SmartScreen, macOS Gatekeeper).
- **Offline Checksum Verification** — [docs/install.md, L57-87] provides accurate and easy-to-follow commands (PowerShell `Get-FileHash`, macOS `shasum`, Linux `sha256sum`) for binary signature verification.

## What couldn't be assessed

- All documentation and files listed in the scope were fully accessible locally in the workspace directory. No external paywalled documents or broken links prevented audit completion.

---

## Doc asset inventory

| Asset | Exists? | Status | Finding(s) |
|---|---|---|---|
| README.md | Yes | Strong | None (Resolved) |
| ARCHITECTURE.md | Yes (as docs/architecture.md) | Strong | None (Resolved) |
| User manual / guide | Yes (as docs/user_manual.md) | Weak | DOC-010, DOC-011 |
| Install guide | Yes (as docs/install.md) | Strong | None |
| FAQ | Yes | Strong | None (Resolved) |
| CHANGELOG | Yes | Adequate | None (Resolved) |
| CONTRIBUTING | Yes | Adequate | DOC-007 |
| SECURITY | Yes | Strong | None (Resolved) |
| LICENSE | Yes | Strong | None |
| Landing / marketing page | Yes (as docs/index.html) | Adequate | None |

---

## Persona walk-through

### First-time user
A first-time user reads `README.md`, downloads the installer, and handles OS warnings using the steps in `docs/install.md`. They configure their profile, download the LLM via the wizard, and run their first scan. They promote a lead to a story, edit it in the Workbench, and click "Approve for Static Publish". They open the Publish panel to compile the site. Everything works seamlessly. However, if they try to find their database file using the paths in `docs/user_manual.md` (e.g. to inspect it or copy it to a backup drive), they will search under `com.civicnewspaper.app` instead of `org.civicnews.app` and get blocked.

### Returning user
A returning user wishes to integrate their editor with the command-line assistant skill (`client.js`) or other developer scripts. They run `node assistant-skill/client.js pair <token>`. The loopback server accepts the connection. They read the user manual's architecture section to see how it works, but are confused by a Mermaid diagram showing that the React UI itself sends local HTTP requests to port 12053.

### New team member
A new developer clones the repository and reads `README.md` and `CONTRIBUTING.md`. While `README.md` shows the modular `src/components/` layout, `CONTRIBUTING.md` still advises them that `App.tsx` is a single monolithic 1,918-line component that needs refactoring. This creates a point of confusion during orientation.

---

## Findings

> **Finding ID prefix:** `DOC-`
> **Categories:** Accuracy / Completeness / Onboarding / Architecture / API / FAQ / Marketing / Tone / Hygiene

### [DOC-007] — Major — Onboarding / Accuracy — Outdated Single-File App.tsx Structure in Onboarding Docs

**Evidence**
- `CONTRIBUTING.md` (Line 51):
  Describes `App.tsx` as a "1,918-line single-page React component" and recommends modularization.

**Why this matters**
Confuses new contributors. They expect a massive, monolithic file to modularize, but find a refactored directory (`src/components/`, `useApp.ts`) in the workspace, indicating that onboarding documentation has not been kept up to date.

**Status: Unresolved**
`README.md` has been successfully updated to show the modular `src/components/` layout. However, `CONTRIBUTING.md` (L51) still contains the outdated refactoring description.

**Fix path**
Remove the outdated description and replace it with instructions on how to contribute to the modular React component architecture in `src/components/`. Drafted in `doc-rewrites/CONTRIBUTING.md`.

---

### [DOC-009] — Minor — Marketing / Accuracy — Discussion Seeds Claim Guardrails Block Compiler

**Evidence**
- `docs/discussion_seeds.md` (Line 97):
  `* Note: The CivicNews compiler will block publication if arrest keywords are used...`

**Why this matters**
Reinforces the misconception about compiler-enforced guardrails to new community members, leading them to believe the system behaves restrictively when it is actually a visual linting helper in the UI.

**Status: Unresolved**
`docs/discussion_seeds.md` still contains this sentence.

**Fix path**
Modify `docs/discussion_seeds.md` line 97 to state that the editor flags a warning instead of claiming the compiler blocks publication. Drafted in `doc-rewrites/docs/discussion_seeds.md`.

---

### [DOC-010] — Critical — Onboarding / Accuracy — Incorrect App Data Paths in User Manual

**Evidence**
- `docs/user_manual.md` (Lines 202-204):
  ```markdown
  * **Windows**: `%APPDATA%\com.civicnewspaper.app\civicnews.db`
  * **macOS**: `~/Library/Application Support/com.civicnewspaper.app/civicnews.db`
  * **Linux**: `~/.config/com.civicnewspaper.app/civicnews.db`
  ```

**Why this matters**
The actual bundle identifier configured in `tauri.conf.json` is `org.civicnews.app`. Under Tauri v2, the database path resolves using this identifier, resulting in files stored under `org.civicnews.app` (e.g. `%APPDATA%\org.civicnews.app\civicnews.db`). A user or developer following the user manual will look in the wrong directory and conclude that their database is missing or corrupt.

**Status: Unresolved**
The incorrect directories remain in `docs/user_manual.md`.

**Fix path**
Update the file paths to match the correct identifier (`org.civicnews.app`) and correct the Linux directory path to use `$XDG_DATA_HOME` (`~/.local/share/`). Drafted in `doc-rewrites/docs/user_manual.md`.

---

### [DOC-011] — Major — Architecture / Accuracy — Incorrect ReactUI-Loopback Server Relationship in User Manual Diagram

**Evidence**
- `docs/user_manual.md` (Lines 108-117):
  ```mermaid
  ReactUI[React 19 Frontend] <-->|Localhost HTTP| LoopbackServer[Axum Loopback Server 127.0.0.1:12053]
  ```

**Why this matters**
This diagram states that the React frontend queries the Axum loopback server over local HTTP, which is architecturally false. The React frontend interacts with the Rust backend strictly via Tauri IPC commands (`invoke`), while the Axum server on port 12053 is dedicated to external client integrations (like the browser extension and CLI tools). This misrepresents the system's actual network and subprocess configuration.

**Status: Unresolved**
The diagram remains inaccurate in `docs/user_manual.md`.

**Fix path**
Correct the Mermaid graph syntax in `docs/user_manual.md` to remove the HTTP connection between ReactUI and LoopbackServer, showing instead how the external clients connect to the loopback server and the React UI connects via Tauri IPC. Drafted in `doc-rewrites/docs/user_manual.md`.

---

## Resolved findings (verified)

- **[DOC-001] — Blocker — Status Mismatch Between UI and Compiler**: Resolved. Verified that `compiler.rs` checks for `ready_to_publish` status.
- **[DOC-002] — Blocker — Missing Origin Header Rejection**: Resolved. Verified that `src-tauri/src/core/auth.rs` allows absent Origin headers for CLI clients.
- **[DOC-003] — Critical — Host Header Validation**: Resolved. Verified that `is_valid_host` accepts `"localhost:12053"`.
- **[DOC-004] — Critical — Guardrail Checks Block Compilation**: Resolved. Verified that `docs/architecture.md` and `docs/manual-smoke.md` correctly characterize guardrails as UI lints rather than compiler-blocking restrictions.
- **[DOC-005] — Major — FAQ Falsely Claims App Never Runs Update Checks**: Resolved. FAQ correctly documents the background Tauri update checks.
- **[DOC-006] — Major — SECURITY.md Misstates Stored-XSS Vulnerability**: Resolved. Outdated warning has been completely removed from `SECURITY.md`.
- **[DOC-008] — Minor — README Lists Outdated Naming TODOs**: Resolved. Outdated TODOs have been removed.

---

## Drafts produced

The following updated files have been drafted and placed in the `doc-rewrites/` directory:
- `doc-rewrites/CONTRIBUTING.md`
- `doc-rewrites/docs/user_manual.md`
- `doc-rewrites/docs/discussion_seeds.md`

## Marketing / honesty audit

CivicNewspaper's landing pages, readme introduction, and FAQ text are generally very honest:
- **Pre-alpha Warnings**: [README.md] explicitly warns users that the software is pre-alpha, database schemas are fragile, and it lacks security review.
- **Local AI Limitations**: [FAQ.md] transparently states that local models are weaker, hallucinate, and the responsibility for truth lies with the author.
- **No telemetry / Outbound traffic**: The FAQ accurately documents the background updater check and points out that no user tracking or telemetry metrics are sent.

## Patterns and systemic observations

There is a clear systemic drift where documentation lags behind codebase updates:
1. **Frontend Refactoring Drift**: React code was refactored into components, but onboarding text in `CONTRIBUTING.md` remained unchanged.
2. **Path Configuration Drift**: The Tauri application bundle identifier changed to `org.civicnews.app` during development, but the user manual still referenced the development placeholder `com.civicnewspaper.app`.
3. **Guardrails vs. Compiler**: The design originally specified that guardrails should block compilation, but the compiler was developed independently as a simple flat generator. While core docs were updated to reflect this, the launch templates (`discussion_seeds.md`) still carried the older expectation.
