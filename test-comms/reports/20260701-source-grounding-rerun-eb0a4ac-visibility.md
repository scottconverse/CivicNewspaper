# 20260701 Source Grounding Rerun eb0a4ac - Visibility Report

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Directive: `test-comms/directives/20260701-source-grounding-rerun-eb0a4ac.md`

## Installer Verification

- Installer: `test-comms/artifacts/20260701-source-grounding-rerun-eb0a4ac/The Civic Desk_0.3.1_x64-setup.exe`
- Expected SHA256: `3105CAD4EB00D6DDE501679E9C0820721267AC9F106B660735B42C3616734295`
- Actual SHA256: `3105CAD4EB00D6DDE501679E9C0820721267AC9F106B660735B42C3616734295`
- Expected size: `5622050`
- Actual size: `5622050`
- Product commit represented by installer: `eb0a4ac284eedeb281891bb468f06cf9d564b1fe`

## Cleanroom Install

Prior state was wiped before install:

- stopped stale `civicnews`
- stopped product-owned `ollama`
- ran prior `The Civic Desk` uninstaller
- removed `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- removed `C:\Users\civic\AppData\Local\The Civic Desk`
- removed `C:\Users\civic\.ollama`

The new installer exited `0`, installed `civicnews.exe`, and launched a visible desktop app window titled `The Civic Desk`.

## Product-Owned Runtime And Model

PASS.

- App process: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Product-owned Ollama runtime: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Model reached ready state by 120 seconds:
  - `phi4-mini:latest`
  - ID `78fad5d182a7`
  - size `2.5 GB`

Dashboard reached local AI ready with model selected as `phi4-mini:latest`.

## Evidence

- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/install-clean-launch.log`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/model-watch.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/machine-profile.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-10s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-30s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-60s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-120s.png`

## Notes

The post-model DB snapshot already contains Longmont starter data from the product's recovered/intake path:

- sources: 19
- evidence items: 68
- leads: 12
- daily scan runs: 0
- drafts: 0
- publish runs: 0

The identity setting still reads `My Local Publication` at this early visibility checkpoint. The directive requires saving a real Longmont publication identity during the full product flow, so this will be retested and reported in the final report.
