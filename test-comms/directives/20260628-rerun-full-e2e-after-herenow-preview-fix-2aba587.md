# Directive: rerun full E2E after here.now preview publish fix 2aba587

Role: cleanroom tester

Product branch: `stable-readiness-local-gates`

Required product commit: `2aba587`

This directive supersedes the previous `ff21a83` cleanroom rerun. The previous run proved clean install, app-managed local AI runtime install, app-managed model pull, Longmont source discovery/import, Daily Scan lead promotion, one draft, advisor review, approval, and local ZIP export. It blocked because the here.now anonymous connector publish button did not produce a URL or publish record, and because the run stopped after one story.

## Artifact to install

Preferred installer:

`test-comms/artifacts/2aba587-herenow-preview-publish/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`E698D542096C179AEC46A73AC9E68DB984823C6A8C964FB02AF72A018D524D1D`

Fallback MSI:

`test-comms/artifacts/2aba587-herenow-preview-publish/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`D4F368B6F0BB07AEE8F36486AE6303C88946C1A1C45AC7349F0A703DDD84776D`

## Cleanroom reset boundary

Before installing this artifact, wipe only CivicNewspaper-related product state:

- Stop CivicNewspaper and app-local Ollama/runtime processes.
- Remove CivicNewspaper app data/profile/database from this cleanroom run.
- Remove app-managed Ollama/runtime/model state created by CivicNewspaper.
- Remove prior CivicNewspaper test output folders created by earlier cleanroom runs.
- Leave Windows, the user account, browser, Git, Codex, and unrelated tools intact.

Do not manually install Ollama, models, PATH entries, document tools, OCR tools, or app prerequisites. If the product needs a dependency and the app/installer does not install or drive it, report that as a product failure.

## Required test scope

Test the full product as an end user, top to bottom. Use the UI unless a report-only inspection is explicitly needed. Do not handwrite article content. Do not manually install missing dependencies. Do not bypass the app workflow.

Target publication identity:

- Publication name: `The Longmont Ledger`
- Editor name: `Cleanroom Tester`
- City/state: `Longmont, CO`
- Organization type: choose a realistic single-person or local publication option if asked.

Target city/reporting scope:

- Longmont, Colorado.
- Include official sources and public/social/community sources that are readable without login.
- Public readable scraping is allowed. Do not use private groups, private accounts, credentials, or non-public data.

## Pass/fail bar

This run passes only if the app, from a clean CivicNewspaper/product state:

1. Installs and launches.
2. Completes first-run identity setup.
3. Detects the machine profile well enough to pick a model path.
4. Installs/starts the needed local AI runtime itself.
5. Downloads the chosen model itself with visible progress.
6. Discovers/imports useful Longmont sources, including official and public/community/social candidates.
7. Runs Daily Scan from the UI and produces at least 10 reviewable leads, or records exactly why the app could not reach that count and what expansion it attempted.
8. Lets a writer turn leads into drafts using the local AI.
9. Lets an editor open, edit/save, run the optional advisor, approve, send back/hold/cut where applicable, and continue to the next story.
10. Produces a full publication with at least 5 reader-facing approved items. These may be stories and briefs, but they must be real outputs generated from the workflow, not hand-authored by the tester outside the app.
11. Exports the local static package ZIP through the Publishing screen.
12. Publishes the exported issue through anonymous here.now preview publishing and records/displays a public URL.
13. Provides the local output folder and ZIP path.
14. Provides the here.now URL.
15. Provides screenshots and a plain-English report.

If any step breaks, write exactly where it breaks, include screenshots/logs/DB evidence when available, and leave the watcher armed. Do not “help” the product by manually installing dependencies or writing content outside the product.

## Multi-story requirement

The previous run stopped at one story. Do not stop at one story in this run. After Daily Scan, open Story Queue and repeat the in-app flow for enough leads to approve at least 5 reader-facing items:

- Open a lead.
- Generate a draft using the app's selected local model.
- Open/save the draft in Workbench.
- Run the press-freedom/legal-risk advisor when available.
- Exercise at least one non-publish editorial disposition if the UI supports it, such as hold/send back/cut, then continue with other stories.
- Approve at least 5 items for publication.
- Compile/export/publish the issue.

If the app becomes too slow or gets stuck during draft generation, report the exact model, elapsed time, UI state, and whether cancel/retry/recovery works.

## here.now verification

The fix in commit `2aba587` is specifically intended to make anonymous here.now preview publish work even if no here.now API key or saved connector profile exists.

Required here.now steps:

- Compile the site locally first.
- In Publishing, use provider `here.now`.
- `Test connection` should pass for temporary preview publishing.
- Click `Publish with connector`.
- Wait for visible progress or result.
- Verify a public URL appears in the app or publish history.
- Open the public URL in a browser and verify it loads the generated paper.
- Include the URL in the report.

## Required report file

Write the report to:

`test-comms/reports/20260628-full-e2e-longmont-publication-report-2aba587.md`

Include:

- Result: PASS / PARTIAL / BLOCKED.
- Product commit tested.
- Installer artifact and SHA256.
- Machine profile observed.
- Clean reset actions.
- Onboarding/runtime/model setup evidence.
- Sources imported, with source names/types/status if visible.
- Daily Scan lead count and evidence count.
- Number of drafts generated.
- Number of advisor reviews run.
- Number of items approved.
- Local output folder path.
- ZIP path.
- here.now URL.
- Screenshot list.
- Plain-English quality notes about the generated paper.
- Any exact blocker and recommended next fix.

## Required artifacts

If the app successfully exports a ZIP, copy it to:

`test-comms/artifacts/20260628-full-e2e-longmont-publication-2aba587/site-package-2aba587.zip`

Also copy, when available:

- `publish-manifest.json`
- screenshots of source discovery, Daily Scan results, Story Queue with leads, Workbench draft/advisor/approval, Publishing compile receipt, here.now URL/result, and opened published URL.

Commit/push report and artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.

Keep your 15-minute watcher armed after reporting, unless coder explicitly tells you the loop is complete.
