# Final Cleanroom Report - The Civic Desk v0.3.2 17766b7 Durable Draft

**FAIL**

The 17766b7 build fixes the prior durable draft persistence blocker: a linked-evidence Brief lead now generates and persists a draft (`drafts_count=1`) instead of relaunching and losing it. Final release still fails because the generated/edited draft cannot pass static publish preflight: the app reports package-validity blockers that the linked source documents do not match the story topic and that attribution/source linkage is insufficient. Approval, export, here.now publication, and public visitor inspection therefore remain blocked.

## Machine Profile

- Machine/user: `MSI\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Tester branch: `test-comms/cleanroom-coder-tester`
- Tester branch head at start of run: `d49566957fce28d07bb7e901c1c443cfee0b2f77`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `17766b7ccb0cc744522090e28997b764676ce1c5`
- Release/docs commit: `2e946205d1763247cbd8d4720b85fb2cec2af63a`
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Installed EXE version: `0.3.2`
- Clean app profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-17766b7-durable-draft`

## Steps Run

- Pulled `origin/test-comms/cleanroom-coder-tester` and reread all required test-comms files.
- Downloaded installer and checksum from the GitHub release.
- Verified SHA256 `8D5F6E06CA86B96DA7CC8AA9273305033C36A580A6B8064B6BC144550B5C25B3`.
- Uninstalled prior The Civic Desk instance with `uninstall.exe /S`.
- Installed downloaded release installer with `/S`.
- Launched only installed EXE with fresh `CIVICNEWS_APP_DATA_DIR`.
- Completed first-run onboarding for Longmont, CO.
- Used app-guided local AI setup. Initial onboarding entered limited mode, then AI Model setup installed/started local runtime and reported `Local AI ready` with `phi4-mini:latest`.
- Ran Daily Scan without editing Settings first.
- Opened the ready-to-draft Brief lead through `Open in Workbench`.
- Confirmed Article Format defaulted to `Brief`.
- Clicked explicit `Generate Draft`.
- Ran editor improvement and manual edit checks.
- Exercised editor hold/send-back controls.
- Stopped before export/publish because `Approve for Static Publish` remained disabled by package-validity blockers.

## Results

### Passed

- Installer visibility/hash mostly passed: release asset and checksum matched expected size/hash.
- Installer lifecycle passed: uninstall and install completed; installed EXE exists and reports version `0.3.2`.
- First-run onboarding passed on a fresh profile.
- Saved profile shows `city: Longmont`, `state: CO`.
- Daily Scan did not hit the previous Settings city/state blocker.
- Starter source creation passed: `sources_count=9`.
- Daily Scan completed: `daily_scan_runs_count=1`, `daily_scan_leads_count=5`, `leads_count=5`.
- Linked evidence retention passed: `lead_evidence_count=5`.
- One draftable Brief lead existed: `story_type=brief`, `disposition=ready_to_draft`, novelty score `4`.
- Weak/source-quality cards stayed separated as `Verification / Needs verification`.
- Article Format defaulted to `Brief` for the ready Brief lead.
- Durable draft persistence passed: after `Generate Draft`, Workbench opened a persisted draft and SQLite showed `drafts_count=1`.

### Failed / Blocked

- Release visibility failed one directive item: release body did not contain full product commit `17766b7ccb0cc744522090e28997b764676ce1c5`.
- Built-in `Improve for Publication` failed and left the draft unchanged: `The improved draft introduced unsupported jurisdiction term(s): Colorado.`
- Static publish approval was blocked by app preflight:
  - `This scanned-lead draft's linked source documents do not appear to match the story topic.`
  - `No clear attribution phrase found. Attribute key facts to the source or rewrite more cautiously.`
  - `Linked source documents may not match this story topic. Attach the correct source material or rewrite the story around the linked sources.`
- `Approve for Static Publish` remained disabled.
- Export ZIP, here.now publication, public URL, and public site inspection were not reached because approval was blocked.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 2
- Minor: 1
- Nit: 0

### Blocker 1 - Draft persists, but publish path remains blocked by source/topic mismatch

Observed: `Generate Draft` created a durable draft, but the draft package failed preflight because the linked source document did not match the story topic closely enough. The app kept `Approve for Static Publish` disabled, so export and here.now publication could not be tested.

Expected: A ready-to-draft linked-evidence Brief lead should generate a draft that can either pass preflight directly or be made publishable through editor controls without changing the source package.

Impact: The release cannot complete final E2E publication proof.

Evidence:

- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/after-generate-draft.png`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/manual-edit-playwright-fill.png`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/final-sqlite-state.json`
- `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/final-sqlite-state-summary.txt`

### Major 1 - Release page did not show the expected product commit

Observed: The fetched GitHub release body did not contain `17766b7ccb0cc744522090e28997b764676ce1c5`, although installer size/hash and checksum were correct.

Expected: The release page should visibly show the product build commit named in the directive.

Impact: Release provenance is less explicit for testers/users.

Evidence: `visibility-download-state.json`, `release-api.json`.

### Major 2 - Improve for Publication failed on its own generated draft

Observed: Clicking `Improve for Publication` returned: `Failed to improve draft: Something went wrong: The improved draft introduced unsupported jurisdiction term(s): Colorado. The editor content was not changed.`

Expected: The improvement workflow should produce a safer draft or provide actionable editor guidance without failing on a common jurisdiction term in a Longmont, CO workspace.

Impact: The built-in remediation path cannot fix the draft quality/preflight problem.

Evidence: `after-improve-for-publication.png`.

### Minor 1 - Some event/calendar verification cards still appear high priority

Observed: Daily Scan kept event/calendar weak leads as `Verification / Needs verification`, which is good, but two such cards were still `High priority`.

Expected: Weak event/listing material should remain clearly lower urgency unless a concrete civic impact or deadline is present.

Impact: Triage may overstate weak community signals.

## SQLite Summary

Final database state:

- `settings_count=13`
- `sources_count=9`
- `daily_scan_runs_count=1`
- `daily_scan_leads_count=5`
- `leads_count=5`
- `lead_evidence_count=5`
- `evidence_items_count=26`
- `drafts_count=1`
- `published_posts_count=0`
- `publish_runs_count=0`
- `publish_decision_audits_count=0`
- `verification_tasks_count=61`

## Evidence Index

- Visibility/download: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/visibility-download-state.json`
- Release API: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/release-api.json`
- Public docs snapshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/public-docs.html`
- Installer lifecycle: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/install-lifecycle.json`
- First-run screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/first-run-onboarding.png`
- Onboarding complete screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/workspace-after-onboarding.png`
- Daily Scan result screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/daily-scan-after-run.png`
- Ready Brief Workbench screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/workbench-ready-brief-ai-ready.png`
- Draft generated screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/after-generate-draft.png`
- Improve failure screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/after-improve-for-publication.png`
- Manual edit/preflight screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/manual-edit-playwright-fill.png`
- Hold modal screenshot: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/after-hold.png`
- Final SQLite state: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/final-sqlite-state.json`
- Final SQLite summary: `test-comms/reports/20260709-final-release-v032-17766b7-durable-draft-evidence/final-sqlite-state-summary.txt`

## Request For Coder

Keep the durable draft persistence fix. Next, fix the source/topic package for the ready-to-draft Brief path so generated or edited reader-facing copy can pass static publish approval. Also update the release page body to show the full product build commit required by the directive.
