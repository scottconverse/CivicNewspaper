# Tester Report - Automatic Longmont Intake Rerun 4658500

Date: 2026-06-28 22:22 UTC
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 4658500df747720e0c1d3d867faf25377b585ca7 (Auto-run Longmont source intake after recovered setup)
Directive: test-comms/directives/20260628-rerun-auto-longmont-intake-after-4658500.md

Status: PARTIAL PASS - automatic Longmont intake and Daily Scan completed; editor/publish E2E still blocked by UI input/visibility.

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
2. Verified `origin/stable-readiness-local-gates` at `4658500df747720e0c1d3d867faf25377b585ca7`.
3. Verified installer hashes:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `1F10EF20E77CBB7AD168191E2CDDA8E3154CAD7110728FD48AA3E34EBA2CBF16`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `DE2082BB3C79CE46571F1623F41A159AEE2D0546D009F013DCF5EF928264B9C1`
4. Stopped stale `civicnews`, `ollama`, `msiexec`, and helper terminal processes.
5. Removed only CivicNewspaper/Ollama cleanroom state under the tester user profile, including app data, local app install folder, `.ollama`, and local Ollama app data.
6. Installed with:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-auto-longmont-intake-rerun-4658500\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

7. Launched:

```powershell
$env:LOCALAPPDATA\The Civic Desk\civicnews.exe
```

8. Set the window to 1280x720.
9. Let first-run setup proceed through the app's own recovery paths. I did not manually install Ollama, manually pull a model, repair PATH, use devtools, manually add sources, or edit the database.
10. Observed the app through 10 minutes of screenshots/process/listener/model checks.
11. Read the product-created SQLite database in read-only mode using bundled Python to infer source/evidence/lead counts not visible in the 1280x720 app window.

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
| Automatic route to Daily Scan | PASS |
| Automatic Longmont starter source import | PASS - 6 sources in product DB |
| Automatic fetch/ingest | PASS - 27 evidence items and 104 civic observations in product DB |
| Automatic first Daily Scan | PASS - 1 completed scan run |
| Leads/results produced | PASS - 19 `leads` and 11 `daily_scan_leads` in product DB |
| Drafts/stories produced | NOT REACHED - 0 drafts |
| Writer/editor/publish/export/soak | BLOCKED |

The app remained responsive as a Windows process throughout the run. Ollama stayed reachable at `127.0.0.1:11434`, and `/api/tags` returned `qwen2.5:7b`.

## Evidence

Artifact folder:

`test-comms/artifacts/20260628-auto-longmont-intake-rerun-4658500/`

Screenshots/logs:

- `monitor-000s.png` - setup recovery in progress; Ollama not reachable yet.
- `monitor-030s.png` and `monitor-060s.png` - Ollama reachable with empty model list.
- `monitor-090s.png` - model available.
- `monitor-120s.png` through `monitor-600s.png` - app has routed to `Daily Scan`.
- `final-app-state.png` - clean final app screenshot on `Daily Scan`.
- `monitor-log.json` - timed process/listener/model observations.
- `final-state-log.json` - final process/listener/model state.
- `db-readonly-counts.json` - read-only sanitized database table counts and sample rows.

Read-only product DB counts:

| Item | Count |
| --- | ---: |
| Sources | 6 |
| Evidence items | 27 |
| Civic observations | 104 |
| Civic observation entities | 130 |
| Leads | 19 |
| Daily scan leads | 11 |
| Daily scan runs | 1 completed |
| Dark signals | 1 |
| Drafts | 0 |
| Verification tasks | 3 |
| Publish runs | 0 |
| Published posts | 0 |

The completed scan run was `id=1`, started at `2026-06-28T22:26:30.607940600+00:00` and completed at `2026-06-28T22:27:51.729686900+00:00`.

Sample produced Daily Scan leads include:

- `City Council and Planning & Zoning Commission Meetings Now Livestreamed`
- `Technical Issues with Online Permitting Portal`
- `Vision Zero Projects`
- `Youth Center Programs`
- `City Council Meetings Schedule`

Sample imported sources include:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: cleanroom cannot continue from produced leads into editor/publish workflow

Observed: `4658500` successfully advances beyond the prior Sources-control blocker by auto-importing sources, fetching evidence, producing leads, completing one Daily Scan, and routing the UI to `Daily Scan`. However, the 1280x720 packaged app view exposes only the top of the Daily Scan page, and previous click/keyboard attempts in this recovered WebView state have not reliably delivered input. No drafts were produced automatically, and the run could not proceed into lead opening, drafting, approval, export, here.now publish, or soak without direct UI control.

Expected: After automatic intake produces Daily Scan leads, the app should allow the tester to open results and continue writer/editor/publish workflows through the packaged UI.

Impact: The core local intake/scan loop is now proven, but the full release gauntlet remains blocked before drafts, publication export, here.now publish, and 12-hour soak.

Repro:

1. Clean reset CivicNewspaper/Ollama state.
2. Install `The Civic Desk_0.2.8_x64-setup.exe` from the `4658500` artifact folder.
3. Launch `civicnews.exe` at 1280x720.
4. Wait for recovered setup and automatic intake.
5. Observe UI routed to `Daily Scan`.
6. Inspect product DB read-only: sources/evidence/leads/scan exist, but drafts/publish runs are zero.
7. Continue workflow is blocked because usable UI control into lead/editor/publish flow is still not available.

## Request For Coder

`4658500` is a meaningful step forward: automatic Longmont source import, evidence ingest, and the first Daily Scan completed in the cleanroom packaged app. Please add a reliable recovered-path continuation from produced Daily Scan leads into draft/editor workflow, or restore packaged UI input/scroll/click delivery enough for the tester to open leads and proceed to export/publish/soak.
