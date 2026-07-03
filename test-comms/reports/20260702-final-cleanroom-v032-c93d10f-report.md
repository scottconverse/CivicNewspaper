# BLOCKED - Final Cleanroom Release Recheck - Civic Desk v0.3.2 c93d10f

Directive: `20260702-final-cleanroom-v032-c93d10f`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `c93d10f3cd1a913dcb5fb0c846126303c26a8c19`
Tester machine/user/path: Windows tester as `civic`, `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Verdict

BLOCKED.

The build passed clean install, first-run identity setup, app-guided AI setup, Daily Scan completion, linked-source draft generation, no-source verification assignment gating, and Workbench top-action-strip visibility. It is blocked at compile/export: after an approved, source-linked, attested draft existed, `Compile site` did not write the static package, did not create a publish run, did not create `site-package.zip`, and did not expose `Publish to here.now`.

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-c93d10f/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256 observed: `96BC3D9EAF499765887F5AD82D09CD8BD9B22691AD84ACCFA7EBA68A6A777754`
- Size observed: `5200988`
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App data path observed: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`

## Clean Wipe / Install

Performed within directive boundary:

- Stopped previous `civicnews` / product runtime processes when present.
- Ran previous The Civic Desk uninstaller if present.
- Removed `%APPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\The Civic Desk`.
- Removed prior `%USERPROFILE%\.ollama` from previous CivicNewspaper testing.
- Installed only the directive NSIS artifact.
- Launched installed app normally from the installed path.

Evidence: `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/install-clean-launch.log`

## Flow Results

1. Pull/read active directive: PASS.
2. Verify installer hash and size: PASS.
3. Clean wipe product/runtime state: PASS.
4. Install directive NSIS artifact: PASS.
5. Launch installed app normally: PASS.
6. Confirm native window title and product identity: PASS, title `The Civic Desk`.
7. Complete first-run identity setup: PASS. The requested values persisted.
8. App-guided AI setup reaches AI Status Ready without manual dependency installation: PASS.
9. Add/discover Longmont starter sources through the app: PASS.
10. Run Daily Scan: PASS.
11. Newest `daily_scan_runs` row after leads present is not left `in_progress`: PASS.
12. Generate at least two drafts from different leads: PASS.
13. No-source verification assignment behavior: PASS.
14. Linked-source generated draft checks: PASS WITH FINDINGS.
15. Open generated drafts from Workbench draft picker: PASS.
16. Verify Improve for Publication on linked-source draft: PARTIAL/FAIL. The top action was visible and updated the editor UI, but introduced malformed/unsupported citation text and did not persist until manual save/edit.
17. Approve only source-linked, attributed, reader-facing copy: PASS after manual editor cleanup.
18. Go to Publish: PASS.
19. Before compiling, click Open folder on output folder card: PASS.
20. Confirm default output folder opens/creates: PASS.
21. Compile/export publication package: BLOCKED.
22. Verify ZIP/package files are present: NOT RUN; no ZIP was produced.
23. Publish to here.now using app flow: NOT RUN; here.now publish control was not exposed after compile no-op.
24. Inspect here.now publication: NOT RUN.

## Database Snapshot

From `final-db-summary.json`:

- `sources`: 9
- `daily_scan_runs`: 1
- `daily_scan_leads`: 17
- `leads`: 20
- `evidence_items`: 31
- `lead_evidence`: 9
- `drafts`: 3
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 93
- `model.selected`: `phi4-mini:latest`
- `identity.newsroom_name`: `Longmont Cleanroom Beta Desk`
- `identity.editor_name`: `Cleanroom Tester`
- `identity.city`: `Longmont`
- `identity.state`: `CO`

Newest `daily_scan_runs` row:

```json
{
  "id": 1,
  "started_at": "2026-07-02T23:57:17.225356200+00:00",
  "completed_at": "2026-07-02T23:59:48.977054600+00:00",
  "run_status": "completed"
}
```

## Generated Drafts Considered For Approval

Full draft rows are in `test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/drafts-full.jsonl`.

### Draft 1

- Draft id: 1
- Lead id: 15
- Status: `draft_generated`
- Format: `watch`
- Title: `St. Vrain Valley Schools Receives ASBO Certificate of Excellence for 22nd Consecutive Year: The`
- Content:

