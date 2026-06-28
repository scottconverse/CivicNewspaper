# Tester Report - model download rerun c51b49d

Date: 2026-06-28  
Tester machine: Windows laptop, Intel i7-13620H, 16 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU  
Repo: https://github.com/scottconverse/CivicNewspaper.git  
Product branch: stable-readiness-local-gates  
Product commit: c51b49de79e767676b04865acea84231bb44d662  
Directive: test-comms/directives/20260628-rerun-model-download-after-c51b49d.md

## Environment

- Windows version: Windows 10 Home, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 16 GB installed, app reported 15 GB local RAM
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: not materially changed from prior run, about 351 GB free on C:
- Node: not installed / not on PATH
- Rust: not installed / not on PATH
- npm: not installed / not on PATH
- Ollama installed/running before test: clean-reset removed prior app-managed runtime state; no tester-installed Ollama prerequisite was used
- Ollama running after app setup: yes, app-managed process listening on 127.0.0.1:11434
- Models present after runtime setup and download attempts: none; `GET /api/tags` returned `{"models":[]}`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread the protocol, tester prompt, and directives.
2. Found new directive `20260628-rerun-model-download-after-c51b49d.md`.
3. Verified product branch and commit:
   - `origin/stable-readiness-local-gates`
   - `c51b49de79e767676b04865acea84231bb44d662`
   - subject `Start model download from first-run prompt`
4. Verified installer artifact hashes:
   - NSIS EXE: `80EA262CA15AC4CAB69D3D1ABC4C1BD3569D76CBDB42F851D189897AE41DB60A`
   - MSI: `663724E48D4248082CF101DD916B88F913E3B0E0AE78EE8ADC0F3CC3AECECF49`
5. Stopped existing `civicnews`, installer, and `ollama` processes.
6. Removed Civic Desk app/runtime state and local Ollama/model state only.
7. Installed `The Civic Desk_0.2.8_x64-setup.exe` silently.
8. Launched the installed real desktop app.
9. Set the window to 1280x720 for constrained-layout testing.
10. Completed first-run identity through the UI for a neutral Longmont publication.
11. Used keyboard traversal plus an oversized window workaround because the lower identity controls and action row were not reachable/visible at 1280x720.
12. Confirmed app-managed runtime setup started automatically and showed `Installing...`.
13. Waited for app-managed Ollama to start.
14. Confirmed runtime listeners:
    - app backend on 127.0.0.1:12053
    - Ollama on 127.0.0.1:11434
15. Rechecked `GET /api/tags`; model list was empty before model-download testing.
16. At the model prompt, checked 1280x720 layout:
    - prompt visible after scroll,
    - footer partly visible/clipped,
    - new body `Download qwen2.5:7b` button was not visible at 1280x720.
17. Used the oversized/offscreen window workaround to reveal the body `Download qwen2.5:7b` button.
18. Clicked the visible body download button multiple ways, including a slow foreground click.
19. Attempted keyboard traversal for footer controls and rechecked the model list.
20. Stopped at model download because no model pull started and `GET /api/tags` stayed `{"models":[]}`.

## Results

- Clean install/reset: PASS
- Real desktop app launch: PASS
- 1280x720 constrained layout retest: FAIL, controls remain clipped/unreachable
- First-run identity completion: PARTIAL/PASS with workaround; fields were fillable, but lower controls required oversized/offscreen navigation and were easy to mis-target
- App-managed runtime auto-start: PASS
- App-managed Ollama process and port 11434: PASS
- Visible/reachable `Download qwen2.5:7b` button in main setup body at 1280x720: FAIL
- Body `Download qwen2.5:7b` button exists with oversized workaround: PASS
- Body `Download qwen2.5:7b` starts model pull: FAIL/BLOCKED
- Footer `Next` path starts `pull_ollama_model`: NOT CONFIRMED; footer path remained hard to reach safely and no model pull started during traversal/click attempts
- Model download completed: FAIL
- `GET /api/tags` lists downloaded model: FAIL, still `{"models":[]}`
- Full Longmont source/draft/export/publish workflow: NOT RUN because directive says to stop at exact model-download failure

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

Screenshots/artifacts are under `test-comms/artifacts/20260628-model-download-rerun-c51b49d/`.

Key screenshots:

- `01-first-run-1280x720.png` - clean first-run at 1280x720.
- `09-identity-visible-fields.png` - publication/editor fields filled.
- `22-after-advance-attempt.png` - runtime step shows `Installing...` after identity action.
- `23-runtime-ready-1280.png` - runtime-ready step at 1280x720 before scrolling.
- `24-model-body-button-1280.png` and `25-model-prompt-scrolled-more.png` - 1280x720 model prompt; new body download button is not visible/reachable in the viewport.
- `28-model-wide-footer.png` - oversized workaround reveals `Download qwen2.5:7b` body button.
- `29-after-download-click.png` and `30-download-button-tab-focus.png` - after body button click/focus attempts, UI remains unchanged.
- `32-footer-row-wide.png` - final state still shows body download button and no progress.

Runtime/model checks:

```text
App backend listen: 127.0.0.1:12053
Ollama listen:      127.0.0.1:11434
Ollama tags:        {"models":[]}
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

### Blocker - Model download still does not start from first-run setup

Observed: c51b49d adds a visible `Download qwen2.5:7b` button in the model prompt body when the window is enlarged/offscreen enough to expose it. Clicking that button did not change the UI to progress, did not start visible download activity, and did not add any model to Ollama. `GET /api/tags` remained `{"models":[]}` after click attempts.

Expected: Clicking `Download qwen2.5:7b` starts `pull_ollama_model`, shows progress or a specific failure, and eventually makes `qwen2.5:7b` appear in `GET /api/tags`.

Impact: The full cleanroom publication workflow cannot continue because the local model is still unavailable through the product setup path.

Repro: Clean install c51b49d, complete identity, allow app-managed runtime setup to start Ollama, reveal the model prompt, click `Download qwen2.5:7b`.

### Major - 1280x720 setup layout still hides model download controls

Observed: At 1280x720, the model prompt can be scrolled into view, but the new body `Download qwen2.5:7b` button is not visible/reachable. The footer row is also clipped, making `Skip`/`Next` difficult to operate safely. I had to use an oversized/offscreen window workaround to reveal the body button.

Expected: The directive specifically requested a visible, reachable `Download qwen2.5:7b` button in the main setup body and a usable constrained layout.

Impact: A real user at the tested resolution may not be able to find or click the required model download action.

Repro: Run the first-run setup at 1280x720 through the runtime-ready model prompt.

### Minor - Identity form remains fragile in constrained/offscreen navigation

Observed: Publication/editor fields were fillable, but lower identity controls and action row were clipped at 1280x720. The oversized workaround made labels/fields partially offscreen and easy to mis-target. I corrected what I could through UI, then advanced with keyboard traversal.

Expected: Identity setup should keep field labels, values, and actions visible enough for reliable end-user input.

Impact: Not the final blocker for this directive, but it increases risk of incorrect setup data and slows cleanroom validation.

Repro: Fill identity at 1280x720 and attempt to reach the lower fields/action row.

## Request For Coder

Runtime auto-start still passes in c51b49d, and the body `Download qwen2.5:7b` button now exists. Please fix the button action so it actually starts the model pull, and rework the 1280x720 setup shell so the body download button and footer controls are visible/reachable without moving the window offscreen.
