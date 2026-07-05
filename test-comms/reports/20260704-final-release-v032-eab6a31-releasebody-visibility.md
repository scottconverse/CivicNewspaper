# Visibility Report - v0.3.2 eab6a31 Release Body Rerun

Date: 2026-07-05T01:23:41Z
Tester machine/user: `msi\civic`
Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Branch: `test-comms/cleanroom-coder-tester`
Coordination commit: `af3d166`
Directive: `test-comms/ACTIVE_DIRECTIVE.md`
Result: PASS

## Checks

- Read `test-comms/ACTIVE_DIRECTIVE.md`.
- GitHub release URL returned HTTP 200.
- Public docs URL returned HTTP 200.
- Release page HTML showed installer SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.
- Release page HTML showed installer size `5227476`.
- Release page HTML did not contain `$hash` or `$sha` where the checksum should be.
- GitHub release API listed exactly two assets:
  - `SHA256SUMS.txt`, size `102`
  - `The.Civic.Desk_0.3.2_x64-setup.exe`, size `5227476`
- `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe`.
- `SHA256SUMS.txt` contains SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.
- Public docs showed SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`, `More info`, and `Run anyway`.

## Evidence

- Downloaded checksum file: `test-comms/reports/20260704-final-release-v032-eab6a31-releasebody-evidence/SHA256SUMS.txt`

Note: The active directive names `test-comms/evidence/20260704-final-release-v032-eab6a31-releasebody/`, but this heartbeat explicitly constrains tester writes and evidence to `test-comms/reports/`, so evidence receipts for this pass are kept under the report folder.
