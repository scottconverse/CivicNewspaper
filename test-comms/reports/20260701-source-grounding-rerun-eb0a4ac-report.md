# Tester Report - Source Grounding Rerun eb0a4ac

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `eb0a4ac284eedeb281891bb468f06cf9d564b1fe`
Directive: `test-comms/directives/20260701-source-grounding-rerun-eb0a4ac.md`
Result: BLOCKED

## Environment

- Windows version: Windows 11 Home
- CPU: MSI Cyborg 15 A13VE host, Intel platform
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
   - Expected SHA256: `3105CAD4EB00D6DDE501679E9C0820721267AC9F106B660735B42C3616734295`
   - Actual SHA256: `3105CAD4EB00D6DDE501679E9C0820721267AC9F106B660735B42C3616734295`
   - Expected/actual size: `5622050`
3. Stopped stale `civicnews` and product-owned `ollama`, ran the prior uninstaller, removed product app data and product-owned model data, installed only from the directive artifact, and launched the app normally.
4. Confirmed visible app startup and product-owned local AI setup through `phi4-mini:latest`.
5. Saved Longmont identity as `Longmont Source Grounding Desk`.
6. Ran Daily Scan twice from the UI.
7. Inspected Story Queue and DB evidence linkage.
8. Selected the ready-to-draft lead `Summer Reading Challenge Starts at Longmont Public Library`.
9. Generated a local-model draft, opened Workbench, reviewed preflight warnings, checked the editor responsibility box, clicked `Approve for Static Publish`, and then clicked `Publish anyway (logged)` in the warning dialog.

## Results

BLOCKED: the rerun still permits a source-mismatched draft to reach `ready_to_publish`.

Clean install, visible launch, product-owned Ollama setup, model readiness, Daily Scan, draft generation, and identity persistence all worked. The blocking failure is semantic source grounding: the Story Queue exposed a draftable Summer Reading lead whose linked evidence was only broad Longmont events/departments material. Workbench did not report an unrelated linked-source blocker and allowed the editor to publish anyway after only advisory warnings.

Counts at block:

- sources: 19
- daily scan runs: 2
- daily scan leads: 25
- story queue leads: 37
- evidence items: 77
- lead-evidence links: 54
- drafts: 1
- publish decision audits: 1
- publish runs: 0
- published posts: 0

Identity audit:

- `settings.identity.newsroom_name`: `Longmont Source Grounding Desk`
- `settings.identity.city`: `Longmont`
- `settings.identity.state`: `CO`

No here.now publish was attempted because the product hit the source-grounding blocker before a valid package could be responsibly published.

## Source-Grounding Failure

Selected lead:

`Summer Reading Challenge Starts at Longmont Public Library: The 2026 Summer Reading Challenge is starting on Wednesday, May 21 with activities running through July 31.`

Linked evidence:

- Evidence 74, source `Longmont city events`: gaming club, Yoga Storytime, Spanish-English Conversation Group.
- Evidence 75, source `Longmont city events`: museum concert, library closure, Youth Center Independence Day closure, museum closure, Independence Weekend Free Concert.
- Evidence 76, source `Longmont city events`: broad city departments/navigation/newsletter text.

Generated draft body:

`Longmont Housing Authority Longmont Museum Longmont Power & Communications Municipal Court Municipal Probation NextLight Fiber Internet Parks and Natural Resources Planning and Development Services Public Information Public Safety Purchasing and Contracts. [Source](evidence:76).`

The draft title remains Summer Reading Challenge, but the cited paragraph is copied from broad departments/navigation evidence. Workbench warnings were advisory only:

- `[Verbatim Overlap]` for evidence 76
- `[Citation Coverage]`
- `[story quality] No clear attribution phrase found`

There was no explicit unrelated-source blocker. After clicking `Publish anyway (logged)`, DB state changed to:

- draft id 1 status: `ready_to_publish`
- `attested_by`: `Publisher`
- `guardrail_override_reason`: `Editor reviewed pre-publication warnings and chose to publish.`

## Evidence

- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/install-clean-launch.log`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/model-watch.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/machine-profile.txt`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/queue-grounding-audit.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/ready-leads-evidence-excerpts.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/draft-1-source-grounding-audit.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/final-blocked-db-snapshot.json`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-10s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-30s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-60s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-model-120s.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-identity-saved.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-after-daily-scan.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-story-queue.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-story-queue-rows.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-story-queue-actions.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-after-summer-reading-draft-click.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-workbench-summer-reading-generated.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-workbench-approve-warning-dialog.png`
- `test-comms/evidence/20260701-source-grounding-rerun-eb0a4ac/screenshot-workbench-after-publish-anyway-ready.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 0
- Nit: 0

### Blocker: Workbench permits ready-to-publish status for an unrelated-source draft

Observed: A Summer Reading Challenge lead was draftable with broad Longmont events/departments evidence. The generated draft cited evidence 76, which contains city departments/navigation text, not Summer Reading Challenge support. Workbench showed only advisory warnings and allowed `Publish anyway (logged)`, changing the draft to `ready_to_publish`.

Expected: Either the lead should not be draftable with this evidence set, or Workbench/static approval should block unrelated linked source documents and cited paragraphs before `ready_to_publish`.

Impact: A reader-facing package could include an unsupported headline/story topic or a paragraph citing unrelated evidence.

Repro: Clean install from the directive installer, run Longmont Daily Scan, open Story Queue, draft `Summer Reading Challenge Starts at Longmont Public Library`, generate draft, approve static publish with warnings.

### Major: Source-topic matching still marks broad event/calendar fragments as ready evidence

Observed: Ready-to-draft leads included Summer Reading, Baby Storytime, and Free Zone Day items backed by broad event-page excerpts that did not visibly contain the specific lead facts in the linked evidence excerpts.

Expected: Broad calendar/navigation snippets should be verification work unless the exact event/topic facts are present in linked evidence.

Impact: The queue can send weakly supported or unsupported model-suggested topics into the editor workflow.

Repro: See `ready-leads-evidence-excerpts.json` and `queue-grounding-audit.json`.

## Request For Coder

Fix or tighten the source-topic gate so ready-to-draft leads require evidence that supports the specific story topic, not just source/domain/category similarity. Also make Workbench/static approval treat unrelated linked source documents or cited paragraphs as package-validity blockers rather than advisory warnings.
