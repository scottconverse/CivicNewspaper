# Tester Report - Tourism Calendar Rerun 23d8dce

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `23d8dcec12adf5b5dadd4f48dd9906edb1c1aa56`
Directive: `test-comms/directives/20260701-tourism-calendar-rerun-23d8dce.md`

## Environment

- Windows version: Windows 11 Home
- CPU: Intel Core i7-13620H
- RAM: 16 GB
- GPU: Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: recorded in `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/machine-profile.txt`
- Node: not used for this installed-app run
- Rust: not used for this installed-app run
- npm: not used for this installed-app run
- Ollama installed/running: product-owned runtime only, under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Models present: product-owned `phi4-mini:latest`, ID `78fad5d182a7`, size `2.5 GB`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
2. Verified the installer artifact:
   - Path: `test-comms/artifacts/20260701-tourism-calendar-rerun-23d8dce/The Civic Desk_0.3.1_x64-setup.exe`
   - SHA256: `49372BCF0FB4A6F149E316DDAEC2CC42B48EAB82FC5644AEE164A58D7D8DC6FB`
   - Size: `5638803`
3. Stopped stale `civicnews` and product-owned `ollama`, uninstalled the prior app, removed prior app data and product-owned model data, then installed only the directive NSIS artifact.
4. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally and verified the visible desktop app window titled `The Civic Desk`.
5. Waited through product-owned local AI setup until the dashboard reached local AI ready with `phi4-mini:latest`.
6. Saved a real Longmont identity:
   - `identity.newsroom_name`: `Longmont Tourism Calendar Desk`
   - `identity.city`: `Longmont`
   - `identity.state`: `CO`
7. Ran source discovery/intake for Longmont, CO, then ran Daily Scan.
8. Inspected Story Queue and exported database queue audits.
9. Did not force draft/workbench/export/here.now because the only `ready_to_draft` lead was broad city-news navigation chrome.

## Results

Overall result: BLOCKED.

The targeted Visit Longmont tourism/calendar blocker is improved: no tourism/event-calendar hit reached `ready_to_draft` in the final database audit. The prior Summer Reading case also did not reach `ready_to_draft`.

The full publication E2E remains blocked because a broad city-news category/navigation page still produced a `ready_to_draft` Story Queue lead:

- Lead ID: `26`
- Disposition: `ready_to_draft`
- Lead text: `Longmont city news: All Categories Adults Adults 55+ Art in Public Places Awards BIFF Film Board Recruitme...`
- Linked evidence IDs: `9`, `70`
- Evidence source: `Longmont city news`
- Evidence URL: `https://www.longmontcolorado.gov/news`
- Evidence excerpt begins: `All Categories Adults Adults 55+ Art in Public Places Awards BIFF Film Board Recruitment Button Rock Children City Council Classes and Trainings Climate Coffman Street Community Event Concert Connect Longmont...`

This is category/navigation/news-index chrome, not a grounded reader-facing story. The directive says generic Longmont city events, city departments, services index, event index, newsletter, calendar, or broad navigation leads must not be `ready_to_draft` when evidence is only broad page chrome. I therefore stopped before draft generation and publish.

## Database Counts

Final database snapshot:

- sources: `19`
- daily_scan_runs: `2`
- daily_scan_leads: `19`
- Story Queue leads: `31`
- evidence_items: `79`
- lead_evidence: `25`
- drafts: `0`
- publish_runs: `0`
- published_posts: `0`

Final Story Queue dispositions:

- background: `4`
- needs_verification: `11`
- ready_to_draft: `1`
- watch: `15`

## Required Audits

- Evidence-linkage audit for ready-to-draft leads: BLOCKED. The only ready lead had linked evidence, but the linked evidence was broad category/navigation chrome and not a specific grounded story.
- Tourism/calendar navigation rescue audit: PASS for this targeted fix. No Visit Longmont tourism/event-calendar hit was `ready_to_draft`.
- City-site navigation rescue audit: BLOCKED. Lead `26` from `Longmont city news` still reached `ready_to_draft` from broad city-news category/navigation evidence.
- Summer Reading prior-failure audit: PASS for this run. Summer Reading hit was lead `22`, disposition `background`, not draftable.
- Unsupported model-suggested leads: weak/broad leads stayed mostly `watch`, `needs_verification`, or `background`, except the blocker lead above.
- Draft/Workbench/editor workflow: not attempted because there was no genuinely grounded ready-to-draft lead.
- ZIP/package path: none produced.
- here.now URL: none produced.
- Output quality audit: not applicable because no output was generated.

## Evidence

- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/install-clean-launch.log`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/model-watch.txt`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/machine-profile.txt`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/queue-tourism-calendar-audit.json`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/final-blocked-db-snapshot.json`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-10s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-30s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-60s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-model-120s.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-identity-saved.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-scan-started.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-after-daily-scan.png`
- `test-comms/evidence/20260701-tourism-calendar-rerun-23d8dce/screenshot-story-queue-blocked-ready-city-news-lead.png`

## Findings

Severity counts:

- Blocker: `1`
- Critical: `0`
- Major: `0`
- Minor: `0`
- Nit: `0`

### Blocker - Broad city-news navigation lead still reaches ready_to_draft

Observed: After clean install, Longmont identity setup, source discovery, and Daily Scan, lead `26` was `ready_to_draft` with evidence IDs `9` and `70`. Both evidence rows are from `https://www.longmontcolorado.gov/news` and contain category/navigation/news-index text such as `All Categories`, `Adults`, `Adults 55+`, `Art in Public Places`, `Awards`, `BIFF Film`, `Board Recruitment`, `Button Rock`, and similar categories.

Expected: Broad city-site navigation, category, news-index, departments, services, event-index, newsletter, and calendar chrome should not be treated as a draftable reader-facing lead.

Impact: The app still offers a weak broad-index city-site lead as draft-ready, so a cleanroom user could draft/publish unsupported civic content unless they manually catch it.

Repro: Clean install the directive NSIS artifact, set identity to Longmont, run source discovery/intake, run Daily Scan, inspect Story Queue lead `26`.

## Request For Coder

Please tighten the city-site/news-index chrome filter so broad Longmont news category/navigation evidence like lead `26` cannot reach `ready_to_draft`. The Visit Longmont tourism/calendar rescue looked improved in this run; the remaining blocker is city-site broad navigation/category text.
