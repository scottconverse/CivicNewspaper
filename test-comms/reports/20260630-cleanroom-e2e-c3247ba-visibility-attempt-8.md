# CivicNewspaper Cleanroom E2E Attempt 8 - Visibility

UTC report time: 2026-06-30T14:50:00Z

Status: PASS for visibility prerequisites.

## Machine And Branch

- Windows user: `MSI\civic`
- Hostname: `MSI`
- Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Coordination commit observed: `f838ff138d62e5113d3b091b99f9b1a99bf94700`
- Active directive pointer: `test-comms/directives/20260630-cleanroom-e2e-c3247ba-attempt8.md`
- Active directive exists: yes.
- Watcher: 15-minute heartbeat remains armed.

## Product Under Test

- Product branch: `main`
- Product commit: `c3247bab7c20129e99d8beb8515b124a2e49248f`
- Product version: `0.3.0`

## Installer Verification

NSIS installer:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-c3247ba/The Civic Desk_0.3.0_x64-setup.exe`
- Expected SHA256: `6801E4C41B081B55045646102DBFA6EE3CD2360AB0827BBFBCC5753D6FF861A8`
- Observed SHA256: `6801E4C41B081B55045646102DBFA6EE3CD2360AB0827BBFBCC5753D6FF861A8`
- Expected size: `5621164`
- Observed size: `5621164`
- Result: match.

MSI fallback:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-c3247ba/The Civic Desk_0.3.0_x64_en-US.msi`
- Expected SHA256: `F2F3B35C92143DDF1C30B39FB6DDE1546E00A795BF4A9CE5B3925AD620EDF9F6`
- Observed SHA256: `F2F3B35C92143DDF1C30B39FB6DDE1546E00A795BF4A9CE5B3925AD620EDF9F6`
- Expected size: `9129984`
- Observed size: `9129984`
- Result: match.

## Next

Proceeding with the attempt-8 cleanroom E2E using the NSIS installer first.
