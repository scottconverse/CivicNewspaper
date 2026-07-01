# 20260701 Evidence Linkage Rerun 33bc936 - Final Report

## Result

Overall: BLOCKED

Exact blocker: the evidence-linkage gate passed mechanically, but the published reader-facing story failed source-grounding. The selected draftable lead, `Council vote on library roof contract` (lead id 22), had three linked evidence rows, so Workbench did not report missing linked source documents. Those linked rows were not semantically appropriate for the council/library roof lead: they came from Downtown Longmont event-calendar excerpts, including Road Pony and other event listings. The generated and published story then mixed unsupported claims into the council/library roof story, including `increase capacity by 40%`, `$10 million over ten years`, and `Road Pony`.

This requires coder action before stable readiness. The product now prevents zero-evidence draft-ready leads, but it still allows evidence-backed drafting when the linked evidence is unrelated or badly crossed between topics.

## Directive And Installer

- Active directive: `test-comms/directives/20260701-evidence-linkage-rerun-33bc936.md`
- Product branch named by directive: `main`
- Product commit represented by installer: `33bc93645ed3a726d7292bd5aad394a677add4e8`
- Installer: `test-comms/artifacts/20260701-evidence-linkage-rerun-33bc936/The Civic Desk_0.3.1_x64-setup.exe`
- SHA256 verified: `4968F81CF21CBAD5DD634375DBF00F67595CE0A023DF0654358F9FBD3092E8E4`
- Size verified: `5638753`

## Counts

- Sources: 19
- Daily Scan runs: 1
- Daily Scan leads: 12
- Story Queue leads: 24
- Evidence items: 65
- Lead-evidence links: 31
- Drafts: 1
- Publish runs: 1
- Published posts: 1

## Evidence-Linkage Audit

PASS mechanically.

- No `ready_to_draft` or `review` lead lacked linked evidence in the current audit.
- Ready/review leads had evidence counts:
  - lead 23, `Annual Music Festival in Longmont`, `ready_to_draft`, evidence count 2
  - lead 22, `Council vote on library roof contract`, `ready_to_draft`, evidence count 3
  - lead 17, `Teen Temporary Tattoo Studio Launch at Longmont Public Library`, `ready_to_draft`, evidence count 3
  - lead 16, `Independence Day Events at City of Longmont`, `review`, evidence count 3
  - lead 15, `New STEM Programs at Longmont Public Library`, `review`, evidence count 3
- Unsupported zero-evidence story/brief leads remained `needs_verification`, including leads 21, 20, and 19.

FAIL semantically.

- Lead 22 was used for the end-to-end publish path.
- Its linked evidence rows were ids 57, 58, and 59.
- The final DB snapshot shows rows 57 and 58 came from `Downtown Longmont events` at `https://www.downtownlongmont.com/events/calendar`, not a council/library roof source.
- The published story mixed event-calendar material into a council/library roof article.

Evidence files:

- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/evidence-linkage-audit.json`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/evidence-linkage-audit-current.json`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/db-snapshot-final.json`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/output-audit.json`
- `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/output-audit-correction.json`

## Workbench And Editor Workflow

PASS for required mechanics.

- Generated draft from an evidence-backed draftable lead.
- Opened the draft in Workbench.
- Workbench guardrails did not report missing linked source documents.
- Story-quality preflight showed one advisory: no clear attribution phrase.
- Exercised send back / needs work modal.
- Exercised hold.
- Resumed editing.
- Marked ready for review.
- Checked publisher attestation.
- Approved for static publish through the warning modal.
- Final draft status: `ready_to_publish`.
- Publisher attestation recorded as `Publisher` at `2026-07-01T07:39:20.711115500+00:00`.

## Export And Publish

PASS for local static export and ZIP.

- Local output path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\evidence\20260701-evidence-linkage-rerun-33bc936\publish-output`
- ZIP/package path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\evidence\20260701-evidence-linkage-rerun-33bc936\publish-output\site-package.zip`
- Generated files: 18
- Article count: 1
- Skipped count: 0

