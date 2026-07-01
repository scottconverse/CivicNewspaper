# 20260701 Evidence Linkage Rerun 33bc936

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester:

C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main

Product commit represented by installer:

33bc93645ed3a726d7292bd5aad394a677add4e8

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260701-evidence-linkage-rerun-33bc936/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

4968F81CF21CBAD5DD634375DBF00F67595CE0A023DF0654358F9FBD3092E8E4

Expected installer size:

5638753

Required visibility report:

test-comms/reports/20260701-evidence-linkage-rerun-33bc936-visibility.md

Required final report:

test-comms/reports/20260701-evidence-linkage-rerun-33bc936-report.md

Evidence folder:

test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/

## Why This Rerun Exists

The previous cleanroom run passed clean install, visible startup, product-owned runtime setup, and automatic phi4-mini model pull. It then blocked because Daily Scan exposed a lead as ready to draft even though the promoted Story Queue lead had no linked source evidence. Draft generation correctly produced only a verification placeholder, and Workbench blocked publication.

This build changes the scan-to-queue contract:

- A Daily Scan story or brief cannot stay in ready-to-draft or reader-facing review status unless linked evidence rows can be found.
- Unsupported model-suggested leads should remain visible as verification work.
- Evidence-backed rescue leads should carry the draftable queue slots instead.

## Required Cleanroom Procedure

1. Fetch and update this coordination branch.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Verify this directive path matches the active directive.
4. Verify the installer hash and size exactly.
5. Clean wipe the product state as in prior runs:
   - Stop stale `civicnews` and `ollama` processes.
   - Remove prior The Civic Desk app data and product-owned Ollama/model data.
   - Leave Windows, browser, user account, and unrelated software intact.
6. Install only from the installer artifact in this directive.
7. Launch normally as an end user. Do not manually install Ollama or models.
8. Confirm the app remains visible through startup and model setup.
9. Confirm product-owned local AI setup completes and reaches the main dashboard with a ready model.

## Required Product Flow

Run the full Longmont end-to-end flow:

1. Use Longmont, CO.
2. Use official, media, and public community/social sources that are readable without login.
3. Run source discovery / source intake as an end user would.
4. Run Daily Scan.
5. Inspect Story Queue.
6. Confirm no `ready_to_draft` Story Queue lead lacks linked source evidence.
7. If unsupported model-suggested leads appear, confirm they are labeled as verification work rather than normal draft-ready stories.
8. Select an evidence-backed draftable lead.
9. Generate a draft with the local model.
10. Open it in Workbench.
11. Confirm the Workbench story-quality preflight does not report missing linked source documents for that evidence-backed draft.
12. Exercise editor workflow enough to prove:
    - draft,
    - send back / needs work,
    - hold,
    - restore or resume,
    - ready for review,
    - approve for publishing.
13. Export the static-site ZIP/package.
14. Publish anonymously to here.now. This here.now anonymous publish is authorized for this test.
15. Save the here.now URL in the final report.
16. Save the local output path and ZIP/package path in the final report.

## Required Output Audits

Audit generated local output, ZIP extract if applicable, RSS/share artifacts, and here.now pages for:

- No duplicate story topics in the final issue.
- No public leakage of `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.
- No mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD in public output.
- No unsupported lead published as a reader-facing story.
- At least 5 reader-facing items if the source material supports it; if fewer, document exactly why and what source/search expansion failed.
- Headlines should read like publication headlines, not reporter-note summaries.
- Stories should be source-grounded and useful to a Longmont reader.

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
- Workbench/editor workflow result.
- ZIP/package path.
- here.now URL if publish succeeds.
- Output quality audit findings.
- Any product bug that requires coder action.
