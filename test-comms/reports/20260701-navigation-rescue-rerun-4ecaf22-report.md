# Tester Report - Navigation Rescue Rerun 4ecaf22

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `4ecaf22b8c52273ae6ec8bfc143fb7acb32645d1`
Directive: `test-comms/directives/20260701-navigation-rescue-rerun-4ecaf22.md`
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
   - Expected SHA256: `41FE39E228BD943650B94FD8BC056FE4EC84637BA498FC0D8B9F52F30804D8F5`
   - Actual SHA256: `41FE39E228BD943650B94FD8BC056FE4EC84637BA498FC0D8B9F52F30804D8F5`
   - Expected/actual size: `5640839`
3. Stopped stale `civicnews` and product-owned `ollama`, ran the prior uninstaller, removed product app data and product-owned model data, installed only from the directive artifact, and launched the app normally.
4. Confirmed visible app startup and product-owned local AI setup through `phi4-mini:latest`.
5. Saved Longmont identity as `Longmont Navigation Rescue Desk`.
6. Ran `Discover for my city`, then ran Daily Scan from the UI.
7. Inspected Story Queue and DB evidence linkage.
8. Stopped before drafting because the directive says to write BLOCKED if broad navigation/event-index evidence becomes `ready_to_draft`.

## Results

BLOCKED: a broad navigation/event-calendar lead still reached `ready_to_draft`.

What passed:

- Clean install and visible desktop launch passed.
- Product-owned Ollama/model setup passed.
- Identity persistence passed: `settings.identity.newsroom_name` is `Longmont Navigation Rescue Desk`.
- Summer Reading prior-failure case did not become `ready_to_draft`; the only Summer Reading-related lead was `review`.

What blocked:

- The queue produced one `ready_to_draft` lead from broad Visit Longmont navigation/event-calendar page chrome.
- The linked evidence excerpt contains `Skip navigation`, top-level site navigation, category lists, and visitor-guide/media/partner links, not specific festival details.
- Per directive item 9, this should not be `ready_to_draft` when the evidence is broad page chrome/event-list navigation.

Counts at block:

- sources: 19
- daily scan runs: 2
- daily scan leads: 21
- story queue leads: 33
- evidence items: 70
- lead-evidence links: 24
- drafts: 0
- publish runs: 0
- published posts: 0

Lead dispositions:

- `ready_to_draft`: 1
- `needs_verification`: 13
- `watch`: 13
- `background`: 4
- `review`: 2

## Navigation / Index Rescue Audit

Blocked ready lead:

- Lead id: 27
- Disposition: `ready_to_draft`
- Title/topic: `Longmont Arts Week Festival`
- Evidence ids: 50
- Source: `Visit Longmont events`
- URL: `https://www.visitlongmont.org/events/`

Lead text:

`Longmont Arts Week Festival: Annual events featuring art exhibits, live music performances throughout downtown Longmont. Editor context: An annual cultural event with potential high attendance and significant impact on local businesses. Suggested treatment: brief. Newsworthiness: 12/20...`

Evidence excerpt begins:

`Things to Do in Longmont, Colorado - Event Calendar --> Your browser is not supported for this experience. We recommend using Chrome, Firefox, Edge, or Safari. Skip navigation Skip to main content Explore Events Live Music & Concerts Annual Events & Festivals This Weekend Submit Your Event Sundance Film Festival Things to Do Family Fun Events Parks...`

This is broad calendar/navigation page chrome and category navigation. It does not provide specific Longmont Arts Week Festival dates, artists, venues, or event details. This is the exact class of rescue lead the directive says must not be `ready_to_draft`.

## Summer Reading Regression Audit

Summer Reading-related leads found:

- Lead 31: disposition `review`.

No Summer Reading lead reached `ready_to_draft`, so the earlier Summer Reading grounding failure remains improved in this build.

## Evidence-Linkage Audit

The ready lead had linked evidence, but it was unrelated/broad page chrome. This fails semantic grounding for ready-to-draft status.

No `ready_to_draft` lead lacked linked source evidence.

## Workbench / Editor Workflow

Not run. The directive explicitly says to write BLOCKED if generic navigation/events/departments/index evidence appears as `ready_to_draft`. I did not force a draft from the weak broad-index lead.

## ZIP / Publish

- ZIP/package path: not produced.
- here.now URL: not produced.
- Latest `publish_runs.provider`: no publish run.
- Latest `publish_runs.published_url`: none.
- Latest `publish_runs.deployment_id`: none.

Export and here.now publish were not attempted because the only draftable lead hit the directive's navigation-rescue blocker before draft generation.

## Output Quality Audit

No public output was generated, exported, zipped, or published in this run. Therefore duplicate-topic, public scaffolding, mojibake, unsupported-lead, paragraph-citation, headline, and here.now page audits could not be performed.

## Evidence

- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/install-clean-launch.log`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/model-watch.txt`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/machine-profile.txt`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/db-snapshot-after-model-watch.json`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/queue-navigation-rescue-audit.json`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/final-blocked-db-snapshot.json`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-model-10s.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-model-30s.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-model-60s.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-model-120s.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-identity-saved-corrected.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-scan-started.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-after-daily-scan.png`
- `test-comms/evidence/20260701-navigation-rescue-rerun-4ecaf22/screenshot-story-queue-blocked-ready-navigation-lead.png`

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Broad navigation/event-calendar evidence still becomes ready_to_draft

Observed: Lead 27, `Longmont Arts Week Festival`, reached `ready_to_draft` with linked evidence 50 from `Visit Longmont events`. The evidence excerpt is broad event-calendar navigation/page chrome beginning with browser-support text, `Skip navigation`, top-level categories, and visitor guide/media/partner links.

Expected: Generic city events, event index, services/departments/calendar, newsletter, and broad navigation/page-chrome excerpts should not become actionable rescue evidence or `ready_to_draft`.

Impact: The editor queue still presents a weak broad-index lead as draft-ready, wasting model/editor time and risking unsupported story generation if drafted.

Repro: Clean install 4ecaf22, save Longmont identity, run `Discover for my city`, run Daily Scan, inspect Story Queue/DB for `ready_to_draft` leads.

## Request For Coder

Tighten the queue-side navigation/index rescue filter further for Visit Longmont event-calendar chrome. Evidence that is mostly site navigation/category lists should remain verification/watch/background, not `ready_to_draft`, even when the model invents a plausible festival topic.
