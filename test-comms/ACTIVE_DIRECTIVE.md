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

test-comms/directives/20260629-herenow-retest-f092852.md

This is a focused here.now connector retest after the 5a24a5a clean-wipe run proved the rest of the Longmont workflow. It uses commit `f092852e9df3808f16cf56b829993f028e31d255`, which makes here.now use the compiled publication title instead of a generic or empty connector display name.

Supersedes:

test-comms/directives/20260629-full-cleanwipe-longmont-5a24a5a.md

Reason: the 5a24a5a run passed clean-wipe state, AI setup, source discovery, drafting, editor workflow, kill persistence, compile/export, ZIP, identity, mojibake, and draft-prefix checks, but here.now rejected the publish because the request display name was empty after normalization.

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

Run the focused here.now retest: install the artifact, reuse the already-proven 5a24a5a Longmont output package, publish anonymously to here.now from the visible app UI, verify the URL, and produce a human-readable report with screenshots, output path, and here.now URL.

Commit reports/artifacts with [skip ci].
