# Test Suite Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-26
**Role:** Test Engineer
**Scope audited:** Frontend Vitest component tests (`src/components/*.test.tsx`), global setup configuration (`src/test/setup.ts`), state management hook (`src/useApp.ts`), hook unit test suite (`src/useApp.test.tsx`), Rust cargo tests (`src-tauri/src/core/tests.rs` and `src-tauri/src/core/server_tests.rs`), CI configuration (`.github/workflows/ci.yml`), and Definition of Done validation scripts (`scripts/`).
**Auditor posture:** Adversarial

---

## TL;DR

The project has resolved its most severe historical test coverage gaps by introducing unit tests for the React state manager (`useApp.ts`), frontend components (`AppContent.tsx`, `SourcesPanel.tsx`), and the DuckDuckGo HTML discovery parser. However, the overall testing posture remains top-heavy and heavily reliant on static mocks. 

In Phase 4, major integration and implementation shortcomings have been uncovered:
1. **Mock Self-Validation (Lying Tests)**: The test `test_plain_language_rewrite_invokes_ollama` is a complete fake. It defines a local mock LLM client and calls its own method directly in the test body, bypassing the application code completely.
2. **Migration Bypass**: The legacy data backfill migration test `test_source_tier_backfill_media_lead` inserts records *after* the migration has already run and asserts only on the row count, completely bypassing verification of the SQL UPDATE backfill logic.
3. **Missing Features**: The required "Prompt Library" dropdown UI is completely missing from the `Workbench.tsx` editor, and the corresponding Tauri commands have been stubbed to only support a single hardcoded prompt ID.
4. **Hardcoded Logic**: The backend command `plain_language_rewrite` hardcodes its system prompt string in Rust rather than loading the required `07-plain-language.md` template from the resource directory.

---

## Severity roll-up (tests)

| Severity | Count | Status |
|---|---|---|
| Blocker | 1 | (Active: LLM mock tests itself) |
| Critical | 2 | (Active: Migration backfill bypassed, Prompt Library UI dropdown completely missing) |
| Major | 9 | (Active: Global Mocking, Scraper/LLM coverage, Verbatim plagiarism check, Phase 3 Mocking, Phase 3 Flakiness, Phase 3 Untested UI, Prompt Library path bypass, Rewrite prompt hardcoded, list_prompts stubs) |
| Minor | 2 | (Active: Missing regression test for `evidence://`, Phase 3 Untested log rotation) |
| Nit | 0 | |

---

## What's working

- **State manager hook coverage introduced** — The core application hook `useApp.ts` now has dedicated test coverage under `useApp.test.tsx`, verifying hook initialization and active navigation state changes.
- **Component test coverage introduced** — `AppContent.tsx` is now covered by `AppContent.test.tsx`, and `SourcesPanel.tsx` is covered by `SourcesPanel.test.tsx`.
- **DuckDuckGo HTML parser test added** — The auto-discovery parsing logic is now unit tested in `tests.rs` via `test_parse_duckduckgo_html`.
- **Thorough local database and migration tests** — Migration version jumps and rollback idempotency are fully validated under `test_migrations` in `tests.rs`.
- **Robust Axum Auth and Rate-Limit middleware coverage** — Integration tests in `server_tests.rs` actively test IP-based pairing rate limiting and correct rejection of invalid Host/Origin headers using Tower's oneshot service caller.
- **XSS sanitization verification** — Static compiler output is verified to ensure markdown rendering escapes malicious `<script>` tags via `test_compiler_xss_safe` in `tests.rs`.
- **Daily scan parser validation** — The parser for daily scan responses is tested via `test_daily_scan_parses_fixture_response` in `tests.rs`, validating that JSON structures are correctly saved to `daily_scan_leads`.

---

## What couldn't be assessed

- **Actual frontend code coverage percentages** — Coverage reporting tooling is not configured in `package.json` or `vitest.config.ts`, making it impossible to audit mechanical line/branch coverage statistics.

---

## Test landscape

