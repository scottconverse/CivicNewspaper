# Sprint Punch List — CivicNewspaper (v0.2.6 / branch `v0.2.5-hotpatch`)

**Audit date:** 2026-05-28
**For sprint ending:** v0.2.6 release cut

Actionable fixes for the **current sprint**. Every item has an ID, severity, owner hint (the role that surfaced it), a one-line fix, and a rough size (S/M/L) for planning only. Full detail for each ID is in the matching role deep-dive.

---

## Must-fix (Blockers + Criticals)

| # | ID | Severity | Role | What to do | Size |
|---|---|---|---|---|---|
| 1 | UX-001 | Critical | UX | Give the diff modal a focus trap, Esc-to-close, initial focus on Reject, and `aria-labelledby` pointing at the `<h3>`. Best done by extracting a shared `<Modal>` so the three sibling modals get it too. | M |

---

## Should-fix (high-leverage Majors)

Tackle after the Critical, in roughly this order (the integrity fix is highest leverage).

| # | ID | Severity | Role | What to do | Size |
|---|---|---|---|---|---|
| 1 | TEST-001 + ENG-002 | Major | Test + Eng | One coordinated fix to `verify_no_unauthorized_platform_gates`: replace the `[^)]*` regex with a balanced-paren extractor that tests the extracted `cfg(...)` predicate for any platform token; make the guard self-discover all `#[cfg(test)]` files instead of two hardcoded paths; add a self-test asserting `#[cfg(all(not(test), target_os = "linux"))]` is flagged. | M |
| 2 | ENG-001 | Major | Engineering | Reject/coalesce a duplicate in-flight same-model pull at the `CANCEL_PULL_MAP` insert (or key by invocation id), and soften the `llm.rs:128-133` "per-pull isolation" comment to per-*model*. Add a same-model concurrent-pull test. | S–M |
| 3 | UX-002 | Major | UX | Make download cards stack on mobile: wrap the desktop `repeat(3,1fr)` rule in `@media (min-width:769px)` (or move it before the 768px block). Verify live at 320/375/768. | S |
| 4 | UX-003 | Major | UX | Add a non-color diff signal: leading `+`/`−` gutter glyph (and/or a 3px colored left border) on changed lines; raise tint alpha to ~0.30–0.35. Keep color as the secondary channel. | S |
| 5 | UX-004 | Major | UX | Fix `var(--color-danger)` → `var(--color-error)` at `Workbench.tsx:295`; surface rewrite errors in-context with the existing `<AlertTriangle>`; add an in-flight indicator. Add the missing loading + error-path tests. | S–M |
| 6 | DOC-001 + DOC-002 | Major | Docs | One reconciliation pass: rewrite the stale `CHANGELOG.md:12` sidecar-test entry to match shipped code; fix the test name + target fn in `scripts/audit/mutations.json:52`; mark P5-005 RESOLVED, restore P5-002/P5-007 as DEFERRED, fix the dangling `CHANGELOG.md:65` "Tracked as P5-007" ref. | S |

---

## Suggested sequencing

1. **UX-001 first, via a shared `<Modal>` (UX-006).** It's the only Critical and the wrapper is the substrate for the rest of the diff-modal work — building it first means UX-003/UX-004's component changes land in the finished container, and the three sibling modals inherit the a11y fix for free.
2. **TEST-001 + ENG-002 next.** Highest leverage in the audit: it's what makes the empty whitelist safe to leave empty. Independent of the UI work, so it can run in parallel with a different owner. Add the detector self-test in the same change — without it the fix isn't verifiable.
3. **ENG-001** alongside the detector work (both are Rust/`src-tauri`, same owner can batch them); remember it changes the existing per-pull-cancel test's assertions.
4. **UX-002, UX-003, UX-004** as the diff-modal/landing-page states pass, after the `<Modal>` exists.
5. **DOC-001 + DOC-002 last**, once the code is final, so the reconciliation reflects what actually shipped. Coordinate with whoever touches the test rename so `mutations.json` and `CHANGELOG.md` are fixed together.

**Dependencies:** UX-003/UX-004 land cleanest *after* UX-001's `<Modal>` exists. ENG-001's fix invalidates `test_cancel_ollama_pull_is_per_pull`'s `contains_key` assertions — update in the same change. DOC-001's stale test name must be fixed in both `CHANGELOG.md` and `mutations.json`.

---

## Items deferred to next sprint

Consciously considered and moved to `next-sprint-watchlist.md`:

- TEST-002 / ENG-004 — pull-command consolidation: structural, touches 3 commands + 2 frontend call sites; clean it as one deliberate refactor, not a hotpatch edit.
- ENG-003 — orphan-sweep predicate unit test: the omission in `start_for_test` is the correct call; the missing coverage is hygiene.
- TEST-003 / TEST-004 / TEST-005 — `computeLineDiff` edge tests, the `AppHandlePullSink` emit seam, fixed-sleep flakiness: test-culture debt, not release-blocking.
- UX-005 / UX-007 / UX-008 / UX-006 — empty-draft no-op, badge clipping, ambiguous install links, the shared-Modal refactor itself (beyond what UX-001 needs).
- QA-001 / QA-002 / QA-003 — mac arch keys off host, non-Chromium fallback, AppImage-vs-deb target drift: low live impact; QA-003 needs a product decision.

---

## Sign-off gate

The sprint is not done until:

- [ ] UX-001 fixed and verified with keyboard + screen reader in the running Tauri app (not just the unit test).
- [ ] The platform-gate detector hardened **and** its own self-test added and passing (TEST-001 + ENG-002).
- [ ] ENG-001 fixed with the per-pull-cancel test updated and a same-model case added.
- [ ] Diff-modal states pass complete (UX-002/003/004) with the new loading + error tests green.
- [ ] Doc reconciliation done — CHANGELOG, `mutations.json`, and `carried-debt.md` all consistent with shipped code.
- [ ] Full suite re-run green on Windows: `cargo test --lib`, `clippy -D warnings`, `fmt --check`, `tsc --noEmit`, `vitest`.

---

*Generated from the `audit-team` skill. Full detail for every ID is in the matching role deep-dive (`01-engineering-deepdive.md`, `02-uiux-deepdive.md`, `03-documentation-deepdive.md`, `04-test-deepdive.md`, `05-qa-deepdive.md`).*
