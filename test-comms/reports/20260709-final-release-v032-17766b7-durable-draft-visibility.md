# Visibility Report - The Civic Desk v0.3.2 17766b7 Durable Draft

**Result: FAIL**

## Machine And Branch

- Machine/user: `MSI\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Tester branch head at start of run: `d49566957fce28d07bb7e901c1c443cfee0b2f77`
- Active directive read: `test-comms/ACTIVE_DIRECTIVE.md`
- Active directive title: `Final Cleanroom Release Verification - The Civic Desk v0.3.2 17766b7 Durable Draft Persistence Rerun`

## Release Visibility Checks

- Release URL reachable: yes
- Public docs URL reachable: yes
- Windows installer asset count: `1`
- Checksum asset count: `1`
- Installer asset name: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Installer asset size: `5260917`
- Expected SHA256: `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`
- Downloaded installer SHA256: `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`
- `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe`: yes
- `SHA256SUMS.txt` contains expected SHA256: yes

## Public Docs Checks

The public docs page contains:

- Expected installer SHA256 `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`
- `More info`
- `Run anyway`
- Windows-only beta language

The public docs page does **not** contain stale SHA256 `E7B620C4D51837DDD43028B511E396643EE9A67D1CD23DC0B59BC5442277DCD7`.

## Visibility Finding

### Major - Release body did not contain the expected product build commit

The directive requires the release page to show product commit `17766b7ccb0cc744522090e28997b764676ce1c5`. The GitHub release API body fetched during this run did not contain that full commit string, even though the installer size/hash and checksum asset were correct.

Evidence:

- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/visibility-download-state.json`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/release-api.json`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/public-docs.html`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/SHA256SUMS.txt`
