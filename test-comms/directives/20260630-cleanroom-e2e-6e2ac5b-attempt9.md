# CivicNewspaper Cleanroom E2E Attempt 9

Status: ACTIVE
Issued by: coder
Issued at: 2026-06-30T15:36:00Z

Single source of truth:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Coordination branch: test-comms/cleanroom-coder-tester
- Tester coordination path: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-only path on the tester machine: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms

Product under test:

- Product branch: main
- Product commit: 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad
- Product version: 0.3.0
- Fix under test: app-managed local AI runtime install now avoids stale or partial runtime folders and installs into a fresh versioned folder when needed.

Installer artifacts:

- Artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b
- NSIS installer: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64-setup.exe
- NSIS SHA256: 8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719
- NSIS size bytes: 5626730
- MSI installer: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64_en-US.msi
- MSI SHA256: AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A
- MSI size bytes: 9117696

Reports to write:

- Visibility report: test-comms/reports/20260630-cleanroom-e2e-6e2ac5b-visibility-attempt-9.md
- Final report: test-comms/reports/20260630-cleanroom-e2e-6e2ac5b-report.md
- Evidence folder: test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b
- Tester output artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/tester-output/

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

Primary regression to prove fixed:

1. App-guided local AI/runtime setup:
   - The app must complete local AI runtime setup without the tester manually installing Ollama or models.
   - If setup fails, capture exact UI text, save diagnostics if exposed, and include screenshots plus any app diagnostic output.
   - Confirm whether setup creates a runtime under the app data `ollama-runtime` folder and whether the app reaches a model download screen.

Carry-forward regressions to recheck if setup passes:

1. Public output cleanup:
   - Generated local HTML, ZIP extract, RSS/share artifacts, and here.now pages must not leak `Next steps:`, `Next step:`, `Verification steps:`, reporter scaffolding, `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]`.
   - Public output must not contain mojibake marker code points U+00C2, U+00C3, U+00E2, or U+FFFD.

2. Public article taxonomy:
   - Internal watch drafts that the editor approves as reader-facing output must publish as public `brief` items, not public `watch` items.
   - Manifest entries and visible story meta should show `brief` for those items.
   - Public page paths for such items should be under `briefs/`, not `watch/`.

3. First-run source breadth:
   - Longmont starter intake should seed 19 sources if all add operations are accepted.
   - Verify the two local media sources are present: Longmont Leader local news and Times-Call Longmont news.
   - Verify their tier is accepted as news reporting, not silently skipped.
   - Verify additional event/community sources are present: Longmont Area Chamber of Commerce, Visit Longmont events, and Downtown Longmont events.

4. Workbench approval/navigation:
   - A warned ready_to_review draft must approve through the normal UI, log the editor note, and transition to ready_to_publish.
   - Back to Queue and sidebar Publishing navigation must recover without restart.

Full E2E test:

1. Install CivicNewspaper from the NSIS installer unless Windows blocks it completely. Use the MSI only if the NSIS installer cannot proceed.
2. Complete onboarding as a normal user for Longmont, Colorado.
3. Let the app perform any app-guided AI/runtime/model setup. The tester must not manually install Ollama or models.
4. Run Daily Scan for Longmont.
5. Confirm source intake includes official, local media, public/social/community, and event sources.
6. Confirm the Story Queue produces multiple leads.
7. Draft ready items with the normal Draft action first.
8. Use Draft anyway only when the UI gives an explicit reason and record that reason.
9. Try to produce at least 5 reader-facing stories or briefs from the scan without padding with generic source-intake notes.
10. If fewer than 5 can be honestly approved, report exactly which leads were not publishable and why.
11. Exercise writer/editor workflow:
    - Generate draft.
    - Edit copy.
    - Save.
    - Put on hold.
    - Return/send back for more work if exposed.
    - Cut or remove one unsuitable item if exposed.
    - Approve publishable items, including at least one warned item through the warning checkpoint.
12. Publish a static issue locally.
13. Export the publication ZIP.
14. Publish the same issue to here.now using the app's anonymous preview flow.
15. Save the here.now URL in the final report.

Output quality checks:

- No duplicate topic cluster like two stories about the same Building Services portal outage.
- Public story pages must use real headlines, not lead-summary blobs.
- Stories should read as public copy or clearly labeled briefs, not raw reporter notes.
- Stories must have a current, specific reason to exist.
- Generic source-intake notes, search-result notes, static service descriptions, and keyword-detection notes are failures if presented as stories.
- ZIP extract, RSS/share artifacts, local HTML output, and here.now pages must all pass the same public-output scans.

Final report requirements:

- Plain-English verdict: PASS or FAIL.
- Exact install path used.
- Exact product commit installed.
- Installer hash verification results.
- Whether app-guided AI/runtime/model setup worked without tester manual installation.
- Whether identity displayed cleanly as Longmont / CO.
- Whether first-run starter sources were seeded without manual import, including local media count and total count.
- Daily Scan outcome.
- Lead count and story/brief count.
- Editorial workflow outcomes, especially the warned ready_to_review approval path.
- Final database statuses for approved, held, sent-back, and cut drafts.
- Which weak or generic items were held, cut, sent back, or approved after warning.
- Local static output path.
- ZIP output path.
- here.now URL.
- Manifest URL, UI URL, and whether they agree.
- Output quality scan results.
- Public taxonomy/path results for formerly internal watch-format drafts.
- Screenshots or copied text evidence for any failure.
- If FAIL, state exactly where it broke and what the user could not do next.

After writing the final report, keep the 15-minute repo watcher armed and continue treating test-comms/ACTIVE_DIRECTIVE.md as the only active directive pointer.
