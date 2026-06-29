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

`test-comms/directives/20260629-output-scaffolding-rerun-c01e32f.md`

This is a cleanroom Longmont rerun after the prior full E2E test failed because reader-facing public output exposed internal editor scaffolding strings including `EDITOR_NOTE` and `Body:`.

Supersedes:

`test-comms/directives/20260629-full-e2e-output-quality-landing-cd038d6.md`

Reason: the cd038d6 rerun proved install/setup/publish mechanics and the landing page, but failed output quality. This directive verifies the fixed installer and the public output cleanup.

Product branch:

`stable-readiness-local-gates`

Product commit:

`c01e32fdccb50b5a19182b7128f666e8de5cc304`

Artifact folder:

`test-comms/artifacts/20260629-output-scaffolding-rerun-c01e32f/`

Expected preferred NSIS SHA256:

`9A2828D9B98EBBDEA2F625F5BD3EEFAB824B79E6A80FF8FD57AF7EF534D415DE`

Expected fallback MSI SHA256:

`669B9B40CECDA12657210EE2247C6920B5A1F91FF23BD50CB05B06FC5A49FBEA`

Expected report:

`test-comms/reports/20260629-output-scaffolding-rerun-c01e32f-report.md`

## Current Goal

Run the cleanroom Longmont output-quality rerun. Confirm the app still installs, sets up local AI without tester-installed prerequisites, scans Longmont sources, exercises editorial workflow, exports ZIP, publishes to here.now, and no public artifact leaks internal scaffolding such as `EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.

Commit reports/artifacts with `[skip ci]`.
