# TEST REPORT - Civic Desk v0.3.2 1e03894 Source-Backed Brief Rerun

## Result

**FAIL**

The 1e03894 cleanroom rerun passed the release visibility checks, downloaded the installer and checksum from the GitHub v0.3.2 release, verified installer SHA256 `E7B620C4D51837DDD43028B511E396643EE9A67D1CD23DC0B59BC5442277DCD7`, installed the GitHub release asset, launched the installed app from a fresh isolated `CIVICNEWS_APP_DATA_DIR`, completed Longmont/CO onboarding, and ran Daily Scan without the prior city/state blocker.

The release still cannot pass this directive because the fresh Longmont run did not produce any source-backed Story or Brief lead. SQLite shows `lead_evidence_count: 0`, `drafts_count: 0`, and all five Daily Scan leads were `story_type: verification`, `disposition: needs_verification`, `priority/risk: low`. Because no credible linked-evidence Story/Brief lead existed, the required Brief default, drafting, editor, export, here.now publish, and public visitor checks could not be completed.

## Environment

- Windows host/user: `msi\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Coordination HEAD at run: `a0c4a86`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `1e03894496f7800b072586c30aac1e1b9afe4533`
- Release/docs commit under test: `c38c0f84a4eeaef39c4cd30d7a8c34f9903eb3af`
- Clean profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-1e03894-source-backed-brief`

## Steps Run

- Read the active directive from `test-comms/ACTIVE_DIRECTIVE.md`.
- Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release.
- Verified installer size `5241214` bytes and SHA256 `E7B620C4D51837DDD43028B511E396643EE9A67D1CD23DC0B59BC5442277DCD7`.
- Confirmed release page and public docs visibility in `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-visibility.md`.
- Uninstalled the prior app instance and installed from the downloaded GitHub release installer.
- Launched only `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` with `CIVICNEWS_APP_DATA_DIR` set to the fresh clean profile.
- Verified first-run onboarding and completed setup for Longmont, Colorado.
- Captured `community_profile-after-onboarding.json`, showing `city: Longmont` and `state: CO`.
- Ran Daily Scan without manually repairing city/state in Settings.
- Captured Daily Scan UI screenshots/text and SQLite summary evidence.

## Findings

### Blocker 1 - No linked-evidence Story or Brief lead existed

Directive steps 19-21 require finding at least one credible Longmont lead with linked evidence and Story or Brief treatment, confirming Brief default behavior, and drafting reader-facing copy from linked source evidence. This run could not satisfy those steps.

Evidence:

- `sqlite-summary-final.json` shows `daily_scan_leads_count: 5`, `leads_count: 5`, `lead_evidence_count: 0`, `drafts_count: 0`, and `published_posts_count: 0`.
- Every stored lead has `story_type: verification`, `disposition: needs_verification`, `risk_level: low`, and `confidence: low`.
- Daily Scan UI evidence shows five saved leads, all verification/community/source-quality items.
- Daily Scan lead publishability notes repeatedly state that no source documents could be linked and that public source material must be attached or verified before drafting.

Because there was no linked-evidence Story/Brief lead, Workbench Brief-default validation, draft generation, copy-quality review, editor workflow, export, here.now publication, and public site inspection were not run.

### Major 1 - Source-backed Brief promotion did not manifest in the cleanroom run

The directive exists to validate the 1e03894 fix for source-backed Daily Scan Brief promotion. The run produced no `lead_evidence` links and no Story/Brief leads, so the central fix could not be validated as passing.

## Passing / Improved Checks

- Release visibility passed for GitHub release page, public docs page, installer hash, installer size, checksum asset, unsigned installer guidance, and absence of the stale ccc6699 hash.
- Installer lifecycle was exercised from the downloaded GitHub release asset.
- First-run onboarding used the isolated profile and persisted Longmont/CO.
- Daily Scan opened and ran without the prior `Choose your publication city and state in Settings before running Daily Scan.` blocker.
- Weak/no-source leads remained low-priority verification work instead of ready-to-draft stories.
- Source-quality issue cards were presented as `Verify source-quality issue...` rather than raw page navigation as the lead title.
- `Draft anyway` was not observed in the captured Daily Scan evidence.
- Source discovery using both `Longmont` / `Colorado` and `Longmont` / `CO` produced reviewed candidate sources and did not return an empty no-guidance result.

## Not Completed

The following directive steps were not completed because Blocker 1 prevented the required source-backed Story/Brief drafting path:

- Confirm Workbench Article Format defaults to Brief for a source-backed Story/Brief lead.
- Draft a credible Longmont story or brief from linked evidence.
- Confirm generated public copy quality.
- Exercise hold/send-back/approve/cut workflow.
- Compile/export the static site.
- Publish through here.now.
- Inspect the public here.now site as a visitor.

Source discovery using both full state name `Colorado` and abbreviation `CO` was captured. The `CO` discovery result returned candidate sources but had `Selected: 0 sources` in the captured modal state, so no additional source import was completed after the initial watched-source set. The clean profile had 9 watched sources, and Daily Scan used those sources.

## Evidence

- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-visibility.md`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/visibility-receipt.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/release-page.html`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/public-docs.html`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/release-api.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/SHA256SUMS.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/The.Civic.Desk_0.3.2_x64-setup.exe`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/state-paths-before-install.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/state-paths-after-install.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/install-lifecycle.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/community_profile-after-onboarding.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-12-daily-before-run.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-13-daily-run-18.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-16-discover-colorado-filled.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-23-discover-co-results.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/sqlite-summary-final.json`

## Notes

- An additional final CDP capture attempted after SQLite readback did not complete because the bundled Node runtime could not resolve `playwright-core`; existing captured Daily Scan evidence is still present under the evidence folder.
- No product code was changed, merged, tagged, or published.
