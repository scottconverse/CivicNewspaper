# Executive Audit — CivicNewspaper (civicnews v0.2.6)

**Audit date:** 2026-05-28
**Audit scope:** Scoped to the uncommitted working-tree changes on branch `v0.2.5-hotpatch` (the v0.2.6 candidate, 11 files): the AppHandle-decoupling refactor that makes 7 previously platform-gated tests run cross-platform, the new plain-language-rewrite diff modal in `Workbench.tsx`, the per-platform download resolver in `docs/script.js`, and the `carried-debt.md` / whitelist edits.
**Posture:** Balanced (adversarial on test-gaming, per engagement brief)
**Roles engaged:** Principal Engineer, Senior UI/UX Designer, Technical Writer, Test Engineer, QA Engineer

---

## Executive summary

This is an honest, well-disciplined hotpatch, and that is the headline. The central claim — that decoupling business logic from Tauri's `AppHandle` lets seven formerly Windows-`ignore`d tests run cross-platform with no gates, no `mock_app()`, and preserved assertions — was independently verified true on a Windows host by three roles (`cargo test --lib` → 44 passed, **0 ignored**), the assertions are real behavioral checks (not stubs or tautologies), and emptying the platform-gate whitelist to `[]` is truthful for the code as it stands today. For a repo whose own changelog documents five prior pipeline-integrity incidents, finding **no test-gaming in this change set** is the most important result. The one Critical is an accessibility gap on the new diff modal (no focus management, no Esc, no accessible name). The single highest-leverage issue is not in the test bodies but in the guard meant to keep them honest: the platform-gate detector has two independent holes (it scans only 2 files, and its regex is evadable with a nested-paren cfg gate), so the empty whitelist is honest *today* but not durably self-enforcing.

---

## Readiness at a glance

| Dimension | Status | Summary |
|---|---|---|
| Architecture & code | Solid | Clean dependency-inversion seam; refactor is the right shape, not the convenient one. One carried-forward cancellation defect. |
| UI / UX | Concerns | Diff-modal redesign is the right product call, but its a11y, states, and the mobile download layout are unfinished. |
| Documentation | Concerns | RESOLVED claims are accurate; CHANGELOG and three dangling debt refs lag the code. |
| Test suite | Solid (with one integrity caveat) | Tests genuine and non-gamed; the guard protecting the empty whitelist is bypassable. |
| Runtime QA | Solid | Every in-scope runtime check passes green on Windows; download resolver verified live against the real release. |

---

## Severity roll-up

> Counts are by **enumerated findings** across the five deep-dives. (Two deep-dive summary tables overcount themselves by one each — the per-finding counts below are authoritative.)

| Severity | Count | What it means |
|---|---|---|
| Blocker | 0 | Cannot ship / cannot defer |
| Critical | 1 | Fix this sprint |
| Major | 9 | Fix this or next sprint |
| Minor | 15 | Batch for hygiene work |
| Nit | 12 | Preference-level; flag once |
| **Total** | **37** | |

---

## Top 10 findings

> Sorted by severity, then by leverage. If the team fixes only ten things, these deliver the most value.

| # | ID | Severity | Role | Title | Blast |
|---|---|---|---|---|---|
| 1 | UX-001 | Critical | UX | Diff modal has no focus trap, no Esc, no `aria-labelledby` | Keyboard/SR users can tab out of the "modal"; same gap in 3 sibling modals |
| 2 | TEST-001 | Major | Test | Platform-gate detector evadable by nested-paren cfg gate | Empty whitelist passes green while a test is compiled out on Windows |
| 3 | ENG-002 | Major | Engineering | Gate detector only inspects 2 of N test files | A gate in any other module isn't seen; same guard as TEST-001 |
| 4 | ENG-001 | Major | Engineering | Concurrent same-model pull leaks a cancel sender | Cancel button silently no-ops; doc comment over-claims "per-pull isolation" |
| 5 | UX-002 | Major | UX | Download cards don't stack on mobile; recommended card off-screen | Primary install CTA degraded at the decision moment |
| 6 | UX-003 | Major | UX | Diff add/removed signaled by faint color tint only | ~8% of men can't see what the AI changed → accept blind |
| 7 | UX-004 | Major | UX | Diff modal has no in-modal loading/error state; `--color-danger` typo | Slow/failed rewrites give near-invisible, mis-colored feedback |
| 8 | TEST-002 | Major | Test | Three coexisting pull commands; only the new path is tested | Raw-line `pull_model` path wired to `useApp.ts` is uncovered |
| 9 | DOC-001 | Major | Docs | CHANGELOG describes a `mock_app()`/`cfg(unix)`-gated test that no longer exists | New engineer can't tell changelog from code; `mutations.json` stale too |
| 10 | DOC-002 | Major | Docs | P5-002/005/007 deleted from debt ledger without resolution notes | Dangling refs in CHANGELOG/README/FAQ; ledger no longer trustworthy |

