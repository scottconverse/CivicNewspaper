# Tester Report - Final Cleanroom v0.3.2 916653b

Date: 2026-07-02T22:43:00Z
Tester machine: Windows 11 Intel cleanroom box
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit: 916653b87e09814d4c42bdcb31f91ca7ac4fae09
Directive: test-comms/directives/20260702-final-cleanroom-v032-916653b.md
Verdict: BLOCKED

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 15.7 GB
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Node: not installed / not used
- Rust: not installed / not used
- npm: not installed / not used
- Ollama installed/running: app-guided runtime reached ready state
- Models present: `phi4-mini:latest`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester`.
2. Reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and `test-comms/directives/20260702-final-cleanroom-v032-916653b.md`.
3. Verified the directive installer artifact:
   - `test-comms/artifacts/20260702-final-cleanroom-v032-916653b/The Civic Desk_0.3.2_x64-setup.exe`
   - SHA256 `F1DD475B97F497241DEDF00F48EBCC7A59318A7FFE3994E0030183072026DE54`
   - size `5200358`
4. Stopped running `civicnews` / `ollama` processes as needed and wiped only directive-approved app/runtime state:
   - `%APPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\com.scottconverse.civicdesk`
   - `%LOCALAPPDATA%\The Civic Desk`
   - `%USERPROFILE%\.ollama`
5. Installed only the directive NSIS installer.
6. Launched installed app from `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Completed first-run setup with the Longmont, CO starter profile.
8. Used only app-guided AI setup. It reached `Local AI ready` with `phi4-mini:latest`.
9. Let the app discover/import starter sources and run Daily Scan.
10. Waited for Daily Scan completion and inspected the DB.
11. Generated three drafts from three different leads:
    - two linked-source leads,
    - one no-source/background lead routed to verification notes.
12. Opened Workbench and attempted to open a generated linked-source draft.
13. Opened Publishing and tested the output-folder control before compile.
14. Stopped before compile/export/publish because approved draft count was 0 and Publishing was blocked.

## Results

- Installer hash and size: PASS.
- Clean wipe: PASS.
- Native installed launch: PASS.
- First-run Longmont setup: PASS.
- App-created default site folder after setup: PASS.
- App-guided AI setup: PASS.
- Source discovery / Daily Scan mechanics: PASS.
- Daily Scan stale `in_progress` recheck: PASS. Newest run completed after leads were present.
- No-source verification assignment behavior: PASS. No-source draft persisted as `needs_verification` with missing-evidence notes and did not invent named outlets/reporters/people.
- Linked-source attribution/citation fallback: PASS. Linked-source drafts used `According to the linked source` and valid `[Source](evidence:3)` syntax. No malformed `[Source(evidence:...)]` appeared.
- Open folder before compile: PASS. Windows Explorer opened the default site folder.
- Improve for Publication: BLOCKED. Opening a draft from Workbench repeatedly rendered a blank Workbench state, so I could not verify or run Improve for Publication.
- Compile/export package: BLOCKED. There were 0 approved drafts.
- here.now publish and public output inspection: NOT RUN, blocked before compile/export/publish.

## Counts

- Sources: 18
- Daily scan runs: 1
- Daily scan leads: 20
- Leads: 23
- Evidence items: 31
- Lead-evidence links: 22
- Drafts: 3
- Approved drafts: 0
- Publish runs: 0
- Published posts: 0
- Verification tasks: 94

Newest `daily_scan_runs` row:

```json
{
  "id": 1,
  "started_at": "2026-07-02T22:21:37.923226200+00:00",
  "completed_at": "2026-07-02T22:23:41.863269200+00:00",
  "run_status": "completed"
}
```

Observed paths:

- Installed app path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- App data path: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk`
- Default site folder: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\sites\default`
- ZIP/local artifact path: none; blocked before export
- here.now URL: none; blocked before publish

## Generated Drafts Considered For Approval

### Draft 1

- id: 1
- lead_id: 23
- status: `needs_verification`
- title: `Longmont Summer Concert Series: 2MX2 on Independence Day Weekend: The City of Longmont's summer`

```text
According to the linked source, View Events Summer Concert Series: 2MX2 Thursday, July 2 • 7 pm - 8:30 pm 400 Quail Rd. [Source](evidence:3).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It is source-bound and attributed, but persisted as `needs_verification` and Workbench listed it as `Sent back / needs work`.

### Draft 2

- id: 2
- lead_id: 21
- status: `needs_verification`
- title: `Multilingual service announcements: The City Clerk announced that English-only email`
- missing evidence note: `No source documents are linked to this lead yet. Treat this as a verification assignment until public source material is attached or cited.`

```text
No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.
```

Decision: not approved. This is correct no-source verification-assignment behavior.

### Draft 3

- id: 3
- lead_id: 22
- status: `needs_verification`
- title: `Independence Day Festival and Concerts in Longmont: The City of Longmont will host its`

```text
According to the linked source, View Events Summer Concert Series: 2MX2 Thursday, July 2 • 7 pm - 8:30 pm 400 Quail Rd. [Source](evidence:3).

