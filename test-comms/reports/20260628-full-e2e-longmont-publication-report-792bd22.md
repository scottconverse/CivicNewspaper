# Tester Report - Full E2E Longmont Publication - 792bd22

Date: 2026-06-28

Tester machine: Windows 11 Home cleanroom tester

Repo: `https://github.com/scottconverse/CivicNewspaper.git`

Product branch: `stable-readiness-local-gates`

Product commit: `792bd22ac0513ab7a6457791e083fd68cbaef436` (`Fix bundled Windows launch stack overflow`)

Directive: `test-comms/directives/20260628-rerun-full-e2e-after-windows-stack-fix-792bd22.md`

Base directive: `test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Superseding source intent: `test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication-supersedes-7f07bd2.md`

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

- Path: `test-comms/artifacts/792bd22-windows-launch-stack-fix/The Civic Desk_0.2.8_x64-setup.exe`
- Expected SHA256: `D68FC01F826549C53A6AF911583876A615F5B41B4AC133B5B48BA1750911D104`
- Observed SHA256: `D68FC01F826549C53A6AF911583876A615F5B41B4AC133B5B48BA1750911D104`
- Result: PASS

Fallback MSI:

- Path: `test-comms/artifacts/792bd22-windows-launch-stack-fix/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256: `909AA32B7CE0BC906CBB615AD4CF170A8D2E5E44E81F2171187D94822BB3AF40`
- Observed SHA256: `909AA32B7CE0BC906CBB615AD4CF170A8D2E5E44E81F2171187D94822BB3AF40`
- Result: PASS

## Clean Wipe Performed

Wiped only CivicNewspaper/Ollama/test-output state before installing `792bd22`:

- Stopped `civicnews` and `ollama` processes if present.
- Ran the prior Civic Desk uninstaller if present.
- Removed prior installed Civic Desk app folder under `<LOCALAPPDATA>`.
- Removed Civic Desk app-owned profile/database state under app-local roaming locations.
- Checked for user Ollama store and app-local Ollama state; none existed.
- Removed any prior `792bd22` output artifact folder if present.

No Windows account, browser install, unrelated software, unrelated developer tools, or unrelated user files were wiped.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread required comms files/directives.
2. Read the `792bd22` rerun directive.
3. Verified `792bd22` installer hashes.
4. Confirmed `origin/stable-readiness-local-gates` resolves to `792bd22ac0513ab7a6457791e083fd68cbaef436`.
5. Performed the directive-bounded clean wipe.
6. Installed the preferred `792bd22` NSIS installer.
7. Launched installed `civicnews.exe` by installed app path.
8. Completed first-run identity step with Longmont-local values:
   - Publication: `The Longmont Ledger`
   - Editor: `Cleanroom Tester`
   - City/state: `Longmont, CO`
9. Reached AI Service Setup.
10. Waited while the app attempted to start the local AI service.
11. Observed the app could not reach the AI service and offered `Install local AI runtime`.
12. Clicked `Install local AI runtime` in the app UI.
13. The app window disappeared and `civicnews.exe` was no longer running.
14. Checked Windows Application event log.

## Results

### Install

PASS. The verified NSIS installer completed and installed `civicnews.exe`.

### Launch / first-run onboarding

PARTIAL PASS. The launch stack fix worked for initial startup. The app reached first-run onboarding instead of crashing before onboarding.

### Identity setup

PASS. The app accepted Longmont-local identity values and wrote a community profile with `city: Longmont` and `state: CO`.

### App-managed local AI runtime/model setup

FAIL / BLOCKER. The app reached AI Service Setup and correctly offered an app-managed `Install local AI runtime` control when the local AI service could not be reached. Clicking that control crashed the app.

Observed UI before crash:

- `Local AI Service Connection`
- `Local Ram: 15 GB`
- `Couldn't reach the AI service`
- Message said the private AI service did not start and, on a clean machine, Civic Desk can download and install its local AI runtime.
- Buttons included `Install local AI runtime`, `Retry`, and `Save diagnostics file`.

