# Tester Visibility Report - runtime auto rerun 40aa58f

Date: 2026-07-01T05:54:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: 40aa58f4fc7c7cf05fefe709e40dba8bb4d376cc
Directive: test-comms/directives/20260701-runtime-auto-rerun-40aa58f.md

## Result

PASS for installed-app startup visibility, Step 1 auto-continue, identity persistence, disabled Step 2 Next, Step 2 action placement, and recovered Step 2 product-owned runtime auto-install start.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for this gate.

The app auto-continued from Step 1 to Step 2 without tester clicking Next. Step 2 appeared visibly and the app DB persisted identity settings. On recovered Step 2, the app displayed a notice that setup was not receiving input events and that The Civic Desk was installing the local AI runtime automatically.

Within the required 90-second wait, product-owned runtime setup started `ollama.exe` from the Civic Desk managed runtime path and advanced the app to Step 3, `Download AI Model`.

## Installer Verification

- Installer: test-comms/artifacts/20260701-runtime-auto-rerun-40aa58f/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5632601
- Actual size: 5632601
- Expected SHA256: 1237D4FE08A03D9662585D760F501D987277297F6420634B67F351704B6EBA31
- Actual SHA256: 1237D4FE08A03D9662585D760F501D987277297F6420634B67F351704B6EBA31

## Evidence

- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/cleanwipe-install-launch.log
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-30s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-60s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/screenshot-auto-install-90s.png
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/runtime-autoinstall-watch.txt
- test-comms/evidence/20260701-runtime-auto-rerun-40aa58f/db-snapshot-step2-auto-install-start.json

## Runtime Process Evidence

```text
ProcessName : ollama
Path        : C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe
```
