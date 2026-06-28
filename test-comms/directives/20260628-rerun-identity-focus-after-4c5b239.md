# Directive: rerun cleanroom after identity focus/runtime-console fix 4c5b239

Tester, keep your 15-minute watcher armed. This directive replaces the `4381f3f` run, which stopped at first-run identity because Publication Name could not be reliably focused at 1280x720 and a helper terminal stole focus.

## Product branch and artifact

- Product branch: `stable-readiness-local-gates`
- Product commit to test: `4c5b239`
- Commit subject: `Fix first-run identity focus and hide runtime console`
- Installer artifacts:
  - `test-comms/artifacts/20260628-identity-focus-rerun-4c5b239/The Civic Desk_0.2.8_x64-setup.exe`
  - `test-comms/artifacts/20260628-identity-focus-rerun-4c5b239/The Civic Desk_0.2.8_x64_en-US.msi`
- Expected SHA256:
  - NSIS setup EXE: `AC0D62B329A4BD995366BC023913302ABECC263358D66E0E642C04115C2B5F96`
  - MSI: `E939A8830BFA639AA7133FEB4EA51F0F08160570F0FA2466ADB81E5F348BE3AE`

## Clean reset boundary

Wipe only CivicNewspaper, Ollama, local models, test files, app data, PATH changes, and related prerequisites. Leave Windows, the user account, browser, Git, and Codex tester environment intact.

Do not manually install Ollama, models, document tools, OCR tools, or runtime prerequisites. If the product cannot install or drive its own required runtime pieces, report that as a product failure and stop at the exact break.

## Required setup retest

1. Install the new artifact as an end user would.
2. Launch The Civic Desk.
3. Set the app window to 1280x720.
4. Confirm the Publication Name field has initial focus or can be focused normally.
5. Fill Publication Name, Editor Name, Publisher Type, City, and State at 1280x720 without moving the window offscreen.
6. Confirm the first-run action row remains visible/reachable while the body scrolls.
7. Confirm no helper/terminal/console window appears or steals focus during app-managed runtime setup.
8. Confirm app-managed runtime setup starts Ollama automatically.
9. Confirm `http://127.0.0.1:11434/api/tags` is reachable after runtime setup.
10. At the no-model prompt:
    - Confirm `Download qwen2.5:7b` is visible/reachable at 1280x720, or reachable by normal in-card scrolling without moving the window offscreen.
    - Click the body `Download qwen2.5:7b` button.
    - Confirm the UI moves to Step 3 and shows `Starting pull...`, progress, completion, or a specific failure.
    - If the body button passes, clean reset again or use an equivalent clean no-model state and confirm footer `Next` also starts the pull path.
11. Wait for the model download to complete.
12. Confirm `GET /api/tags` lists `qwen2.5:7b` or the exact downloaded tag.

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

## Report and artifacts

Write:

- `test-comms/reports/20260628-identity-focus-rerun-report-4c5b239.md`

Use plain English for the human owner. Include:

- pass/fail status,
- exact identity/setup/model result,
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

- `test-comms/artifacts/20260628-identity-focus-rerun-4c5b239/`

Commit reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
