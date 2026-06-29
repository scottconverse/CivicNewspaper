# Directive - Full Cleanroom Longmont Publication Rerun - 637e941

Status: ACTIVE

This directive supersedes `test-comms/directives/20260629-rerun-full-e2e-workbench-return-8801b10.md`.
The completed a8c35fb run proved a first-gate Story Queue blocker: clicking a visible `Draft` button did not route into the draft wizard. This build hardens the Story Queue handoff by passing the rendered lead object directly from the visible Draft/card action to the parent instead of relying only on a later ID lookup. It keeps the 8801b10 Workbench return/layout fix, the advisory mojibake fix, and the draft-persistence regression.

## Coordination Rules

- You are the tester on the separate cleanroom machine `msi\civic`.
- Use coordination checkout `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Always read `test-comms/ACTIVE_DIRECTIVE.md` first.
- Do not use paths under `C:\Users\instynct`; that is the coder machine and does not exist on the tester machine.
- Do not manually install Ollama, models, PATH fixes, or prerequisites. If the product cannot do it, report the exact product-visible failure.
- Live anonymous here.now publish is authorized for this test only.
- Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
- Keep the 15-minute watcher armed. If blocked, write the report and continue watching for the next `ACTIVE_DIRECTIVE.md`.

## Product Under Test

Repo: https://github.com/scottconverse/CivicNewspaper

Product branch: `stable-readiness-local-gates`

Required product commit: `637e941ac77361033fc22b48fac33ae1aa50a6b3`

Product subject: `Harden story queue draft handoff`

## Installer Artifacts

Preferred NSIS installer:

`test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64-setup.exe`

Expected SHA256:

`50F64FFCE76106BC1745766CA3AF0A50A46C5464F22BDB65220C8EDED348F67F`

Fallback MSI:

`test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64_en-US.msi`

Expected SHA256:

`04DCB36733FD969C4E17C763220BD9E135256524101883432FCD09E50EC1C7F1`

Verify hashes before install. If hashes mismatch, stop and report.

## Required Clean Wipe Boundary

Wipe only CivicNewspaper, app-owned Ollama/runtime/model state, prior test outputs, app data, and related product prerequisites. Leave Windows, the user account, browser, unrelated software, and unrelated user files intact.

Before installing, confirm and record:

- no `civicnews` process is running,
- no `ollama` process/service is running unless it is app-owned and part of the state being removed,
- whether a user/global `ollama` command exists before product install,
- whether any user model store exists before product install.

## Full E2E Acceptance Target

Run the whole product as a normal clean user would. The test passes only if the software itself can:

1. install and launch from the provided installer,
2. complete first-run setup for Longmont, CO,
3. install/start its own local AI runtime if none exists,
4. select and download the appropriate model for the tester hardware,
5. discover/import official, local-media, and public social/community Longmont sources,
6. run Daily Scan and/or source expansion until it has enough material for a real issue,
7. produce at least 10 reviewable leads if possible, or document the product-visible reason and expansion attempts if not,
8. produce 5-10 reader-facing stories/briefs from app AI output only,
9. exercise writer/editor actions: draft, edit/save, press-freedom/legal-risk advisor, approve, hold, cut/kill or send-back where available,
10. prevent or clearly route duplicate drafting of the same lead,
11. export the publication static output and ZIP from the product,
12. publish anonymously to here.now,
13. verify the here.now URL returns HTTP 200 and shows the publication,
14. save/report local output path, ZIP path, here.now URL, screenshots, and human-readable quality notes.

Do not hand-author article content. Do not repair dependencies outside the app. Do not skip any visible core workflow unless blocked; if blocked, report exactly where and why.

## Specific Regression Checks From Prior Reports

- Current app does not crash after loading screen.
- Install local AI runtime does not crash the app.
- Draft generation saves successfully; no `save_draft created_at` failure.
- Daily Scan saved leads are visible or reachable in Story Queue without requiring a confusing second scan.
- Clicking the visible Story Queue `Draft` button on any non-drafted lead must route into the Workbench draft wizard.
- Clicking the lead card body on any non-drafted lead must route into the Workbench draft wizard.
- Already-drafted leads open the existing draft instead of creating duplicate drafts.
- Draft generation controls are reachable on the tester window.
- Clicking Draft on any lead must move the user into the Workbench route, visibly reveal the draft wizard, and keep generation progress visible.
- The Generate Draft button must receive focus when the draft wizard opens.
- Pressing Enter on the draft wizard, outside a form field, must trigger Generate Draft.
- Pressing Enter while the Generate Draft button is focused must not double-start generation.
- Clicking the visible top or bottom Generate Draft button must trigger generation, show progress, and persist draft 1.
- After generating draft 1, returning to Story Queue and clicking Draft on a different visible lead must visibly reveal the draft wizard for lead 2 and allow draft 2 to persist.
- Repeat drafting must be possible until at least 5 drafts/stories/briefs exist, unless the app itself shows a clear blocker.
- The editor `Back to Queue` button must be fully visible and clickable at 1280x720.
- The press-freedom/advisory panel must not show mojibake such as `Ã` or `â`.
- Bulk Import and Discovery commit buttons are reachable on constrained windows.
- Navigation resets each screen to the top instead of preserving old scroll position.
- Publication identity/output text is user-controlled and does not invent ad-policy, AI, ownership, or public-record-only claims.

## Report Paths

Write the main human-readable report here:

`test-comms/reports/20260629-full-e2e-queue-handoff-637e941-report.md`

Put screenshots/logs/output artifacts here:

`test-comms/artifacts/20260629-rerun-full-e2e-637e941/`

Required report contents:

- Plain-English pass/fail summary.
- Exact install artifact and observed hashes.
- Hardware profile and model selected by the app.
- Sources imported, grouped by official/local media/social/community.
- Lead count and story/brief count.
- What writer/editor controls were exercised.
- Local output folder and ZIP path.
- here.now URL and HTTP verification.
- Screenshots list.
- All blockers/major/minor findings with exact repro steps.
- Whether this is ready for Scott to use for a real Longmont publication next week.
