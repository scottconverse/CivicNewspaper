# CivicNewspaper Cleanroom E2E Attempt 8 - c3247ba

Date: 2026-06-30 UTC

Verdict: FAIL.

Attempt 8 failed during app-guided local AI/runtime setup before Daily Scan, draft generation, compile, ZIP export, or here.now publish could be reached.

## Product Under Test

- Product branch: `main`
- Product commit: `c3247bab7c20129e99d8beb8515b124a2e49248f`
- Product version: `0.3.0`
- Installer used: NSIS
- Install path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS SHA256 expected/observed: `6801E4C41B081B55045646102DBFA6EE3CD2360AB0827BBFBCC5753D6FF861A8`
- MSI fallback SHA256 observed: `F2F3B35C92143DDF1C30B39FB6DDE1546E00A795BF4A9CE5B3925AD620EDF9F6`

## What Passed

- Product clean wipe completed.
- NSIS install completed.
- Real desktop app launched.
- First-run identity entry completed for Longmont, Colorado.
- Noisy state input was entered as `CO94 TES` during onboarding.
- The flow advanced to AI Service Setup.

## Break Point

App-guided local AI/runtime setup failed.

Observed UI text:

`Initialization Error: Local AI runtime install failed: The Civic Desk didn't have permission to complete this. Check the file or folder permissions and try again.`

The setup screen remained on Step 2 of 5 with:

- `Couldn't reach the AI service`
- `The Civic Desk didn't have permission to complete this. Check the file or folder permissions and try again.`
- Buttons: `Install local AI runtime`, `Retry`, `Save diagnostics file`, `Back`, `Skip for now`, `Next`

Per directive, I did not manually install Ollama, models, runtimes, or repair permissions. Because the app could not complete its own required dependency setup, the cleanroom E2E cannot proceed honestly.

## Not Reached

- AI ready state.
- Model download/selection.
- First-run source breadth check.
- Daily Scan.
- Story Queue/draft generation.
- Warned approval checkpoint.
- Compile.
- ZIP export.
- here.now publish.
- Public-output scans.
- Taxonomy/path verification.

## Evidence

Evidence folder:

`test-comms/evidence/20260630-cleanroom-e2e-c3247ba/`

Key files:

- `00-clean-wipe-summary.json`
- `01-install-launch-summary.json`
- `02-first-launch.json`
- `03-identity-before-next.json`
- `04-after-identity-next.json`
- `05-ai-service-poll.json`
- `06-ai-service-ready.json`
- `07-model-download-poll.json`
- `08-model-download-final.json`
- `11-runtime-install-poll.json`
- `12-runtime-install-final.json`
- screenshots alongside those JSON files

## Request For Coder

Fix the app-guided local AI runtime install permission failure on a product-clean Windows profile. The tester should not need to manually install or repair Ollama/runtime state. After that, rerun the attempt-8 E2E to verify source breadth, public taxonomy, ZIP/live publish, and mojibake/public-output cleanup.

Watcher remains armed.
