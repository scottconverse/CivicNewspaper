# Tester Report - Output Trust Cleanroom Rerun d9cb382

Date: 2026-06-30 / 2026-07-01 UTC
Tester machine: Windows 10 Home 10.0.26100.1, Intel Core i7-13620H, 16 GB RAM, Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Coordination branch: test-comms/cleanroom-coder-tester
Directive: test-comms/directives/20260630-output-trust-rerun-d9cb382.md
Product branch: main
Product commit represented by installer: d9cb38210b3c71d0406219ed8c42f25212bcb977
App version: 0.3.1

## Verdict

FAIL overall.

The important output-trust regression gate improved: compile blocked approved drafts that had linked evidence but no inline `evidence:` citations, and after I rewrote one draft as a source-bound brief, the final local and here.now public output contained no tester notes, no approval-note-only article body, no scaffolding markers, no mojibake markers, and source links were attached.

The run still fails readiness because:

- The final reviewable publication had only 1 article, not the requested 5-10.
- Draft generation still produced severe junk and unsupported civic claims before the compile gate caught them.
- The only way to complete publication was to cut the junk draft and manually rewrite the remaining approved story into cautious source-bound copy.

## Environment

- Windows version: Windows 10 Home, 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Node used for automation: v24.14.0 from Codex bundled runtime
- App-managed runtime: `ollama.exe` under `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Model after app-guided setup: `phi4-mini:latest`
- Manual dependency help: No. I did not manually install Ollama, models, Node, Rust, or product dependencies.

## Artifact Verification

Installer:

`test-comms/artifacts/20260630-output-trust-rerun-d9cb382/The Civic Desk_0.3.1_x64-setup.exe`

Expected SHA256:

`F0558BE2E21EED4C83152E376E2FA8DDAFDB35D2CE657CFF4A798E2B8C0395BA`

Observed SHA256:

`F0558BE2E21EED4C83152E376E2FA8DDAFDB35D2CE657CFF4A798E2B8C0395BA`

Expected size: `5635913`

Observed size: `5635913`

Result: PASS.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and read `ACTIVE_DIRECTIVE.md`.
2. Verified installer SHA256 and byte size.
3. Stopped prior Civic Desk/Ollama processes.
4. Wiped directive-scoped state:
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%APPDATA%\com.scottconverse.civicdesk`
   - `%USERPROFILE%\.ollama`
5. Installed from the verified NSIS installer.
6. Launched the installed app from `%LOCALAPPDATA%\The Civic Desk\civicnews.exe`.
7. Completed Longmont first-run setup.
8. Used app-guided local runtime/model setup and downloaded `phi4-mini:latest`.
9. Ran Daily Scan.
10. Generated drafts, approved two to test the compile gate, cut one blocked/junk draft, rewrote one source-bound brief, compiled, exported ZIP/static site, and published anonymously to here.now.
11. Scanned local public output for regression markers, scaffolding markers, mojibake markers, missing source links, and live HTTP reachability.

## Results

- Clean first-run onboarding: PASS
- App-guided runtime setup: PASS, but required explicit app button/progression rather than immediate auto-start
- App-guided model download: PASS
- Starter Longmont sources: PASS, 19
- Daily Scan: PASS, 67 evidence items, 15 leads
- Draft creation: PARTIAL/FAIL, 2 drafts retained; generated output included one severe junk draft
- Editor states:
  - Cut: PASS, bad draft cut through UI
  - Hold/send-back/cut modal stacking check: PASS for this run; each action showed one modal and cancel cleared it
- Compile quality gate for missing inline evidence citations: PASS
- Final compile after editor rewrite: PASS, 1 article, 18 files
- ZIP export: PASS
- here.now publish: PASS
- Publication volume target: FAIL, only 1 article
- Final public-output marker scan: PASS

## Counts

- Sources: 19
- Evidence items: 67
- Leads: 15
- Drafts: 2
- Draft statuses:
  - `ready_to_publish`: 1
  - `killed`: 1
- Published stories/briefs: 1
- Publish run files written: 18

## Publication Artifacts

