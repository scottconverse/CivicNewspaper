# Engineering Deep-Dive — CivicNewspaper (civicnews v0.2.6)

**Audit date:** 2026-05-28
**Role:** Principal Engineer
**Scope audited:** Uncommitted working-tree changes on branch `v0.2.5-hotpatch` (11 files): `.agent-workflows/section2-auth.json`, `carried-debt.md`, `docs/script.js`, `src-tauri/src/core/daily_scan.rs`, `src-tauri/src/core/llm.rs`, `src-tauri/src/core/reproduction_tests.rs`, `src-tauri/src/core/tests.rs`, `src-tauri/src/tauri_cmds.rs`, `src/App.css`, `src/components/Workbench.tsx`, `src/components/Workbench.test.tsx`. Surrounding code read for context (lib.rs, useApp.ts, OnboardingWizard.tsx, docs/index.html, Cargo.toml).
**Auditor posture:** Balanced

---

## TL;DR

This is a clean, honest refactor. The core claim — decoupling business logic from Tauri's `AppHandle` so seven previously platform-gated tests run cross-platform — is verified true: I ran all eight formerly-gated tests on this Windows host and they execute and pass with **0 ignored** (`cargo test --lib` → 44 passed, 0 failed, 0 ignored). The tests are genuine, not stubs: they spawn a real fixture process, hit a real local axum stub server, and mutate the real global cancel map. The production wire contract (`ollama-pull-progress`/`-complete`/`-error` event names and the structured progress payload) is preserved exactly, and the model-from-settings lookup survived in `daily_scan.rs`. Emptying `section2-auth.json` to `[]` is honest: no platform `#[cfg(...)]`/`cfg_attr(... ignore)` test gates remain anywhere under `src-tauri/`. The most notable engineering issue is a **carried-forward** per-pull cancellation defect: keying the cancel map by model id (not by invocation) means two concurrent pulls of the *same* model leak a sender and silently lose cancellation — and the new doc comment over-claims a "per-pull isolation guarantee" that only holds across *different* models. No Blockers. Architectural debt is low; the seam (`Arc<dyn LlmClient>` + `PullProgressSink` trait + extracted `port_in_use`) is the right shape.

## Severity roll-up (engineering)

| Severity | Count |
|---|---|
| Blocker | 0 |
| Critical | 0 |
| Major | 2 |
| Minor | 3 |
| Nit | 3 |

## What's working

- **The decoupling refactor is correct and the integrity claim is verifiable.** I ran the eight named tests on Windows: `test_plain_language_rewrite_invokes_ollama`, `test_daily_scan_command_does_not_panic_when_state_registered`, `test_daily_scan_uses_settings_model_not_hardcoded`, `test_ollama_sidecar_spawns_with_expected_pid_pattern`, `test_ollama_sidecar_terminates_cleanly_on_drop`, `test_pull_ollama_model_propagates_http_error`, `test_cancel_ollama_pull_is_per_pull`, `test_sidecar_skips_spawn_when_port_occupied` — all pass, **0 ignored**. These exercise real code paths (a spawned fixture process, a live axum stub on an ephemeral port, the real `CANCEL_PULL_MAP`), not grep-bait or tautologies.
- **The seam is well-chosen.** `run_daily_scan`/`plain_language_rewrite` taking an injected `Arc<dyn LlmClient>` + prompt string (`daily_scan.rs:43-50`, `llm.rs:89-98`), the `PullProgressSink` trait with an `AppHandlePullSink` adapter (`tauri_cmds.rs:484-505`), and `port_in_use(addr)` extracted from `ollama_port_in_use()` (`llm.rs:294-306`) are textbook dependency-inversion. Production wrappers are thin and delegate; tests inject fakes. This is the right pattern, not the convenient one.
- **Wire contract preserved exactly.** The new `AppHandlePullSink` emits the same event names and the same structured `{model,status,completed,total}` payload the previous inline implementation emitted (`tauri_cmds.rs:489-504`), matching the `OnboardingWizard.tsx:193` consumer that actually invokes `pull_ollama_model`. No frontend change was required.
- **The empty whitelist is honest.** A repo-wide scan found no platform-gating test attributes (`#[cfg(unix)]`, `#[cfg_attr(target_os="windows", ignore)]`, etc.) anywhere under `src-tauri/`. The only `cfg!(target_os=...)` use is a runtime binary-selection expression in `spawn_test_fixture` that compiles and runs on all platforms (`llm.rs:369-379`). The two reproduction guards (`reproduce_m1_cfg_family_bypass`, `reproduce_structural_closure_0_22_violations`) pass on Windows.
- **The diff-modal change is a genuine safety improvement.** `Workbench.tsx` replaced a destructive in-place overwrite behind a `window.confirm("...cannot be undone")` with a reviewable side-by-side LCS diff and explicit Accept/Reject (`Workbench.tsx:303-309`, `412-456`). Rendering is via React text interpolation (`{row.text || " "}`), so there is no XSS surface. Three new component tests assert the real DOM behavior and mock only the IPC boundary (`Workbench.test.tsx:155-206`).
- **Honest debt accounting.** `carried-debt.md` marks P5-001/003/004/005/007 RESOLVED with accurate descriptions of what was actually done, and the test count was corrected (P5-003 went from "four" to the correct "three" Tauri-state tests + two pull tests).

