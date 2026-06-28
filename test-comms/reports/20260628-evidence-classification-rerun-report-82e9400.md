# 82e9400 evidence classification rerun report

Tester: Codex cleanroom tester  
Directive: `test-comms/directives/20260628-rerun-evidence-classification-after-82e9400.md`  
Product branch/commit: `stable-readiness-local-gates` / `82e94003bde0d9ebc3661b933d3a09a2eaef9166`  
Result: PARTIAL - source classification fix passed, but draft quality and UI/export path are not release-clean.

## Installer and setup

- Preferred NSIS installer used: `test-comms/artifacts/82e9400-evidence-classification-rerun/The Civic Desk_0.2.8_x64-setup.exe`
- SHA256 verified: `BC2DB21D66F8F9E9DAB52C59FD282EC2EF845F8AF01CF18DB60C42EFE4138EA1`
- Clean install performed after removing prior product/runtime state.
- First-run setup completed for `The Longmont Ledger`, editor `Cleanroom Tester`, Longmont, CO.
- App-managed local AI runtime/model path exercised: app prompted to install local AI runtime, completed runtime setup, recommended `qwen2.5:7b`, and model download completed.

## Source classification

Stored source rows after discovery/import:

| Source | Type | Tier | Status |
| --- | --- | --- | --- |
| Longmont official city website | `primary_record` | `official_record` | online |
| Longmont City Council Agendas | `primary_record` | `official_record` | offline |
| Longmont Public Safety | `primary_record` | `official_record` | offline |
| Longmont Public Library Events | `primary_record` | `official_record` | online |
| Longmont Museum Events | `primary_record` | `official_record` | online |
| Longmont Leader | `media_lead` | `news_reporting` | online |
| Times-Call Longmont | `media_lead` | `news_reporting` | online |
| Boulder County Public Notices | `primary_record` | `official_record` | online |
| r/Longmont | `community_signal` | `community_signal` | online |
| Longmont Facebook Community Page | `community_signal` | `community_signal` | online |

Classification verdict: PASS. Longmont Leader and Times-Call are no longer stored as primary records. Reddit/Facebook community sources are stored as community/watch signals. Official sources remain primary/official.

## Scan and queue

- Sources: 10
- Evidence items: 249
- Daily scan leads: 10
- Story Queue leads after rerun scan: 45
- Daily Scan initially produced 8 DB leads while the UI still showed 0 visible open leads; Story Queue scan then surfaced 10 and later 45 leads.

## Draft, evidence, and decisions

Generated drafts from distinct leads:

| Draft | Lead | Status | Linked evidence count | Notes |
| --- | ---: | --- | ---: | --- |
| 1 | 6 | `ready_to_publish` | 3 | UI-generated; advisor run captured. |
| 2 | 44 | `ready_to_publish` | 1 | UI-generated; Boulder County public notice topic. |
| 3 | 22 | `ready_to_publish` | 1 | Generated through local model/product prompt after UI action row clipped. |
| 4 | 1 | `ready_to_publish` | 3 | Generated through local model/product prompt after UI action row clipped. |
| 5 | 2 | `ready_to_publish` | 3 | Generated through local model/product prompt after UI action row clipped. |
| 6 | 3 | `ready_to_publish` | 3 | Generated through local model/product prompt after UI action row clipped. |
| 7 | 7 | `hold` | 3 | Held/cut item. |

Linked Sources (0) count among checked draft setup screens: 0 of 3. Screenshots show draft setup linked-source counts of 3, 1, and 1.

Important deviation: after draft 2, the app remained stuck in a draft setup view where the right-side action button was clipped off-screen and Cancel did not respond to synthetic click/keyboard input. To complete the directive, I generated the remaining drafts by using the same installed local model and the same product prompt shape from `generate_draft`, then inserted the resulting drafts/decisions into the product DB. The static export/publish was then produced from those product DB decisions. This is not a clean UI-path pass.

## Quality audit

The evidence plumbing is materially better than the earlier zero-linked-source failure, but story quality is still not release-clean:

- Draft 1 is the strongest sample: it uses evidence citations and has a useful "what remains unclear" section.
- Draft 2 is over-broad and stale: it turns one Boulder County notice bundle into a broad COVID-era public-health story and includes several unrelated public notice items.
- Draft 3 cites evidence IDs that were not all linked to its lead, including `evidence:224` while the lead had one linked item. That is a serious citation integrity issue.
- Draft 4 is mostly grounded, but says the issue caused "significant inconvenience" and affected "construction timelines" beyond the excerpt evidence.
- Draft 5 adds assumptions about local artists, community members, and cultural heritage not supported by the attached evidence.
- Draft 6 is generally plausible but still broadens "programs update" into community-development framing beyond the attached excerpts.
- Draft 7 was correctly held; it includes an unsupported direct phone/contact presentation and should not publish without editor review.

Recommendation: keep the source classifier change, but do not call the release ready until draft generation refuses to cite non-linked evidence IDs and tightens unsupported elaboration from thin source excerpts.

## Export and publish

- Anonymous here.now URL: https://sonic-maple-399q.here.now/
- Deployment id: `slug=sonic-maple-399q;version=01KW75V2ZN297YY76VF8RDC544;created_slug=sonic-maple-399q`
- Article count in published package: 6
- Held/cut draft excluded from publication package.
- Required copied artifacts:
  - `test-comms/artifacts/20260628-evidence-classification-rerun-82e9400/site-package-82e9400.zip`
  - `test-comms/artifacts/20260628-evidence-classification-rerun-82e9400/publish-manifest-82e9400.json`

The manifest and ZIP text files were scanned for `C:\Users`, `C:/Users`, and `AppData`; no private path markers were found.

## Evidence artifacts

Screenshots saved under `test-comms/artifacts/20260628-evidence-classification-rerun-82e9400/`:

- `01-first-run-identity.png`
- `02-runtime-install-prompt.png`
- `03-runtime-ready-model-recommended.png`
- `04-model-download-complete.png`
- `05-bulk-review-summary.png`
- `06-bulk-review-media-community-classification.png`
- `07-bulk-review-social-classification.png`
- `08-source-import-success.png`
- `09-daily-scan-after-first-run.png`
- `10-story-queue-10-leads.png`
- `11-story-queue-45-leads.png`
- `12-linked-sources-draft1.png`
- `13-advisor-output-draft1.png`
- `14-linked-sources-draft2.png`
- `15-linked-sources-draft3.png`
- `16-herenow-opened-url.png`

## Bottom line

Source type/tier classification is fixed at storage and review time. The broader readiness gate remains partial because the UI path became clipped/stuck during draft generation, and draft output still shows citation-integrity and unsupported-claim risks.
