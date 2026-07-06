# VISIBILITY REPORT - Civic Desk v0.3.2 1467846 Weak Lead and Brief Gate Rerun

Date: 2026-07-06T03:20:00Z  
Tester machine: ``msi\\civic`` (required) / ``C:\Users\instynct`` (actual)  
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms` (required by directive) / `C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms` (actual)  
Branch: `test-comms/cleanroom-coder-tester`  
Directive: `test-comms/ACTIVE_DIRECTIVE.md`  
Product commit under test: `14678467489a49c169006a8f05511a60c78ec6fa`  
Release commit/checkpoint: release page `v0.3.2`, docs hash `99B3C381C877D8B67997FCB2CEA07222C1C78A0C8CF5B4DB35424F6B01300292`

## Result

**FAIL** (cleanroom context required by directive is not satisfied in this session, so mandatory visibility checks 1–2 cannot be fully validated from this environment.)

## Required Confirmations

- tester machine is `msi\civic`: **FAIL** (running on `C:\Users\instynct`).
- repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`: **FAIL** (running on `C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms`).
- branch is `test-comms/cleanroom-coder-tester`: **PASS**.
- ACTIVE_DIRECTIVE.md was read: **PASS**.
- release URL reachable (`200`): **PASS**.
- public docs URL reachable (`200`): **PASS**.
- release page includes expected installer SHA256/size/commit: **PASS**.
- release page lists exactly 2 assets (`SHA256SUMS.txt`, `The.Civic.Desk_0.3.2_x64-setup.exe`): **PASS**.
- `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains expected SHA: **PASS**.
- public docs show expected hash, `More info`, `Run anyway`, unsigned warning language, Windows-only beta language, and no stale hash `6CD5B8C6D3565AFAE8A39357DEAEC1CE53ADEDADB8316BEB6C44DCB86C87EE74`: **PASS** (per fetched snapshots).

## Evidence Used

- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/reachability.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-checks.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/installer-integrity.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-assets-summary.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/sha256sums-checks.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-page.html`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/docs-page.html`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/SHA256SUMS.txt`

## Notes

This report is intentionally marked **FAIL** because this run is not on the required cleanroom node/path (`msi\civic`) and therefore cannot satisfy all mandatory pre-test visibility assertions from the active directive.
