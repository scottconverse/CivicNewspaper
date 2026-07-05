# TEST REPORT (REWRITTEN FOR CODER DISCOVERY)
#
# Coder discovery anchor (do not move):
# Repo: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
# Branch: test-comms/cleanroom-coder-tester
# Active directive file: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\ACTIVE_DIRECTIVE.md
# Expected report path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260705-final-release-v032-1863122-identity-evidence-report.md
# Commit this message should include [skip ci]

# Tester Report - Civic Desk v0.3.2 1863122 Identity And Evidence

Date: 2026-07-05T04:26:53Z  
Tester machine: `msi\civic`  
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`  
Product/release: GitHub release `v0.3.2`  
Product build commit under test: `186312209b743824ae33bd48777c90b0e6a545ec`  
Directive: `test-comms/ACTIVE_DIRECTIVE.md` / `20260705-final-release-v032-1863122-identity-evidence.md`

## Result

**FAIL**

The rerun confirms targeted fixes for onboarding identity and discovery are working. Release visibility checks passed, cleanprofile first-run opens onboarding as expected, and `Longmont`/`CO` now persists to `community_profile.json` without manual Settings repair before Daily Scan.

The release still fails because Story Queue/draft behavior and draft quality still expose unsupported, weak, or no-linked-evidence leads as normal draft candidates and generated copy for a ready lead is publication-unready.

## Environment

- Windows: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 15.71 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on C: 341 GB
- Ollama: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- App launch path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Clean profile override: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-1863122-identity-evidence`

## Steps Run

1. Pull this branch and confirm `test-comms/ACTIVE_DIRECTIVE.md`.
2. Download `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from GitHub release.
3. Verify installer size/hash and commit fields.
4. Confirm public/docs visibility requirements and stale hash absence (see evidence list).
5. Capture pre-install app-data path states.
6. Uninstall prior app instance, install release installer from GitHub, launch only installed EXE under `CIVICNEWS_APP_DATA_DIR`.
7. Complete onboarding for Longmont, CO as new user.
8. Run Daily Scan immediately post-onboarding (no Settings repair).
9. Capture `community_profile.json` after onboarding and after discovery.
10. Run discovery with `Colorado` and `CO`.
11. Run Daily Scan and inspect Story Queue / SQLite artifacts.
12. Attempt draft flow; stop short of export/publish due major quality findings.

## Results

- Visibility: **PASS**.
- Installer visibility + hash/size: **PASS**.
- Installed-app launch from GitHub asset: **PASS**.
- Clean profile run: **PASS**.
- First-run identity correctness: **PASS**.
- Source discovery (Colorado / CO): **PASS**.
- Daily Scan quality gate: **FAIL**.
- Workbench/draft quality gate: **FAIL**.
- Export/publish: **NOT RUN** (quality blockers present).

## Evidence files (required location)

- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/visibility-receipt.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cleanprofile-launch.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/community_profile-after-onboarding-before-settings.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-run-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/sqlite-lead-evidence-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/sqlite-drafts-publish-summary.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-11-story-queue-after-scan.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-12-ready-lead-draft-start.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/cdp-13-generated-draft.png`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/environment-final.json`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/release-page.html`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/public-docs.html`
- `test-comms/reports/20260705-final-release-v032-1863122-identity-evidence-evidence/SHA256SUMS.txt`

## Severity

- Blocker: 0
- Critical: 0
- Major: 2
- Minor: 2
- Nit: 0

## Findings

### Major 1 - Story Queue still exposes draft paths for weak/no-linked-evidence leads

- 20 leads were observed in summary; 15 had zero linked evidence.
- Weak/background/watch-style leads still showed normal-looking draft actions.
- This remains a release blocker per identity/evidence quality expectations.

### Major 2 - Draft generation from ready linked-evidence lead is low quality

- A ready St. Vrain Valley Schools lead generated malformed, fragmentary draft content with headline/body quality issues.
- Public-ready draft path failed the quality expectation despite linked evidence.

### Minor - Discovery duplication / format mismatch

- Repeated discovery attempts retained duplicates/overlap.
- `Article Format` default did not align with lead treatment label (`Brief` vs `watch`).

## Request for Coder

1. Quarantine/disable normal draft UX for no-linked-evidence and weak/no-ready leads.
2. Improve draft generation output quality for ready linked-evidence leads (headline/body completeness and signal quality).
3. Align draft format default with lead treatment label in Workbench.