- Local output path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- Copied local output evidence: `test-comms/evidence/20260630-output-trust-rerun-d9cb382/published-site-copy/`
- Export ZIP path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default\site-package.zip`
- Copied ZIP evidence: `test-comms/evidence/20260630-output-trust-rerun-d9cb382/site-package.zip`
- here.now URL: https://sleek-wisdom-gesy.here.now
- Deployment ID: `slug=sleek-wisdom-gesy;version=01KWDRADQR2DT8ZM1D0EN787PS;created_slug=sleek-wisdom-gesy`

Live HTTP checks:

- `/`: 200
- `/briefs/1.html`: 200
- `/feed.xml`: 200

## Required Regression Checks

- Approval-note-only body blocked or absent from final public output: PASS. Final article body was reader-facing copy, not a tester/approval note.
- `Approved during cleanroom mechanics test`, `despite quality warnings`, `see tester report`, `mechanics test`, `tester report` absent from final public output: PASS.
- Lead-based public story with no source links attached: PASS in final output; the final article included source links.
- Lead-based public story with linked source material but no inline `evidence:` citation: PASS for gate behavior. Compile blocked both approved generated drafts with this exact error before I rewrote one.
- Materially different claim than cited source supports: FAIL during drafting, PASS in final output after manual rewrite. The app generated unsupported cancellation/funding/pandemic claims from city events material.
- Reporter scaffolding/internal markers absent from final public output: PASS.
- Mojibake marker code points absent from final text output: PASS.
- Duplicate-topic story separation: Not meaningfully testable in final output because only 1 article published.

## Findings

Severity counts:

- Blocker: 0
- Critical: 1
- Major: 2
- Minor: 1
- Nit: 0

### Critical - Draft generation still creates unsupported civic claims and junk text

Observed:

The app generated and allowed approval of drafts with serious unsupported claims, including:

- `Longmont Youth Center to Cancel 'Snacks and Antojitos' Program This Year Due To COVID Concerns`
- `Library Summer Programs at Risk Due To Funding Cuts`

The second draft body was roughly 188k characters of junk-like text beginning with `proof of research required by projects and reports...`.

Expected:

The writer workflow should not generate unsupported cancellation/funding/pandemic claims from event-listing evidence, and it should not produce massive junk text.

Impact:

The compile gate prevented public release, but the newsroom workflow still presents bad drafts as approvable and creates a high-risk editing burden.

Repro:

Run Daily Scan on the cleanroom Longmont starter sources, draft the high/medium-priority library/youth-program leads, and inspect `test-comms/evidence/20260630-output-trust-rerun-d9cb382/draft-approval-results.json` plus `draft-*-fields.json`.

### Major - Publication volume target still not met

Observed:

After compile gates and source-bound cleanup, only 1 reader-facing article compiled and published.

Expected:

Directive target was 5-10 reader-facing stories or briefs, or a clear product explanation for why that volume cannot be produced.

Impact:

The product does not yet demonstrate a complete reviewable issue from the cleanroom flow.

Repro:

Run the cleanroom flow through Daily Scan, draft approval, compile. The final manifest reports `article_count: 1`.

### Major - Compile gate works late, after bad drafts can be approved

Observed:

The app allowed approval of two drafts with linked evidence but no recognized inline `evidence:` citations. Compile later blocked them with:

`Public output quality gate failed for draft 2: lead-based draft has linked evidence but no inline evidence citations`

and then:

`Public output quality gate failed for draft 1: lead-based draft has linked evidence but no inline evidence citations`

Expected:

The app should ideally block or require repair at approval time, not only at compile time.

Impact:

The late gate prevents bad public output but still leaves editors thinking an item is approved until compile.

Repro:

Generate the two drafts described above, approve them, then open Publishing and click Compile site.

### Minor - Runtime setup message stays on failure until user advances

Observed:

During first-run setup, Step 2 reported it could not reach the AI service and showed `Install local AI runtime`. Advancing eventually reached model download and the app-managed model download succeeded.

Expected:

The setup flow should make the exact next app-guided action and state transition clearer.

Impact:

Not a blocker, but the flow is easy to misread as failed.

## Evidence

Evidence folder:

`test-comms/evidence/20260630-output-trust-rerun-d9cb382/`

Important evidence files:

- `fresh-launch.png`
- `setup-longmont-selected.png`
- `setup-ai-service-result.png`
- `model-download-result.png`
- `workspace-after-onboarding.png`
- `daily-scan-started.png`
- `daily-scan-final.png`
- `draft-approval-results.json`
- `draft-*-fields.json`
- `compile-result.png`
- `compile-after-cut-result.png`
- `approved-draft1-source-bound-rewrite.png`
- `compile-after-rewrite-result.png`
- `publish-after-rewrite-result.png`
- `editor-modal-checks.json`
- `published-site-copy/`
- `site-package.zip`

## Request For Coder

The compile-time output-trust gate is working and should stay. Next fixes should move the protection earlier:

1. Stop generating unsupported cancellation/funding/pandemic claims from event-listing evidence.
2. Reject or regenerate massive junk draft bodies before they can be saved or approved.
3. Enforce inline citation/source requirements at approval time, not only at compile.
4. Improve the first-run AI runtime setup state so users know when to click the app-guided install path.

After those fixes, rerun the same cleanroom flow and require 5-10 publishable, sourced, reader-facing stories before calling the release ready.
