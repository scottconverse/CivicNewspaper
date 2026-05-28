# Stop Report 002 — Sidecar Test Blocker on Windows

## Condition Triggered
**S-9**: A test cannot pass without `#[cfg]` gating to platform-specific code.

## Description of the Blocker
To fulfill WT-3, the unit tests `test_ollama_sidecar_spawns_with_expected_pid_pattern` and `test_ollama_sidecar_terminates_cleanly_on_drop` must verify the process lifecycle of `OllamaSidecar`. Because `OllamaSidecar::start` requires an `AppHandle`, these tests must use `tauri::test::mock_app()`. However, `mock_app()` triggers a DLL loader crash (`exit code: 0xc0000139, STATUS_ENTRYPOINT_NOT_FOUND`) on Windows console library unit tests.

Per the contract and S-9:
1. We have NOT introduced any new `assert!(true)` stubs, bypasses, or unauthorized `#[cfg]` gates.
2. We are halting execution and filing this report to request the operator's decision/amendment on authorizing `#[cfg_attr(target_os = "windows", ignore = "...")]` gating for the sidecar tests.
