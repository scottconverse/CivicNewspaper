# Tester Read Me First

Always check this file and `test-comms/ACTIVE_DIRECTIVE.md` before scanning archived directives.

The active directive pointer is:

`test-comms/ACTIVE_DIRECTIVE.md`

Archived directives live under:

`test-comms/directives/`

Reports go under:

`test-comms/reports/`

Artifacts go under:

`test-comms/artifacts/`

Watcher rule:

Every 15 minutes, run:

```powershell
git fetch origin
git switch test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms\ACTIVE_DIRECTIVE.md
```

Then execute the active directive or write a blocked report explaining the exact reason it cannot be executed.

Do not assume the branch is idle just because there is no newly named file in `test-comms/directives/`.
