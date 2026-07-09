# Final Cleanroom Report - The Civic Desk v0.3.2 ba49af4 Publish Flow

**FAIL**

The ba49af4 build improves the publish path substantially: release visibility passes, the linked-evidence Brief draft persists, editor attestation can move the draft to `ready_to_publish`, static export writes 18 files plus `site-package.zip`, and the anonymous here.now preview publishes successfully. Final release still fails because the published reader-facing brief is not coherent civic copy: it is a navigation/service-list fragment from the Longmont events page, not a concrete source-backed civic item.

## Machine Profile

- Machine/user: `MSI\civic`
- Repo path: `C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms`
- Tester branch: `test-comms/cleanroom-coder-tester`
- Active directive: `test-comms/ACTIVE_DIRECTIVE.md`
- Product build commit under test: `ba49af4d69d2c4d6d88bfd148490494f243cc9d7`
- Release/docs commit in refreshed directive: `2cb62b8262a04111d00b1b4e1d0ebd9b4a78eeb1`
- Installed EXE: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Clean app profile: `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-ba49af4-publish-flow`

## Steps Run

- Pulled and fast-forwarded `origin/test-comms/cleanroom-coder-tester`, including the refreshed docs commit pointer.
- Reread `test-comms/ACTIVE_DIRECTIVE.md`.
- Downloaded installer and checksum from the GitHub release.
- Verified SHA256 `1D6E650C44B44A74C5E7640097D2F8FF0618631D4C7311738229F424441F8BD5`.
- Uninstalled prior The Civic Desk instance and installed the downloaded release installer.
- Launched only the installed EXE with fresh `CIVICNEWS_APP_DATA_DIR`.
- Completed first-run Longmont, CO onboarding.
- Ran Daily Scan without editing Settings first.
- Opened the linked-evidence Brief lead and generated a draft.
- Used editor attestation/override to approve the draft for static publishing.
- Set publication identity to `Longmont Civic Desk` to clear the starter-name compile gate.
- Ran the compile checklist and clicked `Compile site`.
- Inspected generated `index.html`, `briefs/1.html`, and SQLite state.
- Published through the authorized anonymous here.now preview path.
- Fetched the public here.now home page and brief page as a visitor.

## What Passed

- Release visibility passed: release/docs were reachable, hash/size/commit matched the directive, and stale hashes were absent.
- Installer lifecycle passed.
- First-run onboarding passed on a fresh profile.
- Saved profile/settings show `Longmont` / `CO`.
- Daily Scan did not hit the prior city/state Settings blocker.
- Starter source creation passed: `sources_count=9`.
- Daily Scan completed: `daily_scan_runs_count=1`, `daily_scan_leads_count=5`, `leads_count=5`.
- Linked evidence retention passed: `lead_evidence_count=5`.
- Durable draft persistence passed: `drafts_count=1`.
- Editor attestation/override passed: `publish_decision_audits_count=1`, draft status `ready_to_publish`.
- Static export passed mechanically: the local export run wrote 18 files for 1 article with `skipped_count=0`.
- ZIP package was created at `C:\Users\civic\AppData\Local\Temp\civicdesk-final-v032-ba49af4-publish-flow\sites\default\site-package.zip`.
- here.now publication passed mechanically: `publish_runs_count=2`, provider `here_now`, public URL `https://chilly-hearth-wcnn.here.now`.
- Public here.now visitor fetches returned HTTP 200 for the home page and `briefs/1.html`.

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### Blocker 1 - Static export publishes navigation/source-chrome text as the civic brief

The release cannot pass because the generated public article is not coherent reader-facing civic copy.

Observed public article headline:

`Review community signal from Longmont city events: Public Safety Sustainability Transportation`

Observed public article body:

`According to the linked source, Public Safety Sustainability Transportation Utilities Wellness Explore All Services Explore a comprehensive list of services provided by the City of Longmont. Source. Taken together, the linked records give Longmont readers a source-backed civic brief. Keep the story limited to these records until an editor confirms any public impact, cost, decision date, or agency response not shown in the linked sources.`

This is a page-navigation/service-list fragment. It does not identify a concrete civic action, date, meeting, deadline, amount, decision, or public impact. It also tells the reader to keep the story limited until an editor confirms impact, which is internal/editorial process language rather than a publishable civic brief.

Impact: The app can now compile, but the compiled output still fails the directive's public-copy quality gate.

Evidence:

- `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/exported-brief-1.html`
- `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/exported-index.html`
- `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/after-checklist-Compile.png`
- `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/final-sqlite-state-summary.json`

### Minor 1 - Event/calendar weak leads still appear high priority

Daily Scan still included event/calendar-style weak leads as high priority verification work. They were not draftable, but the priority label may overstate weak community/event material.

## Static Site Quality Checks

- Duplicate topic stories: no duplicate story files observed; one brief was exported.
- Raw Markdown in HTML pages: none observed in `index.html` or `briefs/1.html`.
- Internal tester/developer paths in generated public HTML: none observed in `index.html` or `briefs/1.html`.
- Unsupported Mac/Linux installer claims in generated public HTML: none observed.
- Broken story links: local `index.html` links to existing `briefs/1.html`.
- Public here.now URL: `https://chilly-hearth-wcnn.here.now`
- Public here.now visitor fetches: HTTP 200 for home and brief.
- Public copy quality: failed, as described in Blocker 1.

## SQLite Summary

From `final-sqlite-state-summary.txt`:

- `settings_count=13`
- `sources_count=9`
- `daily_scan_runs_count=1`
- `daily_scan_leads_count=5`
- `leads_count=5`
- `lead_evidence_count=5`
- `evidence_items_count=26`
- `drafts_count=1`
- `published_posts_count=1`
- `publish_runs_count=2`
- `publish_decision_audits_count=1`
- `verification_tasks_count=61`

## Evidence Index

- Visibility/download: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/visibility-download-state.json`
- Release API: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/release-api.json`
- Public docs snapshot: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/public-docs.html`
- Installer lifecycle: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/install-lifecycle.json`
- Onboarding screenshot: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/first-run-onboarding.png`
- Daily Scan screenshot: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/daily-scan-after-run.png`
- Draft/workbench screenshots: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/workbench-ready-lead-4.png`, `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/lead-4-generate-form.png`
- Approval/export screenshots: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/workbench-relaunch-current.png`, `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/publishing-after-identity-save.png`, `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/after-checklist-Compile.png`
- Static ZIP copy: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/site-package.zip`
- Exported HTML copies: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/exported-index.html`, `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/exported-brief-1.html`
- here.now public fetch evidence: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/herenow-home-response.json`, `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/herenow-brief-response.json`
- Final SQLite summary: `test-comms/reports/20260709-final-release-v032-ba49af4-publish-flow-evidence/final-sqlite-state-summary.json`

## Request For Coder

Keep the release visibility, durable draft, editor override, static export, and here.now publication improvements. Next, prevent navigation/service-list source chrome from becoming a publishable Brief.
