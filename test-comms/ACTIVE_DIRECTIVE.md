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

test-comms/directives/20260629-mojibake-evidence-audit-f092852.md

This is a focused evidence-quality audit after the f092852 here.now retest reported PASS, but its JSON evidence contained mojibake-looking text such as `cityâ€™s` and `LONGMONT Â· CO`. The downloaded HTML and screenshots appear clean, so this directive checks whether the remaining issue is tester evidence serialization or real product/public output.

Supersedes:

test-comms/directives/20260629-herenow-retest-f092852.md

Reason: the here.now connector retest passed the actual publish path and produced URL `https://merry-frost-9arx.here.now`, but the evidence scanner/reporting path gave a false sense of certainty. This audit does not require a new installer or a new publish unless the URL has expired.

Product branch:

stable-readiness-local-gates

Product commit:

f092852e9df3808f16cf56b829993f028e31d255

Artifact folder:

test-comms/artifacts/20260629-herenow-retest-f092852/

Expected preferred NSIS SHA256:

140F2893FFD77751E7C69E8542CEF2BA9AB664E8FE12E430AB1E435AFFBD108D

Expected fallback MSI SHA256:

8EA8D5F210A435AB8DBD06478AA3C5816C0CF0953281FAC44B3100287547E333

## Current Goal

Run the corrected mojibake evidence audit against the f092852 output, downloaded here.now HTML, browser-rendered text, and JSON evidence. Determine whether there is any real public-output mojibake left, and whether evidence serialization is corrupting otherwise clean text.

Commit reports/artifacts with [skip ci].
