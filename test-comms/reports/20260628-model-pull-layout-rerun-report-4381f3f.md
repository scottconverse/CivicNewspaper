# Tester Report - model pull/layout rerun 4381f3f

Date: 2026-06-28  
Tester machine: Windows laptop, Intel i7-13620H, 16 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU  
Repo: https://github.com/scottconverse/CivicNewspaper.git  
Product branch: stable-readiness-local-gates  
Product commit: 4381f3fb2500c7da92ee49c8baab04d1bd23d408  
Directive: test-comms/directives/20260628-rerun-model-pull-layout-after-4381f3f.md

## Environment

- Windows version: Windows 10 Home, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 16 GB installed
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Node: not installed / not on PATH
- Rust: not installed / not on PATH
- npm: not installed / not on PATH
- Ollama installed/running before test: clean-reset removed prior app-managed runtime state; no tester-installed Ollama prerequisite was used
- Models present before test: clean-reset removed local model state

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread the README, protocol, tester prompt, and directives.
2. Found new directive `20260628-rerun-model-pull-layout-after-4381f3f.md`.
3. Verified product branch and commit:
   - `origin/stable-readiness-local-gates`
   - `4381f3fb2500c7da92ee49c8baab04d1bd23d408`
   - subject `Fix first-run model pull and setup layout`
4. Verified installer artifact hashes:
   - NSIS EXE: `F200A5B3841BA2F393984710933DAEABF16BBFDE61340E53404B063E95A674F3`
   - MSI: `47798151D7944E8C874ED517E4C29E6B7865B6FF2EE6FA67F9542033063539D0`
5. Stopped existing Civic Desk, installer, helper terminal, and Ollama processes.
6. Removed only Civic Desk app/runtime state and local Ollama/model state.
7. Installed the provided NSIS setup artifact silently.
8. Launched the installed real desktop app.
9. Set the app window to 1280x720.
10. Attempted to complete first-run identity at 1280x720 without moving the window offscreen.
11. Stopped at the exact blocker: the Publication Name input could not be focused/filled through normal click/keyboard entry, while the Editor Name field accepted the typed text instead.

## Results

- Clean install/reset: PASS
- Real desktop app launch: PASS
- App set to 1280x720: PASS
- First-run identity at 1280x720 without moving window offscreen: BLOCKED
- Action row visible/reachable while body scrolls: NOT REACHED
- App-managed runtime setup starts Ollama automatically: NOT REACHED in this run
- `GET /api/tags` after runtime setup: NOT REACHED
- No-model prompt and model pull: NOT REACHED
- Full Longmont publication workflow: NOT RUN because directive says to stop at exact break

Counts requested by directive:

- Sources: 0
- Leads: 0
- Drafts: 0
- Approved items: 0
- Held/cut items: 0
- Published stories: 0
- here.now URL: none
- Local output folder path: none
- ZIP path: none

## Evidence

Screenshots/artifacts are under `test-comms/artifacts/20260628-model-pull-layout-rerun-4381f3f/`.

Key screenshots:

- `01-first-run-1280x720.png` - clean first-run identity screen at 1280x720.
- `06-app-foreground-after-helper-kill.png` - app restored after a helper terminal stole focus.
- `07-identity-normal-entry-after-helper-kill.png` - normal click/type attempt did not populate the visible Publication Name field.
- `08-identity-correct-targets.png` - clicking/typing at the visible field positions put the publication text into Editor Name, not Publication Name.
- `09-publication-final-focus-attempt.png` - final direct click into Publication Name area inserted `ABC` into Editor Name, proving the publication input could not be targeted normally.

I removed two screenshots that captured a private local path in the helper terminal title bar and did not commit them.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker - First-run Publication Name input cannot be filled reliably at 1280x720

Observed: At 1280x720, the first-run identity screen renders the Publication Name field, but repeated normal attempts to click/type into that field did not populate it. When I clicked the visible Publication Name input area and typed, the text was either ignored or inserted into the Editor Name field below. The final probe typed `ABC` after clicking the Publication Name area, and `ABC` appeared inside Editor Name.

Expected: A user should be able to click Publication Name and enter the publication name at 1280x720 without offscreen window movement or database manipulation.

Impact: The required setup retest cannot reach runtime setup or model pull. The directive explicitly required completing first-run identity at 1280x720 without moving the window offscreen.

Repro: Clean install 4381f3f, launch the desktop app, set window to 1280x720, attempt to click/fill Publication Name.

### Major - Helper terminal steals focus and exposes local machine path

Observed: During the first-run attempts, a helper terminal window appeared in front of the app and stole keyboard focus. Its title bar exposed a private local filesystem path, so I deleted the affected screenshots and excluded them from the commit.

Expected: App-managed helper/runtime work should not surface a terminal window with private machine paths during onboarding.

Impact: It interrupts user input and creates private-data screenshot risk during cleanroom validation.

Repro: Launch the installed app during clean first-run and begin keyboard entry; helper terminal appeared twice during this run.

## Request For Coder

Please fix the 1280x720 first-run identity input hit-testing/focus issue before another model-pull rerun. I could not honestly reach the runtime/model steps because the visible Publication Name input was not fillable through normal end-user interaction, and a helper terminal repeatedly stole focus.
