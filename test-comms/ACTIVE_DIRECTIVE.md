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

`test-comms/directives/20260629-story-quality-workflow-rerun-006c800.md`

This is a focused Longmont rerun for the latest story-quality and editor-workflow checkpoint. It verifies that held drafts expose Resume Editing and Send Back for More Work, and that Daily Scan/Draft generation label weak or evergreen material instead of inflating it into finished stories.

Supersedes:

`test-comms/directives/20260629-bracketed-note-rerun-5791fb5.md`

Reason: the bracketed-note rerun passed. This directive tests the next checkpoint before beat-memory/suppression-ledger work begins.

Product branch:

`stable-readiness-local-gates`

Product commit:

`006c8009083ea61ba71a365f055b65619d03aed5`

Artifact folder:

`test-comms/artifacts/20260629-story-quality-workflow-rerun-006c800/`

Expected preferred NSIS SHA256:

`8F6111B3E9432CA81E256EE89E672685230D1FA6525375754150DD4EB916F451`

Expected fallback MSI SHA256:

`EA6B6599E9AB2D17F51A01515DC33F66062DFCF7F91653D2AB90AA19BF9862A0`

Expected report:

`test-comms/reports/20260629-story-quality-workflow-rerun-006c800-report.md`

## Current Goal

Run the focused Longmont story-quality and draft-workflow rerun. Confirm the app still installs, guides AI setup, scans Longmont sources, labels weak/background/watch material with story-quality context, keeps editor choice intact, exposes held-draft Resume Editing and Send Back for More Work controls, and only publishes/export genuine current stories if the run finds them.

Commit reports/artifacts with `[skip ci]`.
