# Engineering Deep-Dive — CivicNewspaper (CivicNews)

**Audit date:** 2026-05-23
**Role:** Principal Engineer
**Scope audited:** Rust backend (`src-tauri/src/`), Database migrations and CRUD layers (`db.rs`, `migrations.rs`), Authentication middleware (`auth.rs`), HTML Compiler (`compiler.rs`), Feed Scraper (`scraper.rs`), Auto-discovery (`discovery.rs`), and the frontend state management integration (`src/useApp.ts`).
**Auditor posture:** Balanced

---

## TL;DR

CivicNews provides a secure, local-first desktop database and static compiler for local government surveillance, separating UI commands, a loopback Axum API, and SQLite WAL storage. Following the recent implementation of fixes, the previously identified Blocker compiler error in the detectors module and the Critical auth middleware block on external CLI/IDE integrations have been fully resolved. However, some architectural debt and security concerns remain, particularly a newly identified loopback-wide denial of service vector in the pairing rate limiter, N+1 lock contention in feed scraping, uncapped feed downloads, and template-based HTML injection vectors in compiled static output.

## Severity roll-up (engineering)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 0 |
| Major | 4 |
| Minor | 5 |
| Nit | 0 |

## What's working

- **Successful Compiler Resolution (ENG-001)** — The detector's math logic has been updated to use the standard Rust modulo operator, resolving the compilation blocker.
- **Integration Access Restored (ENG-002)** — The authentication middleware now correctly allows requests without an `Origin` header, enabling the Node.js assistant skill and CLI tools to communicate with the core API.
- **Strict Client-side Content Security Policy** — The `tauri.conf.json` defines a highly restrictive CSP (`default-src 'none'`) preventing unauthorized script execution inside the webview dashboard.
- **SQLite WAL Mode & Migration Guards** — SQLite uses WAL mode, and database backups are verified for integrity during restore procedures.
- **Markdown Content Sanitization** — The static-site compiler (`compiler.rs`) explicitly strips HTML/inline HTML tags from `pulldown_cmark` events, protecting compiled feeds from XSS.

## What couldn't be assessed

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

## Findings

> **Finding ID prefix:** `ENG-`
> **Categories:** Architecture / Correctness / Security / Performance / Data provenance / Dependencies / Hygiene

### [ENG-003] — Major — Performance — N+1 database locks and lack of batch transactions during feed ingestion

