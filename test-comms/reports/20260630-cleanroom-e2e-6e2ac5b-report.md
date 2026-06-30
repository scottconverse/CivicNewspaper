# Tester Report - Cleanroom E2E Attempt 9

Date: 2026-06-30
Tester machine: MSI\civic on MSI
Repo: https://github.com/scottconverse/CivicNewspaper.git
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: 6e2ac5b4aff8ea069e3fd33c3cb796ab29d955ad
Directive: test-comms/directives/20260630-cleanroom-e2e-6e2ac5b-attempt9.md

## Verdict

PASS.

The attempt-8 runtime install regression appears fixed in this cleanroom pass. The real installed Windows app completed app-guided local AI setup without tester-installed Ollama or model setup, created an app-data runtime under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11`, downloaded/used `qwen2.5:7b`, completed Daily Scan, generated/approved publishable briefs, compiled static output, exported a ZIP, and published the same issue to here.now.

Important caveat: Daily Scan produced 19 leads, but only 2 were honestly approved as reader-facing briefs. The remaining tested weak/background/source-intake drafts were held or cut rather than padded into the public issue.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 344.9 GB on C:
- Node: v24.14.0 bundled runtime
- Rust: not installed/on PATH
- npm: not installed/on PATH
- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Runtime process path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`

## Installer Verification

- NSIS installer: `test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64-setup.exe`
- NSIS SHA256: `8E38C8641B5A9302B1E70361A62212DF73917F14607C2040BCC7CFB0B6581719`
- NSIS size: 5626730 bytes
- MSI fallback: `test-comms/artifacts/20260630-cleanroom-e2e-6e2ac5b/The Civic Desk_0.3.0_x64_en-US.msi`
- MSI SHA256: `AAA2F595C7DB896843EE4DF6AE54BB5516C6753932455977C8B61797DA7E1C8A`
- MSI size: 9117696 bytes
- Installer used: NSIS

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`, read `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
2. Verified installer hashes and wrote the required visibility report before full testing.
3. Removed prior CivicNewspaper app data, install state, output folders, local model/app state, and prior here.now/local artifacts for this test.
4. Installed and launched the real desktop app from the NSIS installer.
5. Completed onboarding as Longmont, Colorado with tester identity `Attempt Nine Editor`.
6. Allowed the app-guided AI setup to install/start the app-managed Ollama runtime and model without tester manual dependency installation.
7. Ran Daily Scan and captured source, queue, runtime, and scan-progress evidence.
8. Exercised draft/workbench workflow: generate, edit/save, hold, send-back/needs-work path, warning checkpoint approval, Back to Queue, and sidebar Publishing navigation.
9. Approved the two publishable briefs, held one weak generated brief, and cut two unsuitable/source-intake drafts.
10. Compiled the static issue, exported the ZIP, and published through the here.now connector.
11. Copied local output and ZIP extract into evidence, fetched live here.now pages, and scanned local/ZIP/live text for scaffolding, editor notes, mojibake, and taxonomy/path regressions.

## Results

- App-guided AI/runtime/model setup: PASS. Runtime was created under app data, `ollama.exe` ran from that folder, and the UI reached `Local AI ready / qwen2.5:7b`.
- Identity display: PASS. UI displayed `LONGMONT / CO`.
- First-run starter sources: PASS. 19 sources seeded. Local media sources present and accepted as `news_reporting`: Longmont Leader local news and Times-Call Longmont news. Community/event sources present: Longmont Area Chamber of Commerce, Visit Longmont events, Downtown Longmont events.
- Daily Scan: PASS. 19 sources watched; 19 leads produced; 2 high priority leads.
- Story/brief count: PASS with caveat. 2 reader-facing briefs approved; fewer than 5 were honestly publishable.
- Warned approval path: PASS. A warned ready-to-review draft approved through the UI and recorded `attested_by`, `attested_at`, and `guardrail_override_reason`.
- Navigation recovery: PASS. Back to Queue and sidebar Publishing recovered without restart.
- Static compile: PASS. Compile receipt: 2 articles, 19 files, 0 skipped.
- ZIP export: PASS. `site-package.zip` generated.
- here.now publish: PASS. Public URL: https://bright-sphinx-ywpd.here.now
- Manifest/UI URL agreement: PASS. Manifest `published_url` is `https://bright-sphinx-ywpd.here.now`, matching the UI publish result.
- Public output cleanup: PASS. Local output, ZIP extract, RSS/share artifacts, and live fetched here.now pages had no scan hits for forbidden scaffolding/editor markers or mojibake markers.
- Public taxonomy/path: PASS. Internal `watch` drafts approved for reader output published as manifest `format: brief`, with public paths `briefs/1.html` and `briefs/2.html`.

## Final Database Statuses

- `ready_to_publish`: 2
  - `Longmont City Council Meeting Rescheduled for July 1`
  - `Longmont Youth Center Lists Summer Programs`
- `hold`: 1
  - `Longmont Public Library Hosts Open Chess Night` was held because the generated copy contained a broken sentence and unsupported/generalized framing.
- `killed`: 2
  - `City of Longmont Updates Public Information Resources` was cut because it was a generic source-intake/update note.
  - `Longmont Public Library Hosts Weekly Spanish-English Conversation Group` was cut because it remained background/event-copy and included a reporter checklist in the draft body.
- `sent back`: exercised during the workbench flow, but no draft remained in final sent-back status after the weak Spanish-English draft was cut.

## Output Paths

- Local static output: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Copied local output: `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/site-output-copy`
- ZIP output: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- ZIP extract: `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/site-package-extract`
- Live fetch copy: `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/here-now-fetch`

## Evidence

- Visibility report: `test-comms/reports/20260630-cleanroom-e2e-6e2ac5b-visibility-attempt-9.md`
- Clean wipe/install: `test-comms/evidence/20260630-cleanroom-e2e-6e2ac5b/00-clean-wipe-summary.json`, `01-install-launch-summary.json`
- Runtime setup: `05-ai-service-poll.json`, `07-runtime-install-poll.json`, `08a-runtime-folder-state.json`, `11-model-download-poll.json`
- Source breadth: `15a-source-summary.json`
- Daily Scan: `17-daily-scan-progress-*.png`, `20-daily-scan-final.json`
- Workbench approval/navigation: `31-first-draft-approval-warning-modal.png`, `32-first-draft-approved.json`, `34-publishing-reachable-after-approval.json`
- Publish/compile: `59-after-compile.json`, `62-after-here-publish.json`
- Live fetch and scan: `67-live-httpclient-fetch.json`, `68-final-output-quality-scan.json`
- Machine profile: `69-machine-profile-final.json`

## Findings

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### Minor - Daily Scan Did Not Yield Five Honest Public Stories

Observed: Daily Scan produced 19 leads, but after testing the strongest available items only two were suitable for public output. Several leads were generic source-intake notes, duplicate/background event notes, keyword detections needing verification, or low-novelty watch items. Generated weak drafts were held or cut instead of published.

Expected: The directive asked the tester to try to produce at least 5 reader-facing stories or briefs without padding.

Impact: The release path works, but the seeded Longmont scan content did not support five honest reader-facing stories in this run.

Repro: Clean install attempt 9, onboard Longmont, run Daily Scan, inspect Story Queue and generated drafts.

## Request For Coder

No blocker from this pass. The attempt-8 runtime install regression appears fixed. Consider improving Daily Scan lead quality if the product goal is for a clean first-run Longmont scan to reliably produce five publishable briefs without relying on weak/background source-intake items.
