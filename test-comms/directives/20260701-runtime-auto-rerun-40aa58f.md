# Tester Directive - Runtime Auto Install Rerun 40aa58f

Date: 2026-07-01T05:50:00Z

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

40aa58f4fc7c7cf05fefe709e40dba8bb4d376cc

What changed since the previous blocked run:

- The recovered clean-machine setup flow now auto-starts product-owned local AI runtime installation when Step 2 cannot receive input and the local AI service is offline.
- Manual offline AI setup does not auto-install during ordinary use.
- Step 2 footer Next remains disabled while the local AI service is unavailable.
- Step 2 runtime controls remain lifted above the long explanatory copy and footer.

Installer artifact:

test-comms/artifacts/20260701-runtime-auto-rerun-40aa58f/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

1237D4FE08A03D9662585D760F501D987277297F6420634B67F351704B6EBA31

Expected size:

5632601

Required visibility report:

test-comms/reports/20260701-runtime-auto-rerun-40aa58f-visibility.md

Required final report:

test-comms/reports/20260701-runtime-auto-rerun-40aa58f-report.md

Evidence folder:

test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/

Instructions:

1. Fetch and read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
2. Verify installer hash and byte size exactly before install.
3. Product clean wipe: Civic Desk install, app data, output folders, and Civic Desk managed Ollama runtime/model state.
4. Install the NSIS package silently.
5. Launch the installed app normally. Do not manipulate the native window handle for visibility gates.
6. Confirm Step 1 auto-continues to Step 2 and persists Longmont identity settings.
7. Confirm Step 2 footer Next is disabled while the local AI service is unavailable.
8. Confirm the Install local AI runtime and Check Initialization Status controls are fully visible above the footer at the normal desktop viewport.
9. Do not manually install Ollama, models, runtime dependencies, or PATH entries. The product must own runtime setup.
10. In recovered no-input Step 2 state, wait at least 90 seconds for the product to auto-start local AI runtime installation.
11. Capture visible progress, notices, or errors while runtime setup is running.
12. If product-owned runtime install starts and succeeds, continue the full Longmont E2E flow: model download, source discovery, scan, enough leads, draft generation, writer/editor workflow, export ZIP, here.now publish, public output quality, duplicate-topic audit, and mojibake audit.
13. If product-owned runtime install does not start, stalls without progress, errors without a useful recovery message, or requires tester-installed prerequisites, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, process lists, installed files, logs, and a plain-English failure explanation.
14. If any later E2E gate fails, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, output artifacts, and a plain-English failure explanation.

Pass criteria for this rerun:

- Step 1 recovery auto-continues and persists identity.
- Step 2 Next cannot hide the app when the local AI service is unavailable.
- Runtime setup controls are visible and not covered by the footer.
- Without tester help, the product starts local AI runtime installation from recovered Step 2.
- Runtime setup gives visible progress or a useful failure message.
- If setup succeeds, full Longmont E2E continues until publication output or the next exact blocker.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
