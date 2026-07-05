# Tester Report - Civic Desk v0.3.2 c1239fd Clean-Profile Env

Date: 2026-07-05T03:25:38Z
Tester machine: `MSI\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product/release: GitHub release `v0.3.2`
Product build commit under test: `c1239fda79dd3bfc58e24c52657997550ccd2930`
Directive: `test-comms/ACTIVE_DIRECTIVE.md` / `20260704-final-release-v032-c1239fd-cleanprofile-env.md`

## Result

FAIL.

The clean-profile env rerun proves the old inherited Longmont-state blocker is fixed when launching the installed app with `CIVICNEWS_APP_DATA_DIR`. The installed release opened natural first-run onboarding in the isolated profile, completed onboarding, reached the newsroom with zero inherited leads/drafts/sources, and used `phi4-mini:latest`.

The release still does not pass because first-run setup did not persist the publication city/state from onboarding, and Daily Scan generated multiple unsupported/model-looking leads, including high-priority items, with no linked evidence. I stopped before drafting/exporting/publishing because there was no credible, source-backed Longmont story suitable for publication.

## Environment

- Windows: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 15.71 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on C: 340.52 GB
- Ollama: running from `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- App launch path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Clean profile override: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-c1239fd-cleanprofile-env-debug`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read the active directive plus protocol files.
2. Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release.
3. Verified installer size `5251952` and SHA256 `A8E32A95AE64C69B3A58B0A5CC670F04B6ADCA726933CAA49C5CC4AEBD3D37B1`.
4. Verified the release and docs visibility requirements; see the separate visibility report.
5. Recorded the directive-listed state paths before install.
6. Ran the prior app uninstaller silently, then installed the downloaded GitHub release installer silently.
7. Launched only the installed EXE with `CIVICNEWS_APP_DATA_DIR` set to a fresh temp profile.
8. Confirmed first launch showed `Workspace Setup`, Step 1, not inherited Longmont newsroom state.
9. Completed onboarding using the Longmont/typed identity path and selected the detected local AI model `phi4-mini:latest`.
10. Verified main navigation routes: Sources, Daily Scan, Workbench, Publishing, Ethics & Backups, System & Status, AI Model, and Story Queue.
11. Ran source discovery for `Longmont, CO`, imported five official Longmont sources, then ran Daily Scan.
12. Inspected Daily Scan output, Workbench behavior, clean-profile DB rows, and evidence linkage.

## Results

- Visibility: PASS.
- Installer download/hash/size: PASS.
- Installed-app launch from GitHub release installer: PASS.
- `CIVICNEWS_APP_DATA_DIR` isolated profile: PASS.
- First-run onboarding instead of inherited Longmont state: PASS.
- AI setup controls/model detection: PASS.
- Core route navigation: PASS.
- Source discovery/import: PASS after using `CO`; `Colorado` returned no visible results.
- Daily Scan can run after manually saving city/state in Settings: PASS mechanically.
- Daily Scan quality and evidence linkage: FAIL.
- Credible draft/export/publish: NOT RUN because no source-backed publishable story was available and release acceptance already had major findings.

## Evidence

- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/visibility-receipt.json`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/install-receipt.json`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/state-paths-before-install.json`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/cleanprofile-first-launch-screen.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/onboarding-step5-done-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/after-finish-onboarding-dashboard-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/sources-after-import-official-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/daily-scan-after-direct-settings-save-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/workbench-first-lead-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/sqlite-leads-sources-drafts.json`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/environment-final.json`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 2
- Minor: 2
- Nit: 0

### Major 1 - Onboarding completion did not persist publication city/state

Observed: The clean profile opened onboarding correctly and completed Step 5. After finishing, the newsroom showed no inherited leads/drafts/sources, but `community_profile.json` still had `"city": ""` and `"state": ""`. Daily Scan then refused to run with `Choose your publication city and state in Settings before running Daily Scan.`

Expected: Completing first-run setup for Longmont, Colorado should persist the publication city/state so Daily Scan can run without a second Settings repair.

Impact: A true first-run user can complete onboarding and still be blocked from Daily Scan until they rediscover and re-enter the same location in Settings.

Repro: Launch installed EXE with a fresh `CIVICNEWS_APP_DATA_DIR`, complete onboarding for Longmont, finish onboarding, then click Daily Scan.

### Major 2 - Daily Scan generated unsupported/model-looking high-priority leads with no linked evidence

Observed: After manually saving `LONGMONT / CO` in Settings and importing five official sources, Daily Scan saved 10 leads. Multiple leads had `source_id: null`, no entries in `lead_evidence`, and no linked source documents. Examples include `City Council to Review Library Roof Contract Vote`, which claimed an `XYZ Construction Company` contract proposal, and `New Sustainability Program at City Hall`, which had no original URL. The first high-priority roof lead was correctly limited to `Generate Verification Notes` in Workbench, but the scan still presented unsupported/high-priority civic claims as leads.

Expected: Unsupported or model-suggested claims should be suppressed, downgraded, or clearly separated from ordinary leads, especially when no linked evidence exists.

Impact: The release cannot satisfy the directive requirement to draft a credible Longmont story from linked source evidence. It also risks editor trust by surfacing invented-looking specifics as high-priority leads.

Repro: In the clean profile, import the five official Longmont sources from discovery, run Daily Scan, then inspect `daily_scan_leads`, `lead_evidence`, and the Daily Scan UI.

### Minor 1 - State abbreviation required for source discovery

Observed: Source discovery with `Longmont` / `Colorado` returned to an empty source list with no visible error. Retrying with `Longmont` / `CO` produced the expected discovery list.

Expected: Either full state names should work, or the UI should clearly request a two-letter abbreviation and show an actionable validation message.

Impact: Mild first-run friction, but recoverable.

### Minor 2 - Windows Security sample submission prompt appeared during installed app testing

Observed: Microsoft Defender opened a sample-submission dialog for `C:\Users\civic\.codex\config.toml` while the installed app was running. I did not submit or interact with it. Alt-Tab restored app focus.

Expected: Not product-caused as far as I can prove, but it interrupted cleanroom UI automation.

Impact: Tester friction only; app remained usable.

## Request For Coder

1. Persist city/state through first-run onboarding so Daily Scan is ready immediately after setup.
2. Tighten Daily Scan so no-source/model-suggested items cannot appear as strong or high-priority leads without linked source evidence.
3. Consider accepting full state names like `Colorado` in discovery, or validate the state field explicitly.
