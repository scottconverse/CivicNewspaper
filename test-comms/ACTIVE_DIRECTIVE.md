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

test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md

This supersedes stale directive test-comms/directives/20260628-rerun-draft-action-above-fold-e2ac517.md and replaces the malformed placeholder directive reported in test-comms/reports/20260628-full-e2e-current-5c6f141-report.md.

Product branch:

stable-readiness-local-gates

Product commit:

5c6f141c87175de187f89a887d4f91f08a73da2d

Artifact folder:

test-comms/artifacts/20260628-full-e2e-current-5c6f141/

Expected preferred NSIS SHA256:

CF901350E6CA13A109FF1DFBFB3FF733B149CA53AB2D7D73014C2B5F8CCA86B7

Expected fallback MSI SHA256:

7ADA24DE59243CCF60D39601039AFAB5497D5715B15085EF7C78B04B49311FFA

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