This is a watch brief for Longmont readers. The linked source does not, by itself, confirm a broader development; watch for a newly posted date, vote, cost, agency response, or other public update before expanding it into a full story.
```

Decision: not approved. It is source-bound and attributed, but persisted as `needs_verification` and Workbench listed it as `Sent back / needs work`.

## Evidence

Evidence folder:

`test-comms/evidence/20260702-final-cleanroom-v032-916653b/`

Key evidence:

- `install-clean-launch.log`
- `environment.json`
- `db-after-ai-ready.txt`
- `db-after-scan-wait120.txt`
- `drafts-full.jsonl`
- `final-db-summary.json`
- `screenshot-01-launch.png`
- `screenshot-02-after-longmont-next.png`
- `screenshot-03-ai-ready-wait120.png`
- `screenshot-04-story-queue-leads.png`
- `screenshot-07-draft-gate.png`
- `screenshot-14-second-draft-gate.png`
- `screenshot-15-after-verification-notes.png`
- `screenshot-24-lead22-gate.png`
- `screenshot-25-after-lead22-generate.png`
- `screenshot-27-workbench-scroll.png`
- `screenshot-28-open-lead22-draft.png`
- `screenshot-29-publishing-screen.png`
- `screenshot-30-publishing-lower.png`
- `screenshot-31-after-open-folder.png`

## Findings

Severity counts:

- Blocker: 2
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

### BLOCKER-1: No approvable draft, so compile/export/publish cannot proceed

Observed: All three generated drafts persisted as `needs_verification`, Workbench listed them as `Sent back / needs work`, and `approved_drafts` was 0.

Expected: The cleanroom flow should produce at least one source-linked, attributed, reader-facing draft that can be approved under the directive quality bar.

Impact: Blocks compile/export, ZIP/package verification, here.now publishing, and public output inspection.

Repro: Complete setup, run Daily Scan, generate drafts for linked-source leads 22/23, then inspect Workbench/DB/Publishing.

Evidence: `drafts-full.jsonl`, `final-db-summary.json`, `screenshot-27-workbench-scroll.png`, `screenshot-30-publishing-lower.png`.

### BLOCKER-2: Opening a generated draft renders a blank Workbench state

Observed: After opening a generated draft from Workbench, the Workbench page rendered blank except for the page header and lower placeholder cards. Restarting the installed app restored navigation, but opening a draft reproduced the blank state.

Expected: Opening a draft should show the draft editor and controls, including any Improve for Publication or approval path available to the editor.

Impact: Blocks verification of Improve for Publication and prevents the tester from approving even source-bound fallback drafts through the UI.

Repro: Generate a draft, go to Workbench, click Open on the generated draft.

Evidence: `screenshot-27-workbench-scroll.png`, `screenshot-28-open-lead22-draft.png`, `screenshot-25-after-lead22-generate.png`.

### MAJOR-1: Publishing is also paused by starter public identity

Observed: Publishing showed `Publication My Local Publication` and warned that the publication name still uses starter text. The first-run Longmont starter path populated starter identity values instead of the directive's suggested `Longmont Cleanroom Beta Desk` / `Cleanroom Tester` identity.

Expected: The first-run setup should either collect the requested identity values or make the required identity edit unavoidable before publish.

Impact: Even with approved drafts, Publishing would remain paused until identity is edited.

Evidence: `screenshot-29-publishing-screen.png`, `final-db-summary.json`.

### MINOR-1: Two different linked-source leads produced identical fallback copy

Observed: Drafts 1 and 3 came from different leads but generated identical fallback content using evidence 3.

Expected: Source-bound fallback is preferable to unsupported claims, but different source-linked leads should ideally either use their own linked evidence or clearly explain that the same source is the only usable evidence.

Impact: This did not create public duplicate-topic output because nothing was approved or published, but it remains a likely quality issue once compile/publish is unblocked.

Evidence: `drafts-full.jsonl`.

## Request For Coder

The 916653b build improved several previous blocker areas:

- Daily Scan no longer remained `in_progress`.
- Linked-source fallback copy is attributed and uses valid evidence citation syntax.
- No-source verification assignment copy no longer invents named outlets/reporters/people.
- The default output folder opens successfully.

Please focus next on the remaining release blockers:

- Make Workbench open generated drafts reliably.
- Provide a clear Improve for Publication / approval path for source-linked, attributed fallback drafts when they are safe enough to publish.
- Ensure first-run identity or Publishing identity review does not leave starter text blocking publish in the cleanroom flow.
- Avoid identical fallback copy for distinct linked-source leads where possible.

Once one source-linked attributed draft can be improved/approved, the tester can rerun compile/export, ZIP/package verification, here.now publish, and public-output inspection.
