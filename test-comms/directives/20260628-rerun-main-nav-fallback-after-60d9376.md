# Directive: rerun cleanroom after main navigation fallback

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `6f73b3f` proved setup now completes and enters the main app, but normal click/key navigation did not move from `Story Queue` to `Daily Scan`.

Product commit `60d9376` adds resilient main navigation fallbacks:

- native capture listeners for `pointerdown`, `mousedown`, and `click`,
- direct button `onPointerDown`, `onMouseDown`, and `onClick`,
- Alt/Ctrl-number shortcuts for nav tabs.

The goal is to prove the packaged Windows app can be navigated after recovered setup, then continue the full Longmont E2E publication test.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`60d9376`

Product commit subject:

`Add resilient main navigation fallbacks`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-main-nav-fallback-rerun-60d9376/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `B89C33C70A5BF5789AF38158809833F5CE8BDF35E5A8E5E0E6DF33B24F159AD5`
- MSI SHA256: `8C5B7BC4E7AC602ACD04870E2B73B18D148E59C4D06D781CC6D7BF4607A76260`

## Required cleanroom reset

Before installing this artifact, remove only CivicNewspaper-related cleanroom state:

- installed CivicNewspaper app,
- CivicNewspaper app data,
- prior exported CivicNewspaper outputs from this test,
- prior local Ollama/model state created by this product test if it was product-installed,
- stale helper windows/processes started by the previous run.

Do not wipe Windows, the browser, the tester user account, Git, or the coordination checkout.

## Setup and navigation retest

1. Install `The Civic Desk_0.2.8_x64-setup.exe` from the artifact folder.
2. Launch the app as a normal user at 1280x720.
3. Do not manually install Ollama, manually pull a model, use a helper terminal, repair PATH yourself, or use developer tools.
4. Let first-run setup complete through the app UI and recovery paths.
5. When the main app opens, try normal nav clicks.
6. If normal clicks fail, try keyboard shortcuts:
   - Alt+2 or Ctrl+2 for Daily Scan.
   - Alt+6 or Ctrl+6 for Sources.
   - Alt+8 or Ctrl+8 for Publishing.
7. Capture screenshots showing whether each route changes.

If the app still cannot navigate, stop and report the exact route state and which fallback attempts failed.

## Full Longmont publication workflow

If navigation works, continue without waiting for another directive:

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

`test-comms/reports/20260628-main-nav-fallback-rerun-report-60d9376.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-main-nav-fallback-rerun-60d9376/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
