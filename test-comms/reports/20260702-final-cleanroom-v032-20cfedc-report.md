# PASS WITH FINDINGS - Final Cleanroom Release Recheck - Civic Desk v0.3.2 20cfedc

Directive: `20260702-final-cleanroom-v032-20cfedc`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `20cfedc5bc7a4cd45d954e8a55b87fe4a23f1311`
Tester machine/user/path: Windows tester as `civic`, `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Verdict

PASS WITH FINDINGS.

The fixed installer passed the cleanroom path from install through app-guided AI setup, Daily Scan, linked-source drafting, Workbench visibility, editor review/attestation, local package export, here.now publish, and public-output audit.

Important findings remain:

1. First-run typed custom identity values were not accepted cleanly during setup; the selected Longmont starter did persist concrete non-generic identity values, and the tester later used `Edit identity` to set `Longmont Cleanroom Beta Desk` before compile.
2. `Plain Language Rewrite` was visible and invokable, but after a 90 second wait it did not change the linked-source draft content in the DB. The tester manually edited the title/body in the visible editor before approval.
3. One generated linked-source draft had a lead-summary headline and source-overlap warnings before editing. The app correctly routed approval through a logged preflight warning modal.

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-20cfedc/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256 observed: `B41AF31919D2271DF2200F6B449CE1B6FB21871826979CF71601730AB97D5C1D`
- Size observed: `5199754`
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

Evidence: `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/install-clean-launch.log`

## Flow Results

1. Pull/read active directive: PASS.
2. Verify installer hash and size: PASS.
3. Clean wipe product/runtime state: PASS.
4. Install directive NSIS artifact: PASS.
5. Launch installed app normally: PASS.
6. Confirm native window title and product identity: PASS, title `The Civic Desk`.
7. Complete first-run identity setup: PASS WITH FINDING. Longmont starter persisted concrete values, but typed custom values were not accepted cleanly in first-run setup.
8. App-guided AI setup reaches AI Status Ready without manual dependency installation: PASS.
9. Add/discover Longmont starter sources through the app: PASS.
10. Run Daily Scan: PASS.
11. Newest `daily_scan_runs` row after leads present is not left `in_progress`: PASS.
12. Generate at least two drafts from different leads: PASS.
13. No-source verification assignment behavior: PASS.
14. Linked-source generated draft checks: PASS WITH FINDINGS.
15. Open generated drafts from Workbench draft picker: PASS. The editor and actions became visible/reachable.
16. Verify Improve for Publication / rewrite path: FINDING. `Plain Language Rewrite` was visible and invoked, but no DB content change was observed after 90 seconds.
17. Approve only source-linked, attributed, reader-facing copy: PASS. Tester manually edited one linked-source draft before approval.
18. Go to Publish: PASS.
19. Before compiling, click Open folder on output folder card: PASS.
20. Confirm default output folder opens/creates: PASS for `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
21. Compile/export publication package: PASS.
22. Verify ZIP/package files are present: PASS.
23. Publish to here.now using app flow: PASS.
24. Inspect here.now publication: PASS.

## Database Snapshot

From `final-db-summary.json`:

- `sources`: 18
- `daily_scan_runs`: 1
- `daily_scan_leads`: 19
- `leads`: 22
- `evidence_items`: 31
- `lead_evidence`: 11
- `drafts`: 3
- `publish_runs`: 1
- `published_posts`: 1
- `verification_tasks`: 93
- `model.selected`: `phi4-mini:latest`
- `identity.newsroom_name`: `Longmont Cleanroom Beta Desk`
- `identity.editor_name`: `Local Editor`
- `identity.city`: `Longmont`
- `identity.state`: `CO`

Newest `daily_scan_runs` row:

```json
{
  "id": 1,
  "started_at": "2026-07-02T23:17:59.719051100+00:00",
  "completed_at": "2026-07-02T23:20:11.872014700+00:00",
  "run_status": "completed"
}
```

## Generated Drafts Considered For Approval

Full draft rows are in `test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/drafts-full.jsonl`.

### Draft 1

- Draft id: 1
- Lead id: 22
- Status: `draft_generated`
- Format: `watch`
- Title: `Longmont official city website: View Events Summer Concert Series: 2MX2 Thursday, July 2 • 7`
- Content:

```text
According to the linked source, View Events Summer Concert Series: 2MX2 Thursday, July 2 • 7 pm - 8:30 pm 400 Quail Rd. [Source](evidence:3).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It was linked, attributed, and editable, but the tester used a different source-linked draft for publish.

### Draft 2

- Draft id: 2
- Lead id: 21
- Status after editor approval: `ready_to_publish`
- Format: `watch`
- Attested by: `Local Editor`
- Attested at: `2026-07-02T23:43:27.227676500+00:00`
- Final title: `Longmont Chamber flags 2026 state-session issues for businesses`
- Final content:

```text
According to the linked source, the Longmont Area Chamber of Commerce reported that Colorado lawmakers finished the 2026 legislative session and said budget, labor, and energy-policy decisions could affect local businesses. [Source](evidence:19).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: approved after manual editor rewrite. The generated draft was source-linked and attributed, but had a lead-summary title and source-overlap warnings. `Plain Language Rewrite` did not alter the DB content, so the tester edited the visible Workbench fields, saved the draft, marked it ready for review, checked the editor responsibility box, and approved via the logged preflight-warning modal.

