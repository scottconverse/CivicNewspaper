# CivicNewspaper cleanroom E2E report - a094ce1 attempt 4

Date: 2026-06-30 UTC
Tester machine: Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: a094ce12c8aca503a75c76a3d89b25b204a2d4cc
Directive: test-comms/directives/20260630-cleanroom-e2e-a094ce1-attempt4.md

## Verdict

FAIL.

Attempt 4 fails at the first-run identity and Daily Scan gate. The typed identity values were visible before clicking Next, and the app-guided AI setup reached `Local AI ready` with `qwen2.5:7b`, but the completed setup stored/displayed a corrupted location string:

`LONGMONT / CO94 TES`

After source discovery/import, Daily Scan could not run and repeatedly reported:

`Something went wrong: Invalid city or state format`

No leads, drafts, approved stories, ZIP, or here.now publication were produced in this attempt.

## Environment

- Windows: Windows 10 Home, version 2009, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- App install method: NSIS installer
- Manual Ollama/model install: no
- Temporary WebView2 debug flag used only to inspect the real installed app; cleared after the run

## Installer and Cleanroom Setup

- NSIS installer: `test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256: `AC8610ECDCA97674377309AA4A9F3AC826E275AF43137F799384F57E4DB9CA53`
- NSIS size: `5622087`
- NSIS install exit code: `0`
- Installed executable launched: yes
- Product-clean wipe removed previous CivicNewspaper app data, prior app-installed Ollama/model state, and this attempt's prior tester-output folder.

Evidence:

- `test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/evidence/00-clean-wipe-summary.json`
- `test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/evidence/01-install-launch-summary.json`

## First-Run Identity

Typed values before clicking Next:

- Publication name: `Attempt Four Longmont Ledger`
- Editor name: `A094 Tester Editor`
- Publisher type: `Community group`
- City: `Longmont`
- State: `CO`

The values were visible in the form before clicking Next.

After setup completed and the app reached the main shell, the masthead/location displayed `LONGMONT / CO94 TES`, indicating the typed identity state was not captured cleanly. This fails the attempt-4 pass bar item requiring typed first-run identity values to be captured without the old auto-continue/corruption behavior.

Evidence:

- `02-first-launch.png`
- `03-identity-values-typed-before-next.png`
- `03-identity-values-before-next.json`
- `03b-identity-values-typed-before-next.png`
- `03b-identity-values-before-next.json`
- `09-after-setup-complete.png`
- `09-after-setup-complete.json`
- `13-current-after-identity-timeout.png`
- `13-current-after-identity-timeout.json`

## Local AI Setup

The app handled local AI setup without tester-installed dependencies.

- Model/runtime path was created by the app.
- AI status reached `Local AI ready`.
- Model shown: `qwen2.5:7b`.

Evidence:

- `05-ai-service-setup-start.png`
- `06-ai-service-wait-summary.json`
- `07-model-download-progress-*.png`
- `08-setup-step4-defaults.png`
- `09-after-setup-complete.png`

## Sources

Because no starter sources were registered after setup, I used `Discover for my city`, entered `Longmont` / `CO`, ran Auto-Find Feeds, selected trusted official/public-readable sources, and imported them.

The app reported:

- `Successfully imported 7 source(s).`
- Sources watched before Daily Scan: `7`.

Evidence:

- `14-after-discover-for-my-city.png`
- `15-discover-modal-filled.png`
- `16-after-auto-find-feeds.png`
- `17-source-checkbox-inventory.json`
- `12-selected-sources-before-import.json`
- `13-sources-after-import.json`

## Daily Scan

Daily Scan failed and stayed failed for the full monitor window:

- Error: `Invalid city or state format`
- Evidence: `0`
- Saved leads: `0`
- Open leads: `0`
- Drafts in desk: `0`

This is the exact break point for the run. Since Daily Scan produced no reviewable leads, the run could not proceed to lead inventory, Draft versus Draft anyway selection, draft generation, duplicate-topic gating, workflow exercise, ZIP export, or here.now publication.

Evidence:

- `14-daily-scan-start.png`
- `15-daily-scan-progress-*.png`
- `16-daily-scan-final.png`
- `16-daily-scan-summary.json`
- `18-daily-scan-invalid-city-state-failure.png`
- `18-daily-scan-invalid-city-state-failure.json`

## Counts

- Sources imported: 7
- Leads produced: 0
- Generated drafts: 0
- Clean approved stories: 0
- Published stories: 0
- ZIP produced: no
- here.now URL: none

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Daily Scan cannot run after corrupted typed identity state

Observed: The identity form accepted typed values before Next, but the completed setup displayed `LONGMONT / CO94 TES`. Daily Scan then failed with `Invalid city or state format` and produced zero leads.

Expected: Typed city/state values should persist as `Longmont, CO`, and Daily Scan should run against the imported Longmont sources.

Impact: Blocks the full E2E workflow before lead inventory, drafting, approval, output, ZIP, and here.now publication.

Repro: Product-clean install from the a094ce1 NSIS artifact, type distinct identity values, proceed through app-guided AI setup, import sources, then run Daily Scan.

### Major: Setup completed with no starter sources

Observed: After setup completed, the Sources screen showed `No feeds or portals registered yet`. Manual app discovery/import was required.

Expected: For a completed Longmont setup, the app should either add starter sources or preserve enough valid city/state identity for discovery and Daily Scan to work reliably.

Impact: Adds recovery work and, combined with corrupted identity state, leaves the product unable to complete Daily Scan.

## Request For Coder

Fix the typed identity persistence/city-state handling so the main shell does not corrupt `Longmont, CO` into `LONGMONT / CO94 TES`, then rerun this directive class. The next run should specifically prove typed identity values survive setup and that Daily Scan runs with nonzero leads.

## Watcher Status

The 15-minute CivicNewspaper watcher remains armed.
