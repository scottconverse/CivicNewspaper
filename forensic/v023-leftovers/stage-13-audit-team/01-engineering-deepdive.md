# Engineering Deep-Dive Report — CivicNewspaper

**Audit Date:** 2026-05-26  
**Audited Directory:** `C:\Users\scott\Documents\antigravity\eager-archimedes`  
**Role:** Principal Engineer  
**Scope Audited:** Rust backend (`src-tauri/src/`), React frontend state and onboarding (`src/useApp.ts`, `src/components/OnboardingWizard.tsx`), Database migrations, `LlmClient` integration, and the Ollama sidecar process manager.

---

## TL;DR

CivicNews provides a secure, local-first desktop database and static compiler for local government surveillance. Separating UI commands, a loopback Axum API, and SQLite WAL storage is architectural sound. Following the recent implementation of fixes, the previously identified Blocker compiler error in the detectors module (`ENG-001`) and the Critical auth middleware block on external CLI/IDE integrations (`ENG-002`) have been fully resolved. 

However, a new severe Blocker and multiple Critical and Major flaws have been identified in the new local LLM integration and database migration logic:
1. **[ENG-012] Blocker:** The Tauri backend will panic and crash the entire application upon initiating a Daily Scan due to retrieving an unregistered `Arc<dyn LlmClient>` from the Tauri state.
2. **[ENG-013] Critical:** Existing database installations will crash at startup on upgrade because dropping the parent `sources` table during migrations violates SQLite foreign key constraints under active `PRAGMA foreign_keys = ON`.
3. **[ENG-014] Critical:** The onboarding wizard model-pull commands execute raw requests inside background tokio tasks without checking HTTP status codes or providing failure feedback, resulting in a permanently hung screen if Ollama is offline or experiences an error.
4. **[ENG-015] Major:** The application hardcodes `gemma2:9b` dependencies in both the frontend Daily Scan logic and backend aggregator, completely breaking the Daily Scan feature for users running on low-resource machines who were recommended and downloaded lighter models.
5. **[ENG-016] Minor:** Command sprawl and payload mismatch in model pull events will cause state array contamination in `useApp.ts` which acts as a React child-rendering crash landmine.

---

## Severity Roll-up (Engineering)

| Severity | Active Count | Resolved Count | Total |
|---|---|---|---|
| **Blocker** | 1 | 1 | 2 |
| **Critical** | 2 | 1 | 3 |
| **Major** | 5 | 0 | 5 |
| **Minor** | 6 | 0 | 6 |
| **Nit** | 0 | 0 | 0 |
| **Total** | **14** | **2** | **16** |

---

## What's Working

- **Trait-based LLM Abstraction** — The implementation of the `LlmClient` trait is a strong architectural choice that cleanly decouples LLM provider logic (such as `OllamaClient`) from core orchestration commands like `generate_draft`.
- **Clean Sidecar Lifecycle Cleanups** — The `OllamaSidecar` correctly terminates the spawned process on the Tauri `Exit` event, avoiding orphaned background server instances.
- **Onboarding Persistence** — The onboarding wizard progress state is stored in the database's `settings` table, ensuring user configuration is preserved across launches.
- **System Memory Check** — The application successfully queries system RAM using `sysinfo` to make model recommendations, illustrating a good attempt at local-first resource planning.

---

## What Couldn't Be Assessed

- All items in scope were fully accessible. Production telemetry, remote server behavior (outside of the mock localhost configuration), and CI run logs were not assessed as they do not exist for this codebase.

---

## Verification of Fixes for Previous Blocker/Critical Findings

- **[ENG-001] — Blocker — Correctness — Compiler error in `detectors.rs` due to non-existent method `is_multiple_of` on primitive `usize`**
  - **Status:** **Resolved**. 
  - **Evidence:** `src-tauri/src/core/detectors.rs#L323` now evaluates `if i > 0 && (num_bytes - i) % 3 == 0` instead of calling `.is_multiple_of(3)`. The backend compiles cleanly.
- **[ENG-002] — Critical — Correctness — Authentication middleware blocks all non-browser integrations (IDE coding assistant skill, CLI tools) due to strict Origin header check**
  - **Status:** **Resolved**. 
  - **Evidence:** `src-tauri/src/core/auth.rs#L50-L64` allows requests with a missing `Origin` header to fall through to token checks, restoring function to local integrations. A residual routing/hygiene issue with unreachable pairing route code is tracked below as a minor finding (`ENG-011`).

---

## Active Findings

### [ENG-012] — Blocker — Correctness — System Panic on Daily Scan due to unmanaged `Arc<dyn LlmClient>` state in Tauri

