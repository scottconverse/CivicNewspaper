# Tester Report - step2 runtime rerun faac9fc

Date: 2026-07-01T05:23:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: faac9fc39224d7629e4d5bff870a55b8d33ec9f7
Directive: test-comms/directives/20260630-step2-runtime-rerun-faac9fc.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376367652864 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: not found / not running
- Models present: clean-wiped; no prior model state present

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260630-step2-runtime-rerun-faac9fc.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260630-step2-runtime-rerun-faac9fc\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews` / `ollama` if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama runtime/model state used by Civic Desk
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Confirmed Step 1 auto-continued to Step 2 without tester clicking Next.
9. Captured DB snapshot showing identity settings persisted.
10. Verified the Step 2 footer Next button was disabled while the local AI service was unavailable; did not click it.
11. Clicked the visible `Install local AI runtime` control area.
12. Captured screenshot after the click, then closed the Chrome window that came to foreground on here.now dashboard sign-in. No external publish was performed.
13. Re-captured app state and runtime diagnostics.

## Results

FAIL at the Step 2 runtime recovery control gate.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery auto-continues to Step 2: PASS.
- Identity settings persisted in app DB: PASS.
- Step 2 footer Next disabled while local AI service unavailable: PASS.
- Step 2 local AI runtime install controls fully visible/clickable above footer: FAIL.
- Product-owned runtime install: NOT PROVEN because the install control remained clipped and the click did not start runtime installation.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, compile/export, and here.now publish: NOT RUN because directive says to stop at a failed gate.

Observed Step 2 state:

- Step 2 showed `Starting the local AI service`, then stayed on AI service setup with no local AI service available.
- The `Install local AI runtime` and `Check Initialization Status` controls were still partially clipped behind the sticky footer at the bottom of the content panel.
- The Step 2 footer Next button appeared disabled, which fixes the prior hide-on-Next path for this gate.
- Clicking the visible install-runtime control area did not start runtime installation. Instead, Chrome came to foreground on `here.now/dashboard` sign-in. No here.now publish was performed.
- After closing Chrome, The Civic Desk remained visible on Step 2 with the same clipped runtime controls.

## Evidence

- test-comms/reports/20260630-step2-runtime-rerun-faac9fc-visibility.md
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/cleanwipe-install-launch.log
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-02-after-click-install-runtime.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-03-app-state-after-closing-chrome.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/db-snapshot-step2-auto-continue.json
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/db-snapshot-final-step2-clipped.json
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/community_profile.json
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/runtime-diagnostics.txt
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/environment.txt

Persisted identity settings:

```json
[
  {"key": "identity.city", "value": "Longmont"},
  {"key": "identity.editor_name", "value": "Publisher"},
  {"key": "identity.newsroom_name", "value": "My Local Publication"},
  {"key": "identity.organization_type", "value": "single_person"},
  {"key": "identity.state", "value": "CO"},
  {"key": "model.selected", "value": "phi4-mini:latest"}
]
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Step 2 runtime install controls remain clipped and runtime install is not usable

Observed: On Step 2, the local runtime recovery controls were still partly covered by the sticky footer. The top/label area of `Install local AI runtime` was visible, but the lower body of the button was clipped. Clicking the visible area did not start runtime installation and instead brought Chrome to the foreground at here.now dashboard sign-in. After closing Chrome, The Civic Desk remained on Step 2 with the same clipped controls.

Expected: The install-runtime controls should be fully visible above the footer and clicking `Install local AI runtime` should let the product drive local AI runtime installation.

Impact: A clean-machine user cannot complete product-owned AI setup, so the required Longmont E2E publication flow cannot proceed.

Repro:

1. Clean wipe Civic Desk app data, output folders, and Ollama runtime/model state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-step2-runtime-rerun-faac9fc/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for Step 1 to auto-continue to Step 2.
5. Observe disabled footer Next and clipped Step 2 runtime recovery controls.
6. Click the visible install-runtime control area.
7. Observe runtime installation does not start.

## Request For Coder

Please fix Step 2 layout so the runtime install controls are fully visible/clickable above the footer at this normal desktop viewport, and verify the install button invokes product-owned runtime setup instead of leaving the app blocked on Step 2. The disabled Step 2 Next behavior is fixed in this build.
