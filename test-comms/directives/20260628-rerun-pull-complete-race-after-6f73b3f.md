# Directive: rerun cleanroom setup after pull-complete recovery race fix

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `7d42e59` proved:

- identity recovery advanced setup,
- app-managed Ollama started,
- the recommended model pull started,
- `/api/tags` listed `qwen2.5:7b`,
- the app still stayed on Step 3 after more than 3 minutes.

Product commit `6f73b3f` fixes the likely race: if the pull-complete event fires before the recovery health poll advances the wizard, Step 3 must still save the selected model, save defaults, mark onboarding complete, and enter the app. It also uses exact Ollama tag matching with `:latest` equivalence instead of raw list inclusion.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`6f73b3f`

Product commit subject:

`Finish setup after recovered pull completion`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `C9A584326FD3E3A8825A9BB7275F6F90A218FE3AF7C9C7F3C246904E0D1F5CC1`
- MSI SHA256: `39C8C7FE35614AD71FBD63D40C4474D4725B033850A5D1AE2635FF563AECBE8E`

## Required cleanroom reset

Before installing this artifact, remove only CivicNewspaper-related cleanroom state:

- installed CivicNewspaper app,
- CivicNewspaper app data,
- prior exported CivicNewspaper outputs from this test,
- prior local Ollama/model state created by this product test if it was product-installed,
- stale helper windows/processes started by the previous run.

Do not wipe Windows, the browser, the tester user account, Git, or the coordination checkout.

## Setup retest

1. Install `The Civic Desk_0.2.8_x64-setup.exe` from the artifact folder.
2. Launch the app as a normal user at 1280x720.
3. Do not manually install Ollama, manually pull a model, use a helper terminal, repair PATH yourself, or use developer tools.
4. Complete first-run setup only through the app UI and its own automatic recovery paths.
5. Confirm the app reaches the usable main application after model installation without manual clicks if input remains stuck.
6. Capture screenshots of each recovery stage and the first usable main-app screen.

If setup still blocks, stop and report the exact screen, exact process/listener state, `/api/tags`, screenshots, and logs.

## Full Longmont publication workflow

If setup succeeds, continue without waiting for another directive:

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

`test-comms/reports/20260628-pull-complete-race-rerun-report-6f73b3f.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-pull-complete-race-rerun-6f73b3f/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
