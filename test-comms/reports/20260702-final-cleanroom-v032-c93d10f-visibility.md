# BLOCKED - Visibility Report - Civic Desk v0.3.2 c93d10f

Directive: `20260702-final-cleanroom-v032-c93d10f`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `c93d10f3cd1a913dcb5fb0c846126303c26a8c19`
Tester path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-c93d10f/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256 observed: `96BC3D9EAF499765887F5AD82D09CD8BD9B22691AD84ACCFA7EBA68A6A777754`
- Size observed: `5200988`
- Hash/size result: PASS

## Visibility Findings

- Installed app launched from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Native window title observed: `The Civic Desk`.
- First-run setup completed with requested identity values:
  - `identity.newsroom_name = Longmont Cleanroom Beta Desk`
  - `identity.editor_name = Cleanroom Tester`
  - `identity.city = Longmont`
  - `identity.state = CO`
- App-guided AI setup reached ready with `model.selected = phi4-mini:latest`; no manual Ollama/model/source build was used.
- Daily Scan completed and the newest scan row was not left `in_progress`.
- Two linked-source drafts were generated and persisted as editable `draft_generated` items.
- Workbench draft picker opened a visible editor.
- The new compact top action strip was visible near the top of Workbench with draft status, title, `Improve for Publication`, `Ready`, attestation checkbox, and `Approve`.
- No-source lead behavior was visible: `Verification assignment`, `Linked Sources (0)`, and note that the item should not be approved until source material is attached or cited.
- Open folder before compile passed for `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
- BLOCKED: `Compile site` did not write the static package or create a publish run after an approved/attested draft existed. Only `.civicdesk-output`, `styles.css`, and `print.css` were present; `site-package.zip`, `index.html`, article HTML, and here.now controls were not produced/exposed.

## Evidence

- Clean install and launch log: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/install-clean-launch.log`
- Installer verification: `installer-verify.txt`
- First launch: `screenshot-01-launch.png`
- Identity/setup: `screenshot-02-after-identity-next.png`
- AI ready: `screenshot-03-ai-ready.png`
- Workbench top action strip: `screenshot-06-workbench-top-action-strip.png`
- No-source assignment: `screenshot-07-nosource-assignment.png`
- Compile no-op/blocker: `screenshot-08-publish-after-compile-failed.png`, `screenshot-09-compile-button-noop.png`
- DB snapshots: `db-after-ai-ready.txt`, `db-after-scan-wait120.txt`, `drafts-full.jsonl`, `final-db-summary.json`

## Result

Visibility PASS through Workbench/top-action-strip and no-source assignment checks. Overall directive BLOCKED at compile/export because the visible `Compile site` action did not produce the package/ZIP or enable here.now publishing.
