# Tester Report - Full E2E Workbench Return 8801b10

Date: 2026-06-29T05:32:06Z heartbeat run
Tester machine: msi\civic, Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 8801b105edf483d63ec065143eea5b20cd66e5fe
Directive: test-comms/directives/20260629-rerun-full-e2e-workbench-return-8801b10.md
Result: PARTIAL PASS / INCOMPLETE FULL E2E

## Summary

The 8801b10 build installed and launched cleanly, completed cleanroom first-run setup, started app-owned Ollama, selected qwen2.5:7b, and recovered Longmont content with 19 leads.

Draft generation and repeat drafting improved versus the immediately prior a8c35fb report. In this run:

- A visible Story Queue Draft button opened the Workbench draft wizard.
- Generate Draft was focused and Enter triggered generation.
- Draft 1 persisted.
- Returning to Story Queue with Alt+1 plus mouse-wheel scrolling worked.
- A second distinct lead opened the draft wizard and persisted draft 2.

I stopped this heartbeat run after 2 persisted drafts. The full acceptance target is not yet complete: 5-10 stories, editor actions, export, ZIP creation, here.now publish, and HTTP verification were not reached.

## Environment

- Windows version: Windows 11 Home 10.0.26200
- CPU: Intel Core i7-13620H
- RAM: 15.7 GB
- GPU: Intel UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: previously recorded in this cleanroom loop as 346.4 GB
- Node: not used for product run
- Rust: not used for product run
- npm: not used for product run
- Ollama installed/running before clean wipe: prewipe state recorded in artifact JSON
- Models present after product setup: app showed Local AI ready, qwen2.5:7b

## Product And Artifact Verification

- Required product commit: `8801b105edf483d63ec065143eea5b20cd66e5fe`
- Observed remote branch head: `8801b105edf483d63ec065143eea5b20cd66e5fe`
- Preferred NSIS installer: `test-comms/artifacts/20260629-rerun-full-e2e-8801b10/The Civic Desk_0.2.8_x64-setup.exe`
- Observed NSIS SHA256: `CFE61A7858523C370924F37BD7DCA2102F85C9CCF429F3FDC57C6B85C67CC506`
- Fallback MSI: `test-comms/artifacts/20260629-rerun-full-e2e-8801b10/The Civic Desk_0.2.8_x64_en-US.msi`
- Observed MSI SHA256: `BEF4C67E948AEFAE762DEDAF4362C16096D56F3C51A38BC43F8CE919923373E4`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, ACTIVE_DIRECTIVE, and the active 8801b10 directive.
2. Verified product branch head and both installer hashes.
3. Clean wiped CivicNewspaper/app-owned runtime and model state.
4. Installed the matched NSIS installer silently.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
6. Set app window to 1280x720 and monitored setup at 0, 30, 60, 120, 240, and 420 seconds.
7. Confirmed Local AI ready with qwen2.5:7b.
8. Read SQLite database in read-only mode after recovery: 19 leads, 0 drafts.
9. Navigated to Story Queue and exposed lead cards with mouse-wheel scrolling.
10. Clicked Draft on lead 13, pressed Enter in the wizard, waited, and verified draft 1 persisted.
11. Returned to Story Queue using Alt+1, exposed another lead, clicked Draft on lead 12, pressed Enter in the wizard, waited, and verified draft 2 persisted.
12. Stopped the run and wrote this report before reaching five drafts or publication.

## Results

- Install and launch from NSIS: PASS
- Clean app-data/profile wipe: PASS
- First-run setup/recovery to Longmont content: PASS
- App-owned local AI runtime/model availability: PASS, app showed Local AI ready with qwen2.5:7b
- Sources/evidence/leads available after recovery: PASS, 6 sources, 27 evidence items, 19 leads
- 10+ reviewable leads: PASS by database count, 19 leads
- Draft click routes into Workbench drafting screen: PASS
- Generate Draft focus/Enter activation: PASS for two leads
- Draft 1 persists: PASS
- Draft 2 on a different lead persists: PASS
- Repeat drafting to at least 5: NOT COMPLETED in this run
- Back to Queue path at 1280x720: PARTIAL PASS via Alt+1 and wheel scrolling; direct button click not retested to completion
- Advisory mojibake: NOT FULLY RETESTED
- Writer/editor controls: NOT COMPLETED
- Export local output/ZIP: NOT REACHED
- here.now publish and HTTP 200 verification: NOT REACHED
- Ready for Scott to use for real Longmont publication next week: NO, full publication gate remains incomplete

## Evidence

All evidence is under `test-comms/artifacts/20260629-rerun-full-e2e-8801b10/`.

Key artifacts:

- `run880-prewipe-state.json`
- `run880-postwipe-paths.json`
- `run880-launch.json`
- `run880-monitor-log.json`
- `run880-monitor-420.png` - Local AI ready, qwen2.5:7b
- `run880-db-counts-after-recovery.json` - 19 leads, 0 drafts
- `run880-01-leads-visible.png`
- `run880-02-draft1-wizard.png`
- `run880-03-after-enter-5s.png`
- `run880-04-after-draft1-wait.png`
- `run880-db-after-draft1.json` - 1 persisted draft
- `run880-05-queue-after-draft1.png`
- `run880-06-draft2-wizard.png`
- `run880-07-after-draft2-enter-5s.png`
- `run880-08-after-draft2-wait.png`
- `run880-db-after-draft2.json` - 2 persisted drafts
- `run880-db-final.json` - final state, 2 persisted drafts

Final database excerpt:

```json
{
  "sources": 6,
  "evidence_items": 27,
  "leads": 19,
  "daily_scan_leads": 11,
  "daily_scan_runs": 1,
  "drafts": 2,
  "publish_runs": 0,
  "published_posts": 0,
  "verification_tasks": 3
}
```

Persisted drafts:

```json
[
  [1, 13, "watch", "Draft: City Council Meetings Schedule: The City of Longmont regularly holds City Council meetings to discuss important issues, with detailed agendas available online for public participation.", "draft_generated"],
  [2, 12, "watch", "Draft: Youth Center Programs: The Youth Center offers programs to support families, children, and youth in Longmont, aiming to help them grow and succeed.", "draft_generated"]
]
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Full Longmont publication E2E remains incomplete

Observed: The cleanroom run successfully reached two persisted drafts from two different leads, but stopped before producing the required 5-10 stories, exercising full editor controls, exporting static output/ZIP, publishing to here.now, or verifying HTTP 200.

Expected: Continue from clean install through at least five stories and a real publication/export/publish proof.

Impact: 8801b10 is a meaningful improvement over a8c35fb for draft routing/persistence, but the release readiness gate is not closed.

Repro:

1. Clean install 8801b10 NSIS artifact with matched hash.
2. Let setup reach Local AI ready.
3. Draft lead 13, wait for persistence.
4. Return to Story Queue, draft lead 12, wait for persistence.
5. Actual final state for this run: `drafts = 2`, no export/publish rows.

## Request For Coder

The 8801b10 build restores the cleanroom draft path through two persisted drafts. Please either send a continuation directive for the remaining 5-draft/export/publish path on this same build, or send the next build if you want the tester to continue from a new clean install.
