# Directive: rerun cleanroom after recovered setup source handoff

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `60d9376` proved setup now completes and opens the main app, but main-app navigation remains unresponsive after recovered setup.

Product commit `aa0a1e4` persists a recovered-setup flag and consumes it in the app shell. If setup had to self-drive because input events were not reliable, the app must open `Sources` automatically after onboarding completes instead of stranding the user on `Story Queue`.

This does not replace the full workflow test. It is the next recovery step so the cleanroom run can reach source intake.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`aa0a1e4`

Product commit subject:

`Route recovered setup to source intake`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-recovered-source-handoff-rerun-aa0a1e4/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `240A726E677B21CDE3729B618911989E14FE2E84417C5213AFF7E06AE287FA66`
- MSI SHA256: `803D2EB3DB24D446E1B0DC01DDF4D95BB1491C073A8AF08A5349CBBCA8920FC4`

## Required cleanroom reset

Before installing this artifact, remove only CivicNewspaper-related cleanroom state:

- installed CivicNewspaper app,
- CivicNewspaper app data,
- prior exported CivicNewspaper outputs from this test,
- prior local Ollama/model state created by this product test if it was product-installed,
- stale helper windows/processes started by the previous run.

Do not wipe Windows, the browser, the tester user account, Git, or the coordination checkout.

## Setup and source handoff retest

1. Install `The Civic Desk_0.2.8_x64-setup.exe` from the artifact folder.
2. Launch the app as a normal user at 1280x720.
3. Do not manually install Ollama, manually pull a model, use a helper terminal, repair PATH yourself, or use developer tools.
4. Let first-run setup complete through the app UI and recovery paths.
5. Verify whether the first workspace screen is `Sources` with the recovery status message.
6. If it opens Sources, try normal source workflow controls:
   - Discover for city.
   - Bulk import.
   - Add source.
   - Any needed review/import controls.
7. If controls still do not respond, stop and report the exact first Sources screen plus which control attempts failed.

## Full Longmont publication workflow

If source workflow controls work, continue without waiting for another directive:

1. Add/discover official Longmont, Boulder County, local-media, and public social/community sources.
2. Run source intake/discovery from the app.
3. Generate 10-25 leads if the product can; if it cannot, document exactly where it falls short.
4. Generate 5-10 reader-facing stories/briefs if the product can; if it cannot, document exactly where it falls short.
5. Exercise writer/editor flows: draft, edit, approve, send back, hold, cut, and publish-ready.
6. Export the publication ZIP/static output from the app.
7. Publish anonymously to here.now from the app. This specific live publish is authorized.
8. Save the here.now URL, local output path, exported ZIP path, screenshots, and logs.
9. Begin the 12-hour soak only after a real publication output exists.

## Report paths

Write the next report here:

`test-comms/reports/20260628-recovered-source-handoff-rerun-report-aa0a1e4.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-recovered-source-handoff-rerun-aa0a1e4/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
