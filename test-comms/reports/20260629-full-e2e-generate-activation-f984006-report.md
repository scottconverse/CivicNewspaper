# Tester Report - Full E2E Generate Activation f984006

Date: 2026-06-29T04:44:05Z heartbeat run
Tester machine: msi\civic, Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: f98400668680a5b579ad186a33a0ace8f5df7aed
Directive: test-comms/directives/20260629-rerun-full-e2e-generate-activation-f984006.md
Result: PARTIAL PASS / BLOCKED BEFORE PUBLICATION

## Summary

The targeted Generate Draft activation fix is materially improved. In this clean install run:

- Pressing Enter from the draft wizard changed the primary button to `Generating Draft...`.
- Draft 1 persisted successfully.
- Returning to Story Queue and drafting a different visible lead persisted draft 2 successfully.
- Already-drafted leads were labeled `Draft exists` and exposed `Open draft`, so duplicate prevention was visible.

The full release E2E did not complete. I stopped at 2 persisted drafts, below the directive's 5-10 story target. Attempts to continue to a third non-drafted lead became unreliable around the editor/queue transition at 1280x720: the direct Back to Queue button was partly clipped on the right edge and did not respond to two click attempts, while keyboard navigation did return to the Story Queue top. I could expose another non-drafted lead, but a subsequent click did not open a third wizard and the app remained in the existing draft/editor area. Final database count stayed at 2 drafts.

The run therefore proves the specific f984006 activation/persistence fix through two drafts, but it is not ready for Scott to use for a real Longmont publication next week because the 5-draft/editor/export/publish gates were not reached.

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

- Required product commit: `f98400668680a5b579ad186a33a0ace8f5df7aed`
- Observed remote branch head: `f98400668680a5b579ad186a33a0ace8f5df7aed`
- Preferred NSIS installer: `test-comms/artifacts/20260629-rerun-full-e2e-f984006/The Civic Desk_0.2.8_x64-setup.exe`
- Observed NSIS SHA256: `AE99DA832C8126122A68DEC2ECD2498B56253D35B491FB1E5035B1ED11807CB3`
- Fallback MSI: `test-comms/artifacts/20260629-rerun-full-e2e-f984006/The Civic Desk_0.2.8_x64_en-US.msi`
- Observed MSI SHA256: `29A5148C1DB048125E26743B3E7588E1928C0FE06BBF25A5CDF544D824EF4183`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, ACTIVE_DIRECTIVE, and active directive.
2. Verified `stable-readiness-local-gates` branch head and installer hashes.
3. Recorded prewipe state, stopped product/runtime processes, and removed CivicNewspaper/app-owned runtime/model state.
4. Installed NSIS silently:
   `Start-Process -FilePath "...The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait`