**Evidence**
- **File:** `src-tauri/src/core/daily_scan.rs#L94`
- **Why this matters:** The `run_daily_scan` orchestrator attempts to retrieve the managed LLM client state from the Tauri application handle using `app.state::<T>()`. However, `src-tauri/src/lib.rs` does not register `Arc<dyn LlmClient>` in the application state using `app.manage()`. Tauri's `app.state::<T>()` method panics immediately at runtime if the requested type has not been managed. When a user runs a Daily Scan, the Tauri backend will panic, crashing the entire desktop application.

**Blast radius:**
- Adjacent code: `src-tauri/src/lib.rs`, `src-tauri/src/core/daily_scan.rs`
- Shared state: Tauri application managed state.
- User-facing: Initiating a Daily Scan crashes the app.
- Tests to update: Add integration tests simulating a command invocation inside the Tauri context.
- Related findings: TEST-011 (mock LLM client fakes command tests).

**Fix path:**
In `src-tauri/src/lib.rs` inside the `setup` closure, manage the `LlmClient` instance:
```rust
app.manage(Arc::new(crate::core::llm::OllamaClient) as Arc<dyn crate::core::llm::LlmClient>);
```
Alternatively, in `daily_scan.rs`, use `app.try_state::<std::sync::Arc<dyn LlmClient>>()` and fall back to `OllamaClient` if unregistered.

---

### [ENG-013] — Critical — Correctness — Database migration failure on upgrade due to foreign key violations in `0007_source_tier_check.sql`

**Evidence**
- **File:** `src-tauri/migrations/0007_source_tier_check.sql#L14`
- **Why this matters:** The migration `0007_source_tier_check.sql` performs a table migration by copying data to `sources_new`, dropping the parent table `sources`, and renaming `sources_new` to `sources`. This migration is executed under `PRAGMA foreign_keys = ON` as enforced by `migrations.rs#L31`. The child tables `evidence_items` and `daily_scan_leads` define foreign keys pointing to `sources(id)`. When an existing user with records in these tables updates the application, SQLite blocks `DROP TABLE sources;` with a foreign key constraint violation. The transaction rolls back, crashing the application at startup.

**Blast radius:**
- Adjacent code: `src-tauri/src/core/migrations.rs`
- Shared state: SQLite schema constraints.
- User-facing: Startup crashes on app upgrades.
- Related findings: TEST-012 (migration test skips constraints).

**Fix path:**
Temporarily disable foreign keys within the migration or execute it prior to turning constraints on:
```sql
PRAGMA foreign_keys = OFF;
DROP TABLE sources;
ALTER TABLE sources_new RENAME TO sources;
PRAGMA foreign_keys = ON;
```

---

### [ENG-014] — Critical — Correctness — Silent failure of onboarding model pulling commands if Ollama is offline or returns error

**Evidence**
- **File:** `src-tauri/src/tauri_cmds.rs#L484-L530` and `L624-L675`
- **Why this matters:** The commands `pull_ollama_model` and `ollama_pull_model` execute raw `reqwest::Client` calls inside tokio tasks to fetch the Ollama model. They do not verify if `resp.status().is_success()` is true, and they do not catch connection errors or emit error events back to the frontend. If the local Ollama sidecar is offline, takes too long to respond, or returns an error (e.g. 404 Model Not Found), the backend fails silently. The user is presented with a permanently spinning onboarding progress bar with no error feedback, retry path, or exit mechanism.

**Blast radius:**
- Adjacent code: `src-tauri/src/tauri_cmds.rs`
- User-facing: Stuck Onboarding wizard screen when downloading models fails.

**Fix path:**
Refactor `pull_ollama_model` to leverage the robust `llm::pull_ollama_model` backend helper (which checks status and propagates errors) and emit an `"ollama-pull-error"` event to the frontend on failure.

---

### [ENG-015] — Major — Correctness / Architecture — Hardcoded `gemma2:9b` model dependency in Daily Scan and Onboarding Wizard blocks low-spec machine users

**Evidence**
- **Files:** `src/components/OnboardingWizard.tsx#L125`, `src-tauri/src/core/daily_scan.rs#L95`, `src/useApp.ts#L268`
- **Why this matters:** Onboarding wizard checks the machine's RAM size and recommends lighter models (like `llama3:8b` or `phi3:mini`) for low-resource systems. However, the download trigger in `OnboardingWizard.tsx` is hardcoded to pull `gemma2:9b`, and the backend daily scan orchestrator is hardcoded to execute calls on `gemma2:9b`. The frontend `useApp.ts` check also blocks Daily Scans unless `gemma2:9b` is present. Users on low-specification systems are shown a recommendation to download lighter models, but the installer forces them to download the large 5.4 GB `gemma2:9b` model anyway. Furthermore, if they do manage to download a smaller model, they are blocked from running a Daily Scan because of hardcoded backend and frontend checks that demand `gemma2:9b`.

