# CivicNewspaper Cleanroom E2E Attempt 7 - Final Report

UTC report time: 2026-06-30T14:27:00Z

Verdict: FAIL.

Attempt 7 fixed the primary approval/navigation problem from attempt 6: warned drafts can be approved through the normal UI, the warning modal accepts an editor note, and the story transitions to `Approved for publishing`. Queue and Publishing navigation also recover without restarting the app. The run still fails the directive's full public-output quality bar because generated public output leaked reporter-note text (`Next steps:`) and mojibake marker `Â`, and the scan produced only 2 published articles rather than 5.

## Repo And Directive

- Coordination branch: `test-comms/cleanroom-coder-tester`
- Visibility commit pushed before run: `83e1274 test-comms: visibility for cleanroom e2e 7c1bbbd [skip ci]`
- Active directive: `test-comms/directives/20260630-cleanroom-e2e-7c1bbbd-attempt7.md`
- Product branch: `main`
- Product commit installed: `7c1bbbd42279c13adeb80d604b156a2e6df7eb81`
- Product version: `0.3.0`

## Installer And Cleanroom Setup

- Clean wipe evidence: `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/00-clean-wipe-summary.json`
- Installer used: `test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/The Civic Desk_0.3.0_x64-setup.exe`
- Installed executable: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- NSIS expected SHA256: `E45BD165A902AE711F950B3CA39EAA4E5BFBA30946F54A06E866504EB40B7C86`
- NSIS observed SHA256: `E45BD165A902AE711F950B3CA39EAA4E5BFBA30946F54A06E866504EB40B7C86`
- MSI fallback expected SHA256: `DBFC81BF4F4916A15D631940A0A484BD4A89AAEE3DA527DDBC2A7BFF87CAB18A`
- MSI fallback observed SHA256: `DBFC81BF4F4916A15D631940A0A484BD4A89AAEE3DA527DDBC2A7BFF87CAB18A`
- NSIS install result: PASS.
- MSI fallback used: no.

## App-Guided AI Setup

- App-guided runtime setup worked without tester manually installing Ollama or models.
- Model shown ready: `qwen2.5:7b`
- Shell identity after onboarding: `LONGMONT / CO`
- Identity input normalization: typed noisy state `CO94 TEST`; field normalized to `CO` before Next.
- AI setup result: PASS.

## First-Run Sources

Starter sources were seeded without manual import.

- Source count: 14.
- Official/primary/public sources included Longmont official city website, Longmont city news, Longmont Agenda Management Portal, Longmont City Council Meetings, Longmont Public Information, Public Notice Colorado, St. Vrain Valley Schools, Longmont city events, and Longmont city YouTube.
- Watch/community sources included City of Longmont Facebook, Longmont Public Safety Facebook, Longmont subreddit, and Longmont Colorado subreddit.

Source breadth result: PASS.

Evidence: `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/12-sources-seeded.json`

## Daily Scan And Queue

- Daily Scan completed.
- Lead count: 17.
- Source count in queue: 14.
- High priority count: 1.
- Draft count after workflow exercise: 3.
- Normal `Draft` actions observed for ready items.
- Weak/non-ready items remained labeled with `Draft anyway`, `Background`, `Editor review`, `Voting signal`, `Meeting scheduled`, or primary-record wording.

Queue labeling result: PASS.

## Approval And Workflow Regression

Primary attempt-7 approval regression: PASS.

A warned draft was moved through the normal UI:

- Generated draft.
- Saved draft.
- Put on hold.
- Resumed editing.
- Sent back for more work.
- Resumed editing.
- Marked ready for review.
- Clicked `Approve for Static Publish`.
- Warning modal appeared.
- Entered editor note in `#override-reason`.
- Clicked `Publish anyway (logged)`.
- App reported: `Story approved for publishing; a verification record was saved.`
- Workbench showed `Current Status: Approved for publishing`.
- Workbench showed `Unapprove`, confirming the editor decision was not stranded.

Warning modal text included:

- `Publish with review warnings?`
- `This story has 3 review warning(s) from your newsroom's guardrail and story-quality checks. The app will not veto the editor, but this decision is recorded with the story.`
- `[Citation Coverage] Paragraph may contain factual claims without a linked source.`

Second warned story:

- A second draft also approved through the warning modal with editor note.
- App again reported successful approval.
- Workbench showed `Current Status: Approved for publishing`.

## Navigation Recovery

Navigation recovery result: PASS.

After approval:

- `Back to Queue` returned to Story Queue.
- Sidebar navigation to Publishing worked.
- Sidebar navigation back to Story Queue worked.
- Publishing controls were reachable without restarting the app.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/25-back-to-queue-after-approval.json`
- `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/26-publishing-reachable-after-approval.json`
- `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/27-queue-reachable-after-publishing.json`

## Weak Lead Handling

Weak lead handling result: PASS.

The Vision Zero/background draft showed:

- Guardrail issue: `The linked lead is marked as background. Confirm a current, specific, verified development before treating this as a reader-facing news story.`
- The draft was not approved.
- It showed `Restore to Drafting`.
- `Approve for Static Publish` was disabled.

This preserves the attempt-6 gain that weak/background/watch leads are not silently counted as ordinary publishable copy.

## Static Output, ZIP, And here.now

- Local static output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Tester output copy: `test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/tester-output/`
- ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- ZIP SHA256: `239582CE90B91BB346CFFE5E5884EF7D02E04E196E03B4C09607F4BFE98F54B1`
- Issue id: `issue-20260630-142345-121201700`
- Published URL from manifest: `https://faint-lodge-c9h5.here.now`
- URL shown/saved in UI: `https://serene-reef-gydy.here.now`
- Deployment id from manifest: `slug=faint-lodge-c9h5;version=01KWCENEXT7TEMN4BP45PTGDDE;created_slug=faint-lodge-c9h5`
- Article count: 2
- Files written: 19
- here.now fetches returned HTTP 200 for both the UI URL and manifest URL.

Publish mechanics result: PASS, with a minor URL inconsistency to investigate.

## Public Output Quality Failure

Quality scan evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/quality-scan/quality-scan-results.json`
- `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/quality-scan/manifest-url-fetch-results.json`

Scanned:

- Local output folder
- ZIP extract
- here.now fetched pages

Failures:

1. Reporter-note text leaked into public output:
   - Marker: `Next steps:`
   - File: `watch/1.html`
   - Snippet from public HTML:
     - `Next steps:`
     - `Confirm the new meeting details (date, time, location) from the city's official website or a press release.`
   - This appears in local output, ZIP extract, and live fetched here.now page.

2. Mojibake marker remained:
   - Marker: `Â`
   - File: `watch/3.html`
   - This appears in local output and ZIP extract according to `quality-scan-results.json`.

3. Only 2 articles were published:
   - Directive requested at least 5 reader-facing stories or briefs if the app can support it from the scan.
   - The app produced and published 2 approved articles from this run.

4. Both published articles are tagged `format: watch` in the manifest:
   - `Longmont Public Library Launches Annual Summer Reading Challenge`
   - `City Council Meeting Rescheduled for July 1st`
   - These were drafted/approved as reader-facing briefs, so public `watch` taxonomy remains confusing.

Output quality result: FAIL.

## Exact User-Visible Result

The user can install, complete onboarding, use app-guided AI setup, run Daily Scan, approve warned drafts through the warning checkpoint, navigate back to Queue/Publishing, compile, export ZIP, and publish to here.now.

The user still cannot get a clean public issue that satisfies the directive's output bar because public HTML leaks reporter-note text and mojibake markers remain in generated output.

## Evidence Index

- Visibility report: `test-comms/reports/20260630-cleanroom-e2e-7c1bbbd-visibility-attempt-7.md`
- Clean wipe: `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/00-clean-wipe-summary.json`
- Install launch: `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/01-install-launch-summary.json`
- Identity/onboarding: `02-first-launch*`, `03-identity-noisy-state-before-next*`, `04-after-identity-next*`
- AI setup/model: `05-ai-setup-poll.json`, `06-ai-setup-ready-or-current*`, `07-download-button-click-timeout-state*`, `10-after-model-wait-timeout-current*`
- Sources: `12-sources-seeded*`
- Queue: `11-story-queue-after-daily-scan*`, `13-story-queue-returned*`, `37-queue-before-weak-target*`
- Approval workflow: `14-normal-lead16-draft-opened*` through `24-after-publish-anyway-logged*`
- Navigation recovery: `25-back-to-queue-after-approval*`, `26-publishing-reachable-after-approval*`, `27-queue-reachable-after-publishing*`
- Weak lead handling: `41-after-dom-click-weak-lead15*`
- Second approval: `34-second-normal-generated*`, `35-second-normal-existing-modal*`, `36-second-normal-approved*`
- Publishing: `42-publishing-before-compile*` through `48-here-now-publish-final-ui*`
- Tester output: `test-comms/artifacts/20260630-cleanroom-e2e-7c1bbbd/tester-output/`
- Quality scan: `test-comms/evidence/20260630-cleanroom-e2e-7c1bbbd/quality-scan/`

## Final Result

FAIL. Attempt 7 fixes the approval/navigation blocker and preserves weak-lead warnings, but the public-output gate still fails because generated public pages leak `Next steps:` reporter-note text and mojibake remains in article output.
