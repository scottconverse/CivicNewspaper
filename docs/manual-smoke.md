# CivicNewspaper v0.3.0 Manual Smoke Test

This smoke test verifies the release paths that unit tests and browser component tests cannot prove by themselves.

Save screenshots, logs, output ZIPs, here.now URLs, model notes, and failure notes beside the release receipt.

## Required Environment

- Clean Windows user profile, Windows Sandbox, VM, or second test machine.
- No existing The Civic Desk app data.
- No preinstalled Ollama, local models, or CivicNewspaper test files unless the specific test says otherwise.
- Network available for source discovery, source fetches, model download, and here.now publishing.
- Public web access for official and public social/community sources.

The tester must behave like a normal user. If the app needs a runtime, model, path, parser, extension, or setup step, the app or installer must guide it. The tester should not manually repair the product.

## 1. Install And First Launch

1. Install the release artifact.
2. Confirm the app launches from the normal desktop or Start menu path.
3. Confirm app data is created under the clean profile.
4. Complete first-run identity setup.
5. Confirm organization type is selectable and saved.
6. Confirm publisher identity, footer language, and editorial policy text can be configured.

## 2. Local AI Setup

1. Start with no Ollama or model installed.
2. Confirm the app detects machine hardware and recommends an appropriate model.
3. Confirm the app explains model size, expected time, and degraded-mode options.
4. Download or install the required AI runtime/model through the app flow.
5. Confirm progress is visible.
6. Run a real local-model draft or advisor request and save the output.
7. Disable or stop the model service and confirm the app reports the real problem without misleading "model not found" copy.

## 3. Source Intake

1. Add at least one official Longmont source manually.
2. Run source discovery for Longmont, Colorado.
3. Confirm official, local media, public social/community, and search-fallback candidates are labeled clearly.
4. Import realistic fixture files:
   - CSV
   - TXT
   - XLSX
   - DOCX
   - text-readable PDF
   - scanned-style PDF
5. Confirm URLs are extracted as separate candidates, not flattened into one long string.
6. Confirm duplicates are shown or handled clearly.
7. Confirm image-only scanned PDFs explain the OCR limitation if OCR is unavailable.

## 4. Daily Scan And Civic Intelligence

1. With zero sources, confirm the scan routes the user to Sources instead of running empty.
2. With real sources configured, run Daily Scan.
3. Confirm staged progress shows source fetch, deterministic checks, entity extraction, optional AI review, saving, and completion.
4. Confirm observations, entities, source performance, leads, and dark signals are created where appropriate.
5. Confirm fetch failures do not break the whole scan.

## 5. Dark Signals And Verification

1. Confirm public social/community inputs can produce reviewable dark signals.
2. Confirm signals are ranked and explained, but never hidden from the editor.
3. Confirm low-confidence material is kept out of published evidence by default.
4. Convert at least one signal or lead into a verification task.
5. Move verification tasks through suggested, auto-checked, needs-human, blocked, and resolved states where available.

## 6. Writer And Editor Workflow

1. Generate or create 10 to 25 leads from Longmont sources.
2. Draft 5 to 10 reader-facing stories or briefs.
3. Open each draft in the Workbench.
4. Edit text manually.
5. Send at least one draft back for more work.
6. Put at least one draft on hold.
7. Approve at least five publishable items.
8. Run the optional press-freedom/legal-risk advisor on at least one story.
9. Confirm guardrails warn but do not veto the editor.

## 7. Publishing

1. Compile the issue.
2. Confirm the receipt lists article count, generated files, skipped items, and output path.
3. Inspect:
   - homepage
   - article pages
   - RSS feed
   - corrections page
   - about page
   - ethics/reporting pages
   - ZIP export
   - newsletter markdown
   - Substack-ready markdown
   - Facebook, subreddit, Nextdoor, and short-link blurbs
4. Publish anonymously to here.now.
5. Save the here.now URL.
6. Confirm the public site loads and article links work.
7. Confirm the local ZIP matches the published issue.
8. Confirm no duplicate stories are published as separate articles unless the editor intentionally approved both.

## 8. Browser Extension

1. Install or load the extension through the app-guided flow.
2. Generate a pairing code.
3. Pair the extension.
4. Confirm the app's Paired Devices list updates.
5. Send a public page from the browser to the app.
6. Confirm the evidence appears in the newsroom workflow.

## 9. Backup, Restore, And Recovery

1. Create a backup from the UI.
2. Restore from the backup in a clean profile.
3. Interrupt a scan and confirm recovery behavior.
4. Interrupt an import and confirm partial-work handling.
5. Interrupt or fail a publish attempt and confirm recovery copy is clear.

## 10. Output Quality Gate

The smoke test does not pass unless it produces:

- A real Longmont publication.
- 10 to 25 leads.
- 5 to 10 reader-facing stories or briefs.
- A reviewable ZIP/output folder.
- A live here.now URL.
- A human-readable report explaining what passed, what failed, and what was fixed.

One lead or one story is a failure for this gate.
