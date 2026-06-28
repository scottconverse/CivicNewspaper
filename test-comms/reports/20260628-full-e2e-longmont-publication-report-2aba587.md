# Full E2E Longmont Publication Report - 2aba587

Tester: Codex cleanroom tester  
Directive: `20260628-rerun-full-e2e-after-herenow-preview-fix-2aba587.md`  
Product branch: `stable-readiness-local-gates`  
Required product commit: `2aba587`  
Result: PASS with noted product-quality observations

## Summary

Installed and ran the clean product build from the preferred NSIS installer artifact. Completed first-run setup for `The Longmont Ledger` in Longmont, CO, installed/started the app-managed local AI runtime, downloaded `qwen2.5:7b`, discovered/imported sources, ran Daily Scan, generated app-authored drafts, exercised advisor/approval/hold editorial paths, compiled a static issue, exported a local ZIP package, published an anonymous here.now preview, and verified the public URL.

Public preview URL:

https://emerald-island-gevx.here.now

HTTP verification:

- Status: `200`
- Title: `The Longmont Ledger`
- Content check: response contained `Longmont`

## Installer And Product Identity

- Preferred installer used: `test-comms/artifacts/2aba587-herenow-preview-publish/The Civic Desk_0.2.8_x64-setup.exe`
- Observed SHA256: `E698D542096C179AEC46A73AC9E68DB984823C6A8C964FB02AF72A018D524D1D`
- Required product branch verified: `origin/stable-readiness-local-gates`
- Required product commit verified: `2aba587a5d97e7ae59bfcf2e2dd6f9a48db6c1b1`
- Installed executable launched from the installed application directory.

## Clean Reset

Before install, I stopped product/runtime processes and removed only CivicNewspaper product state, app-local runtime/model state, and prior CivicNewspaper app state. I did not wipe unrelated tools or user data.

## First-Run Setup And Local AI

Identity used:

- Publication: `The Longmont Ledger`
- Editor: `Cleanroom Tester`
- City/state: `Longmont, CO`
- Publisher type: single-person/local publication

AI/runtime flow:

- The app initially could not reach the local AI service.
- The app-managed local AI runtime installer was used.
- The app started the local AI service successfully.
- The app recommended `qwen2.5:7b` for 15 GB RAM.
- The app-managed model download showed progress and completed at `100.0%`.
- The newsroom workspace showed Local AI ready with `qwen2.5:7b`.

## Source Discovery And Daily Scan

Imported sources:

| ID | Name | Type | Status |
|---:|---|---|---|
| 1 | Longmont official city website | primary_record | online |
| 2 | Longmont public notices search | primary_record | online |
| 3 | Longmont Public Safety | official_comm | offline |
| 4 | r/Longmont | community_signal | online |

Daily Scan result:

- Sources watched: `4`
- Open/reviewable leads produced: `8`
- Target was at least 10 leads, or exact reason and expansion attempt. I attempted expansion through city auto-discovery and imported four candidates including official/public records and a community/social signal. The app produced 8 reviewable leads from that source set, below the 10-lead target.

## Drafting And Editorial Review

Draft generation was performed through the product UI using app-generated content only. I did not hand-author article text.

Draft/editorial status after the run:

| Draft ID | Lead ID | Status | Attested By | Title Prefix |
|---:|---:|---|---|---|
| 1 | 8 | ready_to_publish | Cleanroom Tester | Draft: Longmont Library Programs and Events |
| 2 | 7 | ready_to_publish | Cleanroom Tester | Draft: Vision Zero Projects in Longmont |
| 3 | 6 | ready_to_publish | Cleanroom Tester | Draft: Longmont Museum Exhibits and Events |
| 4 | 5 | ready_to_publish | Cleanroom Tester | Draft: Vision Zero Projects in Longmont |
| 5 | 7 | hold | none | Draft: Vision Zero Projects in Longmont |
| 6 | 7 | ready_to_publish | Cleanroom Tester | Draft: Vision Zero Projects in Longmont |

Editorial paths exercised:

- Generated drafts through the Writer flow.
- Ran the press-freedom/legal-risk advisor on the first story and received advisory output.
- Approved 5 reader-facing items for static publishing with editor attestation.
- Exercised a non-publish disposition by setting one generated draft to `hold`.

Observation: the queue continued to show Draft actions for already-drafted leads, and repeated navigation led to duplicate Vision Zero drafts for lead 7. I preserved this product behavior and reported it rather than editing content or database state.

## Publishing And here.now

Compile/export result:

- Article count: `5`
- Files written: `22`
- Skipped: `0`
- Provider: `here_now`
- Deployment ID/note: `slug=emerald-island-gevx;version=01KW6Y081NPN6129RFVJ4B9S8Z;created_slug=emerald-island-gevx`
- Published URL: `https://emerald-island-gevx.here.now`

here.now flow:

- Provider selected: `here.now`
- API key left blank for anonymous temporary preview.
- Test connection passed: here.now ready for temporary preview publishing.
- Publish with connector succeeded.
- The app displayed and saved the live URL.
- Opened the URL in the system browser and verified by HTTP.

Local output and package:

- Local output folder: `<app-data>/com.scottconverse.civicdesk/sites/default`
- ZIP package in product output: `<app-data>/com.scottconverse.civicdesk/sites/default/site-package.zip`
- Required committed ZIP artifact: `test-comms/artifacts/20260628-full-e2e-longmont-publication-2aba587/site-package-2aba587.zip`
- Required committed manifest artifact: `test-comms/artifacts/20260628-full-e2e-longmont-publication-2aba587/publish-manifest-2aba587.json`

The committed manifest and embedded ZIP manifest were sanitized to remove private machine paths.

## Artifacts

Artifact folder:

`test-comms/artifacts/20260628-full-e2e-longmont-publication-2aba587/`

Screenshots:

- `01-first-run-identity.png`
- `02-ai-service-initial.png`
- `03-runtime-ready.png`
- `04-model-download-complete.png`
- `05-onboarding-ready.png`
- `06-workspace-after-onboarding.png`
- `07-sources-imported.png`
- `08-daily-scan-complete.png`
- `09-story-queue-8-leads.png`
- `10-herenow-publish-success.png`
- `11-herenow-browser-verification.png`

Package artifacts:

- `site-package-2aba587.zip`
- `publish-manifest-2aba587.json`

## Issues / Notes For Coder

1. Daily Scan produced 8 reviewable leads, not 10, even after city discovery/import expansion to four sources including official records and `r/Longmont`.
2. The queue allowed repeated drafting of the same lead and did not clearly remove or disable already-drafted leads; this led to duplicate Vision Zero drafts.
3. Draft content was very thin for some generated stories, especially repeated Vision Zero drafts.
4. One imported source, `Longmont Public Safety`, was marked offline during scan.

## Final Verdict

PASS with notes. The rerun confirms that commit `2aba587` fixes the previously blocked here.now anonymous preview publish path: the connector test passed, publish succeeded, the app displayed/saved a public here.now URL, and the URL returned HTTP 200 with the expected publication title.