**Blast radius:**
- Adjacent code: `src/components/OnboardingWizard.tsx`, `src/useApp.ts`, `src-tauri/src/core/daily_scan.rs`
- User-facing: Slowdowns or outright out-of-memory lockups on lower-spec machines during download/running.

**Fix path:**
Save the selected/recommended model to the SQLite `settings` table during onboarding, and load this value dynamically in `useApp.ts` and `daily_scan.rs` instead of hardcoding `"gemma2:9b"`.

---

### [ENG-016] — Minor — Hygiene / Architecture — Command sprawl and redundant event formats for Ollama model pulling

**Evidence**
- **Files:** `src-tauri/src/tauri_cmds.rs` and `src/useApp.ts#L170-L182`
- **Why this matters:** The Rust backend contains three separate Tauri commands for pulling models: `pull_model`, `pull_ollama_model`, and `ollama_pull_model`. Furthermore, `pull_model` streams raw JSON strings to `"ollama-pull-progress"`, while `pull_ollama_model` streams deserialized `ProgressPayload` objects to the same event. Because both commands stream different formats to the same event name, the global `useApp.ts` listener receives the object payload from `pull_ollama_model` and catches a syntax error. Inside the catch block, it appends the raw object payload to `pullProgressText` (typed as `string[]`), which is a UI crash landmine if React attempts to render objects directly as text children.

**Fix path:**
Remove `pull_model` and `ollama_pull_model`. Consolidate logic to a single command using the `llm` module. Ensure the event payloads are typed consistently as serialized objects and update `useApp.ts` to parse them properly.

---

### [ENG-003] — Major — Performance — N+1 database locks and lack of batch transactions during feed ingestion

**Evidence**
- **File:** `src-tauri/src/core/scraper.rs#L124-L145` and `L166-L187`
- **Why this matters:** During scraping, `scrape_source` processes feed items and HTML text chunks in a loop. For every item, it locks the `DbConn` (`Arc<Mutex<Connection>>`), queries if it exists (`get_evidence_by_hash`), releases the lock, then locks again to insert (`insert_evidence_item`). No overarching transaction is opened, so each check and insert commits to disk individually. Ingesting large feeds (100+ items) incurs massive SQLite lock contention and disk I/O commit overhead. This causes noticeable freezes in the Tauri UI and unnecessary SSD wear due to write amplification.

**Blast radius:**
- Adjacent code: `src-tauri/src/core/db.rs`
- Shared state: Mutex locks on the active SQLite connection.
- User-facing: Feed ingestion causes dashboard stuttering and high disk activity.

**Fix path:**
Acquire the database connection lock once per feed source and wrap the duplicate checking and insertions in a single transaction block (`BEGIN TRANSACTION ... COMMIT`).

---

### [ENG-004] — Major — Correctness — Absence of payload size limits during feed downloading leading to potential OOM crashes

**Evidence**
- **File:** `src-tauri/src/core/scraper.rs#L92`
- **Why this matters:** The scraper downloads feed resources using `reqwest` and reads the entire body directly into memory as bytes. There is no size limit configured on the download stream. If a source URL resolves to a large binary file, the application will run out of memory (OOM) and crash, potentially corrupting local database state.

**Blast radius:**
- Adjacent code: `src-tauri/src/core/scraper.rs`
- User-facing: Scraping a misconfigured or malicious source crashes the desktop application.

**Fix path:**
Read response bytes in chunks (using a stream reader) and abort the request if the downloaded size exceeds a safe maximum limit (e.g., 5MB).

---

### [ENG-005] — Major — Security / Correctness — Fragile regex-based HTML tag stripping in scraper

**Evidence**
- **File:** `src-tauri/src/core/scraper.rs#L194-L212`
- **Why this matters:** The `clean_html` utility uses simple regular expressions to strip HTML tags from scraped web content. Using regex for HTML parsing is unreliable. It fails on attributes containing greater-than signs (e.g. `<input value=">">`), strips text incorrectly, and is vulnerable to regex catastrophic backtracking when parsing malformed or maliciously crafted markup.

**Blast radius:**
- Adjacent code: `src-tauri/src/core/scraper.rs` (ingestion extraction).
- Shared state: Evidence database records.

**Fix path:**
Replace the custom regex cleaner with a robust, dedicated HTML extraction library like `ammonia` or `html2text`.

---

### [ENG-010] — Major — Security — Loopback-wide denial of service (DoS) via IP-based pairing rate limiting

