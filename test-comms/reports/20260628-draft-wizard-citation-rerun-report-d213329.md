# Tester Report - d213329 draft wizard/citation rerun

Date: 2026-06-28 13:49Z  
Tester machine: Windows 11 Intel/NVIDIA laptop  
Repo: `https://github.com/scottconverse/CivicNewspaper.git`  
Product branch: `stable-readiness-local-gates`  
Product commit: `d2133295ec7d9e930bde170ee427158ea169e8a5`  
Directive: `test-comms/directives/20260628-rerun-draft-wizard-citation-after-d213329.md`  
Result: BLOCKED

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 354.2 GB
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before app setup: no
- Models present after clean reset: none

## Installer

- Preferred installer used: `test-comms/artifacts/d213329-draft-wizard-citation-rerun/The Civic Desk_0.2.8_x64-setup.exe`
- Expected SHA256: `C0CC01CC4B3676A97C8BCC221F088DCE6F1058466584FB514B152FC7E3DCB10F`
- Observed SHA256: `C0CC01CC4B3676A97C8BCC221F088DCE6F1058466584FB514B152FC7E3DCB10F`
- MSI fallback hash also verified: `2BE3AA89013CB0EB1985D0450A69CC40FA2901FB41DD23D8951581AF92025CE0`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read the d213329 directive.
2. Verified `origin/stable-readiness-local-gates` resolved to `d2133295ec7d9e930bde170ee427158ea169e8a5`.
3. Stopped prior `civicnews`, `ollama`, and installer processes.
4. Removed prior product/runtime state under the app profile, local app install folder, `.ollama`, and Ollama app-data folders.
5. Installed the preferred NSIS installer silently.
6. Launched the real installed desktop app from the local app install folder.
7. Completed identity fields through the app UI:
   - Publication: `The Longmont Ledger`
   - Editor: `Cleanroom Tester`
   - City/state: `Longmont`, `CO`
8. Advanced to the app-managed local AI setup step.
9. Clicked `Install local AI runtime` twice from the visible app UI and waited after each click.
10. Checked for an `ollama` process and app log output.

## Results

- Clean install: PASS
- Natural first-run desktop onboarding: PASS
- Identity saved through UI: PASS
- App-managed local AI runtime setup: BLOCKED
- Source import/classification: NOT RUN
- Daily Scan/Story Queue: NOT RUN
- Draft wizard repeated generation: NOT RUN
- Citation-integrity validation: NOT RUN
- Editor approve/hold/advisor/edit controls: NOT RUN
- App export ZIP: NOT RUN
- App anonymous here.now publish: NOT RUN

The directive forbids manual prerequisite/model installation and forbids manual DB draft insertion for this rerun. Because app-driven runtime setup did not start, I stopped at the exact blocker instead of bypassing the product UI.

## Evidence

Screenshots and artifacts are under `test-comms/artifacts/20260628-draft-wizard-citation-rerun-d213329/`:

- `01-first-run.png` - clean first-run app screen.
- `05-identity-corrected.png` - identity fields corrected through the UI.
- `08-identity-bottom.png` - lower identity form with `Next` visible.
- `12-after-next-attempt.png` - app advanced to local AI setup.
- `13-runtime-installing.png` - app reports it cannot reach local AI service and offers install.
- `14-runtime-install-button-inert.png` - after repeated install-button clicks, still on the same runtime setup state.

App state evidence:

- `settings` table contained the UI-entered identity values and selected model `qwen2.5:7b`.
- `sources`: 0
- `leads`: 0
- `drafts`: 0
- `ollama` process count after app install-button attempts: 0
- App log directory was not present at the checked path.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### BLOCKER: App-managed runtime install does not start from first-run setup

Observed: On a clean app/runtime state, the first-run AI setup page showed `Couldn't reach the AI service` and the `Install local AI runtime` button. Clicking that app control twice did not start an `ollama` process, did not advance progress, and did not produce an app log directory at the checked location.

Expected: The app should drive the local AI runtime install/start flow without tester-installed prerequisites, then proceed to model download.

Impact: The d213329 rerun cannot reach source intake, draft wizard, citation-integrity, export, or publish testing under the directive's UI-only/no-manual-prereq constraints.

Repro: Clean reset product/app/Ollama state, install the d213329 NSIS build, complete identity, advance to AI setup, click `Install local AI runtime`, and observe no runtime process or progress.

### MINOR: First-run form required window repositioning to reach controls on 1280x720 capture viewport

Observed: The identity form's lower controls and `Next` button were partially off-screen until the app window was moved/resized. After repositioning, the fields and `Next` became reachable.

Expected: First-run onboarding controls should remain visible or scrollable in a standard 1280x720 desktop capture viewport.

Impact: This did not block identity entry, but it is related to the prior UI-reachability failures and should stay on the polish list.

## Request For Coder

Please investigate why `Install local AI runtime` does not start the app-managed runtime on a clean machine for d213329. Once that is fixed or clarified, issue a new directive with the next product commit and installer so I can rerun the draft wizard/citation-integrity gate without manual DB or prerequisite bypasses.
