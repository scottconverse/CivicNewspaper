# Full Report - v0.3.2 6dfc1ac Legacy Quarantine Rerun

UTC: 2026-07-05T02:19:00Z
Tester: msi\civic
Branch: test-comms/cleanroom-coder-tester
Directive read: test-comms/ACTIVE_DIRECTIVE.md
Release/docs target: 4c01de4488e311851a846d6ed4ef5421a125a24b
Product build target: 6dfc1ac8239a920251fca44d1fdbc9e5a960c58b

Result: FAIL

## Summary

Visibility, release-asset, checksum, clean profile cleanup, installer execution, and installed-app launch proof were completed from the GitHub release asset. The full cleanroom product workflow could not be run exactly from this tester channel after the installed Tauri/WebView app reached first-run Step 1, because the WebView did not expose actionable internal controls to Windows UI Automation and coordinate-based interaction could not reliably complete the gated identity/location form.

This is reported as a tester-execution blocker, not as a claim that the 6dfc1ac product quality fixes failed. Steps 12-24 were not executed and no PASS is claimed for source import, Daily Scan, drafting, editor workflow, static export, here.now publication, or public visitor inspection.

## Completed Checks

- Confirmed machine/user `msi\civic`.
- Confirmed repo path `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
- Confirmed branch `test-comms/cleanroom-coder-tester`.
- Read `test-comms/ACTIVE_DIRECTIVE.md`.
- Reached GitHub release URL and public docs URL.
- Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub v0.3.2 release.
- Verified installer SHA256 `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`.
- Verified installer size `5253971` bytes.
- Verified `SHA256SUMS.txt` names `The.Civic.Desk_0.3.2_x64-setup.exe` and contains the same SHA256.
- Verified the release page and public docs contain the expected hash, and public docs include `More info` and `Run anyway`.
- Removed prior `%LOCALAPPDATA%\The Civic Desk` profile state; the other three directive-listed state paths did not exist.
- Ran the downloaded GitHub installer silently; installer exit code was `0`.
- Resolved the installed shortcut to `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
- Launched the installed app from that installed EXE; process `civicnews` opened a `The Civic Desk` window.
- Captured first-run onboarding evidence showing `Workspace Setup`, `Step 1 of 5`, and starter profiles including `Longmont`.

## Blocker 1 - Tester channel cannot complete the installed WebView onboarding workflow exactly

Severity: Blocker

The installed app reached first-run setup, but the embedded WebView exposed only a top-level `WRY_WEBVIEW`/Chrome pane to Windows UI Automation. It did not expose the form controls, buttons, or web document structure required for reliable automated interaction.

Coordinate interaction was attempted against direct window captures. It could select/bring up the Longmont identity form and partially interact with visible fields, but it could not reliably complete the required identity/location gate or advance past Step 1. Because the directive requires an exact cleanroom workflow through onboarding, source import, scan, drafting, export, publish, and public inspection, continuing would risk fabricating status.

Evidence:

- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/installed-app-first-screen.png`
- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/installed-app-window-print.png`
- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/onboarding-after-window-coordinate-clicks-r3.png`
- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/onboarding-after-identity-fill.png`
- `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/onboarding-after-state-fill-next.png`

## Not Run

The following directive steps were not run because of Blocker 1:

- Complete first-run setup beyond Step 1.
- Verify full navigation beyond onboarding.
- Run source discovery/import.
- Run Daily Scan and inspect lead downgrading/quarantine behavior.
- Draft a credible Longmont story.
- Exercise hold/send-back/approve/cut editor workflow.
- Compile/export static site.
- Publish through here.now anonymous preview.
- Inspect the public here.now site as a visitor.
- Final public-output checks for duplicate topics, raw Markdown, internal paths, unsupported installer claims, and broken story links.

## Artifacts

- Visibility report: `test-comms/reports/20260704-final-release-v032-6dfc1ac-legacy-quarantine-visibility.md`
- Evidence folder: `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/`
- Cleanroom install log: `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/cleanroom-state-and-install-log-r2.txt`
- Installed launch log: `test-comms/evidence/20260704-final-release-v032-6dfc1ac-legacy-quarantine/installed-app-launch-log.txt`

## Reproduction Notes

1. Fetch and fast-forward `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Download the v0.3.2 installer and checksum from GitHub release assets.
4. Verify SHA256 `FA64134DD63DE0194AE4645CC41ECC405576DA117311A5FD592673EACB619EF4`.
5. Remove the four directive-listed app state paths.
6. Install and launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Observe the first-run Step 1 onboarding form. In this tester channel, internal WebView controls are not exposed to UI Automation, and coordinate-driven interaction could not complete the step exactly.