## What couldn't be assessed

- **Runtime behavior of the production `ollama pull` path** against a real Ollama server — only the test stub server path was exercisable here. The wire-format inspection (event names + payload shape) was done by code reading, which is sufficient to confirm contract parity.
- **`docs/script.js` in a browser** — the per-platform download-link resolution was reviewed statically against `docs/index.html` (card IDs and fallback hrefs confirmed present). Live behavior across browsers/UA-CH was not executed.
- **`cargo audit` / `npm audit`** — not run in this pass; dependency CVE status is assessed by version inspection only (see Dependency snapshot).

---

## Findings

### [ENG-001] — Major — Correctness — Concurrent same-model pull leaks a cancel sender and silently loses cancellation

**Evidence**
`src-tauri/src/core/llm.rs:171-175` registers the cancel sender keyed by model id:
```rust
let (tx, mut rx) = watch::channel(false);
{
    let mut map = CANCEL_PULL_MAP.lock().unwrap();
    map.insert(model_id.clone(), tx);   // overwrites any existing entry for this model
}
```
and the spawned task's cleanup (`llm.rs:253-254`) removes by the same key:
```rust
let mut map = CANCEL_PULL_MAP.lock().unwrap();
map.remove(&model);
```
If `run_ollama_pull` is invoked twice for the **same** `model_id` while the first is still in flight (user double-clicks "Download", retries a stalled pull, or two UI surfaces trigger it), the second `insert` overwrites the first task's `tx`. Consequences:
1. The first task's `rx` can never be signaled — its sender was dropped/replaced — so `cancel_pull(model)` is a no-op for it; it only exits on stream end/error.
2. When the first task finishes, its `map.remove(&model)` deletes the **second** pull's entry, so cancelling the second pull also becomes a no-op.

The new doc comment claims more than the code delivers (`llm.rs:128-133`): *"Keyed by model id so cancelling one pull never disturbs another (the per-pull isolation guarantee)."* The guarantee holds across *different* models but is false for concurrent *same-model* pulls.

**Why this matters**
A user who clicks Download twice (common when a pull appears stalled) ends up with a cancel button that does nothing and a possibly-orphaned background HTTP stream. No data loss, but a confusing, un-cancellable state on a flow that is already failure-prone (large model downloads). This is **carried-forward** behavior — the pre-refactor `tauri_cmds.rs` used the identical insert/remove-by-model pattern — so the refactor did not introduce it, but it did re-document it with an over-strong guarantee.

**Blast radius**
- Adjacent code: `cancel_pull` (`llm.rs:135-141`), `cancel_ollama_pull` command (`tauri_cmds.rs:481-485`), `pull_ollama_model` command (`tauri_cmds.rs:507-513`). The two *other* registered pull commands — `pull_model` (`tauri_cmds.rs:457-479`) and `ollama_pull_model` (`tauri_cmds.rs:609+`) — do not register in the cancel map at all, so they are uncancellable by a different mechanism (see ENG-004).
- Shared state: the global `CANCEL_PULL_MAP` static (`llm.rs:132`).
- User-facing: the onboarding/model-download cancel button; no change for the single-pull happy path.
- Migration: none.
- Tests to update: `test_cancel_ollama_pull_is_per_pull` (`tests.rs`) covers two *different* models and would not catch this; add a same-model concurrent-pull case. If the map is re-keyed (below), that test's `map.contains_key("model-1")` assertions must change.
- Related findings: ENG-004 (command sprawl), ENG-002 (doc over-claim).