| Dimension | Observation |
|---|---|
| Framework(s) | Vitest (JSDOM) for Frontend, Cargo Test for Backend |
| Test pyramid shape | Light frontend component unit tests, light backend integration and unit tests, no E2E tests driving a real Tauri runtime |
| Coverage tool | None configured |
| Reported coverage (if any) | N/A |
| Flakiness posture | Clean execution but heavily dependent on environment mocks. |
| CI blocking? | Yes. `.github/workflows/ci.yml` runs tests and the Definition of Done (DoD) verification script |

---

## Findings

> **Finding ID prefix:** `TEST-`
> **Categories:** Coverage / Shortcut / Flakiness / Quality / Ergonomics / Mocking / Regression / CI

### [TEST-001] — Resolved — Coverage — Core state manager useApp.ts and AppContent.tsx have test coverage

**Evidence**
- `useApp.test.tsx` has been added to the frontend test suite. It renders a test wrapper component invoking the `useApp` hook, mocks initial Tauri backend setup queries (`get_queue`, `get_sources`, etc.), and asserts state properties and active tab navigation transitions correctly.
- `AppContent.test.tsx` validates correct routing container mounts.

**Why this matters**
This ensures that core state machinery, view routing, forms setup, and state mutations upon action triggers do not break silently during UI refactoring.

---

### [TEST-002] — Resolved — Coverage — SourcesPanel component has test coverage

**Evidence**
- The component `SourcesPanel.tsx` is now tested in `SourcesPanel.test.tsx`.
- The test renders the sources list containing mock data and simulates a click on the delete button to assert that `onDeleteSource` is triggered with the correct ID.

**Why this matters**
This resolves the risk of untested URL list actions or single source form additions failing silently upon component refactoring.

---

### [TEST-003] — Major — Mocking — Unit tests are tightly bound to isolated mocks; integration validation is absent

**Evidence**
- `src/test/setup.ts` mocks all Tauri APIs globally.
- Frontend unit tests (e.g. `SettingsPanel.test.tsx`) pass callback mock functions (`vi.fn()`) instead of exercising the real integration layer.

**Why this matters**
The frontend tests only verify that components invoke local callbacks when clicked. They do not verify if `useApp.ts` or components make calls to the IPC layer with parameters matching the backend's expected serialization schema. A mismatch in JSON schemas would crash the runtime but pass the tests.

**Blast radius**
- Test files affected: All frontend component test files (11 files)
- Related findings: `TEST-001`, `TEST-009`, `TEST-014`

**Fix path**
Introduce integration tests that do not mock Tauri's invoke layer or write integration tests at the IPC boundary (`src/ipc.ts`) to validate schemas.

---

### [TEST-004] — Major — Coverage — Untested backend business logic in scraper and LLM orchestration

**Evidence**
- `src-tauri/src/core/scraper.rs` (`compute_hash`, `extract_entities`, `clean_html`, `chunk_text`)
- `src-tauri/src/core/llm.rs` (`call_local_ollama`, `pull_ollama_model`)
- None of these functions are exercised or verified by tests in `tests.rs`.

**Why this matters**
Crucial ingestion-time sanitization and information extraction logic is completely unvalidated. For example, errors or anomalies in HTML stripping or paragraph splitting could corrupt imported articles or drop important facts silently.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`

**Fix path**
Add backend unit tests to `src-tauri/src/core/tests.rs` that exercise these utility functions with various edge case strings, empty strings, and special characters.

---

### [TEST-005] — Minor — Quality — Resolved citation link rendering bug lacks regression test coverage

**Evidence**
- `src-tauri/src/core/compiler.rs` was updated to replace both `evidence:` and `evidence://` prefixes.
- However, the regression test in `src-tauri/src/core/tests.rs` still only tests the legacy `evidence:1` syntax (`[Building C](evidence:1)`), leaving the `evidence://` translation path untested in the backend test suite.

