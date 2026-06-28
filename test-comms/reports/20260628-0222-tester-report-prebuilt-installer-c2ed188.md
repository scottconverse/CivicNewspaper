# Tester Report - Prebuilt Installer Cleanroom c2ed188

Date: 2026-06-28T02:22:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop, 16 GB RAM class
Repo: https://github.com/scottconverse/CivicNewspaper.git
Product branch: stable-readiness-local-gates
Product commit: c2ed188, represented by prebuilt artifact
Directive:

- test-comms/directives/20260627-1958-coder-directive-prebuilt-installer-cleanroom.md

## Environment

- Windows version: Microsoft Windows 11 Home, version 10.0.26200, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16,870,060,032 bytes installed / 16 GB class
- GPU: Intel(R) UHD Graphics, 2,147,479,552 bytes adapter RAM, driver 32.0.101.5972
- GPU: NVIDIA GeForce RTX 4050 Laptop GPU, 4,293,918,720 bytes adapter RAM, driver 32.0.15.8129
- Disk free: C: 379,812,225,024 bytes free during report write
- Node: not used for product execution
- Rust: not used for product execution
- npm: not used for product execution
- Ollama installed/running: no system `ollama` command on PATH before install; installed app launched bundled `ollama.exe` sidecar from its install folder, but UI reported Local AI offline
- Models present: no model downloaded; selected model shown as `qwen2.5:7b`

## Steps Run

1. Pulled the comms branch and read the new directive:

```powershell
git fetch origin
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms\directives\20260627-1958-coder-directive-prebuilt-installer-cleanroom.md
```

2. Verified artifact contents and hashes:

```powershell
Expand-Archive test-comms\artifacts\c2ed188\The-Civic-Desk-0.2.8-c2ed188-windows-x64-cleanroom.zip -DestinationPath ..\artifact-c2ed188 -Force
Get-Content ..\artifact-c2ed188\SHA256SUMS.txt
Get-FileHash '..\artifact-c2ed188\The Civic Desk_0.2.8_x64-setup.exe' -Algorithm SHA256
Get-FileHash '..\artifact-c2ed188\The Civic Desk_0.2.8_x64_en-US.msi' -Algorithm SHA256
```

Results:

- NSIS installer hash matched expected `40829AD1793C50252EDA3D03F0635964C71506DE182EC6CDBDBCAD7B68FC0F2A`
- MSI hash matched expected `A1BFEEC4634E33DD61DE5BFDE183BEA38AB97B307647F718095DC1C0EDD10013`

3. Installed the NSIS artifact as the current non-admin user:

```powershell
Start-Process -FilePath '..\artifact-c2ed188\The Civic Desk_0.2.8_x64-setup.exe' -ArgumentList '/S' -Wait
```

Result: installer exit code 0.

4. Confirmed installed app location and installed browser extension resources:

```powershell
Get-ItemProperty HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*
Get-ChildItem '<USER>\AppData\Local\The Civic Desk'
Get-ChildItem '<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium'
```

5. Launched installed app executable, not source/Vite:

```text
<USER>\AppData\Local\The Civic Desk\civicnews.exe
```

6. Completed first-run onboarding with a clean tester profile:

- Publication: Cleanroom Gazette
- Editor: Cleanroom Tester
- City/state: Longmont, CO
- AI service step: waited for service result
- Model step: skipped `qwen2.5:7b` download after confirmation
- Defaults step: accepted defaults
- Finished onboarding into workspace

7. Exercised required installed-app areas:

- Daily Scan with Local AI offline
- Workbench with no draft/lead and Local AI offline
- Browser Pairing and Open extension folder
- Narrow window around 508 px wide for Sources, Publishing, Workbench, and System & Status

## Results

