# 20260701 Navigation Rescue Rerun 4ecaf22

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main

Product commit represented by installer:

4ecaf22b8c52273ae6ec8bfc143fb7acb32645d1

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260701-navigation-rescue-rerun-4ecaf22/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

41FE39E228BD943650B94FD8BC056FE4EC84637BA498FC0D8B9F52F30804D8F5

Expected installer size:

5640839

Coder-side installer smoke receipt:

test-comms/artifacts/20260701-navigation-rescue-rerun-4ecaf22/windows-installer-smoke-receipt.json

Required visibility report:

test-comms/reports/20260701-navigation-rescue-rerun-4ecaf22-visibility.md

Required final report:

test-comms/reports/20260701-navigation-rescue-rerun-4ecaf22-report.md

Evidence folder:

test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/

## Why This Rerun Exists

The prior cleanroom run on fae9457 fixed the Summer Reading grounding failure and Workbench correctly blocked unrelated linked sources. It still produced one bad ready_to_draft rescue lead: a generic Longmont city events/departments/navigation item backed by broad page chrome and index text.

This build adds a queue-side filter so broad navigation, events index, departments index, and page chrome excerpts do not become actionable rescue evidence. The goal is to stop wasting editor/model time before draft generation.

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
9. Specifically verify the fae9457 blocker:
   - Look for generic Longmont city events, city departments, event index, services index, newsletter, or broad navigation leads.
   - These must not be ready_to_draft when their evidence is only broad page chrome, event-list navigation, departments/services lists, or old undifferentiated event index text.
   - If such a lead appears as ready_to_draft, write BLOCKED and include the exact lead title, linked evidence IDs, and evidence excerpts.
10. Specifically verify the prior Summer Reading failure:
   - Look for `Summer Reading Challenge Starts at Longmont Public Library` or similar Summer Reading leads.
   - It must not be draftable unless linked evidence specifically mentions the Summer Reading Challenge or its exact dates/details.
   - Broad events/departments/calendar snippets alone must leave it as verification work or block approval.
11. If unsupported model-suggested leads appear, confirm they are labeled as verification work rather than normal draft-ready stories.
12. If at least one genuinely grounded ready_to_draft lead exists:
   - Select an evidence-backed draftable lead.
   - Generate a draft with the local model.
   - Open it in Workbench.
   - Confirm Workbench story-quality preflight does not report unrelated linked source documents for a correctly grounded draft.
   - Exercise editor workflow enough to prove draft, send back / needs work, hold, restore or resume, ready for review, and approve for publishing.
   - Export the static-site ZIP/package.
   - Publish anonymously to here.now through the app UI connector button labeled `Publish to here.now`. This here.now anonymous publish is authorized for this test.
   - Save the here.now URL in the final report.
   - Save the local output path and ZIP/package path in the final report.
13. If no genuinely grounded ready_to_draft lead exists:
   - Do not force a draft from a weak or broad-index lead.
   - Report PASS for the navigation-rescue fix only if all weak generic leads are no longer ready_to_draft.
   - Report BLOCKED for the full publication E2E and explain that the source set did not produce a publishable lead.
   - Include counts and examples of the highest-ranked non-draftable leads, with why each stayed verification/watch/background.

Do not use a separate direct API script for here.now unless the product UI path fails. If direct API fallback is used, mark the product UI path blocked.

## Required Database / Artifact Audits

If publish succeeds, inspect the app database and generated output. Report:

- `settings.identity.newsroom_name` equals the publication name used in setup/settings.
- Latest `publish_runs.provider` is `here_now` after app UI publish.
- Latest `publish_runs.published_url` is the here.now URL.
- Latest `publish_runs.deployment_id` is present if here.now returned one.
- Generated `publish-manifest.json` contains the live URL if the app updated the package after publish.
- ZIP/package exists and contains the same public issue output that was published.

If publish does not succeed because there are no grounded ready_to_draft leads, report database counts and queue dispositions instead.

## Required Output Audits

If output is generated, audit generated local output, ZIP extract if applicable, RSS/share artifacts, and here.now pages for:

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

test-comms/reports/20260701-navigation-rescue-rerun-4ecaf22-visibility.md

Final report must be written to:

test-comms/reports/20260701-navigation-rescue-rerun-4ecaf22-report.md

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
- Navigation/index rescue audit result for broad city events/departments/services/calendar leads.
- Semantic source-grounding audit result for the Summer Reading prior-failure case if present.
- Semantic source-grounding audit result for at least the selected draftable lead if one exists.
- Workbench/editor workflow result if a draftable lead exists.
- ZIP/package path if export succeeds.
- here.now URL if publish succeeds.
- Output quality audit findings if output is produced.
- Any product bug that requires coder action.
