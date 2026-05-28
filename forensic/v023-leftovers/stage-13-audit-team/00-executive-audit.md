# Executive Audit — CivicNewspaper

**Audit date:** 2026-05-26
**Audit scope:** Targeted Release Audit of v0.2.0 State (Phase 4 Remediations, Ollama Sidecar, Onboarding Model-Pull, Documentation, and Landing Page)
**Posture:** Balanced
**Roles engaged:** Principal Engineer, UI/UX Designer, Technical Writer, Test Engineer, QA Engineer

---

## Executive summary

Following a comprehensive audit of the v0.2.0 state, the codebase successfully addresses historical vulnerabilities (such as arbitary file writes in diagnostic exports and mobile main-content squeezed views) and establishes a solid architectural core. However, critical gaps in the new Phase 4 features prevent release: initiating a Daily Scan panics the Rust backend and crashes the app immediately due to an unmanaged state wrapper, database migrations block upgrades for existing databases due to SQLite foreign key constraints, and critical validation tests are mocked fakes that assert themselves instead of executing code. Finally, the required Prompt Library dropdown UI is completely missing, and mobile responsive collapses hide all landing page navigation controls. The app is **NOT READY TO SHIP** without remediation.

---

## Readiness at a glance

| Dimension | Status | Summary |
|---|---|---|
| Architecture & code | **Serious issues** | Tauri managed state panic on scan; foreign-key violations break migration upgrades. |
| UI / UX | **Serious issues** | Missing Prompt Library UI; destructive plain-language overwrite; mobile landing page is broken. |
| Documentation | **Concerns** | Database file paths in the manual refer to the wrong Tauri bundle identifier. |
| Test suite | **Serious issues** | Key LLM mock tests call themselves instead of the codebase; migration test bypasses constraints. |
| Runtime QA | **Serious issues** | Absent Origin headers block developer CLI tools; model pull fails silently on Ollama errors. |

---

## Severity roll-up

| Severity | Count | What it means |
|---|---|---|
| Blocker | 4 | Cannot ship / cannot defer |
| Critical | 8 | Fix this sprint |
| Major | 10 | Fix this or next sprint |
| Minor | 6 | Batch for hygiene work |
| Nit | 1 | Preference-level; flag once |
| **Total** | **29** | |

---

## Top 10 findings

| # | ID | Severity | Role | Title | Blast |
|---|---|---|---|---|---|
| 1 | [ENG-012] | Blocker | Engineering | System Panic on Daily Scan due to unmanaged `Arc<dyn LlmClient>` state | Daily Scan crashes the app instantly |
| 2 | [UX-015] | Blocker | UI/UX | Hardcoded Gemma2 Model Ingestion Forced on Low-RAM Systems | Low-spec systems freeze/OOM |
| 3 | [TEST-011] | Blocker | Test | Mock LLM Client tests itself in plain-language rewrite unit test | Bypasses actual code coverage |
| 4 | [QA-002] | Blocker | QA | Missing Origin Header causes 403 Forbidden on Axum Loopback | Blocks coding assistant CLI bridge |
| 5 | [ENG-013] | Critical | Engineering | Database migration failure on upgrade due to FK violations | Startup crashes on existing upgrades |
| 6 | [ENG-014] | Critical | Engineering | Silent failure of onboarding model pull if Ollama is offline | Onboarding hangs permanently |
| 7 | [UX-017] | Critical | UI/UX | Jarring and Destructive Blind Text Overwrite in Rewrite Dialog | High risk of user draft data loss |
| 8 | [UX-013] | Critical | UI/UX | Navigation Links and Primary CTAs Hidden on Mobile Landing Page | Mobile landing visitors cannot navigate |
| 9 | [TEST-014] | Critical | Test | Phase 4 Prompt Library dropdown UI is completely missing | Core feature absent from UI |
| 10 | [DOC-010] | Critical | Docs | Incorrect App Data Paths in User Manual | Users cannot find database backup files |

---

## Cross-role findings

