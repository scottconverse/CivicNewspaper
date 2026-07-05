# Tester Report - v0.3.2 eab6a31 Rerun

Date: 2026-07-05T01:05:10Z
Tester machine: `msi\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product release: `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2`
Release/docs target: `f0cb4a96183da91f262ec15c8035a03d1da78250`
Product build commit per directive: `eab6a31e0bfb1463bcb8f0f26d8909adc4d77d8c`
Directive: `test-comms/ACTIVE_DIRECTIVE.md`
Result: FAIL

## Verdict

The eab6a31 cleanroom rerun did not pass the directive's pre-install visibility gate. The release URL, public docs URL, GitHub release API, and `SHA256SUMS.txt` asset were reachable, and the checksum asset matched the directive. However, the GitHub release page HTML fetched during this pass did not show the required installer SHA256 or expected installer size.

Because the active directive requires the visibility report to confirm that the release page shows SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766` and size `5227476` before installation, I stopped before installing and wrote this failure report.

## Evidence

- Visibility report: `test-comms/reports/20260704-final-release-v032-eab6a31-visibility.md`
- Downloaded checksum file: `test-comms/evidence/20260704-final-release-v032-eab6a31/SHA256SUMS.txt`

## Checks Completed

- Machine/user confirmed as `msi\civic`.
- Repo path confirmed as `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Branch confirmed as `test-comms/cleanroom-coder-tester`.
- Active directive read from `test-comms/ACTIVE_DIRECTIVE.md` at coordination commit `0425dbf`.
- Release URL returned HTTP 200.
- Public docs URL returned HTTP 200.
- GitHub release API listed exactly two assets:
  - `SHA256SUMS.txt`, size `102`
  - `The.Civic.Desk_0.3.2_x64-setup.exe`, size `5227476`
- Downloaded `SHA256SUMS.txt` contains SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`.
- Downloaded `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe`.
- Public docs contained the expected SHA256, `More info`, and `Run anyway`.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker 1 - Required release-page visibility check failed before install

Observed: The GitHub release page HTML fetched from `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2` did not contain SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766` and did not contain size `5227476`.

Expected: The directive's visibility check requires confirming that the release page shows installer SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766`, size `5227476`, and no unreplaced shell variable where the checksum should be.

Additional note: No unreplaced `$hash` placeholder was found in the release page HTML. The release API and checksum asset were internally consistent, and the public docs did show the expected SHA256.

Impact: This prevents an exact run of the directive because a required pre-install release-page confirmation failed before installer testing.

## Not Run

- Installer download and SHA256 verification for `The.Civic.Desk_0.3.2_x64-setup.exe`.
- Clean uninstall/remove prior app state.
- Installed app launch.
- First-run setup.
- Source discovery/import.
- Daily Scan quality checks.
- Drafting/editor workflow.
- Static export and here.now publish.
- Public here.now visitor inspection.

These steps were not run because the active directive's visibility gate failed before install.

## Request For Coder

Update the GitHub release page body so it visibly includes installer SHA256 `1FCDCC2524A16C90A766EFF74ADA8675614FD5A15628749B77678802BCD9B766` and expected size `5227476`, or clarify that the tester should treat the GitHub release API and `SHA256SUMS.txt` asset as the release-page source of truth for this visibility requirement.
