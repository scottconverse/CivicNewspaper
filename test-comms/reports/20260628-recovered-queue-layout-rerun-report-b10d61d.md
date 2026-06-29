# Tester Report - Recovered Queue Layout Rerun b10d61d

Date: 2026-06-29 00:15 UTC
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: b10d61df0f50582e34343620b0a4ecfefe5a02b8 (Fix cleanroom recovered queue layout)
Directive: test-comms/directives/20260628-rerun-recovered-queue-layout-b10d61d.md

Status: PARTIAL PASS - recovered layout reaches Story Queue and lead cards, but draft generation is blocked by clipped/unreachable draft controls.

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
2. Verified `origin/stable-readiness-local-gates` at `b10d61df0f50582e34343620b0a4ecfefe5a02b8`.
3. Verified installer hashes from `test-comms/artifacts/20260628-recovered-queue-layout-b10d61d/`:
   - `The Civic Desk_0.2.8_x64-setup.exe`: `9EE583BAEBE1C93F76A8AC683C37165A6F9F9A0BE53DF44F14DEFDCC9C9AF3A9`
   - `The Civic Desk_0.2.8_x64_en-US.msi`: `AC846CD62DC51392948AEA6F93C3C7417B54A8B011370DA112EE6AD1FC18265B`
4. Stopped stale `civicnews`, `ollama`, `msiexec`, and helper terminal processes.
5. Removed only CivicNewspaper/Ollama cleanroom state under the tester user profile, including app data, local app install folder, `.ollama`, and local Ollama app data.
6. Installed with:

```powershell
Start-Process -FilePath "test-comms\artifacts\20260628-recovered-queue-layout-b10d61d\The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait
```

7. Launched:

```powershell
$env:LOCALAPPDATA\The Civic Desk\civicnews.exe
```

8. Set the window to 1280x720.
9. Let first-run setup proceed through the app's own recovery paths. I did not manually install Ollama, manually pull a model, repair PATH, use devtools, manually add sources, or edit the database.
10. Observed the app through 10 minutes of screenshots/process/listener/model checks.
11. Used visible UI only to scroll Story Queue, reveal lead cards, click a visible Draft action, and open the draft form for one produced Longmont lead.
12. Attempted to reach/generate the draft from the draft form. The primary action at the bottom edge was clipped and not safely reachable at 1280x720.
13. Read the product-created SQLite database in read-only mode using bundled Python to record source/evidence/lead/draft counts.

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
| Automatic source intake and Daily Scan | PASS |
| Recovered path lands on Story Queue | PASS |
| Story Queue content visible/reachable at 1280x720 | PASS - after scroll/PageDown |
| Open at least five leads | FAIL - stopped at first draft-generation blocker |
| Open one produced Longmont lead | PASS |
| Generate drafts through app UI | FAIL |
| Writer/editor controls | NOT REACHED |
| Export/publish/soak | NOT REACHED |

The app remained responsive as a Windows process throughout the run. Ollama stayed reachable at `127.0.0.1:11434`, and `/api/tags` returned `qwen2.5:7b`.

## Evidence

Artifact folder:

`test-comms/artifacts/20260628-recovered-queue-layout-rerun-b10d61d/`

Screenshots/logs:

- `monitor-000s.png` through `monitor-600s.png` - setup/model/intake progression, ending on compact Story Queue layout.
- `01-story-queue-top.png` - Story Queue selected after recovered path.
- `02-story-queue-wheel-down.png` - Story Queue summary counts visible: 20 new leads, 0 in drafting, 9 high priority, 6 sources.
- `03-story-queue-pagedown.png` - lead cards and filter/sort controls visible.
- `04-story-queue-end.png` - lead cards with visible `Draft` button.
- `05-click-story-queue-nav.png` - draft form opened for a produced lead.
- `06-draft-form-start.png` - draft form showing lead and article format.
- `07-draft-form-scrolled.png` - attempted scroll did not expose the full primary action.
- `08-after-generate-click-low.png` and `09-draft-form-refocused.png` - draft form still clipped at the bottom; no draft created.
- `monitor-log.json` - timed process/listener/model observations.
- `final-state-log.json` - final process/listener/model state.
- `db-readonly-counts.json` - read-only sanitized database table counts and sample rows.

I intentionally deleted screenshots that captured a Windows OS overlay after a clipped bottom-edge click escaped the app.

Read-only product DB counts:

| Item | Count |
| --- | ---: |
| Sources | 6 |
| Evidence items | 27 |
| Civic observations | 104 |
| Civic observation entities | 130 |
| Leads | 20 |
| Daily scan leads | 12 |
| Daily scan runs | 1 completed |
| Dark signals | 1 |
| Drafts | 0 |
| Verification tasks | 3 |
| Publish runs | 0 |
| Published posts | 0 |

The completed scan run was `id=1`, started at `2026-06-29T00:19:10.960188100+00:00` and completed at `2026-06-29T00:20:38.164461500+00:00`.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: draft generation control is clipped/unreachable at 1280x720

Observed: `b10d61d` fixes the prior Story Queue layout enough to show the compact navigation, summary counts, lead cards, and visible `Draft` buttons after scrolling. Clicking a `Draft` button opened the drafting form for a Longmont lead. The draft form then clipped the primary generation control at the bottom edge of the 1280x720 app window. Wheel/PageDown attempts did not reveal the full action, and low-edge clicks did not create a draft. Database count remained `drafts=0`.

Expected: The drafting form should keep its primary action fully visible or scrollable at 1280x720 so the tester can generate drafts through the packaged UI.

Impact: The recovered local newsroom value loop now reaches produced leads and a draft form, but it still cannot generate drafts, exercise writer/editor flows, export, publish to here.now, or begin the 12-hour soak.

Repro:

1. Clean reset CivicNewspaper/Ollama state.
2. Install `The Civic Desk_0.2.8_x64-setup.exe` from the `b10d61d` artifact folder.
3. Launch `civicnews.exe` at 1280x720.
4. Wait for recovered setup, auto source intake, Daily Scan, and Story Queue route.
5. Scroll Story Queue until lead cards and `Draft` buttons are visible.
6. Click a lead's `Draft` button.
7. Observe the draft form opens, but the bottom primary generation control is clipped/unreachable and no draft is created.

## Request For Coder

`b10d61d` is another meaningful layout step: Story Queue and lead cards are visible/reachable now. Please fix the draft form's 1280x720 layout/scrolling so the primary generation action is fully visible and usable, then the cleanroom run can continue into draft generation, editor workflow, export, here.now publish, and soak.
