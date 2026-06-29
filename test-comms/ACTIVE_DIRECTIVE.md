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

`test-comms/directives/20260629-beat-memory-rerun-77ece86.md`

This is a focused Longmont rerun for the latest story-quality and editor-workflow checkpoint. It verifies that held drafts expose Resume Editing and Send Back for More Work, and that Daily Scan uses advisory beat memory to label recurring or evergreen material without hiding or blocking it from the editor.

Supersedes:

`test-comms/directives/20260629-story-quality-workflow-rerun-006c800.md`

Reason: the beat-memory product change is now implemented and should be tested instead of the earlier pre-beat-memory checkpoint.

Product branch:

`stable-readiness-local-gates`

Product commit:

`77ece863db668df9889828587416696f3a39b6cc`

Artifact folder:

`test-comms/artifacts/20260629-beat-memory-rerun-77ece86/`

Expected preferred NSIS SHA256:

`FBAA8AB176A0AB256A0D710B781472DEC15216F99250C30D787D99D430DC85F0`

Expected fallback MSI SHA256:

`EA30BB05B5FFFFDEB7576D42B6C61DB780B2BBE5EF3C6D727AEC94C70125622F`

Expected report:

`test-comms/reports/20260629-beat-memory-rerun-77ece86-report.md`

## Current Goal

Run the focused Longmont story-quality, beat-memory, and draft-workflow rerun. Confirm the app still installs, guides AI setup, scans Longmont sources, labels weak/background/watch material with story-quality and beat-memory context, keeps editor choice intact, exposes held-draft Resume Editing and Send Back for More Work controls, and only publishes/exports genuine current stories if the run finds them.

Commit reports/artifacts with `[skip ci]`.
