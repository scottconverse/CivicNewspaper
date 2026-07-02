# Tester Visibility Report - Final Cleanroom v0.3.2 916653b

Date: 2026-07-02T22:42:00Z
Tester machine: Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Directive: test-comms/directives/20260702-final-cleanroom-v032-916653b.md
Product branch label: main
Product commit represented by installer: 916653b87e09814d4c42bdcb31f91ca7ac4fae09

## Current Status

BLOCKED from compile/export/publish because all generated drafts remained `needs_verification` and the Workbench did not expose a usable approval or improve path for them.

## Visibility So Far

- Installer hash matched: F1DD475B97F497241DEDF00F48EBCC7A59318A7FFE3994E0030183072026DE54.
- Installer size matched: 5200358 bytes.
- Clean wipe completed for the allowed CivicNewspaper / The Civic Desk state boundaries.
- Installed app launched from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Native window title was `The Civic Desk`.
- First-run setup selected the Longmont, CO starter profile.
- App-guided local AI setup reached `Local AI ready` with `phi4-mini:latest`.
- Daily Scan completed in the DB: newest `daily_scan_runs.run_status` is `completed`.
- No-source draft behavior improved: no-source draft was stored as `needs_verification` with an explicit missing-source assignment note and no invented outlet/reporter names.
- Linked-source fallback drafts now include `According to the linked source` and valid `[Source](evidence:3)` syntax.
- Publishing output folder control opened Windows Explorer at `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`; no missing-folder error appeared.

## Blockers

- All three generated drafts were `needs_verification`; approved draft count was 0.
- Publishing stayed blocked before compile/export/publish.
- Opening a generated draft from Workbench repeatedly produced a blank Workbench view, so I could not run or verify `Improve for Publication` through the UI.
- Publishing also showed the public identity still using starter text (`My Local Publication`), pausing public publishing until identity is edited.

## Evidence

Evidence is under:

`test-comms/evidence/20260702-final-cleanroom-v032-916653b/`

Key files:

- `install-clean-launch.log` - clean install and launch notes.
- `environment.json` - installer/app/DB paths and hash evidence.
- `db-after-ai-ready.txt` - first DB snapshot after AI-ready.
- `db-after-scan-wait120.txt` - Daily Scan completion snapshot.
- `drafts-full.jsonl` - full persisted draft content.
- `final-db-summary.json` - final counts and latest scan row.
- `screenshot-03-ai-ready-wait120.png` - Local AI ready.
- `screenshot-14-second-draft-gate.png` - no-source/background candidate routed to verification notes.
- `screenshot-15-after-verification-notes.png` - no-source verification note generation result.
- `screenshot-27-workbench-scroll.png` - Workbench shows all drafts as sent back / needs work.
- `screenshot-28-open-lead22-draft.png` - opening draft leads to blank Workbench state.
- `screenshot-29-publishing-screen.png` - Publishing identity review and paused starter identity.
- `screenshot-30-publishing-lower.png` - compile checklist pending.
- `screenshot-31-after-open-folder.png` - Windows Explorer opened the default site folder.

## Next

Writing the full final report with exact counts, draft text, findings, and reproduction details.
