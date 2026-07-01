# Tester Directive - Step 2 Action Placement Rerun 8e02eef

Date: 2026-07-01T05:35:00Z

Single source of truth:

test-comms/ACTIVE_DIRECTIVE.md

Coordination repo:

https://github.com/scottconverse/CivicNewspaper

Coordination branch:

test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch:

main

Product commit represented by installer:

8e02eef16b31fa74e77da97dc6520c762b8b67c2

What changed since the previous blocked run:

- Step 2 runtime install actions were moved directly under the recovery heading so they should be visible above the footer.
- A regression test now checks that the install action appears before the long clean-machine explanatory copy.
- The prior disabled Step 2 footer Next behavior remains in place.

Installer artifact:

test-comms/artifacts/20260630-step2-actions-rerun-8e02eef/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

6BE8E9AA80ABBD58AAC6692FE69E17C12A188C32E1BEEEC2CF028D07D4DD5B2F

Expected size:

5631167

Required visibility report:

test-comms/reports/20260630-step2-actions-rerun-8e02eef-visibility.md

Required final report:

test-comms/reports/20260630-step2-actions-rerun-8e02eef-report.md

Evidence folder:

test-comms/evidence/20260630-step2-actions-rerun-8e02eef/

Instructions:

1. Fetch and read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
2. Verify installer hash and byte size exactly.
3. Product clean wipe: Civic Desk install, app data, output folders, and Civic Desk managed Ollama runtime/model state.
4. Install the NSIS package silently.
5. Launch the installed app normally. Do not manipulate the native window handle for visibility gates.
6. Confirm Step 1 auto-continues to Step 2 and persists Longmont identity settings.
7. Confirm Step 2 footer Next is disabled while the local AI service is unavailable.
8. Confirm the Install local AI runtime and Check Initialization Status controls are fully visible and clickable above the footer at the normal desktop viewport.
9. Click Install local AI runtime. The tester must not install Ollama or models manually.
10. If product-owned runtime install succeeds, continue the full Longmont E2E flow: model download, source discovery, scan, enough leads, draft generation, writer/editor workflow, export ZIP, here.now publish, public output quality, duplicate-topic audit, and mojibake audit.
11. If any gate fails, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, and a plain-English failure explanation.

Pass criteria for this rerun:

- Runtime install actions are fully visible and not covered by the footer.
- Clicking Install local AI runtime invokes product-owned runtime setup.
- Step 2 Next cannot hide the app when the local AI service is unavailable.
- If setup proceeds, full Longmont E2E continues until publication output or the next exact blocker.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
