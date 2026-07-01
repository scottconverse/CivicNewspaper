# Tester Visibility Report - step2 runtime rerun faac9fc

Date: 2026-07-01T05:22:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: faac9fc39224d7629e4d5bff870a55b8d33ec9f7
Directive: test-comms/directives/20260630-step2-runtime-rerun-faac9fc.md

## Result

PASS for installed-app startup visibility, Step 1 auto-continue, identity persistence, and disabled Step 2 Next.

FAIL for the Step 2 runtime recovery control visibility/clickability regression gate.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for this gate.

The app auto-continued from Step 1 to Step 2 without tester clicking Next. Step 2 appeared visibly and the app DB persisted identity settings. On Step 2 with no AI service available, the footer Next button appeared disabled and was not clicked.

However, the local AI runtime install controls were still clipped by the sticky footer. The `Install local AI runtime` label and upper part of the control were visible, but the lower body was covered by the footer. Clicking the visible area did not start product-owned runtime installation; Chrome came to foreground on a here.now dashboard sign-in page, and no publish was performed.

## Installer Verification

- Installer: test-comms/artifacts/20260630-step2-runtime-rerun-faac9fc/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5633385
- Actual size: 5633385
- Expected SHA256: 2979D07468778EAF08978A52D1CB82266948042C3464D997374B39FF7F61BAD3
- Actual SHA256: 2979D07468778EAF08978A52D1CB82266948042C3464D997374B39FF7F61BAD3

## Evidence

- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/cleanwipe-install-launch.log
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-02-after-click-install-runtime.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/screenshot-03-app-state-after-closing-chrome.png
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/db-snapshot-step2-auto-continue.json
- test-comms/evidence/20260630-step2-runtime-rerun-faac9fc/db-snapshot-final-step2-clipped.json

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
