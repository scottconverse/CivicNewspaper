# Test Suite Deep-Dive — CivicNewspaper (v0.2.6)

**Audit date:** 2026-05-28
**Role:** Test Engineer
**Scope audited:** Uncommitted working-tree changes on branch `v0.2.5-hotpatch`. Rust unit suite (`src-tauri/src/core/tests.rs`, `reproduction_tests.rs`, `server_tests.rs`, `llm.rs`, `daily_scan.rs`, `tauri_cmds.rs`) and the frontend Vitest suite (`src/components/Workbench.test.tsx` and siblings). Platform-gate whitelist `.agent-workflows/section2-auth.json`.
**Auditor posture:** Balanced (adversarial on test-gaming, per engagement brief)

---

## TL;DR

The central claim holds: the 7 previously Windows-`ignore`d / `#[cfg(unix)]`-bodied tests now run cross-platform with **no cfg gates, no `mock_app()`, and preserved assertions**, achieved by genuine decoupling (injected `Arc<dyn LlmClient>`, a `PullProgressSink` trait, and a `base_url`/`probe_addr` parameter). I ran both suites on this Windows host: **Rust = 44 passed, 0 failed, 0 ignored, 0 filtered out**; **Vitest = 32 passed across 13 files**. I individually confirmed each of the 9 named tests reports `running 1 test … 1 passed; 0 ignored` on Windows — they are genuinely compiled in and executing, not stubbed. The rewritten assertions are real (the `FakeLlmClient` panics if the production code passes the wrong model/prompt/system; the per-pull-cancel test proves isolation against real shared state; the 404 test exercises a real axum stub). The empty whitelist (`[]`) is **honest for the test code as it stands today** — there are zero platform gates in `tests.rs`/`server_tests.rs`.

**The one thing the dev team most needs to know:** the guard that is supposed to *keep* the whitelist honest — `verify_no_unauthorized_platform_gates` — has a still-live regex bypass (weakness "a" from the prior audit). A nested-paren cfg gate such as `#[cfg(all(not(test), target_os = "linux"))]` compiles a test OUT on Windows yet **slips past the detector unflagged** (TEST-001, empirically reproduced below). The second prior weakness ("b", checker pointed at an empty `server_tests.rs`) is **resolved** — that file now has 7 real `#[tokio::test]` functions which the scanner reads. A secondary coverage gap: three coexisting Tauri pull commands (`pull_model`, `pull_ollama_model`, `ollama_pull_model`) with different progress-event shapes, only one of which is on the newly-tested path (TEST-002).

## Severity roll-up (tests)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 0 |
| Major | 2 |
| Minor | 3 |
| Nit | 2 |

## What's working

- **The decoupling is real, not cosmetic.** `git diff HEAD -- src-tauri/src/core/tests.rs` shows the BEFORE state was exactly the documented anti-pattern: every one of the 7 tests carried `#[cfg_attr(target_os = "windows", ignore = "…")]` AND wrapped its entire body in `#[cfg(unix)] { … }` — so on Windows the function compiled to an empty body that passed trivially. The AFTER state removes both, routing through `core::llm::run_ollama_pull`, `core::llm::plain_language_rewrite`, `run_daily_scan(&Arc<dyn LlmClient>, …)`, and `OllamaSidecar::start_for_test(probe_addr)`. This is the correct fix, not a gate-flip.
- **Assertions were preserved, not hollowed.** `tests.rs:807-821` — the `FakeLlmClient` in `test_plain_language_rewrite_invokes_ollama` still asserts `model == "fake-model"`, `prompt.contains("Rewrite this")`, `system.contains("summarizer")` *inside* `call()`, so if the production rewrite path passed the wrong arguments the fake panics and the test fails. Same pattern in `test_daily_scan_uses_settings_model_not_hardcoded` (`tests.rs:1033`, asserts `model == "my-custom-model"`). These are behavioral assertions on the real code path, not assertions on a canned return.
- **`test_cancel_ollama_pull_is_per_pull` genuinely proves isolation** (`tests.rs:1091-1153`). It registers two real pulls in the real shared `CANCEL_PULL_MAP`, cancels `model-1`, sleeps 200ms, then asserts `!map.contains_key("model-1") && map.contains_key("model-2")`. If cancellation were global or keyed wrong, this fails. Not trivially passing.
- **`test_pull_ollama_model_propagates_http_error` hits the real 404 path** (`tests.rs:1064-1085`) against a live `axum` stub bound to an ephemeral port, asserting `res.unwrap_err().contains("status 404")` — which corresponds to `llm.rs:193-200`. Real wire, real error string.
- **Ephemeral-port discipline.** Every networked test binds `127.0.0.1:0` (OS-assigned) and the sidecar tests inject the probe address, so the suite never collides on fixed port 11434 nor kills a developer's real `ollama serve` (`llm.rs:399-416`, comment + `start_for_test`). This is thoughtful test hygiene.
- **Clean shortcut census on the frontend.** Grep of `src/` for `.skip`/`.only`/`xit`/`it.todo`/`TODO: add test`/`assert!(true)`/`expect(true)` returned **zero matches**. No `#[ignore]` remains in `tests.rs`/`server_tests.rs` (the lone "ignore" token at `tests.rs:1159` is a comment narrating the prior state).
- **carried-debt.md is truthful.** P5-001 (diff modal), P5-003, P5-004 are all marked RESOLVED with descriptions that match the code I read. The "two pull tests" are correctly folded into P5-003.
- **Workbench diff-modal tests are real, not shallow.** `Workbench.test.tsx:155-205` renders the component, mocks `plainLanguageRewrite`, clicks the real button, asserts the modal appears, asserts the mock was called with the right args (`"Content with citations", "watch"`), reads both diff panes by DOM id, and verifies accept→`onUpdateDraftContent("Plain simple text")` / reject→not-called / open→not-yet-applied. These exercise the open/accept/reject state machine, not just a render snapshot.

