# c4666f1 Longmont quality rerun report

Verdict: PARTIAL

## Scope

- Role: cleanroom tester
- Product branch tested: `stable-readiness-local-gates`
- Product commit tested: `c4666f1b4b1824be5bbd8df201f26cc59421d1c8`
- Installer used: `test-comms/artifacts/c4666f1-quality-rerun/The Civic Desk_0.2.8_x64-setup.exe`
- Installer SHA256: `7B9DF817D69A5E3D1F30433E6BA1C42B110A76F4E8BE91C778F8E07800015AAF`
- Machine profile: Windows 11 Home 10.0.26200, Intel Core i7-13620H, 15.7 GB RAM
- Local AI runtime/model: app-managed runtime install, `qwen2.5:7b` downloaded and used

## Result summary

The clean install, first-run setup, app-managed local AI install/model download, source import, queue scan, duplicate-draft prevention, advisor, approval flow, hold disposition, local compile/export, and anonymous here.now publish all completed.

The run is still PARTIAL because the quality workflow has remaining issues:

- Auto-discovery initially presented/imported only 1 Longmont source. I had to use Bulk Import to add the rest.
- Bulk Import correctly flagged local media and community URLs for review, but stored every imported source as `primary_record` / `official_record`, including Longmont Leader, Times-Call, and r/Longmont.
- Daily Scan ultimately produced 11 leads, but the queue required the Story Queue `Scrape & Detect` action after the Daily Scan page initially showed 0 open leads. The later Daily Scan UI receipt reported `Evidence: 20` / `Saved leads: 11`; the database contained 258 `evidence_items`.
- The draft prompt is stronger structurally, but generated drafts still introduced unsupported specifics. Example: the permitting-portal draft added "Since early April" crash/slow-response detail not present in the lead snippet I saw.
- The five published items came from five distinct lead IDs, but topics were not fully distinct: two museum stories and repeated permitting/Vision Zero/library themes were present in the 11-lead queue.

## Sources imported

Auto-discovered:

- Longmont official city website - `https://www.longmontcolorado.gov/`

Bulk-imported through product UI:

- Longmont City Council Agendas - `https://www.longmontcolorado.gov/departments/departments-a-d/city-clerk/agendas-and-minutes`
- Longmont Public Safety - `https://www.longmontcolorado.gov/departments/departments-n-z/public-safety`
- Longmont Public Library Events - `https://www.longmontcolorado.gov/departments/departments-e-m/library`
- Longmont Museum Events - `https://www.longmontcolorado.gov/departments/departments-e-m/museum`
- Longmont Leader - `https://www.longmontleader.com/`
- Times-Call Longmont - `https://www.timescall.com/`
- Boulder County Public Notices - `https://bouldercounty.gov/news/`
- r/Longmont - `https://www.reddit.com/r/Longmont/`

Stored source count: 9.

## Counts

- Sources: 9
- Evidence rows in DB: 258
- Daily scan leads: 11
- Story leads: 11
- Drafts: 6
- Approved / ready to publish: 5
- Held: 1
- Exported articles: 5
- Exported files: 22
- Export skipped count: 0

## Duplicate-draft verification

PASS. I drafted lead ID 11, returned to Story Queue, and clicked the same lead again. The queue showed `Draft exists` and the action changed to `Open draft`. Reopening it did not create a duplicate: DB count stayed at exactly one draft for lead ID 11.

## Approved stories

- Lead 11: Vision Zero Projects in Longmont
- Lead 7: Book-a-Librarian Workshop for Tweens
- Lead 10: Longmont Museum Events and Programs
- Lead 9: Technical Issues with Building Services Online Permitting Portal
- Lead 8: Longmont Museum Closure for Expansion

Non-publish disposition:

- Lead 6: Longmont Library Offers Spanish-Specific Programs and Services - status `hold`

## Advisor

Ran the press-freedom / legal-risk advisor on the Vision Zero draft. It completed and produced an editorial/legal-risk review. The advisor output was useful because the generated draft had no linked source documents and needed verification caution.

## Publish result

- Local output path: `<app-profile>/sites/default`
- ZIP artifact: `test-comms/artifacts/20260628-quality-rerun-c4666f1/site-package-c4666f1.zip`
- Manifest artifact: `test-comms/artifacts/20260628-quality-rerun-c4666f1/publish-manifest-c4666f1.json`
- here.now URL: `https://eternal-trellis-g2f7.here.now`
- HTTP verification: 200 OK, title `The Longmont Ledger`
- Published provider: `here_now`
- Deployment ID: `slug=eternal-trellis-g2f7;version=01KW71X5Q5N3E2GV5QKJ1GREKN;created_slug=eternal-trellis-g2f7`

## Quality assessment

Source volume is improved only after manual tester intervention. The app did not independently discover enough useful Longmont sources in the initial source-discovery UI. Bulk Import let me add official, local-media, and community sources, but classification quality regressed or remains incomplete because all stored records became official/primary.

Lead volume meets the 10+ target after using Story Queue `Scrape & Detect`, with 11 saved leads. However, the topic set still contains duplicates and near-duplicates. A human local editor could review the resulting issue, but should not publish without fact-checking several AI-added claims and merging overlapping museum/permitting/Vision Zero angles.

Draft structure is better than the prior run: the drafts generally include headline, nutgraf, factual paragraphs, unanswered questions or community impact. The main remaining quality issue is unsupported specificity. Drafts are usable as rough working drafts, not publication-ready copy.

## Fix recommendations

1. Expand or repair automatic Longmont source discovery so the tester does not need to paste a hand-built source list.
2. Preserve source classification from Bulk Import review: local media should not be stored as `official_record`, and community/social should not become `primary_record`.
3. Link leads and drafts back to evidence rows. Several drafted leads showed `Linked Sources (0)`, increasing hallucination risk.
4. Tighten drafting prompts or guardrails to forbid invented dates, durations, causes, officials, project details, or history unless present in linked evidence.
5. Deduplicate lead topics before they reach Story Queue, or cluster related leads so five approved items are more clearly distinct.

## Evidence artifacts

Screenshots and exported artifacts are under:

`test-comms/artifacts/20260628-quality-rerun-c4666f1/`
