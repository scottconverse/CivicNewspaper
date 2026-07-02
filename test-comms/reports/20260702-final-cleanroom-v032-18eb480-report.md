# Tester Report - Final Cleanroom v0.3.2 18eb480

Date: 2026-07-02T21:55:00Z
Tester machine: Windows 11 Intel cleanroom box
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit: 18eb4805a2d00e0b3efad670bfe041bde6d90724
Directive: test-comms/directives/20260702-final-cleanroom-v032-18eb480.md
Verdict: BLOCKED

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.4 GB on C:
- Node: not installed / not used
- Rust: not installed / not used
- npm: not installed / not used
- Ollama installed/running: app-guided runtime running as process 26932
- Models present: `phi4-mini:latest`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260702-final-cleanroom-v032-18eb480.md`.
3. Verified the directive installer artifact:
   - `test-comms/artifacts/20260702-final-cleanroom-v032-18eb480/The Civic Desk_0.3.2_x64-setup.exe`
   - SHA256 `14414BAA3CDF4C6DD0EA80630983F982BBAA749D353ACB7E953D475C5A4E6C8B`
   - size `5197270`
4. Stopped running `civicnews` / `ollama` processes as needed and wiped only directive-approved app/runtime state:
   - `%APPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%USERPROFILE%\.ollama`
5. Installed only the directive NSIS installer.
6. Launched installed app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Completed first-run setup with:
   - Publication: Longmont Cleanroom Beta Desk
   - Editor: Cleanroom Tester
   - City: Longmont
   - State: CO
8. Used only app-guided AI setup. It reached `Local AI ready` with `phi4-mini:latest`.
9. Let the app discover/import starter sources and run Daily Scan.
10. Generated one linked-source draft and one no-source/verification draft through the app UI.
11. Opened Publishing and tested the output-folder control before compile.
12. Stopped before compile/export/publish because there were 0 approved drafts and I could not approve the generated content under the directive's quality rule.

## Results

- Installer hash and size: PASS.
- Clean wipe: PASS, after stopping a locked product-owned `ollama.exe`.
- Native installed launch: PASS.
- First-run identity setup: PASS.
- App-created default site folder after setup: PASS.
- App-guided AI setup: PASS.
- Source discovery / Daily Scan mechanics: PASS.
- No-source verification assignment behavior: PASS. The no-source draft persisted as `needs_verification` with missing-evidence notes.
- Prior blocker recheck, output folder: PASS. The app opened a Windows folder picker directly at `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`; no missing-folder error appeared.
- Compile/export package: BLOCKED, because the app had `0 approved stories ready for the public package`.
- here.now publish and public output inspection: NOT RUN, blocked before compile/export/publish.

## Counts

- Sources: 9
- Daily scan runs: 1
- Daily scan leads: 20
- Leads: 23
- Evidence items: 26
- Lead-evidence links: 10
- Drafts: 2
- Approved drafts: 0
- Publish runs: 0
- Published posts: 0
- Verification tasks: 57

## Evidence

Evidence folder:

`test-comms/evidence/20260702-final-cleanroom-v032-18eb480/`

Key evidence:

- `install-clean-launch.log`
- `environment.json`
- `db-snapshot-after-ai-ready.txt`
- `db-snapshot-after-daily-scan-wait.txt`
- `drafts-full.jsonl`
- `final-db-summary.json`
- `screenshot-01-after-launch.png`
- `screenshot-03-after-identity-next.png`
- `screenshot-05-ai-runtime-install-wait90.png`
- `screenshot-08-story-queue-leads.png`
- `screenshot-14-second-draft-confirmation.png`
- `screenshot-15-after-verification-notes.png`
- `screenshot-19-drafts-tab-opened.png`
- `screenshot-24-publishing-output-folder.png`
- `screenshot-25-after-folder-control-click.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 2
- Minor: 1
- Nit: 0

### BLOCKER-1: No approvable draft, so compile/export/publish cannot proceed

Observed: Publishing showed `0 approved stories ready for the public package. Move at least one draft to approved/ready-to-publish before compiling.`

Expected: The cleanroom flow should produce at least one source-linked, attributed, reader-facing draft that can be approved under the directive quality bar.

Impact: Blocks compile/export, ZIP/package verification, here.now publishing, and public output inspection.

Repro: Complete setup, let the app run source discovery/Daily Scan, generate drafts from the visible leads, then open Publishing.

Evidence: `screenshot-24-publishing-output-folder.png`, `final-db-summary.json`.

### MAJOR-1: Linked-source draft lacks clear attribution and includes unsupported-looking claims

Observed: Draft `Upcoming Events at St. Vrain Valley Schools Board Meetings` included source markers, but did not include a plain attribution phrase such as "According to..." and included questionable claims about "School District 2 and Burlington Public Schools," "district office staff member and school counselors," and "senior volunteers" that I could not validate from the displayed evidence during the run.

Expected: Improve/draft output should be reader-facing, source-grounded, and clearly attributed before approval.

Impact: Tester cannot approve this copy without violating the directive.

Repro: Generate a draft for the linked-source St. Vrain Valley Schools lead.

Evidence: `drafts-full.jsonl`, `screenshot-19-drafts-tab-opened.png`.

### MAJOR-2: No-source draft includes invented source suggestions while correctly marked needs-verification

Observed: The chip-seal draft was correctly persisted as `needs_verification` with the note `No source documents are linked to this lead yet`, but its content suggested specific external outlets/people such as `ABC News 13 Longmont`, `KMGH Morning Edition`, and `Road Reporter Sam Mims`.

Expected: Verification assignments should avoid invented source/person suggestions and should remain bounded to concrete next reporting steps.

Impact: The guardrail state is correct, but the generated verification copy could mislead an editor.

Repro: Draft the no-source Chip Seal lead.

Evidence: `drafts-full.jsonl`, `screenshot-14-second-draft-confirmation.png`, `screenshot-15-after-verification-notes.png`.

### MINOR-1: Daily scan run remained marked in_progress in the database after leads existed

Observed: `daily_scan_runs` had one row with `run_status` `in_progress` and `completed_at` null, while `daily_scan_leads` and `leads` were populated.

Expected: Completed scan work should mark the run complete.

Impact: Could confuse status displays and downstream automation.

Repro: Run first cleanroom discovery / Daily Scan and inspect `daily_scan_runs`.

Evidence: `db-snapshot-after-daily-scan-wait.txt`.

## Request For Coder

The 18eb480 build appears to fix the default publish folder creation/opening blocker. Please focus next on the draft quality gate:

- Ensure linked-source drafts produce reader-facing, clearly attributed, source-grounded copy.
- Ensure no-source verification assignments do not invent specific sources, outlets, or people.
- Ensure Daily Scan run status completes when lead generation completes.

Once the app can produce one approvable source-linked draft, the tester can rerun compile/export, ZIP/package verification, here.now publish, and public-output inspection.
