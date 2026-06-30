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

`test-comms/directives/20260629-gauntlet-all-cleanmachine-v030-b0be4d7.md`

This is the full clean-machine v0.3.0 GauntletGate test for The Civic Desk. It wipes prior app/Ollama/model state within the approved boundary, installs from the v0.3.0 installer artifact on this coordination branch, proves first-run dependency-absent state, verifies app-guided AI setup, generates and edits a Longmont issue, exports ZIP/static output, publishes anonymously to here.now, and reports human-readable release evidence.

Supersedes:

`test-comms/directives/20260629-beat-memory-rerun-77ece86.md`

Reason: v0.3.0 source is now on main and must receive a full clean-machine release-gate run before any tag, merge, or GitHub Release work.

Product branch:

`main`

Product commit:

`b0be4d7432e9f5f791da68770a9631b8c5892697`

Artifact folder:

`test-comms/artifacts/20260629-gauntlet-all-cleanmachine-v030-b0be4d7/`

Expected preferred NSIS SHA256:

`F3256C116F04B734C8C311E5B3EFEB69B24DAF3134C521C986BDF2C45CC1DF7E`

Expected fallback MSI SHA256:

`D294096A95FEBF55E0CB30D104ADD8B31BC27981F150BA8B70FEDFD547EC07E1`

Expected report:

`test-comms/reports/20260629-gauntlet-all-cleanmachine-v030-b0be4d7-report.md`

## Current Goal

Run the full clean-machine v0.3.0 release-gate test. Confirm the app installs from the provided installer, guides AI/Ollama/model setup without tester-installed dependencies, scans Longmont official and public/social sources, generates leads and drafts, exercises writer/editor workflow, exports a ZIP/static publication package, publishes anonymously to here.now, and produces a human-readable report with evidence artifacts.

Commit reports/artifacts with `[skip ci]`.
