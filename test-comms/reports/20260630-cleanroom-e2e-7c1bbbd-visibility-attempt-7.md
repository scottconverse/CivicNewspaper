# CivicNewspaper Cleanroom E2E Attempt 7 - Visibility

UTC report time: 2026-06-30T14:08:22Z

Status: PASS for visibility prerequisites.

## Machine And Branch

- Windows user: `MSI\civic`
- Coordination path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Coordination commit observed: `4555065 test-comms: cleanroom e2e attempt 7c1bbbd [skip ci]`
- Remote branch HEAD observed: `4555065 test-comms: cleanroom e2e attempt 7c1bbbd [skip ci]`
- Active directive pointer: `test-comms/directives/20260630-cleanroom-e2e-7c1bbbd-attempt7.md`
- Active directive exists: yes.
- Working tree before this visibility report: clean.

## Product Under Test

- Product branch: `main`
- Product commit: `7c1bbbd42279c13adeb80d604b156a2e6df7eb81`
- Product version: `0.3.0`

## Installer Verification

NSIS installer:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/The Civic Desk_0.3.0_x64-setup.exe`
- Exists: yes.
- Expected SHA256: `E45BD165A902AE711F950B3CA39EAA4E5BFBA30946F54A06E866504EB40B7C86`
- Observed SHA256: `E45BD165A902AE711F950B3CA39EAA4E5BFBA30946F54A06E866504EB40B7C86`
- Expected size: `5623239`
- Observed size: `5623239`
- Result: match.

MSI fallback:

- Path: `test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/The Civic Desk_0.3.0_x64_en-US.msi`
- Exists: yes.
- Expected SHA256: `DBFC81BF4F4916A15D631940A0A484BD4A89AAEE3DA527DDBC2A7BFF87CAB18A`
- Observed SHA256: `DBFC81BF4F4916A15D631940A0A484BD4A89AAEE3DA527DDBC2A7BFF87CAB18A`
- Expected size: `9125888`
- Observed size: `9125888`
- Result: match.

## Next

Proceeding with the attempt-7 cleanroom E2E using the NSIS installer first.