**Evidence**
- **File:** `src-tauri/src/core/server.rs#L120-L130` and `L143-L153`
- **Why this matters:** Rate limiting for client pairing (`/api/pair`) is tracked globally by the caller's IP address. Since the application is bound to the loopback interface, all legitimate clients (Tauri webview, CLI tools, browser extension, IDE assistant) and any requests generated by local browser tabs share the IP `127.0.0.1`. A malicious webpage loaded in any standard browser on the user's computer can send 5 rapid cross-origin POST requests to `http://127.0.0.1:12053/api/pair` with dummy data. Because the browser will execute these requests from `127.0.0.1`, the server will exhaust the 5-attempt pairing budget for the entire loopback IP. This triggers a denial of service (DoS), blocking the legitimate user from pairing their browser extension or IDE tools for 30 minutes.

**Blast radius:**
- Adjacent code: `src-tauri/src/core/server.rs`
- Shared state: `pair_attempts` HashMap inside AppState.
- User-facing: Legitimate pairing attempts by browser extensions and IDE helpers are locked out.

**Fix path:**
Store rate limit state keyed by a combination of client identifier and/or custom headers, or perform Host and Origin verification before registering rate limit counters.

---

### [ENG-006] — Minor — Correctness — Flawed text chunking logic failing to split large single-paragraph blocks

**Evidence**
- **File:** `src-tauri/src/core/scraper.rs#L218-L229`
- **Why this matters:** The text chunker iterates over split paragraphs. If a single paragraph is larger than `chunk_size` (2000 chars), the logic appends it to `current_chunk` and skips the chunk boundary check because `current_chunk` was empty. It then yields a chunk that exceeds the target limit. Very large paragraphs (such as long transcripts or legal notices of 10,000+ characters) bypass chunking and are stored as a single massive record, causing database bloat and UI performance issues.

**Fix path:**
Modify the chunker so that if a single paragraph's length exceeds `chunk_size`, it slices the text at sentence boundaries or fixed-character offsets.

---

### [ENG-007] — Minor — Performance — Redundant statement preparation and regex compilation in loops

**Evidence**
- **File:** `src-tauri/src/core/detectors.rs` (Lines 44-51)
- **Why this matters:** Six regexes (`re_money`, `re_vote`, etc.) are compiled from string literals every time `run_detectors` is invoked. Furthermore, SQLite prepared statements in helpers like `lead_exists` are re-prepared on every call within the evidence ingestion loop. Regex compilation and SQL statement preparing are expensive operations. Running them repeatedly in hot loops wastes CPU cycles and slows down feed analysis.

**Fix path:**
Store compiled regexes in `std::sync::OnceLock` variables, and prepare SQLite statements once outside the main loop.

---

### [ENG-008] — Minor — Security / Correctness — Unescaped template replacement of draft format leading to HTML injection

**Evidence**
- **File:** `src-tauri/src/core/compiler.rs#L157`
- **Why this matters:** The compiler replaces `{{POST_FORMAT}}` directly with the value of `draft.format` without escaping it. Since the REST API accepts arbitrary string payloads for `format` on draft creation, a client could insert malicious HTML/JS. This creates an HTML injection/XSS vulnerability on the compiled public static website if the editor's workspace is compromised.

**Fix path:**
Escape `draft.format` using `html_escape::encode_safe` before template replacement, or validate the format against a strict whitelist at the API boundary.

---

### [ENG-009] — Minor — Correctness — Incomplete transaction implementation in `insert_lead`

**Evidence**
- **File:** `src-tauri/src/core/db.rs#L231-L251`
- **Why this matters:** The code comment in `insert_lead` states "We execute the insert and linking inside a transaction to keep lead integrity," but it executes separate `conn.execute` statements directly on the connection without calling `conn.transaction()?`. If linking fails in the middle of the loop, the database is left with an orphaned lead record, violating the stated atomicity.

**Fix path:**
Instantiate a transaction wrapper with `conn.transaction()?`, execute the queries, and call `tx.commit()?`.

---

### [ENG-011] — Minor — Hygiene / Architecture — Unreachable pairing route bypass and dead code in auth middleware

**Evidence**
- **File:** `src-tauri/src/core/server.rs#L73-L89` and `src-tauri/src/core/auth.rs#L47-L50`
- **Why this matters:** The `auth_middleware` contains logic to inspect if the request path matches `/api/pair` and check for an `x-civicnews-pair` header to skip origin validation. However, the middleware layer is only applied to the nested `/api` sub-router (`api_routes`) in `server.rs`. The `/api/pair` route is mounted on the root router outside this middleware layer, meaning it never triggers `auth_middleware`. The bypass logic inside the middleware is completely dead, unreachable code. Additionally, because `/api/pair` bypasses the middleware entirely, it receives no Host validation, meaning it lacks basic protection against DNS rebinding.

**Fix path:**
Remove the dead routing check from `auth_middleware` or correctly apply the middleware to the root router, managing the token exemptions cleanly.
