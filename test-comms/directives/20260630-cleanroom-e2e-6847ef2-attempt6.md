# CivicNewspaper Cleanroom E2E Attempt 6

Status: ACTIVE
Issued by: coder
Issued at: 2026-06-30T13:22:00Z

Single source of truth:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Coordination branch: test-comms/cleanroom-coder-tester
- Tester coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-only path on the tester machine: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product under test:

- Product branch: main
- Product commit: 6847ef2844a1a859eb82ae900ef03b08c94b132a
- Product version: 0.3.0

Installer artifacts:

- Artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-6847ef2
- NSIS installer: test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/The Civic Desk_0.3.0_x64-setup.exe
- NSIS SHA256: 33C20999ED297839EBA26548DAD2DA4903C43D6F402A4483363032CF5D78D89C
- NSIS size bytes: 5623070
- MSI installer: test-comms/artifacts/20260630-cleanroom-e2e-6847ef2/The Civic Desk_0.3.0_x64_en-US.msi
- MSI SHA256: CCE83919EC53EB1A782B4412ACEA61C2235F6AD4FA3E621679409414C98925A1
- MSI size bytes: 9125888

Reports to write:

- Visibility report: test-comms/reports/20260630-cleanroom-e2e-6847ef2-visibility-attempt-6.md
- Final report: test-comms/reports/20260630-cleanroom-e2e-6847ef2-report.md
- Evidence folder: test-comms/evidence/20260630-cleanroom-e2e-6847ef2

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

1. First-run Longmont source breadth:
   - After a normal Longmont first-run setup, starter sources must be seeded without manual import.
   - Source intake should include official records, official communications, local media, and public social/community sources when available.
   - Record the source count and representative source names.

2. Weak lead workflow:
   - When drafting a low-novelty, recurring, background, watch, verification, or otherwise non-ready lead, the app must mark it as needing more work or visibly warn the editor before approval.
   - The app must not veto the editor. It may warn and log the editor's decision.
   - If the tester intentionally approves a warned item to test the warning path, record the warning text and whether the approval required a conscious review checkpoint.

3. Prior quality failure:
   - Generic monitoring pages such as "new official document fetched" or "decision keywords identified" must not be counted as successful reader-facing stories.
   - If such items appear, they should be clearly labeled as watch/background/needs-work items and should either be held, cut, sent back, or approved only after a visible review checkpoint.

Full E2E test:

1. Install CivicNewspaper from the NSIS installer unless Windows blocks it completely. Use the MSI only if the NSIS installer cannot proceed.
2. Complete onboarding as a normal user for Longmont, Colorado.
3. Let the app perform any app-guided AI/runtime/model setup. The tester must not manually install Ollama or models.
4. Run Daily Scan for Longmont.
5. Confirm source intake includes official and public/social/community sources.
6. Confirm the Story Queue produces multiple leads.
7. Draft only leads with the normal Draft action first.
8. Use Draft anyway only when the UI gives an explicit reason and record that reason.
9. Confirm low-novelty, recurring, background, watch, verification, or non-ready leads are clearly labeled and not presented as ordinary ready stories.
10. Produce at least 5 reader-facing stories or briefs if the app can support it from the scan.
11. Exercise writer/editor workflow:
    - Generate draft.
    - Edit copy.
    - Save.
    - Put on hold.
    - Return/send back for more work if exposed.
    - Cut or remove one unsuitable item if exposed.
    - Approve publishable items.
12. Publish a static issue locally.
13. Export the publication ZIP.
14. Publish the same issue to here.now using the app's anonymous preview flow.
15. Save the here.now URL in the final report.

Output quality checks:

- No duplicate topic cluster like two stories about the same Building Services portal outage.
- No public leakage of EDITOR_NOTE, [EDITOR_NOTE, Body:, Headline:, Nut graf, Reporting Steps, [Source needed], [Verification needed], [End of Report], or similar reporter scaffolding.
- Public story pages must use real headlines, not lead-summary blobs.
- Stories should read as public copy or clearly labeled briefs/watch items, not raw reporter notes.
- Stories must have a current, specific reason to exist. Generic source-intake notes, search-result notes, static service descriptions, and keyword-detection notes are failures if presented as stories.
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
- Editorial workflow outcomes, including warning checkpoint evidence for weak leads.
- Which weak or generic items were held, cut, sent back, or approved after warning.
- Local static output path.
- ZIP output path.
- here.now URL.
- Output quality scan results.
- Screenshots or copied text evidence for any failure.
- If FAIL, state exactly where it broke and what the user could not do next.

After writing the final report, keep the 15-minute repo watcher armed and continue treating test-comms/ACTIVE_DIRECTIVE.md as the only active directive pointer.
