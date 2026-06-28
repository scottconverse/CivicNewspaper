# Tester Report - auto runtime rerun 4db14b5

Date: 2026-06-28  
Tester machine: Windows laptop, Intel i7-13620H, 16 GB RAM, Intel UHD + NVIDIA RTX 4050 Laptop GPU  
Repo: https://github.com/scottconverse/CivicNewspaper.git  
Product branch: stable-readiness-local-gates  
Product commit: 4db14b59c622bd14e523200340b66e9b249c2466  
Directive: test-comms/directives/20260628-rerun-auto-runtime-after-4db14b5.md

## Environment

- Windows version: Windows 10 Home, build 26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 16 GB installed, app reported 15 GB local RAM
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: about 351 GB free on C:
- Node: not installed / not on PATH
- Rust: not installed / not on PATH
- npm: not installed / not on PATH
- Ollama installed/running before test: clean-reset removed prior runtime state; no tester-installed Ollama prerequisite was used
- Ollama running after app setup: yes, app-managed process listening on 127.0.0.1:11434
- Models present after app setup and after model Next attempts: none; `GET /api/tags` returned `{"models":[]}`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread the protocol, tester prompt, and directives.
2. Verified current directive `20260628-rerun-auto-runtime-after-4db14b5.md`.
3. Verified preferred installer SHA256:
   - `The Civic Desk_0.2.8_x64-setup.exe`
   - `45FAF025082DD88FB636A874F3BB43F9E74904849780A5084E34F99CEB60724A`
4. Verified product commit from `origin/stable-readiness-local-gates`:
   - `4db14b59c622bd14e523200340b66e9b249c2466`
   - subject `Auto-start first-run runtime setup`
5. Stopped any existing `civicnews`, installer, browser helper, and `ollama` processes.
6. Removed product app data, runtime data, and package residue from the normal Windows app/runtime locations.
7. Installed the preferred NSIS installer silently.
8. Launched the real desktop app from the installed app executable.
9. Completed first-run identity for Longmont, Colorado through the app UI.
10. Advanced to Step 2 and did not click `Install local AI runtime`.
11. Waited for the app to auto-start runtime setup.
12. Observed runtime progress text and disabled install state during setup.
13. Confirmed app-managed `ollama` process appeared and port 11434 listened.
14. Continued to the model recommendation prompt.
15. Attempted to start the recommended model download through the app UI:
    - At 1280x720, the model action footer was not reachable/visible after scroll, PageDown, End, or Ctrl+End.
    - Used an oversized/offscreen window workaround to expose the footer.
    - Focused the visible `Next` button by keyboard traversal and activated it with Enter and Space.
    - Also attempted a visible click on `Next`.
16. Rechecked `GET /api/tags` after activation attempts; model list remained empty.

## Results

- Clean install/reset: PASS
- Real desktop app launch: PASS
- First-run identity path: PASS, with UI focus/scroll awkwardness observed
- App-driven runtime auto-start without clicking install: PASS
- Visible runtime progress while setup was running: PASS
- App-managed `ollama` process and port 11434: PASS
- Recommended model download through app: BLOCKED
- Source import, scan/story queue, draft generation, citation integrity, editor workflow, advisor, export ZIP, here.now publish: NOT RUN because the directive says to stop at the exact blocker

## Evidence

Screenshots/artifacts are under `test-comms/artifacts/20260628-auto-runtime-rerun-4db14b5/`.

Key screenshots:

- `05-identity-filled.png` - first-run identity fields visibly populated for publication/editor.
- `07-step2-auto-runtime.png` - Step 2 shows automatic runtime setup in progress, with install state disabled.
- `08-runtime-after-30s.png` - runtime setup completed enough for the app to report the AI service ready and recommend `qwen2.5:7b`.
- `16-model-screen-1280x720.png` - normal 1280x720 frame shows the model step but not the footer/action row.
- `18-model-screen-wheel.png` and `21-model-after-end.png` - scroll/end attempts still did not expose the action footer at 1280x720.
- `24-after-tab-to-next.png` - oversized/offscreen workaround shows keyboard focus on the `Next` button, avoiding the `Skip for now` path.
- `25-after-enter-next.png` and `26-after-space-next.png` - activating focused `Next` left the model prompt in place; no model appeared in Ollama.

Runtime checks:

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

### Blocker - Recommended model download does not start from the first-run model prompt

Observed: After app-managed runtime setup succeeded, the app showed `The AI service is ready. Download a recommended model?` and recommended `qwen2.5:7b`. Activating the visible/focused `Next` button by click, Enter, and Space did not start a model download, did not change the UI to progress, and did not add any model to Ollama. `GET /api/tags` stayed `{"models":[]}`.

Expected: The recommended model download starts through the app, with visible progress or a specific failure.

Impact: The full cleanroom release gate cannot continue because scan/draft generation depends on the model path, and the directive requires stopping at the exact blocker.

Repro: Clean install 4db14b5, complete identity, allow Step 2 runtime auto-start to finish, reach the recommended model prompt, focus `Next`, activate it.

### Major - Model action footer is not reachable at 1280x720

Observed: With the app window positioned at 1280x720, the model prompt content is visible, but the footer/action row is below the viewport. PageDown, End, Ctrl+End, and mouse wheel scrolling did not expose the action row. I had to move/resize the window offscreen to see `Skip for now` and `Next`.

Expected: The 100vh/sticky-action shell keeps the action row reachable at 1280x720.

Impact: A cleanroom user on the tested resolution cannot reliably complete model setup without window-position workarounds.

Repro: Open Step 2 model prompt at 1280x720 after runtime setup succeeds.

### Minor - First-run identity form remains awkward to navigate in constrained view

Observed: Identity entry succeeded, but lower fields required awkward scrolling/focus movement and were harder to verify visually than the top fields.

Expected: Identity fields and actions should remain straightforward to reach and confirm in the first-run shell.

Impact: Not blocking for this rerun, but it increases cleanroom testing/user setup friction.

Repro: Start first-run identity at the tested resolution and fill publication/editor/city/state fields.

## Request For Coder

Runtime auto-start appears fixed in 4db14b5. Please fix or clarify the next model prompt action: either `Next` should start the recommended model pull with visible progress, or the UI should expose a distinct reachable `Download` action. Also recheck the 1280x720 first-run shell because the model action footer was not reachable without an offscreen window workaround.
