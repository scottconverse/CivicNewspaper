# Tester Report - identity focus rerun 4c5b239

Date: 2026-06-28  
Tester machine: Windows laptop, Intel i7-13620H, 16 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU  
Repo: https://github.com/scottconverse/CivicNewspaper.git  
Product branch: stable-readiness-local-gates  
Product commit: 4c5b239eeaf9217463dcf1f2ee09885c573de899  
Directive: test-comms/directives/20260628-rerun-identity-focus-after-4c5b239.md

## Coordination State

- `pwd`: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Local HEAD before report commit: `af325c7 test-comms: add msi civic directive visibility report [skip ci]`
- Remote HEAD before report commit: `af325c7 test-comms: add msi civic directive visibility report [skip ci]`
- Active directive filename: `test-comms/directives/20260628-rerun-identity-focus-after-4c5b239.md`

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

1. Pulled `test-comms/cleanroom-coder-tester` and reread `README.md`, `protocol.md`, `prompts/tester-codex-desktop-prompt.md`, `TESTER_READ_ME_FIRST.md`, and `ACTIVE_DIRECTIVE.md`.
2. Followed active directive pointer to `test-comms/directives/20260628-rerun-identity-focus-after-4c5b239.md`.
3. Verified product branch and commit:
   - `origin/stable-readiness-local-gates`
   - `4c5b239eeaf9217463dcf1f2ee09885c573de899`
   - subject `Fix first-run identity focus and hide runtime console`
4. Verified installer artifact hashes:
   - NSIS EXE: `AC0D62B329A4BD995366BC023913302ABECC263358D66E0E642C04115C2B5F96`
   - MSI: `E939A8830BFA639AA7133FEB4EA51F0F08160570F0FA2466ADB81E5F348BE3AE`
5. Stopped existing Civic Desk, Ollama, installer, and helper terminal processes.
6. Removed only Civic Desk app/runtime state and local Ollama/model state.
7. Installed the provided NSIS setup artifact silently.
8. Launched the installed real desktop app.
9. Set the app window to 1280x720.
10. Confirmed the Publication Name field had initial visible focus.
11. Attempted to fill Publication Name, Editor Name, Publisher Type, City, and State at 1280x720 without moving the window offscreen.
12. Stopped at the exact setup blocker: the focused Publication Name input did not accept normal keyboard text, and a direct click/type probe left the field blank.

## Results

- Clean install/reset: PASS
- Real desktop app launch: PASS
- App set to 1280x720: PASS
- Publication Name has initial visible focus: PASS
- Publication Name can be filled normally at 1280x720: FAIL/BLOCKED
- Editor Name / Publisher Type / City / State completion: NOT REACHED because Publication Name could not be entered
- First-run action row visibility/reachability: NOT REACHED
- Helper terminal/console during runtime setup: NOT REACHED; no runtime setup was reached in this run
- App-managed runtime setup starts Ollama: NOT REACHED
- `GET /api/tags` after runtime setup: NOT REACHED
- Model download/model tags: NOT REACHED
- Full Longmont publication workflow and 12-hour soak: NOT RUN because setup gate failed

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

Screenshots/artifacts are under `test-comms/artifacts/20260628-identity-focus-rerun-4c5b239/`.

Key screenshots:

- `01-first-run-1280x720.png` - clean first-run screen; Publication Name has visible initial focus.
- `02-identity-filled-1280.png` - app foregrounded at 1280x720 after input attempts; Publication Name still blank.
- `03-identity-filled-1280.png` - repeated keyboard entry attempt; Publication Name still blank.
- `04-publication-abc-test.png` - direct click/type probe using `ABC`; Publication Name remained blank.

I deleted two screenshots that captured non-product foreground windows with private/local-path context and did not commit them.

Process state at stop:

```text
civicnews running: yes
ollama running: no, runtime setup was not reached
helper terminal: no helper terminal remained running at stop
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Publication Name has focus but does not accept normal text entry

Observed: In 4c5b239, the Publication Name field visibly receives initial focus at 1280x720, which is an improvement over 4381f3f. However, normal keyboard input did not populate the field. Repeated attempts to type the full publication name after launch left the field blank. A direct click into the visible Publication Name input followed by typing `ABC` also left the field blank.

Expected: With visible focus in Publication Name, normal keyboard entry should fill the publication name so first-run setup can continue.

Impact: The required cleanroom setup retest cannot proceed to identity completion, runtime setup, model pull, Longmont publication workflow, or the 12-hour soak.

Repro: Clean install 4c5b239, launch The Civic Desk, set window to 1280x720, observe Publication Name focus, type text.

## Request For Coder

Please fix the Publication Name input so visible focus also accepts normal keyboard/click text entry at 1280x720. The initial focus fix appears partially successful, but the setup gate still fails before runtime/model testing can begin.
