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

test-comms/directives/20260629-rerun-full-e2e-draft-wizard-a8c35fb.md

This supersedes stale directive test-comms/directives/20260629-rerun-full-e2e-generate-activation-f984006.md. The coder found and fixed adjacent draft-wizard focus/activation risks before waiting for the tester to rediscover them.

Product branch:

stable-readiness-local-gates

Product commit:

a8c35fbb7e99ec7589c7699f73152893081208fa

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-a8c35fb/

Expected preferred NSIS SHA256:

DF588D903A56ACB7DD2FC469D70BCB3DC872F830F9F4B73C5D0AA7B33193AEDE

Expected fallback MSI SHA256:

2B7D9164ADB6DCA8F38AFD68B1DBF8FAA300E08BAA36A00B9648B05EC8621841

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
