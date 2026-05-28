# Stop Report 001 — Cross-Platform Test Blocker on Windows

## Condition Triggered
**S-9**: A test cannot pass without `#[cfg]` gating to platform-specific code.

## Description of the Blocker
The Tauri test harness API `tauri::test::mock_app()` is unavailable/non-functional on Windows console unit tests for the library crate in this project. Invoking it results in a runtime DLL loader mismatch crash:
`exit code: 0xc0000139, STATUS_ENTRYPOINT_NOT_FOUND`

This prevents the following tests from running or passing on Windows:
- `test_plain_language_rewrite_invokes_ollama`
- `test_daily_scan_command_does_not_panic_when_state_registered`
- `test_daily_scan_uses_settings_model_not_hardcoded`

Per WT-1 and S-9 instructions:
1. We have NOT introduced any new `assert!(true)` stub or bypasses.
2. We are halting execution and filing this report to request the operator's decision/amendment on authorizing `#[cfg(not(target_os = "windows"))]` gating for these Tauri test harness-dependent unit tests.
