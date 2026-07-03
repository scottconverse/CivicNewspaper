# BLOCKED - Visibility Report - Civic Desk v0.3.2 c93d10f

Directive: `20260702-final-cleanroom-v032-c93d10f`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `c93d10f3cd1a913dcb5fb0c846126303c26a8c19`
Tester path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-c93d10f/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `96BC3D9EAF499765887F5AD82D09CD8BD9B22691AD84ACCFA7EBA68A6A777754`
- Size: `5200988`
- Hash/size result: PASS

## Visibility Findings

- Installed app launched as `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Process/window observed: `civicnews`, title `The Civic Desk`.
- First-run identity setup completed with directive identity values:
  - `identity.newsroom_name = Longmont Cleanroom Beta Desk`
  - `identity.editor_name = Cleanroom Tester`
  - `identity.city = Longmont`
  - `identity.state = CO`
  - `onboarding.step = 5`
  - `onboarding_complete = 1`
- AI setup completed through app-guided runtime installation. No tester-installed Ollama/model/manual source build was used.
- AI state reached ready with `model.selected = phi4-mini:latest`.
- Default site folder exists: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
- Source discovery / Daily Scan completed mechanically according to DB snapshot.
- After restart, the app displayed the dashboard/nav hub, but mouse clicks and keyboard activation changed only the highlighted tile and did not navigate into Story Queue, Daily Scan, Workbench, Sources, or Publishing.

## Evidence

- Installer verification: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/installer-verify.txt`
- Clean install and launch log: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/install-clean-launch.log`
- First launch: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-01-launch.png`
- Identity advanced: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-02-after-identity-next.png`
- AI ready: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-03-ai-ready.png`
- Dashboard after restart / navigation blocker:
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-current-2358z.png`
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-04-story-queue-open.png`
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-05-story-queue-click2.png`
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-06-nav-clicks.png`
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-07-after-restart.png`
  - `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/screenshot-08-keyboard-nav-attempt.png`
- DB summary: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/final-db-summary.json`

## Result

Visibility/setup checks PASS through app-guided AI-ready state and Daily Scan database state. The directive is BLOCKED because the dashboard/nav hub would not activate Story Queue, Workbench, or other tiles after restart; the required draft, Workbench action strip, Improve/approval, export, and publish checks could not be run exactly.
