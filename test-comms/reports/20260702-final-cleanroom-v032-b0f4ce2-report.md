# BLOCKED - Final Cleanroom Release Check - Civic Desk v0.3.2

Directive: `20260702-final-cleanroom-v032-b0f4ce2`
Tester branch: `test-comms/cleanroom-coder-tester`
Product branch label: `main`
Product commit represented by installer: `b0f4ce21ac4e0e2aa2bd9b2f1139aefd25f63e17`
Tester machine/user/path: Windows tester as `civic`, `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

## Installer

- Path: `test-comms/artifacts/20260702-final-cleanroom-v032-b0f4ce2/The Civic Desk_0.3.2_x64-setup.exe`
- SHA256: `D3C29AB23F740EFED8535320C8CE762E50C3B6131BDD041BCD151AA528D228EE`
- Size: `5203001`
- App observed: `The Civic Desk` v0.3.2

## Clean Wipe / Install

Performed within directive boundary:

- Stopped existing `civicnews` process if present.
- Ran previous The Civic Desk uninstaller if present.
- Removed `%APPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\com.scottconverse.civicdesk`.
- Removed `%LOCALAPPDATA%\The Civic Desk`.
- Verified `%USERPROFILE%\.ollama` was absent before the v0.3.2 run.
- Installed only the directive NSIS artifact.
- Launched installed app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.

Evidence: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/install-clean-launch.log`

## Flow Results

1. Pull/read active directive: PASS.
2. Verify installer hash and size: PASS.
3. Clean wipe product/runtime state: PASS.
4. Install directive NSIS artifact: PASS.
5. Launch installed app normally: PASS.
6. Confirm native window title and product identity: PASS, window title `The Civic Desk`.
7. Complete first-run identity setup: PASS.
8. Verify identity input blocker gone: PASS. Values remained visible and database persisted `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, and `onboarding.step`.
9. Continue through app-guided AI setup: PASS. Initial local service check failed, but the app-guided `Install local AI runtime` action completed and dashboard reached `AI Status: Ready`; no manual dependency install was used.
10. Verify dashboard/workspace state: PASS.
11. Accept starter Longmont sources: PASS, `9` sources.
12. Run Daily Scan: PASS.
13. Verify Longmont leads from public readable sources: PASS, `46` leads and `43` daily-scan leads.
14. Verify duplicate-topic suppression/source grounding/evidence linkage: PARTIAL. The app produced lead dispositions and evidence links, but one generated verification draft had no linked source documents and was blocked by preflight.
15. Exercise editorial workflow: PARTIAL. Verification assignment and ready-to-draft flow both generated drafts. Approval/hold/cut/return paths were not completed before publish blocker.
16. Produce reader-facing publication with 5 to 10 stories/briefs: BLOCKED. Only 2 drafts were generated before export/publish blocker.
17. Reject/downgrade evergreen/static pages: PARTIAL. Leads include `watch`, `background`, `needs_verification`, and `ready_to_draft` dispositions. Some static/evergreen-like items were downgraded, but full publication review was not completed.
18. Verify public output scaffolding leakage: NOT REACHED. No public package/output was produced.
19. Verify public output mojibake markers: NOT REACHED. No public package/output was produced.
20. Export ready-to-publish ZIP/package: BLOCKED.
21. Publish anonymously to here.now: NOT REACHED.
22. Report URL/package paths: NOT AVAILABLE due blocker.

## Database Snapshot

From `blocked-final-db-window-snapshot.json`:

- `sources`: 9
- `daily_scan_runs`: 2
- `daily_scan_leads`: 43
- `leads`: 46
- `evidence_items`: 26
- `lead_evidence`: 18
- `drafts`: 2
- `publish_runs`: 0
- `published_posts`: 0
- `ai.setup_skipped`: `false`
- `model.selected`: `phi4-mini:latest`

Drafts:

- Draft 1: `Sign Up for City of Longmont News and Updates`, status `needs_verification`; preflight reported no linked source documents.
- Draft 2: `New Academic Focus Programs Announced at St. Vrain Valley Schools`, status `draft_generated`; linked source evidence was present, but preflight still warned: `No clear attribution phrase found. Attribute key facts to the source or rewrite more cautiously.`

## Blocking Defect

### P0 - Publishing/export cannot proceed because output folder open/choose step fails

On the Publishing screen, the directive flow requires export/package and here.now publish. The app presented `Review before compiling` with an `Open folder` button. Clicking it did not open a folder chooser or create/select an output directory. Instead, the app showed:

```text
Couldn't open folder: Something went wrong: The folder or file does not exist:
C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default
```

The path did not exist at final snapshot time:

```text
default_site_exists = false
```

Because export/package could not proceed through the app's normal publishing UI, the run stopped here per directive. I did not manually create the app's missing default publish folder or bypass the app's export path.

Evidence:

- Publishing screen before click: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-37-publishing-screen.png`
- Error after `Open folder`: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-38-after-open-folder-click.png`
- Final snapshot: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/blocked-final-db-window-snapshot.json`

## Additional Findings

### P1 - Verification-note draft can be generated without linked source documents

The `Verify first` path generated a draft for `Sign Up for City of Longmont News and Updates`, but preflight blocked it with:

```text
This scanned-lead draft has no linked source documents.
```

Evidence:

- Verification assignment: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-30-after-verify-first-click.png`
- Generated verification notes with blocker: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-31-after-generate-verification-notes-60s.png`

### P2 - Draft improvement did not clear attribution warning

The ready-to-draft St. Vrain draft had linked evidence (`Citation ID: #12`) but preflight warned:

```text
No clear attribution phrase found. Attribute key facts to the source or rewrite more cautiously.
```

Using `Improve for Publication` and waiting 90 seconds did not visibly clear the warning.

Evidence:

- Ready-to-draft lead: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-33-filtered-academic-lead-visible.png`
- Draft after generation: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-35-after-generate-ready-draft-90s.png`
- After improve action: `test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/screenshot-36-after-improve-for-publication-90s.png`

## Output / Publish

- ZIP/package path: not produced.
- here.now URL: not produced.
- Local output path: not produced by app.
- Publish status: NOT REACHED because export/package was blocked by missing default site folder.

## Evidence Folder

All screenshots/logs/snapshot for this run are under:

```text
test-comms/evidence/20260702-final-cleanroom-v032-b0f4ce2/
```

Key evidence includes screenshots `screenshot-01` through `screenshot-38`, install log, and final snapshot JSON.

## Result

BLOCKED. v0.3.2 fixed the first-run identity input blocker and completed app-guided AI setup, Longmont source ingest, Daily Scan, and draft generation. The run cannot pass because the normal Publishing/export path fails before package generation and here.now publication.
