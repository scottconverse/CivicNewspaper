# Tester Report - v0.3.1 Stage 5 Cleanroom

Date: 2026-06-30 / 2026-07-01 UTC
Tester machine: Windows 10 Home 10.0.26100.1, Intel Core i7-13620H, 16 GB RAM, Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Directive: test-comms/directives/20260630-stage5-cleanroom-v031-1273bc7.md
Product branch: main
Product commit represented by installer: 1273bc73ea660be6852a9ced6b3954fc494b5e29
App version: 0.3.1

## Verdict

FAIL.

Install, app-guided local AI setup, model download, source seeding, scan, draft/editor state mechanics, local compile, ZIP export, and here.now connector publish all worked without manual dependency installation.

The run fails Stage 5 readiness because the generated/published issue did not reach the requested 5-10 reader-facing stories and the public output quality is not acceptable:

- Only 3 approved articles compiled and published.
- Two public article pages contain the editor approval note as the entire article body: `Approved during cleanroom mechanics test despite quality warnings; see tester report.`
- All 3 public article pages report `No source links were attached to this article.`
- One generated held draft made a serious unsupported claim that Georgia Boys BBQ was the chosen vendor for public library roof work while its linked citations were Downtown Longmont events snippets.

## Environment

- Windows version: Windows 10 Home, 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free during final check: 372410556416 bytes on C:
- Node used for automation: v24.14.0 from Codex bundled runtime
- npm: not on PATH
- Rust: not on PATH
- Ollama before clean wipe: prior product-managed state existed under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime`
- Ollama after app-guided setup: app-managed `ollama.exe` running from `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Model present after setup: `phi4-mini:latest`

## Artifact Verification

Installer:

`test-comms/artifacts/20260630-stage5-cleanroom-v031-1273bc7/The Civic Desk_0.3.1_x64-setup.exe`

Expected SHA256:

`12FF893863684996045A6802406698D825CA6B411006B5355AC8F5C2A4B319B6`

Observed SHA256:

`12FF893863684996045A6802406698D825CA6B411006B5355AC8F5C2A4B319B6`

Expected size:

`5633364`

Observed size:

`5633364`

Result: PASS.

## Steps Run

Key commands and actions:

```powershell
git pull --ff-only
Get-Content -Raw test-comms\ACTIVE_DIRECTIVE.md
Get-FileHash -Algorithm SHA256 -LiteralPath 'test-comms\artifacts\20260630-stage5-cleanroom-v031-1273bc7\The Civic Desk_0.3.1_x64-setup.exe'
(Get-Item -LiteralPath 'test-comms\artifacts\20260630-stage5-cleanroom-v031-1273bc7\The Civic Desk_0.3.1_x64-setup.exe').Length
Start-Process -FilePath "$env:LOCALAPPDATA\The Civic Desk\uninstall.exe" -ArgumentList '/S' -Wait -WindowStyle Hidden
Remove-Item -LiteralPath "$env:LOCALAPPDATA\The Civic Desk" -Recurse -Force
Remove-Item -LiteralPath "$env:LOCALAPPDATA\com.scottconverse.civicdesk" -Recurse -Force
Remove-Item -LiteralPath "$env:APPDATA\com.scottconverse.civicdesk" -Recurse -Force
Remove-Item -LiteralPath "$env:USERPROFILE\.ollama" -Recurse -Force
Start-Process -FilePath 'test-comms\artifacts\20260630-stage5-cleanroom-v031-1273bc7\The Civic Desk_0.3.1_x64-setup.exe' -ArgumentList '/S' -Wait -WindowStyle Hidden
Start-Process -FilePath "$env:LOCALAPPDATA\The Civic Desk\civicnews.exe"
```

For UI automation and evidence, I connected to the real installed app WebView target at `http://127.0.0.1:9333/json/list` using Playwright over CDP. I did not run the product from source and did not manually install Ollama, models, Node, Rust, or app dependencies.

Manual/UI steps performed through the installed app:

