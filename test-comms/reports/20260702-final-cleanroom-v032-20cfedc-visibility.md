# PASS WITH FINDINGS - Visibility Report - Civic Desk v0.3.2 20cfedc

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

- Installed app launched from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Native window title observed: `The Civic Desk`.
- First-run Longmont setup completed.
- Persisted identity after setup initially used concrete starter values:
  - `identity.newsroom_name = Longmont Civic Desk`
  - `identity.editor_name = Local Editor`
  - `identity.city = Longmont`
  - `identity.state = CO`
- Publishing page later paused compile until starter publication text was replaced. Tester used the app's `Edit identity` control to set `identity.newsroom_name = Longmont Cleanroom Beta Desk`.
- App-guided AI setup reached ready with `model.selected = phi4-mini:latest`; no manual Ollama/model/source build was used.
- Daily Scan completed and the newest scan row was not left `in_progress`.
- Two linked-source drafts were generated and persisted as editable `draft_generated` items.
- Workbench draft opening ultimately showed a visible editor, guardrail warnings, source evidence, `Plain Language Rewrite`, review controls, attestation checkbox, and `Approve for Static Publish`.
- No-source lead behavior was visible in Story Queue and Workbench: `Needs verification`, `Verify first`, `Linked Sources (0)`, and an assignment note saying it should not be approved until source material is attached or cited.
- Publishing page `Open folder` opened the default output folder successfully.
- Compile/export wrote static files and `site-package.zip`.
- here.now publish succeeded at `https://flint-mango-ee62.here.now`.

## Evidence

- Clean install and launch log: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/install-clean-launch.log`
- First launch: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/screenshot-01-launch.png`
- Identity setup: `screenshot-02-identity-filled.png`, `screenshot-03-after-longmont-next.png`
- AI ready: `screenshot-04-ai-ready-wait150.png`
- Story queue and source-linked draft generation: `screenshot-05-story-queue.png` through `screenshot-14-after-generate-draft2.png`
- Workbench editor/actions visible: `screenshot-19-workbench-actions-lower.png`
- Approval and publish path: `screenshot-25-approved-draft.png`
- No-source assignment: `screenshot-26-nosource-assignment.png`, `screenshot-27-nosource-notes-generated.png`
- Public-output fetches: `here-now-index.html`, `here-now-briefs-2.html`, `here-now-feed.xml`
- DB/public audit: `final-db-summary.json`, `drafts-full.jsonl`, `public-output-audit.json`

## Result

Visibility PASS with product findings. The app reached installed first-run, AI-ready, Workbench, approval, local package export, and here.now public output. Remaining findings are covered in the final report.
