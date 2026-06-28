# Tester Report - Prebuilt Installer Rerun 67994ae

Date: 2026-06-28T02:39:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop, 16 GB RAM class
Repo: https://github.com/scottconverse/CivicNewspaper.git
Product branch: stable-readiness-local-gates
Product commit: 67994ae, represented by prebuilt artifact
Directive:

- test-comms/directives/20260627-2025-coder-directive-prebuilt-installer-67994ae-rerun.md

## Environment

- Windows version: Microsoft Windows 11 Home, version 10.0.26200, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores, 16 logical processors
- RAM: 16,870,060,032 bytes installed / 16 GB class
- GPU: Intel(R) UHD Graphics, 2,147,479,552 bytes adapter RAM, driver 32.0.101.5972
- GPU: NVIDIA GeForce RTX 4050 Laptop GPU, 4,293,918,720 bytes adapter RAM, driver 32.0.15.8129
- Disk free: C: 379,745,472,512 bytes free during report write
- Node: not used for product execution
- Rust: not used for product execution
- npm: not used for product execution
- Ollama installed/running: no system `ollama` command used; installed app launched bundled `ollama.exe` sidecar from install folder, while UI showed Local AI offline / `qwen2.5:7b`
- Models present: no model downloaded

## Steps Run

1. Pulled the comms branch and read the new rerun directive.

```powershell
git fetch origin
git pull --ff-only origin test-comms/cleanroom-coder-tester
Get-Content test-comms\directives\20260627-2025-coder-directive-prebuilt-installer-67994ae-rerun.md
```

2. Verified artifact contents and hashes.

```powershell
Expand-Archive test-comms\artifacts\67994ae\The-Civic-Desk-0.2.8-67994ae-windows-x64-cleanroom.zip -DestinationPath ..\artifact-67994ae -Force
Get-FileHash '..\artifact-67994ae\The Civic Desk_0.2.8_x64-setup.exe' -Algorithm SHA256
Get-FileHash '..\artifact-67994ae\The Civic Desk_0.2.8_x64_en-US.msi' -Algorithm SHA256
```

Results:

- NSIS hash matched `8BF4D50772584F1C0640D16BF73B0315AD9ED47E89AB0E5FB156B3384AA49D05`
- MSI hash matched `217B7B8DD8B11B76564F3D92C5B2D5EB0CC868A7BBC2EA749CBFC3D8D814C57F`

3. Uninstalled the prior app and cleared only Civic Desk test app data.

```powershell
Stop-Process civicnews,ollama -Force
Start-Process '<USER>\AppData\Local\The Civic Desk\uninstall.exe' -ArgumentList '/S' -Wait
Remove-Item '<Codex LocalCache>\Roaming\com.scottconverse.civicdesk' -Recurse -Force
Remove-Item '<Codex LocalCache>\Local\com.scottconverse.civicdesk' -Recurse -Force
```

4. Installed and launched the 67994ae NSIS app as current user.

```powershell
Start-Process '..\artifact-67994ae\The Civic Desk_0.2.8_x64-setup.exe' -ArgumentList '/S' -Wait
<USER>\AppData\Local\The Civic Desk\civicnews.exe
```

5. Completed onboarding with clean test data:

- Publication: Cleanroom Gazette
- Editor: Cleanroom Tester
- City/state: Longmont, CO
- AI service step: observed local service startup/degraded state
- Model step: skipped `qwen2.5:7b` download after confirmation
- Defaults step: inspected Publish Path and Backup Path
- Finished onboarding into workspace

6. Rechecked Daily Scan, Workbench, Browser Pairing, and narrow-width pages.

## Results

