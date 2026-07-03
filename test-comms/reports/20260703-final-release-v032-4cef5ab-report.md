# Tester Report - v0.3.2 Repair Rerun 4cef5ab

Date: 2026-07-03T23:58Z
Tester machine: `msi\civic`
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
Product branch: release installer from `v0.3.2` GitHub release; directive names release/docs commit `8ac4fc3eb7246205074960c638ab1f3eaa0bde44` and embedded app build commit `4cef5ab218b6fe7b6167f143a0db57377e6ac3fe`
Product commit: `4cef5ab218b6fe7b6167f143a0db57377e6ac3fe` embedded build per directive
Directive: `test-comms/ACTIVE_DIRECTIVE.md` / `test-comms/directives/20260703-final-release-v032-4cef5ab.md`
Result: FAIL

## Environment

- Windows version: Windows 10 Home, version 2009, OS build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16,870,060,032 bytes reported by Windows
- GPU: Not separately enumerated in this run
- Disk free: Not separately enumerated in this run
- Node: Not required for release-installer path
- Rust: Not required for release-installer path
- npm: Not required for release-installer path
- Ollama installed/running: no `ollama` process running during setup step 2; user-level model cache exists under `C:\Users\civic\.ollama\models`
- Models present: `phi4-mini/latest` cache present under `C:\Users\civic\.ollama\models\manifests\registry.ollama.ai\library\phi4-mini\latest`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`; fast-forwarded from `006204d` to `40251a4`, which added active rerun directive `20260703-final-release-v032-4cef5ab.md`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/`.
3. Verified machine/user as `msi\civic` and repo path as `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.
4. Reached release URL and public docs URL with HTTP 200.
5. Confirmed release/docs both contain SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`, `More info`, and `Run anyway`; no `$hash` placeholder found.
6. Queried GitHub release API. It listed exactly two assets: `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt`.
7. Downloaded both release assets from the GitHub release URL.
8. Verified installer size `5,232,809` bytes and SHA256 `0E038A6D03436BAC572CA9ABB47F17221F6F4B87F08A4D963B192AD99708834A`.
9. Verified `SHA256SUMS.txt` contains the same SHA256 and names `The.Civic.Desk_0.3.2_x64-setup.exe`.
10. Wrote and pushed visibility checkpoint commit `2156159` with `[skip ci]`.
11. Stopped prior `civicnews.exe` and app-managed `ollama.exe` processes.
12. Uninstalled prior registered The Civic Desk 0.3.2 via `C:\Users\civic\AppData\Local\The Civic Desk\uninstall.exe /S`.
13. Verified app-specific residual paths were absent after uninstall:
    - `C:\Users\civic\AppData\Local\The Civic Desk`
    - `C:\Users\civic\AppData\Roaming\The Civic Desk`
    - `C:\Users\civic\AppData\Local\the-civic-desk`
    - `C:\Users\civic\AppData\Roaming\the-civic-desk`
    - `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
14. Installed from the downloaded GitHub release installer using `/S`.
15. Verified registered app install:
    - DisplayName: `The Civic Desk`
    - DisplayVersion: `0.3.2`
    - Publisher: `scottconverse`
    - InstallLocation: `C:\Users\civic\AppData\Local\The Civic Desk`
16. Launched installed executable `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
17. Confirmed fresh app data was created under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`.
18. Confirmed first-run setup opened naturally at `Workspace Setup`, step 1 of 5.
19. Used keyboard activation to advance from step 1 to step 2. Mouse click on the starter profile/Next did not advance until keyboard focus/Enter was used.
20. Reached `AI Service Setup`, step 2 of 5. The screen states that setup can continue by installing local AI runtime or choosing `Skip for now`.
21. Attempted to proceed from step 2 with visible click target, PageDown, mouse wheel, Ctrl-minus zoom, and keyboard focus/Enter.
22. Could not reach a visible or operable continuation control. A focus walk followed by Enter opened a native `Save As` dialog for `civicnews-diagnostics` instead of advancing setup.
23. Canceled the diagnostics dialog and stopped downstream testing because first-run setup could not be completed.

## Results

- Visibility and release asset verification: PASS.
- Release page asset count: PASS, exactly one installer and one checksum asset found by GitHub release API.
- Installer checksum: PASS.
- Clean uninstall/remove prior app state: PASS.
- Install from downloaded GitHub release installer: PASS.
- Real installed app launch: PASS.
- Natural first-run onboarding: PASS.
- Complete first-run setup: FAIL/BLOCKED at step 2.
- Source discovery/import: NOT RUN because setup could not be completed.
- Daily Scan lead quality checks: NOT RUN because setup could not be completed.
- Drafting from source evidence: NOT RUN because setup could not be completed.
- Editor hold/send-back/approve/cut workflow: NOT RUN because setup could not be completed.
- Export static site: NOT RUN because setup could not be completed.
- Publish here.now preview: NOT RUN because setup could not be completed.
- Public here.now visitor inspection: NOT RUN because setup could not be completed.
- Public docs/release final honesty check: PARTIAL; hash and SmartScreen guidance are present, but automated text search did not find explicit `Windows-only` or `Windows only` wording on release/docs pages.

## Evidence

- Visibility report: `test-comms/reports/20260703-final-release-v032-4cef5ab-visibility.md`
- Downloaded installer: `test-comms/evidence/20260703-final-release-v032-4cef5ab/The.Civic.Desk_0.3.2_x64-setup.exe`
- Downloaded checksum file: `test-comms/evidence/20260703-final-release-v032-4cef5ab/SHA256SUMS.txt`
- Installed app launch screenshot: `test-comms/evidence/20260703-final-release-v032-4cef5ab/installed-app-launch.png`
- Foreground app screenshot: `test-comms/evidence/20260703-final-release-v032-4cef5ab/app-foreground-current.png`
- Step 1 keyboard/Longmont setup screenshots:
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-keyboard-after-tab-enter.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-step-after-next-key.png`
- Step 2 blocked setup screenshots:
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-step2-lower.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-step2-wheel-lower.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-step2-zoomed.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-after-click-skip-text.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-after-step2-tab-enter.png`
  - `test-comms/evidence/20260703-final-release-v032-4cef5ab/setup-after-diagnostics-dialog-cancel.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

