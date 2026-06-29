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

test-comms/directives/20260629-full-e2e-output-quality-landing-cd038d6.md

This is a full cleanroom end-to-end rerun after the public landing page was redesigned and output-quality fixes were added. The prior ASCII/mojibake rerun passed, but it did not retest the full newsroom value loop on the latest product commit.

Supersedes:

test-comms/directives/20260629-ascii-ui-rerun-607b0f3.md

Reason: the ASCII UI rerun passed. This directive verifies the latest installer, redesigned public landing page, app-guided AI setup, Longmont source discovery, full editorial workflow, output quality, ZIP export, and here.now publication.

Product branch:

stable-readiness-local-gates

Product commit:

cd038d696fe9708aaa54c23dd766eff36112f93b

Artifact folder:

test-comms/artifacts/20260629-full-e2e-output-quality-landing-cd038d6/

Expected preferred NSIS SHA256:

520F226F62FCD94B8BF8D3345EB492A990931938FC49D8AA2222EC22DEA07695

Expected fallback MSI SHA256:

C72791A1BC269670EB0D376ED0BA452B2DA375D21588994355535E94294CB2AF

## Current Goal

Run the full cleanroom end-to-end rerun. Confirm the public landing page is updated, the installed app can run the full Longmont newsroom workflow from clean state, the generated output is reader-facing and not reporter notes, the issue exports as ZIP, and the issue publishes to here.now.

Commit reports/artifacts with [skip ci].
