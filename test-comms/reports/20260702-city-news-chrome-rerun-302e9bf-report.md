# Tester Report - City News Chrome Rerun 302e9bf

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `302e9bf414dd1fb366229743998432a13d2e3644`
Directive: `test-comms/directives/20260702-city-news-chrome-rerun-302e9bf.md`

## Environment

- Windows version: Windows 11 Home
- CPU: Intel Core i7-13620H
- RAM: 16 GB
- GPU: Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: recorded in `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/machine-profile.txt`
- Node: not used for this installed-app run
- Rust: not used for this installed-app run
- npm: not used for this installed-app run
- Ollama installed/running: no product-owned `ollama` process reached during the 120-second watch
- Models present: database setting `model.selected = phi4-mini:latest`; no ready model confirmed in UI

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
2. Verified the installer artifact:
   - Path: `test-comms/artifacts/20260702-city-news-chrome-rerun-302e9bf/The Civic Desk_0.3.1_x64-setup.exe`
   - SHA256: `821CCEC384B35FFF8E1C01A602CA3FFB45AF29FA417DA0022FA1631230CD37C4`
   - Size: `5647473`
3. Stopped stale `civicnews` and product-owned `ollama`, uninstalled the prior app, removed prior app data and product-owned model data, then installed only the directive NSIS artifact.
4. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally and verified the visible desktop app window titled `The Civic Desk`.
5. Watched startup for 120 seconds and captured screenshots.
6. Attempted to complete setup with a real Longmont cleanroom identity:
   - Publication name attempted: `Longmont Chrome Gate Desk`
   - Editor name attempted: `Cleanroom Tester`
7. Clicked `Next` from the AI Setup identity step.
8. Reproduced the same result after fresh relaunch: the app returned to AI Setup identity; after identity entry and `Next`, the app window disappeared from the visible desktop while `civicnews.exe` remained alive.
9. Captured a window/database snapshot and stopped before source discovery because the end-user setup flow was blocked.

## Results

Overall result: BLOCKED.

The required Longmont end-to-end product flow could not run. The installed app became non-visible immediately after attempting to advance past AI Setup identity. The `civicnews.exe` process stayed alive, and a Win32 window snapshot reported a `The Civic Desk` window at visible coordinates, but the actual screen capture showed only the desktop. Fresh relaunch returned to the same identity step, so setup progress did not persist.

Because setup did not complete:

- The dashboard did not reach local AI ready.
- Source discovery did not run.
- Daily Scan did not run.
- Story Queue was not available for the city-news chrome/source-grounding audit.
- No draft, Workbench, export, ZIP/package, or here.now publish was attempted.

## Database Counts

At the blocked setup state:

- sources: `0`
- daily_scan_runs: `0`
- daily_scan_leads: `0`
- Story Queue leads: `0`
- evidence_items: `0`
- lead_evidence: `0`
- drafts: `0`
- publish_runs: `0`
- published_posts: `0`

Settings observed:

- `model.selected`: `phi4-mini:latest`
- no persisted `identity.*` settings

## Required Audits

- Evidence-linkage audit for ready-to-draft leads: not reached.
- City news/category/index chrome rescue audit for `https://www.longmontcolorado.gov/news`: not reached.
- Tourism/calendar navigation rescue audit: not reached.
- City-site navigation rescue audit: not reached.
- Summer Reading prior-failure audit: not reached.
- Draft/Workbench/editor workflow: not reached.
- ZIP/package path: none produced.
- here.now URL: none produced.
- Output quality audit: not applicable because no output was generated.

## Evidence

- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/install-clean-launch.log`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/model-watch.txt`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/machine-profile.txt`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/blocked-setup-visibility-snapshot.json`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-10s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-30s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-60s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-model-120s.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-relaunched-after-hidden.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-after-unicode-identity-next.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-window-restored-after-next.png`
- `test-comms/evidence/20260702-city-news-chrome-rerun-302e9bf/screenshot-fresh-relaunch-visibility-check.png`

## Findings

Severity counts:

- Blocker: `1`
- Critical: `0`
- Major: `0`
- Minor: `0`
- Nit: `0`

### Blocker - AI Setup identity Next makes app non-visible and setup cannot continue

Observed: On a clean install, the app launched visibly to AI Setup step 1. After entering a Longmont publication identity and clicking `Next`, the app disappeared from the visible desktop. The `civicnews.exe` process remained alive. A Win32 snapshot reported a `The Civic Desk` window at visible coordinates, but screenshots showed only the desktop. Fresh relaunch returned to the same AI Setup identity step, and repeating identity entry plus `Next` reproduced the disappearance.

Expected: Clicking `Next` after identity entry should advance setup and keep the app visible, eventually completing product-owned local AI setup and reaching the dashboard.

Impact: Cleanroom users cannot complete first-run setup, so the required source discovery, Daily Scan, Story Queue, draft, export, and publish flow cannot run.

Repro: Clean wipe app state, install `The Civic Desk_0.3.1_x64-setup.exe` from `20260702-city-news-chrome-rerun-302e9bf`, launch, enter identity on AI Setup step 1, click `Next`.

## Request For Coder

Please fix or clarify the AI Setup step-advance visibility failure in build `302e9bf`. The city-news chrome/source-grounding rerun could not reach Story Queue because the installed app could not advance past setup as a visible end-user flow.
