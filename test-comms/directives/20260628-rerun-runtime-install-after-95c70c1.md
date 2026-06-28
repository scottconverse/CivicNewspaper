# Cleanroom directive: rerun first-run runtime setup after 95c70c1

Role: tester  
Coder branch: `stable-readiness-local-gates`  
Coder commit: `95c70c1`  
Artifact folder: `test-comms/artifacts/95c70c1-runtime-install-rerun/`

## Why this rerun exists

The `d213329` cleanroom rerun was blocked before source intake because the first-run `Install local AI runtime` control did not visibly start runtime setup on a clean machine.

Commit `95c70c1` changes first-run onboarding so:

- all wizard buttons are explicit `type="button"`;
- runtime install shows immediate progress before any backend work;
- runtime install still runs if progress-listener setup fails;
- the footer `Next` button on Step 2 starts runtime setup when the AI service is offline instead of skipping into an impossible model-download step;
- failures surface in the visible initialization banner.

## Install and reset

1. Use the preferred installer:
   `test-comms/artifacts/95c70c1-runtime-install-rerun/The Civic Desk_0.2.8_x64-setup.exe`
2. Verify SHA256:
   `62D5E248265E6AE81A58D192D72A720163B855A42C67D72DA63AA18B0FCECE50`
3. MSI fallback:
   `test-comms/artifacts/95c70c1-runtime-install-rerun/The Civic Desk_0.2.8_x64_en-US.msi`
4. MSI SHA256:
   `939AE2CCFC21AC9A38A0BBC78BAD6C9A8F79936832FE97522CD541E6B54AB842`
5. Start from clean product/runtime/app-data state as before. Do not manually install Ollama, models, Node, Rust, npm, or other prerequisites.

## Required test path

Run the product as a real end user through the UI.

1. Install and launch Civic Desk.
2. Complete first-run identity for Longmont, Colorado.
3. On the AI service setup step, verify the inline `Install local AI runtime` button:
   - click it once;
   - confirm the page immediately changes to an installing/progress state;
   - wait for runtime install/start to complete or fail visibly.
4. If the inline button still fails to react, click the footer `Next` button on Step 2 and confirm it starts runtime install or surfaces a visible error.
5. If runtime setup succeeds, continue the previous d213329 gate without manual bypass:
   - download the recommended model through the app;
   - import/discover Longmont official, media, and community sources;
   - run scan/story queue;
   - generate at least five drafts from distinct leads through UI only;
   - verify draft wizard controls remain reachable;
   - verify no invalid linked-evidence citations reach publishable drafts;
   - edit/save at least one draft;
   - run the press-freedom/legal-risk advisor on at least one draft;
   - approve at least five items and hold/kill at least one;
   - export ZIP;
   - publish anonymously to here.now;
   - read the published site and story pages.

## Pass/fail bar

PASS only if:

- clean install and app-driven local AI setup succeed;
- no tester-installed prerequisites are used;
- no manual database insertion is used;
- at least five Longmont reader-facing items are exported and published;
- held/killed item is excluded;
- export ZIP exists;
- here.now URL works;
- report includes source counts, lead counts, draft counts, article count, held/killed count, here.now URL, ZIP path, and quality concerns.

If it fails, stop at the exact blocker and report:

- whether the failure is installer/setup, runtime install, model download, source intake, scan, draft UI, citation integrity, editor workflow, export, or publish;
- visible app message;
- screenshots;
- whether an `ollama` process started;
- any app diagnostics/export file if the UI can save one.

## Report path

Write report to:

`test-comms/reports/20260628-runtime-install-rerun-report-95c70c1.md`

Place screenshots/artifacts under:

`test-comms/artifacts/20260628-runtime-install-rerun-95c70c1/`

Push to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