## What couldn't be assessed

- **CI history / flakiness over time.** `ci_log.txt` is a single local capture; I have no run history, so I cannot speak to intermittent flakiness. The two timing-dependent tests (`test_cancel_ollama_pull_is_per_pull`'s 200ms sleep; `test_ollama_sidecar_terminates_cleanly_on_drop`'s post-drop `sysinfo` poll) are theoretical flake candidates but passed cleanly in my runs.
- **Coverage percentage.** No coverage tooling (tarpaulin/llvm-cov/c8) is configured; I assessed *meaningful* coverage by reading paths, not a line number.
- **The production `AppHandlePullSink` emit path** (`tauri_cmds.rs:490-507`) is untestable without a Tauri runtime and is exercised by no test — see TEST-004.

---

## Test landscape

| Dimension | Observation |
|---|---|
| Framework(s) | Rust `cargo test --lib` (+ `tokio::test`, `axum` stubs, `sysinfo`); Vitest + Testing Library (jsdom) for React |
| Test pyramid shape | Heavy unit / thin integration (in-process axum & spawned fixture binary stand in for "integration") / no true E2E or Tauri-runtime tests |
| Coverage tool | None configured |
| Reported coverage (if any) | None. `mutation-checks-results.json` exists but is a project-specific fitness artifact, not a mutation-score report |
| Flakiness posture | Clean in my runs; no `--retry`/`retries` config anywhere. Two timing-sensitive tests noted (TEST-005) |
| CI blocking? | Local `ci_log.txt` present; CI gating not verifiable from working tree |

**Run evidence (this Windows host):**

```
# cargo test --lib
running 44 tests
test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

# each named test, module-qualified --exact (representative):
core::tests::tests::test_pull_ollama_model_propagates_http_error        running 1 test … 1 passed; 0 ignored
core::tests::tests::test_cancel_ollama_pull_is_per_pull                  running 1 test … 1 passed; 0 ignored
core::tests::tests::test_ollama_sidecar_spawns_with_expected_pid_pattern running 1 test … 1 passed; 0 ignored
core::tests::tests::test_ollama_sidecar_terminates_cleanly_on_drop       running 1 test … 1 passed; 0 ignored
core::tests::tests::test_port_in_use_detects_listener_cross_platform     running 1 test … 1 passed; 0 ignored
core::tests::tests::test_sidecar_skips_spawn_when_port_occupied          running 1 test … 1 passed; 0 ignored
core::tests::tests::test_plain_language_rewrite_invokes_ollama           running 1 test … 1 passed; 0 ignored
core::tests::tests::test_daily_scan_command_does_not_panic_when_state_registered  running 1 test … 1 passed; 0 ignored
core::tests::tests::test_daily_scan_uses_settings_model_not_hardcoded    running 1 test … 1 passed; 0 ignored

# npx vitest run
Test Files  13 passed (13)
     Tests  32 passed (32)
```

