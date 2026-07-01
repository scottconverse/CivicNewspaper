# Tester Report - window button rerun 9519547

Date: 2026-07-01T04:55:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 9519547e35be59ad2002af6759cf11097f4d25f1
Directive: test-comms/directives/20260630-window-button-rerun-9519547.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376472178688 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: not found / not running
- Models present: clean-wiped; no prior model state present

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260630-window-button-rerun-9519547.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260630-window-button-rerun-9519547\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews` / `ollama` if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama model state
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Captured a DB snapshot before pressing Step 1 Next.
9. Clicked the visible Step 1 `Next` control.
10. Captured screenshot, DB snapshot, process state, and runtime diagnostics after the click.

## Results

FAIL at required first gates.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery notice visible: PASS.
- Visible Next click: FAIL.
- Step 2 appears after pressing Next: FAIL.
- Identity settings persisted in app DB: FAIL.
- Full Longmont flow, AI setup, scan, draft, compile, ZIP/export, and here.now publish: NOT RUN because directive says to stop if first gates fail.

Observed after pressing the visible Next button:

- The app content disappeared from the desktop.
- No Step 2 UI appeared.
- `civicnews.exe` remained running.
- Windows still reported a main window handle, but `MainWindowTitle` became empty.
- The app DB still contained only `model.selected = phi4-mini:latest`; no publication/editor/city/state identity settings were present.

## Evidence

- test-comms/reports/20260630-window-button-rerun-9519547-visibility.md
- test-comms/evidence/20260630-window-button-rerun-9519547/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-window-button-rerun-9519547/screenshot-02-step1-before-next.png
- test-comms/evidence/20260630-window-button-rerun-9519547/screenshot-03-after-step1-next.png
- test-comms/evidence/20260630-window-button-rerun-9519547/screenshot-04-final-desktop-after-next-failure.png
- test-comms/evidence/20260630-window-button-rerun-9519547/db-snapshot-before-step1-next.json
- test-comms/evidence/20260630-window-button-rerun-9519547/db-snapshot-after-step1-next.json
- test-comms/evidence/20260630-window-button-rerun-9519547/cleanwipe-install-launch.log
- test-comms/evidence/20260630-window-button-rerun-9519547/runtime-diagnostics.txt
- test-comms/evidence/20260630-window-button-rerun-9519547/environment.txt

Key process state after the failure:

```text
ProcessName      : civicnews
Id               : 23148
MainWindowTitle  :
MainWindowHandle : 16580956
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

Key DB state after the failure:

```json
{
  "settings": [
    {
      "key": "model.selected",
      "value": "phi4-mini:latest"
    }
  ]
}
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Step 1 Next hides the app instead of advancing to Step 2

Observed: After normal installed launch, Step 1 rendered visibly with the Longmont no-input recovery notice and a visible Next control. Clicking Next caused the app window/content to disappear from the desktop. No Step 2 UI appeared. The process stayed alive with an empty `MainWindowTitle`.

Expected: Clicking Next from the recovered Longmont Step 1 should advance to Step 2 and keep the app visible.

Impact: The first-run workflow cannot continue past Step 1, so the required Longmont E2E publication flow cannot be run.

Repro:

1. Clean wipe Civic Desk app data and prior outputs.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-window-button-rerun-9519547/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for Step 1 no-input Longmont recovery.
5. Click the visible Next button.
6. Observe app content disappears, Step 2 does not appear, and `civicnews.exe` remains alive.

### Major: Identity settings are not persisted by the Step 1 transition

Observed: Before and after clicking Next, the DB `settings` table contained only `model.selected = phi4-mini:latest`. No publication name, editor name, city, or state values were persisted.

Expected: Identity settings should be persisted before leaving Step 1, per directive gate 8.

Impact: Even if Step 2 had rendered, the app would not have met the required persistence gate.

Repro: Same as blocker; inspect `%APPDATA%\com.scottconverse.civicdesk\civicdesk.db` after clicking Step 1 Next.

## Request For Coder

Please fix the Step 1 Next transition after no-input Longmont recovery so it keeps the installed app visible, advances to Step 2, and persists publication/editor/city/state identity settings before leaving Step 1.
