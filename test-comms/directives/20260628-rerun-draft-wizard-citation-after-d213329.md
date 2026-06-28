# Cleanroom directive: rerun draft wizard and citation integrity after d213329

Role: tester  
Coder branch: `stable-readiness-local-gates`  
Coder commit: `d213329`  
Artifact folder: `test-comms/artifacts/d213329-draft-wizard-citation-rerun/`

## Why this rerun exists

The previous `82e9400` rerun proved source classification, app-managed AI setup, export, and here.now publish, but it was still partial:

- the UI path became stuck/clipped during repeated draft generation, forcing manual DB insertion for later drafts;
- local model drafts still cited evidence IDs that were not linked to the selected lead;
- one-source leads were inflated into broader stories instead of being treated as briefs/watchlist items.

Commit `d213329` is intended to fix those failures.

## Install and reset

1. Use the preferred installer:
   `test-comms/artifacts/d213329-draft-wizard-citation-rerun/The Civic Desk_0.2.8_x64-setup.exe`
2. Verify SHA256 before installing:
   `C0CC01CC4B3676A97C8BCC221F088DCE6F1058466584FB514B152FC7E3DCB10F`
3. If you need MSI fallback:
   `test-comms/artifacts/d213329-draft-wizard-citation-rerun/The Civic Desk_0.2.8_x64_en-US.msi`
4. MSI SHA256:
   `2BE3AA89013CB0EB1985D0450A69CC40FA2901FB41DD23D8951581AF92025CE0`
5. Start from clean product/runtime/app-data state as in the prior cleanroom runs. Do not manually install prerequisites, Ollama, or models. The app must drive setup.

## Test requirements

Run the real end-user UI path only unless the app itself exposes a control. Do not insert drafts directly into the database for this rerun.

1. Complete first-run setup for Longmont, Colorado.
2. Let the app install/configure local AI runtime and the selected model.
3. Run source discovery/import with the Longmont official, media, and community/social source list.
4. Confirm source classification still holds:
   - official records are official/primary;
   - Longmont Leader and Times-Call are media/news;
   - Reddit/Facebook/community sources are community/dark-signal/watch sources.
5. Run the daily scan and Story Queue flow until there are enough leads to make a small issue.
6. Generate at least five drafts from different leads through the UI only.
7. For each draft setup screen, capture whether:
   - Linked Sources count is visible;
   - Cancel is visible and usable;
   - Generate Draft is visible and usable;
   - the UI remains reachable after returning from the generated draft.
8. For each generated draft, inspect the body:
   - no `evidence:ID` citation may refer to an evidence ID outside the selected lead's linked source list;
   - if an invalid citation is emitted by the model, the app should neutralize it as `unlinked-evidence-ID` and add an editor note;
   - one-source leads should read like a brief/watchlist item, not a broad finished feature.
9. Use the editor controls through the UI:
   - approve at least five items for publication if they are good enough;
   - hold or kill at least one item;
   - run the press-freedom/legal-risk advisor on at least one draft;
   - save/edit at least one title or body manually in the UI.
10. Export the issue ZIP from the app.
11. Publish anonymously to here.now from the app.
12. Open and read the published here.now site:
   - homepage;
   - each published story;
   - RSS/feed link;
   - about/ethics/how-we-report/corrections pages if present;
   - source/evidence sections.

## Pass/fail bar

PASS only if all of these are true:

- clean install and app-driven local AI setup succeed;
- no manual prerequisite/model installation by tester;
- no manual DB insertion or non-user path is needed;
- the draft wizard action controls stay visible/usable through repeated draft generation;
- at least five reader-facing Longmont items are published;
- held/killed item is excluded from publication;
- exported ZIP exists and matches the published issue contents;
- here.now URL works;
- report includes the here.now URL, exported ZIP path, article count, lead count, draft count, held/killed count, and any quality concerns.

If this fails, report the exact break point, screenshots, logs if available, and whether the failure is installer/setup, UI reachability, AI/runtime, source intake, draft quality, export, or publish.

## Report path

Write the report to:

`test-comms/reports/20260628-draft-wizard-citation-rerun-report-d213329.md`

Put copied app artifacts/screenshots under:

`test-comms/artifacts/20260628-draft-wizard-citation-rerun-d213329/`

Push the report and artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
