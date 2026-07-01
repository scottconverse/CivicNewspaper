# 20260701 Recovered Model Rerun 71ab39d Final Report

Tester: Codex desktop tester  
Directive: `test-comms/directives/20260701-recovered-model-rerun-71ab39d.md`  
Product branch: `main`  
Installer commit represented: `71ab39d3a8f5b6c947946b6b5af6862064dc8c94`  
Installer: `test-comms/artifacts/20260701-recovered-model-rerun-71ab39d/The Civic Desk_0.3.1_x64-setup.exe`

## Result

Overall directive result: BLOCKED after recovered model success.

The recovered startup/model gate passed: the app stayed visible, installed the product-owned runtime, auto-started the recommended model pull, and reached the main dashboard with `Local AI ready` for `phi4-mini:latest`.

The later Longmont E2E flow blocked in Workbench. Daily Scan completed and produced sources, leads, and evidence globally, but the selected `ready_to_draft` scan lead had no linked source documents. Draft generation created only a verification placeholder, and Workbench preflight explicitly listed package-validity blockers. I stopped before export, here.now publish, and public-output audits because there was no valid reader-facing story package to publish.

## Environment

- OS: Windows 11 Home, 10.0.26200
- App launch path: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Runtime model observed: `phi4-mini:latest`

## Installer Verification

PASS.

- Expected SHA256: `43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04`
- Actual SHA256: `43D590BEEDA25101CEFBCD4D4DAA0F8FEA63B7CAB618B5648C30BA6C9FC59B04`
- Expected size: `5632526`
- Actual size: `5632526`

## Clean Wipe And Native Launch

PASS.

- Stopped stale `civicnews` and `ollama` processes.
- Removed prior product data from `%APPDATA%\com.scottconverse.civicdesk`, `%LOCALAPPDATA%\com.scottconverse.civicdesk`, and `%USERPROFILE%\.ollama`.
- Installed from the directive installer.
- Launched normally; no handle manipulation was used.
- Native window remained visible as `The Civic Desk`.

## Recovered Model Auto-Pull

PASS.

- 10 seconds: app visible on setup path; product-owned `ollama.exe` running; partial model blob files present.
- 30 seconds: `ollama list` showed `phi4-mini:latest`, model id `78fad5d182a7`, size `2.5 GB`.
- 60 seconds: `phi4-mini:latest` still present.
- 120 seconds: app had reached the main dashboard with visible `Local AI ready` and `phi4-mini:latest`.

## Longmont E2E Progress

PASS until draft/linkage quality gate.

- Source discovery / scan state:
  - `sources`: 19 total.
  - Source mix included `primary_record`, `official_comm`, `media_lead`, and `community_signal`.
  - Online sources: 14; offline sources: 5.
- Daily Scan:
  - `daily_scan_runs`: 1 completed run.
  - `daily_scan_leads`: 10.
  - `leads`: 22.
  - `evidence_items`: 70.
- Story Queue UI:
  - Showed 22 leads, 4 high priority, 19 sources.
  - Exposed a `ready_to_draft` lead: `Council Vote on Library Roof Contract`.

## Blocking Failure

FAIL: ready-to-draft scan lead did not retain linked source documents into drafting.

Steps to reproduce from this clean run:

1. Open Story Queue after the completed Daily Scan.
2. Select the ready-to-draft lead `Council Vote on Library Roof Contract`.
3. Invoke draft generation / verification notes.
4. Open the generated draft in Workbench.

Observed:

- `drafts` contained one generated draft for lead `20`.
- Draft title: `Council Vote on Library Roof Contract: The council agenda includes voting for roof work at`
- Draft content: `No source documents are linked to this lead yet. Treat it as a verification assignment until an editor attaches public source material.`
- `lead_evidence` had no rows for `lead_id = 20`.
- Workbench preflight listed these blockers:
  - `This scanned-lead draft has no linked source documents.`
  - `No source documents are linked. Treat this as a verification assignment until you attach or cite public source material.`
  - `Story body is very short for this format. Consider making it a brief or adding verified reporting.`
- Publish tables remained empty:
  - `publish_runs`: 0.
  - `published_posts`: 0.

Expected:

- A Daily Scan lead promoted to `ready_to_draft` should carry the specific source/evidence links that made it draftable, or the UI should keep it in verification instead of creating a publish-path draft.
- The Workbench should expose a reader-facing draft backed by linked evidence before export or static publish can be tested.

## Gates Not Reached

Not run because the draft package failed before publication:

- Static export ZIP.
- here.now publish.
- Public `watch/*.html` quality audit.
- Duplicate-topic audit.
- Mojibake audit.
- Reporter-note scaffolding audit.

## Evidence

- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/cleanwipe-install-launch.log`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/recovered-model-watch.txt`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/db-snapshot-after-recovered-model-watch.json`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/db-snapshot-final-e2e-blocker.json`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-01-normal-launch-after-30s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-10s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-30s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-60s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-recovered-model-120s.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-02-main-dashboard-resume.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-10-uia-scroll-attempt.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-12-story-queue-cards.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-13-after-draft-council-invoke.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-15-verification-notes-after-wait.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-16-workbench-after-draft-placeholder.png`
- `test-comms/evidence/20260701-recovered-model-rerun-71ab39d/screenshot-18-workbench-scrolled-to-draft.png`

