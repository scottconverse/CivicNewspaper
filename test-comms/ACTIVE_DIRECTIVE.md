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

test-comms/directives/20260629-rerun-mojibake-systemic-59eb271.md

This verifies the product fixes in commit `59eb271d323b0e051a01659494958594b6384cf1` after the output cleanup rerun found that the product still needed systemic mojibake repair and that the tester scan needed exact decoded-sequence checks instead of broad Unicode marker checks.

Product branch:

stable-readiness-local-gates

Product commit:

59eb271d323b0e051a01659494958594b6384cf1

Artifact folder:

test-comms/artifacts/20260629-rerun-mojibake-systemic-59eb271/

Expected preferred NSIS SHA256:

0864D76EB0A382A641B03C1A3A65D6B4D6220307DC73FE764C95031E96F02B93

Expected fallback MSI SHA256:

1DC37C593240EECC186486A6F2B750FD10CD69DFAE652043B7A4748DC88AF272

## Current Goal

Verify the fixed build against the published-output blockers: no known mojibake sequences in generated/public output using the exact UTF-8 scanner in the directive, no stale killed-story pages in the export, killed stories cannot be approved directly by accident, and anonymous here.now publishing uses a nonempty display name without manual repair. Then record the here.now URL, screenshots, output paths, ZIP, and a human-readable report.

Commit reports/artifacts with [skip ci].
