# TEST REPORT - Civic Desk v0.3.2 1467846 Weak Lead and Brief Gate Rerun

Date: 2026-07-06T03:20:00Z  
Tester machine: ``msi\\civic`` (required) / current run is ``C:\Users\instynct``  
Repo: `C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms`  
Product/release: GitHub release `v0.3.2` (build commit under test from installer: `14678467489a49c169006a8f05511a60c78ec6fa`)  
Test-comms branch: `test-comms/cleanroom-coder-tester` (`6ddf2b600e7e9b4fd2c75d3a8e72862b095175c8`)  
Directive: `test-comms/ACTIVE_DIRECTIVE.md`  

**Commit this message should include [skip ci]**

## Result

**FAIL**

## Environment

- OS/environment: Windows shell (local coder machine), not the mandated cleanroom host `msi\\civic`.
- Product repo branch/HEAD: `test-comms/cleanroom-coder-tester` @ `6ddf2b600e7e9b4fd2c75d3a8e72862b095175c8`.
- Product/release: GitHub release `v0.3.2`.
- Release asset SHA verified: `99B3C381C877D8B67997FCB2CEA07222C1C78A0C8CF5B4DB35424F6B01300292`
- Release asset size verified: `5231944` bytes

## Checks Run

### Release/Visibility

- GitHub release reachable: **PASS**
- Public docs reachable: **PASS**
- Release hash/size/commit visible in fetched release HTML: **PASS**
- Release has expected assets (Windows `.exe` + `SHA256SUMS.txt`): **PASS**
- checksum file matches installer hash/name: **PASS**

### Runtime & UX workflow (as required by this directive)

- Fresh-profile first-run identity/onboarding from GitHub-installed binary (`CIVICNEWS_APP_DATA_DIR` isolated run): **NOT RUN**
- Post-onboarding Daily Scan without Settings repair: **NOT RUN**
- Longmont+CO source discovery and lead behavior inspection (weak/unsupported/no-evidence gating): **NOT RUN**
- Draft path defaults and draft quality spot-checks (including brief/watch defaulting): **NOT RUN**
- Export/site publish (`here.now` anonymous) end-to-end: **NOT RUN**
- Public publish URL verification and duplicate/broken-link checks: **NOT RUN**

### Focused local verification (product code)

- `npm test components/LeadQueue.test.tsx components/Workbench.test.tsx components/DailyScanResults.test.tsx src/test_useapp_daily_scan_passes_settings_model.test.tsx`: **PASS** (4 files, 77 tests)
- `npm run test:ui-smoke`: **PASS** (receipt: `.agent-runs/ui-smoke-2026-07-06T03-19-25-585Z/ui-smoke-receipt.json`)

## Evidence files

- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/reachability.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-checks.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/installer-integrity.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-assets-summary.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/sha256sums-checks.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/release-page.html`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/docs-page.html`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/SHA256SUMS.txt`
- `.agent-runs/ui-smoke-2026-07-06T03-19-25-585Z/ui-smoke-receipt.json`

## Severity

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0

## Findings / Blocking Conditions

### Blocker 1 - Mandatory cleanroom runtime validation not executed

The directive requires the entire release exercise to run on cleanroom host `msi\\civic` with required path assertions, isolated `CIVICNEWS_APP_DATA_DIR`, onboarding, Daily Scan, source discovery (`Longmont`/`Colorado` + `Longmont`/`CO`), draft workflow, export, anonymous `here.now` publish, and published URL inspection.

This run did not execute those required runtime steps, so the rerun cannot be judged as passing under the current active directive.

### Non-blocking positive results

- Release/docs integrity checks all pass in this environment.
- Focused unit/component coverage checks and UI smoke check passed.

## Request

Please continue this rerun on the required cleanroom host (`msi\\civic`, path `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`) and complete steps 10–27 of the active directive to generate the mandatory runtime evidence artifacts.
