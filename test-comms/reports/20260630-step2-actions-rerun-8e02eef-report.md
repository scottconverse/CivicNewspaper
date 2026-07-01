# Tester Report - step2 actions rerun 8e02eef

Date: 2026-07-01T05:38:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 8e02eef16b31fa74e77da97dc6520c762b8b67c2
Directive: test-comms/directives/20260630-step2-actions-rerun-8e02eef.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376296890368 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: not found / not running
- Models present: clean-wiped; no prior model state present

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260630-step2-actions-rerun-8e02eef.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260630-step2-actions-rerun-8e02eef\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews`, `ollama`, and Chrome if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama runtime/model state used by Civic Desk
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Confirmed Step 1 auto-continued to Step 2 without tester clicking Next.
9. Captured DB snapshot showing identity settings persisted.
10. Verified the Step 2 footer Next button was disabled while the local AI service was unavailable; did not click it.
11. Confirmed `Install local AI runtime`, `Check Initialization Status`, then later `Retry` and `Save diagnostics file` controls were fully visible above the footer.
12. Clicked `Install local AI runtime`, waited 20 seconds, captured screenshot/process state.
13. Clicked `Install local AI runtime` again from the explicit `Couldn't reach the AI service` state, waited 45 seconds, captured screenshot/process/runtime state.
14. Captured final DB snapshot and runtime diagnostics.

## Results

FAIL at the product-owned runtime install action gate.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery auto-continues to Step 2: PASS.
- Identity settings persisted in app DB: PASS.
- Step 2 footer Next disabled while local AI service unavailable: PASS.
- Step 2 local AI runtime install controls fully visible/clickable above footer: PASS for layout and visible click target.
- Product-owned runtime install invoked by button: FAIL.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, compile/export, and here.now publish: NOT RUN because directive says to stop at a failed gate.

Observed Step 2 state:

- Runtime actions were moved above the long explanatory text and are no longer covered by the footer.
- The footer Next button was visibly disabled while the AI service was unavailable.
- First click on `Install local AI runtime` did not create an `ollama` process or visible install progress.
- After the UI changed to `Couldn't reach the AI service`, a second click on `Install local AI runtime` and 45-second wait still did not create an `ollama` process, `.ollama` directory, or Ollama install directory.
- The app stayed visible on Step 2 with the same failure/recovery controls.

## Evidence

- test-comms/reports/20260630-step2-actions-rerun-8e02eef-visibility.md
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/cleanwipe-install-launch.log
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-02-after-click-install-runtime.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-03-after-second-install-click-wait45.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/db-snapshot-step2-auto-continue.json
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/db-snapshot-final-runtime-install-noop.json
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/community_profile.json
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/runtime-diagnostics.txt
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/environment.txt

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

Runtime path/process check after the second click:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk

No ollama process found.
ABSENT C:\Users\civic\.ollama
ABSENT C:\Users\civic\AppData\Local\Programs\Ollama
ABSENT C:\Users\civic\AppData\Local\Ollama
ABSENT C:\Program Files\Ollama
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Install local AI runtime button is visible but does not invoke runtime setup

Observed: On Step 2, the runtime install controls are now fully visible above the footer and the footer Next button remains disabled. Clicking `Install local AI runtime` twice did not start visible install progress, did not start an `ollama` process, and did not create `.ollama` or an Ollama install directory after a 45-second wait.

Expected: Clicking `Install local AI runtime` should invoke product-owned runtime installation and eventually allow setup to proceed to model download / Step 3.

Impact: Clean-machine users can now see the intended recovery action, but cannot complete AI setup, so the full Longmont E2E publication flow remains blocked before source discovery.

Repro:

1. Clean wipe Civic Desk app data, output folders, and Ollama runtime/model state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-step2-actions-rerun-8e02eef/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for Step 1 to auto-continue to Step 2.
5. Confirm disabled footer Next and visible runtime recovery controls.
6. Click `Install local AI runtime`.
7. Wait up to 45 seconds.
8. Observe no install progress, no Ollama process, and no Ollama runtime directory.

## Request For Coder

Please wire the visible `Install local AI runtime` action to product-owned runtime setup and expose progress/error output if setup cannot start. The Step 2 layout and disabled Next behavior are fixed in this build.
