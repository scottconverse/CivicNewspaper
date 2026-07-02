# Final Cleanroom Release Recheck - CivicNewspaper / The Civic Desk v0.3.2

Date: 2026-07-02
Directive id: 20260702-final-cleanroom-v032-c93d10f
Coordination repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester

## Stop Old Context First

Stop using any older CivicCast context and stop using any older CivicNewspaper directive.

This directive supersedes:

- test-comms/directives/20260702-final-cleanroom-v032-b0f4ce2.md
- test-comms/directives/20260702-final-cleanroom-v032-18eb480.md
- test-comms/directives/20260702-final-cleanroom-v032-916653b.md
- test-comms/directives/20260702-final-cleanroom-v032-20cfedc.md
- any active instruction that references product commit b0f4ce250374a6e12ac7511fc4ac20b8019579eb
- any active instruction that references product commit 18eb4805a2d00e0b3efad670bfe041bde6d90724
- any active instruction that references product commit 916653b87e09814d4c42bdcb31f91ca7ac4fae09
- any active instruction that references product commit 20cfedc5bc7a4cd45d954e8a55b87fe4a23f1311

The 20cfedc report was BLOCKED because generated linked-source drafts were saved as draft_generated, but opening them in Workbench left the editor actions below an apparent blank Workbench area, and one generated draft contained mojibake marker text from an event bullet.

This fixed build changes that failure class:

- Workbench now shows a compact top action strip immediately under the Story Workbench header with current draft status, Improve for Publication, editorial attestation, and approval controls.
- Generated drafts are normalized for common mojibake marker sequences before they are saved or shown in selected draft state.
- Backend public-output mojibake repair explicitly handles the triple-encoded bullet family seen in the 20cfedc report.
- Existing linked-source/no-source draft gating from 20cfedc remains in force: attributed linked-source drafts can remain draft_generated, while no-source drafts remain needs_verification assignments.

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
Get-Content test-comms/directives/20260702-final-cleanroom-v032-c93d10f.md
```

## Product Under Test

Installed app name: The Civic Desk
Package version: 0.3.2
Product branch label: main
Product commit represented by installer:

```text
c93d10f3cd1a913dcb5fb0c846126303c26a8c19
```

Important: the installer artifact is the source of truth for this cleanroom run. Do not substitute a locally built app or any older installed app.

## Installer Artifact

Install only this artifact:

```text
test-comms/artifacts/20260702-final-cleanroom-v032-c93d10f/The Civic Desk_0.3.2_x64-setup.exe
```

Expected NSIS SHA256:

```text
96BC3D9EAF499765887F5AD82D09CD8BD9B22691AD84ACCFA7EBA68A6A777754
```

Expected NSIS size:

```text
5200988
```

If hash or size does not match, stop and report BLOCKED.

## Clean Wipe Boundary

Wipe only CivicNewspaper / The Civic Desk product state and related local AI/runtime state. Do not reset Windows and do not wipe the Windows user account.

Remove or verify absent:

- any running civicnews process
- installed The Civic Desk app via its uninstaller if present
- %APPDATA%\com.scottconverse.civicdesk
- %LOCALAPPDATA%\com.scottconverse.civicdesk
- %LOCALAPPDATA%\The Civic Desk
- product-owned Ollama/runtime/model folders if created by this app
- %USERPROFILE%\.ollama only if it exists from prior CivicNewspaper testing on this cleanroom tester account

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
11. Wait until the Daily Scan UI is no longer running. Then inspect the app DB and verify the newest daily_scan_runs row is not left in run_status in_progress after leads are present.
12. Generate at least two drafts from different leads.
13. Verify no-source leads are visibly treated as verification assignments:
    - if a lead has no linked source documents, the draft may be created only as needs-verification work,
    - the editor must see an assignment note,
    - it must not suggest invented outlets, reporters, staff names, or other unsupported people to contact,
    - it must not be approvable for static publish until linked source evidence exists.
14. Verify linked-source generated drafts:
    - each must include a clear attribution phrase such as "According to..." when linked source evidence exists,
    - each must use valid evidence citation syntax such as [Source](evidence:15),
    - none may include malformed citation syntax such as [Source(evidence:15)],
    - none may introduce unsupported named entities or claims not present in the linked evidence,
    - each should remain editable as draft_generated when it is attributed and source-grounded,
    - titles and bodies must not include mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
15. Open generated drafts from the Workbench draft picker:
    - clicking a draft must show the Story Workbench editor,
    - a top action strip must be visible near the top of the Workbench with Improve for Publication and approval controls,
    - Improve for Publication and approval actions must be visible or reachable without hunting below a blank-looking fold,
    - the UI must not appear blank or bounce to unrelated navigation.
16. Verify Improve for Publication on a linked-source draft:
    - reporter scaffolding must not appear,
    - unsupported evidence IDs must be disabled,
    - the improved draft should remain attributed and source-grounded,
    - the improved draft should remain free of mojibake marker code points.
17. Approve only source-linked, attributed, reader-facing copy.
18. Go to Publish.
19. Before compiling, click Open folder on the output folder card.
20. Confirm the default output folder opens or is created/opened successfully. It must not show "The folder or file does not exist" for:

```text
C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default
```

21. Compile/export the publication package.
22. Verify ZIP/package files are present.
23. Publish to here.now using the app flow.
24. Inspect the here.now publication:
    - no duplicate-topic issue,
    - no mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD,
    - no public leakage of EDITOR_NOTE, [EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], or [End of Report],
    - public headlines are reader-facing headlines, not lead summaries,
    - public stories are source-grounded and contain attribution where needed.

## Required Reports

Write first visibility report:

```text
test-comms/reports/20260702-final-cleanroom-v032-c93d10f-visibility.md
```

Write final report:

```text
test-comms/reports/20260702-final-cleanroom-v032-c93d10f-report.md
```

Write evidence under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/
```

The final report must include:

- PASS/BLOCKED verdict.
- Installer hash and size observed.
- Installed app path.
- App data path observed.
- Confirmation that the app-created default site folder exists after first launch/setup.
- Whether Open folder before first compile passed.
- The newest daily_scan_runs row after Daily Scan and whether run_status is completed.
- Number of sources, leads, evidence items, drafts, approved drafts, publish runs, and published posts.
- The full text of every generated draft the tester considered for approval.
- Whether linked-source drafts were editable/approvable and whether no-source drafts stayed assignments.
- Whether Workbench draft picker opens a visible editor with the top action strip, Improve, attestation, and approval actions.
- Whether generated drafts, improved drafts, ZIP extract, RSS/share artifacts, and here.now pages are free of mojibake marker code points.
- ZIP/local artifact path.
- here.now URL.
- Screenshots or logs for setup, AI-ready state, source discovery/import, Daily Scan, Workbench, Publish, ZIP/export, and here.now output.
- Any actionable product failures with exact repro steps and evidence paths.

Do not merge, tag, or push product main. Only write reports/evidence to this coordination branch.
