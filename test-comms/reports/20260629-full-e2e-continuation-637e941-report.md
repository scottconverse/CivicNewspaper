# Full E2E Continuation Report - 637e941

Status: FAIL - release gate still open

Directive: `test-comms/directives/20260629-continue-full-e2e-after-637e941-partial.md`

Product branch: `stable-readiness-local-gates`

Required product commit: `637e941ac77361033fc22b48fac33ae1aa50a6b3`

Tester checkout: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`

Artifact folder: `test-comms/artifacts/20260629-full-e2e-continuation-637e941/`

## Summary

The run resumed from the existing cleanroom install; no wipe or reinstall was performed. The app was installed and running from the previously verified 637e941 artifact state, with Longmont active and bundled Local AI ready on `qwen2.5:7b`.

The product successfully generated enough Longmont drafts, exercised editor controls, compiled static output, created `site-package.zip`, and anonymously published to here.now:

`https://silent-signal-6cm6.here.now`

HTTP verification returned 200 and the page contained Longmont/Civic publication content.

The run is still marked failed because generated/published output contained mojibake-style encoding markers (`â€™`, `â€`) in multiple story HTML files. This violates the directive requirement to confirm advisory/warning/publication text has no garbled encoding markers and is not acceptable for a real Longmont publication next week without cleanup. The run also showed confusing publish-state behavior around kill/cut and duplicate/extra draft generation that should be reviewed.

## Resume / Installation

- Resume used: yes.
- Reinstall performed: no.
- Current installed executable: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Bundled Ollama executable: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Local AI status: `Local AI ready`
- Selected model: `qwen2.5:7b`
- App handled Local AI without tester-installed dependencies.

## Initial State Verified

Database resume snapshot:

- Sources: 6
- Evidence items: 27
- Leads: 18
- Daily scan leads: 10
- Drafts at resume: 2
- Publish runs at resume: 0

Already-drafted lead behavior:

- Opened an existing draft from the workbench/queue state.
- Draft count remained 2 after the open action.
- Result: no duplicate draft created for that already-drafted lead.

Direct Back to Queue:

- The Workbench exposed a direct `Back to Queue` control at 1280x720.
- Returning to queue was verified without relying on Alt+1.

## Drafting / Editor Controls

Drafting continued until enough stories existed for publication:

- Started with 2 persisted drafts.
- Generated additional drafts via visible queue Draft controls.
- Final database state after all interactions: 7 drafts total.
- Final status split: 6 `ready_to_publish`, 1 `draft_generated`.

Editor controls exercised:

- Edited story title.
- Edited article body.
- Saved draft.
- Ran press-freedom/legal-risk advisor.
- Observed non-blocking advisory warnings.
- Held a story.
- Approved stories for static publish.
- Opened and confirmed Kill Story flow.

Cut/kill behavior:

- `Kill Story` opened a confirmation dialog and confirmation reported `Story status updated to 'killed'.`
- Later final DB state showed the previously killed story as `ready_to_publish` and included in publish output. This is unexpected and should be treated as a product finding.

## Publishing

Compile/export output folder:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/publication-output/site/`

ZIP package:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/publication-output/site/site-package.zip`

ZIP SHA256:

`708A19547B345C3CA7AF989A20F91492A0FC1BD129A8E76987148BE17B7877A3`

Publish result:

- Provider: `here_now`
- URL: `https://silent-signal-6cm6.here.now`
- Deployment ID: `slug=silent-signal-6cm6;version=01KW91XWH68N46G5PW9Z44DP1V;created_slug=silent-signal-6cm6`
- Article count reported by product: 6
- Files written: 23
- HTTP verification: 200
- Visibility check: response contained Longmont and Civic/publication content.

Note: first anonymous here.now attempt failed with `Display name must not be empty after normalization.` Saving a here.now config with display name `Longmont Civic Desk Test` through the product command fixed the publish request, and anonymous temporary preview publishing then succeeded.

## Quality Findings

### Major - Published output contains mojibake markers

The generated publication output contains garbled encoding markers:

- `watch/1.html`: `â€™`, `â€`
- `watch/3.html`: `â€™`, `â€`
- `watch/5.html`: `â€™`, `â€`
- `watch/6.html`: `â€™`, `â€`

Evidence: `test-comms/artifacts/20260629-full-e2e-continuation-637e941/mojibake-scan.json`

Impact: not ready for Scott to use as a real Longmont publication next week without editorial cleanup or an encoding fix.

### Major - Kill/cut state was confusing and did not clearly exclude the story from output

The UI reported a story was killed, but final DB/output evidence showed six publishable/published stories and included `watch/2.html` for the story that had been selected for kill. This needs product review because a killed story must not silently remain in the publishing pipeline.

Evidence:

- `kill-story-confirmed.json`
- `final-db-state.json`
- `publication-output/site/publish-manifest.json`

### Minor - here.now anonymous publish needed manual config save

The first product connector publish attempt reached here.now but sent an empty display name. Saving a here.now publisher config with a display name resolved it. The app should probably default this correctly when using anonymous preview publishing.

Evidence:

- `here-now-publish-result.json`
- `here-now-publish-result-2.json`
- `here-now-publish-result-3.json`

## Artifacts

Key artifacts saved under `test-comms/artifacts/20260629-full-e2e-continuation-637e941/`:

- `01-resume-window.png`
- `02-story-queue-resume.png`
- `03-existing-draft-opened.png`
- `04-existing-draft-top-back-to-queue.png`
- `10-story-queue-cdp-active.png`
- `11-final-app-state.png`
- `workbench-dom.json`
- `editor-control-result-2.json`
- `kill-story-confirmed.json`
- `compile-result.json`
- `here-now-publish-result-3.json`
- `here-now-http-verification.json`
- `final-db-state.json`
- `publication-output-files.json`
- `mojibake-scan.json`
- `site-package-zip-sha256.txt`
- `publication-output/site/`
- `publication-output/site/site-package.zip`

Full artifact inventory:

`test-comms/artifacts/20260629-full-e2e-continuation-637e941/artifact-file-list.txt`

## Final Readiness Assessment

Not ready for Scott to use for a real Longmont publication next week.

The core local workflow did run end to end: resume, Local AI draft generation, editor review controls, static compile, ZIP export, and anonymous here.now publishing all completed. However, the published text contains garbled encoding markers, and the kill/cut publishing state is not trustworthy enough for release readiness.
