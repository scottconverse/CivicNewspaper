# Tester Report - runtime auto rerun 40aa58f

Date: 2026-07-01T05:56:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 40aa58f4fc7c7cf05fefe709e40dba8bb4d376cc
Directive: test-comms/directives/20260701-runtime-auto-rerun-40aa58f.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376259047424 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: product-managed runtime started from app data; no global Ollama on PATH
- Models present: none after Step 3 failure

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260701-runtime-auto-rerun-40aa58f.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260701-runtime-auto-rerun-40aa58f\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews`, `ollama`, and Chrome if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama runtime/model state used by Civic Desk
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Confirmed recovered setup auto-continued to Step 2 and persisted identity settings.
9. Confirmed Step 2 displayed no-input recovery copy: `The setup screen is not receiving input events, so The Civic Desk is installing the local AI runtime automatically.`
10. Waited the required 90 seconds for product-owned runtime auto-install.
11. Captured screenshots at 30, 60, and 90 seconds, plus process/runtime state.
12. Confirmed product-managed `ollama.exe` started from `%APPDATA%\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`.
13. Confirmed the app advanced to Step 3, `Download AI Model`.
14. Clicked visible Step 3 Next.
15. Captured screenshot, DB snapshot, process list, model list, runtime files, and environment diagnostics after Step 3 Next hid the app.

## Results

FAIL at Step 3 model setup.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery auto-continues to Step 2: PASS.
- Identity settings persisted in app DB: PASS.
- Step 2 footer Next disabled while local AI service unavailable: PASS.
- Runtime setup controls visible above footer: PASS.
- Product-owned runtime auto-install starts without tester help: PASS.
- Runtime setup gives visible progress and starts managed `ollama.exe`: PASS.
- App advances to Step 3 after runtime setup: PASS.
- Step 3 model download starts or advances safely: FAIL.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, compile/export, and here.now publish: NOT RUN because directive says to stop at failed gate.

Observed Step 3 failure:

- Step 3 displayed `Download AI Model` and `AI Model: phi4-mini:latest (Recommended)`.
- Clicking visible Step 3 Next hid the app from the desktop.
- `civicnews.exe` stayed alive with a main window handle but an empty `MainWindowTitle`.
- Product-managed `ollama.exe` stayed alive.
- `ollama list` returned no models.
- No visible model download progress appeared.

## Evidence

- test-comms/reports/20260701-runtime-auto-rerun-40aa58f-visibility.md
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/cleanwipe-install-launch.log
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-30s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-60s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-90s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-02-after-step3-next.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/runtime-autoinstall-watch.txt
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/db-snapshot-step2-auto-install-start.json
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/db-snapshot-after-step3-next-hide.json
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/community_profile.json
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/runtime-diagnostics.txt
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/environment.txt

Runtime process evidence:

```text
ProcessName : ollama
Path        : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe
```

Process state after Step 3 Next:

```text
ProcessName      : civicnews
MainWindowTitle  :
MainWindowHandle : 1901542

ProcessName      : ollama
Path             : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe
```

Model state after Step 3 Next:

```text
NAME    ID    SIZE    MODIFIED
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Step 3 Next hides the app instead of starting model download

Observed: Product-owned runtime auto-install succeeded and Step 3 appeared. Clicking visible Step 3 Next hid the app from the desktop. The app process remained alive with an empty main window title, managed `ollama.exe` remained alive, and no model appeared in `ollama list`.

Expected: Step 3 Next should start/download the recommended model or keep the user visibly on Step 3 with progress or a useful failure message.

Impact: Clean-machine setup is now past runtime installation but still cannot reach model setup, so the full Longmont E2E publication flow remains blocked before source discovery.

Repro:

1. Clean wipe Civic Desk app data, output folders, and Ollama runtime/model state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260701-runtime-auto-rerun-40aa58f/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for no-input recovery to auto-install the local runtime and advance to Step 3.
5. Click visible Step 3 Next.
6. Observe app disappears from desktop, `civicnews.exe` remains alive, managed `ollama.exe` remains alive, and no model download starts.

## Request For Coder

Please fix Step 3 so the visible Next action starts the model download or keeps the app visible with clear progress/error recovery. Runtime auto-install is now working in this build.
