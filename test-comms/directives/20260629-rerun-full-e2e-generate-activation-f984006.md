# Directive - Full Cleanroom Longmont Publication Rerun - f984006

Status: ACTIVE

This directive supersedes `test-comms/directives/20260629-rerun-full-e2e-workbench-route-8e4fcca.md`.
The prior run proved the Draft click reaches the Workbench drafting route, but Generate Draft did not reliably activate/persist a draft. This rerun tests the Generate Draft activation hardening, then continues the full publication path.

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

Required product commit: `f98400668680a5b579ad186a33a0ace8f5df7aed`

Product subject: `Make draft generation activation reliable`

## Installer Artifacts

Preferred NSIS installer:

`test-comms/artifacts/20260629-rerun-full-e2e-f984006/The Civic Desk_0.2.8_x64-setup.exe`

Expected SHA256:

`AE99DA832C8126122A68DEC2ECD2498B56253D35B491FB1E5035B1ED11807CB3`

Fallback MSI:

`test-comms/artifacts/20260629-rerun-full-e2e-f984006/The Civic Desk_0.2.8_x64_en-US.msi`

Expected SHA256:

`29A5148C1DB048125E26743B3E7588E1928C0FE06BBF25A5CDF544D824EF4183`

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
- Already-drafted leads open the existing draft instead of creating duplicate drafts.
- Draft generation controls are reachable on the tester window.
- Clicking Draft on any lead must move the user into the Workbench route, visibly reveal the draft wizard, and keep generation progress visible.
- The Generate Draft button must receive focus when the draft wizard opens.
- Pressing Enter on the draft wizard, outside a form field, must trigger Generate Draft.
- Clicking the visible Generate Draft button must trigger generation, show progress, and persist draft 1.
- After generating draft 1, returning to Story Queue and clicking Draft on a different visible lead must visibly reveal the draft wizard for lead 2 and allow draft 2 to persist.
- Repeat drafting must be possible until at least 5 drafts/stories/briefs exist, unless the app itself shows a clear blocker.
- Bulk Import and Discovery commit buttons are reachable on constrained windows.
- Navigation resets each screen to the top instead of preserving old scroll position.
- Publication identity/output text is user-controlled and does not invent ad-policy, AI, ownership, or public-record-only claims.

## Report Paths

Write the main human-readable report here:

`test-comms/reports/20260629-full-e2e-generate-activation-f984006-report.md`

Put screenshots/logs/output artifacts here:

`test-comms/artifacts/20260629-rerun-full-e2e-f984006/`

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
