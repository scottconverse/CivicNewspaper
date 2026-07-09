# TEST REPORT - Civic Desk v0.3.2 1e03894 Source-Backed Brief Rerun

## Result

**FAIL**

The 1e03894 cleanroom run passed release visibility, installer integrity, installed-app launch, isolated first-run onboarding, Longmont/CO profile persistence, starter source creation, Daily Scan reachability without Settings repair, and source discovery with both `Longmont` / `Colorado` and `Longmont` / `CO`.

The release still cannot pass the directive because the fresh Longmont run did not produce any credible linked-evidence Story or Brief lead to draft. Story Queue showed 5 leads after Daily Scan, but every lead was `Verification` / `Needs verification` / low priority. SQLite confirmed `lead_evidence: 0`, `drafts: 0`, and `published_posts: 0`. Workbench showed no selected lead or draft, and no draft/export/publish/public-site checks could be completed.

## Environment

- Windows host/user: `msi\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Branch: `test-comms/cleanroom-coder-tester`
- Coordination commit at start: `a0c4a86`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `1e03894496f7800b072586c30aac1e1b9afe4533`
- Release/docs commit under test: `c38c0f84a4eeaef39c4cd30d7a8c34f9903eb3af`
- Clean profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-1e03894-source-backed-brief`
- Installed EXE launched: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`

## Steps Run

- Pulled `test-comms/cleanroom-coder-tester` and reread the active directive plus protocol files.
- Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub v0.3.2 release.
- Verified installer SHA256 `E7B620C4D51837DDD43028B511E396643EE9A67D1CD23DC0B59BC5442277DCD7`.
- Verified installer size `5241214` bytes.
- Verified `SHA256SUMS.txt` names the Windows installer and contains the same hash.
- Confirmed release/docs visibility in `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-visibility.md`.
- Uninstalled the prior app instance and installed from the downloaded GitHub release installer.
- Launched only the installed EXE with `CIVICNEWS_APP_DATA_DIR` set to the fresh clean profile.
- Verified first-run onboarding appeared, not inherited Longmont state.
- Completed onboarding using the Longmont starter profile and the visible `Continue without AI` path after local AI service startup failed.
- Confirmed `community_profile.json` persisted `city: Longmont` and `state: CO`.
- Went directly to Daily Scan without manually repairing Settings.
- Ran Daily Scan; it completed without the prior `Choose your publication city and state in Settings before running Daily Scan.` blocker.
- Captured Daily Scan output, Story Queue output, Workbench output, app-data file lists, and SQLite summaries.
- Ran source discovery for `Longmont` / `Colorado`; the full state name produced a non-empty discovery list with selected sources.
- Imported the discovered sources, then reran source discovery for `Longmont` / `CO`; it produced the same non-empty recommendation list, with 0 selected because the sources were already present.
- Rechecked Story Queue, Workbench, and SQLite after discovery/import.

## Findings

### Blocker 1 - No draftable linked-evidence Story or Brief lead existed

Directive steps 19-21 require finding a credible Longmont lead with linked evidence and Story or Brief treatment, confirming Brief default behavior, and drafting reader-facing copy from linked source evidence. This cleanroom run could not satisfy those steps.

Evidence:

- `sqlite-summary-final.json` shows `leads: 5`, `daily_scan_leads: 5`, `lead_evidence: 0`, `drafts: 0`, and `published_posts: 0`.
- Every lead in SQLite has `story_type: verification` and `disposition: needs_verification`.
- Story Queue showed 5 new leads, 0 in drafting, 0 high priority, and only `Verify first` actions.
- Workbench showed `No lead or draft selected` and `No drafts exist yet`.
- No `Draft anyway` path was visible for no-evidence leads.

Because no credible linked-evidence Story/Brief lead existed, the required draft, editor workflow, export, here.now publish, and public visitor checks were not run.

### Minor 1 - AI remained in limited mode

The onboarding AI step failed to reach the local AI service. The app offered `Continue without AI`, and the dashboard showed `AI limited mode` with `Source checks still work`. Deterministic source checks and Daily Scan still ran, but drafting and AI-assisted review remained limited.

## Passing / Improved Checks

- Release visibility passed for the release page, docs page, GitHub release API asset list, installer SHA256, installer size, and checksum asset.
- The GitHub release API showed exactly two assets: `SHA256SUMS.txt` and `The.Civic.Desk_0.3.2_x64-setup.exe`.
- Public docs showed the new installer hash and did not contain the stale `B0550BFC230EAA67A321150CB458A3206D6C1D044E89E06B8FE392659012D4B6` hash.
- First-run onboarding used the isolated profile and did not inherit old state.
- Onboarding persisted `Longmont` / `CO`.
- Daily Scan opened and ran without city/state repair in Settings.
- Starter sources were added automatically; Daily Scan watched 9 sources.
- Source discovery accepted both full state name `Colorado` and abbreviation `CO`.
- Unsupported/no-source/model-suggested items were low-priority verification work, not high-priority or ready-to-draft story work.
- Story Queue did not show `Draft anyway`.
- Generic, navigation, index, markup, multi-item, and broad organization signals were presented as verification/source-quality work.

## Not Run

The following directive steps were blocked by Blocker 1:

- Draft a credible Longmont story or brief from linked source evidence.
- Confirm Brief/Story Article Format defaults in Workbench for a draftable lead.
- Confirm generated public copy quality.
- Exercise hold/send-back/approve/cut workflow.
- Compile/export the static site.
- Publish through here.now.
- Inspect the public here.now site as a visitor.

## Evidence

- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/visibility-receipt.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/release-api.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/release-page.html`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/public-docs.html`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/SHA256SUMS.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/The.Civic.Desk_0.3.2_x64-setup.exe`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/state-paths-before-install.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/state-paths-after-install.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/install-lifecycle.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cleanprofile-launch.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/community_profile-after-onboarding.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/community_profile-after-daily-scan.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/community_profile-final.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-01-first-screen.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-10-dashboard-after-finish.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-12-daily-before-run.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-14-daily-after-run.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-16-discover-colorado-filled.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-23-discover-co-results.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-24-story-queue.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-26-workbench.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-29-daily-after-second-run.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cdp-30-story-queue-after-second-run.txt`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/sqlite-summary-final.json`
- `test-comms/reports/20260709-final-release-v032-1e03894-source-backed-brief-evidence/cleanprofile-file-list-final.json`

## Reproduction Notes

1. Install from the downloaded release asset in the evidence directory.
2. Launch the installed EXE with:

```powershell
$cleanProfile = Join-Path $env:TEMP "civicdesk-final-v032-1e03894-source-backed-brief"
Remove-Item -LiteralPath $cleanProfile -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $cleanProfile -Force | Out-Null
$env:CIVICNEWS_APP_DATA_DIR = $cleanProfile
& "$env:LOCALAPPDATA\The Civic Desk\civicnews.exe"
```

3. Complete Longmont onboarding, continue without AI if the local AI service cannot start, go directly to Daily Scan, run the scan, and inspect Story Queue / SQLite.