---

## Cross-role findings

> Root causes that surfaced independently in more than one role. These are the highest-leverage fixes in the audit.

### The platform-gate detector is the weak link, not the tests (the central integrity finding)
- **Surfaced in:** TEST-001 (Major), ENG-002 (Major); corroborated green-today by QA ("What's working").
- **What it is:** The empty `section2-auth.json` whitelist is honest *now* — every role confirmed zero platform gates in the changed code and 0 ignored tests on Windows. But `verify_no_unauthorized_platform_gates`, the guard that is the entire justification for leaving the whitelist empty, has two independent holes: (a) it is hardcoded to scan only `tests.rs` and `server_tests.rs`, so a gate in `llm.rs` or any future module is invisible (ENG-002); and (b) its gate-matching regex uses `[^)]*`, which stops at the first `)`, so a one-token-deeper expression like `#[cfg(all(not(test), target_os = "linux"))]` compiles a test out on Windows and slips past unflagged — empirically reproduced (TEST-001).
- **Why this is the most important issue:** This repo's documented history is precisely the class of bug this guard exists to prevent — tests compiled out on Windows to fake a green run. A guard with a known hole is more dangerous than no guard because it manufactures false confidence. An agent under pressure to make Windows green can re-introduce the exact failure mode and CI stays green.
- **Blast radius of the fix:** Contained to `reproduction_tests.rs` (one regex + two call sites) and the whitelist it underwrites. No user-facing or migration impact.
- **Recommended approach:** Fix both holes in one coordinated change. Replace the `[^)]*` regex with a balanced-paren extractor that tests the *extracted* `cfg(...)` predicate for any platform token; and make the guard self-discover its inputs (walk `src-tauri/src/**` for `#[cfg(test)]`/`fn test_` rather than two literal paths). Critically, add a unit test *of the detector itself* asserting `#[cfg(all(not(test), target_os = "linux"))]` is flagged — the detector currently has no test of its own. Until then, treat the empty whitelist as provisional and keep a human reviewer on cfg attributes in test files.

### Pull-command sprawl: three commands, divergent payloads, one tested path
- **Surfaced in:** ENG-004 (Minor), TEST-002 (Major).
- **What it is:** `lib.rs` registers `pull_model`, `pull_ollama_model`, and `ollama_pull_model`. They emit the shared `ollama-pull-progress` event in three different shapes (raw string vs structured object vs a third payload). Only the core behind `pull_ollama_model` (the structured, cancellable path used by `OnboardingWizard`) is tested; the raw-line `pull_model` path wired into `useApp.ts` is uncovered, and `ollama_pull_model` appears dead but is still registered.
- **Why this matters:** Two live wire paths with incompatible payload shapes is a latent frontend-bug class, and a regression in the untested path ships silently. Not introduced by this change set (pre-existing), but the change set's correctness depends on which pairing is live.
- **Blast radius of the fix:** `lib.rs` registry, `tauri_cmds.rs` (3 command bodies), `ipc.ts`, `useApp.ts`, `OnboardingWizard.tsx`; only `run_ollama_pull` registers cancellation.
- **Recommended approach:** Collapse to one pull command + one payload shape (route everything through `pull_ollama_model`), delete the other two and their handler lines, then add one frontend test pinning the surviving event-payload shape. Next-sprint work, not this hotpatch.

