# Tester Report - Full E2E Longmont Publication

Date: 2026-06-28

Tester machine: Windows 11 Home cleanroom tester

Repo: `https://github.com/scottconverse/CivicNewspaper.git`

Product branch: `stable-readiness-local-gates`

Product commit: `7f07bd2d9801f57fb21957d71ad15d197b06a0da` (`Add app-managed local AI runtime install`)

Directive: `test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Result: BLOCKED

## Environment

- Windows version: Microsoft Windows 11 Home, version `10.0.26200`, build `26200`
- CPU: 13th Gen Intel Core i7-13620H, 10 cores / 16 logical processors
- RAM: about 16 GB installed
- GPU: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free on `C:` before install: about 355 GiB
- Ollama installed/running before product install: no `ollama` command, process, service, or user model store found
- Models present before product install: no user Ollama model store found

## Artifact Verification

Preferred installer:

- Path: `test-comms/artifacts/7f07bd2-runtime-bootstrap/The Civic Desk_0.2.8_x64-setup.exe`
- Expected SHA256: `02BE689261EB6975BB346D684D5A16E457C705A7CE6C0AFEBB581F7186AF97D6`
- Observed SHA256: `02BE689261EB6975BB346D684D5A16E457C705A7CE6C0AFEBB581F7186AF97D6`
- Result: PASS

Fallback MSI:

- Path: `test-comms/artifacts/7f07bd2-runtime-bootstrap/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256: `84600C63442BB146C6FB9D9FC8C7163310EA2E2BD6271649B8C504848D0A2D23`
- Observed SHA256: `84600C63442BB146C6FB9D9FC8C7163310EA2E2BD6271649B8C504848D0A2D23`
- Result: PASS

## Clean Wipe Performed

Wiped only CivicNewspaper/Ollama/test-output state:

- Stopped `civicnews` and `ollama` processes if present.
- Ran the prior Civic Desk uninstaller if present.
- Removed prior installed Civic Desk app folder under `<LOCALAPPDATA>`.
- Removed Civic Desk package-local profile/database state under app-owned package-local roaming data.
- Removed prior CivicNewspaper cleanroom scratch/evidence folders from this workspace.
- Checked for user Ollama store and app-local Ollama state; none existed.

No Windows account, browser install, unrelated software, unrelated developer tools, or unrelated user files were wiped.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread the required comms files/directives.
2. Verified the new full E2E directive and installer artifact.
3. Verified installer hashes.
4. Captured hardware/runtime baseline.
5. Performed the directive-bounded clean wipe.
6. Installed the preferred NSIS installer.
7. Confirmed the installed app executable existed after install.
8. Launched the installed app as an end user would.
9. Attempted to capture the first-run window through Windows desktop automation.
10. Observed no targetable app window because the process exited.
11. Checked Windows Application event log.
12. Repeated a second normal launch to confirm reproducibility.

## Results

### Install

PASS. The verified NSIS installer completed and installed `civicnews.exe` under the expected app-local install folder.

### First-run UI

FAIL / BLOCKER. The app crashed before any targetable onboarding window appeared.

Observed:

- Desktop automation launch returned no screenshot target for `civicnews.exe`.
- The process was not running afterward.
- Windows Application log recorded `Application Error` / `APPCRASH`.
- Reproduced on a second launch after install.

Crash details from Windows event log:

- Faulting application: `civicnews.exe`
- Version: `0.2.8.0`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e864f7`
- Faulting module: `civicnews.exe`

### App-managed local AI runtime/model setup

NOT EXERCISED. The app crashed before onboarding or AI setup UI appeared. Tester did not manually install Ollama, pull models, edit PATH, or repair prerequisites.

### Longmont source discovery and publication workflow

NOT EXERCISED. The app never reached a usable UI, so source discovery, source import, scan, lead review, dark-signal review, verification, writer/editor workflows, export, and here.now publish could not be attempted.

## Directive Questions

- Did the app install and set up local AI by itself from a clean product state? No. Install succeeded, but the app crashed before setup UI appeared.
- What model was selected and why? None. The app crashed before hardware/model recommendation UI.
- Did the local AI generate real usable draft content? No. The app crashed before local AI setup.
- How many leads were found? 0. Source discovery was not reachable.
- How many stories/briefs were produced? 0. Writer/editor workflow was not reachable.
- Which official sources were used? None.
- Which social/community/dark-signal sources were used? None.
- What writer/editor workflow controls were successfully exercised? None.
- What could not be exercised? All product workflow after install: first-run setup, AI setup, Longmont source discovery, ingestion, lead review, dark-signal review, verification, writing, editing, approval/attestation, export, and here.now publish.
- What broke, exactly where? Installed `civicnews.exe` crashed immediately on launch before first-run onboarding, with Windows exception `0xc00000fd`.
- What is the local exported publication ZIP/path? None produced.
- What is the here.now URL? None produced.
- Is this ready for Scott to use next week to produce a real publication? No. The installer artifact installs, but the app cannot launch from a clean product state on this tester machine.

## Evidence

Small artifact manifest:

- `test-comms/artifacts/20260628-full-e2e-longmont-publication/failure-manifest.md`

No screenshots were captured because the app exited before a targetable window appeared.

Windows event log evidence summary:

- Event provider: `Application Error`
- Event id: `1000`
- Faulting application: `civicnews.exe`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e864f7`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: installed app crashes before onboarding

Observed:

After clean wipe and install of the verified `7f07bd2` NSIS artifact, launching the installed app exits before any targetable first-run window appears. Windows logs `APPCRASH` for `civicnews.exe` with exception `0xc00000fd`.

Expected:

The app should open first-run onboarding and allow the user to proceed through app-managed AI runtime/model setup.

Impact:

Full cleanroom E2E is impossible. A normal user cannot reach onboarding, local AI setup, source discovery, publishing, export, or here.now publish.

Repro:

1. Clean product state per directive.
2. Install `test-comms/artifacts/7f07bd2-runtime-bootstrap/The Civic Desk_0.2.8_x64-setup.exe`.
3. Launch installed `civicnews.exe`.
4. Observe process exits before UI; Windows Application log records `APPCRASH` / exception `0xc00000fd`.

## Request For Coder

Please fix the launch-time crash in the `7f07bd2` runtime-bootstrap artifact and post a new artifact/directive. The watcher remains active and I will rerun from a clean product state when instructed.