**Fix path**
Key the map by a unique pull/invocation id rather than by model. Return the id from `run_ollama_pull` (or generate one and pass it to the sink); have `cancel_ollama_pull` take that id, or guard `insert` so a second same-model pull is rejected/coalesced with `if map.contains_key(&model_id) { return Err("pull already in progress") }`. Minimum viable fix: soften the doc comment to state the guarantee is per-*model*, and reject a duplicate in-flight same-model pull at the `insert` site.

---

### [ENG-002] — Major — Correctness — `verify_no_unauthorized_platform_gates` only inspects two files; the empty whitelist guarantees less than it appears to

**Evidence**
`src-tauri/src/core/reproduction_tests.rs:84-88` and `216-218` invoke the gate checker against exactly two paths:
```rust
verify_no_unauthorized_platform_gates("src-tauri/src/core/tests.rs");
verify_no_unauthorized_platform_gates("src-tauri/src/core/server_tests.rs");
```
The checker reads `.agent-workflows/section2-auth.json` as the whitelist (`reproduction_tests.rs:20-27`). With that file now `[]`, the guard asserts "no platform-gated test fn in `tests.rs` or `server_tests.rs`." It does **not** scan `llm.rs`'s `#[cfg(test)]` modules, `reproduction_tests.rs` itself, `daily_scan.rs`, or any future test module. A platform gate reintroduced in `llm.rs` test code would pass the guard while the empty whitelist still *reads* as "zero gates anywhere."

I independently scanned all of `src-tauri/` and confirmed there are currently **no** such gates outside the two inspected files, so the empty whitelist is honest *today*. The finding is that the guard's coverage is narrower than the artifact's apparent claim, so the honesty is not self-enforcing going forward.

**Why this matters**
This repo has a documented history of cfg-gate bypasses that compiled tests out on Windows. The whole point of the guard is to make that class of regression impossible to reintroduce silently. A guard that only watches two of the project's test files leaves the back door open in every other module — exactly the failure mode the guard exists to prevent.

**Blast radius**
- Adjacent code: every `#[cfg(test)]` module under `src-tauri/src/` (notably `llm.rs:263+`, `reproduction_tests.rs`, and any `server_tests.rs` siblings).
- Shared state: `.agent-workflows/section2-auth.json` (the whitelist source of truth, also read by the policy pipeline).
- User-facing: none directly; this is a CI/quality-gate integrity concern.
- Migration: none.
- Tests to update: extend `reproduce_m1_cfg_family_bypass` / `reproduce_structural_closure_0_22_violations` to enumerate all test-bearing files (glob `src-tauri/src/**/*.rs` containing `#[cfg(test)]`) rather than a hardcoded two-file list.
- Related findings: none in scope.

**Fix path**
Make the guard discover its inputs: walk `src-tauri/src/` for files containing `fn test_`/`#[cfg(test)]` and run `verify_no_unauthorized_platform_gates` against each, instead of two literal paths. That way the empty whitelist means what it says — no gates anywhere — and a gate dropped into any module trips the guard.

---

### [ENG-003] — Minor — Hygiene — `start_for_test` silently diverges from production `start()` (orphan sweep untested)

**Evidence**
`src-tauri/src/core/llm.rs:387-411` (`start_for_test`) deliberately omits the orphan-`ollama serve` process sweep that production `start()` performs (`llm.rs:317-325`), documented as intentional ("Deliberately omits the orphan-process sweep ... so running the suite never kills a developer's real local `ollama serve`"). Consequently the sidecar tests verify the skip/spawn/lock/drop control flow but exercise a code path that is *not* the production one — the orphan sweep has no test coverage on any platform.

**Why this matters**
The omission is the right call (you don't want the test suite killing a dev's Ollama), but it means a regression in the orphan-sweep logic — the part most likely to behave differently across OSes — would not be caught by these now-cross-platform tests. The "runs on every platform" comments could be read as implying full parity with `start()`.

**Blast radius** (Minor; included because it touches the central refactor)
- Adjacent code: `start()` (`llm.rs:308-357`), `spawn_test_fixture` (`llm.rs:363-385`).
- Tests to update: optionally add a targeted test for the sweep predicate (name/cmd matching at `llm.rs:320-324`) using injected process data rather than a live sysinfo scan.

**Fix path**
Extract the sweep predicate (`(name.contains("ollama") || cmd.contains("ollama")) && cmd.contains("serve")`) into a pure function and unit-test it with synthetic inputs. Leave `start_for_test` sweep-free.

