# Tester Report - Canonical Walkthrough v0.3.0 b0be4d7

Date: 2026-06-30 UTC
Tester machine: Windows cleanroom tester, user `civic`
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Product branch: main
Product commit: b0be4d7432e9f5f791da68770a9631b8c5892697
Directive: test-comms/directives/20260630-canonical-walkthrough-v030-b0be4d7.md
Lane: Walkthrough only

## Verdict

Walkthrough verdict: FAIL, evidence-backed.

First-run verdict: reaches core feature only after recovery, model install, identity edit, and starter-source path. Clean first-run empty-source coverage is INVALID because the app auto-continued identity setup and seeded a Longmont starter profile/sources before I could observe a no-source state.

Core feature reachability: PASS with caveats. I reached scan, lead, draft, Workbench, approval, static compile, ZIP export, and here.now preview publish.

Live preview: https://chilly-trellis-6rxq.here.now

## Environment Attestation

| Item | Result | Evidence |
|---|---|---|
| Installer hash | NSIS matched `6C28D0ACEDAA1A367CA8F2EBFFDCB60B2AFC002F123442D1C7FF84EFD1CC95E4`; MSI matched `AA510FA91B519883190638CBEDB584648B148731DB842371ECB8671D6D7CA154` | `00-context-and-hashes.json` |
| Windows/hardware/network | Captured | `01-windows-hardware.json`, `02-network-state.json` |
| Product-clean state | Removed app install/app data/model state; paths absent before launch | `04-clean-wipe-absent-proof.json` |
| App install | NSIS silent install exit code 0; app exe present | `05-install-result.json` |
| App data path | `%APPDATA%\com.scottconverse.civicdesk`; DB and WAL files created on launch | `06-launch-appdata-proof.json` |
| Ollama/model absent before launch | `.ollama` removed, no model files, no Ollama on PATH | `04-clean-wipe-absent-proof.json` |
| Populated DB state | 6 sources, 9 leads, 1 draft, 1 publish run, 1 published post | `44-db-final-state.json` |
| Static output | `index.html`, `watch/1.html`, `site-package.zip`, manifest written | `37-site-output-files.json`, `38-publish-manifest.json`, `41-site-package.zip`, `42-site-package-sha256.json` |
| here.now fetch | HTTP 200 for index and story page | `50-herenow-fetch-proof.json`, `52-herenow-watch-fetch-proof.json` |

## Steps Run

Commands and UI actions used:

```powershell
git -C C:\Users\civic\Desktop\CODE\civicnewspaper-test-comms pull --ff-only origin test-comms/cleanroom-coder-tester
Get-FileHash "test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/The Civic Desk_0.3.0_x64-setup.exe" -Algorithm SHA256
Get-FileHash "test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/The Civic Desk_0.3.0_x64_en-US.msi" -Algorithm SHA256
Remove-Item clean app/test/model paths
Start-Process "The Civic Desk_0.3.0_x64-setup.exe" -ArgumentList "/S" -Wait
Invoke-WebRequest -UseBasicParsing https://chilly-trellis-6rxq.here.now
Invoke-WebRequest -UseBasicParsing https://chilly-trellis-6rxq.here.now/watch/1.html
```

UI path:

1. Launched installed desktop app as normal user.
2. Observed first-run AI setup recovery screen.
3. Skipped initial model download once, then entered starter Longmont app state.
4. Ran Daily Scan.
5. Drafted a lead; first attempt redirected to model setup because qwen2.5:7b was not installed.
6. Downloaded/installed qwen2.5:7b through app flow.
7. Drafted again; Workbench opened.
8. Approved the story after ticking editor responsibility attestation.
9. Edited publication identity from starter text to `Longmont Walkthrough Gazette`.
10. Compiled static site, exported ZIP, published here.now preview.
11. Stopped Ollama and captured app dependency-stopped status.

## Provisioning Matrix

