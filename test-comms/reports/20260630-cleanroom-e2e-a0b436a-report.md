# CivicNewspaper cleanroom E2E report - a0b436a attempt 1

Date: 2026-06-30 UTC
Tester machine: Windows cleanroom tester, user `MSI\civic`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit: `a0b436af3009500714055a2bff01612716ee36c1`
Directive: `test-comms/directives/20260630-cleanroom-e2e-a0b436a-attempt1.md`

## Plain-English verdict

FAIL, evidence-backed.

The app installed, launched, configured local AI, discovered/imported starter Longmont sources, ran Daily Scan, generated 13 leads, and generated 5 drafts. However, the end-to-end publish path could not complete because the app's own public-output quality gate blocked compile/publish:

`Public output quality gate failed: watch/5.html contains public-output marker employee login; watch/4.html contains public-output marker employee login; watch/3.html contains public-output marker employee login; watch/2.html contains public-output marker editor_note`

I did not manually patch around this product failure. No here.now URL was produced for this attempt because compile/publish did not pass.

## Installer and cleanroom setup

- Preferred NSIS installer used: `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256: `B6777C66A7330A46F6FC443576C06E648E516EC52EC845004044DB4663A23BD8`
- NSIS size: `5605081`
- Fallback MSI visible and verified: `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/The Civic Desk_0.3.0_x64_en-US.msi`
- MSI SHA256: `4C4F40178017853DFA5E65AFD10595306018C0F2B803190A1DB431A28CA8AA2E`
- MSI size: `9117696`
- Visibility report was already present and pushed at `test-comms/reports/20260630-cleanroom-e2e-a0b436a-visibility-attempt-1.md`.
- Evidence: `tester-output/evidence/final-run-command-summary.json`, first-run screenshots, installer/app process evidence under `tester-output/evidence/`.

## AI setup and model

- First-run flow reached local AI setup.
- App installed/used local Ollama runtime under `%APPDATA%\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\`.
- Model shown by app: `qwen2.5:7b`.
- App status at test time: `Local AI ready`.
- Evidence: `first-run-launch-screen.png`, `clean-first-run-screen.png`, `ai-service-setup-after-wait.png`, `ai-model-complete-screen.png`, `final-run-command-summary.json`.

## Sources and discovery

The app ended with 6 sources visible:

- `https://longmontcolorado.gov/city-clerk/agenda-management-portal/`
- `https://longmontcolorado.gov/government/city-council-meetings/`
- `https://longmontcolorado.gov/public-information/`
- `https://www.publicnoticecolorado.com/`
- `https://www.reddit.com/r/Longmont/`
- `https://www.reddit.com/r/LongmontColorado/`

Evidence: `sources-screen-initial.png`, `queue-after-restart.png`, `queue-after-multiple-draft-approvals.png`.

## Lead and story counts

- Leads produced: 13.
- Drafts produced: 5.
- High priority leads visible: 1.
- Approved/publish-path stories attempted: multiple drafts were mechanically approved to exercise the pipeline.
- The app wrote partial static output containing `watch/2.html` through `watch/5.html`, but compile/publish was blocked by quality gates.

Evidence: `queue-after-multiple-draft-approvals.png`, `final-run-command-summary.json`, `tester-output/site-output-copy/`.

## Story/workflow exercise

I exercised these workflow controls:

- Created drafts from queue leads.
- Opened Workbench.
- Edited and saved a draft.
- Used `Send Back for More Work`.
- Used `Hold`.
- Attempted `Cut Story`; the cut confirmation appeared and restore/resume controls were visible.
- Used `Resume Editing`.
- Approved drafts for static publish.
- Ran the press-freedom/legal-risk advisor control.

Important limitation: the press-freedom/legal-risk advisor did not produce a separate visible advisory result during this run. The UI continued to show static advisory guidance and non-blocking warnings. Evidence: `workbench-press-freedom-advisor-result.png`.

Evidence: `workbench-second-draft-editor-note-visible.png`, `workbench-save-draft-after-tester-edit.png`, `workbench-status-send-back.png`, `workbench-status-hold.png`, `workbench-status-cut.png`, `workflow-current-draft-before-approve.png`, `workflow-current-draft-after-approve.png`.

## Story list and quality findings

Observed generated titles included:

- `City Council Set to Discuss Temporary Closure of Hover Street/CO 119 Intersection`
- `Overview of City Departments in Longmont`
- `Understanding How to Participate in City Council Meetings`
- `City to Close Intersection of Hover Street/CO 119 for Overnight Roof Work`

The generated drafts repeatedly exposed reporter/editor scaffolding rather than reader-facing newspaper output:

- One generated body began with `Editor_note: Not enough verified source material for a publishable story yet.`
- Another generated body began with `Editor Note: This looks like background material, not a publishable news story yet.`
- Several stories had no linked source documents.
- The app's own guardrail warned: `Draft still contains internal reporter-note marker(s): editor_note:. Remove or rewrite them as reader-facing copy before publishing.`
- Compile failed because public output included `editor_note` and `employee login` markers.

Evidence: `draft-controlled-generation-final.png`, `draft-4-generated.png`, `draft-5-generated.png`, `draft-6-generated.png`, `final-compile-quality-gate-block.png`, `public-output-marker-scan.json`.

## Export, ZIP, here.now

- Static output folder used by app: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`.
- A committed copy of the failed partial output is included at `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/site-output-copy/`.
- ZIP export did not complete.
- here.now publish did not complete.
- No here.now URL was produced.

Exact failure point: `Publishing -> Review compile checklist -> Compile site`.

## Public output quality gate audit

The final copied output was scanned for directive markers. Findings:

- `watch/2.html`: `EDITOR_NOTE` / `editor_note`
- `watch/4.html`: `Editor Note`
- `watch/3.html`: `employee login`
- `watch/4.html`: `employee login`
- `watch/5.html`: `employee login`

Evidence: `test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/evidence/public-output-marker-scan.json`.

## Evidence index

Primary evidence folder:

`test-comms/artifacts/20260630-cleanroom-e2e-a0b436a/tester-output/evidence/`

Important files:

- `final-compile-quality-gate-block.png`
- `final-compile-block-ui-summary.json`
- `final-run-command-summary.json`
- `public-output-marker-scan.json`
- `publishing-after-compile.png`
- `publishing-identity-saved-neutral.png`
- `queue-after-multiple-draft-approvals.png`
- `site-output-copy/`

## Watcher status

The CivicNewspaper watcher remains armed for follow-up directives. CivicCast context was not used for this run.
