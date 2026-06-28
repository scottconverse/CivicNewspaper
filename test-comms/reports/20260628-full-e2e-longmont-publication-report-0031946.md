# Tester Report - Full E2E Longmont Publication - 0031946

Date: 2026-06-28

Tester machine: Windows 11 Home cleanroom tester

Repo: `https://github.com/scottconverse/CivicNewspaper.git`

Product branch: `stable-readiness-local-gates`

Product commit: `0031946ee05562b0e17f941c91eb4782a2b87f1a` (`Expand Longmont source discovery seeds`)

Directive: `test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication-supersedes-7f07bd2.md`

Base directive: `test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Result: BLOCKED

## Environment

- Windows version: Microsoft Windows 11 Home, version `10.0.26200`, build `26200`
- CPU: 13th Gen Intel Core i7-13620H, 10 cores / 16 logical processors
- RAM: about 16 GB installed
- GPU: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU
- Ollama installed/running before product install: no `ollama` command, process, service, or user model store found
- Models present before product install: no user Ollama model store found

## Artifact Verification

Preferred installer:

- Path: `test-comms/artifacts/0031946-longmont-seeds/The Civic Desk_0.2.8_x64-setup.exe`
- Expected SHA256: `33078515B89A99E715FFF9F931D57AA3C28C495FB555CD69C9F0FC0C17F02D30`
- Observed SHA256: `33078515B89A99E715FFF9F931D57AA3C28C495FB555CD69C9F0FC0C17F02D30`
- Result: PASS

Fallback MSI:

- Path: `test-comms/artifacts/0031946-longmont-seeds/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256: `24FAA0FAA1D12E8335CBC08860E4B8B52200E8297AF3C585B3087BAC2C20B7B5`
- Observed SHA256: `24FAA0FAA1D12E8335CBC08860E4B8B52200E8297AF3C585B3087BAC2C20B7B5`
- Result: PASS

## Clean Wipe Performed

Wiped only CivicNewspaper/Ollama/test-output state before installing the superseding artifact:

- Stopped `civicnews` and `ollama` processes if present.
- Ran the prior Civic Desk uninstaller if present.
- Removed prior installed Civic Desk app folder under `<LOCALAPPDATA>`.
- Removed Civic Desk app-owned profile/database state under app-local roaming locations.
- Checked for user Ollama store and app-local Ollama state; none existed.

No Windows account, browser install, unrelated software, unrelated developer tools, or unrelated user files were wiped.

## Steps Run

1. Read the superseding `0031946` directive after the `7f07bd2` blocked report was already pushed.
2. Verified the `0031946` installer hashes.
3. Confirmed `origin/stable-readiness-local-gates` resolves to `0031946ee05562b0e17f941c91eb4782a2b87f1a`.
4. Performed the directive-bounded clean wipe again.
5. Installed the preferred `0031946` NSIS installer.
6. Launched the installed app.
7. Observed a targetable window showing `Loading The Civic Desk...`.
8. Waited for onboarding/setup.
9. The app exited before onboarding appeared.
10. Checked Windows Application event log.

## Results

### Install

PASS. The verified NSIS installer completed and installed `civicnews.exe`.

### First-run UI

FAIL / BLOCKER. The app showed only the loading screen, then crashed before onboarding or AI setup appeared.

Observed:

- Initial window: `Loading The Civic Desk...`
- After waiting, the process exited and desktop automation could not capture a live app window.
- Windows Application log recorded `Application Error` / `APPCRASH`.

Crash details from Windows event log:

- Faulting application: `civicnews.exe`
- Version: `0.2.8.0`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e84f07`
- Faulting module: `civicnews.exe`

### App-managed local AI runtime/model setup

NOT EXERCISED. The app crashed before onboarding or AI setup UI appeared. Tester did not manually install Ollama, pull models, edit PATH, or repair prerequisites.

### Longmont source discovery and publication workflow

NOT EXERCISED. The app never reached a usable first-run UI, so source discovery, source import, scan, lead review, dark-signal review, verification, writer/editor workflows, export, and here.now publish could not be attempted.

## Directive Questions

- Did the app install and set up local AI by itself from a clean product state? No. Install succeeded, but the app crashed before setup UI appeared.
- What model was selected and why? None. The app crashed before hardware/model recommendation UI.
- Did the local AI generate real usable draft content? No. The app crashed before local AI setup.
- How many leads were found? 0. Source discovery was not reachable.
- How many stories/briefs were produced? 0. Writer/editor workflow was not reachable.
- Which official sources were used? None.
- Which social/community/dark-signal sources were used? None.
- What writer/editor workflow controls were successfully exercised? None.
- What could not be exercised? All product workflow after the loading screen: first-run setup, AI setup, Longmont source discovery, ingestion, lead review, dark-signal review, verification, writing, editing, approval/attestation, export, and here.now publish.
- What broke, exactly where? Installed `civicnews.exe` crashed after the loading screen and before first-run onboarding, with Windows exception `0xc00000fd`.
- What is the local exported publication ZIP/path? None produced.
- What is the here.now URL? None produced.
- Is this ready for Scott to use next week to produce a real publication? No. The superseding artifact installs, but the app cannot reach onboarding from a clean product state on this tester machine.

## Evidence

Artifacts:

- `test-comms/artifacts/20260628-full-e2e-longmont-publication/failure-manifest-0031946.md`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication/0031946-loading-before-crash.png`

Windows event log evidence summary:

- Event provider: `Application Error`
- Event id: `1000`
- Faulting application: `civicnews.exe`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e84f07`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: installed app crashes after loading screen before onboarding

Observed:

After clean wipe and install of the verified `0031946` NSIS artifact, launching the installed app briefly shows `Loading The Civic Desk...`, then exits. Windows logs `APPCRASH` for `civicnews.exe` with exception `0xc00000fd`.

Expected:

The app should open first-run onboarding and allow the user to proceed through app-managed AI runtime/model setup.

Impact:

Full cleanroom E2E is impossible. A normal user cannot reach onboarding, local AI setup, source discovery, publishing, export, or here.now publish.

Repro:

1. Clean product state per directive.
2. Install `test-comms/artifacts/0031946-longmont-seeds/The Civic Desk_0.2.8_x64-setup.exe`.
3. Launch installed `civicnews.exe`.
4. Observe `Loading The Civic Desk...`, then process exit before onboarding.
5. Windows Application log records `APPCRASH` / exception `0xc00000fd`.

## Request For Coder

Please fix the launch-time crash in the `0031946` Longmont-seeds artifact and post a new artifact/directive. The watcher remains active and I will rerun from a clean product state when instructed.
