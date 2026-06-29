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

test-comms/directives/20260629-full-cleanwipe-longmont-5a24a5a.md

This starts the full clean-wipe end-to-end Longmont publication test after the 4f0b09d run proved functional E2E generation/publish but failed clean-wipe certification. It uses commit `5a24a5a597b78907ca5d64019432c1468b3ff30a`, which gates public publishing on a real user-chosen publication name and hardens Kill Story against stale selected-draft state.

Supersedes:

test-comms/directives/20260629-full-cleanwipe-longmont-4f0b09d.md

Reason: the 4f0b09d run found stale output-path contamination, starter identity reaching public output, and Kill Story not persisting a killed/cut item.

Product branch:

stable-readiness-local-gates

Product commit:

5a24a5a597b78907ca5d64019432c1468b3ff30a

Artifact folder:

test-comms/artifacts/20260629-full-cleanwipe-longmont-5a24a5a/

Expected preferred NSIS SHA256:

A19456F776E319E0850463A3494A47B2CBA5668C556724BB1A96C4963E412082

Expected fallback MSI SHA256:

A519ADE9DD15EE20887BB189F6CECD78E6B7BE1CB584B54FB4ACD8159DABF61A

## Current Goal

Run the full clean-wipe end-to-end Longmont publication test: wipe CivicNewspaper/Ollama/model state, install the artifact, use only product-owned setup flows, discover/import Longmont official and public social/community sources when supported, generate leads and stories, exercise writer/editor/advisor paths, export a ZIP/package, publish anonymously to here.now, and produce a human-readable report with screenshots, output paths, ZIP hash, and the here.now URL.

Commit reports/artifacts with [skip ci].