5. Launched:
   `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
6. Set window to 1280x720 and captured setup monitor screenshots at 0, 30, 60, 90, 120, 180, 240, 300, and 420 seconds.
7. Confirmed Local AI ready with qwen2.5:7b.
8. Read SQLite database in read-only mode after recovery.
9. Opened Story Queue, exposed lead cards, and clicked Draft on a visible lead.
10. Pressed Enter from the draft wizard. Verified `Generating Draft...` visible and draft 1 persisted.
11. Returned to Story Queue, selected a different visible lead, pressed Enter from that wizard, and verified draft 2 persisted.
12. Tried to proceed to a third non-drafted lead, but did not get a third draft to persist.

## Results

- Install and launch from NSIS: PASS
- Clean app-data/profile wipe: PASS
- First-run setup/recovery to Longmont content: PASS
- App-owned local AI runtime/model availability: PASS, app showed Local AI ready with qwen2.5:7b
- Sources/evidence/leads available after recovery: PASS, 6 sources, 27 evidence items, 20 leads
- 10+ reviewable leads: PASS by database count, 20 leads
- Draft click routes into Workbench drafting screen: PASS
- Generate Draft receives Enter activation and shows progress: PASS
- Draft 1 persists: PASS
- Draft 2 on a different lead persists: PASS
- Duplicate prevention/routing for already-drafted leads: PARTIAL PASS, queue showed `Draft exists` and `Open draft`
- 5-10 stories/briefs: FAIL/NOT REACHED, final drafts count was 2
- Writer/editor controls: PARTIAL, draft editor/legal-risk advisor area was visible; no approve/hold/cut/export exercised
- Export local output/ZIP: NOT REACHED
- here.now publish and HTTP 200 verification: NOT REACHED
- Ready for Scott to use for real Longmont publication next week: NO

## Evidence

All evidence is under `test-comms/artifacts/20260629-rerun-full-e2e-f984006/`.

Key artifacts:

- `runf984-prewipe-state.json`
- `runf984-postwipe-paths.json`
- `runf984-launch.json`
- `runf984-monitor-log.json`
- `runf984-monitor-420.png` - Local AI ready, qwen2.5:7b
- `runf984-db-counts-after-recovery.json` - 20 leads, 0 drafts
- `runf984-08-draft1-wizard.png` - first drafting wizard
- `runf984-09-after-enter-5s.png` - `Generating Draft...` visible after Enter
- `runf984-db-after-draft1.json` - 1 persisted draft
- `runf984-16-draft2-wizard.png` - second different lead drafting wizard
- `runf984-db-after-draft2.json` - 2 persisted drafts
- `runf984-18-queue-after-draft2.png` - queue shows Drafts 2 and `Draft exists`
- `runf984-db-final.json` - final persisted draft count 2
- `runf984-20-after-draft3-wait.png` and `runf984-22-draft3-wizard-confirm.png` - third-draft continuation did not increase draft count

Final database excerpt:

```json
{
  "sources": 6,
  "evidence_items": 27,
  "leads": 20,
  "daily_scan_leads": 12,
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
  [1, 18, "watch", "Draft: Vision Zero Projects: The Vision Zero initiative focuses on improving transportation safety and involves projects, activities, and opportunities for community involvement.", "draft_generated"],
  [2, 20, "watch", "Draft: Building Services Online Permitting Portal Experiencing Technical Issues: Technical problems with the Building Services online permitting portal are impacting residents' ability to conduct necessary city business, potentially leading to delays in construction projects.", "draft_generated"]
]
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 1
- Minor: 1
- Nit: 0

### Blocker - Full publication path still cannot be completed from cleanroom run

Observed: Draft activation/persistence worked for two different leads, but the run did not reach the required 5 drafts/stories, editor decisions, export, ZIP, or here.now publish. After draft 2, the Back to Queue button was partially clipped at the right edge at 1280x720 and did not respond to two direct click attempts. Keyboard navigation returned to Story Queue top, but attempts to proceed to a third non-drafted lead did not produce a third persisted draft before the run stopped.

Expected: After draft 2, tester should be able to continue drafting different visible leads until at least 5 drafts/stories exist, then exercise editor actions and publish/export.

Impact: f984006 fixes the immediate activation/persistence regression for the first two drafts, but the full release readiness gate is still not closed.

Repro:

1. Clean install f984006 NSIS artifact.
2. Let setup reach Local AI ready.
3. Draft lead 18 and wait for persistence.
4. Return to Story Queue and draft lead 20; wait for persistence.
5. Try to continue to a third lead at 1280x720.
6. Actual final state: drafts table remains at 2; export/publish not reached.

### Major - Back to Queue control is partly clipped and unreliable at 1280x720

Observed: In the editor/advisor area after draft generation, the `Back to Queue` button appears partly off the right edge. Clicks at two visible locations did not navigate. `Alt+1` did return to the Story Queue top.

Expected: The visible button should fit inside the 1280x720 app window and respond to clicks.

Impact: Repeat-drafting and editor workflow are slower and error-prone on the constrained tester window.

### Minor - Advisory heading contains mojibake

Observed: The legal-risk/advisory panel showed `Advisory warnings â€“ these do not block publishing.`

Expected: The dash should render as a normal dash/en dash, not mojibake.

Impact: Cosmetic/quality issue in editor UI.

## Request For Coder

The f984006 fix is a real improvement: Enter activation works and two separate drafts persisted. Please harden the post-draft workflow so the tester can reliably return to the queue and continue drafting at least 5 stories at 1280x720. Please also fix the clipped Back to Queue button and the advisory heading mojibake.
