# Tester Report - Full Clean-Wipe Longmont 5a24a5a

Date: 2026-06-29 UTC
Tester machine: msi\civic cleanroom Windows laptop
Repo: https://github.com/scottconverse/CivicNewspaper.git
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: stable-readiness-local-gates
Product commit: 5a24a5a597b78907ca5d64019432c1468b3ff30a
Directive: test-comms/directives/20260629-full-cleanwipe-longmont-5a24a5a.md

Status: FAIL

The run failed before compile/export/publish. The app installed and began a real clean first-run setup, installed the app-owned local AI runtime/model, discovered/imported Longmont sources, ran scan workflows, and started draft/editor workflow. During Workbench edit/advisor handling the app WebView/process exited. On relaunch, the app returned to AI model setup and the final database was reset to zero sources, zero leads, zero drafts, and zero publish runs. That means the full E2E state did not survive the app exit and the required publication/identity/kill/output gates could not be completed.

## Environment

- Windows version: Microsoft Windows 11 Home
- CPU/GPU: same tester laptop used for prior cleanroom runs
- User: msi\civic
- App install path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App-owned runtime path observed during run: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\...`
- Manual dependency installation: none

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md`, README, protocol, tester prompt, and `test-comms/directives/20260629-full-cleanwipe-longmont-5a24a5a.md`.
3. Verified NSIS and MSI hashes.
4. Performed stronger clean wipe including:
   - stopped `civicnews.exe`
   - stopped app-owned `ollama.exe`
   - ran The Civic Desk uninstaller
   - removed Roaming and Local `com.scottconverse.civicdesk`
   - removed `C:\Users\civic\.ollama`
   - searched Local/Roaming for product-state folders
5. Installed preferred NSIS artifact.
6. Launched real desktop app with WebView2 debugging for observation.
7. Allowed app-owned setup to install local AI runtime and download `qwen2.5:7b`.
8. Ran Longmont source discovery and imported official/public/community sources.
9. Ran expanded Daily Scan with 19 sources.
10. Began draft generation and Workbench editor/advisor workflow.
11. App exited during Workbench edit/advisor handling.
12. Relaunched once; initial relaunch attempt failed to find the executable, then product search found it and second relaunch succeeded.
13. Relaunch returned to AI setup/model download state instead of the in-progress newsroom.
14. Final DB check showed no sources/leads/drafts/publish runs persisted.

## Results

- Hash verification: PASS.
- Clean wipe: PASS; no target paths remained after wipe.
- NSIS install: PASS; exit code 0.
- Real desktop launch: PASS.
- App-owned AI setup: PARTIAL PASS; the app installed runtime/model without tester help, but setup repeatedly displayed input-event fallback messages.
- Source discovery/import: PASS before reset; 21 candidates found, 13 selected/imported, 19 sources shown.
- Daily Scan: PASS before reset; expanded scan reached 30 leads.
- Draft generation/workbench: FAIL; app exited during Workbench edit/advisor handling.
- State persistence after exit: FAIL; final DB reset to 0 sources, 0 leads, 0 drafts, 0 publish runs.
- Publication identity gate: NOT COMPLETED because app reset before compile/publish.
- Kill/cut persistence: NOT COMPLETED because app reset before kill proof.
- Clean output path gate: NOT COMPLETED because app reset before Publishing output-path capture.
- Compile/export/ZIP: NOT COMPLETED.
- here.now publish: NOT COMPLETED.
- Mojibake/Draft-prefix/public identity output checks: NOT COMPLETED because no output was produced.

## Evidence

Evidence folder:

`test-comms/reports/20260629-full-cleanwipe-longmont-5a24a5a-evidence/`

Key files:

- `installer-hashes.json`
- `clean-wipe-log.json`
- `install-result.json`
- `launch-result.json`
- `relaunch-after-workbench-exit.json`
- `relaunch-second-attempt.json`
- `final-db-state.json`
- `final-process-state.json`

Key screenshots:

- `01-first-launch-desktop.png`
- `02-first-run-webview.png`
- `03-ai-setup-progress-1.png`
- `04-model-download-progress-1.png`
- `07-current-after-next-timeout.png`
- `11-discovery-results.png`
- `13-sources-after-import.png`
- `17-story-queue-30-leads.png`
- `20-after-draft-loop-timeout.png`
- `25-after-relaunch-state.png`

Important observed messages:

- `The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile.`
- `The setup screen still is not receiving input events, so The Civic Desk started the recommended model download automatically.`
- After relaunch: `AI Setup Step 3 of 5 Download AI Model ... Initializing download... 2.9%`

Final DB excerpt:

```json
{
  "sources": 0,
  "evidence_items": 0,
  "leads": 0,
  "daily_scan_leads": 0,
  "drafts": 0,
  "publish_runs": 0,
  "published_posts": 0,
  "verification_tasks": 0
}
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 2
- Minor: 1
- Nit: 0

### Blocker: App exited during Workbench edit/advisor flow and reset E2E state

Observed: While exercising Workbench edit/save/advisor/approval, the WebView target closed and no `civicnews.exe`/CDP target was reachable. After relaunch, the app returned to AI setup/model download state. Final DB contained zero sources, leads, drafts, or publish runs.

Expected: Workbench actions should not exit the app, and if the app exits, previously generated sources/leads/drafts should persist.

Impact: Full clean-wipe E2E cannot be certified. Publication identity, kill persistence, output path, compile/export, and here.now publish gates were unreachable.

Repro evidence: `20-after-draft-loop-timeout.png`, `relaunch-after-workbench-exit.json`, `relaunch-second-attempt.json`, `25-after-relaunch-state.png`, `final-db-state.json`.

### Major: Setup still reports missing input events and auto-continues

Observed: On first launch, setup displayed that it did not receive input events and continued with a starter Longmont profile. It later reported it still was not receiving input events and auto-started the recommended model download.

Expected: First-run setup should accept normal user input or clearly pause for user action. It should not require fallback auto-progression in a normal desktop run.

Impact: User-chosen identity setup is at risk and the run could not prove a natural first-run identity flow.

Repro evidence: `02-first-run-webview.png`, `03-ai-setup-progress-1.png`.

### Major: State reset after relaunch prevents required clean output-path and identity gates

Observed: The app had reached 19 imported sources and 30 leads, but after relaunch the DB had no content and the app returned to AI setup. The default output path could not be captured before reset.

Expected: App state should persist across relaunch once setup/scan/draft work has been performed.

Impact: The specific 5a24a5a gates for identity, kill persistence, and output path could not be completed.

Repro evidence: `13-sources-after-import.png`, `17-story-queue-30-leads.png`, `25-after-relaunch-state.png`, `final-db-state.json`.

### Minor: First relaunch attempt could not find the installed executable

Observed: The first relaunch command immediately after the app exit reported `The system cannot find the file specified` for `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`. A product-state search shortly afterward found the executable and the second relaunch succeeded.

Expected: Installed executable path should be stable immediately after an app exit.

Impact: This added uncertainty during failure recovery, though the executable did become available again.

Repro evidence: `relaunch-after-workbench-exit.json`, `relaunch-second-attempt.json`.

## Request For Coder

Fix the Workbench/advisor crash or exit and the post-exit state reset first. Then reissue the 5a24a5a-style directive so tester can complete the identity gate, kill persistence gate, clean output-path gate, compile/export, here.now publish, and public output scans.
