# Tester Visibility Report - step2 actions rerun 8e02eef

Date: 2026-07-01T05:37:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: 8e02eef16b31fa74e77da97dc6520c762b8b67c2
Directive: test-comms/directives/20260630-step2-actions-rerun-8e02eef.md

## Result

PASS for installed-app startup visibility, Step 1 auto-continue, identity persistence, disabled Step 2 Next, and Step 2 action placement.

FAIL for invoking product-owned runtime setup.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for this gate.

The app auto-continued from Step 1 to Step 2 without tester clicking Next. Step 2 appeared visibly and the app DB persisted identity settings. On Step 2 with no AI service available, the footer Next button appeared disabled and was not clicked.

Unlike the prior run, the Step 2 runtime controls are fully visible above the footer at the normal desktop viewport. However, clicking `Install local AI runtime` did not start product-owned runtime installation.

## Installer Verification

- Installer: test-comms/artifacts/20260630-step2-actions-rerun-8e02eef/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5631167
- Actual size: 5631167
- Expected SHA256: 6BE8E9AA80ABBD58AAC6692FE69E17C12A188C32E1BEEEC2CF028D07D4DD5B2F
- Actual SHA256: 6BE8E9AA80ABBD58AAC6692FE69E17C12A188C32E1BEEEC2CF028D07D4DD5B2F

## Evidence

- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/cleanwipe-install-launch.log
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-02-after-click-install-runtime.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/screenshot-03-after-second-install-click-wait45.png
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/db-snapshot-step2-auto-continue.json
- test-comms/evidence/20260630-step2-actions-rerun-8e02eef/db-snapshot-final-runtime-install-noop.json

## Persisted Identity Settings

The DB persisted the recovered Longmont identity on Step 2:

```json
[
  {"key": "identity.city", "value": "Longmont"},
  {"key": "identity.editor_name", "value": "Publisher"},
  {"key": "identity.newsroom_name", "value": "My Local Publication"},
  {"key": "identity.organization_type", "value": "single_person"},
  {"key": "identity.state", "value": "CO"},
  {"key": "model.selected", "value": "phi4-mini:latest"}
]
```
