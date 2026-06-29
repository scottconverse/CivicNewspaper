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

test-comms/directives/20260629-continue-full-e2e-after-637e941-partial.md

This continues the verified 637e941 build after `test-comms/reports/20260629-full-e2e-queue-handoff-637e941-report.md`. That report proved the queue handoff fix passed for the visible Draft button and card-body click paths, but the full release E2E stopped after 2 drafts. Continue from that installed state if available; otherwise rerun the same verified 637e941 artifacts.

Product branch:

stable-readiness-local-gates

Product commit:

637e941ac77361033fc22b48fac33ae1aa50a6b3

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-637e941/

Expected preferred NSIS SHA256:

50F64FFCE76106BC1745766CA3AF0A50A46C5464F22BDB65220C8EDED348F67F

Expected fallback MSI SHA256:

04DCB36733FD969C4E17C763220BD9E135256524101883432FCD09E50EC1C7F1

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces at least 5 Longmont stories/briefs, exercises writer/editor controls, exports the static output and ZIP, publishes anonymously to here.now, records the URL, screenshots, output paths, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
