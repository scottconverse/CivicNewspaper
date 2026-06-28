# Directive: rerun cleanroom after WebView-safe identity fix 65d97f4

Tester, keep your 15-minute watcher armed and stay in the CivicNewspaper context.

You are on the separate tester machine running as `msi\civic`. Do not use `C:\Users\instynct\...`; that is the coder machine. The approved coordination checkout path on your machine is:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Single source of truth:

- Repo: `https://github.com/scottconverse/CivicNewspaper`
- Coordination branch: `test-comms/cleanroom-coder-tester`
- Active pointer: `test-comms/ACTIVE_DIRECTIVE.md`

## Why this rerun exists

The 5bdd7a8 cleanroom run still failed at first-run setup: Publication Name showed focus/caret but did not display typed text. Coder made the first-run identity fields WebView-safe in product commit `65d97f4` by removing controlled `value` props from the identity fields, using native DOM text entry, syncing via `onInput`/`onChange`, and reading DOM refs on Next.

## Product branch and artifact

- Product branch: `stable-readiness-local-gates`
- Product commit to test: `65d97f4`
- Commit subject: `Make first-run identity fields WebView-safe`
- Installer artifacts:
  - `test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/The Civic Desk_0.2.8_x64-setup.exe`
  - `test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256:
  - NSIS setup EXE: `439288E91E78F65783D450C03F2A001D3B4E973EDBEE41FE4F6A5DE76C6C374B`
  - MSI: `D828D3C5B43047ED8B86F7EB2AB5F197BE0747708DF2C5D07216005C7BA00B5A`

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
8. Type `ABC` into Publication Name. Confirm it appears.
9. Clear or replace it, then fill Publication Name, Editor Name, Publisher Type, City, and State at 1280x720 without moving the window offscreen.
10. Click Next and verify the identity values persist into the next setup step.
11. Confirm the first-run action row remains visible/reachable while the body scrolls.
12. Confirm no helper/terminal/console window appears or steals focus during app-managed runtime setup.
13. Confirm app-managed runtime setup starts Ollama automatically.
14. Confirm `http://127.0.0.1:11434/api/tags` is reachable after runtime setup.
15. At the no-model prompt:
    - Confirm `Download qwen2.5:7b` is visible/reachable at 1280x720, or reachable by normal in-card scrolling without moving the window offscreen.
    - Click the body `Download qwen2.5:7b` button.
    - Confirm the UI moves to Step 3 and shows `Starting pull...`, progress, completion, or a specific failure.
    - If the body button passes, clean reset again or use an equivalent clean no-model state and confirm footer `Next` also starts the pull path.
16. Wait for the model download to complete.
17. Confirm `GET /api/tags` lists `qwen2.5:7b` or the exact downloaded tag.

If any setup/model gate fails, stop and report the exact failure with screenshots, visible text, logs, listener/process state, and `GET /api/tags`.

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
   - edit and save,
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

`test-comms/reports/20260628-webview-safe-identity-rerun-report-65d97f4.md`

Use plain English for the human owner. Include:

- pass/fail status,
- exact identity/setup/model result,
- whether Publication Name accepted `ABC` and normal publication text,
- whether the other identity fields accepted text/selection,
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

`test-comms/artifacts/20260628-webview-safe-identity-rerun-65d97f4/`

Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
