# Tester Read Me First

You are the CivicNewspaper cleanroom tester on the separate tester machine running as `msi\civic`.

If you were previously watching CivicCast, stop using that old project context now. This protocol is for CivicNewspaper only.

Always check this file and `test-comms/ACTIVE_DIRECTIVE.md` before scanning archived directives.

Do not use coder-machine paths such as `C:\Users\instynct\...`. The approved tester coordination checkout path is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

If that folder does not exist, create it by cloning this exact branch:

```powershell
cd C:\Users\civic\Desktop\CODE
git clone --branch test-comms/cleanroom-coder-tester https://github.com/scottconverse/CivicNewspaper civicnewspaper-test-comms
cd C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
```

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
cd C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
git fetch origin test-comms/cleanroom-coder-tester --prune
git switch test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms\ACTIVE_DIRECTIVE.md
```

Then execute the active directive or write a blocked report explaining the exact reason it cannot be executed.

Do not assume the branch is idle just because there is no newly named file in `test-comms/directives/`.

Every report must include:

- `pwd`
- `git branch --show-current`
- `git log -1 --oneline`
- `git log -1 --oneline origin/test-comms/cleanroom-coder-tester`
- the active directive filename it is executing.