### Docs and audit tooling lag the renamed/decoupled symbols
- **Surfaced in:** DOC-001 (Major), DOC-002 (Major), and a Test cross-flag.
- **What it is:** The change set correctly advanced the code and `carried-debt.md`/`reproduction_tests.rs`, but did not sweep the other docs that reference the same facts: `CHANGELOG.md` still describes the sidecar test under its old name as `mock_app()`-requiring and `cfg(unix)`-gated; `scripts/audit/mutations.json:52` still maps a mutation to that old test name against the wrong function; and P5-002/005/007 were deleted from the debt ledger leaving dangling references in CHANGELOG/README/FAQ.
- **Why this matters:** In an integrity-sensitive repo a doc/code mismatch reads as a red flag, not a typo. A stale mutation mapping may silently not run.
- **Blast radius of the fix:** Pure doc/config reconciliation; no code or user-facing impact.
- **Recommended approach:** One reconciliation pass keyed off the renamed symbols, plus a uniform `carried-debt.md` disposition rule (an item leaves only when marked RESOLVED-with-pointer or WITHDRAWN-with-reason). Mark P5-005 RESOLVED; keep P5-002/P5-007 as DEFERRED (both still real limitations).

### Diff-modal: right redesign, thin non-happy-path
- **Surfaced in:** UX-001 (Critical), UX-003 (Major), UX-004 (Major); QA-005/TEST-003 note the e2e + LCS-edge-case gaps.
- **What it is:** Replacing a destructive `window.confirm` overwrite with a reviewable accept/reject diff is the correct product decision, and the happy path is polished. The loading, error, empty, and colorblind paths are not: no focus/Esc/accessible-name (UX-001), color-tint-only change signal (UX-003), no in-modal loading/error state plus a `--color-danger` token typo that should be `--color-error` (UX-004); and `computeLineDiff` has no direct unit test for empty/identical/reorder inputs (TEST-003).
- **Blast radius of the fix:** UX-001's root (no shared `<Modal>` primitive, UX-006) means the same a11y gap exists in three sibling modals; a shared wrapper fixes all four at once.
- **Recommended approach:** One focused states-and-signals pass on the component (focus trap + Esc + `aria-labelledby`, a non-color `+`/`−` gutter glyph, in-modal loading/error, the token fix), ideally via a shared `<Modal>` so it propagates. Add the missing state + LCS-edge tests.

---

## What's working

> Specific, honest credit.

- **Engineering:** The refactor is the model of how to retire a platform gate honestly — it removed the *reason* for the gate (AppHandle coupling) via clean seams (`Arc<dyn LlmClient>`, the `PullProgressSink` trait, extracted `port_in_use`), then proved it by running the tests cross-platform. The production wire contract (event names + `{model,status,completed,total}` payload) is preserved exactly; no frontend change was required (`01-engineering-deepdive.md`).
- **UI/UX:** The rewrite-to-diff-modal redesign is the right move for a newsroom tool — non-destructive, reviewable, with honest, instructive microcopy and self-describing pane headers. The download resolver degrades safely, leaving the `releases/latest` fallback in place on any failure (`02-uiux-deepdive.md`).
- **Documentation:** The `carried-debt.md` RESOLVED claims (P5-001/003/004) are accurate line-by-line against the code — the right model to emulate. README's pre-alpha framing and SECURITY.md's sidecar attack-surface section are genuinely strong, honest docs (`03-documentation-deepdive.md`).
- **Tests:** The shortcut census is genuinely clean — zero `.skip`/`.only`/`#[ignore]`/`assert!(true)` in scope. Assertions were preserved, not hollowed: the `FakeLlmClient` panics if the production path passes the wrong model/prompt; the per-pull-cancel test mutates real shared state; the 404 test hits a real axum stub. Ephemeral-port discipline avoids colliding with a developer's live Ollama (`04-test-deepdive.md`).
- **Runtime quality:** Every in-scope runtime check passes on Windows — `cargo test` (44/0/0), `clippy -D warnings`, `fmt --check`, `tsc --noEmit`, `vitest` (32/13). The download resolver was driven **live** against the real GitHub release (win→`.exe`, linux→`.deb`, mac→`.dmg`), with the API-failure and unknown-arch paths both degrading cleanly (`05-qa-deepdive.md`).

---

## This-sprint punch list (summary)

> Full detail and owner hints in `sprint-punchlist.md`.

**Must-fix (all Blockers + Criticals):** 1 item — UX-001 (diff modal focus/Esc/accessible name).
**Should-fix (high-leverage Majors):** 6 items — the platform-gate detector (TEST-001 + ENG-002, fix together), ENG-001 (cancellation key + doc comment), UX-002 (mobile stacking), UX-003 (non-color diff signal), UX-004 (in-modal states + `--color-error` typo), and the doc reconciliation pass (DOC-001 + DOC-002).

---

## Next-sprint watchlist (summary)