**Evidence**
- **File:** [src-tauri/src/core/scraper.rs#L124-L145](src-tauri/src/core/scraper.rs#L124-L145) and [L166-L187](src-tauri/src/core/scraper.rs#L166-L187)
- **Description:** During scraping, `scrape_source` processes feed items and HTML text chunks in a loop. For every item, it locks the `DbConn` (`Arc<Mutex<Connection>>`), queries if it exists (`get_evidence_by_hash`), releases the lock, then locks again to insert (`insert_evidence_item`). No overarching transaction is opened, so each check and insert commits to disk individually.

**Why this matters**
Ingesting large feeds (100+ items) incurs massive SQLite lock contention and disk I/O commit overhead. This causes noticeable freezes in the Tauri UI and unnecessary SSD wear due to write amplification.

**Blast radius**
- Adjacent code: `src-tauri/src/core/db.rs`
- Shared state: Mutex locks on the active SQLite connection.
- User-facing: Feed ingestion causes dashboard stuttering and high disk activity.
- Migration: None.
- Tests to update: None.

**Fix path**
Acquire the database connection lock once per feed source and wrap the duplicate checking and insertions in a single transaction block (`BEGIN TRANSACTION ... COMMIT`).

---

### [ENG-004] — Major — Correctness — Absence of payload size limits during feed downloading leading to potential OOM crashes

**Evidence**
- **File:** [src-tauri/src/core/scraper.rs#L92](src-tauri/src/core/scraper.rs#L92)
- **Code snippet:**
  ```rust
  let body_bytes = response.bytes().await?;
  ```
- **Description:** The scraper downloads feed resources using `reqwest` and reads the entire body directly into memory as bytes. There is no size limit configured on the download stream.

**Why this matters**
If a source URL resolves to a large binary file (e.g. a video, large image database, or an infinite event stream), the application will run out of memory (OOM) and crash, potentially corrupting local database state.

**Blast radius**
- Adjacent code: `src-tauri/src/core/scraper.rs`
- User-facing: Scraping a misconfigured or malicious source crashes the desktop application.
- Migration: None.
- Tests to update: None.

**Fix path**
Read response bytes in chunks (using a stream reader) and abort the request if the downloaded size exceeds a safe maximum limit (e.g., 5MB).

---

### [ENG-005] — Major — Security / Correctness — Fragile regex-based HTML tag stripping in scraper

**Evidence**
- **File:** [src-tauri/src/core/scraper.rs#L194-L212](src-tauri/src/core/scraper.rs#L194-L212)
- **Code snippet:**
  ```rust
  let re_tags = regex::Regex::new(r"<[^>]*>").unwrap();
  ```
- **Description:** The `clean_html` utility uses simple regular expressions to strip HTML tags from scraped web content.

**Why this matters**
Using regex for HTML parsing is unreliable. It fails on attributes containing greater-than signs (e.g. `<input value=">">`), strips text incorrectly, and is vulnerable to regex catastrophic backtracking when parsing malformed or maliciously crafted markup.

**Blast radius**
- Adjacent code: `src-tauri/src/core/scraper.rs` (ingestion extraction).
- Shared state: Evidence database records.
- Migration: None.
- Tests to update: None.

**Fix path**
Replace the custom regex cleaner with a robust, dedicated HTML extraction library like `ammonia` or `html2text`.

---

### [ENG-010] — Major — Security — Loopback-wide denial of service (DoS) via IP-based pairing rate limiting

**Evidence**
- **File:** [src-tauri/src/core/server.rs#L120-L130](src-tauri/src/core/server.rs#L120-L130) and [L143-L153](src-tauri/src/core/server.rs#L143-L153)
- **Code snippet:**
  ```rust
  let ip = addr.ip().to_string();
  {
      let mut attempts = state.pair_attempts.lock().unwrap();
      if let Some(&(count, time)) = attempts.get(&ip) {
          if count >= 5 && time.elapsed().as_secs() < 1800 {
              return Err(StatusCode::TOO_MANY_REQUESTS);
          } else if time.elapsed().as_secs() >= 1800 {
              attempts.remove(&ip);
          }
      }
  }
  ```
- **Description:** Rate limiting for client pairing (`/api/pair`) is tracked globally by the caller's IP address. Since the application is bound to the loopback interface, all legitimate clients (Tauri webview, CLI tools, browser extension, IDE assistant) and any requests generated by local browser tabs share the IP `127.0.0.1`.

**Why this matters**
A malicious webpage loaded in any standard browser on the user's computer can send 5 rapid cross-origin POST requests to `http://127.0.0.1:12053/api/pair` with dummy data. Because the browser will execute these requests from `127.0.0.1`, the server will exhaust the 5-attempt pairing budget for the entire loopback IP. This triggers a denial of service (DoS), blocking the legitimate user from pairing their browser extension or IDE tools for 30 minutes.

**Blast radius**
- Adjacent code: `src-tauri/src/core/server.rs`
- Shared state: `pair_attempts` HashMap inside AppState.
- User-facing: Legitimate pairing attempts by browser extensions and IDE helpers are locked out.
- Migration: None.
- Tests to update: `server_tests.rs#test_pair_rate_limit`.

**Fix path**
Store rate limit state keyed by a combination of client identifier and/or custom headers, or perform Host and Origin verification before registering rate limit counters. Alternatively, utilize transient pairing tokens generated in the UI rather than simple loopback IP rate limiting.

---

### [ENG-006] — Minor — Correctness — Flawed text chunking logic failing to split large single-paragraph blocks

**Evidence**
- **File:** [src-tauri/src/core/scraper.rs#L218-L229](src-tauri/src/core/scraper.rs#L218-L229)
- **Description:** The text chunker iterates over split paragraphs. If a single paragraph is larger than `chunk_size` (2000 chars), the logic appends it to `current_chunk` and skips the chunk boundary check because `current_chunk` was empty. It then yields a chunk that exceeds the target limit.

**Why this matters**
Very large paragraphs (such as long transcripts or legal notices of 10,000+ characters) bypass chunking and are stored as a single massive record, causing database bloat and UI performance issues.

**Fix path**
Modify the chunker so that if a single paragraph's length exceeds `chunk_size`, it slices the text at sentence boundaries or fixed-character offsets.

---

### [ENG-007] — Minor — Performance — Redundant statement preparation and regex compilation in loops

**Evidence**
- **File:** [src-tauri/src/core/detectors.rs](src-tauri/src/core/detectors.rs) (Lines 44-51, and statement prepares inside detector loops).
- **Description:** Six regexes (`re_money`, `re_vote`, etc.) are compiled from string literals every time `run_detectors` is invoked. Furthermore, SQLite prepared statements in helpers like `lead_exists` are re-prepared on every call within the evidence ingestion loop.

**Why this matters**
Regex compilation and SQL statement preparing are expensive operations. Running them repeatedly in hot loops wastes CPU cycles and slows down feed analysis.

**Fix path**
Store compiled regexes in `std::sync::OnceLock` variables, and prepare SQLite statements once outside the main loop.

---

### [ENG-008] — Minor — Security / Correctness — Unescaped template replacement of draft format leading to HTML injection

**Evidence**
- **File:** [src-tauri/src/core/compiler.rs#L157](src-tauri/src/core/compiler.rs#L157)
- **Code snippet:**
  ```rust
  post_html = post_html.replace("{{POST_FORMAT}}", &draft.format);
  ```
- **Description:** The compiler replaces `{{POST_FORMAT}}` directly with the value of `draft.format` without escaping it. Since the REST API accepts arbitrary string payloads for `format` on draft creation, a client could insert malicious HTML/JS.

**Why this matters**
This creates an HTML injection/XSS vulnerability on the compiled public static website if the editor's workspace is compromised.

**Fix path**
Escape `draft.format` using `html_escape::encode_safe` before template replacement, or validate the format against a strict whitelist at the API boundary.

---

### [ENG-009] — Minor — Correctness — Incomplete transaction implementation in `insert_lead`

**Evidence**
- **File:** [src-tauri/src/core/db.rs#L231-L251](src-tauri/src/core/db.rs#L231-L251)
- **Description:** The code comment in `insert_lead` states "We execute the insert and linking inside a transaction to keep lead integrity," but it executes separate `conn.execute` statements directly on the connection without calling `conn.transaction()?`.

**Why this matters**
If linking fails in the middle of the loop, the database is left with an orphaned lead record, violating the stated atomicity.

**Fix path**
Instantiate a transaction wrapper with `conn.transaction()?`, execute the queries, and call `tx.commit()?`.

---

### [ENG-011] — Minor — Hygiene / Architecture — Unreachable pairing route bypass and dead code in auth middleware

**Evidence**
- **File:** [src-tauri/src/core/server.rs#L73-L89](src-tauri/src/core/server.rs#L73-L89) and [src-tauri/src/core/auth.rs#L47-L50](src-tauri/src/core/auth.rs#L47-L50)
- **Code snippet:**
  ```rust
  // auth.rs
  let path = request.uri().path();
  let is_pair_route = path == "/api/pair";
  let skip_origin = is_pair_route && request.headers().contains_key("x-civicnews-pair");
  ```
  and
  ```rust
  // server.rs
  Router::new()
      .route("/api/pair", post(pair_handler))
      .nest("/api", api_routes) // auth middleware only layered on api_routes
  ```
- **Description:** The `auth_middleware` contains logic to inspect if the request path matches `/api/pair` and check for an `x-civicnews-pair` header to skip origin validation. However, the middleware layer is only applied to the nested `/api` sub-router (`api_routes`) in `server.rs`. The `/api/pair` route is mounted on the root router outside this middleware layer, meaning it never triggers `auth_middleware`.

**Why this matters**
The bypass logic inside the middleware is completely dead, unreachable code. Additionally, because `/api/pair` bypasses the middleware entirely, it receives no Host validation, meaning it lacks basic protection against DNS rebinding.

**Fix path**
Remove the dead routing check from `auth_middleware` or correctly apply the middleware to the root router, managing the token exemptions cleanly.

---

## Patterns and systemic observations

- **Denial of Service Surface Area on Loopback Interfaces:** Utilizing raw loopback client IP mapping for rate limits on local desktop helper APIs creates a shared resource bottleneck. Since all local client integrations map to `127.0.0.1`, simple IP-based tracking impacts the entire user ecosystem, making the interface vulnerable to DoS from unprivileged browser tabs.
- **Lack of Transaction Batching:** A systemic pattern exists of running multiple queries in loops without bundling them into single SQLite transactions, which results in significant commit delays and disk write overhead.
- **Bypass and Layering Inconsistencies:** The separation between middleware logic and routing boundaries leads to unreachable configuration paths. Code written to handle authentication exclusions within a middleware function assumes the endpoint falls under its footprint, which is easily subverted when endpoints are nested on different parent routers.

## Dependency snapshot

Dependency surface is clean — no notable concerns.

## Appendix: artifacts reviewed

- `src-tauri/src/lib.rs`
- `src-tauri/src/core/db.rs`
- `src-tauri/src/core/migrations.rs`
- `src-tauri/src/core/auth.rs`
- `src-tauri/src/core/server.rs`
- `src-tauri/src/core/scraper.rs`
- `src-tauri/src/core/detectors.rs`
- `src-tauri/src/core/compiler.rs`
- `src-tauri/src/core/discovery.rs`
- `src-tauri/src/core/backups.rs`
- `src-tauri/src/core/guardrails.rs`
- `src-tauri/src/tauri_cmds.rs`
- `src-tauri/tauri.conf.json`
- `package.json`
- `src/useApp.ts`
