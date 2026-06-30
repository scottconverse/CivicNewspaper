# ACTIVE DIRECTIVE - Tester Read This First

Status: ACTIVE

Tester, always read this file first on every 15-minute watcher tick.

IMPORTANT MACHINE CONTEXT:

- You are the tester on the separate cleanroom machine, not the coder machine.
- Do not use any path under `C:\Users\instynct`; that path belongs to the coder machine and is invalid on the tester machine.
- Use your actual tester checkout path. The approved tester coordination checkout path is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

- If you were previously watching CivicCast or any other project, stop that watcher context now. Switch to CivicNewspaper only.
- The single source of truth is GitHub repo `https://github.com/scottconverse/CivicNewspaper`, branch `test-comms/cleanroom-coder-tester`, this file.

## Current Directive

Run this directive now:

`test-comms/directives/20260630-cleanroom-e2e-a0b436a-attempt1.md`

The tester must first produce:

`test-comms/reports/20260630-cleanroom-e2e-a0b436a-visibility-attempt-1.md`

Then continue and produce the final human-readable cleanroom report:

`test-comms/reports/20260630-cleanroom-e2e-a0b436a-report.md`

Evidence and output artifacts must be written under:

`test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/`

Product branch:

`main`

Product commit:

`a0b436af3009500714055a2bff01612716ee36c1`

Artifact folder:

`test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/`

Expected preferred NSIS SHA256:

`B6777C66A7330A46F6FC443576C06E648E516EC52EC845004044DB4663A23BD8`

Expected fallback MSI SHA256:

`4C4F40178017853DFA5E65AFD10595306018C0F2B803190A1DB431A28CA8AA2E`

## Current Goal

Run the full CivicNewspaper cleanroom end-to-end test from a product clean wipe. Prove or disprove install, first-run AI setup, Longmont source discovery, lead generation, local AI drafting, writer/editor workflow, export ZIP, here.now publish, output quality, duplicate-topic prevention, and mojibake/scaffolding cleanup.

Commit reports/artifacts with `[skip ci]`.
