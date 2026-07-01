# Tester Visibility Report - step3 model rerun c37c756

Date: 2026-07-01T06:09:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: c37c756524a130cd6a415603e1cc12fec019e05e
Directive: test-comms/directives/20260701-step3-model-rerun-c37c756.md

## Result

PASS for installed-app startup visibility, recovered Step 1/Step 2 setup gates, product-owned runtime auto-install, and visible Step 3 `Start download` action.

FAIL for Step 3 model download behavior after clicking `Start download`.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for the visibility gates.

The app auto-continued through recovered setup, started the product-managed Ollama runtime, and reached Step 3. Step 3 showed a visible primary footer action labeled `Start download`, which is an improvement from the previous build.

After clicking `Start download`, the app disappeared from the desktop immediately. The process stayed alive with an empty window title, managed `ollama.exe` stayed alive, and no model download appeared after 120 seconds.

## Installer Verification

- Installer: test-comms/artifacts/20260701-step3-model-rerun-c37c756/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5630582
- Actual size: 5630582
- Expected SHA256: 033D99EFD8EF68C5065BCC7957BDFEF17A977FFF9CAD5733622F762D6625B8FB
- Actual SHA256: 033D99EFD8EF68C5065BCC7957BDFEF17A977FFF9CAD5733622F762D6625B8FB

## Evidence

- test-comms/evidence/20260701-step3-model-rerun-c37c756/cleanwipe-install-launch.log
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-30s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-60s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-auto-install-90s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/screenshot-model-download-10s.png
- test-comms/evidence/20260701-step3-model-rerun-c37c756/model-download-watch.txt
- test-comms/evidence/20260701-step3-model-rerun-c37c756/runtime-autoinstall-watch.txt
- test-comms/evidence/20260701-step3-model-rerun-c37c756/db-snapshot-after-runtime-autoinstall.json
- test-comms/evidence/20260701-step3-model-rerun-c37c756/db-snapshot-after-step3-start-download-hide.json
