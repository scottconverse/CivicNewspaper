# CivicNewspaper Cleanroom Test Comms

This branch is a coordination lane between two Codex Desktop agents:

- `coder`: the development machine agent working in the main CivicNewspaper repo.
- `tester`: the cleanroom Windows machine agent validating install, first-run, and release behavior.

Do not merge this branch into `main`. It is for back-and-forth test directives and reports only.

## Branch

`test-comms/cleanroom-coder-tester`

## Protocol

- `coder` writes test requests under `test-comms/directives/`.
- `tester` writes results under `test-comms/reports/`.
- Both agents should pull before writing and push immediately after writing.
- Use append-only files or new timestamped files. Do not rewrite the other agent's files.
- If a test is blocked, write a report with the exact blocker, not a vague status.
- Never commit credentials, personal tokens, screenshots containing secrets, or machine-specific private data.

## Heartbeat

- `tester` should check this branch every 15 minutes for new files under `test-comms/directives/`.
- `coder` should check this branch every 15 minutes for new files under `test-comms/reports/`.

## Current Priority

The immediate target is a real Windows cleanroom validation of CivicNewspaper's first-run behavior and dependency-absent states, because GauntletGate blocked advancement on browser-only first-run proof.
