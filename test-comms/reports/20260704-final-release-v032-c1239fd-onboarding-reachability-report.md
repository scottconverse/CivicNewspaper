# Full Report - v0.3.2 c1239fd Onboarding Reachability Rerun

UTC: 2026-07-05T02:52:00Z
Tester: `msi\civic`
Branch: `test-comms/cleanroom-coder-tester`
Directive read: `test-comms/ACTIVE_DIRECTIVE.md`
Release/docs target: `3a5bee65596d72597c6903f3aa865d4d5d8f7a6d`
Product build target: `c1239fda79dd3bfc58e24c52657997550ccd2930`

Result: FAIL

## Summary

Visibility, release asset, checksum, clean uninstall/profile cleanup commands, installer execution, and installed-app launch proof were completed from the GitHub release asset. The c1239fd build did improve legacy malformed-content remediation in the inherited Longmont state: the old event-rollup lead is downgraded with quality-gate labels rather than `Ready to draft`, and the old contaminated draft is cut/quarantined with title `Quarantined draft needs fresh source review`.

However, after removing all four directive-listed state paths before install, the installed app still opened directly into the old `LONGMONT / CO` newsroom with 24 leads and one draft. It did not present first-run onboarding, so I could not validate the c1239fd Step 1 onboarding reachability change from a true clean-profile launch.

I did not publish externally. The heartbeat instruction says not to publish externally, so here.now publication was not attempted.

## Completed Checks

- Confirmed machine/user `msi\civic`.
- Confirmed repo path `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Confirmed branch `test-comms/cleanroom-coder-tester`.
- Read `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and recent files under `test-comms/directives/`.
- Reached GitHub release URL and public docs URL.
- Verified release page contained installer SHA256 `A8E32A95AE64C69B3A58B0A5CC670F04B6ADCA726933CAA49C5CC4AEBD3D37B1` and size `5251952`.
- Verified release page did not contain older 6dfc1ac or eab6a31 SHA values.
- Verified release API listed exactly one installer asset and one `SHA256SUMS.txt` asset.
- Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub v0.3.2 release.
- Verified installer SHA256 `A8E32A95AE64C69B3A58B0A5CC670F04B6ADCA726933CAA49C5CC4AEBD3D37B1`.
- Verified installer size `5251952` bytes.
- Verified `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
- Removed prior state paths listed by the directive, after recording their pre-cleanup existence:
  - `%APPDATA%\com.scottconverse.civicdesk`: existed, removed.
  - `%LOCALAPPDATA%\com.scottconverse.civicdesk`: existed, removed.
  - `%APPDATA%\The Civic Desk`: did not exist.
  - `%LOCALAPPDATA%\The Civic Desk`: existed via prior app install, uninstalled/removed.
- Ran the downloaded GitHub installer silently; installer exit code was `0`.
- Resolved installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Launched installed app from that installed EXE; process opened a `The Civic Desk` window.

## Results

- Visibility gate: PASS.
- Installer download and checksum verification: PASS.
- Clean uninstall/remove commands: PASS for the four directive-listed paths.
- Installed app launch: PASS.
- Natural first-run onboarding reachability: FAIL/BLOCKED. The app opened to existing Longmont newsroom state, not Step 1 onboarding.
- Legacy malformed event-rollup lead remediation: PASS for observed inherited state. The event-rollup lead was `Needs verification`, had quality-gate labels, and did not expose `Ready to draft` or `Open draft`.
- Legacy contaminated draft quarantine: PASS for observed inherited state. Workbench showed `Quarantined draft needs fresh source review`, status `Cut`, disabled publish approval, and a short quarantine note instead of copied event-list body text.
- Full E2E source import, fresh drafting, editor workflow, static export, and here.now visitor inspection: NOT RUN.

## Evidence

- Visibility report: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-visibility.md`
- Evidence folder: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/`
- Download/checksum receipt: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/visibility-download-receipt.json`
- Checksum file: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/SHA256SUMS.txt`
- Cleanup/install log: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/cleanroom-state-and-install-log.txt`
- Story Queue screenshot after clean install: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/story-queue-after-clean-install.png`
- Story Queue accessibility text: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/story-queue-accessibility.txt`
- Quarantined draft Workbench screenshot: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/quarantined-draft-workbench.png`
- Quarantined draft accessibility text: `test-comms/reports/20260704-final-release-v032-c1239fd-onboarding-reachability-evidence/quarantined-draft-accessibility.txt`

Note: The active directive names `test-comms/evidence/20260704-final-release-v032-c1239fd-onboarding-reachability/`, but this heartbeat explicitly constrains tester writes and evidence to `test-comms/reports/`, so evidence receipts for this pass are kept under the report folder. The downloaded installer was verified locally, then removed before commit to avoid storing the public 5 MB installer binary in the comms branch.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker 1 - Clean-profile onboarding reachability could not be validated because old Longmont state still appeared after directive-listed cleanup

Observed: After uninstalling and removing all four directive-listed state paths, the installed c1239fd app opened directly into `LONGMONT / CO` newsroom with 24 leads and one draft. It did not show `Workspace Setup`, Step 1, starter profile cards, `Continue setup`, or the Step 1 bottom action bar described by the directive.

Expected: A clean reinstall after removing the listed state paths should show first-run setup so the tester can validate the updated Step 1 reachability behavior.

Impact: The active directive exists specifically to rerun onboarding reachability. That central behavior could not be proven because the app did not enter onboarding from the requested cleanup state.

Repro:

1. Pull `test-comms/cleanroom-coder-tester` at coordination commit `7083e8b`.
2. Download the v0.3.2 installer and `SHA256SUMS.txt` from the GitHub release.
3. Verify SHA256 `A8E32A95AE64C69B3A58B0A5CC670F04B6ADCA726933CAA49C5CC4AEBD3D37B1`.
4. Uninstall prior The Civic Desk.
5. Remove `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, `%APPDATA%\The Civic Desk`, and `%LOCALAPPDATA%\The Civic Desk`.
6. Install from the downloaded release installer.
7. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
8. Observe that the first screen is Story Queue for `LONGMONT / CO`, not onboarding Step 1.

