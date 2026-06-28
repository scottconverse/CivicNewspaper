# Directive: rerun cleanroom after automatic Longmont source intake

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `aa0a1e4` proved recovered setup opens `Sources`, but visible source controls still do not respond. Product commit `4658500` makes the recovered first-run path perform the next required newsroom action itself:

- add starter Longmont official/public/community sources,
- fetch/ingest those sources,
- run the first Longmont Daily Scan,
- route to Daily Scan after the automatic intake.

This should let the cleanroom run reach the actual newsroom value loop even while packaged WebView input delivery remains under investigation.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`4658500`

Product commit subject:

`Auto-run Longmont source intake after recovered setup`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-auto-longmont-intake-rerun-4658500/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `1F10EF20E77CBB7AD168191E2CDDA8E3154CAD7110728FD48AA3E34EBA2CBF16`
- MSI SHA256: `DE2082BB3C79CE46571F1623F41A159AEE2D0546D009F013DCF5EF928264B9C1`

## Required cleanroom reset

Before installing this artifact, remove only CivicNewspaper-related cleanroom state:

- installed CivicNewspaper app,
- CivicNewspaper app data,
- prior exported CivicNewspaper outputs from this test,
- prior local Ollama/model state created by this product test if it was product-installed,
- stale helper windows/processes started by the previous run.

Do not wipe Windows, the browser, the tester user account, Git, or the coordination checkout.

## Automatic intake retest

1. Install `The Civic Desk_0.2.8_x64-setup.exe` from the artifact folder.
2. Launch the app as a normal user at 1280x720.
3. Do not manually install Ollama, manually pull a model, use a helper terminal, repair PATH yourself, use developer tools, or manually add sources.
4. Let first-run setup complete through the app UI and recovery paths.
5. Observe whether the app:
   - opens Sources,
   - reports automatic Longmont starter source import,
   - fetches/ingests records and community signals,
   - runs the first Longmont Daily Scan,
   - routes to Daily Scan/results.
6. Record source count, evidence count if visible or inferable, lead count, scan ID, errors, and screenshots.

If automatic intake/scan blocks, stop and report the exact status text, route, process/listener state, `/api/tags`, screenshots, and logs.

## Continue if leads are produced

If the automatic scan produces leads/drafts/results, continue without waiting for another directive:

1. Try to open lead/draft/editor flows if input works.
2. If input still does not work, document that blocker and do not use devtools or direct database edits.
3. If writer/editor flows work, generate 5-10 reader-facing stories/briefs if the product can.
4. Exercise draft, edit, approve, send back, hold, cut, and publish-ready flows.
5. Export the publication ZIP/static output from the app.
6. Publish anonymously to here.now from the app. This specific live publish is authorized.
7. Save the here.now URL, local output path, exported ZIP path, screenshots, and logs.
8. Begin the 12-hour soak only after a real publication output exists.

## Report paths

Write the next report here:

`test-comms/reports/20260628-auto-longmont-intake-rerun-report-4658500.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-auto-longmont-intake-rerun-4658500/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
