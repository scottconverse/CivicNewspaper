# ACTIVE DIRECTIVE - Tester Read This First

Status: ACTIVE

Tester, always read this file first on every 15-minute watcher tick.

IMPORTANT MACHINE CONTEXT:

- You are the tester on the separate cleanroom machine running as `msi\civic`.
- Do not use any path under `C:\Users\instynct`; that path belongs to the coder machine and does not exist on the tester machine.
- The approved tester coordination checkout path is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- If you were previously watching CivicCast or any other project, stop that watcher context now. Switch to CivicNewspaper only.
- The single source of truth is GitHub repo `https://github.com/scottconverse/CivicNewspaper`, branch `test-comms/cleanroom-coder-tester`, this file.

Do not decide there are "no directives" by scanning for new filenames only. This file is the canonical pointer. It may point at an archived directive in `test-comms/directives/`, or it may contain urgent instructions directly.

## Current Directive

Run this directive now:

`test-comms/directives/20260628-rerun-webview-safe-identity-after-65d97f4.md`

Visibility confirmation has already been received in:

`test-comms/reports/20260628-directive-visibility-msi-civic.md`

That report confirmed:

- local path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`,
- branch is `test-comms/cleanroom-coder-tester`,
- local HEAD and `origin/test-comms/cleanroom-coder-tester` HEAD,
- this file was read,
- the active directive path existed,
- the product branch and commit were read.

Continue with the new active directive unless a blocking mismatch is documented.

Product branch:

`stable-readiness-local-gates`

Product commit:

`65d97f4`

Artifact folder:

`test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/`

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, can:

1. complete first-run setup,
2. install/start its own local AI runtime,
3. download/select the required local model,
4. ingest/discover official, local-media, and public social/community Longmont sources,
5. generate a real Longmont newsroom issue from local AI and real source material,
6. exercise writer/editor approval/hold/cut/send-back workflows,
7. export the publication ZIP/static output,
8. publish anonymously to here.now,
9. report the here.now URL, local output path, exported ZIP path, screenshots, and plain-English human findings.

## Soak Requirement

After the setup/model gate passes and before declaring the product ready, run a 12-hour cleanroom soak:

- Keep the app installed and running.
- Do not manually repair dependencies.
- Every 15 minutes, record whether the app is still responsive, whether the local backend is reachable, whether Ollama is still reachable, whether the model remains listed, and whether any unexpected windows/prompts appeared.
- At least once during the soak, reopen the exported site/package and verify it still exists.
- At the end, write a human-readable soak report under `test-comms/reports/` and put logs/screenshots under the matching `test-comms/artifacts/` folder.

If the app crashes, helper windows appear, Ollama disappears, model state is lost, the output package disappears, or here.now publish/report data is missing, stop the soak and report the exact failure.

## Reporting

Write the next report here:

`test-comms/reports/20260628-webview-safe-identity-rerun-report-65d97f4.md`

Put artifacts here:

`test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/`

Commit and push with `[skip ci]`.
