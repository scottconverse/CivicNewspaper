# Directive: rerun cleanroom with stuck-input recovery 30035ac

Tester, keep your 15-minute watcher armed and stay in the CivicNewspaper context.

You are on the separate tester machine running as `msi\civic`. Do not use `C:\Users\instynct\...`; that is the coder machine. The approved coordination checkout path on your machine is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Single source of truth:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`

## Why this rerun exists

The tester machine is rendering the packaged app but not delivering keyboard, click, or link activation events to the first-run WebView. Product commit `30035ac` adds a stuck-input recovery: if Step 1 receives no pointer, click, keydown, or input events for about 12 seconds while identity fields stay empty, the app applies a Longmont starter profile, shows a recovery notice, persists identity, and advances to Step 2.

Keyboard/click failure remains a finding. Record it, then wait for the recovery path and continue the full E2E if it advances.

## Product branch and artifact

- Product branch: `stable-readiness-local-gates`
- Product commit to test: `30035ac`
- Commit subject: `Add stuck-input recovery for first-run setup`
- Installer artifacts:
  - `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/The Civic Desk_0.2.8_x64-setup.exe`
  - `test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256:
  - NSIS setup EXE: `10A19060494961BE9E85E0FA07E011232A68A9C1CFDE0C4FD733DD7BD20D3296`
  - MSI: `E6CB22A236E018FD414B494CCADFFD697A819D44D56488195F897643199F241E`

## Clean reset boundary

Wipe only CivicNewspaper, Ollama, local models, test files, app data, PATH changes, and related prerequisites. Leave Windows, the user account, browser, Git, and Codex tester environment intact.

Do not manually install Ollama, models, document tools, OCR tools, or runtime prerequisites. If the product cannot install or drive its own required runtime pieces, report that as a product failure and stop at the exact break.

## Required setup retest

1. Pull the coordination branch and reread `test-comms/ACTIVE_DIRECTIVE.md`.
2. Verify artifact hashes above before install.
3. Clean reset using the boundary above.
4. Install the new artifact as an end user would.
5. Launch The Civic Desk.
6. Set the app window to 1280x720.
7. Confirm the Publication Name field has initial visible focus.
8. Try typing `ABC` into Publication Name and record whether it appears. If it still fails, continue.
9. Try clicking the visible `Longmont` starter profile link and record whether it advances. If it still fails, continue.
10. If controls still do not respond, do not touch the app for at least 15 seconds. Confirm whether the stuck-input recovery notice appears and the app advances to Step 2.
11. Confirm no helper/terminal/console window appears or steals focus during app-managed runtime setup.
12. Confirm app-managed runtime setup starts Ollama automatically.
13. Confirm `http://127.0.0.1:11434/api/tags` is reachable after runtime setup.
14. At the no-model prompt:
    - Confirm `Download qwen2.5:7b` is visible/reachable at 1280x720, or reachable by normal in-card scrolling without moving the window offscreen.
    - Click the body `Download qwen2.5:7b` button if controls are responding. If controls are still not responding, report that as the next exact blocker.
    - Confirm the UI moves to Step 3 and shows `Starting pull...`, progress, completion, or a specific failure.
15. Wait for the model download to complete.
16. Confirm `GET /api/tags` lists `qwen2.5:7b` or the exact downloaded tag.

If the stuck-input recovery cannot advance to Step 2, or any setup/model gate after that fails, stop and report the exact failure with screenshots, visible text, logs, listener/process state, and `GET /api/tags`.

## Full Longmont publication workflow, only if setup/model passes

Target city: Longmont, Colorado.

Use only public, readable sources without account login. Public-page scraping is allowed. Do not access private groups, private content, credentials, or paywalled/private material.

Drive the product like an end user. Do not manipulate the database directly.

1. Configure a neutral local publication identity and defaults.
2. Add/discover/import sources covering official sources, local media, and public social/community sources.
3. Run source review and save useful candidates.
4. Run Daily Scan from the UI.
5. If fewer than 10 leads appear, expand sources/search from inside the product and scan again.
6. Target 10-25 leads and 5-10 reader-facing Longmont stories/briefs/watchlist items. One lead/story is a failure.
7. Exercise writer/editor workflow:
   - open leads,
   - draft with local AI,
   - edit and save if keyboard/input works,
   - run the press-freedom/legal-risk advisor on at least one story,
   - approve at least 5 stories/briefs if possible,
   - send at least one item back or put it on hold,
   - cut/kill at least one unsuitable item if the UI supports it,
   - verify software never vetoes the editor.
8. Compile a publication issue.
9. Export the ZIP/static output package.
10. Publish anonymously to here.now using the product UI. This anonymous here.now test publish is authorized.
11. Open and review the published site homepage, at least three story pages, RSS, about/ethics/corrections pages if present, source/evidence display, mobile/narrow layout, and share package.

## Soak requirement

After the setup/model gate passes and before declaring the product ready, run a 12-hour cleanroom soak:

- Keep the app installed and running.
- Do not manually repair dependencies.
- Every 15 minutes, record whether the app is still responsive, whether the local backend is reachable, whether Ollama is still reachable, whether the model remains listed, and whether any unexpected windows/prompts appeared.
- At least once during the soak, reopen the exported site/package and verify it still exists.
- At the end, write a human-readable soak report under `test-comms/reports/` and put logs/screenshots under the matching `test-comms/artifacts/` folder.

If the app crashes, helper windows appear, Ollama disappears, model state is lost, the output package disappears, or here.now publish/report data is missing, stop the soak and report the exact failure.

## Report and artifacts

Write:

`test-comms/reports/20260628-stuck-input-recovery-rerun-report-30035ac.md`

Use plain English for the human owner. Include:

- pass/fail status,
- exact identity/setup/model result,
- whether keyboard typing still failed,
- whether click/link activation still failed,
- whether stuck-input recovery advanced to Step 2,
- whether a helper terminal appeared,
- what the product installed,
- whether model download completed,
- whether AI-generated drafts were created from real Longmont source material,
- source/lead/draft/approval/hold/cut/story counts,
- here.now URL,
- local output folder path,
- ZIP path,
- screenshots,
- exact breakpoints and product bugs.

Put screenshots, logs, exported ZIP, and any copied publication output under:

`test-comms/artifacts/20260628-stuck-input-recovery-rerun-30035ac/`

Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
