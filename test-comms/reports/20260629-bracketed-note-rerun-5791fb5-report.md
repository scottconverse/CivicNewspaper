# CivicNewspaper cleanroom rerun report - bracketed note cleanup 5791fb5

Status: PASS

UTC run window: 2026-06-29T17:18Z to 2026-06-29T17:33Z

Coordination branch: `test-comms/cleanroom-coder-tester`

Directive: `test-comms/directives/20260629-bracketed-note-rerun-5791fb5.md`

Product commit under test: `5791fb5146d76fc5e97012488c995d0de1bb99c6`

Evidence folder: `test-comms/reports/20260629-bracketed-note-rerun-5791fb5-evidence/`

## Summary

The cleanroom rerun passed. The app installed from the new NSIS artifact, completed app-guided local AI setup, loaded Longmont starter sources, ran Daily Scan, generated and approved five drafts, exercised the advisor and hold control, compiled a five-article static issue, exported a ZIP, published to here.now, and passed the mandatory public-output marker/mojibake audit.

Live here.now URL:

`https://yearly-kernel-h752.here.now`

## Installer

Preferred NSIS installer:

`test-comms/artifacts/20260629-bracketed-note-rerun-5791fb5/The Civic Desk_0.2.9_x64-setup.exe`

Expected SHA256:

`9CF4714A253E32D04E1FB1394B6D583B37CCC77C21FDACEBE212D6F1BBDD117C`

Observed SHA256:

`9CF4714A253E32D04E1FB1394B6D583B37CCC77C21FDACEBE212D6F1BBDD117C`

Install result:

- Method: NSIS
- Exit code: 0
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App launched with WebView CDP available.

Evidence:

- `installer-hashes.json`
- `clean-wipe-log.json`
- `install-result.json`
- `launch-result.json`

## Setup and Sources

The app completed its own setup path without tester-installed prerequisites. The UI reported:

- `LONGMONT / CO`
- `Local AI ready`
- `qwen2.5:7b`

The app loaded six Longmont sources:

- Longmont Agenda Management Portal
- Longmont City Council Meetings
- Longmont Public Information
- Public Notice Colorado
- Longmont subreddit
- Longmont Colorado subreddit

Evidence:

- `01-ai-setup-*.png`
- `ai-setup-states.json`
- `db-state-final.json`

## Daily Scan

Daily Scan ran from the UI. The story queue showed 15 new leads, 6 sources, and 6 high-priority items. The final DB snapshot recorded 18 leads and 10 daily scan lead rows.

Evidence:

- `02-daily-scan-before.png`
- `03-daily-scan-*.png`
- `04-story-queue-after-scan.txt`
- `daily-scan-states.json`
- `db-state-final.json`

## Editorial Workflow

Five drafts were generated and approved from the Leads tab. The press-freedom/legal-risk advisor was run on the first draft. A sixth draft was generated and held successfully. Return/send-back still was not exposed in the held-draft state tested, so that remains recorded as unavailable rather than claimed.

Result counts from final DB snapshot:

- drafts: 6
- published posts: 5
- publish runs: 1

Evidence:

- `draft-editor-results.json`
- `hold-return-result.json`
- `13-advisor-result-1.png`
- `14-after-approve-*.png`
- `21-after-hold.png`
- `db-state-final.json`

## Publication

The issue compiled, exported ZIP, and published to here.now.

Generated output folder:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-bracketed-note-rerun-5791fb5-evidence\publication-output\site`

ZIP:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\reports\20260629-bracketed-note-rerun-5791fb5-evidence\publication-output\site\site-package.zip`

ZIP SHA256:

`5DF3260311FD1637787EA099037C637252394A254EAFB9AFD43D8B46814B3B91`

here.now URL:

`https://yearly-kernel-h752.here.now`

Evidence:

- `38-publishing-final.txt`
- `publish-ui-result.json`
- `zip-check.json`
- `zip-hash.json`
- `public-herenow-home.txt`
- `public-herenow-article-*.txt`
- `public-herenow-links.json`

## Output Quality Audit

Mandatory scans passed.

`output-quality-audit.json` results:

- local output article count: 5
- live here.now article count: 5
- local output marker/mojibake hits: 0
- live here.now marker/mojibake hits: 0

`zip-check.json` results:

- ZIP exists: true
- extracted file count: 21
- extracted ZIP marker/mojibake hits: 0

Checked forbidden scaffolding markers:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[End of Report]`
- `[Source needed]`
- `[Verification needed]`

Checked mojibake markers:

- U+00C2 / `Â`
- U+00C3 / `Ã`
- U+00E2 / `â`
- U+FFFD / replacement character

No reader-facing local, ZIP, or live here.now artifact contained those markers.

## Remaining Notes

The app still allows a human editor to approve drafts even when warnings are present, as required. In this run, the public output cleanup successfully removed the bracketed editor-note leakage that failed the prior `c01e32f` rerun.

The return/send-back control remained unavailable in the held-draft state tested. I recorded that as a workflow limitation, but it did not block this directive's pass/fail bar because approve, hold, compile, ZIP, here.now publish, and public-output cleanup all passed.

## Final Determination

PASS.

The `5791fb5` build meets the directive's pass criteria for install, app-guided setup, Longmont source scan, editorial workflow, ZIP export, here.now publish, and public-output cleanup of internal scaffolding and mojibake markers.
