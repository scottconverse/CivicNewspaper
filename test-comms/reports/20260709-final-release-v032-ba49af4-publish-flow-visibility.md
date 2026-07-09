# Final Release Visibility Check - v0.3.2 ba49af4

Tester branch: `test-comms/cleanroom-coder-tester`  
Directive: `Final Cleanroom Release Verification - The Civic Desk v0.3.2 ba49af4 Publish Flow Rerun`  
Checked at: 2026-07-09 UTC  
Result: PASS for release visibility and installer integrity.

## Target

- Product commit under test: `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`
- Release/docs commit: `4ba609690e0094c453b4a2852fd209cc8c8b2c83`
- Installer: `The.Civic.Desk_0.3.2_x64-setup.exe`
- Expected size: `5250809`
- Expected SHA256: `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`

## Evidence

Evidence folder:
`test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/`

Key files:

- `release-api.json`
- `public-docs.html`
- `SHA256SUMS.txt`
- `visibility-download-state.json`

## Checks Performed

- GitHub release API was reachable.
- Public docs page was reachable.
- Release exposed one installer asset and one checksum asset.
- Installer downloaded successfully from the public release asset.
- Downloaded installer size was `5250809` bytes.
- Downloaded installer SHA256 matched the expected hash.
- `SHA256SUMS.txt` contained the expected hash and installer filename.
- Public docs contained the expected hash, expected size, Windows SmartScreen guidance (`More info`, `Run anyway`), and Windows-only beta copy.
- Public docs did not contain the known stale hashes checked during this run.
- Release body contained the expected product commit, hash, and size.

## Visibility Result

PASS. The public release and documentation were visible and internally consistent for the expected v0.3.2 ba49af4 installer.
