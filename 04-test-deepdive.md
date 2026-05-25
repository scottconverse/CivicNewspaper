# Test Suite Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-24
**Role:** Test Engineer
**Scope audited:** Frontend Vitest component tests (`src/components/*.test.tsx`), global setup configuration (`src/test/setup.ts`), state management hook (`src/useApp.ts`), hook unit test suite (`src/useApp.test.tsx`), Rust cargo tests (`src-tauri/src/core/tests.rs` and `src-tauri/src/core/server_tests.rs`), CI configuration (`.github/workflows/ci.yml`), and Definition of Done validation scripts (`scripts/`).
**Auditor posture:** Adversarial

---

## TL;DR

The project has resolved its most severe test coverage gaps by introducing unit tests for the React state manager (`useApp.ts`), frontend components (`AppContent.tsx`, `SourcesPanel.tsx`), and the DuckDuckGo HTML discovery parser. However, the overall testing posture remains top-heavy and heavily reliant on static mocks. Major integration gaps still exist: frontend component tests depend entirely on isolated Tauri API mocks (failing to validate real Rust IPC schema structures). **In Phase 3, new "Diagnostics" features were introduced with tests that exist solely to pass the DoD script's test-count checks. The Phase 3 tests are integration tests in disguise that mock the Tauri boundary, assert against empty database states, introduce network-dependent flakiness, and entirely omit frontend component coverage for the new Export UI.**

---

## Severity roll-up (tests)

| Severity | Count | Status |
|---|---|---|
| Blocker | 0 | (All Resolved) |
| Critical | 0 | (All Resolved) |
| Major | 6 | (Active: Global Mocking, Scraper/LLM coverage, Verbatim plagiarism check, Phase 3 Mocking, Phase 3 Flakiness/Empty State, Phase 3 Untested UI) |
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

---

## What couldn't be assessed

- **Actual frontend code coverage percentages** — Coverage reporting tooling (such as `@vitest/coverage-v8` or Istanbul) is not configured in `package.json` or `vitest.config.ts`, making it impossible to audit mechanical line/branch coverage statistics.

---

## Test landscape

| Dimension | Observation |
|---|---|
| Framework(s) | Vitest (JSDOM) for Frontend, Cargo Test for Backend |
| Test pyramid shape | Light frontend component unit tests, light backend integration and unit tests, no E2E tests driving a real Tauri runtime |
| Coverage tool | None configured |
| Reported coverage (if any) | N/A |
| Flakiness posture | Clean execution but heavily dependent on strict environment mocks. Phase 3 introduces unmocked network flakiness. |
| CI blocking? | Yes. `.github/workflows/ci.yml` runs cargo test, npm test, and the Definition of Done (DoD) verification script |

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
- Related findings: `TEST-001`, `TEST-009`

**Fix path**
Introduce integration tests that do not mock Tauri's invoke layer or write integration tests at the IPC boundary (`src/ipc.ts`) to validate schemas.

---

### [TEST-004] — Major — Coverage — Untested backend business logic in scraper and LLM orchestration

**Evidence**
- `src-tauri/src/core/scraper.rs` (compute_hash, extract_entities, clean_html, chunk_text)
- `src-tauri/src/core/llm.rs` (call_local_ollama, pull_ollama_model)
- None of these functions are exercised or verified by tests in `tests.rs`.
- *Note:* Auto-discovery parsing has been partially tested via `test_parse_duckduckgo_html` in `tests.rs`, which resolves the search scraper parser gap.

**Why this matters**
Crucial ingestion-time sanitization and information extraction logic is completely unvalidated. For example, errors or anomalies in HTML stripping (`clean_html`) or paragraph splitting (`chunk_text`) could corrupt imported articles or drop important facts silently.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Related findings: None

**Fix path**
Add backend unit tests to `src-tauri/src/core/tests.rs` that exercise these utility functions (`clean_html`, `chunk_text`, `extract_entities`) with various edge case strings, empty strings, and special characters.

---

### [TEST-005] — Minor — Quality — Resolved citation link rendering bug lacks regression test coverage