### Mock Self-Validation and Functional Gaps
- **Surfaced in:** [TEST-011, TEST-014, TEST-016, UX-017, QA-013, QA-014]
- **What it is:** In trying to complete Phase 4 prompt and rewrite features quickly, functional implementations were skipped or hardcoded. The Prompt Library UI dropdown was completely omitted, the backend prompts list and rewrite inputs were hardcoded to aggregator stubs, and unit tests were written to call local mock methods directly instead of invoking the application.
- **Why this is the most important issue:** It masks critical functional absences as passing, bypassing CI checks.
- **Blast radius of the fix:** Story workbench (`Workbench.tsx`), backend commands (`tauri_cmds.rs`), and prompt library APIs (`prompts.rs`).
- **Recommended approach:** Complete the full prompt manager implementation, wire the actual template loader to the rewrite commands, build the dropdown UI selector, and rewrite the mock tests to pass handles to the Tauri application layer.

---

## What's working

- **Engineering:** Trait-based `LlmClient` abstraction decouples provider logic, and sidecars terminate cleanly on quit.
- **UI/UX:** High contrast ratios, beautiful Gazatte newspaper theme, and correct relative subdirectory asset routing.
- **Documentation:** Robust installation guides for smart-install Gatekeeper/SmartScreen bypasses.
- **Tests:** Sound migration version rollback tests and thorough Axum auth/rate-limiting oneshot tower testing.
- **Runtime quality:** Correct folder compilation and quiet feed detection lead generators.

---

## This-sprint punch list (summary)

**Must-fix (all Blockers + Criticals):** 12 items (See details in `sprint-punchlist.md`)
1. [ENG-012] Fix managed state registration for `Arc<dyn LlmClient>` in `lib.rs`.
2. [UX-015] Dynamically resolve and download the recommended model in onboarding step 3.
3. [TEST-011] Update mock rewrite tests to invoke real command handlers.
4. [QA-002] Allow loopback requests with absent Origin headers.
5. [ENG-013] Execute migration 0007 table modifications with temporary `PRAGMA foreign_keys = OFF`.
6. [ENG-014] Propagate status errors to the wizard progress bar on model pull failures.
7. [UX-017] Implement a side-by-side preview diff for plain-language rewrites before overwriting.
8. [UX-013] Prevent CSS collapse of primary buttons and landing links on mobile widths.
9. [TEST-014] Add the Prompt Library selection UI dropdown to `Workbench.tsx`.
10. [DOC-010] Correct the database directories in the user manual to use `org.civicnews.app`.
11. [TEST-012] Correct the execution sequence and assertions of `test_source_tier_backfill_media_lead`.

---

## Next-sprint watchlist (summary)

- [ENG-003] Wrap scraper checks and inserts in database batch transactions to optimize disk write locks.
- [ENG-004] Configure download buffer stream limit checks to avoid OOM crashes.
- [ENG-005] Replace custom regex HTML cleaners with clean parsing libraries.
- [TEST-003] Set up integration tests at the IPC boundary.

---

## What we couldn't assess

- All in-scope roles completed their audit on the agreed artifacts. Running UI execution behavior was verified through thorough static source code and test assertion reviews.

---

## Recommended next actions (for leadership, PM, or tech lead)

1. **Halt Release**: Do not tag v0.2.0 or push the branch to main until the 4 Blockers and 8 Criticals are remediated.
2. **Remediate Core Backend Faults**: Execute the state registration (`ENG-012`), SQLite migration foreign key constraints (`ENG-013`), and loopback origin validation (`QA-002`) changes immediately.
3. **Align Prompts and Rewrite Integration**: Build the missing dropdown selector, load actual templates, and write honest integration tests for rewrite and prompt lists.

---

## Reference — role deep-dives

- `01-engineering-deepdive.md` — Principal Engineer
- `02-uiux-deepdive.md` — Senior UI/UX Designer
- `03-documentation-deepdive.md` — Technical Writer
- `04-test-deepdive.md` — Test Engineer
- `05-qa-deepdive.md` — QA Engineer
