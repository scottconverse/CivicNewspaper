# Final Cleanroom Release Check - CivicNewspaper / The Civic Desk v0.3.2

Date: 2026-07-02
Directive id: 20260702-final-cleanroom-v032-b0f4ce2
Coordination repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester

## Stop Old Context First

Stop using any older CivicCast context and stop using any older CivicNewspaper directive.

This directive supersedes:

- test-comms/directives/20260702-webview-identity-rerun-4bede5c.md
- any active instruction that references product commit 4bede5c6773189e24c8aa05a105e503b93111fca
- any installer named The Civic Desk_0.3.1_x64-setup.exe

The 4bede5c report tested an obsolete v0.3.1 installer. The current test target is the v0.3.2 installer below.

## Tester Machine Coordination Path

Use this path on the tester machine:

```text
C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
```

Do not use this coder-machine path on the tester:

```text
C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms
```

## Required Sync Commands

Run these before testing:

```powershell
git fetch origin test-comms/cleanroom-coder-tester --prune
git checkout test-comms/cleanroom-coder-tester
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms/ACTIVE_DIRECTIVE.md
Get-Content test-comms/directives/20260702-final-cleanroom-v032-b0f4ce2.md
```

## Product Under Test

Installed app name: The Civic Desk
Package version: 0.3.2
Product branch label: main
Product commit represented by installer:

```text
b0f4ce21ac4e0e2aa2bd9b2f1139aefd25f63e17
```

Important: the installer artifact is the source of truth for this cleanroom run. Do not substitute a locally built app or any older installed app.

## Installer Artifact

Install only this artifact:

```text
test-comms/artifacts/20260702-final-cleanroom-v032-b0f4ce2/The Civic Desk_0.3.2_x64-setup.exe
```

Expected NSIS SHA256:

```text
D3C29AB23F740EFED8535320C8CE762E50C3B6131BDD041BCD151AA528D228EE
```

Expected NSIS size:

```text
5203001
```

If hash or size does not match, stop and report BLOCKED.

## Clean Wipe Boundary

Wipe only CivicNewspaper / The Civic Desk product state and related local AI/runtime state. Do not reset Windows and do not wipe the Windows user account.

Remove or verify absent:

- any running `civicnews` process
- installed The Civic Desk app via its uninstaller if present
- `%APPDATA%\com.scottconverse.civicdesk`
- `%LOCALAPPDATA%\com.scottconverse.civicdesk`
- `%LOCALAPPDATA%\The Civic Desk`
- product-owned Ollama/runtime/model folders if created by this app
- `%USERPROFILE%\.ollama` only if it exists from prior CivicNewspaper testing on this cleanroom tester account

Do not manually install Ollama, models, Node, Rust, npm dependencies, or source builds. The tester may use normal Windows tools and PowerShell only to drive and observe the packaged installer, exactly as a user/tester would.

## Test City

Use Longmont, Colorado.

Suggested identity values:

- Publication name: Longmont Cleanroom Beta Desk
- Editor name: Cleanroom Tester
- City: Longmont
- State: CO

## Required Test Flow

Run this as a true cleanroom user flow from the packaged installer:

1. Pull the coordination branch and read ACTIVE_DIRECTIVE.md plus this directive.
2. Verify installer hash and size.
3. Clean wipe product state within the boundary above.
4. Install only the directive NSIS artifact.
5. Launch the installed app normally from the installed path.
6. Confirm native window title and visible product identity are The Civic Desk.
7. Complete first-run identity setup.
8. Specifically verify the identity input blocker is gone:
   - ordinary keyboard or clipboard entry remains visible before advancing,
   - the Longmont starter profile, if used, visibly populates fields,
   - clicking Next advances out of Identity,
   - the database contains identity.newsroom_name, identity.editor_name, identity.city, identity.state, and onboarding.step or onboarding completion values.
9. Continue through app-guided AI setup.
   - The app must read hardware/profile and select an appropriate model.
   - The app must warn the user what it is doing.
   - The app must show progress so setup does not look hung.
   - Do not manually install Ollama or pull models for the app.
10. If AI setup succeeds, verify dashboard/workspace state.
11. Add or accept starter Longmont sources.
12. Run Daily Scan.
13. Verify source discovery/intake produces real Longmont leads from public readable sources.
14. Verify duplicate-topic suppression, source grounding, and evidence linkage.
15. Use the editorial workflow:
   - generate drafts,
   - review/edit,
   - approve,
   - send back for more work,
   - put on hold,
   - cut,
   - return from hold if available.
16. Produce a reader-facing publication with 5 to 10 stories or briefs if available from real current Longmont civic material.
17. Reject or downgrade evergreen/static pages that are not new news.
18. Verify public output does not leak reporter scaffolding such as EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], or [End of Report].
19. Verify public output has no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
20. Export the ready-to-publish ZIP/package from the app's normal output location.
21. Publish anonymously to here.now.
22. Report the here.now URL, local output path, ZIP/package path, installer hash, app version, and full pass/fail findings.

## Evidence To Capture

Write all evidence under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/
```

Capture:

- machine profile
- installer hash/size proof
- clean wipe log
- install log
- screenshots for identity, AI setup/progress, dashboard/workspace, source setup, Daily Scan, Story Queue, Workbench, Publish, here.now result
- database/settings snapshot after identity setup
- source list snapshot
- leads/story queue snapshot
- editorial workflow evidence
- output ZIP/package path and file hash if available
- here.now URL and publish receipt if available
- screenshots of any blocker/error

## Required Reports

First write this visibility report:

```text
test-comms/reports/20260702-final-cleanroom-v032-b0f4ce2-visibility.md
```

Then write this final report:

```text
test-comms/reports/20260702-final-cleanroom-v032-b0f4ce2-report.md
```

The final report must include:

- PASS or BLOCKED at the top
- exact installer path, SHA256, and size
- app version observed
- tester machine/user/path
- clean wipe actions performed
- every required flow step with pass/fail
- here.now URL if publish succeeded
- ZIP/package path if export succeeded
- output quality notes
- full list of blockers/defects with evidence paths

## Pass Bar

PASS only if the clean tester machine can install The Civic Desk v0.3.2 from the directive artifact, complete first-run identity setup, complete app-guided AI/runtime setup without tester-installed dependencies, ingest Longmont sources, run Daily Scan, exercise the editorial workflow, export a publication package, publish to here.now, and provide report/evidence/URL/package paths.

If any blocking issue appears, stop at the blocker, preserve evidence, write BLOCKED, and push the report/evidence.
