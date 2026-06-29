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

test-comms/directives/20260629-rerun-full-e2e-generate-activation-f984006.md

This supersedes stale directive test-comms/directives/20260629-rerun-full-e2e-workbench-route-8e4fcca.md after the tester reported that the Workbench draft route revealed but Generate Draft did not persist draft 1.

Product branch:

stable-readiness-local-gates

Product commit:

f98400668680a5b579ad186a33a0ace8f5df7aed

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-f984006/

Expected preferred NSIS SHA256:

AE99DA832C8126122A68DEC2ECD2498B56253D35B491FB1E5035B1ED11807CB3

Expected fallback MSI SHA256:

29A5148C1DB048125E26743B3E7588E1928C0FE06BBF25A5CDF544D824EF4183

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
