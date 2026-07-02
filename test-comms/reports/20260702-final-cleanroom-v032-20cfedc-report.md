# BLOCKED - Final Cleanroom Release Recheck - Civic Desk v0.3.2 20cfedc

Directive: `20260702-final-cleanroom-v032-20cfedc`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `20cfedc5bc7a4cd45d954e8a55b87fe4a23f1311`
Tester machine/user/path: Windows tester as `civic`, `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Verdict

BLOCKED.

The build passed installer verification, clean install, first-run Longmont setup, app-guided local AI setup, source discovery/Daily Scan mechanics, and generated two linked-source drafts that persisted as `draft_generated`. It is blocked at the Workbench draft-open requirement: opening a generated draft renders a blank Story Workbench content area, so Improve for Publication and approval actions are not visible/reachable. Because there are zero approved drafts, compile/export, ZIP verification, here.now publish, and public-output inspection were not run.

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-20cfedc/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `B41AF31919D2271DF2200F6B449CE1B6FB21871826979CF71601730AB97D5C1D`
- Size: `5199754`
- App observed: `The Civic Desk` v0.3.2
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App data path observed: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`

## Clean Wipe / Install

Performed within directive boundary:

- Ran previous The Civic Desk uninstaller if present.
- Removed `%APPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\The Civic Desk`.
- Removed prior `%USERPROFILE%\.ollama` from previous CivicNewspaper testing.
- Installed only the directive NSIS artifact.
- Launched installed app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.

Evidence: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/install-clean-launch.log`

## Flow Results

1. Pull/read active directive: PASS.
2. Verify installer hash and size: PASS.
3. Clean wipe product/runtime state: PASS.
4. Install directive NSIS artifact: PASS.
5. Launch installed app normally: PASS.
6. Confirm native window title and product identity: PASS, window title `The Civic Desk`.
7. Complete first-run identity setup: PASS.
8. App-guided AI setup reaches AI Status Ready without manual dependency installation: PASS.
9. Add/discover Longmont starter sources through the app: PASS.
10. Run Daily Scan: PASS.
11. Newest daily_scan_runs row after leads present is not left `in_progress`: PASS.
12. Generate at least two drafts from different leads: PASS.
13. No-source verification assignment behavior: NOT FULLY TESTED before Workbench blocker. The run focused on two linked-source leads and stopped when opening generated drafts from Workbench rendered blank.
14. Linked-source generated draft checks: PARTIAL/PASS for persistence and citation syntax, FAIL for mojibake in one draft.
    - Both generated drafts persisted as `draft_generated`.
    - Both drafts include `According to the linked source`.
    - Both drafts use valid `[Source](evidence:...)` syntax.
    - Draft 1 contains mojibake marker text `â€¢` in title/content, from the source event bullet.
15. Open generated drafts from Workbench draft picker: BLOCKED.
    - Clicking/opening a generated draft leaves Story Workbench blank except for the page header and subtitle.
    - Improve for Publication and approval actions were not visible/reachable.
16. Improve for Publication on a linked-source draft: NOT RUN, blocked by blank Workbench state.
17. Approve source-linked attributed copy: NOT RUN, blocked by blank Workbench state.
18. Go to Publish: NOT RUN after this blocker.
19. Open folder before first compile: NOT RUN after this blocker.
20. Confirm default output folder opens/creates: NOT RUN after this blocker.
21. Compile/export publication package: NOT RUN.
22. Verify ZIP/package files: NOT RUN.
23. Publish to here.now: NOT RUN.
24. Inspect here.now publication: NOT RUN.

## Database Snapshot

From `final-db-summary.json`:

- `sources`: 18
- `daily_scan_runs`: 1
- `daily_scan_leads`: 19
- `leads`: 22
- `evidence_items`: 31
- `lead_evidence`: 11
- `drafts`: 2
- `publish_runs`: 0
- `published_posts`: 0
- `ai.setup_skipped`: `false`
- `model.selected`: `phi4-mini:latest`
- `identity.newsroom_name`: `Longmont Civic Desk`
- `identity.editor_name`: `Local Editor`
- `identity.city`: `Longmont`
- `identity.state`: `CO`
- `onboarding.step`: `5`
- `onboarding_complete`: `1`

Newest `daily_scan_runs` row in the snapshot is completed; no stale `in_progress` run remained after leads were present.

## Generated Drafts Considered For Approval

Full draft rows are in `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/drafts-full.jsonl`.

### Draft 1

- Draft id: 1
- Lead id: 22
- Status: `draft_generated`
- Format: `watch`
- Title: `Longmont official city website: View Events Summer Concert Series: 2MX2 Thursday, July 2 â€¢ 7`
- Content:

```text
According to the linked source, View Events Summer Concert Series: 2MX2 Thursday, July 2 â€¢ 7 pm - 8:30 pm 400 Quail Rd. [Source](evidence:3).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It is linked and attributed, but contains mojibake marker text `â€¢` and Workbench blanked before Improve/approval could be verified.

