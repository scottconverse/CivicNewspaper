# Final Cleanroom Report - The Civic Desk v0.3.2 35e6cf0 Dark-Signal Evidence

**FAIL**

Release does not pass final cleanroom. The 35e6cf0 build fixes the previous linked-evidence absence: Daily Scan produced 5 leads, `lead_evidence_count=5`, and one `Brief / Ready to draft` Longmont lead with linked source evidence. However, the explicit draft-generation path fails: clicking `Generate Draft` from the ready-to-draft Brief lead closes/relaunches the app/WebView, writes no draft, and leaves `drafts_count=0`. Because no draft can be produced, editor workflow, static export, here.now publication, and public-site inspection are blocked.

## Machine Profile

- Machine/user: `MSI\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Tester branch: `test-comms/cleanroom-coder-tester`
- Tester branch head at start of run: `4e9114dc10e8f82fe23f9e79ff9c4753e0d3577d`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `35e6cf0f4a8f01d74ef79247feaaadbd34dbb3da`
- Release/docs commit: `62a44b1ea4b4ea3ba05a76811d9c45af37d825c8`
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Installed EXE version: `0.3.2`
- Clean app profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-35e6cf0-dark-signal-evidence`

## Commands And Procedure

- Pulled `origin/test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/`.
- Downloaded `The.Civic.Desk_0.3.2_x64-setup.exe` and `SHA256SUMS.txt` from the GitHub release, not from any `C:\Users\instynct` path.
- Verified installer SHA256 exactly: `8204BB4210DD284518D114C57A3089BAC11D7B0EC8E0F83D8D61928D44FEB6E0`.
- Uninstalled prior The Civic Desk instance via `C:\Users\civic\AppData\Local\The Civic Desk\uninstall.exe /S`.
- Installed downloaded release installer with `/S`; installer exit code `0`.
- Launched only installed EXE with `CIVICNEWS_APP_DATA_DIR=C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-35e6cf0-dark-signal-evidence`.
- Used first-run onboarding from the installed app; selected Longmont, CO and accepted detected local AI model `phi4-mini:latest`.
- Did not repair city/state in Settings before Daily Scan.
- Waited for starter source creation, then ran Daily Scan.
- Opened the ready-to-draft Brief lead with the explicit `Draft` button and confirmed Article Format defaulted to `Brief`.
- Clicked explicit `Generate Draft`.
- Reopened the same clean profile to inspect persisted SQLite and UI state after the app/WebView relaunched.

## What Passed

- Release visibility passed. Release and docs are reachable and show the new hash, size, product commit, unsigned installer guidance, Windows-only beta language, and no stale `E7B620...` hash.
- Installer lifecycle passed. Prior app was uninstalled, downloaded release installer installed cleanly, and installed EXE version is `0.3.2`.
- First-run onboarding passed. The first screen was onboarding on the fresh profile; saved `community_profile.json` and `settings` show `Longmont` / `CO`.
- Daily Scan city/state blocker did not recur. After onboarding, starter sources were added and Daily Scan ran without `Choose your publication city and state in Settings before running Daily Scan.`
- Starter sources were added: SQLite `sources_count=9`.
- Daily Scan produced leads: SQLite `daily_scan_runs_count=1`, `daily_scan_leads_count=5`, `leads_count=5`.
- Linked evidence retention improved: SQLite `lead_evidence_count=5`, with every lead linked to an evidence row.
- Ready-to-draft Brief promotion exists: lead `id=4`, `story_type=brief`, `disposition=ready_to_draft`, novelty score `4`, linked evidence `id=25`.
- Weak/source-quality leads are separated from ordinary draft work. The UI shows source-quality cards as `Verification / Needs verification` with `Verify first`, and no `Draft anyway` was observed for those cards.
- Workbench defaulted Article Format to `Brief` when opening the ready-to-draft Brief lead.

## Blocker Findings

### Blocker 1 - Generate Draft from the linked-evidence Brief lead does not create a draft

The required cleanroom path still cannot reach a reader-facing draft.

Observed sequence:

