# Tester Report - Source Grounding Rerun fae9457

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `fae94570ab75cd3548cf5b8d254aa668ca96fce9`
Directive: `test-comms/directives/20260701-source-grounding-rerun-fae9457.md`
Result: BLOCKED

## Environment

- Windows version: Windows 11 Home, kernel `10.0.26100.1`
- CPU: 13th Gen Intel Core i7-13620H
- RAM: 16 GB
- GPU: Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Product app: `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
- Product-owned Ollama: `C:\Users\civic\AppData\Roaming\com.scottconverse.civicdesk\ollama-runtime\v0.30.11\ollama.exe`
- Model present: `phi4-mini:latest`, ID `78fad5d182a7`, size `2.5 GB`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `ACTIVE_DIRECTIVE.md`, README, protocol, tester prompt, and the active directive.
2. Verified installer `The Civic Desk_0.3.1_x64-setup.exe`.
   - Expected SHA256: `ABA11CFFA0A52E130C2B77C2E20F139C22039DE305CC5C32C62F2C245C83AC45`
   - Actual SHA256: `ABA11CFFA0A52E130C2B77C2E20F139C22039DE305CC5C32C62F2C245C83AC45`
   - Expected/actual size: `5650162`
3. Stopped stale `civicnews` and product-owned `ollama`, ran the prior uninstaller, removed product app data and product-owned model data, installed only from the directive artifact, and launched the app normally.
4. Confirmed visible app startup and product-owned local AI setup through `phi4-mini:latest`.
5. Saved Longmont identity as `Longmont Source Grounding Desk FAE`.
6. Ran Daily Scan from the UI, then ran `Discover for my city`, which triggered a second scan/intake pass.
7. Inspected Story Queue and DB evidence linkage.
8. Confirmed the prior Summer Reading regression is not draftable.
9. Drafted the only `ready_to_draft` lead, `Longmont city events...`, generated a local-model draft, opened Workbench, checked the editor responsibility box, and attempted `Approve for Static Publish`.

## Results

BLOCKED: the prior Summer Reading failure is improved, but the full E2E flow cannot proceed because no valid grounded draft can be approved/exported/published.

What passed:

- Clean install and visible desktop launch passed.
- Product-owned Ollama/model setup passed.
- Identity persistence passed: `settings.identity.newsroom_name` is `Longmont Source Grounding Desk FAE`.
- Summer Reading prior-failure case is no longer `ready_to_draft`; all Summer Reading-related leads found were `needs_verification`.
- Workbench now blocks static approval when linked source documents do not match the story topic.

What blocked the E2E:

- The only `ready_to_draft` lead was a generic `Longmont city events...` rescue lead linked to broad city events/departments excerpts.
- The generated draft was titled `Upcoming City Events in Longmont`.
- Workbench reported: `This scanned-lead draft's linked source documents do not appear to match the story topic.`
- Static approval did not succeed; draft status remained `draft_generated`, with no attestation and no publish decision audit.
- Because no draft could be approved for static publish, ZIP export and here.now UI publishing were not attempted.

Counts at block:

- sources: 19
- daily scan runs: 2
- daily scan leads: 22
- story queue leads: 34
- evidence items: 69
- lead-evidence links: 21
- drafts: 1
- publish runs: 0
- published posts: 0

Lead dispositions:

- `ready_to_draft`: 1
- `needs_verification`: 16
- `watch`: 12
- `background`: 4
- `review`: 1

## Evidence-Linkage Audit

Ready-to-draft lead:

`Longmont city events: Longmont Housing Authority Longmont Museum Longmont Power & Communications Municip...`

Linked evidence:

- Evidence 64: old broad events index, `Events from Wednesday, Nov. 20, 2024 - Friday, Jan. 3, 2025 - City of Longmont`.
- Evidence 68: broad city departments/navigation/newsletter text.

This ready lead was linked to evidence rows, so it did not lack evidence. However, the evidence did not support a specific reader-facing article, and Workbench correctly blocked approval after draft generation.

## Summer Reading Regression Audit

