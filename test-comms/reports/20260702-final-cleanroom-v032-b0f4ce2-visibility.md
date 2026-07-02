# BLOCKED - Visibility Report - Civic Desk v0.3.2

Directive: `20260702-final-cleanroom-v032-b0f4ce2`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `b0f4ce21ac4e0e2aa2bd9b2f1139aefd25f63e17`
Tester path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-b0f4ce2/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `D3C29AB23F740EFED8535320C8CE762E50C3B6131BDD041BCD151AA528D228EE`
- Size: `5203001`
- Hash/size result: PASS

## Visibility Findings

- Installed app launched as `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Process/window observed: `civicnews`, title `The Civic Desk`.
- App version visible in first-run screen: `v0.3.2`.
- Identity setup screen was visible and accepted persisted keyboard/clipboard input after DPI-aware automation corrected the click coordinates.
- Identity values persisted in database:
  - `identity.newsroom_name = Longmont Cleanroom Beta Desk`
  - `identity.editor_name = Cleanroom Tester`
  - `identity.city = Longmont`
  - `identity.state = CO`
  - `onboarding.step = 5`
- AI setup completed through app-guided runtime installation. No tester-installed Ollama/model/manual source build was used.
- AI state reached app dashboard with `AI Status: Ready`.
- Longmont starter/source state reached `9` sources watched.

## Evidence

- Clean install and launch log: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/install-clean-launch.log`
- First launch / identity screen: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-01-after-launch.png`
- Identity input accepted: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-14-identity-fields-filled-dpi-aware.png`
- Identity Next advanced to AI setup: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-21-after-identity-next.png`
- AI setup initial failure/recovery: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-22-ai-setup-after-wait.png`
- App-guided runtime install in progress: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-23-after-click-install-local-ai-runtime.png`
- Dashboard after AI setup: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-24-ai-install-after-2min.png`
- Final DB/window snapshot: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/blocked-final-db-window-snapshot.json`

## Result

Visibility and first-run identity blocker checks PASS for v0.3.2. The overall directive remains BLOCKED later in the publish/export flow; see final report.
