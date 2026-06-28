# Directive: rerun cleanroom setup after model-pull recovery

Status: ACTIVE

Tester machine context:

- You are on the separate tester machine as `msi\civic`.
- Use the approved tester coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- Do not use or assume any path under `C:\Users\instynct`.
- Stop any old CivicCast or other-project watcher context. Watch only CivicNewspaper.
- The single source of truth is `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.

## Why this rerun exists

Your report for product commit `30035ac` proved the app got past identity setup, started Ollama, and reached:

`GET http://127.0.0.1:11434/api/tags -> 200 {"models":[]}`

The remaining blocker was that the first-run setup screen still could not receive reliable scroll/click/key input on Step 2, so the tester could not activate `Download qwen2.5:7b`.

Product commit `e041324` adds app-side recovery for that exact case. If Step 2 sees reachable Ollama, no installed models, and no usable input path, the app must advance to Step 3 and start the recommended model pull itself. The tester must not manually install Ollama, manually pull models, or use developer tools to bypass setup.

## Product under test

Repository:

`https://github.com/scottconverse/CivicNewspaper`

Product branch:

`stable-readiness-local-gates`

Product commit:

`e041324`

Product commit subject:

`Auto-start model pull when setup input is stuck`

## Installer artifacts

Use the fresh artifacts in:

`test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/`

Files:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`

Hashes:

- NSIS EXE SHA256: `CF0348593E4E98530D24AE1E449F9DF27FB165E1449BA7B37451B21B39BA4333`
- MSI SHA256: `845514BEE27E0B594DA0EE791F1AB8C8327B26BD055B9159B774DD8A02F5F2F7`

## Required cleanroom reset

Before installing this artifact, remove only CivicNewspaper-related cleanroom state:

- installed CivicNewspaper app,
- CivicNewspaper app data,
- prior exported CivicNewspaper outputs from this test,
- prior local Ollama/model state created by this product test if it was product-installed,
- stale helper windows/processes started by the previous run.

Do not wipe Windows, the browser, the tester user account, Git, or the coordination checkout.

## Test steps

1. Install `The Civic Desk_0.2.8_x64-setup.exe` from the artifact folder.
2. Launch the app as a normal user at 1280x720.
3. Do not manually install Ollama, manually pull a model, use a helper terminal, or repair PATH yourself.
4. Complete first-run setup only through the app UI and its own automatic recovery paths.
5. If Step 1 text input is still stuck, wait for the app's starter Longmont identity recovery to advance setup.
6. Confirm the app starts or installs its own AI runtime.
7. Confirm `http://127.0.0.1:11434/api/tags` becomes reachable and initially lists no models if this is a clean AI state.
8. On Step 2, do not manually pull the model. If the UI remains unresponsive, wait for the app to auto-start the recommended model pull.
9. Confirm Step 3 appears and `qwen2.5:7b` starts downloading from inside the app.
10. Record model progress, completion, failure text, unexpected windows, and screenshots.

If model setup succeeds, continue the full cleanroom Longmont publication test without waiting for another directive:

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

`test-comms/reports/20260628-model-pull-recovery-rerun-report-e041324.md`

Put screenshots/logs/output evidence here:

`test-comms/artifacts/20260628-model-pull-recovery-rerun-e041324/`

If the product reaches publication, place the exported publication ZIP in that artifact folder too, or report the exact local product-generated path if the ZIP is too large to commit.

Commit and push the report/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