- Artifact hash verification: Pass
- NSIS install as current user: Pass
- Installed app launch: Pass
- First-run state natural: Pass. The installed Tauri app opened onboarding naturally; no Vite/source window or query-string forcing was used.
- Clean app-data state: Pass with caveat. No existing The Civic Desk install/app-data was found before install. The installed app created fresh app data during this run.
- App-data/database/settings path known: Pass. Database path observed under the launched app environment at `<USER>\AppData\Local\Packages\OpenAI.Codex_2p2nqsd0c76g0\LocalCache\Roaming\com.scottconverse.civicdesk\civicdesk.db`. The app UI default backup path showed `<USER>\AppData\Roaming\com.scottconverse.civicdesk\backups`.
- Onboarding reaches workspace: Pass
- Missing AI service behavior: Pass. UI reported "Couldn't reach the AI service" and offered Retry, Save diagnostics file, Back, Skip for now, and Next.
- Missing selected model behavior: Pass. UI warned that skipping `qwen2.5:7b` means Daily Scan and AI drafting run in limited mode until a model is downloaded.
- Daily Scan degraded behavior: Pass. Daily Scan showed AI Status Offline, explained deterministic checks still build a review packet, and routed user to add sources first.
- Draft/Workbench degraded behavior: Partial. Workbench did not dead-end, but with no lead/draft it only showed an empty-state route back to Story Queue, so no AI drafting control was available to exercise.
- Browser extension folder bundling: Partial. Installed resources include `manifest.json` under `<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium`. Browser Pairing showed the Open extension folder button, but clicking it did not open a targetable File Explorer window during the test.
- Narrow-window navigation: Fail for Workbench. At 508 px wide, Sources, Publishing, and System & Status content remained reachable by vertical scrolling. Workbench remained selected and its content was exposed to accessibility, but visible content did not scroll into view; the viewport stayed on the navigation/Local AI block and blank area.
- Brand-new user reaches core feature without developer tools: Partial. A new user can install, launch, finish onboarding, reach Story Queue/Daily Scan/Sources/Publishing/System, and see offline/model guidance. Remaining issues block a clean pass.

## Evidence

Local-only screenshot evidence, not committed:

- work/installed-evidence-c2ed188/01-initial-0.png
- work/installed-evidence-c2ed188/04-ai-service-state-0.png
- work/installed-evidence-c2ed188/05-onboarding-step3-0.png
- work/installed-evidence-c2ed188/07-after-skip-model-0.png
- work/installed-evidence-c2ed188/09-workspace-initial-0.png
- work/installed-evidence-c2ed188/10-daily-scan-page-0.png
- work/installed-evidence-c2ed188/11-workbench-offline-0.png
- work/installed-evidence-c2ed188/12-browser-pairing-0.png
- work/installed-evidence-c2ed188/13-browser-pairing-open-folder-visible-0.png
- work/installed-evidence-c2ed188/15-narrow-sources-scrolled-0.png
- work/installed-evidence-c2ed188/16-narrow-publishing-scrolled-0.png
- work/installed-evidence-c2ed188/18-narrow-system-scrolled-0.png
- work/installed-evidence-c2ed188/19-narrow-workbench-focused-scroll-0.png

Observed installed extension folder contents:

```text
background.js
content.js
icon.png
manifest.json
popup.css
popup.html
popup.js
README.md
```

Observed app-data files:

```text
civicdesk.db
civicdesk.db-shm
civicdesk.db-wal
community_profile.json
```

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 1
- Minor: 2
- Nit: 0

### Major: Narrow Workbench content remains visually trapped

- Observed: At 508 px wide, selecting Workbench highlights the Workbench nav item, and accessibility exposes "No lead or draft selected" plus "Back to Story Queue". Visually, however, scrolling leaves the viewport on the navigation/Local AI block and a blank content area; the Workbench empty-state content does not become visible.
- Expected: Selecting Workbench at narrow width should make Workbench page content visible or clearly reachable by scrolling.
- Impact: The previous narrow-window/content-trap class is still present for Workbench in the installed desktop app.
- Repro: Install artifact c2ed188, complete onboarding, resize app to about 508 px wide, select Workbench, attempt to scroll down.

### Minor: Browser Pairing Open extension folder did not open a targetable Explorer window

- Observed: Browser Pairing shows "Open extension folder"; clicking the visible button did not produce a targetable File Explorer window in the tester automation. The installed extension folder itself exists and includes `manifest.json`.
- Expected: Clicking Open extension folder should visibly open the installed app resource folder for the user.
- Impact: User may be unable to complete extension loading from the app button, or the open action may be unreliable under this test environment.
- Repro: Install artifact c2ed188, open Browser Pairing, scroll to the Open extension folder button, click it, observe no targetable File Explorer window.

### Minor: Default publish path points into OneDrive Documents

- Observed: The onboarding Defaults step prefilled Publish Path under `<USER>\OneDrive\Documents\CivicNews\sites\default`.
- Expected: For local-first cleanroom behavior, a non-synced local documents path may be safer as the default, or the UI should explicitly explain the sync implication.
- Impact: A new user may unintentionally compile local publishing output into a synced folder.
- Repro: Fresh onboarding, reach Defaults step, inspect Publish Path.

## Request For Coder

Please fix or clarify:

1. The remaining narrow Workbench visibility trap at around 508 px width.
2. Whether Open extension folder should always open File Explorer from the installed resource path; if yes, make the action observable/reliable.
3. Whether the default Publish Path should avoid OneDrive/synced folders by default.

After those changes, send a new directive/artifact and tester can rerun the cleanroom pass.