1. Story Queue showed one `Brief / Ready to draft` lead: `Review community signal from Longmont city events: Public Safety Sustainability Transportation Utilities`.
2. SQLite confirmed this lead had linked evidence through `lead_evidence` (`lead_id=4`, `evidence_id=25`).
3. I clicked the explicit `Draft` button.
4. Workbench opened `Drafting Article`, showed linked source count `1`, and Article Format defaulted to `Brief`.
5. I clicked the explicit `Generate Draft` button.
6. The WebView/debug session closed and the app relaunched under a new `civicnews.exe` process.
7. Reopening the same clean profile showed `Drafts 0`, `IN DRAFTING 0`, and SQLite `drafts_count=0`.

Impact: editor workflow, hold/send-back/approve/cut, static export, here.now publication, and public here.now inspection are blocked. The release cannot pass because the directive requires drafting at least one credible Longmont story or brief from linked source evidence.

Evidence:

- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/after-draft-click.png`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/post-draft-crash-relaunch-ui.png`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/final-sqlite-state.json`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/final-sqlite-state-summary.txt`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/process-relaunch-observations.json`

## Major Findings

### Major 1 - Source discovery submit also closes/relaunches the WebView/app inspection session

After the draft blocker was established, I opened the `Discover for my city` modal and entered `Longmont` / `Colorado`. Submitting `Auto-Find Feeds` closed the WebView/debug session and the app continued under a new process with no debug endpoint. SQLite still showed `sources_count=9`; no new discovery result was persisted. This prevented a complete `Longmont / Colorado` then `Longmont / CO` discovery rerun.

Evidence:

- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/source-discovery-open.png`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/process-relaunch-observations.json`
- `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/final-sqlite-state.json`

## Minor Findings

### Minor 1 - Local event/calendar content still produces high-risk verification cards

The app correctly keeps these cards as `Verification / Needs verification` with `Verify first`, so this is not a blocker by itself. However, two verification cards were labeled `Risk: high` from event/calendar content:

- `Glam Group (Summer Program) Series Longmont Youth Center...`
- `Free Document Shredding Event...`

They are not draftable, but the risk label may still make weak community/event material look more urgent than it is.

## Not Completed Because Blocked

- Draft body quality audit: blocked because `Generate Draft` wrote no draft.
- Editor hold/send-back/approve/cut workflow: blocked because there was no draft.
- Static export ZIP: blocked because there was no draft to publish.
- here.now anonymous preview publication: blocked because there was no publishable draft/site.
- Public here.now visitor inspection: blocked because publication could not be reached.

## SQLite Summary

From `final-sqlite-state-summary.txt`:

- `settings_count=13`
- `sources_count=9`
- `daily_scan_runs_count=1`
- `daily_scan_leads_count=5`
- `leads_count=5`
- `lead_evidence_count=5`
- `evidence_items_count=26`
- `drafts_count=0`
- `published_posts_count=0`
- `publish_runs_count=0`
- `verification_tasks_count=61`
- Ready-to-draft lead: `id=4`, `story_type=brief`, `disposition=ready_to_draft`, novelty score `4`

## Evidence Index

- Visibility/download: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/visibility-download-state.json`
- Release API: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/release-api.json`
- Public docs snapshot: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/public-docs.html`
- Installer lifecycle: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/install-lifecycle.json`
- Preinstall state paths: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/preinstall-state-paths.json`
- Onboarding profile: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/community_profile-after-onboarding.json`
- Final profile: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/community_profile-after-daily-scan.json`
- Daily Scan ready screenshot: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/daily-scan-ready-after-starter-sources.png`
- Story Queue after Daily Scan: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/ui-after-relaunch-scan-state.png`
- Draft flow screenshot: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/after-draft-click.png`
- Post-draft relaunch screenshot: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/post-draft-crash-relaunch-ui.png`
- Final SQLite state: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/final-sqlite-state.json`
- Final SQLite summary: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/final-sqlite-state-summary.txt`
- Process relaunch observations: `test-comms/reports/20260709-final-release-v032-35e6cf0-dark-signal-evidence-evidence/process-relaunch-observations.json`

## What Tester Needs From Coder Next

Fix and rerun the installed-app draft path from a linked-evidence Brief lead. The current build now creates linked evidence and a ready-to-draft Brief, but `Generate Draft` must create a durable draft row and keep the app alive before final release can pass.
