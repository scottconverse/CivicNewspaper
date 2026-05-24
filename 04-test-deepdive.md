# Test Suite Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-23
**Role:** Test Engineer
**Scope audited:** Frontend Vitest component tests (`src/components/*.test.tsx`), global setup configuration (`src/test/setup.ts`), state management hook (`src/useApp.ts`), hook unit test suite (`src/useApp.test.tsx`), Rust cargo tests (`src-tauri/src/core/tests.rs` and `src-tauri/src/core/server_tests.rs`), CI configuration (`.github/workflows/ci.yml`), and Definition of Done validation scripts (`scripts/`).
**Auditor posture:** Adversarial

---

## TL;DR

The project has resolved its most severe test coverage gaps by introducing unit tests for the React state manager (`useApp.ts`), frontend components (`AppContent.tsx`, `SourcesPanel.tsx`), and the DuckDuckGo HTML discovery parser. However, the overall testing posture remains top-heavy and heavily reliant on static mocks. Major integration gaps still exist: frontend component tests depend entirely on isolated Tauri API mocks (failing to validate real Rust IPC schema structures), and backend business logic for scraper sanitization/chunking and the verbatim overlap plagiarism check remain completely untested.

---

## Severity roll-up (tests)

| Severity | Count | Status |
|---|---|---|
| Blocker | 0 | (All Resolved) |
| Critical | 0 | (All Resolved) |
| Major | 3 | (Active: Global Mocking, Scraper/LLM coverage, Verbatim plagiarism check) |
| Minor | 1 | (Active: Missing regression test for `evidence://`) |
| Nit | 0 | |

---

## What's working

- **State manager hook coverage introduced** — The core application hook `useApp.ts` now has dedicated test coverage under [useApp.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/useApp.test.tsx), verifying hook initialization and active navigation state changes.
- **Component test coverage introduced** — `AppContent.tsx` is now covered by [AppContent.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/components/AppContent.test.tsx), and `SourcesPanel.tsx` is covered by [SourcesPanel.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/components/SourcesPanel.test.tsx).
- **DuckDuckGo HTML parser test added** — The auto-discovery parsing logic is now unit tested in `tests.rs` via [test_parse_duckduckgo_html](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/tests.rs#L457-L482).
- **Thorough local database and migration tests** — Migration version jumps and rollback idempotency are fully validated under `test_migrations` in [tests.rs](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/tests.rs#L21-L41).
- **Robust Axum Auth and Rate-Limit middleware coverage** — Integration tests in [server_tests.rs](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/server_tests.rs) actively test IP-based pairing rate limiting and correct rejection of invalid Host/Origin headers using Tower's oneshot service caller.
- **XSS sanitization verification** — Static compiler output is verified to ensure markdown rendering escapes malicious `<script>` tags via `test_compiler_xss_safe` in [tests.rs](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/tests.rs#L484-L564).

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
| Flakiness posture | Clean execution but heavily dependent on strict environment mocks |
| CI blocking? | Yes. `.github/workflows/ci.yml` runs cargo test, npm test, and the Definition of Done (DoD) verification script |

---

## Findings

> **Finding ID prefix:** `TEST-`
> **Categories:** Coverage / Shortcut / Flakiness / Quality / Ergonomics / Mocking / Regression / CI

### [TEST-001] — Resolved — Coverage — Core state manager useApp.ts and AppContent.tsx have test coverage

**Evidence**
- [useApp.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/useApp.test.tsx) has been added to the frontend test suite. It renders a test wrapper component invoking the `useApp` hook, mocks initial Tauri backend setup queries (`get_queue`, `get_sources`, etc.), and asserts state properties and active tab navigation transitions correctly.
- [AppContent.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/components/AppContent.test.tsx) validates correct routing container mounts.

**Why this matters**
This ensures that core state machinery, view routing, forms setup, and state mutations upon action triggers do not break silently during UI refactoring.

---

### [TEST-002] — Resolved — Coverage — SourcesPanel component has test coverage

**Evidence**
- The component [SourcesPanel.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/components/SourcesPanel.tsx) is now tested in [SourcesPanel.test.tsx](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/components/SourcesPanel.test.tsx).
- The test renders the sources list containing mock data and simulates a click on the delete button to assert that `onDeleteSource` is triggered with the correct ID.

**Why this matters**
This resolves the risk of untested URL list actions or single source form additions failing silently upon component refactoring.

---

### [TEST-003] — Major — Mocking — Unit tests are tightly bound to isolated mocks; integration validation is absent

**Evidence**
- [setup.ts:L4-L21](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src/test/setup.ts#L4-L21) mocks all Tauri APIs globally.
- Frontend unit tests (e.g. `SettingsPanel.test.tsx`) pass callback mock functions (`vi.fn()`) instead of exercising the real integration layer.

**Why this matters**
The frontend tests only verify that components invoke local callbacks when clicked. They do not verify if `useApp.ts` or components make calls to the IPC layer with parameters matching the backend's expected serialization schema. A mismatch in JSON schemas would crash the runtime but pass the tests.

**Blast radius**
- Test files affected: All frontend component test files (11 files)
- Related findings: `TEST-001`

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
- [compiler.rs:L114-L115](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/compiler.rs#L114-L115) was updated to replace both `evidence:` and `evidence://` prefixes.
- However, the regression test in [tests.rs:L406](file:///C:/Users/scott/Documents/antigravity/eager-archimedes/src-tauri/src/core/tests.rs#L406) still only tests the legacy `evidence:1` syntax (`[Building C](evidence:1)`), leaving the `evidence://` translation path untested in the backend test suite.

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

## Shortcut census

| Shortcut pattern | Count |
|---|---|
| `.skip` / `xit` / `@skip` | 0 |
| `.only` (left in) | 0 |
| `TODO: add test` / similar | 0 |
| Empty assertion / placeholder | 0 |
| `--retry` / retries normalized | No |

---

## Blind spots by class

- **External Network Failures**: Lack of mock tests for third-party network issues (e.g. DuckDuckGo being rate-limited or returning unexpected HTML, feed websites returning 500 errors).
- **Tauri IPC Schema Synchronization**: Mismatch between TypeScript `ipc.ts` data structure definitions and Rust `db.rs` structs can occur silently since they are maintained separately and not validated by integration tests.
- **Concurrency and DB Locking**: SQLite DB locking behavior is not tested under concurrent accesses (e.g. browser extension attempting to write via Axum server while the Tauri app is calling ingest or database updates).

---

## Patterns and systemic observations

- **Pure behavioral-mock dependency**: Component tests do not assert real workflows. They only ensure that a button click fires a prop callback, leaving the actual callback implementation in `useApp.ts` entirely unchecked.
- **Ad-hoc parsing logic**: Parsers (HTML scraper, Markdown compiler, DDG search scraper, verbatim matcher) are hand-coded using regular expressions rather than formal parser generators, but they lack the test suites required to ensure regex edge-cases do not cause stack overflows or bypass validations.

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
