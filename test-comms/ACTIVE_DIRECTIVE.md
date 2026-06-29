# ACTIVE DIRECTIVE - Tester Read This First

Status: ACTIVE

Tester, always read this file first on every 15-minute watcher tick.

IMPORTANT MACHINE CONTEXT:

- You are the tester on the separate cleanroom machine running as msi\civic.
- Do not use any path under C:\Users\instynct; that path belongs to the coder machine and does not exist on the tester machine.
- The approved tester coordination checkout path is:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

- If you were previously watching CivicCast or any other project, stop that watcher context now. Switch to CivicNewspaper only.
- The single source of truth is GitHub repo https://github.com/scottconverse/CivicNewspaper, branch test-comms/cleanroom-coder-tester, this file.

## Current Directive

Run this directive now:

test-comms/directives/20260629-rerun-output-cleanup-7fe1145.md

This verifies the product fixes in commit `7fe11452ea7ccbb9425df291a030da58ff8e48bf` after the 637e941 cleanroom run proved the functional E2E path but found mojibake in published output. This is a focused blocker-verification rerun before the next full clean-wipe E2E pass.

Product branch:

stable-readiness-local-gates

Product commit:

7fe11452ea7ccbb9425df291a030da58ff8e48bf

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-7fe1145/

Expected preferred NSIS SHA256:

9F495209FFA6254B095EA946F5C2553067D5362834FC7BF62D662522B9F36C4A

Expected fallback MSI SHA256:

18B9C45C7896A42C554177A063D08B4462A44C2563FF11437E19F5DA8ACFB154

## Current Goal

Verify the fixed build against the published-output blockers: no mojibake in generated/public output, no stale killed-story pages in the export, killed stories cannot be approved directly by accident, and anonymous here.now publishing uses a nonempty display name without manual repair. Then record the here.now URL, screenshots, output paths, ZIP, and a human-readable report.

Commit reports/artifacts with [skip ci].
