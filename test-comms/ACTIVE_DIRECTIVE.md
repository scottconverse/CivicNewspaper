# ACTIVE DIRECTIVE - Tester Read This First

Status: ACTIVE

Tester, always read this file first on every 15-minute watcher tick.

IMPORTANT MACHINE CONTEXT:

- You are the tester on the separate cleanroom machine running as msi\civic.
- Do not use any path under C:\Users\instynct; that path belongs to the coder machine and is invalid on the tester machine.
- The approved tester coordination checkout path is:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

- If you were previously watching CivicCast or any other project, stop that watcher context now. Switch to CivicNewspaper only.
- The single source of truth is GitHub repo https://github.com/scottconverse/CivicNewspaper, branch test-comms/cleanroom-coder-tester, this file.

## Current Directive

Run this directive now:

test-comms/directives/20260629-full-cleanwipe-longmont-c3db2ac.md

This starts the full clean-wipe end-to-end Longmont publication test after the focused 59eb271 mojibake/output verification passed. It uses commit `c3db2aca6166787e6fb74daf8e1f91c8d8e3dbbb`, which also strips legacy `Draft:` working-title prefixes from public publication output.

Product branch:

stable-readiness-local-gates

Product commit:

c3db2aca6166787e6fb74daf8e1f91c8d8e3dbbb

Artifact folder:

test-comms/artifacts/20260629-full-cleanwipe-longmont-c3db2ac/

Expected preferred NSIS SHA256:

CDA5B555107980A9BC3C9D07D59EFA0A429F5F26A9AB197BB5FB6CC25A7BC0E5

Expected fallback MSI SHA256:

4C4543DCE006112775AC6A3DCBCF915454BE896D20E3266737583461FC2E5C6C

## Current Goal

Run the full clean-wipe end-to-end Longmont publication test: wipe CivicNewspaper/Ollama/model state, install the artifact, use only product-owned setup flows, discover/import Longmont official and public social/community sources when supported, generate leads and stories, exercise writer/editor/advisor paths, export a ZIP/package, publish anonymously to here.now, and produce a human-readable report with screenshots, output paths, ZIP hash, and the here.now URL.

Commit reports/artifacts with [skip ci].
