# Tester Visibility Report - autocontinue rerun 4e96d0a

Date: 2026-07-01T05:06:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: 4e96d0a2bc744364388c2a92316e25eb67b28c63
Directive: test-comms/directives/20260630-autocontinue-rerun-4e96d0a.md

## Result

PASS for the startup visibility gate and Step 1 auto-continue gate.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for this gate.

The app auto-continued from Step 1 to Step 2 without tester clicking Next. Step 2 appeared visibly, and the app DB persisted identity settings.

## Installer Verification

- Installer: test-comms/artifacts/20260630-autocontinue-rerun-4e96d0a/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5631868
- Actual size: 5631868
- Expected SHA256: 3142132BD633C187D5D21BEDBF03EB52CC0890C73D4CECF21656C8BD023AC12C
- Actual SHA256: 3142132BD633C187D5D21BEDBF03EB52CC0890C73D4CECF21656C8BD023AC12C

## Evidence

- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/cleanwipe-install-launch.log
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/screenshot-01-normal-launch-after-30s.png
- test-comms/evidence/20260630-autocontinue-rerun-4e96d0a/db-snapshot-step2-auto-continue.json

## Persisted Identity Settings

```json
[
  {"key": "identity.city", "value": "Longmont"},
  {"key": "identity.editor_name", "value": "Publisher"},
  {"key": "identity.newsroom_name", "value": "My Local Publication"},
  {"key": "identity.organization_type", "value": "single_person"},
  {"key": "identity.state", "value": "CO"}
]
```

## Notes

The app is currently on Step 2 of 5, `AI Service Setup`, with `Starting the local AI service` visible. Full Longmont flow testing continues in the final report.
