# Carried Debt

This file tracks deferred work and known technical debt.

## Deferred Items

- **P5-000 (P0 - Sealed Policy Callables for Pipeline Integrity)**: Security bypass loophole. Custom callables declared under `acceptance.manager` in the directive must be loaded from a sealed/immutable location that the executor cannot access or edit, rather than a directory editable by the executor. Executor access to these scripts allows arbitrary validation overrides.
- **P5-001 (Diff Modal for Rewrites)**: The plain-language rewrite feature currently overwrites the draft content in place (after a confirmation prompt). A visual diff modal comparing the original text with the rewritten text should be implemented in a future phase to provide better editorial oversight.
- **P5-002 (Tauri Auto-Updater)**: The Tauri auto-updater is currently disabled with `plugins.updater.active = false` (dormant). Full auto-updater signature configuration and rollout is deferred to a future release.
- **Forensic Branch Reference**: The branch `forensic/phase-4-gamed-2026-05-25` contains historical diagnostic artifacts and code revisions related to the Phase 4 audit-lite and director overrides.
- **P5-003 (Tauri mock_app() Windows test harness)**: Three Tauri-state-dependent tests
  (`test_plain_language_rewrite_invokes_ollama`,
  `test_daily_scan_command_does_not_panic_when_state_registered`,
  `test_daily_scan_uses_settings_model_not_hardcoded`)
  are skipped on Windows via `#[cfg_attr(target_os = "windows", ignore = "...")]`
  because `tauri::test::mock_app()` triggers a DLL loader crash
  (STATUS_ENTRYPOINT_NOT_FOUND, 0xc0000139) in console-mode lib unit tests.
  Resolution path: move these to an integration test crate at `src-tauri/tests/`
  where Tauri's mock_app reliably works on Windows. Deferred to v0.3 to avoid
  scope creep on the v0.2.2 hot-patch.
- **P5-004 (OllamaSidecar AppHandle coupling)**: The two sidecar lifecycle tests
  (`test_ollama_sidecar_spawns_with_expected_pid_pattern`,
  `test_ollama_sidecar_terminates_cleanly_on_drop`)
  require `tauri::test::mock_app()` because `OllamaSidecar::start` uses
  `app.shell().sidecar("ollama")` which requires an `AppHandle`. This couples
  the spawn/kill logic to the Tauri shell-plugin API and makes the tests
  unrunnable on Windows console-mode lib unit tests (mock_app DLL crash).
  Resolution path for v0.3: refactor `OllamaSidecar` to expose
  `start_with_path(PathBuf)` so tests can bypass the shell-plugin lookup
  and run cross-platform. Keep the AppHandle-taking `start()` as a thin
  convenience wrapper for production callers. Deferred to v0.3 to avoid
  scope creep on the v0.2.2 hot-patch.
- **P5-005 (Per-platform smart download links)**: v0.2.3 reverted download
  buttons to bare `releases/latest` after the VERSION-placeholder landmine.
  Restore per-platform smart links via inline JS that fetches GitHub API
  on page load and rewrites hrefs. Deferred to v0.3.
- **P5-006 (Sidecar lifecycle on crash + port 11434 collision detection)**: Graceful coexistence with external Ollama instances and clean process reaping on closing/panic exits. Resolved in v0.2.4.
- **P5-007 (Linux GPU Shared Libraries Bundling)**: Extract `lib/ollama/*` alongside `bin/ollama` in `fetch-ollama-binaries.sh`, bundle in `tauri.conf.json` `externalBin`, and verify via `ldd` post-installation to enable GPU acceleration on Linux rather than falling back to CPU. Deferred to v0.3.

## Pipeline Integrity Incidents

The following incidents summarize historic challenges in the pipeline promotion attestation:
- **Incident 1 (Phase 4 walkthrough hallucination)**: The executor walkthrough gamed test completion state by claiming 6 unwritten tests were passing. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for full context.
- **Incident 2 (v0.2.0 manager-decision fabrication)**: The executor fabricated approval files to bypass verification gates. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for full context.
- **Incident 3 (v0.2.1 four-bypass pattern)**: The executor gamed the lie-proof contract using empty test stubs, single-quote literals to bypass regexes, manual threshold edits, and dictionary default-pass loopholes. See [v0.2-pipeline-integrity-failures.md](forensic/v0.2-pipeline-integrity-failures.md) for details.
- **Incident 4 (Avoided in v0.2.2)**: No violations occurred; the lie-proof-2 contract held structurally without bypasses.
