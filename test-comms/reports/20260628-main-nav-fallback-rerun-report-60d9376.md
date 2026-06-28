# Tester Report - Main Navigation Fallback Rerun 60d9376

Date: 2026-06-28 21:52 UTC
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 60d9376a5d81be7111049ebdc13c3f967bfd63c6 (Add resilient main navigation fallbacks)
Directive: test-comms/directives/20260628-rerun-main-nav-fallback-after-60d9376.md

Status: FAIL - setup completes, but main-app navigation still does not change routes.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.8 GB on C:
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before launch: not running and no listener after clean reset
- Models present before launch: none after clean reset

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, `test-comms/ACTIVE_DIRECTIVE.md`, and the active directive.
2. Verified `origin/stable-readiness-local-gates` at `60d9376a5d81be7111049ebdc13c3f967bfd63c6`.
3. Verified installer hashes:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `B89C33C70A5BF5789AF38158809833F5CE8BDF35E5A8E5E0E6DF33B24F159AD5`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `8C5B7BC4E7AC602ACD04870E2B73B18D148E59C4D06D781CC6D7BF4607A76260`
4. Stopped stale `civicnews`, `ollama`, `msiexec`, and helper terminal processes.
5. Removed only CivicNewspaper/Ollama cleanroom state under the tester user profile, including app data, local app install folder, `.ollama`, and local Ollama app data.
6. Installed with:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-main-nav-fallback-rerun-60d9376\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

7. Launched:

```powershell
$env:LOCALAPPDATA\The Civic Desk\civicnews.exe
```

8. Set the window to 1280x720.
9. Let first-run setup proceed through the app's own recovery paths. I did not manually install Ollama, manually pull a model, repair PATH, use devtools, or use a helper terminal.
10. After the main app opened on `Story Queue`, attempted navigation:
    - Normal click on `Daily Scan`.
    - `Alt+2`.
    - `Ctrl+2`.
    - `Alt+6`.
    - `Ctrl+6`.
    - `Alt+8`.
    - Alternate click on the `Daily Scan` icon.
    - Alternate click on the `Daily Scan` label/right side.
    - Direct low-level Windows key events for `Alt+2`, `Ctrl+2`, `Alt+6`, `Ctrl+6`, and `Alt+8`.

## Results

| Gate | Result |
| --- | --- |
| Pull/reread active directive | PASS |
| Verify product branch/commit | PASS |
| Verify installer hashes | PASS |
| Clean reset | PASS |
| Install packaged app | PASS |
| Launch real Tauri desktop app | PASS |
| App backend listener on 127.0.0.1:12053 | PASS |
| Missing Ollama behavior at launch | PASS - Ollama was absent initially |
| App starts its own Ollama runtime | PASS |
| App-managed model availability | PASS - `/api/tags` listed `qwen2.5:7b` by 60 seconds |
| First-run recovery reaches main app | PASS |
| Normal click navigation from Story Queue to Daily Scan | FAIL |
| Alt/Ctrl-number navigation fallbacks | FAIL |
| Continue Longmont source intake/E2E/publish/soak | BLOCKED |

The app remained responsive as a Windows process throughout the run. Ollama stayed reachable at `127.0.0.1:11434`, and `/api/tags` returned `qwen2.5:7b`.

## Evidence

Artifact folder:

`test-comms/artifacts/20260628-main-nav-fallback-rerun-60d9376/`

Screenshots/logs:

- `01-after-launch.png` - setup recovery reached AI Service Setup; app backend listening, Ollama not yet reachable.
- `monitor-030s.png` - Ollama reachable with empty model list.
- `monitor-060s.png` - `/api/tags` lists `qwen2.5:7b`.
- `monitor-090s.png` through `monitor-240s.png` - main app visible on `Story Queue`.
- `07-click-daily-scan.png` - after normal Daily Scan click; still on `Story Queue`.
- `08-alt-2-daily-scan.png` - after `Alt+2`; still on `Story Queue`.
- `09-ctrl-2-daily-scan.png` - after `Ctrl+2`; still on `Story Queue`.
- `10-alt-6-sources.png` - after `Alt+6`; still on `Story Queue`.
- `11-ctrl-6-sources.png` - after `Ctrl+6`; still on `Story Queue`.
- `12-alt-8-publishing.png` - after `Alt+8`; still on `Story Queue`.
- `13-click-daily-scan-icon.png` - alternate click on Daily Scan icon; still on `Story Queue`.
- `14-click-daily-scan-label-right.png` - alternate click on Daily Scan label/right side; still on `Story Queue`.
- `15-keybd-alt-2.png` through `19-keybd-alt-8.png` - low-level Windows key-event fallbacks; still on `Story Queue`.
- `monitor-log.json` - timed process/listener/model observations.
- `final-state-log.json` - final process/listener/model state.

Final process/listener state:

- `civicnews`: running, title `The Civic Desk`, responding.
- `ollama`: running.
- Backend listener: `127.0.0.1:12053`.
- Ollama listener: `127.0.0.1:11434`.
- `/api/tags`: HTTP 200, listed `qwen2.5:7b`.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: main-app route navigation still does not work after setup recovery

Observed: The packaged Windows app completes setup and opens the main application on `Story Queue`, but normal clicks and the new Alt/Ctrl-number fallbacks do not change routes. The selected nav item and page title remain `Story Queue` after every attempt.

Expected: At least one of normal click, pointer/mouse fallback, or Alt/Ctrl-number shortcut should navigate to `Daily Scan`, `Sources`, or `Publishing` so the tester can continue source intake and publication workflow.

Impact: Blocks all remaining release validation after setup: Longmont source intake/discovery, lead generation, draft generation, editor workflows, export, here.now publish, and the 12-hour soak.

Repro:

1. Clean reset CivicNewspaper/Ollama state.
2. Install `The Civic Desk_0.2.8_x64-setup.exe` from the `60d9376` artifact folder.
3. Launch `civicnews.exe` at 1280x720.
4. Wait for recovery setup to enter the main app.
5. Try Daily Scan click, Alt+2, Ctrl+2, Alt+6, Ctrl+6, and Alt+8.
6. Observe the app remains on `Story Queue`.

## Request For Coder

Please continue investigating packaged Windows WebView input/navigation delivery after recovered setup. `60d9376` confirms the setup/runtime/model path still works, but neither direct nav clicks nor keyboard shortcut fallbacks are reaching route-changing behavior in the main application.
