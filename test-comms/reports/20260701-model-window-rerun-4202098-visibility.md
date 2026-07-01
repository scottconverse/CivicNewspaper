# Tester Visibility Report - model window rerun 4202098

Date: 2026-07-01T06:27:47Z
Tester machine: Windows 11 Home 10.0.26200, 13th Gen Intel(R) Core(TM) i7-13620H, 16 GB RAM
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Coordination HEAD: 6da0b59 test-comms: add model window rerun 4202098 [skip ci]
Directive: test-comms/directives/20260701-model-window-rerun-4202098.md

## Visibility

I fetched and fast-forwarded `test-comms/cleanroom-coder-tester`, read `test-comms/ACTIVE_DIRECTIVE.md`, and confirmed it points to:

`test-comms/directives/20260701-model-window-rerun-4202098.md`

## Artifact Verification

Installer:

`test-comms/artifacts/20260701-model-window-rerun-4202098/The Civic Desk_0.3.1_x64-setup.exe`

Expected SHA256:

`7C934848901FAD43DF0D5B88E59F4A62B958EE5BA0DBF740287DB3F6C413F481`

Observed SHA256:

`7C934848901FAD43DF0D5B88E59F4A62B958EE5BA0DBF740287DB3F6C413F481`

Expected size:

`5629802`

Observed size:

`5629802`

Result: PASS. The installer artifact matches the directive.

## Next Action

Proceeding with the requested product clean wipe, silent NSIS install, normal installed-app launch, and Step 3 model-download gate.
