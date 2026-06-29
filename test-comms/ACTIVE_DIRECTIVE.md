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

test-comms/directives/20260629-full-cleanwipe-longmont-4f0b09d.md

This starts the full clean-wipe end-to-end Longmont publication test after the focused 59eb271 mojibake/output verification passed. It uses commit `4f0b09d9099ca5426c6e75ef36f962906634811a`, which strips legacy `Draft:` working-title prefixes from public publication output and keeps onboarding starter identity neutral instead of inventing city mastheads.

Supersedes:

test-comms/directives/20260629-full-cleanwipe-longmont-c3db2ac.md

Reason: the coder found an adjacent major issue before the tester finished the c3db2ac run. Onboarding starter profiles could still create invented publication names such as Longmont Civic Desk, which would poison the clean-run output.

Product branch:

stable-readiness-local-gates

Product commit:

4f0b09d9099ca5426c6e75ef36f962906634811a

Artifact folder:

test-comms/artifacts/20260629-full-cleanwipe-longmont-4f0b09d/

Expected preferred NSIS SHA256:

7B1A15005679678E1E3E99861D83F4B2BC0741266758C0EEA1898AB56D745CA0

Expected fallback MSI SHA256:

5EA52BA952052E600C3736171365C328289A10E87A720180EDD7930D8217F871

## Current Goal

Run the full clean-wipe end-to-end Longmont publication test: wipe CivicNewspaper/Ollama/model state, install the artifact, use only product-owned setup flows, discover/import Longmont official and public social/community sources when supported, generate leads and stories, exercise writer/editor/advisor paths, export a ZIP/package, publish anonymously to here.now, and produce a human-readable report with screenshots, output paths, ZIP hash, and the here.now URL.

Commit reports/artifacts with [skip ci].
