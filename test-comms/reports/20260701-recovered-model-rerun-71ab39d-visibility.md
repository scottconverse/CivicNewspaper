# 20260701 Recovered Model Rerun 71ab39d Visibility Check

Tester: Codex desktop tester  
Directive: `test-comms/directives/20260701-recovered-model-rerun-71ab39d.md`  
Product branch: `main`  
Installer commit represented: `71ab39d3a8f5b6c947946b6b5af6862064dc8c94`  
Installer: `test-comms/artifacts/20260701-recovered-model-rerun-71ab39d/The Civic Desk_0.3.1_x64-setup.exe`

## Result

Visibility checkpoint: PASS.

The installer matched the directive metadata, the product was wiped and installed cleanly, and The Civic Desk launched normally as a visible native window without handle manipulation. The recovered setup path stayed visible while installing the local AI runtime automatically, then started the recommended model pull programmatically after runtime readiness. By the 30 second recovered-model checkpoint, `ollama list` showed `phi4-mini:latest` installed, and by the 120 second checkpoint the app had reached the main dashboard with a visible `Local AI ready` status for `phi4-mini:latest`.

## Installer Verification

- Expected SHA256: `43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04`
- Actual SHA256: `43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04`
- Expected size: `5632526`
- Actual size: `5632526`

## Clean Install And Launch

- Stopped stale `civicnews` and `ollama` processes before install.
- Removed prior product data from `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, and `%USERPROFILE%\.ollama`.
- Installed from the directive installer.
- Launched normally from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Visible window observed: `The Civic Desk`, process `civicnews.exe`.

## Recovered Model Watch

- 10 seconds: app visible on setup path; `ollama.exe` running from product-owned runtime path; partial model blob files present under `%USERPROFILE%\.ollama\models`.
- 30 seconds: `ollama list` showed `phi4-mini:latest`, model id `78fad5d182a7`, size `2.5 GB`.
- 60 seconds: `ollama list` still showed `phi4-mini:latest`.
- 120 seconds: `ollama list` still showed `phi4-mini:latest`; app visible on main dashboard with `Local AI ready` and `phi4-mini:latest`.

## Evidence

- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/cleanwipe-install-launch.log`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/recovered-model-watch.txt`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/db-snapshot-after-recovered-model-watch.json`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-01-normal-launch-after-30s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-10s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-30s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-60s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-120s.png`

