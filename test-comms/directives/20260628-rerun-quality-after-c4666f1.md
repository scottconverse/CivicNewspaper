# Directive: rerun Longmont issue quality after c4666f1

Role: cleanroom tester

Product branch: `stable-readiness-local-gates`

Required product commit: `c4666f1`

This rerun follows the successful `2aba587` cleanroom E2E pass. That pass proved clean install, app-managed local AI setup, model download, Longmont scan, drafting, advisor, approval, local export, and anonymous here.now publish. It also exposed product-quality issues:

- Daily Scan produced 8 leads instead of 10+.
- Story Queue allowed repeated drafting of the same lead.
- Several generated stories/briefs were thin and duplicated the same Vision Zero lead.

Commit `c4666f1` addresses those quality issues by expanding Longmont source candidates, opening existing drafts instead of creating duplicate drafts from the same lead, and strengthening the draft prompt.

## Artifact to install

Preferred installer:

`test-comms/artifacts/c4666f1-quality-rerun/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`7B9DF817D69A5E3D1F30433E6BA1C42B110A76F4E8BE91C778F8E07800015AAF`

Fallback MSI:

`test-comms/artifacts/c4666f1-quality-rerun/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`2CEF07B7957A0989171C81238014FB04F8DDD52821706D2F50AB0BBE188C16E0`

## Clean reset

Do a clean CivicNewspaper product reset as before:

- Stop app/runtime processes.
- Remove CivicNewspaper app profile/database/output/runtime/model state from the test run.
- Do not manually install Ollama, models, PATH entries, OCR/document tools, or other prerequisites.
- Leave Windows, account, browser, Git, Codex, and unrelated tools intact.

## Required test scope

Run the full end-user flow again for Longmont, Colorado:

1. Install and launch the app.
2. Complete first-run identity:
   - Publication: `The Longmont Ledger`
   - Editor: `Cleanroom Tester`
   - City/state: `Longmont, CO`
3. Let the app install/start local AI runtime and download the selected model.
4. Run source discovery for Longmont.
5. Import enough official, local-media, and public community/social sources to make a real issue. Do not stop at four sources if more useful candidates are presented.
6. Run Daily Scan.
7. Record source count, evidence count, and lead count.
8. Verify the Story Queue no longer starts duplicate drafts from already-drafted leads:
   - Draft one lead.
   - Return to Story Queue.
   - Click that same lead/card/button again.
   - Expected: it opens the existing draft or clearly shows `Draft exists` / `Open draft`; it should not silently create another draft for that lead.
9. Generate drafts from distinct leads until at least 5 reader-facing items are approved.
10. Run the advisory review on at least one story.
11. Exercise at least one non-publish disposition such as hold/send back/cut.
12. Compile/export the local static package ZIP.
13. Publish to anonymous here.now through the connector.
14. Open/verify the published URL.

## Quality checks

In the report, evaluate:

- Did source discovery/import provide enough useful Longmont sources?
- Did Daily Scan produce at least 10 leads? If not, exactly what did the app attempt and where did the source/lead volume fall short?
- Did the app prevent duplicate drafting of the same lead?
- Are the 5 published stories/briefs about distinct topics?
- Are drafts still thin, or do they include a useful local-news brief structure with known/unknown/next-step context?
- Are there any obviously fake, hallucinated, or unsupported claims?
- Did the paper feel reviewable by a human local editor?

## Required report file

Write:

`test-comms/reports/20260628-quality-rerun-report-c4666f1.md`

Include:

- PASS / PARTIAL / BLOCKED.
- Product commit tested.
- Installer SHA.
- Machine profile.
- Source list imported.
- Source/evidence/lead/draft/approved counts.
- Duplicate-draft verification result.
- here.now URL.
- Local output path.
- ZIP artifact path.
- Plain-English quality assessment.
- Exact blocker/fix recommendation if any.

## Required artifacts

If export succeeds, copy:

- `site-package.zip` to `test-comms/artifacts/20260628-quality-rerun-c4666f1/site-package-c4666f1.zip`
- `publish-manifest.json` to `test-comms/artifacts/20260628-quality-rerun-c4666f1/publish-manifest-c4666f1.json`
- screenshots covering source discovery/import, scan results, duplicate-draft check, story workbench/advisor, publish receipt, and opened here.now URL.

Commit and push reports/artifacts to `test-comms/cleanroom-coder-tester` with `[skip ci]`.

Keep the watcher armed.
