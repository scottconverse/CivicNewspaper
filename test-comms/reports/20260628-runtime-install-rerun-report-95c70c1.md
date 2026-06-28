# Tester Report - 95c70c1 runtime install rerun

Date: 2026-06-28 14:44Z  
Tester machine: Windows 11 Intel/NVIDIA laptop  
Repo: `https://github.com/scottconverse/CivicNewspaper.git`  
Product branch: `stable-readiness-local-gates`  
Product commit: `95c70c1e756511399671e37d76bc95339470854f`  
Directive: `test-comms/directives/20260628-rerun-runtime-install-after-95c70c1.md`  
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

- Preferred installer used: `test-comms/artifacts/95c70c1-runtime-install-rerun/The Civic Desk_0.2.8_x64-setup.exe`
- Expected SHA256: `62D5E248265E6AE81A58D192D72A720163B855A42C67D72DA63AA18B0FCECE50`
- Observed SHA256: `62D5E248265E6AE81A58D192D72DA63AA18B0FCECE50`
- MSI fallback hash also verified: `939AE2CCFC21AC9A38A0BBC78BAD6C9A8F79936832FE97522CD541E6B54AB842`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read the 95c70c1 directive.
2. Verified `origin/stable-readiness-local-gates` resolved to `95c70c1e756511399671e37d76bc95339470854f`.
3. Stopped prior `civicnews`, browser, installer, and `ollama` processes.
4. Removed prior product/runtime state under the app profile, local app install folder, `.ollama`, and Ollama app-data folders.
5. Installed the preferred NSIS installer silently.
6. Launched the real installed desktop app from the local app install folder.
7. Reached first-run onboarding and entered identity values through the app UI. The form remained awkward at 1280x720 and required window repositioning. During synthetic UI entry, the saved identity ended malformed, but the app did advance to the runtime setup step.
8. On the AI setup step, clicked the inline `Install local AI runtime` button once and waited.
9. Checked for immediate progress UI, `ollama` process, Ollama profile folders, and port 11434 listener.
10. Because inline install did not react, clicked the footer `Next` button on Step 2 and waited.
11. Rechecked visible UI, runtime processes/folders, and listening ports.

## Results

- Clean install: PASS
- Real desktop launch: PASS
- First-run onboarding reached naturally: PASS
- App-driven local AI runtime setup: BLOCKED
- Inline `Install local AI runtime` immediate progress: FAIL
- Footer `Next` runtime-install fallback: FAIL
- Recommended model download: NOT RUN
- Source import/classification: NOT RUN
- Daily Scan/Story Queue: NOT RUN
- Draft wizard/citation rerun: NOT RUN
- Export ZIP / here.now publish: NOT RUN

Visible app message after both controls:

> Couldn't reach the AI service

The page continued to say the private AI service did not start and offered `Install local AI runtime`, `Retry`, and `Save diagnostics file`. No new progress state or more specific visible error appeared after either required control path.

## Runtime Evidence

After inline install and footer Next attempts:

- `ollama` process count: 0
- Port 11434 listener: none
- Port 12053 listener: app loopback server only
- Ollama local/profile folders: absent
- App DB source count: 0
- App DB lead count: 0
- App DB draft count: 0

The app profile contained only the app DB and `community_profile.json`; no app log directory was present at the checked location.

## Evidence Files

Screenshots are under `test-comms/artifacts/20260628-runtime-install-rerun-95c70c1/`:

- `01-first-run.png` - clean first-run desktop screen.
- `03-identity-corrected.png` - identity-entry attempt through UI.
- `04-runtime-step.png` - AI service setup banner before install click.
- `05-after-inline-install-click.png` - no visible progress after inline install click.
- `06-after-footer-next-click.png` - no visible progress after footer Next fallback.

Additional screenshots in the same folder show intermediate identity-entry/window-positioning states.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

### BLOCKER: Runtime install controls still do not start app-managed setup

Observed: On a clean product/runtime state, the AI setup page showed the expected failure banner and install controls. Clicking inline `Install local AI runtime` once did not change the page to an installing/progress state, did not start `ollama`, did not create Ollama profile folders, and did not open port 11434. Clicking footer `Next` also left the same banner visible with no runtime process or progress.

Expected: Inline install should immediately show installing/progress state and start or visibly fail the app-managed runtime setup. If inline install fails to react, footer `Next` should start runtime setup or surface a specific visible error.

Impact: The rerun cannot proceed to model download, source intake, draft wizard/citation integrity, export, or publish without violating the no-manual-prerequisite/no-manual-DB directive.

Repro: Clean reset app/runtime state, install 95c70c1 NSIS build, launch real app, advance to AI setup, click inline `Install local AI runtime`, then footer `Next`; observe unchanged banner and no runtime process.

### MAJOR: Identity form remains hard to operate at 1280x720

Observed: The onboarding form required moving/resizing the app window to expose lower controls. Synthetic UI input repeatedly landed in the wrong fields while the page was shifted; the app still advanced with malformed identity values.

Expected: First-run identity fields and footer controls should remain reachable and straightforward in a standard 1280x720 desktop viewport, and the app should validate required identity fields before advancing.

Impact: This complicates cleanroom verification and risks bad publication identity/profile data.

### MINOR: No app diagnostics/log artifact found after runtime failure

Observed: After the runtime setup failure, no log directory was present at the checked app-profile location. I did not save a diagnostics file because the UI action could expose private machine paths in a file picker or artifact.

Expected: Runtime setup failures should leave a clear local log/diagnostic artifact path or visible error detail safe for tester reporting.

## Request For Coder

Please investigate why both first-run runtime setup controls remain inert on the clean Windows machine for 95c70c1. The fix still needs to prove that the inline button changes state immediately and that either the inline path or footer Next starts app-managed runtime setup without tester-installed prerequisites.
