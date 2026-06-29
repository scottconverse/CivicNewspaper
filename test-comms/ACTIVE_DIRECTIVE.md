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

test-comms/directives/20260629-rerun-full-e2e-workbench-return-8801b10.md

This supersedes stale directive test-comms/directives/20260629-rerun-full-e2e-draft-wizard-a8c35fb.md because the completed f984006 report proved a post-draft workbench return/layout blocker that the a8c35fb artifact likely still contains.

Product branch:

stable-readiness-local-gates

Product commit:

8801b105edf483d63ec065143eea5b20cd66e5fe

Artifact folder:

test-comms/artifacts/20260629-rerun-full-e2e-8801b10/

Expected preferred NSIS SHA256:

CFE61A7858523C370924F37BD7DCA2102F85C9CCF429F3FDC57C6B85C67CC506

Expected fallback MSI SHA256:

BEF4C67E948AEFAE762DEDAF4362C16096D56F3C51A38BC43F8CE919923373E4

## Current Goal

Continue the cleanroom release loop until the installed product, with no manually installed prerequisites, produces a real Longmont publication, exported ZIP/path, here.now URL, screenshots, and a full human-readable report proving the cleanroom E2E workflow.

Commit reports/artifacts with [skip ci].