## Remediation Checks That Passed In Inherited State

- The old malformed city-events lead no longer showed encoded `&#8211;`; it displayed decoded hyphen text.
- The old malformed city-events lead was labeled `Needs verification`.
- The old malformed city-events lead included quality gates:
  - `Text contains encoded HTML, mojibake, or page-markup debris.`
  - `Text appears to combine multiple calendar or event-listing items; split and verify one specific civic item before drafting.`
- I did not observe that malformed event-rollup lead exposing `Ready to draft` or `Open draft`.
- Workbench showed a single inherited draft picker row titled `Quarantined draft needs fresh source review`, status `watch - Cut`.
- Opening it showed status `Cut`, disabled publish approval, and this quarantine body:
  - `This legacy draft was cut by The Civic Desk quality gate before publication. Text appears to combine multiple calendar or event-listing items; split and verify one specific civic item before drafting. Start a fresh draft only after confirming one current, local, source-backed item.`
- The old copied event-list body text was not present in the quarantined draft body.

## Not Run

The following directive steps were not run because of Blocker 1 and the heartbeat ban on external publishing:

- Complete first-run setup from Step 1.
- Verify full navigation from a clean onboarding flow.
- Run source discovery/import from first-run state.
- Run a fresh Daily Scan.
- Draft a fresh credible Longmont story.
- Exercise hold/send-back/approve/cut on a fresh story.
- Compile/export static site.
- Publish through here.now anonymous preview.
- Inspect a public here.now site as a visitor.

## Request For Coder

Please identify the remaining persistence source that survives the four directive-listed state paths, or add a tester-supported clean-profile reset path so the next cleanroom run can actually reach first-run onboarding. The inherited-state quarantine fixes look materially better, but the onboarding reachability change could not be validated until clean launch reaches Step 1.
