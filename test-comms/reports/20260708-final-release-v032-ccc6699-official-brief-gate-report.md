# TEST REPORT - Civic Desk v0.3.2 ccc6699 Official Brief Gate Rerun

## Result

**FAIL**

The ccc6699 cleanroom run passed the release-asset visibility checks, installed from the GitHub release asset, launched from a fresh isolated `CIVICNEWS_APP_DATA_DIR`, completed Longmont/CO onboarding, and ran Daily Scan without the prior city/state blocker. Weak and unsupported scan results were mostly handled as verification/background work, and `Draft anyway` was not present in Story Queue.

The release still cannot pass this directive because the fresh Longmont run did not produce any credible linked-evidence Story or Brief lead to draft. Story Queue showed 10 new leads, but they were verification/background/community-signal items; the only linked-evidence lead in SQLite was still `story_type: verification`, `disposition: needs_verification`. Workbench had no selected lead or draft, and no draft/export/publish/public-site checks could be completed.

## Environment

- Windows host/user: `msi\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Coordination commit at start: `f0509c0`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `ccc66997031d027e6187b4c1d5f95e117c7c8ac8`
- Release/docs commit under test: `e41c8cd5319dc954feb8f409766fd8ec36a65b8f`
- Clean profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-ccc6699-official-brief-gate`

## Steps Run

- Read the active directive.
- Downloaded the installer and checksum from the GitHub v0.3.2 release.
- Verified installer SHA256 `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6`.
- Verified installer size `5239817` bytes.
- Confirmed release/docs visibility in `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-visibility.md`.
- Uninstalled the prior app instance and installed from the downloaded GitHub release installer.
- Launched only `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` with `CIVICNEWS_APP_DATA_DIR` set to the fresh clean profile.
- Completed first-run onboarding for Longmont, Colorado.
- Confirmed `community_profile.json` persisted `city: Longmont` and `state: CO`.
- Ran Daily Scan without the prior `Choose your publication city and state...` blocker.
- Ran source discovery with both `Longmont` / `Colorado` and `Longmont` / `CO`.
- Inspected Story Queue and Workbench.
- Captured app-data file list and SQLite summary evidence.

## Findings

### Blocker 1 - No draftable linked-evidence Story or Brief lead existed

Directive steps 19-21 require finding a credible Longmont lead with linked evidence and Story or Brief treatment, confirming Brief default behavior, and drafting reader-facing copy from linked source evidence. This cleanroom run could not satisfy those steps.

Evidence:

- `sqlite-summary-final.json` shows `leads_count: 10`, `lead_evidence_count: 1`, `drafts_count: 0`, and `published_posts_count: 0`.
- The one linked-evidence lead was `story_type: verification` with `disposition: needs_verification`, not Story or Brief.
- Story Queue showed verification/background/community-signal cards such as `Review community signal from Longmont city events...` and `Verify source-quality issue...`.
- Workbench showed `No lead or draft selected` and `No drafts exist yet`.

Because no credible linked-evidence Story/Brief lead existed, the required draft, editor, export, here.now publish, and public visitor checks were not run.

## Passing / Improved Checks

- Visibility passed for the release page, docs page, installer SHA256, installer size, and checksum asset.
- First-run onboarding used the isolated profile and did not inherit old state.
- Onboarding persisted `Longmont` / `CO`; Daily Scan opened and ran without city/state repair in Settings.
- Source discovery accepted both full state name `Colorado` and abbreviation `CO`.
- Story Queue did not show `Draft anyway`.
- No-evidence/community/background leads were presented as verification/background work rather than normal draft-ready stories.
- Source-quality issue cards were rewritten as `Verify source-quality issue...` instead of leading directly with raw page chrome.

## Not Run

The following steps were not run because Blocker 1 prevented the required draftable linked-evidence Story/Brief workflow:

- Draft a credible Longmont story or brief from linked evidence.
- Confirm Brief/Story Article Format defaults in Workbench for a draftable lead.
- Confirm generated public copy quality.
- Exercise hold/send-back/approve/cut workflow.
- Compile/export the static site.
- Publish through here.now.
- Inspect the public here.now site as a visitor.

## Evidence

- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/visibility-receipt.json`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/state-paths-before-install.json`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/state-paths-after-install.json`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/install-lifecycle.txt`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/cleanprofile-launch.json`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/community_profile-final.json`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/continue-daily-after.txt`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/continue-story-queue.txt`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/workbench-after-lead-open-attempt.txt`
- `test-comms/reports/20260708-final-release-v032-ccc6699-official-brief-gate-evidence/sqlite-summary-final.json`