```text
According to the linked source, August 1, 2026 &ndash; &ndash; Aug 12 Board of Education Regular Meeting August 12, 2026 &ndash; 6:00 p. [Source](evidence:15).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It was linked, attributed, and editable, but the content did not match the ASBO lead topic well enough for approval.

### Draft 2

- Draft id: 2
- Lead id: 14
- Status after editor approval: `ready_to_publish`
- Format: `watch`
- Attested by: `Cleanroom Tester`
- Attested at: `2026-07-03T00:12:31.250811800+00:00`
- Final title: `St. Vrain highlights experiential academic programs`
- Final content:

```text
According to the linked source, St. Vrain Valley Schools describes academic programs that include experiential learning through the Innovation Center and career-focused classes through the Career Elevation and Technology Center. [Source](evidence:13).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a new district decision; watch for a newly posted board action, program launch date, budget item, or district announcement before expanding it into a full story.
```

Decision: approved after manual editor cleanup. It is source-linked, attributed, reader-facing, and uses valid citation syntax.

### Draft 3

- Draft id: 3
- Lead id: 20
- Status: `needs_verification`
- Format: `watch`
- Missing evidence notes: `No source documents are linked to this lead yet. Treat this as a verification assignment until public source material is attached or cited.`
- Title: `Longmont Residents Get Free Internet Access for July: The City of Longmont is providing`
- Content:

```text
No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.
```

Decision: not approved and not publishable. This passed the no-source assignment requirement. The UI showed `Verification assignment`, `Linked Sources (0)`, and warning text saying it should not be approved until source material is attached or cited. No unsupported outlets, reporters, staff names, or contact people appeared.

## Workbench / Improve / Approval

- Workbench draft picker opened a visible editor.
- Top action strip was visible immediately near the header with:
  - current status `Drafting`
  - selected draft title
  - `Improve for Publication`
  - `Ready`
  - `I reviewed this story.`
  - `Approve`
- The lower Workbench controls were also visible/reachable: article body, linked sources, `Plain Language Rewrite`, `Mark Ready for Review`, attestation checkbox, and `Approve for Static Publish`.
- `Improve for Publication` updated the editor UI for draft 2, but the improved text included malformed/unsupported citation text `(evidence:13)` and future/unsupported phrasing. The tester manually cleaned the title/body, saved the draft, then approved.
- Draft 2 moved to `ready_to_publish` with `Cleanroom Tester` attestation.

## Output / Publish

- App-created default output folder exists: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Open folder before compile: PASS.
- ZIP/local artifact path expected: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- ZIP/local artifact path observed: absent.
- Files observed after compile attempts:
  - `.civicdesk-output`
  - `print.css`
  - `styles.css`
- Missing after compile attempts:
  - `index.html`
  - article HTML
  - feed/share files
  - `site-package.zip`
- here.now URL: none; blocked before publish.
- Publish runs: 0
- Published posts: 0

## Mojibake / Public Output

- Draft title/body scan for mojibake marker code points `Ã`, `Â`, `â`, and `�`: PASS for all 3 drafts.
- ZIP extract/RSS/share artifacts: NOT RUN; no ZIP/package was produced.
- here.now pages: NOT RUN; publish was blocked.

## Blocking Defect

### BLOCKER-1: Compile site does not produce package after approved draft exists

Observed: With draft 2 in `ready_to_publish` and attested by `Cleanroom Tester`, Publishing showed the output folder and `Compile site`. `Open folder` worked and the default output folder existed. Invoking `Compile site` through UI Automation and then by direct visible click did not create a publish run, did not create `site-package.zip`, did not write index/article/feed/share files, and did not expose `Publish to here.now`.

Expected: `Compile site` should write the static site files, create `site-package.zip`, record a publish run, and expose the here.now publish flow.

Impact: Blocks ZIP/package verification, here.now publish, and public-output inspection.

Repro:

1. Clean install c93d10f NSIS artifact.
2. Complete Longmont setup and app-guided AI setup.
3. Let Daily Scan complete.
4. Generate linked-source drafts.
5. Open Workbench, edit/approve a source-linked draft so it reaches `ready_to_publish`.
6. Open Publishing, click `Open folder`, then `Review compile checklist`, then `Compile site`.
7. Observe no publish run and no package/ZIP.

Evidence: `screenshot-08-publish-after-compile-failed.png`, `screenshot-09-compile-button-noop.png`, `publish-folder-check.txt`, `final-db-summary.json`.

## Additional Findings

### FINDING-1: Improve for Publication introduced malformed/unsupported citation text

Observed: `Improve for Publication` updated the editor UI for draft 2 with text containing `(evidence:13)` rather than valid `[Source](evidence:13)` citation syntax, plus phrasing about future details/costs that was not suitable for approval as-is.

Expected: Improved draft should remain attributed, source-grounded, and use valid citation syntax only.

Impact: Requires editor cleanup before approval.

Evidence: UI value inspection after `Improve for Publication`; final cleaned draft in `drafts-full.jsonl`.

### FINDING-2: One generated linked-source draft did not match the lead topic

Observed: Draft 1 was generated from an ASBO-recognition lead but its body referenced an August Board of Education meeting instead.

Expected: Generated draft content should stay aligned to the selected lead and linked evidence.

Impact: The draft was not suitable for approval.

Evidence: `drafts-full.jsonl`.

## Evidence Folder

All screenshots/logs/snapshots for this run are under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-c93d10f/
```

Key evidence includes:

- `installer-verify.txt`
- `install-clean-launch.log`
- `db-after-ai-ready.txt`
- `db-after-scan-wait120.txt`
- `drafts-full.jsonl`
- `final-db-summary.json`
- `environment.json`
- `publish-folder-check.txt`
- `screenshot-01-launch.png`
- `screenshot-02-after-identity-next.png`
- `screenshot-03-ai-ready.png`
- `screenshot-06-workbench-top-action-strip.png`
- `screenshot-07-nosource-assignment.png`
- `screenshot-08-publish-after-compile-failed.png`
- `screenshot-09-compile-button-noop.png`

## Result

BLOCKED. Build `c93d10f` improved setup identity input, Workbench opening, top action-strip visibility, and mojibake normalization in generated drafts. It cannot pass the directive because `Compile site` does not produce the static package/ZIP or enable here.now publishing after a valid approved draft exists.