> Full detail in `next-sprint-watchlist.md`.

- **Structural:** consolidate the three pull commands to one (TEST-002 / ENG-004); unit-test the orphan-sweep predicate (ENG-003); move `tauri`'s `test` feature dev-only (ENG-008).
- **Design debt:** introduce a shared `<Modal>` primitive (UX-006) so a11y stops drifting per-modal; empty-draft no-op (UX-005); badge clipping (UX-007); ambiguous install links (UX-008).
- **Test-culture:** direct `computeLineDiff` LCS tests (TEST-003); the untested `AppHandlePullSink` emit seam (TEST-004); replace fixed-sleep syncs (TEST-005); land *some* coverage on the zero-tested landing-page resolver (QA pattern).
- **Decisions needing product input:** is AppImage a supported Linux artifact? (QA-003 — resolver prefers it, CI doesn't build it).

---

## Blast-radius callouts

> Fixes that ripple outward — coordinate, don't patch locally.

- **UX-001 → UX-006** — Fixing focus/Esc/ARIA only on the diff modal leaves three sibling modals (`AppContent.tsx`, `SourcesPanel.tsx` ×2) still broken. Fix via a shared `<Modal>` so it lands everywhere.
- **TEST-001 + ENG-002** — Same guard, two holes. Harden the regex *and* make it self-discover its inputs in one change, and add a detector self-test; a partial fix re-opens the door.
- **ENG-001** — Re-keying the cancel map by invocation id changes `test_cancel_ollama_pull_is_per_pull`'s `map.contains_key("model-1")` assertions; update the test in the same change and add a same-model concurrent-pull case.
- **DOC-001** — The stale test name lives in both `CHANGELOG.md` and `scripts/audit/mutations.json:52`; fix both or the mutation mapping silently mismatches.

---

## What we couldn't assess

- **Diff modal end-to-end with a real LLM (UX, QA, Test):** the modal only renders after a real Tauri + Ollama rewrite resolves; component tests mock the IPC boundary, so the React state machine is verified but the real IPC round-trip and a real model's multi-paragraph output through the LCS diff were not driven live (QA-005).
- **Cross-browser / cross-host download resolver (QA):** driven only from a Windows Chromium host; Firefox/Safari (no UA-CH) and real Apple-Silicon arch detection were reasoned from code, not run.
- **CI history / flakiness, coverage % (Test):** single local run; no coverage tooling configured. Two timing-dependent tests are theoretical flake candidates (TEST-005).
- **Live release builds / `cargo audit` (Engineering, QA):** no release bundle produced; dependency CVE status assessed by version inspection only.

All five in-scope roles otherwise completed their audit on the agreed working-tree artifacts.

---

## Recommended next actions (for the tech lead)

1. **Ship the Critical first:** fix UX-001 via a shared `<Modal>` wrapper (closes the a11y gap on all four modals at once), then re-run the Workbench tests with added Esc/focus assertions.
2. **Close the integrity hole in one coordinated change:** harden the platform-gate regex to a balanced-paren parser, make it self-discover all test files, and add a detector self-test (TEST-001 + ENG-002). This is what makes the empty whitelist safe to leave empty.
3. **Finish the diff modal:** non-color diff signal, in-modal loading/error, the `--color-error` token fix, and the cancellation-key fix (UX-003, UX-004, ENG-001).
4. **One doc reconciliation pass:** CHANGELOG + `mutations.json` test-name fix, uniform `carried-debt.md` disposition rule, restore P5-002/P5-007 as DEFERRED, mark P5-005 RESOLVED (DOC-001, DOC-002).
5. **Defer consciously:** pull-command consolidation and the landing-page mobile-stacking fix to next sprint, tracked on the watchlist.

---

## Reference — role deep-dives

- `01-engineering-deepdive.md` — Principal Engineer
- `02-uiux-deepdive.md` — Senior UI/UX Designer
- `03-documentation-deepdive.md` — Technical Writer
- `04-test-deepdive.md` — Test Engineer
- `05-qa-deepdive.md` — QA Engineer

No documentation drafts were produced — writer mode stayed audit-only because no Blocker/Critical doc gap was caused by this change set (the RESOLVED claims under audit are accurate).

---

*Audit conducted by the audit-team skill on 2026-05-28. Findings are balanced and evidence-based. Every Critical and Major includes reproduction details and a blast-radius entry in its deep-dive.*
