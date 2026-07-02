# Tester Visibility Report - Final Cleanroom v0.3.2 18eb480

Date: 2026-07-02T21:50:00Z
Tester machine: Windows 11 Home, Intel i7-13620H, 15.7 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Directive: test-comms/directives/20260702-final-cleanroom-v032-18eb480.md
Product branch label: main
Product commit represented by installer: 18eb4805a2d00e0b3efad670bfe041bde6d90724

## Current Status

BLOCKED from full compile/publish, but the prior b0f4ce2 default-folder blocker appears fixed.

## Visibility So Far

- Installer hash matched: 14414BAA3CDF4C6DD0EA80630983F982BBAA749D353ACB7E953D475C5A4E6C8B.
- Installer size matched: 5197270 bytes.
- Clean wipe completed for the allowed CivicNewspaper / The Civic Desk state boundaries.
- Installed app launched from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Native window title was `The Civic Desk`.
- First-run identity completed for Longmont, CO.
- App-guided local AI setup reached `Local AI ready` with `phi4-mini:latest`.
- App-created publish path exists: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
- Publishing screen folder control opened a Windows folder picker directly at `sites\default`; no missing-folder error appeared.

## Blocker

The app reached Publishing with `0 approved stories ready for the public package`. I did not approve the generated drafts because the only linked-source draft lacked clear attribution and contained questionable unsupported reader-facing claims, while the second draft was correctly marked `needs_verification` with a missing-evidence note.

## Evidence

Evidence is under:

`test-comms/evidence/20260702-final-cleanroom-v032-18eb480/`

Key files:

- `screenshot-05-ai-runtime-install-wait90.png` - Local AI ready.
- `db-snapshot-after-ai-ready.txt` - default site folder exists and source counts.
- `screenshot-19-drafts-tab-opened.png` - drafts list shows one sent back/needs work and one drafting/generated draft.
- `drafts-full.jsonl` - full persisted draft content.
- `screenshot-24-publishing-output-folder.png` - Publishing shows default output folder and 0 approved story gate.
- `screenshot-25-after-folder-control-click.png` - folder picker opened at `sites\default`.

## Next

Writing the full final report with exact counts, findings, and reproduction details.
