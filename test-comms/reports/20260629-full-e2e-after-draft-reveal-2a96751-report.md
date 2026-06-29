# Tester Report - Full E2E After Draft Reveal 2a96751

Date: 2026-06-29T03:12Z-03:35Z
Tester machine: msi\civic cleanroom Windows tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 2a96751884f9bb7d23ba2c480cd51618e574913d
Directive: test-comms/directives/20260629-rerun-full-e2e-after-draft-reveal-2a96751.md

## Summary

Fail/blocked. The product passed install, launch, clean setup recovery, app-owned Local AI startup, model readiness, source intake, Story Queue recovery, and first draft generation. The specific regression fix partially passes: after draft 1, returning to Story Queue and clicking Draft on a different visible lead now visibly reveals the draft wizard for lead 2. However, draft 2 generation does not persist; after both keyboard and direct-button attempts the app returns to Story Queue and the database remains at one draft.

This is not ready for Scott to use for a real Longmont publication next week because the product still cannot produce the required 5-10 stories/briefs, so editor controls, export, here.now publish, and soak were not reached.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.4 GB on C:
- Node: not manually used
- Rust: not manually used
- npm: not manually used
- Ollama installed/running: no global `ollama` command before install; app-owned Ollama process started during setup
- Models present: qwen2.5:7b shown by app as Local AI ready

## Install Artifact

Installed:

`test-comms/artifacts/20260629-rerun-full-e2e-2a96751/The Civic Desk_0.2.8_x64-setup.exe`

Observed and expected hashes matched:

- NSIS SHA256: `75B78452EB7863DEE16D69574D6E384D9232886BB308659B2D73E3813EAE05B6`
- MSI SHA256: `408164A4E6808C00F95CFCB0469DED20027C4BF9560C5C8808DB8DF84A61F5DA`

## Clean Wipe Boundary

Pre-wipe state:

- `civicnews` running: yes, from prior test
- `ollama` running: yes, from prior app-owned state
- global `ollama` command: none
- user model store before product install: `C:\Users\civic\.ollama`

Removed only CivicNewspaper/product-owned state:

- `C:\Users\civic\AppData\Local\The Civic Desk`
- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- `C:\Users\civic\.ollama`
- `C:\Users\civic\AppData\Local\Ollama`
- `C:\Users\civic\AppData\Roaming\Ollama`
- `C:\Users\civic\AppData\Local\Programs\The Civic Desk`
- `C:\Users\civic\AppData\Local\Programs\Ollama`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, `test-comms/ACTIVE_DIRECTIVE.md`, and the active directive.
3. Verified product branch and artifact hashes.
4. Performed clean wipe inside the directive boundary.
5. Installed the NSIS installer with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Set the app window to 1280x720.
8. Let setup/model/source recovery complete without manually installing prerequisites.
9. Confirmed Local AI ready with `qwen2.5:7b`.
10. Confirmed recovered queue counts with read-only SQLite.
11. Opened a visible lead and generated draft 1.
12. Returned to Story Queue.
13. Opened a different visible lead and confirmed the draft wizard was visibly revealed for lead 2.
14. Tried to generate draft 2 using keyboard focus and a direct visible-button attempt.
15. Stopped when the app returned to Story Queue and read-only DB counts remained at one draft.

## Results

- Install and launch from provided installer: Pass.
- First-run setup for Longmont, CO: Pass.
- App-owned local AI runtime install/start: Pass.
- Model selection/download: Pass, `qwen2.5:7b` shown ready.
- Source discovery/import: Pass for recovered source set.
- Daily Scan / Story Queue material: Pass.
- At least 10 reviewable leads: Pass, 19 leads.
- Draft 1 generation: Pass, one `draft_generated` row persisted.
- Draft 2 wizard reveal after returning to queue: Pass.
- Draft 2 generation/persistence: Fail.
- 5-10 reader-facing stories/briefs: Fail, only one draft generated.
- Writer/editor actions: Not reached beyond first generated draft editor.
- Duplicate drafting routing: Not fully evaluated.
- Export static output and ZIP: Not reached.
- here.now publish and HTTP verification: Not reached.
- 12-hour soak: Not started.

## Counts

After setup recovery:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 19
- `daily_scan_leads`: 11
- `daily_scan_runs`: 1
- `drafts`: 0
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 3

After draft 1:

- `drafts`: 1

After draft 2 attempts:

- `sources`: 6
- `evidence_items`: 27
- `leads`: 19
- `daily_scan_leads`: 11
- `daily_scan_runs`: 1
- `drafts`: 1
- `publish_runs`: 0
- `published_posts`: 0
- `verification_tasks`: 3

Generated draft:

- Draft id 1
- Lead id 6
- Format: `watch`
- Status: `draft_generated`
- Title begins: `Draft: VOTING SIGNAL: Found vote/decision keywords...`

## Sources Imported

The DB contained 6 sources and 27 evidence items. The test did not reach a publication issue, so no final source grouping for output was produced.

## Evidence

Artifacts are in:

`test-comms/artifacts/20260629-rerun-full-e2e-2a96751/`

Key files:

- `run2a-monitor-log.json`: setup/runtime monitor.
- `run2a-monitor-420.png`: app recovered to Story Queue with Local AI ready.
- `run2a-db-counts-after-recovery.json`: recovered source/lead counts.
- `run2a-01-leads-visible.png`: Story Queue with visible lead cards and Draft buttons.
- `run2a-02-draft1-wizard.png`: draft 1 wizard with Generate Draft visible.
- `run2a-04-after-keyboard-draft1.png`: first generated draft editor path.
- `run2a-db-after-draft1.json`: one draft persisted.
- `run2a-05-second-lead-visible.png`: second visible lead.
- `run2a-06-second-draft-wizard-reveal.png`: second lead wizard visibly revealed.
- `run2a-07-after-draft2-generate.png`: after keyboard attempt, back at Story Queue.
- `run2a-08-final-draft2-direct-attempt.png`: after direct-button attempt, back at Story Queue.
- `run2a-db-final.json`: final DB counts still at one draft.

No local output folder, ZIP path, or here.now URL exists because export/publish was not reached.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Draft 2 wizard reveals but generation does not persist

Observed: After draft 1 persisted, returning to Story Queue and clicking Draft on a different visible lead now correctly reveals the draft wizard for that second lead. However, attempts to generate draft 2 return to Story Queue without creating a second draft. The DB remains `drafts: 1`.

Expected: After the second lead wizard is visible, `Generate Draft` should create draft 2 and allow repeating until at least five drafts/stories exist.

Impact: The release E2E flow remains blocked before required story count, editor workflow, export, here.now publish, and soak.

Repro:

1. Clean install `2a96751` from the directive NSIS installer.
2. Let app complete setup/source recovery.
3. Open Story Queue at 1280x720.
4. Open a visible lead and generate draft 1.
5. Return to Story Queue.
6. Open a different visible lead; observe the draft wizard is revealed.
7. Try to generate draft 2.
8. Observe app returns to Story Queue and `drafts` remains 1.

## Request For Coder

The draft-reveal fix works for the second lead, but generation after that reveal still does not persist draft 2. Please fix the second-and-later draft generation path so tester can create at least five drafts through visible UI and continue to editor/export/publish.
