# Focused Story Quality and Draft Workflow Rerun - 006c800

Status: ACTIVE

Tester identity: you are the tester on the separate cleanroom machine running as `msi\civic`.

Approved coordination checkout on tester:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use any path under `C:\Users\instynct`; that path belongs to the coder machine and is invalid on the tester machine.

GitHub repo: `https://github.com/scottconverse/CivicNewspaper`

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `006c8009083ea61ba71a365f055b65619d03aed5`

Artifact folder:

`test-comms/artifacts/20260629-story-quality-workflow-rerun-006c800/`

Preferred installer:

`test-comms/artifacts/20260629-story-quality-workflow-rerun-006c800/The Civic Desk_0.2.9_x64-setup.exe`

Expected preferred NSIS SHA256:

`8F6111B3E9432CA81E256EE89E672685230D1FA6525375754150DD4EB916F451`

Fallback installer:

`test-comms/artifacts/20260629-story-quality-workflow-rerun-006c800/The Civic Desk_0.2.9_x64_en-US.msi`

Expected fallback MSI SHA256:

`EA6B6599E9AB2D17F51A01515DC33F66062DFCF7F91653D2AB90AA19BF9862A0`

Expected report:

`test-comms/reports/20260629-story-quality-workflow-rerun-006c800-report.md`

Expected evidence folder:

`test-comms/reports/20260629-story-quality-workflow-rerun-006c800-evidence/`

## Purpose

This is not a full release gauntlet. It is a focused rerun for the latest story-quality and editor-workflow checkpoint.

Verify these fixes:

1. Held drafts clearly expose `Resume Editing` and `Send Back for More Work`.
2. Daily Scan carries story-quality context into the Story Queue: story type, why-now / what-changed, newsworthiness score, and what would make a weak lead publishable.
3. Draft generation respects weak/background/watch context and does not inflate unchanged evergreen civic pages into finished public stories.
4. The editor can still choose what to do. The app may warn, label, and guide, but must not hide or veto the editor's decision.

## Setup

1. Fetch the coordination branch and read `test-comms/ACTIVE_DIRECTIVE.md`.
2. Verify this directive file exists and matches the active pointer.
3. Verify the installer hash before running it.
4. Install the app from the preferred NSIS installer unless it fails; use the MSI fallback only if needed.
5. Use a product-clean test profile or clean app data if practical. Do not reset Windows. Do not manually install Ollama or models. If the app cannot guide AI setup, report it as product failure.

## Test Flow

1. Launch CivicNewspaper.
2. Complete first-run setup for Longmont, Colorado.
3. Allow the app to guide AI/Ollama/model setup if it asks.
4. Add or confirm a source set that includes:
   - official Longmont city sources,
   - at least one general/evergreen official page such as a city council meetings, video archive, service, or department information page,
   - at least one public/social/community source that is readable without login.
5. Run source fetch / scrape / detect.
6. Run Daily Scan.
7. Inspect the Story Queue.

Record whether weak/background/watch leads show editor-facing context, including:

- suggested treatment or story type,
- newsworthiness score,
- why now / what changed,
- what would make it publishable.

If an unchanged evergreen page is promoted as a normal story with no low-newsworthiness/background/watch context, mark this rerun failed and include screenshots/text evidence.

## Draft Workflow Check

1. Pick a lead that is clearly background, watch, verification, low-newsworthiness, or no-current-change if one exists.
2. Generate a draft.
3. Verify the draft is a brief/watch/editor-guidance item or clearly warns that more current verified material is needed.
4. Verify it does not present generic standing civic information as a finished news article.
5. Put the draft on Hold.
6. Confirm the Workbench visibly offers:
   - `Resume Editing`
   - `Send Back for More Work`
7. Click `Send Back for More Work`.
8. Confirm the draft status becomes `needs verification` or equivalent and the UI explains more work is required.
9. Reopen or resume the draft and confirm editing remains possible.

## Output Quality Check

If the run produces at least one genuinely current, specific, evidence-grounded story:

1. Approve one story.
2. Compile/export the static site ZIP.
3. Publish to here.now if the normal anonymous preview flow is available.
4. Report the local ZIP/output path and here.now URL.
5. Confirm public output has no internal scaffolding such as `EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.

If the run does not produce a genuinely current story, do not fabricate one. Report that the correct output is a watch/verification package rather than a finished issue.

## Required Report

Write the report to:

`test-comms/reports/20260629-story-quality-workflow-rerun-006c800-report.md`

Include:

- machine/user/path confirmation,
- installer used and hash result,
- product commit confirmed,
- AI setup result,
- sources used,
- Daily Scan result count,
- examples of at least two leads with their story-quality context,
- draft workflow results for Hold, Resume Editing, and Send Back for More Work,
- whether any evergreen/background page was incorrectly promoted to full story,
- whether any story was approved/exported/published,
- ZIP/output path and here.now URL if produced,
- pass/fail verdict with exact blockers if failed.

Commit the report and evidence with `[skip ci]`.