1. Confirmed true first-run onboarding after wiping `The Civic Desk`, `com.scottconverse.civicdesk`, and `.ollama` state.
2. Selected Longmont starter profile.
3. Allowed the app to start/install its local AI runtime.
4. Downloaded the recommended `phi4-mini:latest` model through the app.
5. Finished onboarding.
6. Ran Daily Scan.
7. Generated drafts from leads.
8. Exercised editor states:
   - Hold: saved hold note on one draft.
   - Send back: saved more-work note on one draft.
   - Cut: confirmed cut on one draft.
   - Approve: approved 3 drafts for publication despite warnings for mechanics testing.
9. Set a non-starter publication identity: `Longmont Civic Desk Cleanroom`.
10. Compiled the static site.
11. Published anonymously to here.now through the app connector.
12. Copied local output and ZIP into the evidence folder.
13. Checked live HTTP paths and scanned local public output for scaffolding/mojibake markers.

## Results

- App installed from verified artifact: PASS
- Real installed app launch: PASS
- Clean app-data/profile state: PASS after also removing `com.scottconverse.civicdesk`; initial wipe missed this hidden app-data root.
- Onboarding natural first-run state: PASS
- Unsigned installer messaging visible in app: PASS
- App-guided local runtime setup: PASS
- App-guided model download: PASS
- Manual dependency help needed: NO
- Sources seeded/imported: PASS, 19 starter Longmont sources
- Daily Scan: PASS, 68 evidence items found, 20 reviewed, 15 leads saved
- Draft generation: PARTIAL, 6 drafts created
- Editor hold/send-back/cut/approve paths: PARTIAL PASS, states were reachable, but dialogs can stack if controls are clicked quickly
- Publication volume target: FAIL, only 3 articles compiled/published
- Local compile: PASS, 3 articles, 20 files
- ZIP export: PASS
- here.now connector test: PASS
- here.now publish: PASS
- Public site HTTP checks: PASS for generated paths
- Public output quality: FAIL

## Counts

- Sources: 19
- Evidence items: 68
- Leads: 15
- Drafts: 6
- Draft statuses from local database:
  - `ready_to_publish`: 3
  - `hold`: 1
  - `needs_verification`: 1
  - `killed`: 1
- Approved/published stories/briefs: 3
- Publish run article count: 3
- Publish run files written: 20

## Publication Artifacts

- Local output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Copied local output evidence: `test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/published-site-copy/`
- Export ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- Copied ZIP evidence: `test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/site-package.zip`
- here.now URL: https://serene-aurora-nz5r.here.now
- Deployment ID: `slug=serene-aurora-nz5r;version=01KWDKSY4VEFNGWPF99A71HD21;created_slug=serene-aurora-nz5r`

Live HTTP checks:

- `/`: 200
- `/briefs/2.html`: 200
- `/briefs/3.html`: 200
- `/briefs/6.html`: 200
- `/feed.xml`: 200

## Evidence

Evidence folder:

`test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/`

Important evidence files include:

- `fresh-launch-after-full-state-wipe.png`
- `setup-step1-longmont-selected.png`
- `ai-service-wait-10s.png`
- `ai-service-wait-30s.png`
- `ai-service-wait-60s.png`
- `model-download-started.png`
- `model-download-final-or-latest.png`
- `workspace-after-onboarding.png`
- `daily-scan-started.png`
- `daily-scan-final-or-latest.png`
- `story-queue-after-scan.png`
- `first-generated-draft-fields.json`
- `editor-state-held-confirmed.png`
- `batch-draft-actions.json`
- `unsourced-generate-anyway-fields.json`
- `publishing-initial.png`
- `identity-saved.png`
- `compile-result.png`
- `connector-test-result.png`
- `connector-publish-result.png`
- `published-site-copy/`
- `site-package.zip`

## Quality Gate Scan

Scaffolding marker scan over generated text files found no literal matches for:

- `EDITOR_NOTE`
- `[EDITOR_NOTE`
- `Body:`
- `Headline:`
- `Nut graf`
- `Reporting Steps`
- `[Source needed]`
- `[Verification needed]`
- `[End of Report]`

Text-only mojibake marker scan found no generated text-file occurrences of:

- `U+00C2`
- `U+00C3`
- `U+00E2`
- `U+FFFD`

The ZIP binary itself contains byte sequences that decode as `U+FFFD` if forced through UTF-8; I did not count that as a public text mojibake failure.

## Findings

Severity counts:

- Blocker: 0
- Critical: 2
- Major: 3
- Minor: 2
- Nit: 0

### Critical - Public issue can compile pages with no reader-facing body

