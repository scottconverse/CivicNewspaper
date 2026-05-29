# QA Engineer Deep-Dive — CivicNewspaper

**Audit date:** 2026-05-28
**Role:** QA Engineer
**Scope audited:** Runtime-observable effects of the uncommitted working-tree changes on branch `v0.2.5-hotpatch`. Specifically: (1) the Rust lib test suite's cross-platform claim on Windows, (2) the frontend type-check + component test suite, (3) the landing-page per-platform download resolver (`docs/script.js` + `docs/index.html`) driven live against the real GitHub releases API, and (4) the plain-language-rewrite diff modal in `Workbench.tsx` (component-test + code level only — see limitation note).
**Environment:** Windows 11 Pro (10.0.26200), x86_64. Rust via `cargo` (dev profile). Node/npm with `npx tsc`/`vitest`. Landing page served by `npx serve` on `http://localhost:4406` (launch config `civicnewspaper-docs`), driven through the preview browser (Chromium engine, `navigator.platform = "Win32"`, UA-CH `getHighEntropyValues` available). Tauri app version 0.2.6; latest published release `v0.2.4-hotpatch`.
**Auditor posture:** Balanced

---

## TL;DR

Every runtime check in scope passes. `cargo test --lib` runs **44 tests, 0 failed, 0 ignored** on Windows — the central claim of this hotpatch (seven formerly Windows-`ignore`d tests now run cross-platform) is verified true at runtime, not just on paper. `cargo clippy --all-targets -D warnings`, `cargo fmt --check`, `tsc --noEmit`, and `vitest` (32 tests, 13 files, including the three new diff-modal tests) all pass green. The landing-page download resolver was driven **live against the real GitHub release** and correctly resolved Windows→`_x64-setup.exe`, Linux→`_amd64.deb`, macOS→a `.dmg`, with the unknown-arch and API-failure paths both degrading cleanly to the `releases/latest` fallback. No Blockers, no Criticals. The findings are all Minor/Nit: a small set of edge-behavior observations (mac arch keys off the *visitor's* host arch; an out-of-scope `act()` warning in a neighboring test file; the resolver matches a target shape — AppImage — that CI does not actually build). The diff modal could not be exercised truly end-to-end because it requires the Tauri backend + Ollama sidecar to produce a real rewrite; it is assessed from its passing component tests and source.

## Severity roll-up (QA)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 0 |
| Major | 0 |
| Minor | 3 |
| Nit | 2 |

## What's working

- **Cross-platform test claim is real on Windows.** `cd src-tauri && cargo test --lib` → `test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`. All seven previously Windows-gated tests now appear in the run and pass: `test_plain_language_rewrite_invokes_ollama`, `test_daily_scan_command_does_not_panic_when_state_registered`, `test_daily_scan_uses_settings_model_not_hardcoded`, `test_ollama_sidecar_spawns_with_expected_pid_pattern`, `test_ollama_sidecar_terminates_cleanly_on_drop`, `test_pull_ollama_model_propagates_http_error`, `test_cancel_ollama_pull_is_per_pull`, plus `test_sidecar_skips_spawn_when_port_occupied`. The `0 ignored` count is the load-bearing evidence — no test silently skipped on this platform.
- **The decoupling refactor didn't break the gate-detector.** `reproduce_m1_cfg_family_bypass` (which scans `tests.rs` for unauthorized platform `cfg` gates) passes, and `.agent-workflows/section2-auth.json` is now `[]` — the whitelist is empty and the guard still green, so the suite can't silently re-introduce a Windows gate without tripping a test.
- **No-collision design verified.** The sidecar tests bind an OS-assigned ephemeral port (`127.0.0.1:0`) rather than the real `11434`, and `start_for_test` injects the probe address and skips the orphan-process sweep. Running the suite on a machine with a live `ollama serve` will neither bind 11434 nor kill the developer's process — confirmed by reading the test bodies and by the suite completing in 0.43s with no port errors.
- **Static analysis is clean.** `cargo clippy --all-targets -- -D warnings` exits 0; `cargo fmt --check` exits 0; `npx tsc --noEmit` exits 0. No warnings-as-errors, no formatting drift, no type errors introduced by the refactor.
- **Download resolver works live against the real release.** Driven on the running page (not simulated), the three buttons resolved to real `browser_download_url`s: win→`CivicNewspaper_0.2.4_x64-setup.exe`, linux→`CivicNewspaper_0.2.4_amd64.deb`, mac→`CivicNewspaper_0.2.4_x64.dmg`. Console was clean (no errors). The `.sig` updater artifacts (if present) are correctly ignored — the picker matches `.exe`/`.msi`/`.deb`/`.dmg`/`.appimage` suffixes, never `.sig`.
- **Graceful degradation on failure is real.** With `fetch` monkey-patched to reject, the resolver's catch fired and all three buttons retained the HTML `releases/latest` fallback href. The unknown-mac-arch branch returns `null`, and `setButtonHref` no-ops on `null`, so a multi-dmg release viewed by an unknown-arch visitor lands on `releases/latest` to choose — never a wrong-arch dmg.
- **Diff-modal component tests cover the three real paths.** `vitest` runs the new tests for (a) rewrite opens a modal without mutating the draft, (b) Accept applies via `onUpdateDraftContent`, (c) Reject discards. The destructive `window.confirm("...cannot be undone")` overwrite was removed; the modal is now the gate. All pass.

## What couldn't be assessed

- **The diff modal end-to-end with a real LLM rewrite.** Exercising the modal against a genuine Ollama-produced rewrite requires the Tauri backend + bundled Ollama sidecar running; a plain Vite/browser session cannot invoke the `plain_language_rewrite` Tauri command. The component tests mock `plainLanguageRewrite` from `../ipc`, so the React rendering, diff computation, and Accept/Reject wiring are verified, but the *real IPC round-trip and a real model's output through the LCS diff* are not. Stated explicitly per scope. This is a known limitation, not a defect.
- **macOS-host and Linux-host behavior of the download resolver.** Tested only from a Windows host. The mac-arch detection path (`detectMacArch` → UA Client Hints) was observed returning the host's x86 arch on this Windows machine; its Apple-Silicon branch was verified by feeding `arch === 'arm'` through the picker logic, not by running on real Apple Silicon. See QA-001.
- **Cross-browser landing page.** Driven only through the preview browser's Chromium engine. Firefox/WebKit (which lack `navigator.userAgentData`) were not driven live; their behavior is inferred from code (they hit the `detectMacArch` catch → `''` → unknown-arch fallback). See QA-002.
- **The actual installer artifacts.** No release build was produced or downloaded; `verify-release.sh` was read, not executed (it requires built bundles). Asset-name-shape correctness was assessed by matching the resolver regex against the real published asset names and the Tauri config targets. See QA-003.

---

## Product shape

CivicNewspaper is a Tauri v2 + React 19/TS local-first desktop app with a Rust core (Axum loopback server, SQLite, bundled Ollama sidecar) and a static GitHub-Pages marketing/docs site under `docs/`. This hotpatch touches three runtime surfaces: the Rust unit-test suite (a build/CI surface), the React component suite + one component's behavior (the diff modal), and the static landing page's client-side download resolver. QA focused on actually running each surface on the host platform and driving the landing page live against the production GitHub release.

## Flows exercised

| Flow | Result | Findings |
|---|---|---|
| `cargo test --lib` on Windows (cross-platform claim) | Pass — 44 passed, 0 ignored | — |
| `cargo clippy --all-targets -- -D warnings` | Pass — exit 0 | — |
| `cargo fmt --check` | Pass — exit 0 | — |
| `npx tsc --noEmit` | Pass — exit 0 | — |
| `npx vitest run` | Pass — 32 passed / 13 files | QA-004 (out-of-scope warning) |
| Landing page: Windows download resolves live | Pass — `_x64-setup.exe` | — |
| Landing page: Linux download resolves live | Pass — `_amd64.deb` | QA-003 |
| Landing page: macOS download resolves live | Pass — a `.dmg` | QA-001 |
| Landing page: API-failure fallback | Pass — all 3 retain `releases/latest` | — |
| Landing page: unknown-arch mac fallback | Pass — `null` → fallback | — |
| Diff modal: open / Accept / Reject (component tests) | Pass | QA-005 (e2e gap) |

## Adversarial scenarios exercised

| Scenario | Outcome | Findings |
|---|---|---|
| Force `fetch` to reject mid-resolve | Catch fired; all 3 buttons kept `releases/latest` fallback; no broken href | — (works as designed) |
| Feed resolver `.exe` + `.exe.sig` (updater artifacts) | `.sig` ignored, `.exe` picked | — |
| Feed resolver MSI-only release (no `.exe`) | Falls through to `.msi` | — |
| Feed resolver empty asset list | `null` everywhere → fallback preserved | — |
| Unknown mac arch with two dmgs present | Returns `null` → fallback (never a wrong-arch dmg) | — |
| AppImage + deb both present (Linux) | Prefers AppImage | QA-003 |
| Run suite while a real `ollama serve` could hold 11434 | Tests use ephemeral ports + skip orphan sweep; no collision/kill | — (works as designed) |

---

## Findings

> **Finding ID prefix:** `QA-`
> **Categories:** Flow / API / Security / Performance / Browser / Mobile / Console / Protocol / Install / Auth

### [QA-001] — Minor — Browser — macOS download keys off the *visitor's* host architecture, not a macOS arch

**Evidence**
1. Started `civicnewspaper-docs` on `http://localhost:4406`, loaded the page, waited for the async resolver to settle.
2. Read resolved hrefs via `preview_eval`. On this Windows x86_64 host, `#download-mac .download-btn` resolved to `CivicNewspaper_0.2.4_x64.dmg`.
3. `navigator.userAgentData.getHighEntropyValues(['architecture'])` is available (`_uaData: true`) and on this Windows machine reports an x86 architecture, so `detectMacArch()` returns `'x86'` and `pickMacAsset` selects the x64 dmg.
4. Re-ran the picker logic verbatim with `arch === 'arm'` → `CivicNewspaper_0.2.4_aarch64.dmg`; with `arch === 'x86'` → `CivicNewspaper_0.2.4_x64.dmg`; with `arch === ''` → `null` (fallback). Confirmed at `docs/script.js:117-131`.

**Why this matters**
`detectMacArch()` reads the architecture of the machine the browser is running on — there is no check that the visitor is actually on macOS. A Windows or Linux visitor who clicks "Download for macOS" gets a dmg matched to *their own* CPU arch (here, the x64 dmg), not necessarily the right Mac build. In practice this is low-impact: a non-Mac visitor clicking the macOS button is already off the happy path, and the link still points at a valid, downloadable dmg. The genuine Mac case is the one that matters, and on a real Apple-Silicon Mac UA-CH reports `arm`, yielding the aarch64 dmg correctly. So the *intended* user is served correctly; the oddity only surfaces for cross-platform curiosity clicks. Flagging as Minor for awareness, not as a defect to rush-fix.

**Blast radius**
- Adjacent code: `docs/script.js` `detectMacArch()` (lines 86-99) and `pickMacAsset()` (117-131). No other consumer.
- User-facing: only the macOS button's resolved href, and only for non-Mac visitors. Mac visitors unaffected.
- Migration: none.
- Tests to update: none (no test covers the landing-page resolver — see Patterns).
- Related findings: QA-002 (browsers without UA-CH).

**Fix path**
Optional. If desired, gate `detectMacArch()` behind a macOS check (`/mac/i.test(navigator.platform)` or the same detection used for the `highlighted` class at `script.js:10-21`) and, for non-Mac visitors, return `''` so the macOS button falls back to `releases/latest`. Given the low impact, "won't fix / document" is a defensible call.

### [QA-002] — Minor — Browser — Firefox/Safari visitors on macOS always get the unknown-arch fallback for the mac button

**Evidence**
1. `detectMacArch()` (`docs/script.js:86-99`) relies on `navigator.userAgentData.getHighEntropyValues`. UA Client Hints are a Chromium-only API; Firefox and Safari do not implement `navigator.userAgentData`.
2. In those browsers the `if (uaData && uaData.getHighEntropyValues)` guard is false, the function returns `''` (unknown), and with the current release carrying *two* dmgs, `pickMacAsset` returns `null` → the mac button keeps the `releases/latest` fallback.
3. Verified the branch by feeding `arch === ''` to the verbatim picker: returns `null` (see QA-001 step 4). Not driven live in Firefox/WebKit — those engines were not available in the preview browser (Chromium only).

**Why this matters**
A Mac user on Safari (the default macOS browser) or Firefox will not get a one-click arch-correct dmg; they'll land on the GitHub releases page and must pick `_aarch64.dmg` vs `_x64.dmg` themselves. That's a degraded — but not broken — experience, and it's arguably the *safer* default (no risk of a wrong-arch download). It only bites because the current release ships two dmgs; a single-dmg release would resolve directly regardless of browser (`pickMacAsset` returns the lone dmg when `dmgs.length === 1`, `script.js:119`).

**Blast radius**
- Adjacent code: `docs/script.js` `detectMacArch()` (86-99). Windows/Linux pickers are unaffected (they key off asset suffix, not arch).
- User-facing: macOS button on non-Chromium browsers → `releases/latest` instead of a direct dmg.
- Migration: none.
- Tests to update: none.
- Related findings: QA-001.

**Fix path**
Optional. If a direct download for Safari/Firefox Mac users is wanted, supplement UA-CH with a parse of `navigator.userAgent` (e.g. presence of "Intel" vs absence on some Apple-Silicon UAs is unreliable, so this is genuinely hard — the current "send them to the release page" degrade is a reasonable conservative choice). Recommend documenting the intended behavior rather than over-engineering arch sniffing.

### [QA-003] — Minor — Install — Resolver prefers an AppImage that CI does not currently build

**Evidence**
1. `pickLinuxAsset` (`docs/script.js:109-115`) prefers `.appimage` and only falls back to `.deb`.
2. `src-tauri/tauri.linux.conf.json` overrides `bundle.targets` to `["deb"]` only — no AppImage target. The base `tauri.conf.json` has `"targets": "all"`, but the Linux-specific config narrows it to deb.
3. The real latest release's Linux asset is `CivicNewspaper_0.2.4_amd64.deb` (no AppImage). Driven live, the Linux button correctly resolved to `_amd64.deb` because the AppImage `find` returned undefined and the code fell through to deb.
4. `scripts/verify-release.sh` (lines 60, 113-123) *does* handle `.AppImage` artifacts, so the release-verification tooling anticipates AppImages even though the Linux bundle config doesn't emit them — a latent inconsistency between the verify script's expectations and the actual build targets.

**Why this matters**
No live impact today: with no AppImage in the release, the resolver falls back to `.deb` correctly, and the Linux button works. The note is about *intent drift* — `script.js` and `verify-release.sh` both assume AppImages may exist, but `tauri.linux.conf.json` only builds deb. If someone later flips the Linux target to also build AppImage, the resolver will silently start preferring it (which may or may not be intended); conversely the AppImage preference is currently dead code. Worth a one-line decision: are AppImages a supported Linux artifact or not?

**Blast radius**
- Adjacent code: `docs/script.js:109-115` (AppImage preference), `scripts/verify-release.sh:60,113-123` (AppImage extraction), `src-tauri/tauri.linux.conf.json` (targets).
- Shared state: the Linux bundle target list is the single source of truth that all three reference inconsistently.
- User-facing: none today; the deb fallback is correct.
- Migration: none.
- Tests to update: none (no test covers the resolver).
- Related findings: none.

**Fix path**
Pick one: (a) if AppImage is intended, add `"appimage"` to `tauri.linux.conf.json` targets so the preference is honored end-to-end; or (b) if deb is the only supported Linux artifact, drop the `.appimage` branch from `pickLinuxAsset` to remove the dead preference. Either way, align the three files.

### [QA-004] — Nit — Console — `act(...)` warning in vitest run originates from an out-of-scope test file

**Evidence**
1. `npx vitest run` passes (32 passed / 13 files) but emits repeated `An update to OnboardingWizard inside a test was not wrapped in act(...)` warnings.
2. Per the vitest output, the warnings are tagged to `src/components/OnboardingWizard.test.tsx > ... > test_onboarding_step2_timeout_shows_retry`, **not** the in-scope `Workbench.test.tsx`. The Workbench tests (including the three new diff-modal tests) run clean.

**Why this matters**
Pre-existing noise unrelated to this hotpatch; the in-scope Workbench tests are clean. Flagged only so the reviewer doesn't attribute the warning to the diff-modal work. Worth fixing eventually (wrap the timer-driven state update in `act`/`waitFor`) to keep the test console clean.

**Fix path**
Out of scope for this hotpatch. When touched: wrap the timeout-driven state update in `OnboardingWizard.test.tsx` in `await waitFor(...)` / `act(...)`.

### [QA-005] — Nit — Flow — Diff modal cannot be driven end-to-end without the Tauri + Ollama runtime

**Evidence**
1. `Workbench.tsx:296-...` invokes `await import('../ipc')` → `plainLanguageRewrite(selectedDraft.content, selectedDraft.format)`, which is a Tauri IPC command backed by `core::llm::plain_language_rewrite` and a real Ollama model.
2. The component tests (`Workbench.test.tsx`) mock `plainLanguageRewrite` via `vi.mock("../ipc", ...)`, so they exercise the React state machine (modal open, `computeLineDiff`, Accept→`onUpdateDraftContent`, Reject→discard) but not the real IPC round-trip or a real model's output.
3. A plain Vite/browser session cannot reach the Tauri backend, so the true end-to-end rewrite→diff→accept flow was not driven live in this audit.

**Why this matters**
This is a stated scope limitation, not a defect. The behavioral contract (no in-place overwrite; explicit Accept/Reject) is verified at the component level and the destructive `window.confirm` overwrite is confirmed removed. The residual unknowns are: how the LCS `computeLineDiff` renders against a *real* model's multi-paragraph output (very long lines, whitespace, unicode), and whether the IPC error path surfaces correctly in the modal — neither reachable without the desktop runtime.

**Fix path**
None required for this hotpatch. To close the gap in a future cycle, add a Tauri integration test (or a `tauri dev` smoke walkthrough with a stubbed LLM client) that drives rewrite→modal→Accept and rewrite→modal→Reject against the real IPC layer.

---

## Performance snapshot

| Metric | Observed | Benchmark | Verdict |
|---|---|---|---|
| Rust lib test wall time | 0.43s (44 tests) | — | Fast; no flakiness across run |
| vitest suite wall time | ~4.55s (32 tests / 13 files) | — | Normal |
| Landing-page download resolve latency | < 300ms to first href rewrite (real GitHub API) | — | Good; single API call, no waterfall |
| Landing-page console errors | 0 | 0 | Pass |

(Core Web Vitals, API P50, bundle size omitted — not relevant to the in-scope changes; no full app build was produced.)

## Security / privacy snapshot

- **No new network surface introduced.** The resolver hits exactly one origin, `api.github.com`, which the CSP `connect-src` does not list — but `docs/` is a static GitHub-Pages site served outside the Tauri CSP, so the page-level CSP in `tauri.conf.json` does not apply to it. The `fetch` is a public unauthenticated GitHub API call; no credentials, no user data leave the page.
- **`setButtonHref` only assigns `browser_download_url` values that came from the GitHub API response** — it does not interpolate user input, so there's no href-injection vector from the resolver itself.
- The hotpatch's Rust changes are a pure refactor (decoupling business logic from `AppHandle`); no new auth, IPC, or file-system surface. The `clippy -D warnings` clean run gives reasonable assurance no `unsafe`/lint-worthy patterns were introduced.

## Console and log observations

- Landing page: console clean on the live happy path (no errors, no warnings) per `preview_console_logs`. The only `console.error` in the resolver is inside the catch block; it fired (with the expected message) only when fetch was deliberately forced to reject, never on the real run.
- vitest: clean for the in-scope Workbench tests; an out-of-scope `act()` warning from `OnboardingWizard.test.tsx` (QA-004).
- Rust: no warnings from clippy; fmt clean.

## Patterns and systemic observations

- **The landing-page download resolver has zero automated test coverage.** Every branch I exercised (win/linux/mac picks, sig-exclusion, msi-fallback, appimage-preference, empty-assets, unknown-arch, API failure) was verified manually in-browser this session, but nothing in `vitest` or any other suite guards `pickWindowsAsset`/`pickLinuxAsset`/`pickMacAsset`/`detectMacArch`. The functions are pure and trivially unit-testable if extracted from the DOMContentLoaded closure. This is the single highest-leverage gap surfaced by QA — it's why QA-001/002/003 are observations rather than caught-by-CI regressions. Recommend the Test Engineer flag a coverage gap here; the resolver is exactly the kind of "silently falls back to the release page for real users" logic that benefits from a regression net. (Cross-link to any TEST-* finding on landing-page coverage.)
- **The cross-platform refactor is well-disciplined.** The same root change (inject `Arc<dyn LlmClient>` + prompt/sink rather than reaching through `AppHandle`) cleanly resolved P5-003 and P5-004, emptied the platform-gate whitelist, and kept the gate-detector test (`reproduce_m1_cfg_family_bypass`) green. The thin Tauri command wrappers (`AppHandlePullSink`, the `run_daily_scan`/`plain_language_rewrite` wrappers in `tauri_cmds.rs`) preserve the production wire format while the core logic became testable. This is the right shape and it runs clean on Windows.

## Appendix: what was audited

| Artifact | How exercised |
|---|---|
| `src-tauri` Rust lib suite | `cargo test --lib` (run) → 44 passed / 0 ignored |
| Rust lints | `cargo clippy --all-targets -- -D warnings` (run) → exit 0 |
| Rust formatting | `cargo fmt --check` (run) → exit 0 |
| Frontend types | `npx tsc --noEmit` (run) → exit 0 |
| Frontend tests | `npx vitest run` (run) → 32 passed / 13 files |
| `docs/index.html` | read; confirmed card IDs + `.download-btn` + `releases/latest` fallbacks |
| `docs/script.js` | read; resolver driven LIVE on `localhost:4406` against real GitHub release; all branches re-verified via `preview_eval`; failure path forced via fetch override |
| `scripts/verify-release.sh` | read; arch/size/extension expectations cross-checked vs real asset names |
| `src-tauri/tauri.conf.json` / `tauri.linux.conf.json` | read; bundle targets cross-checked vs resolver suffix matches |
| `src/components/Workbench.tsx` / `Workbench.test.tsx` | read + tests run; diff modal assessed at component level (not e2e — see QA-005) |
| Working tree | `git status --short` before and after — unchanged; no scratch edits left; preview server stopped |

**Tools used:** cargo (test/clippy/fmt), npx (tsc/vitest), Claude Preview MCP (`preview_start`/`preview_eval`/`preview_console_logs`/`preview_screenshot`/`preview_stop`) on a Chromium engine.