---

### [ENG-004] — Minor — Architecture — Three registered Tauri commands pull models; payload shape diverges on the shared `ollama-pull-progress` event

**Evidence**
`src-tauri/src/lib.rs:114-120` registers `pull_model`, `pull_ollama_model`, and `ollama_pull_model`. Each emits `ollama-pull-progress` differently:
- `pull_model` (`tauri_cmds.rs:467`) emits the **raw line string**: `app.emit("ollama-pull-progress", line)`.
- `pull_ollama_model` → `AppHandlePullSink` (`tauri_cmds.rs:489-491`) emits a **structured object** `{model,status,completed,total}`.
- `ollama_pull_model` (`tauri_cmds.rs:632-644`) builds yet another `ProgressPayload`.

The frontend has two listeners that disagree: `useApp.ts:190` declares `listen<string>` and `JSON.parse(event.payload)` (expects a string), while `OnboardingWizard.tsx:193` declares `listen<{model;status;...}>` (expects the object). Tauri delivers the structured payload already deserialized, so `useApp.ts`'s `JSON.parse(object)` throws and falls to its `catch` that stringifies the object.

This is **pre-existing** (the refactor only touched `pull_ollama_model` and preserved its object payload), so it is *not introduced* by this change set — flagged for awareness because the change set's accuracy depends on which command/consumer pairing is live.

**Why this matters**
Command sprawl invites the wrong one being wired up and an event-payload contract that two consumers interpret incompatibly. The live path (`OnboardingWizard` → `pull_ollama_model` → object) is internally consistent; the `useApp.ts` listener appears to be dead or vestigial relative to the older `pull_model` string path.

**Blast radius** (Minor)
- Adjacent code: `lib.rs:107-120` command registry; `ipc.ts:192,206`; `useApp.ts:190-213`; `OnboardingWizard.tsx:193-219`.
- Migration: none if consolidated carefully; confirm no UI still calls `pull_model`/`ollama_pull_model` before removing them (`ipc.ts:192` still references `pull_model`).

**Fix path**
Out of scope for this hotpatch. Recommend a follow-up to collapse to one pull command + one payload shape and delete the unused listeners.

---

### [ENG-005] — Minor — Security — `docs/script.js` assigns `browser_download_url` to `href` without scheme validation

**Evidence**
`docs/script.js` `setButtonHref` assigns `btn.href = asset.browser_download_url` (the asset is selected by matching `asset.name` against `.exe`/`.msi`/`.dmg`/`.appimage`/`.deb`). The `name` filter and the `href` value are independent fields of the GitHub API response; no check confirms the URL begins with `https://`.

**Why this matters**
The data source is a trusted first-party HTTPS endpoint (`api.github.com/repos/scottconverse/CivicNewspaper/releases/latest`), and GitHub's `browser_download_url` is always an `https://github.com/.../releases/download/...` URL. For this to become a `javascript:`/`data:` href sink, an attacker would already need control of the repo's release assets — at which point they could ship a malicious installer regardless. So the practical risk is low/theoretical, but a one-line scheme check is cheap defense-in-depth on a page that auto-resolves download links.

**Why low and not higher**: failure handling is safe — `if (!response.ok) throw`, `catch` logs and *leaves the HTML's `releases/latest` fallback hrefs in place* (`docs/script.js`, confirmed against `docs/index.html:44-67`). No data leakage; the only outbound request is the public releases API.

**Blast radius** (Minor)
- Adjacent code: `setButtonHref`, the three `pick*Asset` helpers in `docs/script.js`.
- User-facing: download buttons on the marketing site.
- Migration: none.

**Fix path**
In `setButtonHref`, guard with `if (!/^https:\/\//i.test(asset.browser_download_url)) return;` before assigning, so a non-HTTPS URL falls back to the static href.

---

### [ENG-006] — Nit — Correctness — `format!("{}/api/pull", base_url)` does no trailing-slash normalization

**Evidence**
`src-tauri/src/core/llm.rs:179`. The only production caller passes `"http://127.0.0.1:11434"` (no trailing slash, `tauri_cmds.rs:512`), so this is correct today. A future caller passing a trailing slash would produce `http://host//api/pull`.

**Fix path**
Trim a trailing `/` from `base_url` before formatting, or document the no-trailing-slash precondition on `run_ollama_pull`. Purely a robustness nit.

---

