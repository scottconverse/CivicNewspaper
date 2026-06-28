# Coder/Tester Protocol

## Roles

`coder` owns source changes, fixes, and new directives.

`tester` owns cleanroom execution, screenshots, logs, receipts, and user-facing findings.

## File Rules

Use these folders only:

- `test-comms/directives/` for coder-to-tester instructions.
- `test-comms/reports/` for tester-to-coder results.
- `test-comms/prompts/` for reusable prompts.

Use filenames like:

- `YYYYMMDD-HHMM-coder-directive-short-title.md`
- `YYYYMMDD-HHMM-tester-report-short-title.md`

Each report should include:

- Machine profile.
- Branch/commit tested.
- Exact commands run.
- Screenshots/log paths.
- Pass/fail/blocked result.
- Findings by severity.
- What the tester needs from coder next.

## Git Safety

Before writing:

```powershell
git fetch origin
git switch test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
```

After writing:

```powershell
git add test-comms
git commit -m "test-comms: add <short description> [skip ci]"
git push origin test-comms/cleanroom-coder-tester
```

If pull fails because the other agent pushed first, stop and resolve by pulling/rebasing without deleting the other agent's files.

## No-Go Items

- Do not merge this branch.
- Do not tag releases from this branch.
- Do not upload provider credentials.
- Do not use real external publishing unless a directive explicitly asks for it.
- Do not hide failures. A clean "blocked because X" is useful.
