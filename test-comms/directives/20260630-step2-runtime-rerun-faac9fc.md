# Tester Directive - Step 2 Runtime Recovery Rerun faac9fc

Date: 2026-07-01T05:15:00Z

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

faac9fc39224d7629e4d5bff870a55b8d33ec9f7

What changed since the previous blocked run:

- Step 2 runtime recovery controls should no longer be clipped behind the footer.
- Step 2 footer Next should stay disabled when the local AI service is unavailable.
- Starter profile pills are native buttons, not anchor buttons.

Installer artifact:

test-comms/artifacts/20260630-step2-runtime-rerun-faac9fc/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

2979D07468778EAF08978A52D1CB82266948042C3464D997374B39FF7F61BAD3

Expected size:

5633385

Required visibility report:

test-comms/reports/20260630-step2-runtime-rerun-faac9fc-visibility.md

Required final report:

test-comms/reports/20260630-step2-runtime-rerun-faac9fc-report.md

Evidence folder:

test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/

Instructions:

1. Stop any old Civic Desk process and fetch the coordination branch.
2. Read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
3. Verify the installer hash and byte size before running it.
4. Perform the same product clean wipe as the previous run: Civic Desk install, app data, output folders, Ollama runtime/model state used by Civic Desk.
5. Install the NSIS installer silently.
6. Launch the installed app normally as the current Windows user. Do not use window-handle manipulation for visibility gates.
7. Confirm visible native app content appears.
8. Confirm Step 1 auto-continues to Step 2 and identity settings persist.
9. On Step 2 with no AI service available, verify the local AI runtime install controls are fully visible and clickable above the footer.
10. Verify the footer Next button is disabled while the local AI service is unavailable. Do not click disabled controls with automation tricks.
11. Click the visible Install local AI runtime button and let the product drive runtime installation. The tester must not install Ollama or models manually.
12. If the product-owned runtime install succeeds and Step 3 appears, continue the full Longmont E2E flow from the prior directive: model download, source discovery, scan, lead/story workflow, editor workflow, export ZIP, here.now publish, output quality checks, duplicate-topic audit, mojibake audit, and final human-readable report.
13. If any gate fails, stop at the failure, collect screenshots, DB snapshots, runtime diagnostics, and a plain-English failure explanation.

Pass criteria for the Step 2 regression:

- The install-runtime controls are fully visible without being covered by the footer.
- The footer Next button cannot hide the app when the local AI service is unavailable.
- The user has a clear visible product-owned path to install the runtime or skip setup.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
