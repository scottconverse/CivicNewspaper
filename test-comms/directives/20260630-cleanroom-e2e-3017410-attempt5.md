# CivicNewspaper Cleanroom E2E Attempt 5

Status: ACTIVE
Issued by: coder
Issued at: 2026-06-30T12:31:00Z

Single source of truth:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Coordination branch: test-comms/cleanroom-coder-tester
- Tester coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-only path on the tester machine: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product under test:

- Product branch: main
- Product commit: 301741042b1a392885ac2de458cc8985a3084bac
- Product version: 0.3.0

Installer artifacts:

- Artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-3017410
- NSIS installer: test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64-setup.exe
- NSIS SHA256: 0C79098D0B8720978E7AE056430B2DB7F3247D0072574DE05EC5F5AA9737D35C
- NSIS size bytes: 5622123
- MSI installer: test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64_en-US.msi
- MSI SHA256: 2F601F00402ACDA01ECA29597A5866526678F9855F6FB6F5A9DBAD8E2C6D6135
- MSI size bytes: 9125888

Reports to write:

- Visibility report: test-comms/reports/20260630-cleanroom-e2e-3017410-visibility-attempt-5.md
- Final report: test-comms/reports/20260630-cleanroom-e2e-3017410-report.md
- Evidence folder: test-comms/evidence/20260630-cleanroom-e2e-3017410

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
- If the app installer or first-run flow cannot set up a required dependency, report that as product failure. Do not repair it manually unless the directive explicitly says to.

Primary regressions to prove fixed:

1. Identity state field corruption:
   - During onboarding enter Longmont as city and deliberately paste or type noisy state text like CO94 TES into the state field.
   - The UI must normalize this to CO before save.
   - After setup, the main shell must display Longmont / CO or equivalent clean city/state text.
   - It must not display CO94 TES.
   - Daily Scan must not fail with Invalid city or state format.

2. First-run starter sources:
   - After a normal Longmont first-run setup, the app should guide the user to Daily Scan with starter Longmont sources already seeded.
   - Do not use manual Discover/import unless the app clearly fails to provide usable starter sources.
   - If manual source discovery is needed, mark that as a failure and continue the test only to gather additional evidence.

Full E2E test:

1. Install CivicNewspaper from the NSIS installer unless Windows blocks it completely. Use the MSI only if the NSIS installer cannot proceed.
2. Complete onboarding as a normal user for Longmont, Colorado.
3. Let the app perform any app-guided AI/runtime/model setup. The tester must not manually install Ollama or models.
4. Run Daily Scan for Longmont.
5. Confirm source intake includes official and public/social/community sources when available.
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
- Editorial workflow outcomes.
- Local static output path.
- ZIP output path.
- here.now URL.
- Output quality scan results.
- Screenshots or copied text evidence for any failure.
- If FAIL, state exactly where it broke and what the user could not do next.

After writing the final report, keep the 15-minute repo watcher armed and continue treating test-comms/ACTIVE_DIRECTIVE.md as the only active directive pointer.
