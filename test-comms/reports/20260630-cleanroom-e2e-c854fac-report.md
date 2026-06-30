# CivicNewspaper cleanroom E2E report - c854fac attempt 3

Date: 2026-06-30 UTC
Tester machine: Windows cleanroom tester, user `MSI\civic`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit: `c854fac6924fc1e584bf3eb9b136142fbddd4b13`
Directive: `test-comms/directives/20260630-cleanroom-e2e-c854fac-attempt3.md`

## Verdict

FAIL, with major improvements.

Attempt 3 successfully installed, launched, completed product-guided local AI setup, generated six clean drafts, compiled a six-article static site, produced `site-package.zip`, published to here.now, and passed the forbidden-marker scan across local output, ZIP extract, RSS/share artifacts, and fetched here.now pages.

It still does not fully pass the directive because:

1. First-run identity setup still did not receive input events and auto-continued with a starter Longmont profile.
2. The published issue has a cluster of overlapping City Council process/participation/voting stories rather than clearly distinct story topics.

here.now URL: https://ancient-hearth-p3sb.here.now

## Installer and cleanroom setup

- NSIS installer used: `test-comms/artifacts/20260630-cleanroom-e2e-c854fac/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256: `22B8BFA79655A65B2310196A262166AF018850FE61C5A6B671F24DE80DA0A105`
- NSIS size: `5621437`
- NSIS install exit code: `0`
- Fallback MSI verified: `test-comms/artifacts/20260630-cleanroom-e2e-c854fac/The Civic Desk_0.3.0_x64_en-US.msi`
- MSI SHA256: `2DE58AF34AFB51E6E09D4C21F623BB623B4C86F47CC5360FD1E10E027EC26788`
- MSI size: `9125888`

Clean wipe evidence: `tester-output/evidence/00-clean-wipe-summary.json`.

## First-run and AI setup

- First-run setup bug persists: `The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile.`
- The app then product-guided local AI setup without tester-installed dependencies.
- Model selected/downloaded: `qwen2.5:7b`.
- Download reached `100.0%`, verified model integrity, and the app reached `Local AI ready`.
- The app added 6 starter Longmont sources.

Evidence:

- `02-first-launch.png`
- `03-ai-setup-wait-summary.json`
- `04-model-download-progress-summary.json`
- `05-sources-after-ai-setup.png`

## Sources, scan, and leads

- Sources: 6 starter Longmont sources.
- Daily Scan result: 14 initial leads, then 15 visible after draft loop.
- High priority: 0.
- Best available lead mix: one `Ready to draft` lead, plus Watch/Verification leads as allowed by directive when fewer than six ready leads are available.

Evidence:

- `08-daily-scan-summary.json`
- `09-lead-card-inventory.json`

## Draft generation

Generated drafts: 6.

Clean before manual editing: 6.

Forbidden-marker check on generated draft bodies found no:

- `EDITOR_NOTE`
- `Editor Note:`
- `TESTER EDIT`
- `Nut graf:`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`
- `Body:`
- `[insert ...]`

Approved drafts:

1. `Longmont Museum Offers New Exhibits and Programs`
2. `City Council Voting Activity Indicated in Recent Records`
3. `Upcoming City Council Meeting Allows Public Participation`
4. `Longmont City Council Voting Process and Public Participation`
5. `City of Longmont Updates Public Information Page`
6. `City Council Provides Guidance on Engaging in Lawmaking Processes`

Evidence:

- `13-draft-loop-summary.json`
- `10-draft-*-wizard.png`
- `11-draft-*-generated.png`
- `12-draft-*-approved.png`

## Workflow exercise

Exercised:

- Draft generation.
- Workbench inspection before editing.
- Press-freedom/legal-risk advisor.
- Send back.
- Hold.
- Resume.
- Re-approve for static publish.
- Compile.
- ZIP export.
- here.now publish.

Advisor result: visible non-blocking warnings appeared; they did not block the editor.

Evidence:

- `14-drafts-tab-before-workflow-exercise.png`
- `15-workbench-open-for-workflow-exercise.png`
- `16-advisor-result.png`
- `17-workflow-send-back.png`
- `18-workflow-hold.png`
- `19-workflow-resume.png`
- `20-workflow-reapprove.png`
- `20-workflow-exercise-summary.json`

## Compile, ZIP, and here.now

- Local output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Copied output path: `test-comms/artifacts/20260630-cleanroom-e2e-c854fac/tester-output/site-output-copy/`
- ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- Copied ZIP path: `test-comms/artifacts/20260630-cleanroom-e2e-c854fac/tester-output/site-output-copy/site-package.zip`
- ZIP SHA256: `1ABD4D90151AB25C85A24E6798295CE13450221369BAC1BB133C36831D47CE4A`
- here.now URL: https://ancient-hearth-p3sb.here.now

Compile receipt:

- Articles: 6.
- Files: 23.
- Skipped: 0.

here.now fetch:

- `/`: HTTP 200.
- `/watch/1.html`: HTTP 200.
- `/watch/2.html`: HTTP 200.
- `/watch/3.html`: HTTP 200.
- `/watch/4.html`: HTTP 200.
- `/watch/5.html`: HTTP 200.
- `/watch/6.html`: HTTP 200.

Evidence:

- `25-compile-publish-summary.json`
- `26-output-scan-summary.json`
- `28-final-content-summary.json`

## Public output scan

Accurate marker scan result: clean.

Scanned:

- Local output copy.
- ZIP extract.
- RSS/share artifacts.
- Fetched here.now pages.

No findings for forbidden editorial scaffolding markers, Longmont footer boilerplate markers, or mojibake marker code points.

Evidence:

- `27-accurate-marker-scan.json`

## Remaining product concerns

The attempt-2 public-output marker failures appear fixed in this run.

Remaining blockers:

1. First-run identity input failure means a normal user still did not complete identity setup by typing; the app auto-continued.
2. The output contains several closely related City Council process/participation/voting pieces. These are not exact duplicate titles, but they are not clearly distinct enough to satisfy the no-duplicate-topic bar without product-side clustering or stronger story selection.

## Watcher status

The CivicNewspaper watcher remains armed for follow-up directives. CivicCast context was not used for this run.