### BLOCKER-1: First-run setup cannot continue past AI Service Setup on this clean profile

Observed: After clean uninstall, clean app-data removal, install from the downloaded GitHub release asset, and real installed-app launch, first-run setup reached `Workspace Setup` step 2 of 5, `AI Service Setup`. The page says to install the local AI runtime or choose `Skip for now`, but no visible button/control was reachable in the viewport. PageDown, mouse wheel, and Ctrl-minus did not expose lower controls. Clicking the visible `Skip for now` text did not advance. Keyboard Tab focus had no visible focus indicator; after multiple Tabs, Enter opened a native `Save As` dialog for `civicnews-diagnostics` rather than continuing setup.

Expected: A cleanroom tester should be able to continue setup using a visible and operable `Skip for now`, `Next`, or local AI setup control, without accidentally activating diagnostics export.

Impact: Blocks completion of first-run setup and therefore blocks required source discovery/import, Daily Scan, drafting, editor workflow, export, publish, and public site inspection.

Repro: Clean uninstall/remove app state, install `The.Civic.Desk_0.3.2_x64-setup.exe` from release, launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`, use keyboard Enter to advance from step 1 to step 2, then attempt to continue from `AI Service Setup`.

### MAJOR-1: Release/docs final honesty check did not confirm explicit Windows-only wording

Observed: Automated content checks found the new installer hash, `More info`, `Run anyway`, unsigned guidance, discussion text, and here.now text on the public docs where applicable. The same checks did not find literal `windows-only` or `windows only` wording on either the GitHub release page HTML or public docs HTML.

Expected: The directive requires confirming v0.3.2, Windows-only beta, unsigned installer guidance, here.now support, discussion links, and release download links are all present and honest.

Impact: Users may not receive an explicit Windows-only beta expectation from the checked pages, or the wording may be present in a form this automated check did not detect. This needs coder review or a more explicit text update.

Repro: Fetch `https://github.com/scottconverse/CivicNewspaper/releases/tag/v0.3.2` and `https://scottconverse.github.io/CivicNewspaper/`; search lowercase HTML content for `windows-only` and `windows only`.

### MINOR-1: Setup step 1 initially shows a banner saying identity input is not receiving input yet

Observed: The clean first-run setup initially displayed: `Setup is not receiving identity input yet. Choose a starter profile, type your city and state, or use Tab and Enter to continue with fields you control.` Mouse clicks on the Longmont starter/Next path did not advance until keyboard navigation was used.

Expected: Starter profile and Next controls should work directly with mouse input on first-run setup.

Impact: This did not block step 1 because keyboard navigation advanced, but it is an early usability/accessibility warning and foreshadowed the step 2 keyboard trap.

Repro: Launch a clean installed app profile and observe step 1 of Workspace Setup.

## Request For Coder

Fix first-run setup step 2 so the local AI runtime path and `Skip for now`/continuation controls are visible and operable at the tester viewport, have visible keyboard focus, and do not route focus to diagnostics export before setup can continue. Also make the Windows-only beta wording explicit on public docs/release text, or provide the exact wording testers should search for.
