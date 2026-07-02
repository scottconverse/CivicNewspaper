# BLOCKED - Visibility Report - Civic Desk v0.3.2 20cfedc

Directive: `20260702-final-cleanroom-v032-20cfedc`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `20cfedc5bc7a4cd45d954e8a55b87fe4a23f1311`
Tester path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-20cfedc/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `B41AF31919D2271DF2200F6B449CE1B6FB21871826979CF71601730AB97D5C1D`
- Size: `5199754`
- Hash/size result: PASS

## Visibility Findings

- Installed app launched as `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Process/window observed: `civicnews`, title `The Civic Desk`.
- Identity setup screen was visible and accepted Longmont starter/identity values.
- Persisted identity values after setup:
  - `identity.newsroom_name = Longmont Civic Desk`
  - `identity.editor_name = Local Editor`
  - `identity.city = Longmont`
  - `identity.state = CO`
  - `onboarding.step = 5`
  - `onboarding_complete = 1`
- AI setup completed through app-guided runtime installation. No tester-installed Ollama/model/manual source build was used.
- AI state reached ready with `model.selected = phi4-mini:latest`.
- Source discovery / Daily Scan completed mechanically.
- Two linked-source drafts were generated and persisted as `draft_generated`.
- Opening a generated draft from Workbench rendered a blank Workbench content area, blocking Improve for Publication and approval.

## Evidence

- Clean install and launch log: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/install-clean-launch.log`
- First launch: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-01-launch.png`
- Identity filled: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-02-identity-filled.png`
- After identity Next: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-03-after-longmont-next.png`
- AI ready: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-04-ai-ready-wait150.png`
- Story queue: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-05-story-queue.png`
- Draft generation evidence: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-08-after-generate-draft1.png`, `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-13-second-draft-gate.png`
- Blank Workbench blocker: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-current-2328z.png`
- DB summary and full draft text: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/final-db-summary.json`, `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/drafts-full.jsonl`

## Result

Visibility and first-run setup checks PASS for v0.3.2 build `20cfedc`. The overall directive is BLOCKED at the Workbench draft-open step: opening a generated draft renders a blank Workbench state, so Improve for Publication, approval, compile/export, ZIP verification, here.now publish, and public-output inspection cannot proceed.
