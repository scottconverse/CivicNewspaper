# Tester Directive - Model Download Window Rerun 4202098

Date: 2026-07-01T06:21:00Z

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

420209825d36f9ee7c9812a8c040ac5f46c9f492

What changed since the previous blocked run:

- The onboarding wizard primary action now has a native DOM click fallback across every step, not only Step 1.
- The inline Step 3 model download button now has the same native click fallback.
- The backend model-pull command now explicitly keeps the main app window visible when the pull starts, reports progress, completes, or errors.
- The Windows installer smoke script now resolves relative output paths to absolute paths before passing the NSIS install directory.

Local verification on coder machine:

- npm test -- OnboardingWizard.test.tsx --run: passed, 27 tests.
- npm test -- --run: passed, 190 tests.
- npm run build: passed.
- cargo test: passed, 178 tests, 4 ignored live or fixture gates.
- npm run tauri -- build: passed.
- scripts/windows-installer-smoke.ps1: passed with receipt C:\Users\instynct\Desktop\CODE\civicnewspaper\.agent-runs\windows-installer-smoke-20260701-002014\windows-installer-smoke-receipt.json. This receipt path is on the coder machine only; do not use it on tester.

Installer artifact:

test-comms/artifacts/20260701-model-window-rerun-4202098/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

7C934848901FAD43DF0D5B88E59F4A62B958EE5BA0DBF740287DB3F6C413F481

Expected size:

5629802

Required visibility report:

test-comms/reports/20260701-model-window-rerun-4202098-visibility.md

Required final report:

test-comms/reports/20260701-model-window-rerun-4202098-report.md

Evidence folder:

test-comms/evidence/20260701-model-window-rerun-4202098/

Instructions:

1. Fetch and read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
2. Verify installer hash and byte size exactly before install.
3. Product clean wipe: Civic Desk install, app data, output folders, and Civic Desk managed Ollama runtime/model state.
4. Install the NSIS package silently.
5. Launch the installed app normally. Do not manipulate the native window handle for visibility gates.
6. Confirm the prior gates still pass: visible native launch, Step 1 auto-continue, identity persistence, disabled Step 2 Next when local AI is offline, visible runtime controls, and product-owned runtime auto-install.
7. At Step 3, click the primary footer action labeled Start download. Do not click Skip for now for this gate.
8. Confirm the app remains visible after clicking Start download.
9. Confirm model download actually starts through the product, without tester installing models manually. Capture screenshots and model directory state at 10, 30, 60, and 120 seconds.
10. If model download starts and succeeds, continue the full Longmont E2E flow: source discovery, scan, enough leads, draft generation, writer/editor workflow, export ZIP, here.now publish, public output quality, duplicate-topic audit, and mojibake audit.
11. If Step 3 hides the app, fails to start the download, stalls without useful progress/error, or requires tester-installed prerequisites, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, process lists, model list, logs, and a plain-English failure explanation.
12. If any later E2E gate fails, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, output artifacts, and a plain-English failure explanation.

Pass criteria for this rerun:

- All prior cleanroom setup gates still pass.
- Step 3 primary footer action starts the recommended model download.
- The app remains visible after the Step 3 action.
- Model setup gives visible progress or a useful failure message.
- If model setup succeeds, full Longmont E2E continues until publication output or the next exact blocker.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