**Why this matters**
Without a regression test verifying the `evidence://` citation format compilation, future refactoring of the markdown compiler could break this protocol path again without triggering a build failure.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs` (under `test_compiler_static_site`)

**Fix path**
Modify the compiler test `test_compiler_static_site` in `src-tauri/src/core/tests.rs` to include a citation using the `evidence://` syntax and assert it is compiled into a local anchor link.

---

### [TEST-006] — Major — Coverage — Untested "Verbatim Source Overlap" check in guardrails

**Evidence**
- `src-tauri/src/core/guardrails.rs` implements `find_verbatim_overlap` (line 165) using tokenization and sliding window indexing to verify that draft text does not copy 7+ consecutive words directly from evidence items.
- There are no tests verifying this logic in `src-tauri/src/core/tests.rs`.

**Why this matters**
Plagiarism check is a core requirement for newspaper integrity. If a bug exists in tokenization or sliding window matching, plagiarism could bypass the guardrails unnoticed, or cause infinite loops/panics on specific strings.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs` (under `test_guardrails`)

**Fix path**
Add a test case `test_guardrails_verbatim_overlap` in `tests.rs` to verify that drafts containing verbatim text sequences raise warnings while paraphrased drafts pass.

---

### [TEST-007] — Major — Mocking — Phase 3 Diagnostics export test tests the serialization, not the Tauri command

**Evidence**
- `src-tauri/src/core/tests.rs` (lines 625-644) introduces `test_export_diagnostics_writes_valid_json`.
- This test manually calls `gather_diagnostics()`, serializes it with `serde_json`, and asserts that the resulting string can be deserialized. It never invokes the actual `export_diagnostics` Tauri command it is purposing to test.

**Why this matters**
This is a test of `serde_json` and local file writes, not of the application's integration boundary. The actual `export_diagnostics` command in `src-tauri/src/tauri_cmds.rs` could be completely broken, have mismatched parameters, or fail to write to the filesystem correctly, and this test would still pass.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Feature affected: Phase 3 Diagnostics Export

**Fix path**
Rewrite the test to execute the Tauri command handler `export_diagnostics` directly, verifying that a file is produced at the expected path with valid JSON.

---

### [TEST-008] — Major — Flakiness & Coverage — Phase 3 Diagnostics test asserts empty state and relies on live network

**Evidence**
- `src-tauri/src/core/tests.rs` (`test_gather_diagnostics_has_all_fields`, lines 603-623) tests `gather_diagnostics` against an empty in-memory DB and asserts all counts are 0.
- `gather_diagnostics` invokes `crate::tauri_cmds::ollama_health().await`, which makes a real HTTP request to `http://127.0.0.1:11434/api/tags`. The test does not mock this request.

**Why this matters**
1. **Empty state only:** Testing that an empty DB returns 0 does not verify that populated databases return correct aggregate counts.
2. **Network flakiness:** The test implicitly requires the host machine's network stack and relies on whether Ollama happens to be running locally, creating a non-deterministic integration test.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- CI reliability: High risk of random CI failures if the runner environment changes or Ollama behaves differently.

**Fix path**
Seed the database with test data before running `gather_diagnostics`. Inject an HTTP client interface or mock server to isolate the Ollama health check from the network.

---

### [TEST-009] — Major — Coverage — Untested Phase 3 Diagnostics UI

**Evidence**
- `src/components/SystemStatus.tsx` was modified to include an "Export Diagnostic Report" button, state management, and an invocation of the `@tauri-apps/plugin-dialog` `save` dialog.
- `src/components/SystemStatus.test.tsx` only tests the pre-existing `ollamaOnline` status indicator. It lacks any tests for the export button, the dialog interaction, or the success/error state rendering.

**Why this matters**
The entire frontend path for the Phase 3 deliverable is uncovered. A typo in the command name, a broken dialog plugin mock, or incorrect state handling will not be caught.

**Blast radius**
- Test files affected: `src/components/SystemStatus.test.tsx`
- Feature affected: Phase 3 Diagnostics Export UI

**Fix path**
Add test cases in `SystemStatus.test.tsx` that mock `save` and `invoke`, simulate a click on the Export button, and assert that the correct Tauri command is called.

---

