# Directive: rerun cleanroom with starter profile fallback 2883286

Tester, keep your 15-minute watcher armed and stay in the CivicNewspaper context.

You are on the separate tester machine running as `msi\civic`. Do not use `C:\Users\instynct\...`; that is the coder machine. The approved coordination checkout path on your machine is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Single source of truth:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`

## Why this rerun exists

The cleanroom machine still failed to type into Publication Name even after the WebView-safe input change. Coder added mouse-driven starter profile buttons in product commit `2883286` so first-run setup is not hard-blocked by keyboard event delivery. Use the Longmont starter profile for this test.

Keyboard entry failure remains a product/test-environment finding. Do not hide it. Record whether typing still fails, but do not stop there if the Longmont starter profile works.

## Product branch and artifact

- Product branch: `stable-readiness-local-gates`
- Product commit to test: `2883286`
- Commit subject: `Add starter identity profiles for first-run setup`
- Installer artifacts:
  - `test-comms/artifacts/20260628-starter-profile-rerun-2883286/The Civic Desk_0.2.8_x64-setup.exe`
  - `test-comms/artifacts/20260628-starter-profile-rerun-2883286/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256:
  - NSIS setup EXE: `37219037C8CFC5399F19872BC9A1CE313452DBA54E6E2AB11F83CA8FCAA1FC2A`
  - MSI: `7F1BAB8868364A9D733583615FACC520DA0960DD90AE98068819839467A8D2D1`

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
9. Click the `Longmont` starter profile button.
10. Confirm Publication Name, Editor Name, City, and State visibly fill without keyboard entry.
11. Click Next and verify the identity values persist into the next setup step.
12. Confirm the first-run action row remains visible/reachable while the body scrolls.
13. Confirm no helper/terminal/console window appears or steals focus during app-managed runtime setup.
14. Confirm app-managed runtime setup starts Ollama automatically.
15. Confirm `http://127.0.0.1:11434/api/tags` is reachable after runtime setup.
16. At the no-model prompt:
    - Confirm `Download qwen2.5:7b` is visible/reachable at 1280x720, or reachable by normal in-card scrolling without moving the window offscreen.
    - Click the body `Download qwen2.5:7b` button.
    - Confirm the UI moves to Step 3 and shows `Starting pull...`, progress, completion, or a specific failure.
    - If the body button passes, clean reset again or use an equivalent clean no-model state and confirm footer `Next` also starts the pull path.
17. Wait for the model download to complete.
18. Confirm `GET /api/tags` lists `qwen2.5:7b` or the exact downloaded tag.

If the starter profile cannot fill fields, or any setup/model gate after that fails, stop and report the exact failure with screenshots, visible text, logs, listener/process state, and `GET /api/tags`.

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

`test-comms/reports/20260628-starter-profile-rerun-report-2883286.md`

Use plain English for the human owner. Include:

- pass/fail status,
- exact identity/setup/model result,
- whether keyboard typing still failed,
- whether the Longmont starter profile filled the identity fields,
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

`test-comms/artifacts/20260628-starter-profile-rerun-2883286/`

Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