**Evidence**
- `src-tauri/src/core/compiler.rs` was updated to replace both `evidence:` and `evidence://` prefixes.
- However, the regression test in `src-tauri/src/core/tests.rs` still only tests the legacy `evidence:1` syntax (`[Building C](evidence:1)`), leaving the `evidence://` translation path untested in the backend test suite.

**Why this matters**
Without a regression test verifying the `evidence://` citation format compilation, future refactoring of the markdown compiler could break this protocol path again without triggering a build failure.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs` (under `test_compiler_static_site`)
- Related findings: None

**Fix path**
Modify the compiler test `test_compiler_static_site` in `src-tauri/src/core/tests.rs` to include a citation using the `evidence://` syntax and assert it is compiled into a local anchor link.

---

### [TEST-006] — Major — Coverage — Untested "Verbatim Source Overlap" check in guardrails

**Evidence**
- `src-tauri/src/core/guardrails.rs` implements `find_verbatim_overlap` (line 165) using tokenization and sliding window indexing to verify that draft text does not copy 7+ consecutive words directly from evidence items.
- There are no tests verifying this logic in `src-tauri/src/core/tests.rs` (the test `test_guardrails` only covers accusatory language and legal naming checks).

**Why this matters**
Plagiarism check is a core requirement for newspaper integrity. If a bug exists in tokenization or sliding window matching, plagiarism could bypass the guardrails unnoticed, or cause infinite loops/panics on specific strings.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs` (under `test_guardrails`)
- Related findings: None

**Fix path**
Add a test case `test_guardrails_verbatim_overlap` in `tests.rs` to verify that drafts containing verbatim text sequences raise warnings while paraphrased drafts pass.

---

### [TEST-007] — Major — Mocking — Phase 3 Diagnostics export test tests the serialization, not the Tauri command

**Evidence**
- `src-tauri/src/core/tests.rs` (lines 625-644) introduces `test_export_diagnostics_writes_valid_json`.
- This test manually calls `gather_diagnostics()`, serializes it with `serde_json`, and asserts that the resulting string can be deserialized. It never invokes the actual `export_diagnostics` Tauri command it is purportedly testing, nor does it verify that the command writes the file correctly.

**Why this matters**
This is a test of `serde_json` and local file writes, not of the application's integration boundary. The actual `export_diagnostics` command in `src-tauri/src/tauri_cmds.rs` could be completely broken, have mismatched parameters, or fail to write to the filesystem correctly, and this test would still pass. This is a shortcut designed to pass the DoD script's test-count checks.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- Feature affected: Phase 3 Diagnostics Export

**Fix path**
Rewrite the test to execute the Tauri command handler `export_diagnostics` directly (or as an integration test), verifying that a file is produced at the expected path with valid JSON.

---

### [TEST-008] — Major — Flakiness & Coverage — Phase 3 Diagnostics test asserts empty state and relies on live network

**Evidence**
- `src-tauri/src/core/tests.rs` (`test_gather_diagnostics_has_all_fields`, lines 603-623) tests `gather_diagnostics` against an empty in-memory DB and asserts all counts are 0.
- `gather_diagnostics` invokes `crate::tauri_cmds::ollama_health().await`, which makes a real HTTP request to `http://127.0.0.1:11434/api/tags`. The test does not mock this request.

**Why this matters**
1. **Empty state only:** Testing that an empty DB returns 0 does not verify that populated databases return correct aggregate counts. The SQL aggregation logic remains unverified.
2. **Network flakiness:** The test implicitly requires the host machine's network stack and relies on whether Ollama happens to be running locally, creating a non-deterministic integration test disguised as a unit test.

**Blast radius**
- Test files affected: `src-tauri/src/core/tests.rs`
- CI reliability: High risk of random CI failures if the runner environment changes or Ollama behaves differently.

**Fix path**
Seed the in-memory database with test data (evidence, leads, drafts) before running `gather_diagnostics` to verify actual counting logic. Inject an HTTP client interface or mock server to isolate the Ollama health check from the network.

---

### [TEST-009] — Major — Coverage — Untested Phase 3 Diagnostics UI ("Export Diagnostic Report" button)

**Evidence**
- `src/components/SystemStatus.tsx` was modified to include a "Export Diagnostic Report" button, state management (`exportStatus`), and an invocation of the `@tauri-apps/plugin-dialog` `save` dialog and `export_diagnostics` command.
- `src/components/SystemStatus.test.tsx` only tests the pre-existing `ollamaOnline` status indicator. It lacks any tests for the export button, the dialog interaction, or the success/error state rendering.