### [TEST-010] — Minor — Coverage — Untested panic log rotation logic

**Evidence**
- `src-tauri/src/lib.rs` (lines 35-37) introduces file size-based log truncation if the panic log exceeds 1MB.
- There are no tests verifying this logic.

**Why this matters**
While a panic hook is difficult to test, the file rotation math and truncation logic can easily fail, leading to runaway log sizes or lost diagnostics.

**Blast radius**
- Feature affected: Phase 3 Diagnostics Panic Hook

**Fix path**
Extract the log file rotation/truncation logic into a standalone, testable function and write a unit test.

---

### [TEST-011] — Blocker — Mocking / Quality — Mock LLM Client tests itself in `test_plain_language_rewrite_invokes_ollama`

**Evidence**
- In `src-tauri/src/core/tests.rs` (lines 787-810), the test `test_plain_language_rewrite_invokes_ollama` declares a local `struct FakeLlmClient` and calls `client.call(...)` directly in the test body.
- It asserts that the fake client returned `"Rewritten text"`. It never invokes the actual `plain_language_rewrite` command handler or any application logic from `tauri_cmds.rs`.

**Why this matters**
This is a test of the mock itself, not of the application's code. If the `plain_language_rewrite` command is broken or deleted from the codebase, this test will still pass.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Feature affected: Phase 4 Plain Language Rewrite Command

**Fix path**
Update the test to invoke `plain_language_rewrite` command handler, passing a mock `AppHandle` containing the register for `Arc<dyn LlmClient>` pointing to the fake client.

---

### [TEST-012] — Critical — Quality / Regression — Legacy migration test `test_source_tier_backfill_media_lead` does not test the backfill logic or verify data integrity

**Evidence**
- In `src-tauri/src/core/tests.rs` (lines 707-734), the test `test_source_tier_backfill_media_lead` inserts a record *after* migration `0004_source_tier.sql` has already run, inserts type `'rss'` and tier `'community_signal'`, and only asserts that `count == 1`.

**Why this matters**
1. **Wrong execution order**: By running migration `0004_source_tier.sql` before inserting the legacy row, the `tier` column already exists when the row is inserted. This does not represent database state at the time of migration.
2. **Incorrect type**: The inserted source type is `'rss'` instead of `'media_lead'`, failing to exercise the SQL update statement.
3. **No value assertion**: The test asserts `assert_eq!(count, 1)` and never checks if the legacy row's tier was actually migrated.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Feature affected: Database Migration 0004 Backfill

**Fix path**
Rewrite `test_source_tier_backfill_media_lead` to run migration `0001_init.sql`, insert a legacy source with `type = 'media_lead'`, run the remaining migrations, and assert that the source's `tier` is now `news_reporting`.

---

### [TEST-013] — Major — Mocking / Quality — Prompt Library `test_get_prompt_loads_aggregator` bypasses Tauri resource path resolution via direct fs read

**Evidence**
- In `src-tauri/src/core/tests.rs` (lines 742-746), `test_get_prompt_loads_aggregator` reads the prompt file directly using `std::fs::read_to_string("prompts/aggregator.md")` instead of calling `crate::core::prompts::get_prompt` or `load_prompt`.

**Why this matters**
The test does not exercise the application's path resolution or `get_prompt` validation code. If path resolution fails in Tauri, this test will not catch it.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Feature affected: Prompt Library File Resolution

**Fix path**
Test the internal folder-mapping logic separately without bypassing it via raw fs reads.

---

### [TEST-014] — Critical — Coverage — Phase 4 Prompt Library dropdown UI is completely missing and untested

**Evidence**
- `src/components/Workbench.tsx` does not implement the prompt library dropdown specified in Phase 4 (section 4b). Consequently, `src/components/Workbench.test.tsx` contains no tests for prompt listing or selection.

**Why this matters**
A core feature of Phase 4 is completely missing from the UI and has no test coverage. This is a severe functional gap that went undetected by CI because the DoD script didn't check frontend components.

