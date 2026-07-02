# 20260702 WebView Identity Rerun 4bede5c

Tester role: cleanroom end-user tester for CivicNewspaper / The Civic Desk.

Coordination repo: https://github.com/scottconverse/CivicNewspaper

Coordination branch: test-comms/cleanroom-coder-tester

Tester local coordination path:

C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms

Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product branch:

main

Product commit represented by installer:

4bede5c6773189e24c8aa05a105e503b93111fca

Important: the product commit is represented by the installer artifact below. Do not assume the product branch has been pushed.

Installer:

test-comms/artifacts/20260702-webview-identity-rerun-4bede5c/The Civic Desk_0.3.1_x64-setup.exe

Expected installer SHA256:

4A40482D29B2C601CF28A9CAB7E1904A15BDD0653F99E26D250F037BF98662AD

Expected installer size:

5653840

Coder-side installer smoke receipt:

test-comms/artifacts/20260702-webview-identity-rerun-4bede5c/windows-installer-smoke-receipt.json

Coder-side release smoke receipt:

test-comms/artifacts/20260702-webview-identity-rerun-4bede5c/release-smoke-receipt.json

Required visibility report:

test-comms/reports/20260702-webview-identity-rerun-4bede5c-visibility.md

Required final report:

test-comms/reports/20260702-webview-identity-rerun-4bede5c-report.md

Evidence folder:

test-comms/evidence/20260702-webview-identity-rerun-4bede5c/

## Why This Rerun Exists

The dfc3c22 cleanroom run still failed at first-run Identity. The real installed WebView focused the fields, but typed, pasted, and native-keyed values did not remain visible and no `identity.*` or `onboarding.step` settings were saved.

Build 4bede5c fixes the generic packaged WebView failure class:

- Identity fields are DOM-backed/uncontrolled so native text entry is not erased if React input events are dropped.
- The identity handoff reads actual DOM field/select values before saving.
- Starter profile buttons also write directly to DOM fields through native capture listeners.
- The state field still visibly normalizes to a two-letter state code.
- here.now anonymous publish now retries transient create/finalize/provider failures seen during release smoke.

This is a generic first-run setup and publishing reliability fix, not a Longmont-specific workaround.

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
7. Launch `The Civic Desk` normally as an end user. Do not manually install Ollama or models.
8. On Identity, verify ordinary native field entry works:
   - Type or paste a publication name that includes Longmont.
   - Type or paste a non-empty editor name.
   - Type or paste city `Longmont`.
   - Type or paste state `CO`.
   - Confirm the visible values remain in the fields before clicking Next.
9. Also verify starter profiles are not broken:
   - If ordinary entry works, you may continue with the typed identity.
   - If ordinary entry fails, try the Longmont starter button and report both paths.
10. Click Next from Identity.
11. Verify the app remains visible and advances to AI Service Setup.
12. After clicking Next, inspect the app database and verify `identity.newsroom_name`, `identity.editor_name`, `identity.city`, `identity.state`, and `onboarding.step` are saved.
13. If the app disappears or blanks, relaunch once and verify identity and setup step recovery. If recovery fails, write BLOCKED with the database snapshot.
14. Continue setup only through product UI actions. The tester must not manually install Ollama or models.
15. Confirm product-owned local AI setup completes and reaches the main dashboard with a ready model.

## Required Product Flow After Setup

After setup reaches the dashboard, continue the interrupted Longmont end-to-end flow:

1. Use Longmont, CO.
2. Use official, media, and public community/social sources that are readable without login.
3. Run source discovery/source intake as an end user would.
4. Run Daily Scan.
5. Inspect Story Queue.
6. Confirm no `ready_to_draft` Story Queue lead lacks linked source evidence.
7. Confirm no draftable Story Queue lead is linked only to unrelated evidence.
8. Recheck broad chrome/navigation blockers:
   - Longmont city news/category/index pages must not become `ready_to_draft` when evidence is only page chrome, category navigation, result counts, department/service labels, newsletter/signup language, sorting controls, or generic link lists.
   - Visit Longmont tourism/calendar pages must not become `ready_to_draft` when evidence is only visitor-guide/media/partner/travel links, category navigation, or broad event-list chrome.
   - Generic Longmont city events, departments, services, calendar, newsletter, or index pages must not become `ready_to_draft` when evidence is only broad navigation or old undifferentiated index text.
9. Recheck semantic grounding:
   - `Summer Reading Challenge Starts at Longmont Public Library` or similar Summer Reading leads must not be draftable unless linked evidence specifically mentions the Summer Reading Challenge or its exact dates/details.
   - If unsupported model-suggested leads appear, confirm they are labeled as verification/watch/background work rather than normal draft-ready stories.
10. If at least one genuinely grounded `ready_to_draft` lead exists:
   - Generate a draft with the local model.
   - Open it in Workbench.
   - Exercise editor workflow enough to prove draft, send back/needs work, hold, restore or resume, ready for review, and approve for publishing.
   - Export the static-site ZIP/package.
   - Publish anonymously to here.now through the app UI button labeled `Publish to here.now`. This anonymous publish is authorized for this test.
   - Save the here.now URL, local output path, and ZIP/package path in the final report.
11. If no genuinely grounded `ready_to_draft` lead exists:
   - Do not force a draft from a weak or broad-index lead.
   - Report PASS for the source-grounding/chrome rescue checks only if weak generic leads are no longer `ready_to_draft`.
   - Report BLOCKED for full publication E2E and include counts/examples of the highest-ranked non-draftable leads with why each stayed verification/watch/background.

Do not use a separate direct API script for here.now unless the product UI path fails. If direct API fallback is used, mark the product UI path blocked.

## Required Database / Artifact Audits

If publish succeeds, inspect the app database and generated output. Report:

- `settings.identity.newsroom_name` equals the publication name used in setup/settings.
- Latest `publish_runs.provider` is `here_now`.
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

test-comms/reports/20260702-webview-identity-rerun-4bede5c-visibility.md

Final report must be written to:

test-comms/reports/20260702-webview-identity-rerun-4bede5c-report.md

Visibility report must include:

- Machine/user identity.
- Coordination path.
- Installer hash and size.
- Whether ordinary identity typing/paste remained visible before Next.
- Whether starter profile selection remained visible if tested.
- Whether Identity Next remained visible and advanced.
- Whether `identity.*` and `onboarding.step` were saved after Next.
- Product-owned runtime/model setup result.
- Whether the dashboard reached local AI ready.

Final report must include:

- Overall PASS/BLOCKED.
- Exact blocker if blocked.
- Screenshot list.
- Counts for sources, evidence items, Daily Scan leads, Story Queue leads, drafts, publish runs, and published posts.
- Evidence-linkage audit result for ready-to-draft leads.
- Chrome/navigation/source-grounding audit results for the prior blocker classes.
- Workbench/editor workflow result if a draftable lead exists.
- ZIP/package path if export succeeds.
- here.now URL if publish succeeds.
- Output quality audit findings if output is produced.
- Any product bug that requires coder action.
