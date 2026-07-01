# 20260701 Evidence Linkage Rerun 33bc936 Visibility Check

Tester: Codex desktop tester  
Directive: `test-comms/directives/20260701-evidence-linkage-rerun-33bc936.md`  
Product branch: `main`  
Product commit represented: `33bc93645ed3a726d7292bd5aad394a677add4e8`  
Installer: `test-comms/artifacts/20260701-evidence-linkage-rerun-33bc936/The Civic Desk_0.3.1_x64-setup.exe`

## Result

Visibility checkpoint: PASS.

The installer matched the directive hash and size, product state was wiped, the NSIS package installed cleanly, and The Civic Desk launched normally as a visible native window. The recovered setup path stayed visible while installing the product-owned local AI runtime, then automatically started the recommended model pull. By the 30 second model checkpoint, `ollama list` showed `phi4-mini:latest` installed. By the 120 second checkpoint, the app had reached the main dashboard with `Local AI ready` and `phi4-mini:latest`.

## Installer Verification

- Expected SHA256: `4968F81CF21CBAD5DD634375DBF00F67595CE0A023DF0654358F9FBD3092E8E4`
- Actual SHA256: `4968F81CF21CBAD5DD634375DBF00F67595CE0A023DF0654358F9FBD3092E8E4`
- Expected size: `5638753`
- Actual size: `5638753`

## Clean Install And Launch

- Stopped stale `civicnews` and `ollama` processes before install.
- Removed prior product data from `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%USERPROFILE%\.ollama`, and the prior app install path.
- Installed from the directive installer.
- Launched normally from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Visible window observed: `The Civic Desk`, process `civicnews.exe`.

## Model Watch

- 10 seconds: app visible; product-owned `ollama.exe` running from `%APPDATA%\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`; partial model blob present.
- 30 seconds: `ollama list` showed `phi4-mini:latest`, model id `78fad5d182a7`, size `2.5 GB`.
- 60 seconds: `phi4-mini:latest` still present.
- 120 seconds: `phi4-mini:latest` still present; dashboard visible with `Local AI ready`.

## Evidence

- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/install-clean-launch.log`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/model-watch.txt`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/screenshot-01-after-launch-30s.png`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/screenshot-model-10s.png`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/screenshot-model-30s.png`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/screenshot-model-60s.png`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/screenshot-model-120s.png`

