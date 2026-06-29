# Tester Report - Full Clean-Wipe Longmont Publication E2E c3db2ac

Date: 2026-06-29 UTC
Tester machine: msi\civic cleanroom Windows laptop
Repo: https://github.com/scottconverse/CivicNewspaper.git
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: stable-readiness-local-gates
Product commit: c3db2aca6166787e6fb74daf8e1f91c8d8e3dbbb
Directive: test-comms/directives/20260629-full-cleanwipe-longmont-c3db2ac.md

Status: FAIL

The build passed the hard output checks and produced a real here.now publication, but the full directive does not pass because several required end-user workflows were unstable or incomplete: first-run identity setup fell back because input events were not received, identity editing routed badly, Workbench edit/save behaved incorrectly, and the advisor path could not be completed reliably because the UI repeatedly landed in a stale draft wizard.

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H, 10 cores / 16 logical processors
- RAM: 15.7 GB
- GPU: Intel UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.1 GB on C:
- Node: bundled Codex Node available for automation
- Rust: not available on PATH
- npm: not available on PATH
- Ollama installed/running after app setup: yes, app-owned runtime at `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Models present: qwen2.5:7b installed by the app-owned setup flow

Evidence: `test-comms/reports/20260629-full-cleanwipe-longmont-c3db2ac-evidence/environment.json`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read `test-comms/ACTIVE_DIRECTIVE.md`, README, protocol, tester prompt, and the active directive.
2. Verified installer hashes before install.
3. Clean-wiped CivicNewspaper app install/state, generated output folders, prior app data, Ollama process/runtime/cache/model state, and CivicNewspaper/Ollama user PATH entries within the directive boundary.
4. Installed the preferred NSIS artifact.
5. Launched the real Windows desktop app with WebView2 remote debugging enabled for observation.
6. Captured first-run state and followed the app-owned setup path.
7. Let the app install/start its bundled Ollama runtime and download `qwen2.5:7b`.
8. Ran source discovery and imported official/public social/community sources once; this import did not survive the later setup/profile transition.
9. Ran Daily Scan after model setup.
10. Generated five drafts from app-created Longmont leads using the Draft -> Generate Draft workflow.
11. Attempted Workbench edit/save, legal-risk advisor, and editor actions.
12. Compiled the site from the Publishing UI to the directive evidence output folder.
13. Saved and tested the here.now connector, then published anonymously to here.now as authorized.
14. Verified here.now homepage and all article pages returned HTTP 200.
15. Downloaded published HTML pages and ran the exact mojibake scanner and public `Draft:` prefix check against local output, ZIP extraction, and downloaded here.now pages.

## Results

- Installer hash verification: PASS.
- Clean wipe: PASS within current user privileges; no target paths remained after wipe.
- NSIS install: PASS, exit code 0.
- Real desktop launch: PASS.
- First-run/setup natural flow: PARTIAL/FAIL. The app displayed `The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile.`
- App-owned AI runtime/model setup: PASS. The app downloaded/installed its local runtime and `qwen2.5:7b` without manual tester dependency installation.
- Source discovery/import: PARTIAL. Discovery found official city sources, Facebook/community signals, Reddit, local media, and events; selected sources imported successfully once, but after setup/profile transition the app reverted to six starter sources.
- Daily Scan: PASS. Fresh scan produced 18 Longmont leads.
- Draft generation: PASS. Five drafts were generated via the app workflow.
- Writer/editor workflow: FAIL. Workbench edit/save was unstable; one save attempt unexpectedly opened the Kill confirmation modal, and a saved marker did not persist on reopen.
- Advisor workflow: FAIL. Attempts to run advisor repeatedly landed in or remained blocked by a stale draft wizard.
- Hold/kill non-publish item: FAIL/not completed. Kill modal appeared unexpectedly and was canceled to avoid dropping below the five-story target.
- Compile/export: PASS. Compile receipt showed 5 articles, 22 files, 0 skipped, ZIP package generated.
- here.now publish: PASS. Published to `https://zen-vow-kmmb.here.now`.
- here.now HTTP verification: PASS. Homepage and all five article pages returned 200.
- Mojibake exact scanner: PASS for local output, ZIP extraction, and downloaded here.now HTML.
- Public `Draft:` prefix check: PASS for local output, ZIP extraction, and downloaded here.now HTML.
- Publication identity: FAIL/Major. The compiled/published site still uses `My Local Publication` / `Local news and community information.` after identity-edit attempts misrouted.

## Output

- here.now URL: https://zen-vow-kmmb.here.now
- Local output folder: `test-comms/reports/20260629-full-cleanwipe-longmont-c3db2ac-evidence/publication-output/site/`
- ZIP/package: `test-comms/reports/20260629-full-cleanwipe-longmont-c3db2ac-evidence/publication-output/site/site-package.zip`
- ZIP SHA256: `DC2F4B37060A3F7D2A1A72F4FF0F8DB3DCD26BFDD4C1CFB107B53064A8E8D27A`
- Article count: 5
- Files written: 22
- Skipped count: 0
- Sources: 6 final starter sources
- Evidence items: 27
- Leads: 18
- Drafts: 5
- Draft statuses: 5 `ready_to_publish`
- Publish runs: 1
- Published posts: 5

