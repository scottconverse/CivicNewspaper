# CivicNewspaper cleanroom E2E attempt 3 - c854fac

You are tester. Stop any old CivicCast context. The only coordination source for this run is:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Branch: test-comms/cleanroom-coder-tester
- Local coordination path on tester: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms
- Product branch: main
- Product commit: c854fac6924fc1e584bf3eb9b136142fbddd4b13

## Active run files

- Directive path: test-comms/directives/20260630-cleanroom-e2e-c854fac-attempt3.md
- Visibility report path: test-comms/reports/20260630-cleanroom-e2e-c854fac-visibility-attempt-3.md
- Final report path: test-comms/reports/20260630-cleanroom-e2e-c854fac-report.md
- Tester output artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-c854fac/tester-output/

## Install artifacts

Use the NSIS installer first:

- test-comms/artifacts/20260630-cleanroom-e2e-c854fac/The Civic Desk_0.3.0_x64-setup.exe
- SHA256: 22B8BFA79655A65B2310196A262166AF018850FE61C5A6B671F24DE80DA0A105
- Size: 5621437

Fallback MSI:

- test-comms/artifacts/20260630-cleanroom-e2e-c854fac/The Civic Desk_0.3.0_x64_en-US.msi
- SHA256: 2DE58AF34AFB51E6E09D4C21F623BB623B4C86F47CC5360FD1E10E027EC26788
- Size: 9125888

## Visibility check

Before running the product test, write and push the visibility report at the path above. It must confirm:

- Current Windows user and hostname.
- Local coordination path is C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms.
- You fetched and checked out branch test-comms/cleanroom-coder-tester.
- ACTIVE_DIRECTIVE.md points to this directive.
- Both installer files are present and their SHA256 hashes match.
- The 15-minute watcher remains armed.

## Cleanroom reset boundary

Perform a product clean wipe only. Remove CivicNewspaper app data, prior CivicNewspaper installs, test output, prior cleanroom artifacts, Ollama/model state created for these tests if the app installed it, and related test files. Do not reset Windows. Do not manually install Ollama, models, runtimes, or missing product dependencies. If the product cannot install or drive its own prerequisite setup, report that as product failure.

## Test objective

Retest the exact attempt-2 failure class after product commit c854fac:

1. Clean install from the NSIS artifact.
2. Complete first run for Longmont, Colorado.
3. Let the app handle local AI setup and model selection.
4. Import/discover official and public readable social/community sources.
5. Run Daily Scan.
6. Generate at least six drafts from the best non-background leads. Prefer leads labeled Ready to draft. If fewer than six are ready, include Watch or Verification leads and record why.
7. For each generated draft, inspect the Workbench body before editing.
8. Verify generated drafts do not contain these public/editorial scaffolding markers before manual editing:
   - EDITOR_NOTE
   - Editor Note:
   - TESTER EDIT
   - Nut graf:
   - Reporting Steps
   - [Source needed]
   - [Verification needed]
   - [End of Report]
   - Body:
   - [insert ...]
9. Approve only drafts that are clean enough to be reader-facing with ordinary editor judgment. Do not approve polluted scaffolding to force a pass.
10. Exercise send-back, hold, resume, advisor, approve for static publish, compile, ZIP export, here.now anonymous publish, and local output review.
11. Publish to here.now only if at least one clean story or brief is approved. If none can be approved, still compile and report the zero-article failure with evidence.
12. Copy the full output folder and ZIP into tester-output, including site-package.zip.
13. Scan copied output, ZIP extract, RSS, newsletter, share package, subreddit, Substack, and here.now pages for the forbidden markers listed above, Longmont footer boilerplate, and mojibake marker code points U+00C2, U+00C3, U+00E2, U+FFFD.

## Pass bar

Pass only if:

- Installer and first-run setup work from a product-clean machine.
- App-guided local AI setup reaches ready state without tester-installed dependencies.
- Daily Scan produces reviewable leads.
- Draft generation produces at least one clean reader-facing story or brief without internal scaffolding markers.
- Editor workflow can send back, hold, resume, advise, approve, compile, export ZIP, and publish to here.now.
- Published here.now page loads HTTP 200.
- Local output and copied ZIP contain the same approved content.
- Marker scan is clean across public output, copied ZIP extract, RSS/share artifacts, and here.now pages.

Fail with exact break point if any requirement fails.

## Final report requirements

Write the final report in plain English at:

test-comms/reports/20260630-cleanroom-e2e-c854fac-report.md

Include:

- PASS or FAIL verdict.
- here.now URL, if published.
- Local output path on tester.
- Copied output path under test-comms/artifacts/20260630-cleanroom-e2e-c854fac/tester-output/.
- ZIP path and SHA256 if produced.
- Number of leads, generated drafts, clean approved stories, published stories.
- Which drafts were clean before manual editing.
- Which drafts still contained forbidden markers, if any.
- Evidence file list and screenshots.
- Whether the watcher remains armed.

Push the report and artifacts with [skip ci].