PASS for anonymous here.now publish via the public anonymous here.now API.

- here.now URL: `https://humble-sonnet-9mqm.here.now/`
- Anonymous publish response was sanitized before saving; no claim token or claim URL was committed.
- Sanitized publish evidence: `test-comms/evidence/20260701-evidence-linkage-rerun-33bc936/here-now-publish-sanitized.json`

Product issue: the app connector path did not record a `published_url` or `deployment_id` in `publish_runs`; the local publish run remains `provider=local_export`. The visible connector section exposes a `here.now API key` field, while the directive-required publish was anonymous. Anonymous publish succeeded by direct here.now API, not through a recorded in-app connector publish.

## Output Audits

PASS:

- No forbidden reporter scaffolding markers found in corrected local scan, ZIP extract scan, or live fetches:
  - `EDITOR_NOTE`
  - `[EDITOR_NOTE`
  - `Body:`
  - `Headline:`
  - `Nut graf`
  - `Reporting Steps`
  - `[Source needed]`
  - `[Verification needed]`
  - `[End of Report]`
- No mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD found in corrected local scan, ZIP extract scan, or live fetches.
- Live site fetch with browser-like user agent returned HTTP 200 for:
  - `https://humble-sonnet-9mqm.here.now/`
  - `https://humble-sonnet-9mqm.here.now/index.html`
  - `https://humble-sonnet-9mqm.here.now/briefs/1.html`
  - `https://humble-sonnet-9mqm.here.now/feed.xml`
- No duplicate story topics in the final issue because only one reader-facing story was published.

FAIL:

- Only 1 reader-facing item was published. More source material existed, but I stopped after one publish because the first story failed source-grounding. Publishing additional items would have expanded bad public output instead of proving readiness.
- The published headline is headline-like, but the story body is not source-grounded or useful enough for a Longmont reader.
- The published story includes unsupported or cross-topic claims:
  - `increase capacity by 40%`
  - `$10 million over ten years`
  - `Road Pony`
- The selected lead's linked evidence was not meaningfully tied to the lead topic.

## Screenshots

- `screenshot-01-after-launch-30s.png`
- `screenshot-model-10s.png`
- `screenshot-model-30s.png`
- `screenshot-model-60s.png`
- `screenshot-model-120s.png`
- `screenshot-03-council-draft-flow.png`
- `screenshot-04-after-generate-draft-wait.png`
- `screenshot-05-workbench-draft-row.png`
- `screenshot-06-workbench-open-draft.png`
- `screenshot-07-workflow-send-back.png`
- `screenshot-08-workflow-hold.png`
- `screenshot-09-workflow-send-back-saved.png`
- `screenshot-12-workflow-resume-editing.png`
- `screenshot-13-workflow-ready-review.png`
- `screenshot-14-workflow-attestation-checked.png`
- `screenshot-15-workflow-approve-static-publish.png`
- `screenshot-16-workflow-approved.png`
- `screenshot-17-publishing-panel.png`
- `screenshot-24-publishing-after-identity-edit.png`
- `screenshot-25-compile-checklist.png`
- `screenshot-26-after-zip-package.png`
- `screenshot-27-after-publish-connector.png`
- `screenshot-29-publishing-connector-bottom.png`

## Product Bugs Requiring Coder Action

1. Evidence rows can satisfy the draftability/linkage gate even when they are unrelated to the lead topic.
2. Draft generation can mix unrelated evidence into a published article, producing unsupported claims.
3. Workbench preflight catches "missing linked source documents" but does not catch "linked source documents are unrelated to this lead/story."
4. The app publish connector did not perform or record anonymous here.now publication, even though anonymous here.now publish is supported and succeeded through the direct API.
5. Identity save appears inconsistent: the Publishing panel showed `Longmont Civic Desk`, and the exported site used `Longmont Civic Desk`, but the DB setting `identity.newsroom_name` still read `My Local Publication` in the final snapshot.
