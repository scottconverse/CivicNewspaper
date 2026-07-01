# 20260701 Source Grounding Rerun eb0a4ac

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main

Product commit represented by installer:

eb0a4ac284eedeb281891bb468f06cf9d564b1fe

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260701-source-grounding-rerun-eb0a4ac/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

3105CAD4EB00D6DDE501679E9C0820721267AC9F106B660735B42C3616734295

Expected installer size:

5622050

Coder-side installer smoke receipt:

test-comms/artifacts/20260701-source-grounding-rerun-eb0a4ac/windows-installer-smoke-receipt.json

Required visibility report:

test-comms/reports/20260701-source-grounding-rerun-eb0a4ac-visibility.md

Required final report:

test-comms/reports/20260701-source-grounding-rerun-eb0a4ac-report.md

Evidence folder:

test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/

## Why This Rerun Exists

The previous cleanroom run proved clean install, visible startup, app-owned local AI setup, model pull, Workbench mechanics, ZIP export, and direct anonymous here.now API publishing. It blocked on semantic source grounding:

- A lead about a library roof contract carried evidence rows from Downtown Longmont event/calendar material.
- The generated/published story mixed unrelated claims including Road Pony, capacity growth, and multi-year spending.
- Workbench caught missing sources but not unrelated linked sources.
- The app UI did not clearly let the tester publish anonymously to here.now through the product connector path.
- The final database still had `identity.newsroom_name` as starter text even though export used the new publication name.

This build adds source-topic matching before scan leads become draftable, filters unrelated linked evidence before local-model drafting, blocks static publish approval when linked sources do not match the story topic, blocks compile when cited paragraphs do not align with linked evidence, allows anonymous here.now connector publishing from the product UI, and persists community profile identity back into settings.

## Required Cleanroom Procedure

1. Fetch and update this coordination branch.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Verify this directive path matches the active directive.
4. Verify the installer hash and size exactly.
5. Clean wipe the product state as in prior runs:
   - Stop stale `civicnews` and product-owned `ollama` processes.
   - Remove prior The Civic Desk app data and product-owned Ollama/model data.
   - Leave Windows, browser, user account, and unrelated software intact.
6. Install only from the installer artifact in this directive.
7. Launch normally as an end user. Do not manually install Ollama or models.
8. Confirm the app remains visible through startup and model setup.
9. Confirm product-owned local AI setup completes and reaches the main dashboard with a ready model.

## Required Product Flow

Run the full Longmont end-to-end flow:

1. Use Longmont, CO.
2. Save a real publication identity, not starter text. Use a temporary cleanroom name that includes Longmont.
3. Use official, media, and public community/social sources that are readable without login.
4. Run source discovery / source intake as an end user would.
5. Run Daily Scan.
6. Inspect Story Queue.
7. Confirm no `ready_to_draft` Story Queue lead lacks linked source evidence.
8. Confirm no draftable Story Queue lead is linked only to unrelated evidence.
9. If unsupported model-suggested leads appear, confirm they are labeled as verification work rather than normal draft-ready stories.
10. Select an evidence-backed draftable lead.
11. Generate a draft with the local model.
12. Open it in Workbench.
13. Confirm the Workbench story-quality preflight does not report unrelated linked source documents for a correctly grounded draft.
14. If Workbench does report unrelated linked source documents, confirm the draft cannot be approved for static publish and report the exact story/evidence mismatch.
15. Exercise editor workflow enough to prove:
    - draft,
    - send back / needs work,
    - hold,
    - restore or resume,
    - ready for review,
    - approve for publishing.
16. Export the static-site ZIP/package.
17. Publish anonymously to here.now through the app UI connector button labeled `Publish to here.now`. This here.now anonymous publish is authorized for this test.
18. Do not use a separate direct API script unless the product UI path fails. If direct API fallback is used, mark the product UI path blocked.
19. Save the here.now URL in the final report.
20. Save the local output path and ZIP/package path in the final report.

## Required Database / Artifact Audits

After publish, inspect the app database and generated output. Report:

- `settings.identity.newsroom_name` equals the publication name used in setup/settings.
- Latest `publish_runs.provider` is `here_now` after app UI publish.
- Latest `publish_runs.published_url` is the here.now URL.
- Latest `publish_runs.deployment_id` is present if here.now returned one.
- Generated `publish-manifest.json` contains the live URL if the app updated the package after publish.
- ZIP/package exists and contains the same public issue output that was published.

## Required Output Audits

Audit generated local output, ZIP extract if applicable, RSS/share artifacts, and here.now pages for:

- No duplicate story topics in the final issue.
- No public leakage of `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.
- No mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD in public output.
- No unsupported lead published as a reader-facing story.
- No story paragraph that cites one evidence row while making claims from an unrelated topic.
- Headlines should read like publication headlines, not reporter-note summaries.
- Stories should be source-grounded and useful to a Longmont reader.
- At least 5 reader-facing items if the source material supports it; if fewer, document exactly why and what source/search expansion failed.

## Reports

Visibility report must include:

- Machine/user identity.
- Coordination path.
- Installer hash and size.
- Product-owned runtime/model setup result.
- Whether the dashboard reached local AI ready.

Final report must include:

- Overall PASS/BLOCKED.
- Exact blocker if blocked.
- Screenshot list.
- Counts for sources, evidence items, Daily Scan leads, Story Queue leads, drafts, publish runs, and published posts.
- Evidence-linkage audit result for ready-to-draft leads.
- Semantic source-grounding audit result for at least the selected draftable lead.
- Workbench/editor workflow result.
- ZIP/package path.
- here.now URL if publish succeeds.
- Output quality audit findings.
- Any product bug that requires coder action.
