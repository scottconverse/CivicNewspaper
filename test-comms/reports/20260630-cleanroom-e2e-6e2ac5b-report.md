# CivicNewspaper Cleanroom E2E Attempt 9 Report

Verdict: FAIL

Reason for FAIL: the main runtime/setup, source breadth, workflow, taxonomy, and public-output checks passed, but the Civic Desk app process exited during/just after the here.now publish action. The publish completed and the live site is reachable, but the app did not remain available for a final in-app confirmation screen.

UTC report time: 2026-06-30T16:04:00Z

Directive: test-comms/directives/20260630-cleanroom-e2e-6e2ac5b-attempt9.md

Coordination branch: test-comms/cleanroom-coder-tester

Product commit installed: 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad

Product version: 0.3.0

## Installer And Setup

Installer used: NSIS

Installer path:

`C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms\test-comms\artifacts\20260630-cleanroom-e2e-6e2ac5b\The Civic Desk_0.3.0_x64-setup.exe`

Installed app path:

`C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

Installer hash verification:

- NSIS expected SHA256: `8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719`
- NSIS actual SHA256: `8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719`
- NSIS expected size: `5626730`
- NSIS actual size: `5626730`
- MSI expected SHA256: `AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A`
- MSI actual SHA256: `AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A`
- MSI expected size: `9117696`
- MSI actual size: `9117696`

Cleanroom reset:

- Initial uninstall ran.
- First delete pass found a locked app-managed runtime DLL from a prior `llama-server.exe`.
- After stopping app-owned runtime processes, retry removed the remaining `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk` tree.
- Final retry showed no remaining CivicNewspaper app/runtime/model paths in the cleanroom reset boundary.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/00-clean-wipe-summary.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/00-clean-wipe-retry-summary.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/01-install-launch-summary.json`

## App-Guided AI Runtime And Model Setup

Result: PASS

The app completed local AI runtime setup without tester manual installation.

- The app created `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11`.
- The app reached `The AI service is ready. Download a recommended model?`
- The app installed the recommended `qwen2.5:7b` model through the first-run flow.
- The app shell then showed `Local AI ready` and `qwen2.5:7b`.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/08-runtime-install-current.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/08a-runtime-folder-state.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/11-model-download-poll.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/14-after-onboarding.json`

## Identity And Sources

Result: PASS

- Identity displayed cleanly as `LONGMONT / CO`.
- First-run starter sources were seeded without manual import.
- Source count: 19.
- Local media sources present:
  - Longmont Leader local news
  - Times-Call Longmont news
- Event/community sources present:
  - Longmont Area Chamber of Commerce
  - Visit Longmont events
  - Downtown Longmont events

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/15-sources-seeded.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/15a-source-summary.json`

## Daily Scan

Result: PASS

- Sources watched: 19.
- Open leads after scan: 19.
- Daily Scan saved 5 leads.
- AI status: Ready.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/20-daily-scan-settled.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/21-story-queue-after-scan.json`

## Editorial Workflow

Result: PASS

Approved stories:

- `Longmont City Council Meeting Rescheduled for July 1`
- `Longmont Youth Center Lists Summer Programs`

Workflow exercised:

- Generated a normal ready-to-draft brief.
- Edited copy.
- Saved draft.
- Put draft on hold.
- Sent draft back for more work.
- Marked draft ready for review.
- Approved through warning checkpoint with editor note.
- Back to Queue and sidebar Publishing navigation both recovered without restart.
- Generated a formerly watch-classified item, received watch/readiness warnings, edited it as a cautious public brief, and approved through the warning checkpoint.
- Generated generic/background/source-intake drafts only to test guardrails and cut/removal, not to pad the publication.

Database statuses:

- Draft 1: `Longmont City Council Meeting Rescheduled for July 1`, status `ready_to_publish`.
- Draft 2: `Longmont Youth Center Lists Summer Programs`, status `ready_to_publish`.
- Draft 3: `City of Longmont Updates Public Information Resources`, status `killed`.
- Draft 4: `Longmont Public Library Hosts Open Chess Night`, status `hold`.
- Draft 5: `Longmont Public Library Hosts Weekly Spanish-English Conversation Group`, status `killed`.

Fewer than 5 publishable stories were honestly approved because the remaining scan results were background, duplicate, verification-only, meeting-scheduled, keyword-detection, or generic source-intake notes. The app correctly labeled them `Draft anyway`, `Background`, `Watch`, or `Needs verification`; I did not pad the issue with them.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/31-first-draft-approval-warning-modal.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/32-first-draft-approved.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/34-publishing-reachable-after-approval.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/35-queue-reachable-after-publishing.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/42-summer-approval-warning-modal.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/43-summer-approved.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/48-generic-cut-confirmed.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/56-unexpected-spanish-cut-confirmed.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/65-database-status-snapshot.json`

## Publish And Output

Compile result: PASS

- Static site compiled: 2 article(s), 19 file(s), ZIP package ready.
- Local static output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- ZIP output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- ZIP SHA256: `59C0D4B55B1809E75432EE6DBB3E7667DDDDAA553B662C96E133323B8BF9A537`
- Tester output copy: `test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/tester-output/`

here.now result: PARTIAL PASS

- Connector test passed.
- Manifest URL: `https://bright-sphinx-ywpd.here.now`
- Deployment ID: `slug=bright-sphinx-ywpd;version=01KWCKXT64JTGTQY4V8CXMD49N;created_slug=bright-sphinx-ywpd`
- Live HTTP checks: 200 for `/`, `/briefs/1.html`, `/briefs/2.html`, and `/feed.xml`.
- UI URL: not captured after publish because the app/WebView target closed during or immediately after the publish action.

Public taxonomy/path result: PASS

- Manifest article 1: `Longmont Youth Center Lists Summer Programs`, format `brief`, path `briefs/2.html`.
- Manifest article 2: `Longmont City Council Meeting Rescheduled for July 1`, format `brief`, path `briefs/1.html`.
- The formerly internal `watch` drafts were published as public `brief` items under `briefs/`, not public `watch` pages.

Output quality scan result: PASS

- No duplicate topic cluster in the 2-article issue.
- No `Next steps:`, `Next step:`, `Verification steps:`, `EDITOR_NOTE`, `[EDITOR_NOTE`, `Body:`, `Headline:`, `Nut graf`, `Reporting Steps`, `[Source needed]`, `[Verification needed]`, or `[End of Report]` found in local output, ZIP extract, RSS/share artifacts, or live pages.
- No U+00C2, U+00C3, U+00E2, or U+FFFD marker hits in the corrected ordinal scan.
- Article pages use public headlines and brief copy, not lead-summary blobs or raw reporter notes.

Evidence:

- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/59-after-compile.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/62-after-test-connection-2.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/quality-scan/quality-scan-results.json`
- `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/quality-scan/ordinal-marker-scan-results.json`
- `test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/tester-output/publish-manifest.json`

## Failure Detail

Exact failure: after clicking `Publish with connector`, the browser-control target closed and the Civic Desk process was no longer running. The local manifest showed the publish succeeded and the live here.now site was reachable, but the app did not remain open for the user to review a final in-app publish confirmation.

What the user could not do next: continue in the same running app session immediately after publish. The published issue itself was available at the manifest URL.
