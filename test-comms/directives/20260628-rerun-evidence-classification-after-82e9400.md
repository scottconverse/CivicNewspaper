# Directive: rerun evidence/classification quality after 82e9400

Role: cleanroom tester

Product branch: `stable-readiness-local-gates`

Required product commit: `82e9400`

This rerun follows the `c4666f1` quality rerun, which proved the app can produce and publish a Longmont issue but still reported:

- Bulk Import stored local media/community sources as official/primary.
- Some Daily Scan leads/drafts had `Linked Sources (0)`.
- Drafts sometimes invented unsupported specifics.
- Related topics were still not fully clustered.

Commit `82e9400` fixes or improves the first three directly:

- Bulk import now infers media/community/official source type from URL/name when rows omit type columns.
- Daily Scan story leads now link matching source evidence into the Story Queue/draft path.
- Draft prompt now forbids invented dates, durations, causes, officials, quotes, project history, and technical details unless present in linked evidence.

## Artifact to install

Preferred installer:

`test-comms/artifacts/82e9400-evidence-classification-rerun/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`BC2DB21D66F8F9E9DAB52C59FD282EC2EF845F8AF01CF18DB60C42EFE4138EA1`

Fallback MSI:

`test-comms/artifacts/82e9400-evidence-classification-rerun/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`E4F177F6CA26DFEB33B97CBF300987D2B753B80FFEDF7C5310E5FEDCD07C5B9A`

## Reset

Do the same clean CivicNewspaper product reset as prior runs. Do not manually install Ollama, models, PATH entries, or app dependencies.

## Required checks

Run the Longmont flow again:

1. Clean install and first-run setup for `The Longmont Ledger`, Longmont, CO.
2. Let app install/start local AI runtime and pull selected model.
3. Use source discovery and/or Bulk Import through the product UI to add official, local media, and public community/social sources.
4. Specifically verify stored source type/tier after import:
   - Longmont Leader and Times-Call should be Media / Secondary or equivalent, not Primary.
   - r/Longmont and Facebook/social/community sources should be Watch/community, not Primary.
   - official city/public notice/agendas sources should remain Primary.
5. Run Daily Scan and/or Story Queue scan until 10+ leads are available or explain exactly why not.
6. Generate at least 5 drafts from distinct leads.
7. For at least 3 generated drafts, verify Workbench shows linked sources/evidence. Record how many generated drafts still show `Linked Sources (0)`.
8. Run advisor on at least one draft.
9. Approve at least 5 items, hold/cut/send back at least 1 item.
10. Compile/export ZIP and publish anonymous here.now.
11. Open/verify here.now URL.

## Quality audit

Read the generated story bodies closely enough to answer:

- Did the app add unsupported dates, durations, causes, officials, project history, quotes, or technical details?
- Did evidence citations appear for factual claims when linked evidence existed?
- Are the 5 approved stories/briefs useful as working drafts for a local editor?
- Are there still obvious topic duplicates that should be clustered before Story Queue?

## Report

Write:

`test-comms/reports/20260628-evidence-classification-rerun-report-82e9400.md`

Include:

- PASS / PARTIAL / BLOCKED.
- Installer SHA and commit.
- Source table with stored type/tier for each source.
- Source/evidence/lead/draft/approved/held counts.
- Linked-source count for each generated draft checked.
- here.now URL.
- ZIP artifact path.
- Plain-English story-quality assessment.
- Remaining blockers and exact fix recommendations.

## Artifacts

If export succeeds, copy:

- ZIP to `test-comms/artifacts/20260628-evidence-classification-rerun-82e9400/site-package-82e9400.zip`
- Manifest to `test-comms/artifacts/20260628-evidence-classification-rerun-82e9400/publish-manifest-82e9400.json`
- screenshots of source type/tier rows, Daily Scan/Story Queue, linked sources in Workbench, advisor, publish receipt, and opened here.now URL.

Commit/push with `[skip ci]`.

Keep watcher armed.
