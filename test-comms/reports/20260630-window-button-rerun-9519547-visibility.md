# Tester Visibility Report - window button rerun 9519547

Date: 2026-07-01T04:53:00Z
Tester machine: Windows 11 cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit represented by installer: 9519547e35be59ad2002af6759cf11097f4d25f1
Directive: test-comms/directives/20260630-window-button-rerun-9519547.md

## Result

PASS for the startup visibility gate.

The NSIS installer matched the expected byte size and SHA256. After product clean wipe, silent install, and normal launch from the installed EXE as the current user, The Civic Desk rendered a visible native desktop window with visible app content and title `The Civic Desk`.

No `ShowWindow`, `MoveWindow`, `SetForegroundWindow`, taskbar trick, or window-handle manipulation was used for this gate.

## Installer Verification

- Installer: test-comms/artifacts/20260630-window-button-rerun-9519547/The Civic Desk_0.3.1_x64-setup.exe
- Expected size: 5635559
- Actual size: 5635559
- Expected SHA256: 10635FFB94C222D1A03BF569DEF104A21FD30ABBA260E1BEE41873A24538C65B
- Actual SHA256: 10635FFB94C222D1A03BF569DEF104A21FD30ABBA260E1BEE41873A24538C65B

## Evidence

- test-comms/evidence/20260630-window-button-rerun-9519547/cleanwipe-install-launch.log
- test-comms/evidence/20260630-window-button-rerun-9519547/screenshot-01-normal-launch-after-30s.png

## Notes

The app opened on Step 1 of 5 with the Longmont no-input recovery notice visible, starter profile buttons visible, and the Next control visible. Further first-run gates continue in the final report.
