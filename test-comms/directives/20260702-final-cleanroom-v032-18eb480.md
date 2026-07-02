# Final Cleanroom Release Recheck - CivicNewspaper / The Civic Desk v0.3.2

Date: 2026-07-02
Directive id: 20260702-final-cleanroom-v032-18eb480
Coordination repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester

## Stop Old Context First

Stop using any older CivicCast context and stop using any older CivicNewspaper directive.

This directive supersedes:

- test-comms/directives/20260702-final-cleanroom-v032-b0f4ce2.md
- any active instruction that references product commit b0f4ce21ac4e0e2aa2bd9b2f1139aefd25f63e17
- any installer with SHA256 D3C29AB23F740EFED8535320C8CE762E50C3B6131BDD041BCD151AA528D228EE

The b0f4ce2 report was BLOCKED because the app pointed at a default publish folder that did not exist:

```text
C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default
```

This fixed build creates standard app-data folders on clean first run and allows opening the app-owned default site folder before the first compile.

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
Get-Content test-comms/directives/20260702-final-cleanroom-v032-18eb480.md
```

## Product Under Test

Installed app name: The Civic Desk
Package version: 0.3.2
Product branch label: main
Product commit represented by installer:

```text
18eb4805a2d00e0b3efad670bfe041bde6d90724
```

Important: the installer artifact is the source of truth for this cleanroom run. Do not substitute a locally built app or any older installed app.

## Installer Artifact

Install only this artifact:

```text
test-comms/artifacts/20260702-final-cleanroom-v032-18eb480/The Civic Desk_0.3.2_x64-setup.exe
```

Expected NSIS SHA256:

```text
14414BAA3CDF4C6DD0EA80630983F982BBAA749D353ACB7E953D475C5A4E6C8B
```

Expected NSIS size:

```text
5197270
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
8. Verify app-guided AI setup reaches AI Status: Ready without manual dependency installation.
9. Add or discover Longmont starter sources through the app.
10. Run Daily Scan.
11. Generate at least two drafts from different leads.
12. Verify no-source leads are visibly treated as verification assignments:
    - if a lead has no linked source documents, the draft may be created only as needs-verification work,
    - the editor must see an assignment note,
    - it must not be approvable for static publish until linked source evidence exists.
13. Verify Improve for Publication on a linked-source draft:
    - reporter scaffolding must not appear,
    - unsupported evidence IDs must be disabled,
    - the improved draft should include a clear attribution phrase such as "According to..." when linked source evidence exists.
14. Approve only source-linked, attributed, reader-facing copy.
15. Go to Publish.
16. Before compiling, click Open folder on the output folder card.
17. PASS condition for the prior blocker: the default output folder opens or is created/opened successfully. It must not show "The folder or file does not exist" for:

```text
C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default
```

18. Compile/export the publication package.
19. Verify ZIP/package files are present.
20. Publish to here.now using the app flow.
21. Inspect the here.now publication:
    - no duplicate-topic issue,
    - no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD,
    - no public leakage of EDITOR_NOTE, [EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], or [End of Report],
    - public headlines are reader-facing headlines, not lead summaries,
    - public stories are source-grounded and contain attribution where needed.

## Required Reports

Write first visibility report:

```text
test-comms/reports/20260702-final-cleanroom-v032-18eb480-visibility.md
```

Write final report:

```text
test-comms/reports/20260702-final-cleanroom-v032-18eb480-report.md
```

Write evidence under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-18eb480/
```

The final report must include:

- PASS/BLOCKED verdict.
- Installer hash and size observed.
- Installed app path.
- App data path observed.
- Confirmation that the app-created default site folder exists after first launch/setup.
- Whether Open folder before first compile passed.
- Number of sources, leads, evidence items, drafts, approved drafts, publish runs, and published posts.
- ZIP/local artifact path.
- here.now URL.
- Screenshots or logs for setup, AI-ready state, source discovery/import, Daily Scan, Workbench, Publish, ZIP/export, and here.now output.
- Any actionable product failures with exact repro steps and evidence paths.

Do not merge, tag, or push product main. Only write reports/evidence to this coordination branch.
