# Next-Sprint Watchlist — CivicNewspaper (v0.2.6)

**Audit date:** 2026-05-28

Forward-looking items from the v0.2.6 hotpatch audit. These don't belong in the current sprint — they need cross-cutting refactoring, a shared primitive, or a product decision — but they must be on the radar for the next planning cycle. Several are pre-existing (not introduced by this change set) and are flagged because the in-scope work touched their neighborhood.

---

## Structural / architectural

| # | ID | Role | What to consider | Trigger to act |
|---|---|---|---|---|
| 1 | TEST-002 / ENG-004 | Test + Eng | Collapse the three pull commands (`pull_model`, `pull_ollama_model`, `ollama_pull_model`) to one command + one payload shape; route `useApp.ts` through the structured, cancellable, tested path; delete the other two and their handler registrations; add one frontend test pinning the surviving event-payload shape. | Before adding any new model-management UI, or the next time the download/progress flow is touched. |
| 2 | ENG-003 | Engineering | Extract the orphan-`ollama serve` sweep predicate into a pure function and unit-test it with synthetic process data. `start_for_test` correctly omits the sweep, so the production sweep is untested on every platform. | When next editing sidecar lifecycle, or if a cross-OS process-reaping bug is reported. |
| 3 | ENG-008 | Engineering | Move `tauri`'s `test` feature from the main dependency to a dev-only activation so the Tauri test harness stops compiling into release builds. The decoupling work reduces reliance on `mock_app()`, which may now make this feasible. | Before the next release-size pass or dependency cleanup. |

## Design debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | UX-006 | UX | Introduce `src/components/Modal.tsx` owning overlay + `role="dialog"` + `aria-labelledby` + focus trap + Esc + body scroll-lock, and migrate all four hand-rolled modals (`Workbench.tsx`, `AppContent.tsx`, `SourcesPanel.tsx` ×2) to it. This is the durable fix behind UX-001; doing it once stops a11y drifting per-modal. |
| 2 | UX-005 | UX | Disable "Plain Language Rewrite" when the draft is empty (`disabled={isRewriting \|\| !selectedDraft.content}`) with an explanatory tooltip, instead of a silent no-op. |
| 3 | UX-007 | UX | Harden the "Recommended Platform" badge: `white-space: nowrap`, enough top padding/`overflow` so the `-11px` `::after` offset isn't clipped even in valid layouts. Mostly resolves once UX-002 ships. |
| 4 | UX-008 / UX-011 | UX | Give the three identical "First time installing?" links platform-specific `aria-label`s; add tuned `:focus-visible` rings to download buttons/callout links to match the nav hamburger treatment. (Separately, out-of-scope: `install.md` is served as raw markdown — render it as HTML.) |

## Documentation debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | DOC-004 | Docs | Add a one-line status convention to the top of `carried-debt.md` (`OPEN \| RESOLVED (vX.Y, <pointer>) \| WITHDRAWN (<reason>)`) and tag every item, so ledger state is unambiguous at a glance. Pairs with the DOC-002 sprint fix. |
| 2 | DOC-003 | Docs | Add one honest line near the landing-page Ollama feature card or in `install.md`: inference runs on CPU by default; GPU acceleration is not yet bundled on Linux. Keeps marketing in lockstep with `CHANGELOG.md:65`. |
| 3 | DOC-006 | Docs | If `carried-debt.md` is ever to be externally publishable, split the backward-looking "Pipeline Integrity Incidents" section into `forensic/` and keep the debt file purely forward-looking. Leave as-is if internal. |

## Test-culture debt

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | TEST-003 | Test | Add a focused unit test of `computeLineDiff` (identical → all `same`; empty rewrite → all `removed`; empty original → all `added`; a multi-line reorder asserting exact `{text,type}` rows). The LCS is non-trivial and currently only hit by two happy-path renders. |
| 2 | TEST-004 | Test | Either extract the `AppHandlePullSink` event-name/payload mapping into a pure, tested function, or add a Tauri-runtime integration test, or record it explicitly in `carried-debt.md` as a known untested core↔frontend seam. |
| 3 | TEST-005 | Test | Replace the two fixed-sleep synchronizations (`test_cancel_ollama_pull_is_per_pull` 200ms; `test_ollama_sidecar_terminates_cleanly_on_drop` single sysinfo poll) with bounded poll-until-condition loops to pre-empt CI flakiness. |
| 4 | QA-pattern / TEST | QA + Test | The landing-page download resolver (`pickWindowsAsset`/`pickLinuxAsset`/`pickMacAsset`/`detectMacArch`) has **zero** automated coverage — every branch was verified manually this audit. The functions are pure; extract from the `DOMContentLoaded` closure and unit-test them. This is why QA-001/002/003 are observations, not caught-by-CI regressions. |
| 5 | TEST-007 / QA-004 | Test + QA | Resolve the `act(...)` warnings from `OnboardingWizard.test.tsx` (wrap timer-driven state updates in `await waitFor`/`act`) to keep the test console clean. |

## Performance and scaling

| # | ID | Role | What to consider | Trigger to act |
|---|---|---|---|---|
| 1 | QA-005 / TEST-003 | QA + Test | Drive the diff modal end-to-end against a real LLM rewrite (Tauri integration test or `tauri dev` smoke with a stubbed client): verify O(n·m) LCS performance and rendering on a genuinely long multi-paragraph article, scroll behavior, and the IPC error path in-modal. | Before a release that markets the rewrite feature, or if editors report slow/garbled diffs on long drafts. |

## Dependency and supply chain

| # | ID | Role | What to consider |
|---|---|---|---|
| 1 | (general) | Engineering | No acute dependency risk in scope — no new third-party deps added by this change set; versions are current (`reqwest` 0.12, `axum` 0.7, `tokio` 1, `async-trait` 0.1). Run `cargo audit` / `npm audit` next cycle to close the CVE-status gap this audit couldn't check. |

## Decisions needing product/leadership input

- **QA-003** — Is AppImage a supported Linux artifact? `docs/script.js` and `scripts/verify-release.sh` both assume AppImages may exist, but `tauri.linux.conf.json` builds `deb` only. Either add `"appimage"` to the Linux targets (so the resolver's preference is honored end-to-end) or drop the `.appimage` branch as dead code. A product/release-owner call, not a pure engineering fix.
- **ENG-005** — Optional defense-in-depth: a one-line `https://` scheme check before assigning `browser_download_url` to a button href. Low/theoretical risk (trusted first-party API); decide whether it's worth the cheap guard.

---

## Review cadence

Revisit this watchlist at:
- Next sprint planning — elevate anything acute (the pull-command consolidation is the most likely candidate).
- Every quarter — retire what's addressed, re-confirm what's still current.
- On any change to sidecar lifecycle, the modal system, or the release pipeline — re-audit the relevant section.

---

*Generated from the `audit-team` skill. Each entry cross-references its full treatment in the relevant role deep-dive.*
