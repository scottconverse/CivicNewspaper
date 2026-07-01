# Tester Report - step3 model rerun c37c756

Date: 2026-07-01T06:11:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: c37c756524a130cd6a415603e1cc12fec019e05e
Directive: test-comms/directives/20260701-step3-model-rerun-c37c756.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 374355439616 bytes on C:
- Node: not found
- Rust: not found
- npm: not found
- Ollama installed/running: product-managed runtime started from app data; no global Ollama on PATH
- Models present: none after Step 3 failure

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260701-step3-model-rerun-c37c756.md`.
2. Verified installer byte size and SHA256:
   - `Get-Item -LiteralPath test-comms\artifacts\20260701-step3-model-rerun-c37c756\The Civic Desk_0.3.1_x64-setup.exe`
   - `Get-FileHash -Algorithm SHA256 -LiteralPath ...`
3. Performed product clean wipe:
   - stopped `civicnews`, `ollama`, and Chrome if present
   - ran installed uninstaller when present
   - removed Civic Desk app data, local app data, prior CivicNews output paths, and prior Ollama runtime/model state used by Civic Desk
4. Installed the NSIS package silently with `/S`.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
6. Waited 30 seconds and captured the initial desktop screenshot.
7. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
8. Confirmed recovered setup auto-continued to Step 2 and started product-owned runtime auto-install.
9. Waited 90 seconds, capturing runtime process/file evidence and screenshots.
10. Confirmed product-managed `ollama.exe` started from `%APPDATA%\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`.
11. Confirmed the app reached Step 3, `Download AI Model`.
12. Clicked the primary footer action labeled `Start download`; did not click `Skip for now`.
13. Watched for 120 seconds, capturing screenshots, process state, `ollama list`, and model directory state at 10, 30, 60, and 120 seconds.
14. Captured final DB snapshot, runtime diagnostics, model list, and environment diagnostics.

## Results

FAIL at Step 3 model download.

- Installer hash and size: PASS.
- Visible native app window after normal launch: PASS.
- Recovered Step 1/Step 2 setup gates: PASS.
- Product-owned runtime auto-install starts without tester help: PASS.
- Runtime setup starts managed `ollama.exe`: PASS.
- App advances to Step 3 after runtime setup: PASS.
- Step 3 primary action is labeled `Start download`: PASS.
- Step 3 `Start download` keeps app visible: FAIL.
- Step 3 `Start download` starts model download: FAIL.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, compile/export, and here.now publish: NOT RUN because directive says to stop at failed gate.

Observed Step 3 failure:

- Step 3 displayed `Download AI Model` and the footer action `Start download`.
- Clicking `Start download` immediately hid the app from the desktop.
- `civicnews.exe` stayed alive with a main window handle but empty `MainWindowTitle`.
- Product-managed `ollama.exe` stayed alive.
- `ollama list` returned no models at 10, 30, 60, and 120 seconds.
- The `.ollama\models` tree only contained empty `blobs` and `manifests` directories; no model data appeared.
- No visible progress or failure message was shown because the app disappeared.

## Evidence

- test-comms/reports/20260701-step3-model-rerun-c37c756-visibility.md
- test-comms/evidence/20260701-step3-model-rerun-c37c756/cleanwipe-install-launch.log
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-30s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-60s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-90s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-model-download-10s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-model-download-30s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-model-download-60s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-model-download-120s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/runtime-autoinstall-watch.txt
- test-comms/evidence/20260701-step3-model-rerun-c37c756/model-download-watch.txt
- test-comms/evidence/20260701-step3-model-rerun-c37c756/db-snapshot-after-runtime-autoinstall.json
- test-comms/evidence/20260701-step3-model-rerun-c37c756/db-snapshot-after-step3-start-download-hide.json
- test-comms/evidence/20260701-step3-model-rerun-c37c756/community_profile.json
- test-comms/evidence/20260701-step3-model-rerun-c37c756/runtime-diagnostics.txt
- test-comms/evidence/20260701-step3-model-rerun-c37c756/environment.txt

Runtime process evidence:

```text
ProcessName : ollama
Path        : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe
```

Model state after 120 seconds:

```text
NAME    ID    SIZE    MODIFIED

C:\Users\civic\.ollama\models\blobs
C:\Users\civic\.ollama\models\manifests
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Step 3 Start download hides the app and does not start model download

Observed: Clicking the primary Step 3 footer action labeled `Start download` immediately hid the app from the desktop. The app process and managed Ollama runtime stayed alive, but `ollama list` remained empty and no model files appeared after 120 seconds.

Expected: `Start download` should keep the app visible and show model download progress or a useful failure message while product-owned model setup runs.

Impact: Clean-machine setup reaches Step 3 now, but cannot download the model or continue to source discovery and publication.

Repro:

1. Clean wipe Civic Desk app data, output folders, and Ollama runtime/model state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260701-step3-model-rerun-c37c756/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for recovered setup to auto-install runtime and advance to Step 3.
5. Click the primary footer action labeled `Start download`.
6. Observe app disappears from the desktop and no model download starts after 120 seconds.

## Request For Coder

Please fix Step 3 `Start download` so it keeps the app visible and actually invokes product-owned `pull_ollama_model` progress/error handling. The runtime auto-install path still works in this build.
