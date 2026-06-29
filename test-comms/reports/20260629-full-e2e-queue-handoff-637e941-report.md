# Tester Report - Full E2E Queue Handoff 637e941

Date: 2026-06-29T05:50:06Z heartbeat run
Tester machine: msi\civic, Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: 637e941ac77361033fc22b48fac33ae1aa50a6b3
Directive: test-comms/directives/20260629-rerun-full-e2e-queue-handoff-637e941.md
Result: PARTIAL PASS / INCOMPLETE FULL E2E

## Summary

The 637e941 build installed and launched cleanly, completed first-run setup, started app-owned Ollama, selected qwen2.5:7b, and recovered Longmont content with 18 leads.

The specific Story Queue handoff fixes passed in this run:

- Clicking a visible Story Queue `Draft` button routed into the Workbench draft wizard.
- The Generate Draft button was focused and Enter started generation.
- Draft 1 persisted.
- Returning to Story Queue and clicking the body of a different lead card routed into the Workbench draft wizard.
- Draft 2 persisted.

The full release E2E remains incomplete because this heartbeat run stopped at 2 persisted drafts. The 5-10 story target, editor action matrix, export/ZIP, here.now publish, and HTTP verification were not reached.

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

- Required product commit: `637e941ac77361033fc22b48fac33ae1aa50a6b3`
- Observed remote branch head: `637e941ac77361033fc22b48fac33ae1aa50a6b3`
- Preferred NSIS installer: `test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64-setup.exe`
- Observed NSIS SHA256: `50F64FFCE76106BC1745766CA3AF0A50A46C5464F22BDB65220C8EDED348F67F`
- Fallback MSI: `test-comms/artifacts/20260629-rerun-full-e2e-637e941/The Civic Desk_0.2.8_x64_en-US.msi`
- Observed MSI SHA256: `04DCB36733FD969C4E17C763220BD9E135256524101883432FCD09E50EC1C7F1`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, ACTIVE_DIRECTIVE, and the active 637e941 directive.
2. Verified product branch head and both installer hashes.
3. Clean wiped CivicNewspaper/app-owned runtime and model state.
4. Installed the matched NSIS installer silently.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
6. Set app window to 1280x720 and monitored setup at 0, 30, 60, 120, 240, and 420 seconds.
7. Confirmed Local AI ready with qwen2.5:7b.
8. Read SQLite database in read-only mode after recovery: 18 leads, 0 drafts.
9. Navigated to Story Queue and exposed lead cards with mouse-wheel scrolling.
10. Clicked a visible `Draft` button for the Youth Center Programs lead, pressed Enter in the wizard, waited, and verified draft 1 persisted.
11. Returned to Story Queue, clicked the body of a different lead card for New Public Meeting Portal, pressed Enter in the wizard, waited, and verified draft 2 persisted.
12. Stopped the run before the five-draft/export/publish gates.

## Results

- Install and launch from NSIS: PASS
- Clean app-data/profile wipe: PASS
- First-run setup/recovery to Longmont content: PASS
- App-owned local AI runtime/model availability: PASS, app showed Local AI ready with qwen2.5:7b
- Sources/evidence/leads available after recovery: PASS, 6 sources, 27 evidence items, 18 leads
- 10+ reviewable leads: PASS by database count, 18 leads
- Visible Story Queue Draft button routes into Workbench wizard: PASS
- Story Queue lead card body routes into Workbench wizard: PASS
- Generate Draft focus/Enter activation: PASS for two leads
- Draft 1 persists: PASS
- Draft 2 on a different lead persists: PASS
- Repeat drafting to at least 5: NOT COMPLETED in this run
- Already-drafted lead handling: NOT FULLY RETESTED
- Back to Queue button fully visible/clickable: NOT FULLY RETESTED
- Advisory mojibake: NOT FULLY RETESTED
- Writer/editor controls: NOT COMPLETED
- Export local output/ZIP: NOT REACHED
- here.now publish and HTTP 200 verification: NOT REACHED
- Ready for Scott to use for real Longmont publication next week: NO, full publication gate remains incomplete

## Evidence

All evidence is under `test-comms/artifacts/20260629-rerun-full-e2e-637e941/`.

Key artifacts:

- `run637-prewipe-state.json`
- `run637-postwipe-paths.json`
- `run637-launch.json`
- `run637-monitor-log.json`
- `run637-monitor-420.png` - Local AI ready, qwen2.5:7b
- `run637-db-counts-after-recovery.json` - 18 leads, 0 drafts
- `run637-01-leads-visible.png` - visible lead card with Draft button
- `run637-02-after-visible-draft-click.png` - Draft button routed into wizard
- `run637-03-after-enter-5s.png` - post-Enter generation state
- `run637-04-after-draft1-wait.png`
- `run637-db-after-draft1.json` - 1 persisted draft
- `run637-05-queue-after-draft1.png`
- `run637-06-after-card-body-click.png` - card body routed into wizard
- `run637-07-after-card-enter-5s.png`
- `run637-08-after-draft2-wait.png`
- `run637-db-after-draft2-card.json` - 2 persisted drafts
- `run637-db-final.json` - final state, 2 persisted drafts

Final database excerpt:

```json
{
  "sources": 6,
  "evidence_items": 27,
  "leads": 18,
  "daily_scan_leads": 10,
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
  [1, 12, "watch", "Draft: Youth Center Programs in Longmont: The city is committed to supporting youth development through the Youth Center, which provides resources and activities for families and children.", "draft_generated"],
  [2, 10, "watch", "Draft: New Public Meeting Portal Launched: Longmont City Council and advisory board agendas are now published on a new public meeting portal, making it easier for residents to access information about upcoming meetings.", "draft_generated"]
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

Observed: The queue handoff regression passed for both visible Draft button and card body paths, and two drafts persisted. The run stopped before five drafts, editor actions, export/ZIP, here.now publish, or HTTP verification.

Expected: Continue from clean install through at least five stories and a real publication/export/publish proof.

Impact: 637e941 fixes the targeted queue handoff regression in this run, but the release readiness gate is not closed.

Repro:

1. Clean install 637e941 NSIS artifact with matched hash.
2. Let setup reach Local AI ready.
3. Draft one lead via the visible `Draft` button and wait for persistence.
4. Return to Story Queue, draft another lead via card body click and wait for persistence.
5. Actual final state for this run: `drafts = 2`, no export/publish rows.

## Request For Coder

The 637e941 queue handoff fix passes for the two paths tested. Please either send a continuation directive for the remaining 5-draft/editor/export/publish path on this build, or send the next build if you want a new clean install rerun.
