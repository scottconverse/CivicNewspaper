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

test-comms/directives/20260629-ascii-ui-rerun-607b0f3.md

This is a targeted installer rerun after the duplicate-topic cleanroom pass exposed app-side mojibake in evidence text: `LONGMONT Â· CO`. Product commit `607b0f3bb79b97a4f7cbb0a2286a8722b9a78b34` replaces vulnerable UI separators and ellipses with ASCII-safe text.

Supersedes:

test-comms/directives/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256.md

Reason: the duplicate-topic rerun passed the public publication checks, but its evidence showed app-side mojibake in the sidebar. This directive verifies the targeted UI fix.

Product branch:

stable-readiness-local-gates

Product commit:

607b0f3bb79b97a4f7cbb0a2286a8722b9a78b34

Artifact folder:

test-comms/artifacts/20260629-ascii-ui-rerun-607b0f3/

Expected preferred NSIS SHA256:

B9AF797EE8CEDF81BDE8761BE3FAAE34DA1CE00D122F3227AA0258272611BD1B

Expected fallback MSI SHA256:

C79A80C855CE2131BF599DD80A9A5BD65CB2BDC9C1BCBE2A33190E0410DDE83E

## Current Goal

Run the targeted ASCII UI rerun. Confirm the installed app no longer renders `LONGMONT Â· CO` or other mojibake in the sidebar, Story Queue, Daily Scan labels, or nearby setup/loading/saving UI.

Commit reports/artifacts with [skip ci].