## Evidence

Key JSON/log evidence:

- `installer-hashes.json`
- `clean-wipe-log.json`
- `install-result.json`
- `launch-result.json`
- `relaunch-after-model-download-exit.json`
- `environment.json`
- `final-db-state.json`
- `output-file-summary.json`
- `site-package-zip-sha256.json`
- `here-now-http-verification.json`
- `here-now-article-downloads.json`
- `exact-output-scans.json`

Key screenshots:

- `01-first-launch-desktop.png`
- `02-first-run-webview.png`
- `06-source-discovery-results.png`
- `08-source-imported.png`
- `18-ai-model-download-started.png`
- `22-ai-service-setup-progress-1.png`
- `22-ai-service-setup-progress-2.png`
- `24-fresh-scan-current-state.png`
- `33-after-fifth-draft.png`
- `34-open-first-draft-workbench.png`
- `43-workbench-edit-marker-after-save.png`
- `44-kill-modal-cancelled.png`
- `53-after-compile-click.png`
- `56-test-connection-result.png`
- `57-after-here-now-publish.png`
- `58-herenow-desktop.png`
- `59-herenow-mobile.png`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 4
- Minor: 2
- Nit: 0

### Major: First-run identity/setup input was missed and app fell back to starter Longmont profile

Observed: After relaunch during setup recovery, the app displayed: `The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile. You can edit identity later in Settings.`

Expected: First-run setup should reliably accept user input or keep the user in setup until complete.

Impact: Cleanroom users can silently inherit starter identity/source state instead of completing a deliberate newsroom setup.

Repro evidence: `21-relaunch-webview-state.png`, `22-ai-service-setup-progress-1.png`.

### Major: Identity editing routed badly and publication retained starter identity

Observed: Publishing warned that the publication name still used starter text. Attempting to edit identity navigated to Ethics & Backups, but subsequent field automation interacted with stale visible queue controls; the compiled site still published as `My Local Publication`.

Expected: Identity editing from Publishing should reliably expose editable identity fields and persist the masthead/tagline before compile.

Impact: Published output can carry starter identity even when tester attempts to set a non-starter publication name.

Repro evidence: `47-publishing-panel-initial.png`, `48-edit-identity-modal.png`, `51-publishing-before-compile.png`, `58-herenow-desktop.png`.

### Major: Workbench save path opened Kill confirmation and did not prove persisted edit

Observed: A visible Workbench edit/save attempt resulted in a `Kill this story?` confirmation modal, and the edit marker was not present after reopening the story.

Expected: `Save Draft` should persist the visible body text and should not trigger destructive editor actions.

Impact: Editors may lose confidence in the story editing path, and the directive-required writer workflow could not be cleanly proven.

Repro evidence: `42-workbench-edit-marker-before-save.png`, `43-workbench-edit-marker-after-save.png`, `44-kill-modal-cancelled.png`.

### Major: Advisor workflow could not be completed reliably

Observed: Attempting to open a draft and run the advisor repeatedly landed in, or remained trapped by, a draft wizard for another lead.

Expected: Opening an existing draft should show Workbench controls consistently, and `Run Advisor` should produce advisory-only risk notes.

Impact: The directive-required legal-risk advisor path was not completed.

Repro evidence: `45-advisor-story-open.png`, `46-advisor-after-run.png`.

### Minor: Source import did not survive setup/profile transition

Observed: Source discovery returned and imported a broader official/public/community source set, but after AI setup/profile transition the app returned to six starter sources.

Expected: Imported sources should persist across setup transitions, or the product should clearly explain that setup reset profile data.

Impact: The final publication used starter sources instead of the broader imported set.

Repro evidence: `08-source-imported.png`, `24-fresh-scan-current-state.png`, `final-db-state.json`.

### Minor: Initial app shell reported Local AI ready before model was actually downloaded

Observed: The sidebar displayed `Local AI ready / qwen2.5:7b`, but trying to generate a draft produced `Generating a draft requires the qwen2.5:7b model, which isn't downloaded yet.`

Expected: AI readiness should distinguish service reachability from model availability.

Impact: Users can be told AI is ready immediately before being redirected to setup.

Repro evidence: `02-first-run-webview.png`, `17-generate-draft-after-wait.png`.

## Request For Coder

Fix the setup/identity/workbench/advisor workflow instability, then issue another full clean-wipe directive. The output compiler/publisher path looks materially improved: 5-article compile, here.now publish, exact mojibake scan, ZIP scan, and public `Draft:` prefix scan all passed on this artifact.
