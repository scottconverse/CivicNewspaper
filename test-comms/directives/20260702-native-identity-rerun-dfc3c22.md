# 20260702 Native Identity Rerun dfc3c22

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch: main

Product commit represented by installer:

dfc3c22789a388dbede422f5c3ac1750efa707d9

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260702-native-identity-rerun-dfc3c22/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

2F2B89F973630BDF8AA5310726E30F45D7228C286BD151B97B4BEC63F5BCC9B3

Expected installer size:

5659065

Coder-side installer smoke receipt:

test-comms/artifacts/20260702-native-identity-rerun-dfc3c22/windows-installer-smoke-receipt.json

Coder-side release smoke receipt:

test-comms/artifacts/20260702-native-identity-rerun-dfc3c22/release-smoke-receipt.json

Required visibility report:

test-comms/reports/20260702-native-identity-rerun-dfc3c22-visibility.md

Required final report:

test-comms/reports/20260702-native-identity-rerun-dfc3c22-report.md

Evidence folder:

test-comms/evidence/20260702-native-identity-rerun-dfc3c22/

## Why This Rerun Exists

The prior fbfb904 cleanroom run still failed on first-run setup. The screenshots showed text fields visually accepting typed values while the app displayed the no-input warning and no `identity.*` settings were written. That means the packaged WebView accepted native field edits but React did not reliably receive the identity-step input/click events.

Build dfc3c22 makes the identity handoff native-first:

- The identity step reads actual DOM input/select values, not only React state.
- The identity Next button has direct DOM `pointerdown`, `mousedown`, and `click` listeners for the first setup handoff.
- Pressing Enter during identity setup also uses the native identity advance path.
- Later setup steps keep their normal React handlers so Finish is not blocked by stale native listeners.
- This is a generic first-run setup fix, not a Longmont-specific workaround.

## Required Cleanroom Procedure

1. Fetch and update this coordination branch.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`.
3. Verify this directive path matches the active directive.
4. Verify the installer hash and size exactly.
5. Clean wipe the product state:
   - Stop stale `civicnews` and product-owned `ollama` processes.
   - Remove prior The Civic Desk app data and product-owned Ollama/model data.
   - Leave Windows, browser, user account, and unrelated software intact.
6. Install only from the installer artifact in this directive.
7. Launch normally as an end user. Do not manually install Ollama or models.
8. Enter a real Longmont cleanroom publication identity:
   - Publication name must include Longmont.
   - Editor name must be non-empty.
   - City must be Longmont.
   - State must be CO.
9. Click Next from AI Setup identity.
10. Verify the app remains visible and advances to AI Service Setup.
11. After clicking Next, inspect the app database and verify `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, and `onboarding.step` are saved.
12. If the app disappears or blanks, relaunch once and verify identity and setup step recovery. If recovery fails, write BLOCKED with the database snapshot.
13. Continue the full setup only through product UI actions. The tester must not manually install Ollama or models.
14. Confirm product-owned local AI setup completes and reaches the main dashboard with a ready model.

## Required Product Flow After Setup

After setup reaches the dashboard, continue the interrupted Longmont end-to-end flow:

1. Use Longmont, CO.
2. Use official, media, and public community/social sources that are readable without login.
3. Run source discovery / source intake as an end user would.
4. Run Daily Scan.
5. Inspect Story Queue.
6. Confirm no `ready_to_draft` Story Queue lead lacks linked source evidence.
7. Confirm no draftable Story Queue lead is linked only to unrelated evidence.
8. Specifically verify the city news chrome blocker:
   - Look for the `Longmont city news` lead or any similar lead sourced from `https://www.longmontcolorado.gov/news`.
   - Broad city news/category/index evidence must not be `ready_to_draft` when it contains only category navigation, page chrome, news sorting controls, newsletter/signup language, result counts, department/service labels, or generic link lists.
   - If such a lead appears as `ready_to_draft`, write BLOCKED and include the exact lead title, linked evidence IDs, and evidence excerpts.
9. Recheck the Visit Longmont tourism/calendar blocker:
   - Look for `Longmont Arts Week Festival` or similar Visit Longmont event-calendar leads.
   - Broad Visit Longmont event-calendar evidence must not be `ready_to_draft` when it contains only page chrome, category navigation, visitor-guide/media/partner/travel links, or broad event-list navigation.
10. Recheck the earlier city-site navigation blocker:
   - Generic Longmont city events, city departments, event index, services index, newsletter, or broad navigation leads must not be `ready_to_draft` when their evidence is only broad page chrome, departments/services lists, or old undifferentiated event index text.
11. Recheck the Summer Reading failure:
   - `Summer Reading Challenge Starts at Longmont Public Library` or similar Summer Reading leads must not be draftable unless linked evidence specifically mentions the Summer Reading Challenge or its exact dates/details.
12. If unsupported model-suggested leads appear, confirm they are labeled as verification work rather than normal draft-ready stories.
13. If at least one genuinely grounded `ready_to_draft` lead exists:
   - Select an evidence-backed draftable lead.
   - Generate a draft with the local model.
   - Open it in Workbench.
   - Confirm Workbench story-quality preflight does not report unrelated linked source documents for a correctly grounded draft.
   - Exercise editor workflow enough to prove draft, send back / needs work, hold, restore or resume, ready for review, and approve for publishing.
   - Export the static-site ZIP/package.
   - Publish anonymously to here.now through the app UI connector button labeled `Publish to here.now`. This here.now anonymous publish is authorized for this test.
   - Save the here.now URL in the final report.
   - Save the local output path and ZIP/package path in the final report.
14. If no genuinely grounded `ready_to_draft` lead exists:
   - Do not force a draft from a weak or broad-index lead.
   - Report PASS for the chrome/source-grounding rescue fix only if all weak generic leads are no longer `ready_to_draft`.
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

If publish does not succeed because there are no grounded `ready_to_draft` leads, report database counts and queue dispositions instead.

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

test-comms/reports/20260702-native-identity-rerun-dfc3c22-visibility.md

Final report must be written to:

test-comms/reports/20260702-native-identity-rerun-dfc3c22-report.md

Visibility report must include:

- Machine/user identity.
- Coordination path.
- Installer hash and size.
- Whether AI Setup identity Next remained visible and advanced.
- Whether `identity.*` and `onboarding.step` were saved after Next.
- If relaunch recovery was needed, whether identity and setup step restored correctly.
- Product-owned runtime/model setup result.
- Whether the dashboard reached local AI ready.

Final report must include:

- Overall PASS/BLOCKED.
- Exact blocker if blocked.
- Screenshot list.
- Counts for sources, evidence items, Daily Scan leads, Story Queue leads, drafts, publish runs, and published posts.
- Evidence-linkage audit result for ready-to-draft leads.
- City news/category/index chrome rescue audit result for `https://www.longmontcolorado.gov/news` and similar broad news/category pages.
- Tourism/calendar navigation rescue audit result for Visit Longmont event-calendar leads.
- City-site navigation rescue audit result for broad city events/departments/services/calendar leads.
- Semantic source-grounding audit result for the Summer Reading prior-failure case if present.
- Semantic source-grounding audit result for at least the selected draftable lead if one exists.
- Workbench/editor workflow result if a draftable lead exists.
- ZIP/package path if export succeeds.
- here.now URL if publish succeeds.
- Output quality audit findings if output is produced.
- Any product bug that requires coder action.
