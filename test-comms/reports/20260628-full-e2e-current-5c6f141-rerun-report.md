# Tester Report - Full E2E Current 5c6f141 Rerun

Date: 2026-06-29T02:20Z-02:40Z
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 5c6f141c87175de187f89a887d4f91f08a73da2d
Directive: test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md

## Summary

Fail/blocked. The corrected directive was runnable and the app passed install, launch, clean setup recovery, app-owned local AI startup, model availability, source intake, Daily Scan/Story Queue recovery, and first draft generation. The run blocked before the required 5-10 stories because opening/generating a second lead through the visible UI repeatedly returned to the top Story Queue/main navigation without creating another draft.

This is not ready for Scott to use for a real Longmont publication next week because the full writer/editor/export/publish path was not reachable.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.4 GB on C:
- Node: not manually used
- Rust: not manually used
- npm: not manually used
- Ollama installed/running: no global `ollama` command before install; app-owned Ollama process started during setup
- Models present: qwen2.5:7b shown in app as Local AI ready

## Install Artifact

Installed:

`test-comms/artifacts/20260628-full-e2e-current-5c6f141/The Civic Desk_0.2.8_x64-setup.exe`

Observed and expected hashes matched:

- NSIS SHA256: `CF901350E6CA13A109FF1DFBFB3FF733B149CA53AB2D7D73014C2B5F8CCA86B7`
- MSI SHA256: `7ADA24DE59243CCF60D39601039AFAB5497D5715B15085EF7C78B04B49311FFA`

## Clean Wipe Boundary

Pre-wipe state:

- `civicnews` running: yes, from prior test
- `ollama` running: yes, from prior app-owned test state
- global `ollama` command: none
- user model store before product install: `C:\Users\civic\.ollama`

Removed only CivicNewspaper/product-owned state:

- `C:\Users\civic\AppData\Local\The Civic Desk`
- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- `C:\Users\civic\.ollama`
- `C:\Users\civic\AppData\Local\Ollama`
- `C:\Users\civic\AppData\Roaming\Ollama`
- `C:\Users\civic\AppData\Local\Programs\The Civic Desk`
- `C:\Users\civic\AppData\Local\Programs\Ollama`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, `test-comms/ACTIVE_DIRECTIVE.md`, and `test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md`.
3. Verified product branch and artifact hashes.
4. Performed clean wipe inside the directive boundary.
5. Installed the NSIS installer with `/S`.
6. Launched the real Tauri app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Set app window to 1280x720.
8. Let setup/model/source recovery run without manually installing Ollama, models, PATH fixes, or prerequisites.
9. Confirmed Local AI ready with `qwen2.5:7b`.
10. Confirmed source intake and queue state using read-only SQLite counts.
11. Opened a visible lead from Story Queue.
12. Confirmed `Generate Draft` is visible above `Article Format`.
13. Generated one draft through the app UI/local model.
14. Returned to Story Queue and attempted to open/generate a second visible lead.
15. Stopped when the second lead attempt returned to Story Queue/main navigation without creating a second draft.
16. Collected final read-only DB counts.

## Results

- Install and launch from provided installer: Pass.
- First-run setup for Longmont, CO: Pass.
- App-owned local AI runtime install/start: Pass.
- Model selection/download: Pass, `qwen2.5:7b` shown ready.
- Source discovery/import: Pass for recovered source set.
- Daily Scan / Story Queue material: Pass.
- At least 10 reviewable leads: Pass, 18 leads.
- 5-10 reader-facing stories/briefs: Fail, only 1 draft generated.
- Writer/editor actions: Not reached beyond initial generated draft editor view.
- Duplicate drafting routing: Not fully evaluated; second-lead flow blocked before enough coverage.
- Export static output and ZIP: Not reached.
- here.now publish: Not reached.
- here.now HTTP 200 verification: Not reached.
- 12-hour soak: Not started.

## Counts

Read-only counts after setup recovery:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 18
- `daily_scan_leads`: 10
- `daily_scan_runs`: 1
- `drafts`: 0
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 3

Read-only counts after first draft:

- `drafts`: 1

Read-only counts after second-lead attempt:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 18
- `daily_scan_leads`: 10
- `daily_scan_runs`: 1
- `drafts`: 1
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 3

Generated draft:

- Draft id 1
- Lead id 5
- Format: `watch`
- Status: `draft_generated`
- Title begins: `Draft: A new official primary document from 'Public Notice Colorado'...`

## Sources Imported

The DB contained 6 sources and 27 evidence items. The visible/source records from prior recovered flow include official/public/community sources such as Longmont city pages, Public Notice Colorado, and a public community signal source. I did not reach a publication issue, so I did not do final source grouping for published output.

## Evidence

Artifacts:

`test-comms/artifacts/20260628-full-e2e-current-5c6f141-rerun/`

Key files:

- `monitor-log.json`: process/runtime monitor; contaminated monitor screenshots were deleted because Chrome was foreground during those captures.
- `01-clean-app-state-after-monitor.png`: clean app UI after setup recovery, Local AI ready.
- `db-counts-after-recovery.json`: recovered source/lead counts before drafting.
- `03-after-wheel-scroll-for-leads.png`: Story Queue lead cards and visible Draft button.
- `04-draft-wizard-open.png`: draft wizard with `Generate Draft` visible above `Article Format`.
- `06-after-keyboard-generate-30s.png`: post-generation route to Workbench/main UI.
- `db-counts-after-generate-attempt.json`: one draft persisted.
- `07-queue-next-leads-visible.png`: second visible lead with Draft button.
- `08-second-draft-generate-attempt.png`: after second attempt, app back at top Story Queue.
- `db-counts-after-second-attempt.json`: still only one draft.

No local output folder, ZIP path, or here.now URL exists because export/publish was not reached.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### Blocker - Cannot produce required 5-10 stories through visible UI

Observed: The first visible lead could be drafted successfully and persisted as one `draft_generated` row. After returning to Story Queue, a different visible lead and Draft button were shown, but attempting to open/generate it returned to the Story Queue/main navigation state without creating a second draft. Final DB count remained `drafts: 1`.

Expected: Tester can repeatedly open produced leads and generate at least 5 drafts/stories through visible UI controls.

Impact: The release E2E flow remains blocked before editor workflow, export, here.now publish, and soak. The product cannot yet prove a real Longmont publication from cleanroom install.

Repro:

1. Clean install `5c6f141` from the directive NSIS installer.
2. Let app complete setup and source recovery.
3. Open Story Queue at 1280x720.
4. Scroll to a lead and click Draft.
5. Use visible `Generate Draft`; wait for first draft to persist.
6. Return to Story Queue.
7. Scroll to a different lead with visible Draft button.
8. Attempt to open/generate it.
9. Observe app returns to top Story Queue/main navigation and DB remains at one draft.

### Minor - Monitor screenshot capture can accidentally capture background Chrome if app is not raised

Observed: During the timed monitor loop, the app process stayed running/responding and Ollama started, but the screenshots captured Chrome because Windows focus was not on the app. I deleted those contaminated screenshots before commit and retained the process log.

Expected: Tester capture scripts should force the app foreground/topmost before taking screenshots.

Impact: Evidence hygiene issue only; clean app screenshots were captured afterward.

## Request For Coder

Please fix or clarify the repeat drafting flow after the first generated draft. The app now reaches setup, Local AI, source intake, 18 leads, and one saved draft, but the full E2E path is still blocked because tester cannot generate 5-10 stories through the visible UI.
