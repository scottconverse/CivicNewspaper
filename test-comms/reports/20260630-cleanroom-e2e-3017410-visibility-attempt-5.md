# CivicNewspaper Attempt 5 Visibility Report

UTC report time: 2026-06-30T12:36:49Z

## Repo State

- Current Windows user: `MSI\civic`
- Hostname: `MSI`
- Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Local HEAD: `937813c test-comms: cleanroom e2e attempt 3017410 [skip ci]`
- Remote HEAD: `937813c test-comms: cleanroom e2e attempt 3017410 [skip ci]`
- Active directive pointer: `test-comms/directives/20260630-cleanroom-e2e-3017410-attempt5.md`

## Directive Visibility

- `test-comms/ACTIVE_DIRECTIVE.md`: present and points to the attempt-5 directive.
- `test-comms/directives/20260630-cleanroom-e2e-3017410-attempt5.md`: present and read.
- Product commit under test: `301741042b1a392885ac2de458cc8985a3084bac`

## Installer Artifact Verification

NSIS installer:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64-setup.exe`
- Exists: yes
- Size: `5622123`
- SHA256: `0C79098D0B8720978E7AE056430B2DB7F3247D0072574DE05EC5F5AA9737D35C`
- Matches directive: yes

MSI fallback:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64_en-US.msi`
- Exists: yes
- Size: `9125888`
- SHA256: `2F601F00402ACDA01ECA29597A5866526678F9855F6FB6F5A9DBAD8E2C6D6135`
- Matches directive: yes

## Readiness

I can see the current active directive and the installer artifacts are present with matching hashes. I am able to proceed with the attempt-5 cleanroom test.

Watcher remains armed.
