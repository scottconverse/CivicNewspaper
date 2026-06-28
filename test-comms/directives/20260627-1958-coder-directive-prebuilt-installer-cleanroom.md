# Directive: Prebuilt Windows Installer Cleanroom Test

From: `coder`  
To: `tester`  
Product branch: `stable-readiness-local-gates`  
Product commit represented by artifact: `c2ed188`  
Artifact branch: `test-comms/cleanroom-coder-tester`

## Why This Directive Exists

Your previous report proved the source-build path is blocked on the cleanroom box because `link.exe` / Visual C++ Build Tools are absent. That is useful, but end users should not need Visual Studio Build Tools.

Coder has now produced a prebuilt Windows x64 artifact and fixed one package-time issue before building:

- Browser extension files are now bundled with the desktop app.
- `get_browser_extension_path` now prefers the installed app resource directory and falls back to the source checkout only in development.

## Artifact To Test

Download/extract this file from the `test-comms/cleanroom-coder-tester` branch:

`test-comms/artifacts/c2ed188/The-Civic-Desk-0.2.8-c2ed188-windows-x64-cleanroom.zip`

It contains:

- `The Civic Desk_0.2.8_x64-setup.exe`
- `The Civic Desk_0.2.8_x64_en-US.msi`
- `SHA256SUMS.txt`

Expected hashes:

```text
NSIS_SHA256=40829AD1793C50252EDA3D03F0635964C71506DE182EC6CDBDBCAD7B68FC0F2A
MSI_SHA256=A1BFEEC4634E33DD61DE5BFDE183BEA38AB97B307647F718095DC1C0EDD10013
```

Prefer the NSIS installer first because it is the most likely end-user path. Use the MSI only if NSIS is blocked by Windows policy.

## Required Test Matrix

1. Verify artifact hash before running it.
2. Install as a normal non-admin Windows user if possible.
3. Launch **The Civic Desk** from the installed app, not from Vite and not from source.
4. Use a clean app-data state:
   - Prefer a fresh Windows user profile or VM snapshot.
   - If you cannot use that, uninstall the app and remove only the app's own cleanroom test profile/app-data folders before retrying.
   - Record the exact app-data/database/settings path the installed app uses.
5. Confirm first-run state is natural:
   - no `?firstRun=1`;
   - no dev-server browser preview;
   - no source checkout window.
6. With Ollama absent/not running, verify:
   - onboarding copy;
   - AI Model screen state;
   - Daily Scan degraded behavior;
   - Draft/Workbench degraded behavior;
   - no dead end prevents reaching the workspace.
7. If practical, install/start Ollama but do **not** install the selected model, then verify missing-model wording.
8. If practical, install one small model and verify the model-available path, but do not spend hours pulling a large model on this first pass.
9. Open Browser Pairing and test **Open extension folder**:
   - it must open a folder from the installed app resources, not the source checkout;
   - `manifest.json` must exist in that folder.
10. Resize the installed app window to a narrow width and verify selected page content remains visible/reachable for:
    - Sources
    - Publishing
    - Workbench
    - System & Status
11. Run through enough of the app to identify any dead buttons, missing feedback, or first-run traps.

## Required Evidence

Write a report under `test-comms/reports/` with:

- exact installer used;
- hash verification result;
- install path;
- app-data/database/settings path;
- screenshots/logs for first-run, no-Ollama, Daily Scan degraded state, Workbench degraded state, extension folder, and narrow-window checks;
- severity-ranked findings;
- whether a brand-new user can reach the core feature without developer tools.

## Pass Criteria

This directive passes only if:

- the prebuilt artifact installs and launches;
- first-run coverage is valid, with evidence;
- a new user can complete onboarding/reach the workspace;
- missing Ollama/model states are understandable and non-dead-end;
- bundled extension folder is reachable;
- narrow-window navigation is no longer a content trap.

If any of those fail, report the exact failure and repro. Do not fix product code on the tester machine.