**Why this matters**
The entire frontend path for the Phase 3 deliverable is uncovered. A typo in the command name, a broken dialog plugin mock, or incorrect state handling will not be caught. The tests were likely bypassed because the DoD script did not enforce frontend test counts.

**Blast radius**
- Test files affected: `src/components/SystemStatus.test.tsx`
- Feature affected: Phase 3 Diagnostics Export UI

**Fix path**
Add test cases in `SystemStatus.test.tsx` that mock `save` from `@tauri-apps/plugin-dialog` and `invoke` from `@tauri-apps/api/core`, simulate a click on the Export button, and assert that the correct Tauri command is called and success messages are displayed.

---

### [TEST-010] — Minor — Coverage — Untested panic log rotation logic

**Evidence**
- `src-tauri/src/lib.rs` (lines 35-37) introduces file size-based log truncation if the panic log exceeds 1MB (`1_048_576` bytes).
- There are no tests in `src-tauri/src/core/tests.rs` or `lib.rs` verifying this logic.

**Why this matters**
While a panic hook is difficult to test, the file rotation math and truncation logic can easily fail (e.g., locking issues or incorrect `truncate(true)` usage) leading to runaway log sizes or lost diagnostics.

**Blast radius**
- Feature affected: Phase 3 Diagnostics Panic Hook

**Fix path**
Extract the log file rotation/truncation logic into a standalone, testable function in `diagnostics.rs` or `backups.rs` and write a unit test that verifies truncation triggers exactly at the 1MB threshold.

---

## Shortcut census

| Shortcut pattern | Count |
|---|---|
| `.skip` / `xit` / `@skip` | 0 |
| `.only` (left in) | 0 |
| `TODO: add test` / similar | 0 |
| Empty assertion / placeholder | 0 |
| `--retry` / retries normalized | No |
| "Beat the test-count" shortcut | 2 | *(Phase 3 tests added just to bump the RUST_COUNT)* |

---

## Blind spots by class

- **External Network Failures**: Lack of mock tests for third-party network issues (e.g. DuckDuckGo being rate-limited or returning unexpected HTML, feed websites returning 500 errors).
- **Tauri IPC Schema Synchronization**: Mismatch between TypeScript `ipc.ts` data structure definitions and Rust `db.rs` structs can occur silently since they are maintained separately and not validated by integration tests.
- **Concurrency and DB Locking**: SQLite DB locking behavior is not tested under concurrent accesses (e.g. browser extension attempting to write via Axum server while the Tauri app is calling ingest or database updates).
- **Happy Path Only / Empty State Validation**: New tests frequently only validate that the code doesn't panic on an empty input or empty database, completely missing data-rich edge cases (as seen in Phase 3 `gather_diagnostics` test).

---

## Patterns and systemic observations

- **Pure behavioral-mock dependency**: Component tests do not assert real workflows. They only ensure that a button click fires a prop callback, leaving the actual callback implementation in `useApp.ts` entirely unchecked.
- **Ad-hoc parsing logic**: Parsers (HTML scraper, Markdown compiler, DDG search scraper, verbatim matcher) are hand-coded using regular expressions rather than formal parser generators, but they lack the test suites required to ensure regex edge-cases do not cause stack overflows or bypass validations.
- **DoD Script Gaming**: The Phase 3 developer clearly wrote tests specifically to satisfy the `[ "$RUST_COUNT" -ge 17 ]` check in `verify-v0.2-phase-3-dod.sh`, without ensuring the tests actually covered the newly introduced feature components or the actual Tauri command execution.

---

## Appendix: test artifacts reviewed

- `package.json`
- `vitest.config.ts`
- `src/test/setup.ts`
- `src/components/*.test.tsx`
- `src/useApp.test.tsx`
- `src-tauri/src/core/tests.rs`
- `src-tauri/src/core/server_tests.rs`
- `.github/workflows/ci.yml`
- `scripts/verify-v0.2-phase-1-dod.sh`
- `scripts/verify-v0.2-phase-3-dod.sh`
