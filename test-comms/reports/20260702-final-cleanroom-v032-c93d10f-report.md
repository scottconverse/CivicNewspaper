# BLOCKED - Final Cleanroom Release Recheck - Civic Desk v0.3.2 c93d10f

Directive: `20260702-final-cleanroom-v032-c93d10f`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `c93d10f3cd1a913dcb5fb0c846126303c26a8c19`
Tester machine/user/path: Windows tester as `civic`, `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Verdict

BLOCKED.

The build passed installer verification, clean install, first-run Longmont identity setup, app-guided local AI setup, default site folder creation, and Daily Scan database completion. The test is blocked before draft generation/workbench/publish verification because the post-restart dashboard/nav hub would not activate Story Queue, Workbench, Sources, Publishing, or other tiles: mouse clicks and keyboard activation only changed the highlighted tile while the app stayed on the same dashboard view. Because I could not navigate to Story Queue/Workbench through the installed app UI, I could not run the required draft generation, Workbench top action strip, Improve for Publication, approval, export, ZIP, here.now, or public-output checks exactly.

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-c93d10f/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `96BC3D9EAF499765887F5AD82D09CD8BD9B22691AD84ACCFA7EBA68A6A777754`
- Size: `5200988`
- App observed: `The Civic Desk` v0.3.2
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App data path observed: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`

## Clean Wipe / Install

Performed within directive boundary:

- Stopped stale `civicnews` and product-owned `ollama`.
- Ran previous The Civic Desk uninstaller if present.
- Removed `%APPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`.
- Removed/verified absent `%LOCALAPPDATA%\The Civic Desk`.
- Removed prior `%USERPROFILE%\.ollama` from previous CivicNewspaper testing.
- Installed only the directive NSIS artifact.
- Launched installed app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.

Evidence: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/install-clean-launch.log`

## Flow Results

1. Pull/read active directive: PASS.
2. Verify installer hash and size: PASS.
3. Clean wipe product/runtime state: PASS.
4. Install directive NSIS artifact: PASS.
5. Launch installed app normally: PASS.
6. Confirm native window title and product identity: PASS, window title `The Civic Desk`.
7. Complete first-run identity setup: PASS.
8. App-guided AI setup reaches AI Status Ready without manual dependency installation: PASS.
9. Add/discover Longmont starter sources through the app: PASS/PARTIAL. DB shows 18 sources after setup/scan.
10. Run Daily Scan: PASS mechanically by DB state.
11. Newest daily_scan_runs row after leads present is not left `in_progress`: PASS; final DB has one daily_scan_runs row and current run state was not blocked in progress.
12. Generate at least two drafts from different leads: BLOCKED/NOT RUN after navigation blocker.
13. No-source verification assignment behavior: NOT RUN.
14. Linked-source generated draft checks: NOT RUN.
15. Open generated drafts from Workbench draft picker: NOT RUN.
16. Improve for Publication on a linked-source draft: NOT RUN.
17. Approve source-linked attributed copy: NOT RUN.
18. Go to Publish: BLOCKED/NOT RUN after navigation blocker.
19. Open folder before first compile: NOT RUN, but default site folder exists on disk.
20. Confirm default output folder opens/creates: NOT RUN through UI.
21. Compile/export publication package: NOT RUN.
22. Verify ZIP/package files: NOT RUN.
23. Publish to here.now: NOT RUN.
24. Inspect here.now publication: NOT RUN.

## Database Snapshot

From `final-db-summary.json`:

- `sources`: 18
- `daily_scan_runs`: 1
- `daily_scan_leads`: 17
- `leads`: 20
- `evidence_items`: 31
- `lead_evidence`: 9
- `drafts`: 0
- `publish_runs`: 0
- `published_posts`: 0
- `ai.setup_skipped`: `false`
- `model.selected`: `phi4-mini:latest`
- `identity.newsroom_name`: `Longmont Cleanroom Beta Desk`
- `identity.editor_name`: `Cleanroom Tester`
- `identity.city`: `Longmont`
- `identity.state`: `CO`
- `onboarding.step`: `5`
- `onboarding_complete`: `1`

Default site folder:

```text
C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default
```

exists after first launch/setup.

## Generated Drafts Considered For Approval

No drafts were generated in this run before the navigation blocker. `drafts-full.jsonl` is present but empty.

## Output / Publish

- Approved drafts: 0
- ZIP/local artifact path: none; blocked before export
- here.now URL: none; blocked before publish
- Publish runs: 0
- Published posts: 0
- Public-output audit: not reached

## Blocking Defect

### BLOCKER-1: Dashboard/nav hub tile activation does not navigate after restart

Observed: After clean setup and AI-ready state, the installed app was restarted. It displayed the dashboard/nav hub. Multiple mouse clicks on Story Queue, Workbench, Daily Scan, Sources, and Publishing changed only the highlighted tile or left the same tile highlighted; the app did not navigate into those sections. Keyboard tab/enter activation also left the app on the dashboard. This prevented the tester from using the product UI to generate drafts or reach Workbench/Publishing.

Expected: Clicking or keyboard-activating Story Queue, Workbench, Sources, Daily Scan, or Publishing should navigate to the selected app section.

Impact: Blocks the required draft generation, Workbench top action strip verification, Improve for Publication, approval, output-folder UI check, compile/export, ZIP verification, here.now publish, and public-output inspection.

Repro:

1. Install and complete setup from the directive artifact.
2. Reach AI-ready/dashboard state.
3. Restart the installed app.
4. Click Story Queue, Workbench, Daily Scan, Sources, or Publishing from the dashboard/nav hub.
5. Observe the app remains on the dashboard/hub while only the highlighted tile changes.

Evidence:

- `screenshot-current-2358z.png`
- `screenshot-04-story-queue-open.png`
- `screenshot-05-story-queue-click2.png`
- `screenshot-06-nav-clicks.png`
- `screenshot-07-after-restart.png`
- `screenshot-08-keyboard-nav-attempt.png`
- `final-db-summary.json`

## Evidence Folder

All screenshots/logs/snapshots for this run are under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/
```

Key evidence includes:

- `installer-verify.txt`
- `install-clean-launch.log`
- `db-after-ai-ready.txt`
- `final-db-summary.json`
- `drafts-full.jsonl`
- `screenshot-01-launch.png`
- `screenshot-02-after-identity-next.png`
- `screenshot-03-ai-ready.png`
- `screenshot-current-2358z.png`
- `screenshot-04-story-queue-open.png`
- `screenshot-05-story-queue-click2.png`
- `screenshot-06-nav-clicks.png`
- `screenshot-07-after-restart.png`
- `screenshot-08-keyboard-nav-attempt.png`

## Result

BLOCKED. Build `c93d10f` completed clean install, identity setup, AI-ready state, and Daily Scan database state, and it created the default site folder. The run cannot pass because the installed app UI would not navigate out of the dashboard/nav hub after restart, so the required draft/workbench/publish checks could not be executed exactly.
