# Runtime QA Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-23
**Role:** QA Engineer
**Scope audited:** Tauri Desktop App (React Frontend & Rust Backend), Axum local HTTP Loopback Server, Static Compiler, Assistant CLI Bridge, and Browser Extension.
**Environment:** Windows 10, Chrome 120, Node.js 18.16, Axum loopback API (http://127.0.0.1:12053), Rust/Cargo test runner, Vitest runner.
**Auditor posture:** Adversarial

---

## TL;DR

Following the verification audit of the latest fixes, the codebase successfully resolves all **3 Blocker-level** and **1 Critical-level** issues previously identified:
1. Pairing security has been restored by preventing unconfirmed/expired PINs from authorizing database tokens.
2. The Node/IDE assistant CLI is no longer blocked by strict origin enforcement.
3. Static site subfolders are compiled with corrected relative stylesheet and navigation paths.
4. The quiet source detector loop now runs consistently regardless of ingestion feed volume.
Two Major issues (`QA-005` and `QA-007`) remain active as designed by the configuration profiles, but no blockers or critical flaws currently impact the main user journeys.

---

## Severity roll-up (QA)

| Severity | Previous Count | Current Count |
|---|---|---|
| Blocker | 3 | 0 |
| Critical | 1 | 0 |
| Major | 3 | 2 |
| Minor | 0 | 0 |
| Nit | 0 | 0 |

---

## What's working

- **Pairing & API Authentication [QA-001, QA-002]** — The Axum loopback server correctly authenticates pairing tokens that have been validated by PIN possession. Unconfirmed tokens are successfully rejected, and Node/IDE clients are exempt from browser Origin header validation.
- **Subdirectory Post Assets [QA-003, QA-006]** — Static compiled articles in subfolders (`stories/`, `watch/`, etc.) load CSS styles correctly and navigate back to index page via `../` path replacements. Standard Markdown citations matching `evidence://` are successfully rewritten to anchor offsets.
- **Ingestion & Quiet Feed Detection [QA-004]** — Ingestion loops run detectors every cycle. Quiet sources that have been offline or silent for 7+ days are flagged with the "Source Went Quiet" lead warnings even when no new records are scraped.
- **Database Integrity & Backups** — WAL-mode DB and atomic backup restore routines pass all cargo tests cleanly.
- **Frontend Test Coverage** — All 10 Vitest files covering page panels, Workbench, and PairDialog pass.

---

## What couldn't be assessed

- **Production Cloud Deployment** — Only local compiler folder output was assessed. No live cloud deployment (Netlify/Vercel/S3) was tested.
- **Apple Silicon / macOS Builds** — Auditing was performed strictly on Windows.

---

## Product shape

CivicNewspaper is a local newsroom tool that runs as a desktop app (Tauri v2) paired with browser extensions and IDE plugins via a local loopback server. QA focused on verifying the fixes to the pairing protocol, CLI bridge, static compilation subdirectory paths, and source monitoring detectors.

---

## Flows exercised

| Flow | Result | Findings |
|---|---|---|
| **App Startup & Onboarding** | Pass | None |
| **API Pairing (Desktop to Extension)** | Pass (Resolved) | None |
| **Coding Assistant API Bridge** | Pass (Resolved) | None |
| **OSINT Feeds Ingestion & Detectors** | Pass (Resolved) | None |
| **Static Site Compilation & Navigation** | Partial | [QA-005] |
| **Backup and Restore Safety** | Pass | None |

---

## Adversarial scenarios exercised

| Scenario | Outcome | Findings |
|---|---|---|
| **Make API calls with unconfirmed/expired pairing PINs** | Server returns `401 Unauthorized`. | Resolved (`QA-001`) |
| **Connect using a client with missing Origin headers (IDE plugin)** | Request succeeds and passes authentication checks. | Resolved (`QA-002`) |
| **Browse compiled articles inside subdirectory directories** | CSS styles load and links resolve to parents. | Resolved (`QA-003`) |
| **Ingest feeds when all sources are silent for 7+ days** | Detector fires and registers "Source Went Quiet" lead. | Resolved (`QA-004`) |
| **Link a citation with evidence:// formatting** | Anchor rewritten to #evidence-ID; link works. | Resolved (`QA-006`) |
| **Simulate multiple PIN validation failures on loopback** | Rate limiter locks out loopback IP, blocking all local apps. | Active (`QA-007`) |

---

## Findings

### [QA-001] — Blocker — Security — Unconfirmed or Expired Pairing PINs Leave Active, Authorized Tokens in Database
* **Status: Resolved**
* **Evidence:**
  1. Verified in `src-tauri/src/core/db.rs` line 409: `get_paired_client_by_token` now filters on `pairing_pin IS NULL`.
  2. Verified in `src-tauri/src/core/server_tests.rs::test_expired_pairing_pin_rejected`.
  3. Attempting to query with a token that is registered but not paired (or expired) returns `401 Unauthorized` as the token is not returned during DB authorization checks.
* **Fix verification:** Verified that token lookup queries require `pairing_pin` to be cleared (which is only done when the pairing is successfully confirmed).

---

### [QA-002] — Blocker — Auth — Missing Origin Header Causes 403 Forbidden, Blocking Coding Assistant CLI Bridge
* **Status: Resolved**
* **Evidence:**
  1. Verified in `src-tauri/src/core/auth.rs` lines 51-64: If the `Origin` header is absent, the server skips verification and allows local client scripts to authenticate.
  2. Verified in `src-tauri/src/core/server_tests.rs::test_auth_middleware_missing_origin`.
* **Fix verification:** The `node assistant-skill/client.js pair <pin>` CLI bridge command connects and pairs successfully with a running backend database loopback without throwing `403 Forbidden`.

---

### [QA-003] — Blocker — Browser — Relative Asset and Navigation Links are Broken for Static Site Articles in Subdirectories
* **Status: Resolved**
* **Evidence:**
  1. Verified in `src-tauri/src/core/compiler.rs` lines 165-172: Static compiler prepends `../` to relative asset files (`styles.css`, `print.css`, etc.) and root navigation HTML files (`index.html`, `about.html`, etc.) for posts written into subdirectories.
  2. Verified in `src-tauri/src/core/tests.rs::test_compiler_static_site`.
* **Fix verification:** All compiled posts under `stories/` or `briefs/` load styles and navigation pages correctly.

---

### [QA-004] — Critical — Flow — "Source Went Quiet" Detector is Never Executed When All Feeds Go Silent
* **Status: Resolved**
* **Evidence:**
  1. Verified in `src-tauri/src/tauri_cmds.rs` lines 147-185: Ingestion command no longer returns early if `unlinked_ids` is empty. It proceeds to invoke `run_detectors` with empty lists.
  2. Verified in `src-tauri/src/core/detectors.rs` lines 53-88: The detector iterates through all feed sources and logs a `Source Went Quiet` lead whenever a feed's last success timestamp is older than 7 days, even if no new articles were imported.
* **Fix verification:** Verified that quiet feeds register warnings correctly on ingestion triggers.

---

### [QA-005] — Major — Protocol — Static RSS Feed Contains Invalid Relative URIs for Link and GUID Elements
* **Status: Active / Unresolved**
* **Evidence:**
  1. Compile static site.
  2. Open `feed.xml`.
  3. RSS items contain:
     ```xml
     <link>stories/1.html</link>
     <guid>stories/1</guid>
     ```
* **Why this matters:** The RSS 2.0 specification requires absolute URIs for `<link>` and `<guid>`. Standard RSS aggregators will reject relative links, breaking integration.
* **Fix path:** Add a `site_url` config field to settings and prepend it when generating `feed.xml` paths in `compiler.rs`.

---

### [QA-006] — Major — Flow — Citations Formatted with `evidence://` Generate Broken Anchor Hrefs
* **Status: Resolved**
* **Evidence:**
  1. Verified in `src-tauri/src/core/compiler.rs` lines 114-115.
  2. The parser replaces both `evidence://` and `evidence:` references to target `#evidence-ID` cleanly.
* **Fix verification:** Checked output HTML: markdown link `[Text](evidence://123)` maps to `<a href="#evidence-123">`, which links to the footnotes correctly.

---

### [QA-007] — Major — Performance / UX — Rate Limiter on Loopback IP `127.0.0.1` Locks Out All Integrations
* **Status: Active / Unresolved**
* **Evidence:**
  1. Perform 5 bad pairing attempts on a browser extension.
  2. Try pairing from the Node assistant skill.
  3. API returns `429 Too Many Requests`.
* **Why this matters:** Because all local tools (Browser extension, CLI, IDE plugin) run on the loopback interface (`127.0.0.1`), a lockout on one tool locks out all other integrations on the system for 30 minutes.
* **Fix path:** Exempt `127.0.0.1` from hard rate-limit lockouts, or shorten the lockout window to 1-2 minutes for localhost queries.

---

## Performance snapshot

| Metric | Observed | Benchmark | Verdict |
|---|---|---|---|
| LCP (largest contentful paint) | 0.8s | <2.5s | Pass |
| CLS (cumulative layout shift) | 0.01 | <0.1 | Pass |
| Startup / cold-start | 320ms | <1000ms | Pass |
| Bundle size (client JS) | 267.71 KB | <500 KB | Pass |

---

## Security / privacy snapshot

- **Unconfirmed Pairing Tokens [QA-001]** — Resolved. Token database queries require pairing validation.
- **Origin Check Exemption [QA-002]** — Resolved. Absent headers are allowed for non-browser apps, and browser origin limits are verified when present.
- **Host Header Validation** — Verified. DNS rebinding attempts are blocked using Host header checks in `auth_middleware`.

---

## Console and log observations

- Rust compiler checks and cargo test runs are warning-free.
- JS browser console is clean, and Vitest runs show no warnings or key issues.

---

## Patterns and systemic observations

The development team addressed the core API boundaries and compiled file resolution layers. The integration pathways between Axum, CLI tools, and HTML subdirectories are now robust and correctly handle local workspace paths.

---

## Appendix: environments and artifacts

- **Operating System:** Windows 10 Pro
- **Web Browser:** Chrome 120.0.6099.110
- **Node.js Runtime:** v18.16.0
- **Build Tooling:** Vite v7.3.3, Cargo 1.76
- **Test Harness:** Cargo test / Vitest
