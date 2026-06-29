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

test-comms/directives/20260629-rerun-full-e2e-after-draft-reveal-2a96751.md

This supersedes stale directive test-comms/directives/20260628-rerun-full-e2e-current-5c6f141.md after the tester reported that the second visible lead did not visibly enter the draft workflow.

Product branch:

stable-readiness-local-gates

Product commit:

2a96751884f9bb7d23ba2c480cd51618e574913d

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-2a96751/

Expected preferred NSIS SHA256:

75B78452EB7863DEE16D69574D6E384D9232886BB308659B2D73E3813EAE05B6

Expected fallback MSI SHA256:

408164A4E6808C00F95CFCB0469DED20027C4BF9560C5C8808DB8DF84A61F5DA

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
