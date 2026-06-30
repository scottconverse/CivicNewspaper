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

`test-comms/directives/20260630-cleanroom-e2e-c4c10b0-attempt2.md`

The tester must first produce:

`test-comms/reports/20260630-cleanroom-e2e-c4c10b0-visibility-attempt-2.md`

Then continue and produce the final human-readable cleanroom report:

`test-comms/reports/20260630-cleanroom-e2e-c4c10b0-report.md`

Evidence and output artifacts must be written under:

`test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/tester-output/`

Product branch:

`main`

Product commit:

`c4c10b0bcbce8fee789a6209ee10a8c216d88dc9`

Artifact folder:

`test-comms/artifacts/20260630-cleanroom-e2e-c4c10b0/`

Expected preferred NSIS SHA256:

`BF12F1B020D355B95ABBF79597EB629A505C5E966C892B57338BD3AE5AFC498C`

Expected fallback MSI SHA256:

`46EDAC61E261D1E17BFA9BE26C0664554486FC826F6B91DCE01DD8264D5A3DA1`

## Current Goal

Rerun the full CivicNewspaper cleanroom E2E after the cleanroom quality fixes. Specifically verify ZIP creation, publish checklist honesty, public-output scaffolding cleanup, Longmont boilerplate cleanup, Workbench held/send-back workflow, Workbench empty-state visibility, here.now publish, and finished output quality.

Commit reports/artifacts with `[skip ci]`.
