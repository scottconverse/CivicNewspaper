# TEST REPORT - Civic Desk v0.3.2 1467846 Weak Lead and Brief Gate Rerun

Date: 2026-07-08T23:18:00Z  
Tester machine: `msi\civic`  
Repo: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`  
Product/release: GitHub release `v0.3.2`  
Product build commit under test: `14678467489a49c169006a8f05511a60c78ec6fa`  
Directive: `test-comms/ACTIVE_DIRECTIVE.md`

## Result

**FAIL**

The core weak-lead gate fix looks substantially better in this cleanroom run: no-evidence Daily Scan leads were low-priority verification items, Story Queue showed `Verify first`, and `Draft anyway` did not appear. However, the release cannot pass this directive because the cleanroom run produced no credible linked-evidence Story or Brief lead to draft. The required draft, editor, export, publish, and public-site checks therefore could not be completed.

## Environment

- Windows host/user: `msi\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Clean profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-1467846-weak-lead-brief-gate`
- WebView debug args: `--remote-debugging-port=9223 --remote-allow-origins=*`
- AI state: app completed setup in limited mode after local AI runtime did not become reachable at `127.0.0.1:11434` during the wait.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `ACTIVE_DIRECTIVE.md`, README, protocol, tester prompt, and directives list.
2. Downloaded release installer and `SHA256SUMS.txt` from GitHub release `v0.3.2`.
3. Verified installer SHA256 `99B3C381C877D8B67997FCB2CEA07222C1C78A0C8CF5B4DB35424F6B01300292` and size `5231944`.
4. Verified release/docs visibility requirements; see visibility report.
5. Recorded required app-data paths before install.
6. Uninstalled prior `The Civic Desk`, installed from downloaded GitHub installer, and launched installed EXE only.
7. Created fresh `CIVICNEWS_APP_DATA_DIR` at `%TEMP%\civicdesk-final-v032-1467846-weak-lead-brief-gate`.
8. Captured first screen: first-run onboarding appeared, not inherited Longmont state.
9. Selected the Longmont starter profile and completed onboarding. Local AI setup did not become reachable, so the app-supported `Skip for now` path was used.
10. Confirmed `community_profile.json` contained `city: Longmont` and `state: CO` before any Settings repair.
11. Went directly to Daily Scan and ran it without manually repairing Settings.
12. Ran source discovery for `Longmont` / `Colorado` and `Longmont` / `CO`; both returned Longmont source candidates.
13. Inspected Story Queue after Daily Scan.
14. Ran `Scrape & Detect` once to see whether a linked-evidence Story/Brief lead became available.
15. Queried SQLite for sources, evidence items, leads, lead/evidence linkage, and drafts.

## Results

- Visibility on required cleanroom host/path: **PASS**.
- Installer download/hash/size: **PASS**.
- Installed-app launch from GitHub release installer: **PASS**.
- Isolated `CIVICNEWS_APP_DATA_DIR`: **PASS**.
- First-run onboarding instead of inherited state: **PASS**.
- Onboarding identity persistence: **PASS** (`Longmont` / `CO` in `community_profile.json`).
- Daily Scan immediately after onboarding without Settings repair: **PASS**.
- Source discovery with `Colorado`: **PASS**.
- Source discovery with `CO`: **PASS**.
- No-evidence lead downgrade: **PASS**. SQLite showed 5 Daily Scan leads, all `story_type=verification`, `disposition=needs_verification`, no linked evidence, and low priority in UI.
- `Draft anyway` removal for weak/no-evidence leads: **PASS**. Story Queue showed `Verify first`; `Draft anyway` was not present.
- Story/Brief linked-evidence draft path: **BLOCKED/FAIL**. No Story or Brief lead was available to draft. `Scrape & Detect` added three linked Watch leads only, still gated as `Verify first`.
- Draft generation quality: **NOT RUN** because no credible Story/Brief draftable lead existed.
- Editor hold/send-back/approve/cut: **NOT RUN**.
- Export/publish/public here.now verification: **NOT RUN**.

## Evidence

- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/visibility-receipt-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/install-lifecycle-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/cleanprofile-launch-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/cdp-1467846-01-first-screen-msi-civic.png`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/community_profile-after-onboarding-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/cdp-1467846-07-daily-scan-after-run-msi-civic.png`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/story-queue-gate-summary-1467846-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/source-discovery-1467846-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/scrape-detect-1467846-msi-civic.json`
- `test-comms/reports/20260705-final-release-v032-1467846-weak-lead-brief-gate-evidence/sqlite-summary-after-scrape-detect-1467846-msi-civic.json`

## Severity

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

## Findings / Blocking Conditions

### Blocker 1 - No linked-evidence Story/Brief lead existed, so required draft/export/publish workflow could not run

Observed: Daily Scan produced 5 open leads, all verification/no-evidence. `Scrape & Detect` increased the queue to 8 leads, but the 3 linked-evidence leads were Watch/Watch `New Primary Record` items with `Verify first`, not Story or Brief leads with Draft/Open Draft controls. SQLite confirmed 8 leads, 3 linked evidence rows, 0 drafts, and the linked rows belonged to Watch leads.

Expected: The directive requires at least one credible Longmont Story or Brief lead with linked evidence so the tester can verify Article Format defaults to Brief, generate reader-facing copy, exercise editor workflow, export, publish, and inspect the public site.

Impact: The release cannot pass this cleanroom directive because steps 19-27 are unreachable from the generated queue without inventing or manually mutating a lead.

Repro: Fresh install/profile, Longmont onboarding, Daily Scan, then Story Queue. Optionally run `Scrape & Detect`; observe only verification/watch `Verify first` leads and no Story/Brief draft path.

### Major 1 - Verification lead text still includes page-markup/navigation debris

Observed: Some low-priority verification leads correctly remained non-draftable, but their displayed text included page/navigation debris such as department-directory copy and a blank/debris item. The app labels this with quality-gate text, but the queue still shows noisy lead bodies.

Expected: Weak or markup-debris items should be suppressed, summarized cleanly as verification work, or moved out of ordinary queue text so editors do not have to parse raw page debris.

Impact: This is less severe than the previous release blocker because the leads are now gated as `Verify first`, but it still lowers editor trust and makes the queue noisy.

Repro: Run Daily Scan in the clean Longmont profile and inspect the verification leads from Longmont city events.

### Minor 1 - Local AI setup did not become reachable during onboarding wait

Observed: The app showed `Starting the local AI service...`, but `127.0.0.1:11434` did not become reachable during the wait, so setup continued through the app-supported `Skip for now` path. Daily Scan still completed in limited mode.

Expected: If local AI runtime startup is expected for this release path, setup should either complete startup or give a clear finished/failed state. If limited mode is acceptable, this is informational only.

## Request

Please provide a build/directive path that reliably yields at least one credible linked-evidence Story or Brief lead in a fresh Longmont cleanroom run, or update the queue logic so one of the linked official-source leads can become a draftable Brief when appropriate. The no-evidence downgrade and `Draft anyway` removal appear improved, but the required draft/export/publish validation remains blocked.