### Draft 2

- Draft id: 2
- Lead id: 21
- Status: `draft_generated`
- Format: `watch`
- Title: `Longmont Area Chamber of Commerce: May 27, 2026 The Colorado General Assembly wrapped up its`
- Content:

```text
According to the linked source, May 27, 2026 The Colorado General Assembly wrapped up its 2026 session, and as always, there was no shortage of action affecting our business community. [Source](evidence:19).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It is linked and attributed, but Workbench blanked before Improve/approval could be verified.

## Output / Publish

- Approved drafts: 0
- ZIP/local artifact path: none; blocked before export
- here.now URL: none; blocked before publish
- Publish runs: 0
- Published posts: 0
- Public-output audit: not reached

## Blocking Defects

### BLOCKER-1: Opening generated drafts from Workbench renders blank content

Observed: After generating linked-source drafts, opening a generated draft from Story Workbench resulted in a blank Workbench content area. The page header/subtitle remained visible, but the draft editor, Improve for Publication action, and approval actions were not visible/reachable.

Expected: Clicking a generated draft from the Workbench draft picker should focus the selected draft editor and expose Improve for Publication and approval actions.

Impact: Blocks Improve for Publication, approval, compile/export, ZIP verification, here.now publishing, and public-output inspection.

Repro: Complete setup, run Daily Scan, generate linked-source drafts, open Workbench, then open one generated draft.

Evidence: `screenshot-current-2328z.png`, `drafts-full.jsonl`, `final-db-summary.json`.

### BLOCKER-2: Generated linked-source draft contains mojibake marker text

Observed: Draft 1 title/content contains `â€¢`, which is mojibake for a bullet character.

Expected: Generated drafts and any eventual public output must not contain mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.

Impact: Even if Workbench approval were unblocked, this draft would fail the public-output quality bar unless improved/cleaned before publication.

Evidence: `drafts-full.jsonl`.

## Evidence Folder

All screenshots/logs/snapshots for this run are under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/
```

Key evidence includes:

- `install-clean-launch.log`
- `db-after-ai-ready.txt`
- `db-after-scan-wait120.txt`
- `final-db-summary.json`
- `drafts-full.jsonl`
- `screenshot-01-launch.png`
- `screenshot-02-identity-filled.png`
- `screenshot-03-after-longmont-next.png`
- `screenshot-04-ai-ready-wait150.png`
- `screenshot-05-story-queue.png`
- `screenshot-06-first-lead.png`
- `screenshot-08-after-generate-draft1.png`
- `screenshot-11-second-source-lead.png`
- `screenshot-13-second-draft-gate.png`
- `screenshot-current-2328z.png`

## Result

BLOCKED. Build `20cfedc` improved the previous `916653b` state by producing linked-source `draft_generated` drafts and by using a concrete starter identity (`Longmont Civic Desk`). The run cannot pass because Workbench still blanks when opening generated drafts, preventing Improve/approval/export/publish, and one generated linked-source draft contains mojibake marker text.
