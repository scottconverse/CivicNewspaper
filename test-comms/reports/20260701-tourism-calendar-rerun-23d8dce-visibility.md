# 20260701 Tourism Calendar Rerun 23d8dce - Visibility Report

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Directive: `test-comms/directives/20260701-tourism-calendar-rerun-23d8dce.md`

## Installer Verification

- Installer: `test-comms/artifacts/20260701-tourism-calendar-rerun-23d8dce/The Civic Desk_0.3.1_x64-setup.exe`
- Expected SHA256: `49372BCF0FB4A6F149E316DDAEC2CC42B48EAB82FC5644AEE164A58D7D8DC6FB`
- Actual SHA256: `49372BCF0FB4A6F149E316DDAEC2CC42B48EAB82FC5644AEE164A58D7D8DC6FB`
- Expected size: `5638803`
- Actual size: `5638803`
- Product commit represented by installer: `23d8dcec12adf5b5dadd4f48dd9906edb1c1aa56`

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

The dashboard reached local AI ready with model selected as `phi4-mini:latest`.

## Evidence

- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/install-clean-launch.log`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/model-watch.txt`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/machine-profile.txt`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-10s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-30s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-60s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-120s.png`

## Notes

The post-model DB snapshot contains Longmont source intake state:

- sources: 19
- evidence items: 63
- leads: 0
- daily scan runs: 0
- drafts: 0
- publish runs: 0

At this visibility checkpoint, `identity.newsroom_name` still reads `My Local Publication`. The full product flow will save a real Longmont publication identity and retest the identity setting in the final report.
