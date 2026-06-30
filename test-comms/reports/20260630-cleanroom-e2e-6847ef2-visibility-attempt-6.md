# CivicNewspaper Cleanroom E2E Attempt 6 - Visibility

UTC report time: 2026-06-30T13:30:00Z

Status: PASS for visibility prerequisites.

## Machine And Branch

- Windows user: `MSI\civic`
- Hostname: `MSI`
- Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Coordination commit observed: `5cd56afeccb88ad1562efa1bff1dc123b3c2abfe`
- Active directive pointer: `test-comms/directives/20260630-cleanroom-e2e-6847ef2-attempt6.md`
- Watcher: 15-minute heartbeat remains armed.

## Product Under Test

- Product branch: `main`
- Product commit: `6847ef2844a1a859eb82ae900ef03b08c94b132a`
- Product version: `0.3.0`

## Installer Verification

NSIS installer:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/The Civic Desk_0.3.0_x64-setup.exe`
- Expected SHA256: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- Observed SHA256: `33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C`
- Expected size: `5623070`
- Observed size: `5623070`
- Result: match.

MSI fallback:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/The Civic Desk_0.3.0_x64_en-US.msi`
- Expected SHA256: `CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1`
- Observed SHA256: `CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1`
- Expected size: `9125888`
- Observed size: `9125888`
- Result: match.

## Next

Proceeding with the attempt-6 cleanroom E2E using the NSIS installer first.
