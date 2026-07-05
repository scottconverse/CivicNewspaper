# Full Report - Civic Desk v0.3.2 c1239fd Clean-Profile Env

Date: 2026-07-05T03:11:13Z
Tester machine: `MSI\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Branch: `test-comms/cleanroom-coder-tester`
Comms commit at start: `518901554f8790a3a4d39c845f6a26eb517b8b76`
Directive: `test-comms/ACTIVE_DIRECTIVE.md`
Release/docs target: `fd3cf0fa980259c7681c85592819af6f28126142`
Product build target: `c1239fda79dd3bfc58e24c52657997550ccd2930`

Result: FAIL

## Summary

The clean-profile rerun fixed the prior inherited-state problem. The app launched from the installed GitHub release EXE with `CIVICNEWS_APP_DATA_DIR` set to a fresh temp folder, created a new `civicdesk.db` there, showed true first-run onboarding, advanced through Step 1, exposed the Step 2 `Skip for now` control, and reached Step 5 `Workspace ready`.

However, after entering the workspace, saving publication identity in Settings, and importing 5 sources, the app still blocked the required Daily Scan workflow with: `Choose your publication city and state in Settings before running Daily Scan.` Because Daily Scan could not proceed even after Settings save and source import, the release cannot pass this directive.

## Completed Checks

- Read `test-comms/ACTIVE_DIRECTIVE.md` on branch `test-comms/cleanroom-coder-tester`.
- Confirmed tester machine/user `MSI\civic`.
- Confirmed repo path `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Reached the GitHub release URL and public docs URL.
- Verified pre-install visibility in `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-visibility.md`.
- Downloaded and verified the GitHub release installer and `SHA256SUMS.txt`.
- Verified installer SHA256 `A8E32A95AE64C69B3A58B0A5CC670F04B6ADCA726933CAA49C5CC4AEBD3D37B1`.
- Verified installer size `5251952` bytes.
- Recorded state for `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%APPDATA%\The Civic Desk`, `%LOCALAPPDATA%\The Civic Desk`, `%APPDATA%\org.civicnews.app`, and `%LOCALAPPDATA%\org.civicnews.app`.
- Uninstalled the prior app and installed from the downloaded GitHub release installer.
- Launched only `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Launched it from PowerShell with `CIVICNEWS_APP_DATA_DIR=C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-c1239fd-cleanprofile-env`.
- Verified a clean-profile app data folder with fresh `civicdesk.db`, `backups`, `logs`, and `sites`.
- Verified true first-run onboarding instead of inherited Longmont state.
- Advanced onboarding from Step 1 to Step 2 using the Longmont setup path.
- Verified Step 2 AI setup exposes `Use selected model` and `Skip for now` above the fold.
- Skipped local AI setup and reached Step 5 `Done` / `Workspace ready`.
- Entered the workspace and observed Story Queue, Daily Scan, Verification, Workbench, Sources, AI Model, and Publishing navigation areas.
- Opened Daily Scan and Sources/source setup paths.
- Saved publication identity in Settings; the app showed `Identity saved`.
- Imported 5 sources; the app showed `Successfully imported 5 source(s).`
- Confirmed the unsigned public beta / SmartScreen guidance appears in-app.

## Blocker 1 - Completed onboarding does not leave Daily Scan usable for Longmont

Severity: Blocker

After completing onboarding in the clean-profile env run, entering the workspace, saving publication identity in Settings, and importing 5 sources, the required Daily Scan workflow was still blocked by an in-app red warning:

`Choose your publication city and state in Settings before running Daily Scan.`

This contradicts the directive expectation that completing first-run setup for Longmont, Colorado and importing Longmont sources should leave the workspace ready to inspect local stories. The clean-profile env did prevent legacy data migration, but the completed onboarding/settings/source-import state still did not satisfy Daily Scan's required publication city/state precondition.

Evidence:

- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/cleanprofile-first-launch-screen.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/onboarding-after-starter-keyboard-continue.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/after-step1-scroll-state-enter.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/after-step2-tab-enter-live.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/step5-scrolled-for-enter-button.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/navigation-daily-scan.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/settings-city-state-saved-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/sources-after-import-official-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/daily-scan-after-run-cdp.png`
- `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/after-defender-dismiss-source-state.png`

## Not Run

The following directive steps were not completed because Blocker 1 stopped the required workflow:

- Run Daily Scan and inspect leads.
- Confirm malformed, generic, recurring, unsupported, non-jurisdictional, navigation/index, markup-debris, multi-item event listing, and broad legislative/chamber rollup items are suppressed or downgraded.
- Draft a credible Longmont story from linked source evidence.
- Exercise hold/send-back/approve/cut editor workflow.
- Compile/export the static site.
- Publish through here.now.
- Inspect the public here.now site as a visitor.
- Complete final public-output checks for duplicate stories, raw Markdown, internal paths, unsupported installer claims, and broken links.

## Environment Note

Microsoft Defender displayed a sample-submission prompt during the run for `C:\Users\civic\.codex\config.toml`. The prompt was dismissed, and the app continued. This was not counted as the release blocker; the release blocker is the app's own missing city/state gate after completed onboarding.

## Artifacts

- Visibility report: `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-visibility.md`
- Evidence folder: `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/`
- Installer lifecycle log: `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/uninstall-install-log.txt`
- Clean-profile launch log: `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/cleanprofile-launch-log.txt`
- Clean-profile file inventory: `test-comms/reports/20260704-final-release-v032-c1239fd-cleanprofile-env-evidence/cleanprofile-files-after-launch.json`