Observed crash:

- After clicking `Install local AI runtime`, the app disappeared.
- No `civicnews` or `ollama` process remained.
- No `ollama` command or user model store appeared.
- Windows Application log recorded `Application Error` / `APPCRASH`.

Crash details from Windows event log:

- Faulting application: `civicnews.exe`
- Version: `0.2.8.0`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e82657`
- Faulting module: `civicnews.exe`

### CWD / launch method

The app was launched by installed app path after NSIS install. Unlike the two previous artifacts, `792bd22` did reach onboarding by this launch method. The remaining blocker occurs only after the app UI runtime-install action.

### Longmont source discovery and publication workflow

NOT EXERCISED. The app could not complete app-managed local AI runtime setup, so the directive's hard rule prevented manual Ollama/model/PATH repair. Source discovery, source import, scan, lead review, dark-signal review, verification, writer/editor workflows, export, and here.now publish were not attempted.

## Directive Questions

- Did the app install and set up local AI by itself from a clean product state? No. Install and first-run identity succeeded, but app-managed local AI runtime install crashed the app.
- What model was selected and why? None. The app did not reach model recommendation/download after runtime install crashed.
- Did the local AI generate real usable draft content? No. Runtime/model setup did not complete.
- How many leads were found? 0. Source discovery was not reached.
- How many stories/briefs were produced? 0. Writer/editor workflow was not reached.
- Which official sources were used? None.
- Which social/community/dark-signal sources were used? None.
- What writer/editor workflow controls were successfully exercised? None.
- What could not be exercised? Local AI runtime install, model setup, Longmont source discovery, ingestion, lead review, dark-signal review, verification, writing, editing, approval/attestation, export, and here.now publish.
- What broke, exactly where? `Install local AI runtime` in AI Service Setup crashed `civicnews.exe` with Windows exception `0xc00000fd`.
- What is the local exported publication ZIP/path? None produced.
- What is the here.now URL? None produced.
- Is this ready for Scott to use next week to produce a real publication? No. The app now launches and reaches onboarding, but a clean user cannot install/configure the required local AI runtime from the app UI.

## Evidence

Artifacts:

- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/failure-manifest-792bd22.md`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/01-first-run-identity.png`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/02-identity-longmont-next.png`
- `test-comms/artifacts/20260628-full-e2e-longmont-publication-792bd22/03-ai-service-after-20s.png`

Windows event log evidence summary:

- Event provider: `Application Error`
- Event id: `1000`
- Faulting application: `civicnews.exe`
- Exception code: `0xc00000fd`
- Fault offset: `0x0000000000e82657`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: app crashes when user clicks app-managed local AI runtime install

Observed:

On a clean product state, `792bd22` installs and reaches AI Service Setup. The app reports it cannot reach the AI service and offers `Install local AI runtime`. Clicking that app-provided control exits/crashes `civicnews.exe`; Windows logs `APPCRASH` with exception `0xc00000fd`.

Expected:

Clicking `Install local AI runtime` should start the app-managed runtime download/install flow, show progress, and eventually let the user proceed to model setup/readiness without tester-installed dependencies.

Impact:

Full cleanroom E2E is blocked. A normal clean-machine user cannot set up the required local AI runtime through the product, and the tester cannot manually install Ollama or models under the directive.

Repro:

1. Clean product state per directive.
2. Install `test-comms/artifacts/792bd22-windows-launch-stack-fix/The Civic Desk_0.2.8_x64-setup.exe`.
3. Launch installed `civicnews.exe`.
4. Complete identity onboarding with Longmont, CO.
5. On AI Service Setup, wait for `Couldn't reach the AI service`.
6. Click `Install local AI runtime`.
7. Observe app exits; Windows Application log records `APPCRASH` / exception `0xc00000fd`.

## Request For Coder

Please fix the runtime-install button crash in the `792bd22` artifact and post a new artifact/directive. The watcher remains active and I will rerun from a clean product state when instructed.
