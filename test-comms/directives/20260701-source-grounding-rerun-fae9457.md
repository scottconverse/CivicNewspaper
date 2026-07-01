# 20260701 Source Grounding Rerun fae9457

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main

Product commit represented by installer:

fae94570ab75cd3548cf5b8d254aa668ca96fce9

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260701-source-grounding-rerun-fae9457/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

ABA11CFFA0A52E130C2B77C2E20F139C22039DE305CC5C32C62F2C245C83AC45

Expected installer size:

5650162

Coder-side installer smoke receipt:

test-comms/artifacts/20260701-source-grounding-rerun-fae9457/windows-installer-smoke-receipt.json

Required visibility report:

test-comms/reports/20260701-source-grounding-rerun-fae9457-visibility.md

Required final report:

test-comms/reports/20260701-source-grounding-rerun-fae9457-report.md

Evidence folder:

test-comms/evidence/20260701-source-grounding-rerun-fae9457/

## Why This Rerun Exists

The prior cleanroom run on eb0a4ac was blocked because a Summer Reading Challenge lead was draftable with broad Longmont events/departments evidence. Workbench let the draft reach ready_to_publish after only advisory warnings.

This build tightens source grounding again:

- Daily Scan source linking now matches only lead title/summary, not editorial workflow context.
- Source-topic matching uses the specific topic text and ignores generic source labels such as city/news/events/departments.
- Rescue leads created directly from source evidence still link back to their own evidence row.
- Workbench topic matching uses the story title/topic, not copied draft body text.
- Static compile checks use the public title/topic, not the body, when deciding whether linked evidence matches.
- Regression tests cover Summer Reading Challenge versus broad calendar/department evidence.

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
7. Confirm no ready_to_draft Story Queue lead lacks linked source evidence.
8. Confirm no draftable Story Queue lead is linked only to unrelated evidence.
9. Specifically try to reproduce the prior failure:
   - Look for `Summer Reading Challenge Starts at Longmont Public Library`.
   - If it appears as ready_to_draft, inspect its linked evidence.
   - It must not be draftable unless linked evidence specifically mentions the Summer Reading Challenge or its exact dates/details.
   - Broad events/departments/calendar snippets alone must leave it as verification work or block approval.
10. If unsupported model-suggested leads appear, confirm they are labeled as verification work rather than normal draft-ready stories.
11. Select an evidence-backed draftable lead.
12. Generate a draft with the local model.
13. Open it in Workbench.
14. Confirm Workbench story-quality preflight does not report unrelated linked source documents for a correctly grounded draft.
15. If Workbench does report unrelated linked source documents, confirm the draft cannot be approved for static publish and report the exact story/evidence mismatch.
16. Exercise editor workflow enough to prove:
    - draft,
    - send back / needs work,
    - hold,
    - restore or resume,
    - ready for review,
    - approve for publishing.
17. Export the static-site ZIP/package.
18. Publish anonymously to here.now through the app UI connector button labeled `Publish to here.now`. This here.now anonymous publish is authorized for this test.
19. Do not use a separate direct API script unless the product UI path fails. If direct API fallback is used, mark the product UI path blocked.
20. Save the here.now URL in the final report.
21. Save the local output path and ZIP/package path in the final report.

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

Visibility report must be written to:

test-comms/reports/20260701-source-grounding-rerun-fae9457-visibility.md

Final report must be written to:

test-comms/reports/20260701-source-grounding-rerun-fae9457-report.md

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
- Semantic source-grounding audit result for the Summer Reading prior-failure case if present.
- Semantic source-grounding audit result for at least the selected draftable lead.
- Workbench/editor workflow result.
- ZIP/package path.
- here.now URL if publish succeeds.
- Output quality audit findings.
- Any product bug that requires coder action.