| Cell | Result | Evidence |
|---|---|---|
| first-run + dependency absent + empty data + online | Partially covered, INVALID for empty sources. App had no app data/Ollama/model before launch, but first-run auto-continued and seeded 6 starter sources. | `04-clean-wipe-absent-proof.json`, `07-first-run-ai-setup.*`, `11-defaults-next.*`, `12-daily-scan-empty-or-starter-sources.*` |
| first-run + dependency absent + empty data + offline/source-failure simulation | Not verified. I did not disable system networking. Daily Scan did preserve unusable model/source output as review material. | `13-daily-scan-progress-05.*`, `14-story-queue-leads.*` |
| returning user + dependency present + populated data + online | Covered. qwen2.5:7b installed, draft generated, approved, compiled, ZIP exported, here.now published. | `19-ai-model-download-progress-09.*`, `23-generate-draft-after-model-progress-03.*`, `27-workbench-approve-result.*`, `37-site-output-files.json`, `50-herenow-fetch-proof.json` |
| returning user + dependency absent/stopped + populated data + online | Covered. After stopping Ollama, app showed `Local AI offline`, disabled model download, and offered runtime install/retry/system status actions. | `63-after-stop-ollama-processes.json`, `64-ai-model-after-ollama-stopped.*` |

## Readiness By Area

| Area | Result | Notes |
|---|---|---|
| Installer/hash | Pass | Preferred NSIS hash matched; install succeeded. |
| Clean first-run attestation | Fail | Clean filesystem state proven, but app skipped natural identity entry and seeded starter data. |
| Onboarding | Fail | Input recovery path auto-continued with starter Longmont profile. |
| Dependency absent guidance | Mixed | App guided runtime/model setup, but status showed `Local AI ready` while draft generation said model missing. |
| Daily Scan | Pass with caveat | Produced 9 leads; top lead preserved unusable model JSON/source material. |
| Story Queue | Pass | Leads and draft actions reachable. |
| Workbench | Pass with quality caveat | Draft generated and approval worked, but generated story was weak/source-page based. |
| Publishing/export | Pass | Static HTML, ZIP, and manifest written. |
| here.now | Pass | Connector produced `https://chilly-trellis-6rxq.here.now`; HTTP 200 verified. |
| Public output quality | Fail | No literal `EDITOR_NOTE` leaked, but story is a thin background item based on source-page copy, not a strong civic story. |

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 5
- Minor: 2
- Nit: 0

1. Major - First-run identity setup auto-continued instead of accepting normal user setup.
   - Observed: App reported the setup screen did not receive input events and continued with a starter Longmont profile.
   - Expected: A brand-new user can enter and save publication identity normally.
   - Impact: Clean first-run identity coverage is invalid.
   - Evidence: `07-first-run-ai-setup.*`, `08-ai-service-after-wait.*`
   - Suggested test: Fresh install with no app data; verify keyboard/mouse entry into every identity field before any auto-continue.

2. Major - Empty-source state was not observable.
   - Observed: After onboarding, Sources already contained 6 starter Longmont sources.
   - Expected: Clean empty-data state should show no sources or explicitly ask the user to choose starter sources.
   - Impact: Required no-source Daily Scan behavior could not be verified.
   - Evidence: `11-defaults-next.*`, `12-daily-scan-empty-or-starter-sources.*`, `44-db-final-state.json`
   - Suggested test: Add a first-run option to start empty, then verify Daily Scan routes to source setup.

3. Major - AI readiness status contradicted actual model availability.
   - Observed: Header showed `Local AI ready qwen2.5:7b`, but draft generation said qwen2.5:7b was not downloaded and redirected to setup.
   - Expected: Status should distinguish runtime ready from selected model installed.
   - Impact: User may believe drafting is available when it is not.
   - Evidence: `15-draft-from-lead-result.*`, `16-generate-draft-progress-00.*`, `17-ai-model-redirect-state-05.*`
   - Suggested test: Clean install, skip model, verify header and draft button state agree.

4. Major - Daily Scan AI extraction produced unusable JSON/source-material leads.
   - Observed: The first lead says the model did not return usable JSON and preserved source evidence for editor review.
   - Expected: App should either generate structured civic leads or clearly separate raw-source fallback from normal leads.
   - Impact: Queue contains low-value/raw review items and can lead to weak drafts.
   - Evidence: `13-daily-scan-progress-05.*`, `14-story-queue-leads.*`
   - Suggested test: Run Daily Scan on the starter source set and assert structured lead JSON success rate.