- Artifact hash verification: Pass
- Prior uninstall and clean app-data reset: Pass
- NSIS install as current user: Pass
- Installed app launch: Pass
- Fresh first-run onboarding: Pass
- Default Publish Path fix: Pass. Publish Path now defaults to `<USER>\AppData\Roaming\com.scottconverse.civicdesk\sites\default`, not OneDrive/Documents.
- Default Backup Path: Pass. Backup Path defaults to `<USER>\AppData\Roaming\com.scottconverse.civicdesk\backups`.
- Missing AI service / model states: Pass. Onboarding explains service startup/offline state, model size, skip consequences, and limited mode; user can continue.
- Daily Scan degraded behavior: Pass. Daily Scan shows Local AI offline / AI Status Offline, explains deterministic checks, and routes to add sources.
- Workbench normal-width degraded behavior: Pass. Empty-state content is visible and routes back to Story Queue.
- Narrow Workbench visibility at about 508 px: Pass. Workbench empty-state content is now visibly reachable after scrolling, not just present in accessibility.
- Narrow Sources behavior: Pass. Sources content is visible/reachable at narrow width.
- Narrow Publishing behavior: Pass. Publishing content is visible/reachable at narrow width.
- Narrow System & Status behavior: Pass. System content is visible/reachable at narrow width.
- Installed extension resources: Pass. Installed resource folder contains `manifest.json`.
- Open extension folder handoff: Fail/Unresolved. The Browser Pairing button is visible and clickable, but clicking it still did not produce a targetable or titled File Explorer window in this cleanroom automation environment.
- Ready for full GauntletGate rerun: Not yet recommended unless coder accepts the extension-folder opener result as environment-specific. The prior narrow/default-path findings appear fixed.

## Evidence

Local-only screenshots, not committed:

- work/installed-evidence-67994ae/01-initial-0.png
- work/installed-evidence-67994ae/03-ai-service-result-0.png
- work/installed-evidence-67994ae/04-model-step-0.png
- work/installed-evidence-67994ae/05-skip-model-modal-0.png
- work/installed-evidence-67994ae/06-defaults-0.png
- work/installed-evidence-67994ae/09-workspace-0.png
- work/installed-evidence-67994ae/10-daily-scan-0.png
- work/installed-evidence-67994ae/11-workbench-0.png
- work/installed-evidence-67994ae/12-narrow-workbench-scrolled-0.png
- work/installed-evidence-67994ae/16-narrow-sources-scrolled-0.png
- work/installed-evidence-67994ae/17-narrow-publishing-scrolled-0.png
- work/installed-evidence-67994ae/18-narrow-system-scrolled-0.png
- work/installed-evidence-67994ae/22-browser-pairing-button-visible-0.png

Observed installed extension folder:

```text
<USER>\AppData\Local\The Civic Desk\_up_\browser-extension\chromium
```

Contents:

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
<Codex LocalCache>\Roaming\com.scottconverse.civicdesk\civicdesk.db
<Codex LocalCache>\Roaming\com.scottconverse.civicdesk\civicdesk.db-shm
<Codex LocalCache>\Roaming\com.scottconverse.civicdesk\civicdesk.db-wal
<Codex LocalCache>\Roaming\com.scottconverse.civicdesk\community_profile.json
```

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### Minor: Open extension folder still does not produce an observable Explorer window

- Observed: Browser Pairing shows the Open extension folder button at narrow width. Clicking the visible button did not create a targetable File Explorer window or a titled Explorer window in the Windows automation app list. `explorer.exe` was running, but no window title/path was observable. The installed resource folder itself exists and contains `manifest.json`.
- Expected: Clicking Open extension folder visibly opens File Explorer to the installed browser-extension resource folder, or otherwise gives user-visible feedback if Windows blocks the handoff.
- Impact: The extension files are bundled correctly, but this test still cannot prove that a user can reach the folder through the app button.
- Repro: Install 67994ae NSIS, complete onboarding, open Browser Pairing, scroll to Open extension folder, click the button, inspect windows/processes.

## Prior Finding Status

- Narrow Workbench visibility: Fixed
- Extension folder opener: Still unresolved in this environment
- Default publish/backup paths avoiding OneDrive/Documents: Fixed

## Request For Coder

Please either:

1. Add user-visible success/error feedback around Open extension folder so tester can prove the handoff from inside the app, or
2. Clarify that direct installed-resource folder existence plus `manifest.json` is acceptable if Explorer is not targetable in this automation environment.

Once that is resolved, the build otherwise looks ready for a full GauntletGate rerun.
