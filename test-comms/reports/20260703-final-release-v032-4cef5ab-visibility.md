# Visibility Report - v0.3.2 Repair Rerun 4cef5ab

Date: 2026-07-03T23:47Z
Tester machine: `msi\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Branch: `test-comms/cleanroom-coder-tester`
Comms commit: `40251a40f579a414d6c26ba6e236df387aa03360`
Directive: `test-comms/ACTIVE_DIRECTIVE.md` / `test-comms/directives/20260703-final-release-v032-4cef5ab.md`

## Visibility Checks

- Confirmed tester account is `msi\civic`.
- Confirmed repo path is `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Confirmed current branch is `test-comms/cleanroom-coder-tester`.
- Read `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/`.
- GitHub release URL reachable: `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2` returned HTTP 200.
- Public docs URL reachable: `https://scottconverse.github.io/CivicNewspaper/` returned HTTP 200.
- Release page contains installer SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`.
- Release page did not contain unreplaced `$hash` placeholder text.
- Public docs contain installer SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`, `More info`, and `Run anyway`.

## Release Asset Precheck

- Release API lists exactly two assets:
  - `SHA256SUMS.txt`, 102 bytes.
  - `The.Civic.Desk_0.3.2_x64-setup.exe`, 5,232,809 bytes.
- Downloaded installer SHA256:
  - `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`
- Downloaded `SHA256SUMS.txt` content:

```text
0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A  The.Civic.Desk_0.3.2_x64-setup.exe
```

## Evidence

- Downloaded installer: `test-comms/evidence/20260703-final-release-v032-4cef5ab/The.Civic.Desk_0.3.2_x64-setup.exe`
- Downloaded checksum file: `test-comms/evidence/20260703-final-release-v032-4cef5ab/SHA256SUMS.txt`