---

## Findings

> **Finding ID prefix:** `TEST-`
> **Categories:** Coverage / Shortcut / Flakiness / Quality / Mocking / Regression / CI

### [TEST-001] — Major — Quality (Detector Effectiveness) — Platform-gate detector still bypassable by a nested-paren cfg gate; the empty whitelist's safety rests on this guard

**Evidence**

The guard is `verify_no_unauthorized_platform_gates` in `src-tauri/src/core/reproduction_tests.rs:19-79`, driven by `reproduce_m1_cfg_family_bypass` (line 84) and `reproduce_structural_closure_0_22_violations` (line 215). Its gate-matching regex is `reproduction_tests.rs:31`:

```
(?i)cfg(_attr)?\s*\(\s*(?:[^)]*unix|[^)]*linux|[^)]*macos|[^)]*not\s*\(\s*windows|[^)]*not\s*\(\s*target_os\s*=\s*"windows"|[^)]*not\s*\(\s*target_family\s*=\s*"windows"|target_os\s*=\s*"windows"\s*,\s*ignore)
```

Every platform alternative uses `[^)]*` — a negated class that **stops at the first `)`**. So a platform token that sits *after* an inner closing paren is unreachable. I reproduced this against the exact regex (Rust-regex-equivalent syntax — no lookarounds/backrefs — verified in Python `re` with `(?i)`):

```
MATCH  #[cfg(unix)]
MATCH  #[cfg(target_os = "macos")]
MATCH  #[cfg_attr(target_os = "windows", ignore = "x")]
MATCH  #[cfg(not(windows))]
MATCH  #[cfg(not(target_os = "windows"))]
MATCH  #[cfg(all(unix, feature = "x"))]                          (token before any ')')
MISS   #[cfg(all(not(debug_assertions), target_os = "macos"))]   <-- the prior-audit case, STILL slips
MISS   #[cfg(all(not(test), target_os = "linux"))]               <-- excludes Windows, EVADES
MISS   #[cfg_attr(all(not(test), target_os = "windows"), ignore = "x")]  <-- ignores on Windows, EVADES
MISS   #[cfg(all(not(debug_assertions), unix))]                  <-- EVADES
```

The documented weakness "a" therefore **still exists in the current code** and is exploitable: any `#[cfg(all(not(<anything>), <platform>))]` or `#[cfg_attr(all(not(<anything>), target_os="windows"), ignore=…)]` compiles a test out of (or marks it ignored on) Windows while the detector reports no violation. The empty whitelist would still pass green.

(For balance: weakness "b" is resolved. `server_tests.rs` now contains 7 `#[tokio::test]` functions — `test_auth_middleware_missing_origin` etc. at lines 60-253 — and the `re_test` regex `(?:async\s+)?fn\s+(test_[a-z0-9_]+)` matches `async fn test_…`, so the scan of that file is no longer vacuous. I verified the capture against `async fn`, `pub async fn`, and `pub fn` forms.)

**Why this matters**

The entire justification for emptying `section2-auth.json` to `[]` is "the detector will catch any future regression." That guarantee is only as strong as the detector. A future contributor (or an automated agent under pressure to make Windows green) can re-introduce exactly the class of bug the prior 5 pipeline-integrity incidents were about — a test compiled out on Windows — using a one-token-deeper cfg expression, and CI stays green. This is a guard with a known hole, which is more dangerous than no guard because it manufactures false confidence.