### Draft 3

- Draft id: 3
- Lead id: 20
- Status: `needs_verification`
- Format: `watch`
- Missing evidence notes: `No source documents are linked to this lead yet. Treat this as a verification assignment until public source material is attached or cited.`
- Title: `Library Summer Concert Series - 2MX2 Performance: The Longmont Library will host a performance`
- Content:

```text
No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.
```

Decision: not approved and not publishable. This passed the no-source assignment requirement: the UI showed `Needs verification`, `Verify first`, `Linked Sources (0)`, and a note that the item should not be approved until source material is attached/cited. No unsupported outlets, reporters, staff names, or contact people appeared.

## Workbench / Approval

- Workbench opened generated drafts into a visible editor.
- Controls visible/reachable: `Plain Language Rewrite`, `Mark Ready for Review`, attestation checkbox, `Approve for Static Publish`, linked evidence pane.
- `Plain Language Rewrite` was invokable but did not change stored draft content after 90 seconds.
- Manual editor edits persisted through `Save Draft`.
- Source-linked draft 2 moved to `ready_to_review`, then `ready_to_publish`.
- Approval modal logged guardrail override reason: `Editor reviewed pre-publication warnings and chose to publish.`

## Output / Publish

- App-created default output folder exists: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Open folder before compile: PASS.
- ZIP/local artifact path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- here.now URL: `https://flint-mango-ee62.here.now`
- Published post path: `briefs/2.html`
- Publish run provider: `here_now`
- Deployment id/note: `slug=flint-mango-ee62;version=01KWJKKS2K8XS41NYBJNX6FSGM;created_slug=flint-mango-ee62`

## Public Output Audit

Fetched and saved:

- `here-now-index.html`
- `here-now-briefs-2.html`
- `here-now-feed.xml`

Audit result in `public-output-audit.json`:

- Duplicate-topic issue: PASS; one published story.
- Mojibake markers `Ã`, `Â`, `â`, `�`: PASS; none found in fetched public pages.
- Editor-note/reporting scaffold leakage: PASS; none of `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]` found.
- Public headline is reader-facing after editor rewrite: PASS.
- Public story remains attributed and source-grounded: PASS; story page contains `According to the linked source`.

## Evidence Folder

All screenshots/logs/snapshots for this run are under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-20cfedc/
```

Key evidence includes:

- `install-clean-launch.log`
- `db-after-ai-ready.txt`
- `db-after-scan-wait120.txt`
- `drafts-full.jsonl`
- `final-db-summary.json`
- `public-output-audit.json`
- `environment.json`
- `screenshot-01-launch.png`
- `screenshot-04-ai-ready-wait150.png`
- `screenshot-05-story-queue.png`
- `screenshot-08-after-generate-draft1.png`
- `screenshot-14-after-generate-draft2.png`
- `screenshot-19-workbench-actions-lower.png`
- `screenshot-25-approved-draft.png`
- `screenshot-26-nosource-assignment.png`
- `screenshot-27-nosource-notes-generated.png`
- `here-now-index.html`
- `here-now-briefs-2.html`
- `here-now-feed.xml`

## Actionable Product Findings

### FINDING-1: First-run typed identity values did not save cleanly

The tester attempted to type directive-suggested values during first-run setup. The UI did not accept those custom typed values cleanly, although choosing Longmont persisted concrete starter values and Publishing later allowed identity editing.

Evidence: `screenshot-02-identity-filled.png`, `final-db-summary.json`.

### FINDING-2: Plain Language Rewrite did not update stored draft content

The `Plain Language Rewrite` control was visible and invoked from Workbench, but draft title/content remained unchanged in the DB after a 90 second wait. The tester manually edited the visible Workbench fields before approval.

Evidence: `screenshot-20-after-rewrite-wait.png`, `drafts-full.jsonl`.

### FINDING-3: Generated linked-source draft required editor cleanup before publish

Draft 2 was attributed and source-linked, but its generated title read like a lead summary and preflight flagged source-overlap/citation-coverage warnings. The app correctly required logged editor approval before publishing.

Evidence: `screenshot-24-ready-to-review.png`, `screenshot-25-approved-draft.png`, `drafts-full.jsonl`.

## Result

PASS WITH FINDINGS. Build `20cfedc` fixed the earlier blocker class enough to complete the cleanroom flow through installed app, linked-source editable drafts, no-source verification assignment, Workbench approval, local package export, and here.now publish. The remaining issues are actionable product findings, not run blockers for this directive.
