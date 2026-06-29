# ACTIVE DIRECTIVE - Tester Read This First

Status: ACTIVE

Tester, always read this file first on every 15-minute watcher tick.

IMPORTANT MACHINE CONTEXT:

- You are the tester on the separate cleanroom machine running as `msi\civic`.
- Do not use any path under `C:\Users\instynct`; that path belongs to the coder machine and is invalid on the tester machine.
- The approved tester coordination checkout path is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- If you were previously watching CivicCast or any other project, stop that watcher context now. Switch to CivicNewspaper only.
- The single source of truth is GitHub repo `https://github.com/scottconverse/CivicNewspaper`, branch `test-comms/cleanroom-coder-tester`, this file.

## Current Directive

Run this directive now:

`test-comms/directives/20260629-bracketed-note-rerun-5791fb5.md`

This is a cleanroom Longmont rerun after the prior c01e32f test failed because reader-facing public output still exposed bracketed internal editor scaffolding: `[EDITOR_NOTE: ...]`. This directive also checks adjacent mojibake cleanup in public evidence excerpts.

Supersedes:

`test-comms/directives/20260629-output-scaffolding-rerun-c01e32f.md`

Reason: the c01e32f rerun proved install/setup/publish mechanics, but failed output quality. This directive verifies the fixed installer and public cleanup.

Product branch:

`stable-readiness-local-gates`

Product commit:

`5791fb5146d76fc5e97012488c995d0de1bb99c6`

Artifact folder:

`test-comms/artifacts/20260629-bracketed-note-rerun-5791fb5/`

Expected preferred NSIS SHA256:

`9CF4714A253E32D04E1FB1394B6D583B37CCC77C21FDACEBE212D6F1BBDD117C`

Expected fallback MSI SHA256:

`D53AF37831195AD2F36B59436ADA30D14D59313AADB819FBE7E5703AAB85ACCF`

Expected report:

`test-comms/reports/20260629-bracketed-note-rerun-5791fb5-report.md`

## Current Goal

Run the cleanroom Longmont output-quality rerun. Confirm the app still installs, sets up local AI without tester-installed prerequisites, scans Longmont sources, exercises editorial workflow, exports ZIP, publishes to here.now, and no public artifact leaks internal scaffolding or mojibake markers.

Commit reports/artifacts with `[skip ci]`.
