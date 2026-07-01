# Tester Report - autocontinue rerun 4e96d0a

Date: 2026-07-01T05:10:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 4e96d0a2bc744364388c2a92316e25eb67b28c63
Directive: test-comms/directives/20260630-autocontinue-rerun-4e96d0a.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376382885888 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: not found / not running
- Models present: clean-wiped; no prior model state present

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260630-autocontinue-rerun-4e96d0a.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260630-autocontinue-rerun-4e96d0a\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews` / `ollama` if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama model state
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Confirmed Step 1 auto-continued to Step 2 without tester clicking Next.
9. Captured DB snapshot showing identity settings persisted.
10. Waited on Step 2 for local AI service startup.
11. Attempted the visible exposed edge of the product-owned `Install local AI runtime` control because the sticky footer occluded the button body.
12. Clicked visible Step 2 `Next` to test whether setup could continue.
13. Captured screenshot, DB snapshot, process state, and runtime diagnostics after the Step 2 failure.

## Results

FAIL at Step 2 AI setup.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery auto-continues to Step 2: PASS.
- Identity settings persisted in app DB: PASS.
- Product-owned local AI runtime recovery: FAIL, install button was visually occluded by the footer and did not respond when clicking the exposed edge.
- Step 2 Next behavior: FAIL, clicking visible Next hid the app instead of advancing.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, compile/export, and here.now publish: NOT RUN because Step 2 blocked setup.

Observed Step 2 state:

- Step 2 showed `Couldn't reach the AI service`.
- The app correctly explained that Civic Desk can download and install its local AI runtime on a clean machine.
- The main install-runtime button was mostly hidden behind the sticky footer. Only the top edge of blue buttons was visible; clicking the visible exposed edge did not start installation or change UI state.
- Clicking the visible Step 2 `Next` control caused app content to disappear from the desktop.
- `civicnews.exe` remained running with a main window handle, but `MainWindowTitle` became empty.

## Evidence

- test-comms/reports/20260630-autocontinue-rerun-4e96d0a-visibility.md
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/cleanwipe-install-launch.log
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-02-step2-after-wait.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-03-after-click-install-runtime.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-04-step2-scrolled-runtime-buttons.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-05-after-click-visible-runtime-button-edge.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-06-after-step2-next.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/db-snapshot-step2-auto-continue.json
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/db-snapshot-after-step2-next-hide.json
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/community_profile.json
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/runtime-diagnostics.txt
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/environment.txt

Persisted identity settings at Step 2:

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

Process state after clicking Step 2 Next:

```text
ProcessName      : civicnews
Id               : 23596
MainWindowTitle  :
MainWindowHandle : 17368210
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Step 2 Next hides the app instead of advancing or blocking safely

Observed: On Step 2, with local AI service unavailable, clicking the visible Next button caused the app content to disappear from the desktop. The process remained alive with an empty `MainWindowTitle`.

Expected: Step 2 should either advance only after required AI setup succeeds, or keep the user on Step 2 with a visible product-owned recovery/error state.

Impact: The first-run setup cannot continue to the required Longmont E2E flow.

Repro:

1. Clean wipe Civic Desk app data and prior output.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-autocontinue-rerun-4e96d0a/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for Step 1 to auto-continue to Step 2.
5. Wait for Step 2 to report `Couldn't reach the AI service`.
6. Click the visible Step 2 Next button.
7. Observe app content disappears, no later step appears, and `civicnews.exe` remains alive.

### Major: Step 2 install-runtime controls are occluded by the sticky footer

Observed: The `Install local AI runtime` and adjacent product-owned recovery buttons were mostly hidden behind the sticky bottom footer. Clicking the visible exposed edge of the install button did not start installation or change UI state. Mouse wheel scrolling did not reveal the button body in this cleanroom run.

Expected: A clean-machine user should be able to clearly see and click the product-owned runtime install control.

Impact: The app gives the right recovery copy, but the primary recovery action is not usable at this viewport/window size. This blocks non-technical cleanroom setup before any source discovery or publication flow.

Repro:

1. Follow the cleanroom install steps above.
2. Wait for Step 2 AI service failure copy.
3. Observe the blue install-runtime button body hidden by the bottom footer.
4. Attempt to click the exposed edge; no visible response.

## Request For Coder

Please fix Step 2 so the local AI runtime install controls are fully visible/clickable above the footer, and so Step 2 Next cannot hide the app when the AI service is unavailable. The Step 1 auto-continue fix worked and persisted identity settings.
