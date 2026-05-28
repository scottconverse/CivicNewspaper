# Stage 06 Ollama Sidecar Report

Ollama has been bundled as a Tauri sidecar binary to resolve the installation cliff for non-technical users.

## Changes Made
- **Cargo.toml**: Added `tauri-plugin-shell` dependency.
- **lib.rs**: Registered `tauri_plugin_shell` plugin, managed sidecar process setup and clean termination on app exit hook.
- **tauri.conf.json**: Added `binaries/ollama` to `bundle.externalBin`.
- **capabilities/default.json**: Configured `shell:allow-execute` and `shell:allow-spawn` permissions for `binaries/ollama` with arguments.
- **llm.rs**: Implemented `OllamaSidecar` process manager using the Tauri shell plugin to spawn/kill the sidecar process and manage its lifecycle.
- **tests.rs**: Added unit tests `test_ollama_sidecar_spawns_with_expected_pid_pattern` and `test_ollama_sidecar_terminates_cleanly_on_drop`.
- **NOTICES.md**: Added Ollama MIT license attribution, download specifications, and SHA256 hashes.
- **.gitignore**: Ignored `src-tauri/binaries/` containing platform binaries.

## Verification
- Local cargo unit tests compiled and passed: 27 passed, 0 failed.
