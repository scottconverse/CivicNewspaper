# Audit Lite — CivicNews v0.2.5-hotpatch working-tree (post-remediation)
**Date:** 2026-05-28
**Scope:** The uncommitted v0.2.5-hotpatch changes that remediate the full `audit-civicnews-2026-05-28/` package — AppHandle decoupling for 8 cross-platform Rust tests, the plain-language-rewrite diff modal, the shared accessible Modal, the per-platform download resolver, the hardened platform-gate detector, the consolidated single pull command, doc reconciliation, and the OnboardingWizard act() cleanup.
**Reviewer:** Claude (audit-lite)

## TL;DR
Ship. This is a closing pre-commit gate over changes that the full `audit-team` already reviewed and that were then remediated finding-by-finding. Every file in scope is honest and coherent: the "no test-gaming" commitment holds empirically (48 Rust tests pass with **0 ignored** on Windows, the formerly-skipped count is gone), the platform-gate detector is genuinely hardened with a real self-test, and the `section2-auth.json` whitelist is honestly empty. No Blockers, no Criticals. Two Nits, both pre-existing and cosmetic.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 2

## Findings

### NIT-001 Nit: Thinking-out-loud comment left in a Rust test
**Dimension:** Tests
**Evidence:** `src-tauri/src/core/tests.rs:766-768` — `test_daily_scan_parses_fixture_response` carries leftover narration ("// Wait, parse_and_save_scan_response needs tables. Let's create tables."). The first `parse_and_save_scan_response` call at `tests.rs:761` binds its result to `_scan_run` and exercises nothing; the real assertion path (fresh conn + migrations + `assert_eq!(count, 1)`) follows it.
**Why it matters:** Cosmetic only — the test's real assertion is sound and passing. The stray comment and the throwaway first call read like an unfinished edit.
**Fix path:** Delete the `let _scan_run = …` call at `tests.rs:761-765` and the three narration lines at `tests.rs:766-768`; keep the fresh-connection assertion block. Pre-existing, not introduced by this hotpatch.

### NIT-002 Nit: SourcesPanel empty-state colSpan is 6 but the table has 7 columns
**Dimension:** UX
**Evidence:** `src/components/SourcesPanel.tsx:129` — `<td colSpan={6}>` in the "no feeds registered yet" row, while the header (`SourcesPanel.tsx:117-123`) defines 7 columns (Source, URL, Type, Tier, Status, Scraped, Action).
**Why it matters:** Purely visual; the empty-state message just doesn't span the final column. Only shows when zero sources exist. Pre-existing, untouched by this change beyond the Modal migration.
**Fix path:** Change `colSpan={6}` to `colSpan={7}`.

## What's working
- **The anti-gaming claim is empirically true, not asserted.** `cargo test --lib` reports `48 passed; 0 failed; 0 ignored; 0 filtered out`. The 8 tests that previously needed `tauri::test::mock_app()` (and were therefore skipped on Windows) now run on Windows because the business logic was decoupled behind `Arc<dyn LlmClient>`, `PullProgressSink`, and `OllamaSidecar::start_for_test(probe_addr)`. The decoupling is real, not a compile-out. (`src-tauri/src/core/tests.rs:787-1402`, `mutations.json`)
- **TEST-006 hardening is genuine.** `test_daily_scan_command_does_not_panic_when_state_registered` (`tests.rs:925-982`) asserts the scan's lead is actually persisted (`COUNT(*) == 1`, `title == "Council overspend"`, `run_status == "completed"`) rather than a bare `is_ok()` — an Ok-with-zero-rows would have passed the old check and silently dropped every lead.
- **The platform-gate detector is hardened and self-tested.** `reproduction_tests.rs` replaced the old `[^)]*` regex (which stopped at the first `)`) with a balanced-paren `extract_cfg_predicates`; `detector_flags_nested_paren_platform_gate` proves it catches `#[cfg(all(not(debug_assertions), target_os = "macos"))]`-style nested gates while ignoring `cfg(test)`, `cfg(feature)`, and runtime `cfg!()`, and honors the whitelist. `.agent-workflows/section2-auth.json` is honestly `[]`.
- **Pull-command consolidation is clean end-to-end.** Three pull commands collapsed to one structured, cancellable `pull_ollama_model`; `formatPullProgressLine` (`useApp.ts:57-64`) pins the surviving object-payload shape with five unit tests (`formatPullProgressLine.test.ts`), and per-model cancellation / duplicate-rejection / trailing-slash trimming each have real stub-server tests.
- **Diff modal + shared Modal are well-built and accessible.** `computeLineDiff` has LCS edge-case tests (identical / empty / moved-line); the Workbench suite covers open / accept / reject / in-flight / error / disabled-when-empty; `Modal` has 7 a11y tests (dialog semantics, focus-on-open, Esc, Tab/Shift+Tab wrap, focus restoration, body scroll-lock). The diff CSS includes a colorblind-safe `+/-` gutter glyph (`App.css:631-639`), not red/green alone.
- **Docs are reconciled and honest.** The GPU-on-Linux limitation is stated on the landing page (`docs/index.html:115`) and tracked as P5-007 (DEFERRED → v0.3) in `carried-debt.md`; the carried-debt ledger's RESOLVED/DEFERRED tags cross-reference real code pointers.

## Watch items
- `OllamaSidecar::start_for_test` mirrors `start()`'s control flow (minus the orphan sweep). `mutations.json` honestly maps the sidecar tests to `start_for_test`, and the shared `port_in_use` helper carries the testable collision logic — so this is acceptable, but if `start()` gains new pre-spawn logic, the test twin must track it or coverage silently drifts.
- `discover_platform_gate_test_files` keys on the literal `#[cfg(test)]` marker, so a module gated as `#[cfg(all(test, not(windows)))]` could escape discovery. Exotic and not present in the tree, but it's the next nesting shape a future bypass would reach for.

## Escalation recommendation
No escalation needed. The full `audit-team` already ran on this exact scope and its findings were remediated; this lite pass is the pre-commit confirmation. Zero Blockers/Criticals, all gates green, no test-gaming found. The two Nits are pre-existing cosmetic items that don't block the commit.
