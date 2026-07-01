# Tester Directive - Recovered Model Pull Rerun 71ab39d

Date: 2026-07-01T06:46:00Z

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

71ab39d3a8f5b6c947946b6b5af6862064dc8c94

What changed since the previous blocked run:

- The app now keeps the Step 3 window visible after the model-download action.
- In recovered setup mode, when runtime auto-install succeeds, the app now sets autoStartPull before entering Step 3.
- This means the recovered-input path should start the recommended model pull programmatically instead of waiting for a dropped WebView click.
- A regression test now covers recovered runtime install automatically invoking pull_ollama_model.

Local verification on coder machine:

- npm test -- OnboardingWizard.test.tsx --run: passed, 28 tests.
- npm test -- --run: passed, 191 tests.
- npm run build: passed.
- cargo test: passed, 178 tests, 4 ignored live or fixture gates.
- npm run tauri -- build: passed.
- scripts/windows-installer-smoke.ps1: passed with receipt C:\Users\instynct\Desktop\CODE\civicnewspaper\.agent-runs\windows-installer-smoke-20260701-004549\windows-installer-smoke-receipt.json. This receipt path is on the coder machine only; do not use it on tester.

Installer artifact:

test-comms/artifacts/20260701-recovered-model-rerun-71ab39d/The Civic Desk_0.3.1_x64-setup.exe

Expected SHA256:

43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04

Expected size:

5632526

Required visibility report:

test-comms/reports/20260701-recovered-model-rerun-71ab39d-visibility.md

Required final report:

test-comms/reports/20260701-recovered-model-rerun-71ab39d-report.md

Evidence folder:

test-comms/evidence/20260701-recovered-model-rerun-71ab39d/

Instructions:

1. Fetch and read test-comms/ACTIVE_DIRECTIVE.md first, then this directive.
2. Verify installer hash and byte size exactly before install.
3. Product clean wipe: Civic Desk install, app data, output folders, and Civic Desk managed Ollama runtime/model state.
4. Install the NSIS package silently.
5. Launch the installed app normally. Do not manipulate the native window handle for visibility gates.
6. Confirm prior gates still pass: visible native launch, recovered Step 1/Step 2 path, identity persistence, disabled Step 2 Next when local AI is offline, visible runtime controls, and product-owned runtime auto-install.
7. At Step 3, do not rely only on manual click behavior. Watch whether recovered setup automatically starts the model download after the runtime becomes ready.
8. Confirm the app remains visible after Step 3 starts.
9. Confirm model download actually starts through the product, without tester installing models manually. Capture screenshots, process state, ollama list, model directory state, app DB snapshot, and diagnostics at 10, 30, 60, and 120 seconds.
10. If model download starts and succeeds, continue the full Longmont E2E flow: source discovery, scan, enough leads, draft generation, writer/editor workflow, export ZIP, here.now publish, public output quality, duplicate-topic audit, and mojibake audit.
11. If Step 3 does not auto-start the model pull, hides the app, fails to show progress/error, or requires tester-installed prerequisites, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, process lists, model list, logs, and a plain-English failure explanation.
12. If any later E2E gate fails, stop at the failure and capture screenshots, DB snapshots, runtime diagnostics, output artifacts, and a plain-English failure explanation.

Pass criteria for this rerun:

- All prior cleanroom setup gates still pass.
- Recovered setup starts the recommended model download automatically after runtime install.
- The app remains visible during model setup.
- Model setup gives visible progress or a useful failure message.
- If model setup succeeds, full Longmont E2E continues until publication output or the next exact blocker.

Do not merge, tag, or push product branches. Only write reports and evidence to test-comms paths.
