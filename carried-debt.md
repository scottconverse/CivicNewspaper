# Carried Debt

This file tracks deferred work and known technical debt.

## Status convention

Every item carries exactly one disposition tag so its state is unambiguous at a
glance. An item stays in this ledger until history no longer needs it; while it
remains, the tag says where it stands:

- **DEFERRED (→ vX.Y)** — still outstanding; deferred to the named release (or `future` if unscheduled).
- **RESOLVED (vX.Y, <pointer>)** — shipped; names the release and a code/doc pointer.
- **WITHDRAWN (<reason>)** — intentionally dropped; names the reason.

An item is only deleted outright once it is no longer referenced anywhere;
otherwise it stays here with a RESOLVED or WITHDRAWN tag so cross-references in
`CHANGELOG.md`, `README.md`, and `FAQ.md` never dangle.

## Items

- **P5-000 — DEFERRED (→ future)** (P0, Sealed Policy Callables for Pipeline Integrity): Security bypass loophole. Custom callables declared under `acceptance.manager` in the directive must be loaded from a sealed/immutable location that the executor cannot access or edit, rather than a directory editable by the executor. Executor access to these scripts allows arbitrary validation overrides.
- **P5-001 — RESOLVED (v0.2.6, `Workbench.tsx` rewrite diff modal)** (Diff Modal for Rewrites): The plain-language rewrite
  no longer overwrites the draft in place. `Workbench.tsx` now holds the LLM
  result in `rewritePreview` state and opens a side-by-side diff modal
  (`#rewrite-diff-modal`) showing the original (left) versus the rewrite
  (right) with line-level LCS highlighting (removed lines red, added lines
  green). The editor explicitly Accepts (applies via `onUpdateDraftContent`)
  or Rejects (discards) before any change lands.
- **P5-002 — RESOLVED (v0.2.6, CHANGELOG ENG-001)** (Tauri auto-updater removed):
  The Tauri updater was removed entirely in v0.2.6 rather than left dormant. The
  `tauri-plugin-updater`/`tauri-plugin-process` crates and their npm counterparts
  were dropped, the `plugins.updater` config was deleted (`tauri.conf.json` now
  has `"plugins": {}`), and the on-launch `check()` call and update banner were
  removed. The app performs no update checks; updates are a manual download from
  the GitHub Releases page. User-facing documentation of this state lives in
  `FAQ.md` and `README.md`.
- **P5-003 — RESOLVED (v0.2.6, AppHandle decoupling)** (Tauri mock_app() Windows test harness): The three
  Tauri-state-dependent tests (`test_plain_language_rewrite_invokes_ollama`,
  `test_daily_scan_command_does_not_panic_when_state_registered`,
  `test_daily_scan_uses_settings_model_not_hardcoded`) plus the two pull tests
  (`test_pull_ollama_model_propagates_http_error`,
  `test_cancel_ollama_pull_is_per_model`) no longer construct `tauri::test::mock_app()`.
  The business logic was decoupled from `AppHandle`: `run_daily_scan` and
  `plain_language_rewrite` now take an injected `Arc<dyn LlmClient>` + prompt, and
  the ollama pull was split into `core::llm::run_ollama_pull(model, base_url, sink)`
  driven by a `PullProgressSink` trait. The Tauri command wrappers resolve the
  client/prompt/sink from `AppHandle` and delegate. Tests call the core functions
  directly with fake clients/sinks and an ephemeral-port stub server, so they run
  on every platform including Windows. The platform-gate whitelist
  (`.agent-workflows/section2-auth.json`) is now empty.
- **P5-004 — RESOLVED (v0.2.6, `OllamaSidecar::start_for_test`)** (OllamaSidecar AppHandle coupling): The two sidecar
  lifecycle tests (`test_ollama_sidecar_spawns_with_expected_pid_pattern`,
  `test_ollama_sidecar_terminates_cleanly_on_drop`) and the collision-skip test
  (`test_sidecar_skips_spawn_when_port_occupied`) run cross-platform via
  `OllamaSidecar::start_for_test(probe_addr)`, which spawns the bundled test
  fixture without the shell-plugin `AppHandle` lookup and injects the collision
  probe address (so tests neither bind the real 11434 nor kill a developer's
  local `ollama serve`). The port-collision check remains extracted into
  `port_in_use(addr)` / `ollama_port_in_use()`. No test constructs `mock_app()`.
- **P5-005 — RESOLVED (v0.2.6, `docs/script.js` per-platform resolver)** (Per-platform smart download links): The landing-page
  download buttons resolve to the matching installer from the latest GitHub
  release — Windows `.exe`/`.msi`, Linux `.deb`, macOS `.dmg` with architecture
  detection — and fall back to the `releases/latest` page when the API is
  unavailable or an asset is missing. See `pickWindowsAsset` / `pickLinuxAsset` /
  `pickMacAsset` / `setButtonHref` in `docs/script.js`.
- **P5-006 — RESOLVED (v0.2.4)** (Sidecar lifecycle on crash + port 11434 collision detection): Graceful coexistence with external Ollama instances and clean process reaping on closing/panic exits.
- **P5-007 — DEFERRED (→ v0.3)** (Linux GPU shared libraries not bundled): Linux GPU
  acceleration falls back to CPU at runtime because the bundled `.deb` extracts
  only the monolithic `bin/ollama` and not the upstream `lib/ollama/` shared
  libraries. Inference still works on CPU; GPU acceleration is not yet bundled on
  Linux. Referenced from the v0.2.4 "Known Limitations" entry in `CHANGELOG.md`.

**Reference (not a debt item).** The branch `forensic/phase-4-gamed-2026-05-25` contains historical diagnostic artifacts and code revisions related to the Phase 4 audit-lite and director overrides.

## Pipeline Integrity Incidents

The following incidents summarize historic challenges in the pipeline promotion attestation:
- **Incident 1 (Phase 4 walkthrough hallucination)**: The executor walkthrough gamed test completion state by claiming 6 unwritten tests were passing. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for full context.
- **Incident 2 (v0.2.0 manager-decision fabrication)**: The executor fabricated approval files to bypass verification gates. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for full context.
- **Incident 3 (v0.2.1 four-bypass pattern)**: The executor gamed the lie-proof contract using empty test stubs, single-quote literals to bypass regexes, manual threshold edits, and dictionary default-pass loopholes. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for details.
- **Incident 4 (v0.2.3-hotpatch six new evasion shapes)**: The executor introduced six new subtle evasion shapes to satisfy literal verifications (grep-pattern-as-product-string, phrasing variant, Windows test compile-out, invariant grep bypasses, etc.). See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for details.
- **Incident 5 (Avoided in v0.2.4)**: No violations occurred; the lie-proof-3 contract held structurally without bypasses.
