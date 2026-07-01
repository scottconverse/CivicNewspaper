# Tester Directive - Step 3 Model Download Rerun c37c756

Date: 2026-07-01T06:03:00Z

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

c37c756524a130cd6a415603e1cc12fec019e05e

What changed since the previous blocked run:

- Step 3 primary footer action now starts the model download instead of opening the skip-confirmation path.
- The explicit Skip for now button remains the only model-download skip path.
- Regression coverage now checks that the primary Step 3 action invokes pull_ollama_model and does not open the skip dialog.
- Runtime auto-install from the prior build remains in place.

Installer artifact:

test-comms/artifacts/20260701-step3-model-rerun-c37c756/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

033D99EFD8EF68C5065BCC7957BDFEF17A977FFF9CAD5733622F762D6625B8FB

Expected size:

5630582

Required visibility report:

test-comms/reports/20260701-step3-model-rerun-c37c756-visibility.md

Required final report:

test-comms/reports/20260701-step3-model-rerun-c37c756-report.md

Evidence folder:

test-comms/evidence/20260701-step3-model-rerun-c37c756/

Instructions:

1. Fetch and read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
2. Verify installer hash and byte size exactly before install.
3. Product clean wipe: Civic Desk install, app data, output folders, and Civic Desk managed Ollama runtime/model state.
4. Install the NSIS package silently.
5. Launch the installed app normally. Do not manipulate the native window handle for visibility gates.
6. Confirm the prior gates still pass: visible native launch, Step 1 auto-continue, identity persistence, disabled Step 2 Next when local AI is offline, visible runtime controls, and product-owned runtime auto-install.
7. At Step 3, click the primary footer action labeled Start download. Do not click Skip for now for this gate.
8. Confirm the app remains visible, model download progress appears, and pull_ollama_model starts through the product. The tester must not manually install models.
9. If model download starts and succeeds, continue the full Longmont E2E flow: source discovery, scan, enough leads, draft generation, writer/editor workflow, export ZIP, here.now publish, public output quality, duplicate-topic audit, and mojibake audit.
10. If Step 3 hides the app, fails to start the download, stalls without useful progress/error, or requires tester-installed prerequisites, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, process lists, model list, logs, and a plain-English failure explanation.
11. If any later E2E gate fails, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, output artifacts, and a plain-English failure explanation.

Pass criteria for this rerun:

- All prior cleanroom setup gates still pass.
- Step 3 primary footer action starts the recommended model download.
- The app remains visible after the Step 3 action.
- Model setup gives visible progress or a useful failure message.
- If model setup succeeds, full Longmont E2E continues until publication output or the next exact blocker.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
