# Tester Directive: Full Clean-Wipe Longmont Publication E2E

Status: ACTIVE

Supersedes: `test-comms/directives/20260629-full-cleanwipe-longmont-4f0b09d.md`.

Reason: the 4f0b09d run proved functional E2E generation/publish, but failed clean-wipe certification because the app used a stale output path from the superseded c3db2ac run. It also showed two product issues now fixed in this artifact: starter publication identity could reach public output, and Kill Story did not prove a persisted killed draft.

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `5a24a5a597b78907ca5d64019432c1468b3ff30a`

Artifact folder: `test-comms/artifacts/20260629-full-cleanwipe-longmont-5a24a5a/`

Preferred installer:

`test-comms/artifacts/20260629-full-cleanwipe-longmont-5a24a5a/The Civic Desk_0.2.8_x64-setup.exe`

Expected preferred NSIS SHA256:

`A19456F776E319E0850463A3494A47B2CBA5668C556724BB1A96C4963E412082`

Fallback MSI:

`test-comms/artifacts/20260629-full-cleanwipe-longmont-5a24a5a/The Civic Desk_0.2.8_x64_en-US.msi`

Expected fallback MSI SHA256:

`A519ADE9DD15EE20887BB189F6CECD78E6B7BE1CB584B54FB4ACD8159DABF61A`

Report path:

`test-comms/reports/20260629-full-cleanwipe-longmont-5a24a5a-report.md`

Artifact evidence path:

`test-comms/reports/20260629-full-cleanwipe-longmont-5a24a5a-evidence/`

## Machine Context

You are the tester on the separate cleanroom machine running as `msi\civic`.

Use this coordination checkout:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Do not use `C:\Users\instynct`; that is the coder machine path and is invalid on the tester machine except as a warning example.

All reports and JSON/text evidence must be written as UTF-8 without BOM. Do not write UTF-16 evidence files.

## Required Clean Wipe

Before installing this artifact, wipe only CivicNewspaper-related software/state and local AI runtime state. Do not reset Windows.

Remove, if present:

- The Civic Desk / CivicNewspaper app install.
- Running `civicnews.exe`, `ollama.exe`, or app-owned runtime processes.
- CivicNewspaper app data under Roaming.
- CivicNewspaper app data under Local.
- WebView/local storage state for The Civic Desk.
- Prior app-owned Ollama runtime and model cache.
- CivicNewspaper-related PATH changes made by prior tests.
- Prior local test files created only for CivicNewspaper cleanroom testing, except do not delete this coordination checkout or committed report/artifact history.

At minimum, inspect and remove matching product state under these tester-machine roots when present:

- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Local\The Civic Desk`
- `C:\Users\civic\AppData\Local\CivicNewspaper`
- `C:\Users\civic\AppData\Roaming\The Civic Desk`
- `C:\Users\civic\.ollama`

Also search `C:\Users\civic\AppData\Local` and `C:\Users\civic\AppData\Roaming` for product-state folders or files with names containing `civicdesk`, `civicnews`, `CivicNewspaper`, or `The Civic Desk`. Remove only product-owned state. Do not remove the coordination checkout at `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`.

If a wipe step requires admin and you cannot do it as the current test user, record the exact step that requires admin and whether a normal end user would see the same blocker.

## No Manual Dependency Installation

After the clean wipe, do not manually install Ollama, models, OCR tools, browser extensions, or other app prerequisites outside the app/installer flow.

The app must drive any required setup. If it cannot, the report must say exactly where the app breaks, what user-visible message appears, and what missing prerequisite was not handled by the product.

You may use normal user actions inside the app, including clicking setup buttons, approving app-driven downloads, waiting for progress, and using browser controls exactly as an end user would.

## Required Product Fix Verification

This artifact specifically needs proof of these fixes:

1. Publication identity gate:
   - The app must not publicly compile/publish with `My Local Publication`.
   - During setup or before publishing, enter and save a tester-chosen temporary publication name through the app UI, for example `Longmont Cleanroom Test`.
   - Report the exact name entered and prove it appears in the public output.
   - If the app allows starter identity to compile or publish publicly, fail this run.

2. Kill/cut persistence:
   - Kill at least one non-publish draft/story through the visible app UI.
   - Confirm the kill modal when shown.
   - Verify the final database and UI show at least one `killed` draft or an equivalent persisted killed/cut state.
   - Verify killed/cut items do not appear as published stories in the public site.
   - If Kill Story appears to work in the UI but the final database has no killed/cut item, fail this run.

3. Clean output path:
   - Before changing the output folder, record the default output path shown by the app.
   - The default output path must not point to any previous `test-comms/reports/...` evidence folder.
   - If it points to a superseded evidence folder, fail the run as stale state.
   - For artifact collection, you may then use the app UI to choose the current evidence output folder:
     `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-full-cleanwipe-longmont-5a24a5a-evidence\publication-output\site`

## Target Scenario

City: Longmont, Colorado.

The source set must include both official and public social/community sources when the app can discover or import them. Use only public/readable-without-login sources. Do not use private groups, credentials, or non-public data.

The product goal is a real local publication:

- 10 to 25 leads if the app can produce them.
- 5 to 10 reader-facing stories or briefs if the app can produce them.
- If fewer are produced, document whether the app tried to expand discovery/search and exactly why it stopped.
- Separate publishable stories/briefs from dark-signal leads or watchlist items that need verification.
- Do not manually write the publication yourself. Use the app and its local AI/workflows.

## Required End-to-End User Flow

1. Fetch and read `test-comms/ACTIVE_DIRECTIVE.md`. Confirm it points to this directive.
2. Verify installer hashes before installation.
3. Perform the clean wipe described above.
4. Install with the NSIS installer. Use MSI only if NSIS fails, and document why.
5. Launch the app as a normal user.
6. Complete first-run/setup for Longmont, Colorado.
7. Let the app inspect hardware and select/download/setup the model or local AI runtime it chooses. Do not manually install runtime/model pieces outside the app.
8. If the app cannot set up local AI from a clean machine, stop only that path, capture screenshots/logs, and report the exact product failure. Continue any non-AI deterministic paths the app allows.
9. Run source discovery for Longmont.
10. Ensure official and public social/community sources are included when the product supports them.
11. Run a daily scan or the closest app-supported full scan workflow.
12. Use the app workflow to generate leads.
13. Use the app workflow to draft stories/briefs with local AI if available.
14. Exercise writer workflow: open a draft, edit text, save, send for review or equivalent.
15. Exercise editor workflow: review, approve, send back for more work if available, put on hold if available, kill/cut at least one non-publish item, then approve publishable stories.
16. Invoke the press-freedom/legal-risk advisor on at least one story if local AI is available. It must advise only; it must not block publication.
17. Compile/preview/export the publication.
18. Export the ZIP/package from the app.
19. Publish anonymously to here.now. This publish is authorized for this test.
20. Verify the here.now URL returns HTTP 200 and visibly contains the generated publication.
21. Download/copy the published here.now HTML pages when practical.
22. Run the exact output checks below.
23. Write a human-readable report to the report path above and commit it with `[skip ci]`.

## Required Output Checks

Run these checks against:

- The local exported output folder.
- The exported ZIP after extraction, if different.
- Downloaded here.now HTML pages, if practical.

The report must include:

- here.now URL.
- local output folder path.
- default output folder path observed before changing it.
- ZIP/package path.
- ZIP SHA256.
- number of sources, evidence items, leads, drafts, approved/published stories, killed/held items.
- model/runtime selected by the app.
- whether the app installed/configured AI from clean state without tester help.
- exact screenshots for first-run setup, AI setup/progress, source discovery, leads, draft/editor, advisor, publish/export, kill/cut proof, and here.now verification.

### Mojibake Check

Use the exact scanner from `test-comms/directives/20260629-rerun-mojibake-systemic-59eb271.md`.

Do not fail on legitimate Unicode characters such as a curly apostrophe, copyright sign, or right arrow. Fail only on known bad decoded sequences.

### Draft Prefix Check

Public output must not contain public-facing article titles beginning with `Draft:`.

If the word `Draft:` appears only in internal app UI, logs, or a test note, document it and do not fail for that alone.

### Publication Identity Check

Public output must not use `My Local Publication` as the site title.

Public output must use the tester-entered publication name from this run.

### Publication Quality Check

Open the generated publication and verify:

- Homepage lists real stories/briefs.
- Article pages load.
- RSS exists.
- About/ethics/how-we-report/corrections pages load.
- Evidence/source links are present where the app has sources.
- Share package/newsletter/Substack/community post files exist.
- Mobile/narrow layout is usable.
- No obvious empty-state text appears in the final published issue unless the app genuinely produced no content and explains why.

## Pass / Fail Bar

PASS only if a clean machine with no preinstalled CivicNewspaper/Ollama/model state can install the app, have the product set up what it needs or clearly guide the user through app-owned setup, ingest/discover Longmont sources including official and public social/community paths when supported, generate a substantial lead/story set, exercise writer/editor/advisor/kill workflows, export a ZIP/package, publish to here.now, and provide a human-readable report plus artifacts.

FAIL if:

- The tester has to manually install a dependency outside the app/installer flow.
- The app cannot proceed from clean state and does not clearly explain what to do.
- The app cannot produce any real Longmont publication output.
- here.now publish fails without actionable recovery.
- Generated public output contains known mojibake sequences.
- Generated public output exposes old working titles beginning with `Draft:`.
- Generated public output uses `My Local Publication` as the publication title.
- Kill/cut does not persist in the UI and final database.
- The default output path after clean wipe points to a superseded report/evidence folder.

If it fails, do not paper over it. Say exactly where it broke and attach evidence. The coder will fix and issue the next artifact.
