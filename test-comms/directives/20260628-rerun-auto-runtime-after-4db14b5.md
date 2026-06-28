# Cleanroom directive: rerun auto runtime setup after 4db14b5

Role: tester  
Coder branch: `stable-readiness-local-gates`  
Coder commit: `4db14b5`  
Artifact folder: `test-comms/artifacts/4db14b5-auto-runtime-rerun/`

## Why this rerun exists

The `95c70c1` rerun still blocked because first-run runtime install controls appeared inert on the clean tester machine.

Commit `4db14b5` changes first-run setup so the product no longer depends on a precise button click:

- when Step 2 confirms the local AI service is offline, Civic Desk auto-starts app-managed runtime setup;
- the page shows visible runtime progress while setup is running;
- the footer `Next` button is disabled during runtime install;
- onboarding is constrained to a scrollable 100vh shell with sticky actions for 1280x720 usability.

## Install and reset

1. Use preferred installer:
   `test-comms/artifacts/4db14b5-auto-runtime-rerun/The Civic Desk_0.2.8_x64-setup.exe`
2. Verify SHA256:
   `45FAF025082DD88FB636A874F3BB43F9E74904849780A5084E34F99CEB60724A`
3. MSI fallback:
   `test-comms/artifacts/4db14b5-auto-runtime-rerun/The Civic Desk_0.2.8_x64_en-US.msi`
4. MSI SHA256:
   `CF5C2461F5DD69E95712AEFAF2B36D77189D5F16B89C5383EBF62DFE77A2306C`
5. Start from clean product/runtime/app-data state as before. Do not manually install Ollama, models, Node, Rust, npm, or other prerequisites.

## Required test path

Run as a real end user through the UI.

1. Install and launch Civic Desk.
2. Complete first-run identity for Longmont, Colorado.
3. On Step 2, do not click `Install local AI runtime` at first. Wait after the service is detected offline.
4. Confirm the app itself auto-starts runtime setup:
   - visible text should change to `Preparing local AI runtime install...` or a later runtime-install progress message;
   - `Next` should be disabled while install is running;
   - an `ollama` process and/or port 11434 should appear if setup reaches start;
   - if setup fails, the visible error must be specific.
5. If runtime setup succeeds, continue:
   - download the recommended model through the app;
   - import/discover Longmont official, media, and community/social sources;
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

If blocked, stop at the exact blocker and report whether it is installer/setup, runtime install, model download, source intake, scan, draft UI, citation integrity, editor workflow, export, or publish.

## Report path

Write report to:

`test-comms/reports/20260628-auto-runtime-rerun-report-4db14b5.md`

Place screenshots/artifacts under:

`test-comms/artifacts/20260628-auto-runtime-rerun-4db14b5/`

Push to `test-comms/cleanroom-coder-tester` with `[skip ci]`.
