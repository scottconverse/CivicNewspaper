# CivicNewspaper Cleanroom E2E Attempt 5 - 3017410

UTC report time: 2026-06-30T13:09:00Z

Verdict: FAIL, with major mechanical improvements.

This report supersedes the earlier attempt-5 result in the same file. Additional local evidence showed the run continued past the earlier editor-workflow break and reached compile, ZIP export, and here.now publish. The corrected verdict is still FAIL because public-output quality did not meet the directive pass bar.

## Product Under Test

- Product commit installed: `301741042b1a392885ac2de458cc8985a3084bac`
- Product version: `0.3.0`
- Installer used: NSIS
- Install path used: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS path: `test-comms/artifacts/20260630-cleanroom-e2e-3017410/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256 observed: `0C79098D0B8720978E7AE056430B2DB7F3247D0072574DE05EC5F5AA9737D35C`
- NSIS SHA256 matched directive: yes

## What Passed

- Clean product wipe completed.
- NSIS install completed.
- First-run identity state normalization worked: I typed/pasted noisy `CO94 TES`, and the field normalized to `CO`.
- Main shell displayed `LONGMONT / CO`.
- Daily Scan did not fail with `Invalid city or state format`.
- App-guided AI/runtime/model setup worked without tester-installed dependencies.
- The app reached `Local AI ready` with `qwen2.5:7b`.
- Starter Longmont sources were seeded without manual import.
- Daily Scan produced 16 leads.
- One normal `Draft` lead was present.
- The UI labeled most lower-confidence/non-ready items as `Draft anyway`, `Background`, `Watch`, `Editor review`, `Seen before`, or similar.
- The editor workflow controls were exercised: save, advisor, hold, ready/resume, send back, mark ready, and approve.
- Static compile succeeded.
- ZIP export succeeded.
- here.now publish succeeded.
- Live here.now pages loaded HTTP 200.
- Public-output marker scan was clean across local output, ZIP extract, share/RSS artifacts, and fetched here.now pages.

## Published Output

- here.now URL: `https://subtle-pepper-nq3a.here.now`
- Local static output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Copied output path: `test-comms/evidence/20260630-cleanroom-e2e-3017410/site-output-copy`
- ZIP path: `test-comms/evidence/20260630-cleanroom-e2e-3017410/site-output-copy/site-package.zip`
- ZIP SHA256: `A95F2DD771B54122A171CB4C461F691BF57FEA2B70016EC879F071B0A5EF8FC9`
- Publish manifest issue id: `issue-20260630-130606-451237600`
- Article count: 5
- Files written: 22

Published article paths:

- `watch/1.html` - `City Council to Vote on Roof Contract for Longmont Library`
- `watch/2.html` - `New Official Document Fetched from Public Notice Colorado`
- `watch/3.html` - `City to Close Hover Street/CO 119 Intersection for Roof Work`
- `watch/4.html` - `City Council Decision Keywords Identified`
- `watch/5.html` - `City of Longmont Updates Public Information Resources`

## Why Verdict Is Still FAIL

The run fails the directive's public-output quality bar.

The app can now reach publication, but the published issue is not reliably reader-facing civic copy:

- Several approved items were watch/verification/background-style items, not clearly publishable stories.
- `watch/3.html` says the city "will temporarily shut down the intersection of Hover Street and CO 119 from June 28" even though this test ran on June 30, making the public copy stale or temporally wrong.
- `watch/4.html` is essentially a keyword-detection note: it says decision words were found but does not identify the specific vote, date, or impact.
- `watch/2.html` is a generic Public Notice Colorado monitoring item rather than a concrete local civic story.
- The issue still contains multiple council/process/background items, so distinct story quality remains weak.
- The workflow allowed approval/publishing of items that the UI itself warned were watch/background/needs verification.

This is a quality failure, not a mechanics failure.

## Counts

- Daily Scan leads: 16.
- Normal Draft leads: 1.
- Draft anyway leads: 15.
- Generated drafts: at least 5 total by the end of the run.
- Published stories/briefs/watch items: 5.
- Public-output marker findings: 0.
- Live pages fetched: 6 of 6 returned HTTP 200.

## Output Quality Scan

Scan file:

`test-comms/evidence/20260630-cleanroom-e2e-3017410/53-public-output-marker-scan.json`

Scanned 44 files across:

- copied local output
- ZIP extract
- RSS/share/newsletter/subreddit/Substack artifacts
- fetched here.now pages

Forbidden marker findings: none.

Markers checked included:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Editor Note:`
- `TESTER EDIT`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`
- `Body:`
- `Headline:`
- `[insert`
- `Employee Login`
- mojibake markers U+00C2, U+00C3, U+00E2, U+FFFD

## Key Evidence

Evidence folder:

`test-comms/evidence/20260630-cleanroom-e2e-3017410/`

Key files:

- `00-clean-wipe-summary.json`
- `01-install-launch-summary.json`
- `03-identity-noisy-state-before-next.json`
- `12-model-download-summary.json`
- `15-daily-scan-summary.json`
- `17-lead-inventory-text.json`
- `26-normal-draft-field-values.json`
- `41-workflow-exercise-summary.json`
- `45-index-approve-summary.json`
- `46-publishing-before-compile.json`
- `48-after-compile.png`
- `49-after-test-connection.png`
- `50-current-publish-state.json`
- `51-output-copy-summary.json`
- `52-live-fetch-summary.json`
- `53-public-output-marker-scan.json`
- `site-output-copy/`
- `site-package-extract/`
- `live-subtle-pepper-nq3a.here.now*.html`

## Watcher

The watcher remains armed and `test-comms/ACTIVE_DIRECTIVE.md` remains the active directive pointer.
