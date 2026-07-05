# Visibility Check - 20260704 Final Release v0.3.2 eab6a31

Result: FAIL

Checked at: 2026-07-05T01:05:10Z heartbeat pass

## Required Confirmations

- Machine/user: `msi\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Active directive read: `test-comms/ACTIVE_DIRECTIVE.md`
- Active directive commit: `0425dbf`
- Release URL reachable: `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2` returned HTTP 200
- Public docs URL reachable: `https://scottconverse.github.io/CivicNewspaper/` returned HTTP 200
- Release API assets: exactly two assets were listed, `SHA256SUMS.txt` and `The.Civic.Desk_0.3.2_x64-setup.exe`
- Release API installer size: `5227476`
- Downloaded `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe`: yes
- Downloaded `SHA256SUMS.txt` contains SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`: yes
- Public docs show SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`: yes
- Public docs show `More info`: yes
- Public docs show `Run anyway`: yes

## Visibility Failure

The GitHub release page HTML fetched during this pass did not contain the expected installer SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.

The GitHub release page HTML fetched during this pass also did not contain the expected installer size `5227476`.

The release page did not contain an unreplaced `$hash` placeholder.

## Evidence

- Downloaded checksum file: `test-comms/evidence/20260704-final-release-v032-eab6a31/SHA256SUMS.txt`

## Notes

This visibility report was written before installing the release asset. Because the active directive explicitly requires the visibility report to confirm that the release page shows the installer SHA256 and size, this checkpoint is a FAIL even though the release API and checksum asset are internally consistent.
