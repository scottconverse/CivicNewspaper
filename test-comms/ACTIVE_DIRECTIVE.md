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

test-comms/directives/20260629-full-cleanwipe-longmont-duplicate-rerun-0941256.md

This is a full clean-wipe Longmont rerun after Scott reviewed the here.now output and found that the five-story paper contained two versions of the same Building Services permitting-portal story. Product commit `09412560a326379fcf75f327439df8d1d2bb47b4` clusters paraphrased Daily Scan leads before they become separate draftable story candidates.

Supersedes:

test-comms/directives/20260629-mojibake-evidence-audit-f092852.md

Reason: publishing and mojibake checks passed, but the newsroom-quality output still failed because duplicated story topics reached the public issue.

Product branch:

stable-readiness-local-gates

Product commit:

09412560a326379fcf75f327439df8d1d2bb47b4

Artifact folder:

test-comms/artifacts/20260629-duplicate-lead-rerun-0941256/

Expected preferred NSIS SHA256:

DC395291F909097A46C273FDC698A0F1822C314F6F019F9092888A6AD7F6B325

Expected fallback MSI SHA256:

B866845F47C32E643A143CD3E5F70FF9F4BCA33912DB036917572D56252ED407

## Current Goal

Run the full clean-wipe Longmont end-to-end workflow again. The publication must contain 5-10 reader-facing stories or briefs with no duplicate story topics, export a ZIP, publish to here.now, and produce a human-readable report with the here.now URL and output path.

Commit reports/artifacts with [skip ci].
