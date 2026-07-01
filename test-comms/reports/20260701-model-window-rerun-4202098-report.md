# Tester Report - model window rerun 4202098

Date: 2026-07-01T06:42:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 420209825d36f9ee7c9812a8c040ac5f46c9f492
Directive: test-comms/directives/20260701-model-window-rerun-4202098.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- Node: not found on PATH
- Rust: not found on PATH
- npm: not found on PATH
- Ollama installed/running before product run: clean-wiped; no manual Ollama install used
- Product-managed runtime during run: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`.
2. Confirmed it points to `test-comms/directives/20260701-model-window-rerun-4202098.md`.
3. Verified installer byte size and SHA256:
   - Path: `test-comms/artifacts/20260701-model-window-rerun-4202098/The Civic Desk_0.3.1_x64-setup.exe`
   - Expected/observed SHA256: `7C934848901FAD43DF0D5B88E59F4A62B958EE5BA0DBF740287DB3F6C413F481`
   - Expected/observed size: `5629802`
4. Wrote and pushed visibility report `test-comms/reports/20260701-model-window-rerun-4202098-visibility.md`.
5. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app install/app data/output paths.
   - Removed prior user `.ollama\models` state.
6. Installed the NSIS package silently with `/S`.
7. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` as the current normal user.
8. Waited 30 seconds and captured the visible-window screenshot.
9. Confirmed the app rendered a visible native desktop window without window-handle manipulation.
10. Confirmed recovered setup reached Step 2 and product-owned runtime auto-install started.
11. Captured runtime setup screenshots and process/model state at 10, 30, 60, and 120 seconds.
12. Confirmed product-managed `ollama.exe` was running.
13. Confirmed the app reached Step 3, `Download AI Model`.
14. Used ordinary keyboard navigation to reveal the Step 3 visible download control because mouse wheel scrolling did not move the page.
15. Clicked the visible product control labeled `Download phi4-mini:latest`.
16. Captured screenshots and model/runtime state at 10, 30, 60, and 120 seconds after the click.
17. Made one second visible click attempt on the same control and captured a 15-second follow-up.
18. Captured DB snapshot, process state, model list, model directory state, and runtime diagnostics.

## Results

FAIL at Step 3 model download action.

- Installer hash and size: PASS.
- Product clean wipe and silent install: PASS.
- Visible native app window after normal launch: PASS.
- Step 1 no-input Longmont recovery and auto-continue: PASS.
- Identity settings persisted in app DB: PASS.
- Step 2 product-owned runtime auto-install: PASS.
- Managed `ollama.exe` starts from Civic Desk app data: PASS.
- App remains visible through Step 3: PASS.
- Step 3 download control can be revealed with keyboard navigation: PARTIAL.
- Step 3 visible download action starts model download: FAIL.
- Model setup progress or useful failure message after clicking download: FAIL.
- Full Longmont source discovery, scan, drafting, editor workflow, export, publish, and output audits: NOT RUN because directive says to stop at failed gate.

Observed Step 3 failure:

- Step 3 showed `Download AI Model`.
- The visible control was labeled `Download phi4-mini:latest`, not `Start download`.
- Clicking `Download phi4-mini:latest` did not change visible app state.
- The app stayed visible and `civicnews.exe` stayed alive with title `The Civic Desk`.
- Product-managed `ollama.exe` stayed alive.
- `ollama list` stayed empty at 10, 30, 60, and 120 seconds after clicking.
- The model directory contained only empty `blobs` and `manifests` folders.
- A second visible click attempt also did not start model download.
- No visible progress or useful error appeared.

## Evidence

Evidence folder: `test-comms/evidence/20260701-model-window-rerun-4202098/`

Key evidence files:

- `cleanwipe-install-launch.log`
- `runtime-diagnostics.txt`
- `runtime-autoinstall-watch.txt`
- `model-download-watch.txt`
- `final-diagnostics.txt`
- `environment.txt`
- `community_profile.json`
- `appdata-roaming-top-level.json`
- `db-snapshot-after-model-click-stall.json`
- `screenshot-01-normal-launch-after-30s.png`
- `screenshot-runtime-auto-10s.png`
- `screenshot-runtime-auto-30s.png`
- `screenshot-runtime-auto-60s.png`
- `screenshot-runtime-auto-120s.png`
- `screenshot-step3-scrolled-for-start-download.png`
- `screenshot-step3-keyboard-navigation-attempt.png`
- `screenshot-step3-after-download-click.png`
- `screenshot-model-download-10s.png`
- `screenshot-model-download-30s.png`
- `screenshot-model-download-60s.png`
- `screenshot-model-download-120s.png`
- `screenshot-step3-second-click-15s.png`

Persisted identity settings after the failure:

```json
[
  {"key": "model.selected", "value": "phi4-mini:latest"},
  {"key": "identity.newsroom_name", "value": "My Local Publication"},
  {"key": "identity.editor_name", "value": "Publisher"},
  {"key": "identity.organization_type", "value": "single_person"},
  {"key": "identity.city", "value": "Longmont"},
  {"key": "identity.state", "value": "CO"}
]
```

Process state after the failure:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk
MainWindowHandle : 1181022
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe

ProcessName      : ollama
Path             : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe
```

Model state after the failure:

```text
NAME    ID    SIZE    MODIFIED
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Step 3 visible download action does not start the model download

Observed: Step 3 was visible and the app stayed visible. The visible control labeled `Download phi4-mini:latest` was clicked. The app did not show progress, did not show an error, did not change state, and no model appeared in `ollama list` after 10, 30, 60, or 120 seconds. A second visible click attempt also did not start model download.

Expected: Clicking the Step 3 primary download action should start the recommended model download or keep the user visibly on Step 3 with a useful progress/error state.

Impact: The clean-machine setup flow now reaches Step 3, but cannot download the recommended model through the product, so the full Longmont E2E flow remains blocked before source discovery.

Repro:

1. Clean wipe Civic Desk app data, install, output folders, and Ollama runtime/model state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260701-model-window-rerun-4202098/`.
3. Launch installed `civicnews.exe` normally.
4. Wait for Step 1 recovery, Step 2 runtime auto-install, and Step 3.
5. Reveal the Step 3 download action if needed.
6. Click `Download phi4-mini:latest`.
7. Observe no progress/error and no model in `ollama list`.

### Major: Step 3 primary control is not naturally visible at the tester viewport

Observed: At the 1280x720 desktop viewport, Step 3 showed the model card but the download control was below the visible portion of the window. Mouse wheel scrolling over the app did not reveal it. Keyboard navigation/PageDown eventually moved the content enough to expose `Download phi4-mini:latest`.

Expected: The Step 3 primary action should be visible and naturally reachable in the normal installed app window.

Impact: Even before the no-op click failure, a clean-machine user can get stuck because the download action is not obvious or reachable via ordinary mouse scroll.

## Request For Coder

Please fix Step 3 so the primary download action is naturally visible/reachable and clicking it starts product-owned model download with visible progress or a useful error. Runtime auto-install and app visibility are now passing, but model download does not start.
