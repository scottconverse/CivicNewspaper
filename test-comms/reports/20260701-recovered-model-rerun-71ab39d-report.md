# Tester Report - recovered model rerun 71ab39d

Date: 2026-07-01T07:08:00Z
Tester machine: Windows 11 Intel/NVIDIA laptop cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 71ab39d3a8f5b6c947946b6b5af6862064dc8c94
Directive: test-comms/directives/20260701-recovered-model-rerun-71ab39d.md

## Environment

- Windows version: Microsoft Windows 11 Home 10.0.26200, 64-bit
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16 GB
- Runtime/model: product-managed Ollama runtime with `phi4-mini:latest`
- Node/Rust/npm: not on PATH; no tester-installed product prerequisites were used

## Summary

FAIL at downstream Workbench/drafting render gate.

The recovered setup path passed the previously blocked model gate. The app installed from the directive artifact, launched as a visible native desktop app, auto-installed the product-owned Ollama runtime, automatically started the recommended model pull, installed `phi4-mini:latest`, and reached the main dashboard with `Local AI ready`.

The app then completed source discovery/scan far enough to create 19 sources and 22 leads. However, the downstream writer/editor path is blocked. After opening the Workbench/draft path for the council roof contract lead and invoking `Generate Verification Notes`, the main content area went blank while the app process remained alive and the `Local AI ready` banner stayed visible. The blank content state persisted after another 60+ seconds and after attempts to return to Workbench/open the draft. Because the directive says to stop at the next exact blocker, I did not continue to approval, export, here.now publish, or output-quality audit.

## Steps Run

1. Fetched and fast-forwarded `test-comms/cleanroom-coder-tester`.
2. Read `test-comms/ACTIVE_DIRECTIVE.md` and confirmed it pointed to this directive.
3. Verified installer:
   - Path: `test-comms/artifacts/20260701-recovered-model-rerun-71ab39d/The Civic Desk_0.3.1_x64-setup.exe`
   - Expected/observed SHA256: `43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04`
   - Expected/observed size: `5632526`
4. Wrote and pushed visibility report `test-comms/reports/20260701-recovered-model-rerun-71ab39d-visibility.md`.
5. Product clean wipe was performed for Civic Desk install, app data, output folders, and product-managed Ollama/model state.
6. Installed the NSIS package silently.
7. Launched installed `civicnews.exe` normally.
8. Confirmed visible native launch without handle manipulation.
9. Confirmed recovered setup path reached runtime/model setup.
10. Watched recovered model setup at 10, 30, 60, and 120 seconds.
11. Confirmed `ollama list` showed `phi4-mini:latest` by the 30-second checkpoint and still at 120 seconds.
12. Confirmed app reached main dashboard with `Local AI ready`.
13. Continued into source discovery/scan.
14. Confirmed app showed 19 sources, 22 leads, and 4 high-priority leads.
15. Opened the Story Queue/lead path and reached a lead workflow panel for a council roof contract lead.
16. Invoked `Generate Verification Notes`.
17. Observed the app content area go blank while process/runtime remained alive.
18. Waited more than 60 additional seconds and captured final diagnostics.
19. Tried normal Workbench navigation/reopen attempts; content remained blank.

## Results

- Installer hash and size: PASS.
- Clean product wipe and silent install: PASS.
- Visible native launch: PASS.
- Recovered Step 1/Step 2 path: PASS.
- Identity persistence: PASS.
- Product-owned runtime auto-install: PASS.
- Recovered model pull auto-start: PASS.
- Model installed through product flow: PASS.
- App remains visible during model setup: PASS.
- Source discovery/scan reaches enough leads: PASS. DB showed 19 sources and 22 leads.
- Writer/draft path creates draft record: PARTIAL. DB showed 1 draft created.
- Workbench/editor remains usable after verification/draft action: FAIL.
- Full Longmont E2E publish flow: BLOCKED.
- Export ZIP, here.now publish, public output checks, duplicate-topic audit, mojibake audit: NOT RUN due blocker.

## Key DB State

After the blocker:

- `sources_count`: 19
- `leads_count`: 22
- `daily_scan_runs_count`: 1
- `drafts_count`: 1
- `publish_runs_count`: 0
- `published_posts_count`: 0

The single draft record was:

```json
{
  "lead_id": 20,
  "format": "watch",
  "title": "Council Vote on Library Roof Contract: The council agenda includes voting for roof work at",
  "content": "No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.",
  "status": "draft_generated"
}
```

That draft-safety behavior is useful, but the UI became unusable before the tester could continue the Workbench/editor workflow.

## Evidence

Evidence folder:

`test-comms/evidence/20260701-recovered-model-rerun-71ab39d/`

Key files:

- `cleanwipe-install-launch.log`
- `recovered-model-watch.txt`
- `db-snapshot-after-recovered-model-watch.json`
- `db-snapshot-after-workbench-blank.json`
- `runtime-diagnostics-after-blank.txt`
- `process-commandlines-after-workbench-blank.txt`
- `appdata-roaming-after-workbench-blank.json`
- `community_profile-final.json`
- `screenshot-01-normal-launch-after-30s.png`
- `screenshot-recovered-model-10s.png`
- `screenshot-recovered-model-30s.png`
- `screenshot-recovered-model-60s.png`
- `screenshot-recovered-model-120s.png`
- `screenshot-02-main-dashboard-resume.png`
- `screenshot-03-story-queue-after-banner-close.png`
- `screenshot-04-story-queue-leads.png`
- `screenshot-11-story-queue-lead-list.png`
- `screenshot-12-scroll-after-focus-body.png`
- `screenshot-13-after-generate-verification-notes-20s.png`
- `screenshot-14-after-generate-verification-notes-80s.png`
- `screenshot-16-workbench-after-draft-placeholder.png`
- `screenshot-17-after-workbench-nav-retry.png`
- `screenshot-18-after-open-draft-retry.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Workbench/drafting path blanks the app content after verification/draft action

Observed: After the model setup and scan passed, the app reached a lead workflow panel. Clicking `Generate Verification Notes` led to a blank main content area. The app process remained alive with title `The Civic Desk`, and product-managed `ollama.exe` remained running with `phi4-mini:latest` installed. The blank state persisted after more than 60 seconds. Attempts to navigate back to Workbench/open the draft left the same blank content state.

Expected: The Workbench/editor path should remain visible and allow the tester to continue draft review, hold/send-back/approve/cut workflow, or show a clear progress/error state.

Impact: This blocks writer/editor workflow and prevents the required full Longmont E2E publication, export, here.now publish, and output-quality validation.

Repro:

1. Clean install build `71ab39d` from the directive artifact.
2. Let recovered setup install runtime/model and reach main dashboard.
3. Run source discovery/scan.
4. Open a lead workflow for the council roof contract lead.
5. Click `Generate Verification Notes`.
6. Observe blank content area with only the `Local AI ready` banner visible.

### Major: Draft generated from lead has no linked source documents

Observed: The DB contains a draft for lead 20 with content: `No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.`

Expected: Lead-based draft workflow should either attach/link public evidence or clearly block advancement before a publishable draft path.

Impact: This appears to be a safety-preserving placeholder rather than public output, but it reinforces that the run could not reach a publishable draft.

## Request For Coder

Please fix the Workbench/drafting render path after `Generate Verification Notes` so the app does not blank its content area and the editor workflow can continue. The recovered runtime/model path is now passing, and the scan produced enough leads; the next blocker is the Workbench/editor surface becoming unusable.
