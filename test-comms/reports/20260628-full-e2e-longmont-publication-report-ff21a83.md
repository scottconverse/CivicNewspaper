# Tester report: ff21a83 full E2E Longmont publication rerun

Directive: `test-comms/directives/20260628-rerun-full-e2e-after-draft-save-scan-queue-fix-ff21a83.md`

Tester role: cleanroom tester

Product branch: `stable-readiness-local-gates`

Product commit verified: `ff21a8309a37d44039fa1e760e9b542cf2a1b14c`

Result: **PARTIAL / BLOCKED**

## Summary

The ff21a83 rerun fixed the two previously targeted issues:

- Daily Scan now promotes scan findings into Story Queue leads without requiring Scrape & Detect.
- Draft generation now persists successfully; no `created_at` save failure occurred.

Clean install, onboarding, bundled local AI runtime install, model download, Longmont source discovery/import, Daily Scan, Story Queue promotion, draft generation, advisor review, approval, and local static package export all passed.

The run did **not** complete the full publication directive because the here.now connector publish action did not produce a URL or DB publish record after the connector test passed. The run also ended with 1 approved/exported story, not the requested 5-10 stories/briefs.

## Artifact hashes

- Preferred installer: `test-comms/artifacts/ff21a83-draft-save-scan-leads/The Civic Desk_0.2.8_x64-setup.exe`
  - SHA256: `879CC345B1A01D2673B525712BEA89258008877A953592284434AD8CFFEAEF02`
- Fallback MSI: `test-comms/artifacts/ff21a83-draft-save-scan-leads/The Civic Desk_0.2.8_x64_en-US.msi`
  - SHA256: `70F411BC2E163BDE2BD6A68E9B139CE51E51C7272B4B6150A061E0F662A2EF6F`
- Exported local ZIP copy: `test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/site-package-ff21a83.zip`
  - SHA256: `3B4FFD2802CD6F5F9851FF5DB42106D55B478F5085E17143CBA96306A001BEA7`

## Environment

- OS: Windows 11 Home `10.0.26200`
- CPU: 13th Gen Intel Core i7-13620H
- RAM: app detected 15 GB local RAM
- GPU devices observed: Intel UHD Graphics and NVIDIA GeForce RTX 4050 Laptop GPU
- Installed app executable: `civicnews.exe`
- Bundled runtime observed: app-local Ollama `v0.30.11`
- Model downloaded by app: `qwen2.5:7b`

## Steps and evidence

1. Clean product state
   - Stopped app/runtime processes.
   - Removed prior app profile, Ollama/runtime state, and installer state before installing the ff21a83 NSIS artifact.

2. Install and onboarding: **PASS**
   - Installed from the preferred NSIS artifact.
   - Launched to first-run onboarding.
   - Entered publication identity:
     - Publication: `The Longmont Ledger`
     - Editor: `Cleanroom Tester`
     - City/state: `Longmont, CO`
   - App installed and started bundled local AI runtime.
   - App downloaded `qwen2.5:7b` to completion.
   - Workspace opened with Local AI ready.
   - Screenshot evidence: `01` through `09` in `test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/`.

3. Longmont source discovery/import: **PASS with one offline source**
   - Discovery found Longmont candidate sources.
   - Imported/added 3 sources:
     - `Longmont official city website`: online
     - `Longmont public notices search`: online
     - `Longmont agendas and minutes`: offline on fetch
   - DB evidence after run:
     - sources: 3
     - evidence items: 10
   - Screenshot evidence: `10-sources-empty.png` through `14-sources-after-import.png`.

4. Daily Scan: **PASS**
   - Daily Scan completed.
   - App UI reported: `Daily Scan saved 8 lead(s). Model: qwen2.5:7b. Evidence: 10. Saved leads: 8.`
   - DB evidence:
     - `daily_scan_runs`: 1 completed run
     - `daily_scan_leads`: 8
     - `leads`: 8
   - Screenshot evidence: `16-daily-scan-before-run.png`, `18-story-queue-after-daily-scan.png`.

5. Story Queue promotion after Daily Scan: **PASS**
   - Story Queue showed `Leads 8`, `Drafts 0` immediately after Daily Scan.
   - This confirms Daily Scan leads are draftable without Scrape & Detect as the only path.
   - Screenshot evidence: `18-story-queue-after-daily-scan.png`.

6. Draft generation and save: **PASS**
   - Generated one draft from a Daily Scan lead.
   - Draft opened in Story Workbench with title/body.
   - No `created_at` save error occurred.
   - DB evidence:
     - `drafts`: 1
     - draft status after approval: `ready_to_publish`
     - `attested_by`: `Cleanroom Tester`
     - `attested_at`: `2026-06-28T09:49:48.276454600+00:00`
   - Screenshot evidence: `19-draft-form-first-lead.png`, `20-draft-generation-result.png`, `21-workbench-draft-saved.png`.

7. Advisor review: **PASS**
   - Ran the Workbench press-freedom/legal-risk advisor.
   - Advisor produced a Markdown advisory review for the draft.
   - Screenshot evidence: `22-advisor-review-result.png`.

8. Approval for static publish: **PASS**
   - Approved the draft for static publish.
   - Workbench displayed `Current Status: ready_to_publish`.
   - DB confirmed `status = ready_to_publish`, `attested_by = Cleanroom Tester`, and non-null `attested_at`.

9. Local export ZIP/package: **PASS**
   - Publishing panel compiled the local static package.
   - Compile receipt showed:
     - articles: 1
     - files: 18
     - skipped: 0
   - DB `publish_runs` row:
     - provider: `local_export`
     - article_count: 1
     - skipped_count: 0
     - files_written: 18
     - published_url: null
   - Generated files included:
     - `index.html`
     - `watch/1.html`
     - `newsletter.md`
     - `substack.md`
     - `share-package.md`
     - social copy files
     - `publish-manifest.json`
     - `site-package.zip`
   - Screenshot evidence: `23-compile-receipt.png`.
   - Copied package artifacts:
     - `test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/site-package-ff21a83.zip`
     - `test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/publish-manifest-ff21a83.json`

10. here.now anonymous publish: **BLOCKED**
    - Publishing panel connector copy states anonymous 24-hour preview publishing is supported.
    - `Test connection` passed with message:
      - `Test passed: here.now is ready for temporary preview publishing. Save an API key for permanent sites.`
    - `Publish with connector` became visually available, but two deliberate clicks followed by waits of 30 seconds and 65 seconds produced no visible progress, URL, error, deployment ID, or DB publish record.
    - DB still contains only the local export row; no connector publish row was recorded.
    - Screenshot evidence: `24-herenow-connector-passed-test-inert-publish.png`.

## Unmet directive items

- 5-10 stories/briefs were not completed. Only 1 story was drafted, advisor-reviewed, approved, and exported.
- here.now anonymous publish did not complete. No public URL was produced.

## Notes

- I did not manually install Ollama or the model; the app performed runtime install and model download.
- I did not handwrite story content.
- I did not commit credentials or private machine data.
- No merge, tag, or external publication outside the explicit here.now connector attempt was performed.