**Blast radius**
- Adjacent code: the same regex is invoked from two test entry points (`reproduce_m1_cfg_family_bypass`, `reproduce_structural_closure_0_22_violations`) over two files (`tests.rs`, `server_tests.rs`). A fix to the regex/parser hardens both.
- Shared state / config: `.agent-workflows/section2-auth.json` (the whitelist whose emptiness this guard underwrites).
- User-facing: none directly; this is a CI/integrity control.
- Migration: none.
- Tests to update: add a self-test of the detector (feed it a known nested-cfg gate string and assert it *fires*). None exists today — the detector has no test of its own.
- Related findings: engineering/process findings about the pipeline-integrity controls (cross-role); the honesty of the empty whitelist (this audit's central claim) is contingent on this.

**Fix path**
Stop using `[^)]*` line-noise matching of a balanced-paren grammar. Recommended: replace the regex with a small balanced-paren extractor — capture the full `cfg(...)`/`cfg_attr(...)` token by counting parens, then test the *extracted* predicate string for `unix|linux|macos|windows|target_os|target_family` anywhere inside. As an interim hardening, add the missing nested cases to the alternation and, critically, add a unit test for the detector itself that asserts `#[cfg(all(not(test), target_os = "linux"))]` is flagged. Until then, treat the empty whitelist as provisional and keep a human reviewer on cfg attributes in test files.

---

### [TEST-002] — Major — Coverage / Regression — Three coexisting Tauri pull commands with divergent progress-event shapes; only the new path is covered

**Evidence**

`src-tauri/src/lib.rs:114-120` registers three pull commands in the invoke handler: `pull_model`, `pull_ollama_model`, `ollama_pull_model`.

- `pull_model` (`tauri_cmds.rs:458-479`) — OLD path. Calls `llm::pull_ollama_model(&model)` (`llm.rs:100-126`, fixed URL `127.0.0.1:11434`), streams raw chunks and emits **raw string lines** via `app.emit("ollama-pull-progress", line)`. Frontend caller: `src/useApp.ts:745` (`pullModel(wizardModel)`). **Untested.**
- `pull_ollama_model` (`tauri_cmds.rs:509-516`) — NEW path. Builds `AppHandlePullSink` and calls `core::llm::run_ollama_pull(model_id, base_url, sink)`, emitting **structured `PullProgress {model,status,completed,total}`**. Frontend caller: `src/components/OnboardingWizard.tsx:221`. The *core* function is the one the new tests exercise.
- `ollama_pull_model` (`tauri_cmds.rs:610+`) — another OLD inline path with its own `app.emit`. No frontend caller found in `src/`. **Untested, likely dead but still registered.**

So two distinct live wire paths can pull a model (`useApp.ts` → `pull_model` raw-line events; `OnboardingWizard.tsx` → `pull_ollama_model` structured events), the frontend listeners must handle two payload shapes, and the test suite covers only the core function behind the second.

**Why this matters**
A regression in the `pull_model` raw-line path — the one wired into `useApp.ts` — would not be caught by any test in this change set, despite the brief's framing that pull behavior is now well-covered. Two payload shapes for the same logical event is a latent frontend bug class (a listener written against `PullProgress` objects receives bare strings, or vice versa). The dead `ollama_pull_model` registration is attack surface and confusion with no test pinning its absence-of-use.

**Blast radius**
- Adjacent code: `src/useApp.ts:745`, `src/components/OnboardingWizard.tsx:221`, the three command bodies in `tauri_cmds.rs`, and `llm::pull_ollama_model` vs `core::llm::run_ollama_pull`.
- Shared state: the `ollama-pull-progress`/`-complete`/`-error` event channel and `CANCEL_PULL_MAP` (only `run_ollama_pull` registers cancellation; `pull_model` has no cancellation hook).
- User-facing: model-download progress UI; a payload-shape mismatch shows a broken/empty progress bar.
- Migration: none, but consolidating to one command is the clean fix.
- Tests to update: if `pull_model`/`ollama_pull_model` are removed, drop their handler registration; if kept, add a test of the raw-line path. `OnboardingWizard.test.tsx:144` already asserts `pull_ollama_model` is the wizard's call — extend coverage to the `useApp` path.
- Related findings: TEST-004 (untested sink); engineering finding on duplicate command surface (cross-role).

**Fix path**
Decide on one pull command. Recommended: route `useApp.ts` through `pull_ollama_model` (the structured, cancellable, tested path), delete `pull_model` and `ollama_pull_model`, and remove their `invoke_handler` lines. Then add one frontend test asserting the surviving path's event-payload shape so the two-shape ambiguity cannot return.

---

### [TEST-003] — Minor — Coverage — `computeLineDiff` LCS has no direct unit test; only exercised indirectly via two happy-path modal renders

**Evidence**
`src/components/Workbench.tsx:11-57` implements a non-trivial LCS line-diff (O(n·m) DP table, three trailing drain loops). It is exercised only through the modal in `Workbench.test.tsx`, and only with simple inputs: a 1-line original vs a 2-line rewrite (`"Content with citations"` vs `"Plain simple text\nsecond line"`, test at line 155) and equal-length single lines (line 173/190). No test covers: identical input (all "same"), empty original or empty rewrite, trailing/leading blank lines (the `row.text || " "` fallback at `Workbench.tsx:426/433`), or a reordering that stresses the `lcs[i+1][j] >= lcs[i][j+1]` tie-break branch (line 40).

**Why this matters**
A diff algorithm with an off-by-one in a drain loop or a wrong tie-break can silently mislabel which lines an AI rewrite dropped — directly undermining the editorial-oversight purpose of the modal (P5-001). The current tests would not catch it because they never feed it a case where the branches matter.

**Fix path**
Add a focused unit test of `computeLineDiff` (export it or test via a thin wrapper): identical input → all `same`; empty rewrite → all `removed` on left; empty original → all `added` on right; a multi-line reorder asserting the exact `{text,type}` rows. Fast, deterministic, no render needed.

---

### [TEST-004] — Minor — Mocking / Coverage — Production `AppHandlePullSink` emit bridge is exercised by no test

**Evidence**
`tauri_cmds.rs:490-507` — `AppHandlePullSink::progress/complete/error` translate `PullProgress` into `app.emit("ollama-pull-progress"|"-complete"|"-error", …)`. Every test uses `NoopPullSink` (`tests.rs:1052-1057`), which discards. No test asserts the production sink emits the three event names with the expected payloads.

**Why this matters**
This is the seam between tested core logic and the frontend. A typo in an event name or payload shape here is invisible to the suite yet breaks the progress UI. It is genuinely hard to test without a Tauri runtime (untestable-by-design is a fair call), but it should be acknowledged rather than implied-covered.

**Fix path**
Either (a) extract the event-name/payload mapping into a pure function tested in isolation, or (b) add an integration test under `src-tauri/tests/` with a real Tauri test runtime. If neither, record it explicitly in `carried-debt.md` as a known untested seam.

---

### [TEST-005] — Minor — Flakiness — Two timing-dependent assertions could flake on a loaded/slow host

**Evidence**
- `tests.rs:1144` — `test_cancel_ollama_pull_is_per_pull` sleeps a fixed `200ms` after `cancel_pull("model-1")` before asserting the map entry was removed by the spawned cleanup task. On a heavily loaded runner the cleanup may not have run yet.
- `tests.rs:908-916` — `test_ollama_sidecar_terminates_cleanly_on_drop` polls `sysinfo` once immediately after drop and asserts the PID is gone; process teardown is not instantaneous on all OSes.

Both passed cleanly in my runs (suite finished in 0.41s).

**Why this matters**
Fixed-sleep synchronization is the classic source of "passes on my machine, flakes in CI" — and the brief notes this repo institutionalizes shortcuts under pressure; a flaky test tends to get a longer sleep rather than a fix.

**Fix path**
Replace the fixed sleeps with bounded poll-until-condition loops (e.g. retry the map check for up to ~2s, 10ms intervals). For the sidecar, loop the `sysinfo` check until the process is absent or a timeout elapses.

---

### [TEST-006] — Nit — Quality — `test_daily_scan_command_does_not_panic_*` asserts only `is_ok()`, not that a lead was persisted

**Evidence**
`tests.rs:923-953` — the fake returns `{"leads":[]}` and the test asserts only `res.is_ok()`. It proves "does not panic," which matches its name, but does not verify the scan persisted anything. (Its sibling `test_daily_scan_parses_fixture_response` at `tests.rs:749` does assert a row count, so the persistence path is covered elsewhere.)

**Fix path**
Optionally return one lead from the fake and assert `daily_scan_leads` count == 1, to make this a behavioral test rather than a panic-smoke test. Low priority given sibling coverage.

---

### [TEST-007] — Nit — Quality — `OnboardingWizard.test.tsx` emits React `act(...)` warnings

**Evidence**
The Vitest run stderr (captured) shows repeated `An update to OnboardingWizard inside a test was not wrapped in act(...)` warnings from `OnboardingWizard.test.tsx`. The suite still passes (32/32).

**Why this matters**
`act()` warnings indicate state updates settling after the assertion point — a latent source of future flakiness and a sign the test isn't fully awaiting the component's async transitions.

**Fix path**
Wrap the triggering interactions in `await act(...)` / use `findBy*`/`waitFor` consistently. Cosmetic today.

---

## Shortcut census

| Shortcut pattern | Count |
|---|---|
| `.skip` / `xit` / `it.todo` / `#[ignore]` | 0 (in changed scope; lone "ignore" token at `tests.rs:1159` is an explanatory comment) |
| `.only` (left in) | 0 |
| `TODO: add test` / similar | 0 |
| Empty assertion / `assert!(true)` / `expect(true)` placeholder | 0 |
| `--retry` / retries normalized | No |

The shortcut census is genuinely clean — a notable and creditable contrast with this repo's documented incident history.

## Blind spots by class

- **cfg-gate regression on Windows via nested parens** — the detector won't catch it (TEST-001). Highest-leverage gap.
- **Old/duplicate pull command paths** — `pull_model` (raw-line events, wired to `useApp.ts`) and `ollama_pull_model` (dead) are untested (TEST-002).
- **Diff-algorithm edge cases** — empty/identical/blank-line/reorder inputs to `computeLineDiff` (TEST-003).
- **Core↔frontend event-emit seam** — `AppHandlePullSink` event names/payloads (TEST-004).
- **Concurrency/timing** — fixed-sleep synchronization in two tests (TEST-005); no test of two simultaneous in-flight pulls beyond map-key presence.

## Patterns and systemic observations

- **The fix is methodologically sound and the assertions are honest.** Across all 7 rewritten tests the pattern is the same: lift business logic out of the `AppHandle`-bound command into a pure core function taking an injected dependency, then assert behavior on that core function with a fake that *verifies its inputs*. This is the textbook correct way to kill a `#[cfg(unix)]`-bodied test without losing coverage, and it was applied consistently. I found no hollowed assertions, tautologies, or fake-asserting-its-own-return in this change set.
- **The integrity guard is the weak link, not the tests.** The single most important systemic observation: this change set is honest, but the mechanism meant to *keep future change sets honest* (the platform-gate detector) has a real, reproduced bypass (TEST-001). Fixing the detector is higher leverage than anything in the test bodies themselves, because it is what makes the empty whitelist safe to leave empty.
- **Command-surface duplication** (TEST-002) is both an engineering smell and a coverage gap; a single coordinated cleanup closes both.

## Verdicts (per engagement brief)

- **(a) Are the tests genuine / non-gamed?** Yes. All 9 named tests compile in and execute on Windows (verified individually, 0 ignored), and their assertions verify real behavior on the production code path. No gaming detected in this change set.
- **(b) Is the empty whitelist honest?** Yes for the code as it stands — there are zero platform gates in `tests.rs`/`server_tests.rs`, so `[]` is truthful today. But its *durability* is undermined by TEST-001: the guard that protects it is bypassable.
- **(c) Does the platform-gate detector actually enforce the guarantee?** Partially. Weakness "b" (vacuous `server_tests.rs` half) is fixed — that file now has 7 real test functions the scanner reads. Weakness "a" (first-`)` regex stop) is **not** fixed and is exploitable via nested-paren cfg gates. The detector enforces against naive gates but not against a one-token-deeper expression. Classified Major (TEST-001).

## Appendix: what was audited

- `src-tauri/src/core/tests.rs` (full, 1201 lines; 7 rewritten tests + 2 supporting cross-platform tests)
- `src-tauri/src/core/reproduction_tests.rs` (full; detector + diff vs HEAD)
- `src-tauri/src/core/llm.rs` (full; `run_ollama_pull`, `PullProgressSink`, `CANCEL_PULL_MAP`, `OllamaSidecar`, `start_for_test`, `port_in_use`)
- `src-tauri/src/core/daily_scan.rs` (full; `run_daily_scan` signature + model resolution)
- `src-tauri/src/core/server_tests.rs` (test-function inventory: 7 `#[tokio::test]`)
- `src-tauri/src/tauri_cmds.rs` (pull commands, `AppHandlePullSink`, lines 440-615)
- `src-tauri/src/lib.rs` (invoke-handler registration, lines 114-120)
- `src/components/Workbench.tsx` and `Workbench.test.tsx` (full; diff modal + 3 new tests)
- `.agent-workflows/section2-auth.json` (`[]`), `carried-debt.md` (P5-001/003/004 RESOLVED entries)
- `src/ipc.ts`, `src/useApp.ts`, `src/components/OnboardingWizard.tsx`/`.test.tsx` (pull-command call sites)
- Commands run on this Windows host: `cargo test --lib` (44/0/0/0); each of the 9 named tests via `cargo test --lib core::tests::tests::<name> -- --exact` (each `1 passed; 0 ignored`); `npx vitest run` (32/32 across 13 files); empirical regex reproduction of TEST-001 in Python (`re`, Rust-equivalent syntax).
