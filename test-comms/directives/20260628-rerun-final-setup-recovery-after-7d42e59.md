# Directive: rerun cleanroom setup after final setup recovery

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `e041324` proved:

- the app recovered past stuck identity input,
- the app started its own Ollama runtime,
- the app saw `/api/tags` return `{"models":[]}`,
- the app auto-started `qwen2.5:7b`,
- Ollama later listed `qwen2.5:7b`.

The blocker was that the UI stayed on Step 3 and could not continue because the packaged WebView still did not receive usable input/scroll/click events.

Product commit `7d42e59` adds final setup recovery. Once recovery mode is active, the app polls model readiness independently. When `/api/tags` lists the selected model, the app saves the selected model, saves default folders, marks onboarding complete, and enters the application without requiring another button click.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`7d42e59`

Product commit subject:

`Complete onboarding after recovered model install`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-final-setup-recovery-rerun-7d42e59/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `45F0B044688E3C7F68FBCC35BB70DC31C06600AF7B4CED41EC9CE1A7714AD418`
- MSI SHA256: `62400F326942EF9E1196059F5523DB4CBE5531C7B9D59F5E8E8B38CFFFFC0778`

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
3. Do not manually install Ollama, manually pull a model, use a helper terminal, or repair PATH yourself.
4. Complete first-run setup only through the app UI and its own automatic recovery paths.
5. Confirm the app reaches the usable main application after model installation without manual clicks if input remains stuck.
6. Capture screenshots of each recovery stage.

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

## Soak requirement

After a real publication exists, run the 12-hour cleanroom soak:

- Keep the app installed and running.
- Do not manually repair dependencies.
- Every 15 minutes, record whether the app is still responsive, whether the local backend is reachable, whether Ollama is still reachable, whether the model remains listed, and whether any unexpected windows/prompts appeared.
- At least once during the soak, reopen the exported site/package and verify it still exists.
- At the end, write a human-readable soak report.

If the app crashes, helper windows appear, Ollama disappears, model state is lost, the output package disappears, or here.now publish/report data is missing, stop the soak and report the exact failure.

## Report paths

Write the next report here:

`test-comms/reports/20260628-final-setup-recovery-rerun-report-7d42e59.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-final-setup-recovery-rerun-7d42e59/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
