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

test-comms/directives/20260629-rerun-full-e2e-workbench-route-8e4fcca.md

This supersedes stale directive test-comms/directives/20260629-rerun-full-e2e-after-draft-reveal-2a96751.md after the tester reported that the second visible lead wizard revealed but draft 2 did not persist.

Product branch:

stable-readiness-local-gates

Product commit:

8e4fcca6f3d762d32c892858fd56605bce971b4b

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-8e4fcca/

Expected preferred NSIS SHA256:

D5D82D9A2BB736D54565ED737DB065B030CB4D83F7E5415451E5EAD0378BE191

Expected fallback MSI SHA256:

9AF8CB74E0D1E80A775D053101824353A9877DA800D29C4B45024F7F5B25659E

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
