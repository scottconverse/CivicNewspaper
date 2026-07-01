# Tester Report - model window rerun 4202098

Date: 2026-07-01T06:39:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 420209825d36f9ee7c9812a8c040ac5f46c9f492
Directive: test-comms/directives/20260701-model-window-rerun-4202098.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 374298365952 bytes on C:
- Ollama installed/running: product-managed runtime running from app data; no global Ollama required
- Models present: none after model download failure

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260701-model-window-rerun-4202098.md`.
2. Verified installer byte size and SHA256.
3. Performed product clean wipe for Civic Desk install, app data, output folders, and Civic Desk managed Ollama state.
4. Installed the NSIS package silently.
5. Launched installed `civicnews.exe` normally, without native window manipulation.
6. Confirmed prior setup gates: visible native launch, recovered Step 1/Step 2 path, identity persistence, visible runtime controls, disabled Step 2 Next, and product-owned runtime auto-install.
7. Confirmed Step 3 appeared with model download copy and a visible `Download phi4-mini:latest` control.
8. Attempted keyboard navigation/click path for Step 3 download.
9. Watched for model download progress for 60+ seconds and captured process state, screenshots, `ollama list`, model tree, and diagnostics.

## Results

FAIL at Step 3 model download.

- Installer hash and size: PASS.
- Visible native app launch: PASS.
- Recovered Step 1/Step 2 setup and identity persistence: PASS.
- Product-owned runtime auto-install: PASS.
- App remains visible during Step 3 attempt: PASS.
- Step 3 model download starts: FAIL.
- Full Longmont flow, source discovery, scan, drafting, editor workflow, export, and publish: NOT RUN because directive says to stop at failed gate.

Observed:

- Step 3 remains visible after the model download action path.
- Managed `ollama.exe` stays running from `%APPDATA%\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`.
- `ollama list` remains empty.
- `.ollama\models` contains only empty `blobs` and `manifests` directories.
- No visible model progress, model files, or useful failure message appeared.

## Evidence

- test-comms/reports/20260701-model-window-rerun-4202098-visibility.md
- test-comms/evidence/20260701-model-window-rerun-4202098/cleanwipe-install-launch.log
- test-comms/evidence/20260701-model-window-rerun-4202098/runtime-autoinstall-watch.txt
- test-comms/evidence/20260701-model-window-rerun-4202098/model-download-watch.txt
- test-comms/evidence/20260701-model-window-rerun-4202098/runtime-diagnostics-final.txt
- test-comms/evidence/20260701-model-window-rerun-4202098/db-snapshot-final-model-download-noop.json
- test-comms/evidence/20260701-model-window-rerun-4202098/environment.txt
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-runtime-auto-10s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-runtime-auto-30s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-runtime-auto-60s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-runtime-auto-120s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-step3-scrolled-for-start-download.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-step3-keyboard-navigation-attempt.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-step3-after-download-click.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-model-download-10s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-model-download-30s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-model-download-60s.png
- test-comms/evidence/20260701-model-window-rerun-4202098/screenshot-final-state.png

Key diagnostic excerpt:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk

ProcessName      : ollama
Path             : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe

Ollama list:
NAME    ID    SIZE    MODIFIED

Model tree:
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

### Blocker: Step 3 model download action leaves the app visible but does not start model pull

Observed: The app reaches Step 3 and remains visible, but the model download action does not create model files, does not change `ollama list`, and does not show progress or an actionable error.

Expected: Step 3 should invoke product-owned model pull for `phi4-mini:latest` and show progress or a useful failure state.

Impact: Clean-machine setup cannot reach source discovery or publication because no local model is installed.

Repro:

1. Clean wipe Civic Desk app data, output folders, and managed Ollama state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260701-model-window-rerun-4202098/`.
3. Launch installed app normally.
4. Wait for recovered setup to auto-install the managed runtime and reach Step 3.
5. Trigger the visible model download control.
6. Observe no model pull progress and empty `ollama list`.

## Request For Coder

Please wire the Step 3 model download action to the product-owned model pull and expose progress or a useful error message. The app no longer disappears during this gate, but the model download does not start.