5. Major - Generated/published story is weak and background-like.
   - Observed: The final story says Longmont residents can access bookmarked agenda videos, with source-page snippets as notes. It is not a strong current civic story.
   - Expected: Publishable output should be a verified civic news item, not source-page background copy.
   - Impact: Mechanics pass, but reader-facing quality is not ready.
   - Evidence: `24-workbench-generated-draft.*`, `39-public-watch-1.html`, `45-public-watch-1-excerpts.txt`, `53-herenow-watch-1.html`
   - Suggested test: Add a quality gate rejecting drafts with no current action, no decision, or only generic source-page copy.

6. Minor - Public identity appears saved in publishing output, but settings table still retained starter name.
   - Observed: Public site used `Longmont Walkthrough Gazette`; DB settings still included `identity.newsroom_name: My Local Publication`.
   - Expected: Stored identity should be consistent across app/profile/settings surfaces.
   - Impact: Future relaunch or diagnostics may show stale identity.
   - Evidence: `32-identity-save-result.*`, `33-publishing-after-identity-save.*`, `44-db-final-state.json`
   - Suggested test: Save identity, relaunch, verify all visible and DB identity stores agree.

7. Minor - Generated story had internal note text in Workbench before publish.
   - Observed: Workbench draft included an `EDITOR_NOTE`; public output did not include the literal token.
   - Expected: Editor-only notes should be visually separated from article body before approval.
   - Impact: Editors may approve internal notes accidentally; prior runs showed public leakage risk.
   - Evidence: `24-workbench-generated-draft.*`, `43-public-output-quality-audit.txt`
   - Suggested test: Generate draft with editor notes; assert notes are stored outside body and never serialized into public HTML.

## What Worked

- Artifact hashes matched.
- Clean install and app-data creation were proven.
- Unsigned beta notice was visible and understandable.
- App-guided model setup recovered from missing model and installed qwen2.5:7b.
- Daily Scan produced leads from starter sources.
- Draft, Workbench, approval, static compile, ZIP export, and here.now publish all completed.
- Stopping Ollama after population correctly changed the app to `Local AI offline` and offered recovery controls.

## What Could Not Be Verified

- A true empty-source first-run path.
- Offline/no-network first-run behavior.
- Narrow/mobile layout; this walkthrough used the desktop app window only.
- Full UI control exhaustion. I captured all major top-level surfaces, but did not exhaustively click every low-level copy/export/subscriber control.

## Evidence Index

Evidence is under:

`test-comms/artifacts/20260630-canonical-walkthrough-v030-b0be4d7/evidence/`

Key evidence files:

- `00-context-and-hashes.json`
- `01-windows-hardware.json`
- `02-network-state.json`
- `04-clean-wipe-absent-proof.json`
- `05-install-result.json`
- `06-launch-appdata-proof.json`
- `07-first-run-ai-setup.*`
- `11-defaults-next.*`
- `12-daily-scan-empty-or-starter-sources.*`
- `14-story-queue-leads.*`
- `17-ai-model-redirect-state-05.*`
- `19-ai-model-download-progress-09.*`
- `23-generate-draft-after-model-progress-03.*`
- `24-workbench-generated-draft.*`
- `27-workbench-approve-result.*`
- `33-publishing-after-identity-save.*`
- `37-site-output-files.json`
- `39-public-watch-1.html`
- `41-site-package.zip`
- `43-public-output-quality-audit.txt`
- `44-db-final-state.json`
- `45-public-watch-1-excerpts.txt`
- `50-herenow-fetch-proof.json`
- `52-herenow-watch-fetch-proof.json`
- `53-herenow-watch-1.html`
- `54-dark-signals.*`
- `55-verification.*`
- `57-sources.*`
- `58-ai-model.*`
- `59-browser-pairing.*`
- `64-ai-model-after-ollama-stopped.*`

## Request For Coder

Fix first-run identity input/auto-continue, provide a true empty-source first-run path, make AI/model readiness states consistent, and add a public-output quality gate for weak background/source-page drafts before Full begins.