**Blast radius**
- Files affected: `src/components/Workbench.tsx`, `src/components/Workbench.test.tsx`
- Feature affected: Prompt Library UI Access

**Fix path**
Implement the prompt library dropdown in `Workbench.tsx` using `list_prompts` and `get_prompt` IPC calls, and add corresponding tests in `Workbench.test.tsx` to assert that prompts are retrieved and paste into the guidelines textarea upon selection.

---

### [TEST-015] — Major — Shortcut / Quality — Tauri command `plain_language_rewrite` hardcodes prompt instead of loading template markdown file from disk

**Evidence**
- In `src-tauri/src/tauri_cmds.rs` (lines 793-801), the command `plain_language_rewrite` uses a hardcoded system prompt string instead of reading `prompts/story/07-plain-language.md` as required by the specification.

**Why this matters**
Changes to `07-plain-language.md` in the prompt library will not affect the rewrite behavior, making the prompt library disconnected from the actual application execution.

**Blast radius**
- Files affected: `src-tauri/src/tauri_cmds.rs`
- Feature affected: Plain Language Rewrite Command

**Fix path**
Refactor `plain_language_rewrite` to read the prompt from `prompts/story/07-plain-language.md` using the app path resolver.

---

### [TEST-016] — Major — Shortcut / Quality — `list_prompts` and `get_prompt` hardcode a single prompt ID, bypassing prompt library requirements and weakening tests

**Evidence**
- `src-tauri/src/core/prompts.rs` defines `VALID_PROMPT_IDS` as `&["aggregator"]`, and `list_prompts()` returns only `["aggregator"]`. In `tests.rs` (line 737), `test_list_prompts_returns_bundled` was changed to only assert `assert!(!prompts.is_empty())` instead of verifying that all 9 prompts in the 5 required categories are loaded and parsed.

**Why this matters**
This is a direct shortcut that allows the developer to claim "Prompt Library" features are complete while skipping the actual implementation of enumerating and loading the 9 bundled markdown prompts.

**Blast radius**
- Files affected: `src-tauri/src/core/prompts.rs`, `src-tauri/src/core/tests.rs`
- Feature affected: Prompt Library Discovery API

**Fix path**
Enumerate and load all 9 prompts in `prompts.rs` using a `PromptMeta` struct with fields `id`, `category`, `title`, `path`, and `description`, and update the unit test to verify that all 9 prompts are listed correctly.

---

## Shortcut census

| Shortcut pattern | Count |
|---|---|
| `.skip` / `xit` / `@skip` | 0 |
| `.only` (left in) | 0 |
| `TODO: add test` / similar | 0 |
| Empty assertion / placeholder | 0 |
| `--retry` / retries normalized | No |
| "Beat the test-count" shortcut | 4 | *(Phase 3 and Phase 4 tests added just to bump RUST_COUNT)* |

---

## Blind spots by class

- **External Network Failures**: Lack of mock tests for third-party network issues (e.g. DuckDuckGo being rate-limited or returning unexpected HTML, feed websites returning 500 errors).
- **Tauri IPC Schema Synchronization**: Mismatch between TypeScript `ipc.ts` data structure definitions and Rust `db.rs` structs can occur silently.
- **Concurrency and DB Locking**: SQLite DB locking behavior is not tested under concurrent accesses.
- **Happy Path Only / Empty State Validation**: New tests frequently only validate that the code doesn't panic on an empty input or empty database, completely missing data-rich edge cases.

---

## Patterns and systemic observations

- **Pure behavioral-mock dependency**: Component tests do not assert real workflows. They only ensure that a button click fires a prop callback, leaving the actual callback implementation in `useApp.ts` entirely unchecked.
- **Ad-hoc parsing logic**: Parsers are hand-coded using regular expressions rather than formal parser generators, but they lack the test suites required to ensure regex edge-cases do not cause stack overflows or bypass validations.
- **DoD Script Gaming**: The developer wrote stubs and watered-down tests specifically to satisfy the test-count gates in the Definition of Done script, without ensuring the tests actually covered the newly introduced feature components or the actual Tauri command execution.
