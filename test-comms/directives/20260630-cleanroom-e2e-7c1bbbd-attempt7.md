# CivicNewspaper Cleanroom E2E Attempt 7

Status: ACTIVE
Issued by: coder
Issued at: 2026-06-30T14:04:00Z

Single source of truth:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Coordination branch: test-comms/cleanroom-coder-tester
- Tester coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-only path on the tester machine: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product under test:

- Product branch: main
- Product commit: 7c1bbbd42279c13adeb80d604b156a2e6df7eb81
- Product version: 0.3.0

Installer artifacts:

- Artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd
- NSIS installer: test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/The Civic Desk_0.3.0_x64-setup.exe
- NSIS SHA256: E45BD165A902AE711F950B3CA39EAA4E5BFBA30946F54A06E866504EB40B7C86
- NSIS size bytes: 5623239
- MSI installer: test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/The Civic Desk_0.3.0_x64_en-US.msi
- MSI SHA256: DBFC81BF4F4916A15D631940A0A484BD4A89AAEE3DA527DDBC2A7BFF87CAB18A
- MSI size bytes: 9125888

Reports to write:

- Visibility report: test-comms/reports/20260630-cleanroom-e2e-7c1bbbd-visibility-attempt-7.md
- Final report: test-comms/reports/20260630-cleanroom-e2e-7c1bbbd-report.md
- Evidence folder: test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd
- Tester output artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/tester-output/

Immediate visibility check:

1. Fetch and pull this branch.
2. Read test-comms/ACTIVE_DIRECTIVE.md.
3. Read this directive file.
4. Verify the tester is running as msi\civic and the coordination path is C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms.
5. Verify both installer artifacts exist locally after pull and their SHA256 hashes match the values above.
6. Write the visibility report before running the full cleanroom test.

Cleanroom reset boundary:

- Remove CivicNewspaper app data, previous CivicNewspaper installs, prior test output, prior exported publications, prior local here.now artifacts, and prior local model/app state used only for this test.
- Do not reset Windows itself.
- Do not manually install product dependencies for CivicNewspaper.
- If the app installer or first-run flow cannot set up a required dependency, report that as product failure. Do not repair it manually unless this directive explicitly says to.

Primary regressions to prove fixed:

1. Workbench approval from warning state:
   - Find or generate a draft that reaches ready_to_review and has story-quality or guardrail warnings.
   - Click Approve for Static Publish through the normal UI.
   - If a warning modal appears, record the warning count/text, provide the editor note requested by the UI, and continue approval.
   - The story must transition to ready_to_publish in the UI and in the final database snapshot.
   - Attestation and warning/audit logging may happen, but logging must not veto or strand the editor decision.

2. Workbench navigation recovery:
   - After the approval attempt, use Back to Queue and sidebar navigation to Story Queue and Publishing.
   - The tester must not remain trapped in a stale Workbench selection.
   - Publishing controls must be reachable without restarting the app.

3. Preserve attempt-6 quality gains:
   - Low-novelty, recurring, background, watch, verification, or non-ready leads must remain clearly labeled.
   - Weak generated drafts should be marked as needing more work or require a conscious warning checkpoint before approval.
   - The app must not veto the editor. It may warn and log the editor decision.

4. Prior quality failures remain blocked:
   - Generic monitoring pages such as "new official document fetched" or "decision keywords identified" must not be counted as successful reader-facing stories.
   - Generic source-intake notes, search-result notes, static service descriptions, and keyword-detection notes are failures if presented as stories.

Full E2E test:

1. Install CivicNewspaper from the NSIS installer unless Windows blocks it completely. Use the MSI only if the NSIS installer cannot proceed.
2. Complete onboarding as a normal user for Longmont, Colorado.
3. Let the app perform any app-guided AI/runtime/model setup. The tester must not manually install Ollama or models.
4. Run Daily Scan for Longmont.
5. Confirm source intake includes official and public/social/community sources.
6. Confirm the Story Queue produces multiple leads.
7. Draft only leads with the normal Draft action first.
8. Use Draft anyway only when the UI gives an explicit reason and record that reason.
9. Produce at least 5 reader-facing stories or briefs if the app can support it from the scan.
10. Exercise writer/editor workflow:
    - Generate draft.
    - Edit copy.
    - Save.
    - Put on hold.
    - Return/send back for more work if exposed.
    - Cut or remove one unsuitable item if exposed.
    - Approve publishable items, including at least one warned item through the warning checkpoint.
11. Publish a static issue locally.
12. Export the publication ZIP.
13. Publish the same issue to here.now using the app's anonymous preview flow.
14. Save the here.now URL in the final report.

Output quality checks:

- No duplicate topic cluster like two stories about the same Building Services portal outage.
- No public leakage of EDITOR_NOTE, [EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], [End of Report], or similar reporter scaffolding.
- Public story pages must use real headlines, not lead-summary blobs.
- Stories should read as public copy or clearly labeled briefs/watch items, not raw reporter notes.
- Stories must have a current, specific reason to exist.
- Public output must not contain mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.
- ZIP extract, RSS/share artifacts, local HTML output, and here.now pages must all pass the same public-output scans.

Final report requirements:

- Plain-English verdict: PASS or FAIL.
- Exact install path used.
- Exact product commit installed.
- Installer hash verification results.
- Whether app-guided AI/runtime/model setup worked without tester manual installation.
- Whether identity displayed cleanly as Longmont / CO.
- Whether first-run starter sources were seeded without manual import.
- Daily Scan outcome.
- Lead count and story/brief count.
- Editorial workflow outcomes, especially the warned ready_to_review approval path.
- Final database statuses for approved, held, sent-back, and cut drafts.
- Which weak or generic items were held, cut, sent back, or approved after warning.
- Local static output path.
- ZIP output path.
- here.now URL.
- Output quality scan results.
- Screenshots or copied text evidence for any failure.
- If FAIL, state exactly where it broke and what the user could not do next.

After writing the final report, keep the 15-minute repo watcher armed and continue treating test-comms/ACTIVE_DIRECTIVE.md as the only active directive pointer.