Observed:

`briefs/2.html` and `briefs/3.html` contain only the approval note as the article body:

`Approved during cleanroom mechanics test despite quality warnings; see tester report.`

Expected:

Approved public pages should contain reader-facing story copy or should be blocked from compile.

Impact:

Readers receive non-story internal/test/editor text instead of articles. This is a publication-quality failure.

Repro:

Generate and approve the Downtown Longmont Creative District and Summer Reading Challenge drafts during the Stage 5 workflow, compile, and inspect `briefs/2.html` and `briefs/3.html`.

### Critical - Generated draft asserted unsupported public-library roof/vendor facts

Observed:

The held draft titled `City Council Vote Approves Roof Work at Public Library` claimed Georgia Boys BBQ was the chosen vendor for public library roof work. The linked citations were Downtown Longmont event snippets, not a council packet, contract, vote, or library roof source.

Expected:

The draft should refuse, stay as verification notes, or require source-backed evidence before stating vendor/contract facts.

Impact:

This is a serious hallucination/source-mismatch risk in a civic/public-contract story.

Repro:

Draft the lead `Council vote scheduled for library roof contract`, generate the draft, and inspect `test-comms/evidence/20260630-stage5-cleanroom-v031-1273bc7/first-generated-draft-fields.json`.

### Major - Stage 5 target volume not met

Observed:

The app compiled and published 3 articles, not the requested 5-10 reader-facing stories or briefs.

Expected:

The workflow should support producing the requested reviewable publication volume when 15 leads are present, or clearly explain why only 3 are suitable.

Impact:

The product did not meet the directive's publication output target.

Repro:

Run the cleanroom flow through Daily Scan, draft/editor workflow, compile. Publish manifest reports `article_count: 3`.

### Major - Approved public pages show no source links

Observed:

All generated article pages include `Source links attached by the editor: No source links were attached to this article.`

Expected:

Reader-facing civic output should include source links when source evidence exists, or the app should block/flag publication more strongly.

Impact:

Published stories are not sufficiently auditable by readers.

Repro:

Inspect `briefs/2.html`, `briefs/3.html`, and `briefs/6.html` in the copied site output.

### Major - Verification/no-source lead generation can be slow/confusing

Observed:

For the `July Fourth Film Show at Longmont Museum` verification lead, the UI used `Generate anyway` instead of `Generate Draft` because no source documents were linked. My first automated attempt waited on the wrong action and exposed that this path is easy to miss.

Expected:

The UI should make the no-source path and next action unmistakable and should not encourage approval into public output without strong friction.

Impact:

Editors may approve weak or unsourced output.

Repro:

Open a verification lead with `Linked Sources (0)` and compare the action label/state to normal draft leads.

### Minor - First cleanroom wipe path was not obvious

Observed:

Removing `The Civic Desk` install/data and `.ollama` was not enough. The app still opened with previous Longmont state until I also removed:

- `C:\Users\civic\AppData\Local\com.scottconverse.civicdesk`
- `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`

Expected:

Cleanroom/reset documentation should identify the actual app-data roots.

Impact:

Testers can falsely think they have a clean profile.

Repro:

Uninstall/remove only `The Civic Desk` paths, relaunch, and observe old state remains.

### Minor - Editor confirmation dialogs can stack

Observed:

Clicking Hold and then Approve before closing the first dialog left both hold and publish-warning dialogs visible.

Expected:

Opening one editor decision modal should close or block other decision dialogs.

Impact:

The UI recovers, but the state is confusing and increases the chance of recording the wrong editor decision.

Repro:

Open a draft, click Hold, then click Approve for Static Publish before dismissing/saving the hold dialog.

## Request For Coder

Fix the output-quality blockers before another readiness claim:

1. Prevent compile/publish of approved drafts whose body is only an editor/test/approval note or otherwise empty.
2. Prevent civic claims from being generated against unrelated source evidence.
3. Require or strongly enforce source links for public civic articles.
4. Make the no-source verification lead path safer and clearer.
5. Document the real cleanroom app-data roots or add an in-app reset path for tester/user support.

Mechanics are close: installer, app-guided runtime/model setup, scan, compile, ZIP export, and here.now publish worked. The remaining failure is public-output trustworthiness and enough publishable story volume.
