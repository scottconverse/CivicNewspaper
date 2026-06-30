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

`test-comms/directives/20260630-canonical-walkthrough-v030-b0be4d7.md`

This is the GauntletGate Walkthrough lane only. Do not run Full.

The tester must produce:

`test-comms/reports/20260630-canonical-walkthrough-v030-b0be4d7-report.md`

Evidence must be written under:

`test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/evidence/`

Product branch:

`main`

Product commit:

`b0be4d7432e9f5f791da68770a9631b8c5892697`

Artifact folder:

`test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/`

Expected preferred NSIS SHA256:

`6C28D0ACEDAA1A367CA8F2EBFFDCB60B2AFC002F123442D1C7FF84EFD1CC95E4`

Expected fallback MSI SHA256:

`AA510FA91B519883190638CBEDB584648B148731DB842371ECB8671D6D7CA154`

## Current Goal

Run the Walkthrough lane for GauntletGate all. Prove or disprove clean first-run dependency-absent behavior with evidence artifacts. Do not run Full.

Commit reports/artifacts with `[skip ci]`.

