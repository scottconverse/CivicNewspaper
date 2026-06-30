# CivicNewspaper cleanroom E2E attempt 4 - a094ce1

You are tester. Stop any old CivicCast context. The only coordination source for this run is:

- Repo: https://github.com/scottconverse/CivicNewspaper
- Branch: test-comms/cleanroom-coder-tester
- Local coordination path on tester: C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms
- Do not use this coder-machine path on tester: C:\Users\instynct\Desktop\CODE\civicnewspaper-test-comms
- Product branch: main
- Product commit: a094ce12c8aca503a75c76a3d89b25b204a2d4cc

## Active run files

- Directive path: test-comms/directives/20260630-cleanroom-e2e-a094ce1-attempt4.md
- Visibility report path: test-comms/reports/20260630-cleanroom-e2e-a094ce1-visibility-attempt-4.md
- Final report path: test-comms/reports/20260630-cleanroom-e2e-a094ce1-report.md
- Tester output artifact folder: test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/

## Install artifacts

Use the NSIS installer first:

- test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/The Civic Desk_0.3.0_x64-setup.exe
- SHA256: AC8610ECDCA97674377309AA4A9F3AC826E275AF43137F799384F57E4DB9CA53
- Size: 5622087

Fallback MSI:

- test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/The Civic Desk_0.3.0_x64_en-US.msi
- SHA256: 2DAACE231273930951C506C335ED139F6A9E37FDB1D23B8835068BFD2A20E766
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

Retest the attempt-3 failure class after product commit a094ce1:

1. Clean install from the NSIS artifact.
2. Complete first run for Longmont, Colorado.
3. On the Identity step, type distinct test values into the visible fields. Do not rely on a starter-profile auto-continue. Record whether typed values remain visible before clicking Next.
4. Let the app handle local AI setup and model selection. Do not manually install Ollama or models.
5. Import/discover official and public readable social/community sources if the app does not add enough sources automatically.
6. Run Daily Scan.
7. Capture the lead-card inventory, including badges and button labels.
8. Generate drafts from the best leads labeled Draft first. Treat Draft anyway leads as cautionary overrides. Use Draft anyway only when there are not enough normal Draft leads, and record why each override was necessary.
9. Avoid approving a cluster of near-duplicate topics. At most one council-process, meeting-access, public-participation, or archive-background item may be approved unless the story has a specific new vote, deadline, dollar amount, outage, filing, meeting item, or public impact.
10. For each generated draft, inspect the Workbench body before editing.
11. Verify generated drafts do not contain these public/editorial scaffolding markers before manual editing:
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
12. Approve only drafts that are clean enough to be reader-facing with ordinary editor judgment. Do not approve polluted scaffolding or duplicate-topic filler to force a pass.
13. Exercise send-back, hold, resume, advisor, approve for static publish, compile, ZIP export, here.now anonymous publish, and local output review.
14. Publish to here.now only if at least one clean story or brief is approved. If none can be approved, still compile and report the zero-article or low-article failure with evidence.
15. Copy the full output folder and ZIP into tester-output, including site-package.zip.
16. Scan copied output, ZIP extract, RSS, newsletter, share package, subreddit, Substack, and here.now pages for the forbidden markers listed above, Longmont footer boilerplate, and mojibake marker code points U+00C2, U+00C3, U+00E2, U+FFFD.

## Pass bar

Pass only if:

- Installer and first-run setup work from a product-clean machine.
- Typed first-run identity values are captured without the old auto-continue behavior.
- App-guided local AI setup reaches ready state without tester-installed dependencies.
- Daily Scan produces reviewable leads with honest Draft versus Draft anyway labeling.
- Draft generation produces clean reader-facing stories or briefs without internal scaffolding markers.
- The approved issue contains distinct story topics, not a cluster of near-duplicate council-process or evergreen background items.
- Editor workflow can send back, hold, resume, advise, approve, compile, export ZIP, and publish to here.now.
- Published here.now page loads HTTP 200.
- Local output and copied ZIP contain the same approved content.
- Marker scan is clean across public output, copied ZIP extract, RSS/share artifacts, and here.now pages.

Fail with the exact break point if any requirement fails.

## Final report requirements

Write the final report in plain English at:

test-comms/reports/20260630-cleanroom-e2e-a094ce1-report.md

Include:

- PASS or FAIL verdict.
- Whether typed identity values were accepted and persisted through first-run setup.
- here.now URL, if published.
- Local output path on tester.
- Copied output path under test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/.
- ZIP path and SHA256 if produced.
- Number of leads, generated drafts, clean approved stories, published stories.
- Which leads were plain Draft versus Draft anyway.
- Which Draft anyway leads were used and why.
- Whether duplicate-topic clustering passed or failed.
- Which drafts were clean before manual editing.
- Which drafts still contained forbidden markers, if any.
- Evidence file list and screenshots.
- Whether the watcher remains armed.

Push the report and artifacts with [skip ci].