Summer Reading-related leads found:

- Lead 16: `Library Closure Schedule Announcement... summer reading list`, disposition `needs_verification`.
- Lead 25: `City of Longmont Announces Upcoming Library Events... Summer Reading Challenge...`, disposition `needs_verification`.
- Lead 28: `Downtown Longmont's Summer Reading Challenge...`, disposition `needs_verification`.

None of the Summer Reading-related leads reached `ready_to_draft`. This is an improvement over eb0a4ac.

## Workbench / Editor Workflow

Generated draft:

- Draft id: 1
- Lead id: 34
- Title: `Upcoming City Events in Longmont`
- Status after approval attempt: `draft_generated`

Workbench blocker text:

- `Fix before static publish approval:`
- `This scanned-lead draft's linked source documents do not appear to match the story topic.`
- `Linked source documents may not match this story topic. Attach the correct source material or rewrite the story around the linked sources.`
- `Before static publish approval: This scanned-lead draft's linked source documents do not appear to match the story topic.`

Editor workflow coverage:

- Draft generation: PASS.
- Static approval blocker for unrelated source material: PASS.
- Send back / hold / ready / approve / export / here.now publish: BLOCKED by no approvable grounded draft.

## ZIP / Publish

- ZIP/package path: not produced.
- here.now URL: not produced.
- Latest `publish_runs.provider`: no publish run.
- Latest `publish_runs.published_url`: none.
- Latest `publish_runs.deployment_id`: none.

These were not attempted because the app correctly blocked static approval for the only draftable lead available in this run.

## Output Quality Audit

No public output was generated, exported, zipped, or published in this run. Therefore duplicate-topic, public scaffolding, mojibake, unsupported-lead, paragraph-citation, headline, and here.now page audits could not be performed.

## Evidence

- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/install-clean-launch.log`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/model-watch.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/machine-profile.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/queue-grounding-audit.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/queue-grounding-audit-after-discovery.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/ready-leads-current-audit.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/draft-1-workbench-grounding-audit.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/final-blocked-db-snapshot.json`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-model-10s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-model-30s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-model-60s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-model-120s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-identity-corrected.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-daily-scan-started.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-after-daily-scan.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-sources-page.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-after-source-discovery.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-story-queue-after-scan.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-story-queue-after-discovery.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-ready-lead-opened.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-workbench-unrelated-source-blocker.png`
- `test-comms/evidence/20260701-source-grounding-rerun-fae9457/screenshot-workbench-approval-blocked.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Full E2E publish path has no approvable grounded draft

Observed: After clean install, source discovery, and two scan/intake passes, only one lead reached `ready_to_draft`. Drafting it produced a generic events story that Workbench correctly blocked as unrelated to linked sources. No draft could be approved, exported, or published.

Expected: A full Longmont E2E run should produce at least one source-grounded, approvable reader-facing draft from the available source set, or the app should clearly label the run as having no publishable leads and explain what source expansion is needed.

Impact: The product cannot complete the directive's publish workflow on this cleanroom data set.

Repro: Clean install fae9457, save Longmont identity, run Daily Scan, run `Discover for my city`, open Story Queue, draft the only ready lead, attempt static approval.

### Major: Generic rescue lead still reaches ready_to_draft

Observed: The only ready lead was a generic `Longmont city events...` rescue lead backed by broad events/departments excerpts. Workbench caught it after draft generation, but the queue still presented it as draft-ready.

Expected: Broad source/navigation/event-index rescue leads should remain verification/background work unless they support a specific reader-facing article topic.

Impact: Editors can spend model time on non-story drafts that are predictably blocked later.

Repro: See `ready-leads-current-audit.json` and draft id 1.

## Request For Coder

The Summer Reading regression appears improved, and Workbench/static approval now blocks unrelated linked sources. The remaining issue is finish-line usability: the scanner needs either at least one genuinely grounded ready-to-draft lead from this Longmont source set, or a clear no-publishable-leads state instead of surfacing a generic broad-events rescue lead as draft-ready.
