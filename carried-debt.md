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
