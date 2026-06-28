# Tester Report - Recovered Source Handoff Rerun aa0a1e4

Date: 2026-06-28 22:07 UTC
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: aa0a1e4f2f94c4f84525564ab14488bef1d65fb9 (Route recovered setup to source intake)
Directive: test-comms/directives/20260628-rerun-recovered-source-handoff-after-aa0a1e4.md

Status: FAIL - recovered setup now opens Sources, but source workflow controls still do not respond.

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
2. Verified `origin/stable-readiness-local-gates` at `aa0a1e4f2f94c4f84525564ab14488bef1d65fb9`.
3. Verified installer hashes:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `240A726E677B21CDE3729B618911989E14FE2E84417C5213AFF7E06AE287FA66`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `803D2EB3DB24D446E1B0DC01DDF4D95BB1491C073A8AF08A5349CBBCA8920FC4`
4. Stopped stale `civicnews`, `ollama`, `msiexec`, and helper terminal processes.
5. Removed only CivicNewspaper/Ollama cleanroom state under the tester user profile, including app data, local app install folder, `.ollama`, and local Ollama app data.
6. Installed with:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-recovered-source-handoff-rerun-aa0a1e4\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

7. Launched:

```powershell
$env:LOCALAPPDATA\The Civic Desk\civicnews.exe
```

8. Set the window to 1280x720.
9. Let first-run setup proceed through the app's own recovery paths. I did not manually install Ollama, manually pull a model, repair PATH, use devtools, or use a helper terminal.
10. Verified the recovered setup handoff opened the main app on `Sources`.
11. Attempted source workflow controls:
    - Clicked the visible `Discover for city` control.
    - Pressed `Enter` after the Discover attempt.
    - Clicked left and right visible hit points inside the Discover control.
    - Attempted keyboard traversal with `Tab` and `Enter`.

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
| App-managed model availability | PASS - `/api/tags` listed `qwen2.5:7b` by 90 seconds |
| First-run recovery reaches main app | PASS |
| First workspace screen is Sources | PASS |
| Source workflow controls respond | FAIL |
| Continue Longmont source intake/E2E/publish/soak | BLOCKED |

The app remained responsive as a Windows process throughout the run. Ollama stayed reachable at `127.0.0.1:11434`, and `/api/tags` returned `qwen2.5:7b`.

## Evidence

Artifact folder:

`test-comms/artifacts/20260628-recovered-source-handoff-rerun-aa0a1e4/`

Screenshots/logs:

- `monitor-000s.png` - setup recovery in progress; Ollama not reachable yet.
- `monitor-030s.png` and `monitor-060s.png` - Ollama reachable with empty model list.
- `monitor-090s.png` - `/api/tags` lists `qwen2.5:7b`.
- `monitor-120s.png`, `monitor-180s.png`, `monitor-240s.png` - main app opens on `Sources`.
- `01-sources-handoff.png` - first Sources screen after recovered setup.
- `02-click-discover-for-city.png` - after visible Discover control click; still unchanged.
- `03-enter-after-discover.png` - after Enter; still unchanged.
- `04-click-discover-left.png` - alternate Discover hit point; still unchanged.
- `05-click-discover-right.png` - alternate Discover hit point; still unchanged.
- `07-tab-enter-sources.png` - after keyboard traversal/activation; still unchanged.
- `monitor-log.json` - timed process/listener/model observations.
- `final-state-log.json` - final process/listener/model state.

Note: `06-wheel-down-sources.png` was captured after a wheel test command that failed before delivery because of a PowerShell argument conversion error. I am not counting that as product behavior.

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

### Blocker: source workflow controls remain unresponsive after successful recovered handoff

Observed: The packaged Windows app completes setup and now opens the `Sources` page automatically, which fixes the previous stranding on `Story Queue`. However, the visible `Discover for city` control does not respond to normal clicks, alternate hit-point clicks, `Enter`, or keyboard traversal/activation. The screen remains unchanged.

Expected: After recovered setup opens `Sources`, the tester should be able to use Discover for city, Bulk import, Add source, or review/import controls to begin Longmont source intake.

Impact: Blocks all remaining release validation after the improved setup handoff: Longmont source intake/discovery, lead generation, draft generation, editor workflows, export, here.now publish, and the 12-hour soak.

Repro:

1. Clean reset CivicNewspaper/Ollama state.
2. Install `The Civic Desk_0.2.8_x64-setup.exe` from the `aa0a1e4` artifact folder.
3. Launch `civicnews.exe` at 1280x720.
4. Wait for recovered setup to enter the main app.
5. Observe that the first workspace screen is `Sources`.
6. Try the visible `Discover for city` control with clicks and keyboard activation.
7. Observe no workflow opens and the screen remains unchanged.

## Request For Coder

`aa0a1e4` successfully hands recovered setup to Sources. Please continue investigating packaged Windows WebView input/control delivery on the Sources page so the cleanroom tester can actually start source intake from that recovered state.