### [ENG-007] — Nit — Correctness — `CANCEL_PULL_MAP.lock().unwrap()` panics on poison

**Evidence**
`src-tauri/src/core/llm.rs:137, 173, 187, 194, 253`. Every lock uses `.unwrap()`. The critical sections are tiny (insert/get/remove with no panic-prone code inside), so poisoning is effectively unreachable. Flagged only for completeness; not worth a code change given the trivial sections.

**Fix path**
None recommended; if hardening is desired later, use `.lock().unwrap_or_else(|e| e.into_inner())` to recover from poison.

---

### [ENG-008] — Nit — Architecture — `tauri` is built with the `test` feature as a non-dev dependency

**Evidence**
`src-tauri/Cargo.toml:22`: `tauri = { version = "2", features = ["test"] }`. The `test` feature is enabled on the main dependency, not gated behind `[dev-dependencies]` or a feature flag, so Tauri's test harness compiles into release builds. **Pre-existing, out of scope** (not touched by this change set) — noted because the decoupling work reduces the project's reliance on `tauri::test::mock_app()`, which may eventually let this feature move dev-only.

**Fix path**
Future cleanup: move the `test` feature to a `dev`-only activation (e.g., via a `[features]`/`[dev-dependencies]` split) once no production code path depends on it.

---

## Patterns and systemic observations

- **The refactor is the model of how to retire a platform gate honestly.** Rather than weakening the guard or whitelisting tests, it removed the *reason* for the gate (AppHandle coupling) by introducing clean seams, then proved the result by running the tests cross-platform. Contrast with this repo's documented history of cfg-gate bypasses. The change set is a credible reversal of that pattern, and I found no evidence of gaming in scope.
- **The remaining risk is guard *coverage*, not guard *honesty* (ENG-002).** The single highest-leverage follow-up is making `verify_no_unauthorized_platform_gates` self-discover its inputs so the empty whitelist is enforced repo-wide, not across two hardcoded files. That converts a currently-true claim into a continuously-enforced invariant.
- **Cancellation identity is keyed by the wrong thing (ENG-001).** Model-id keying is a small, contained defect but it undermines the very "per-pull isolation" the refactor's comment advertises. Worth fixing in the same area while the code is fresh.

## Dependency snapshot

New/relevant dependencies introduced or relied on by the change set. No `cargo audit` was run; versions inspected from `Cargo.toml`.

| Dependency | Version | Concern |
|---|---|---|
| `tokio` | 1 (`features=["full"]`) | `watch` channel now used in core; `full` is broad but pre-existing. No concern. |
| `bytes` | 1 (dev) | Test-only, used by the pull stub. Clean. |
| `futures-util` | 0.3 (dev) | Test-only stream helper. Clean. |
| `axum` | 0.7 | Runtime dep (embedded server in `server.rs`) reused as test stub. Pre-existing; not a CVE concern at 0.7 as of audit date. |
| `async-trait` | 0.1 | Backs `LlmClient`/sink traits. Standard, maintained. |
| `tauri` | 2 (`features=["test"]`) | `test` feature in non-dev deps (ENG-008). Pre-existing. |
| `reqwest` | 0.12 | Used by `run_ollama_pull`; current major. Clean. |

No new third-party dependency was added by this change set; `bytes`/`futures-util` are dev-deps already present. Dependency surface for the diff is effectively clean.

## Appendix: artifacts reviewed

- Diffs (`git diff HEAD`): all 11 in-scope files.
- Full reads: `src-tauri/src/core/daily_scan.rs`, `src-tauri/src/core/reproduction_tests.rs`, `src-tauri/src/core/llm.rs` (lines 60-130, 255-411), `src-tauri/src/tauri_cmds.rs` (lines 30-120, 455-485, 605-645), `src-tauri/Cargo.toml`.
- Context reads: `src/useApp.ts:185-224`, `src/components/OnboardingWizard.tsx:188-232`, `docs/index.html` (download cards), `src/components/Workbench.test.tsx` (full diff).
- Repo-wide scans: platform-gate cfg attributes under `src-tauri/` (none found outside the two guarded files); `#[ignore]`/`cfg_attr(... ignore)` (none); `LlmClient`/`manage(` state registration (`lib.rs:85-87`).
- Executed on Windows: `cargo test --lib` (44 passed, 0 ignored), the eight formerly-gated tests by name (all pass, 0 ignored), `reproduce_m1_cfg_family_bypass` + `reproduce_structural_closure_0_22_violations` (pass).
