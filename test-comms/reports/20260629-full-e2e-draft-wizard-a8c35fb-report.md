# Tester Report - Full E2E Draft Wizard a8c35fb

Date: 2026-06-29T05:13:06Z heartbeat run
Tester machine: msi\civic, Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Product commit: a8c35fbb7e99ec7589c7699f73152893081208fa
Directive: test-comms/directives/20260629-rerun-full-e2e-draft-wizard-a8c35fb.md
Result: BLOCKED

## Summary

The a8c35fb cleanroom run installed and launched successfully, reached Local AI ready with qwen2.5:7b, and recovered Longmont data with 19 leads.

The run blocked before draft generation. After scrolling the Story Queue to visible lead cards, clicking a visible `Draft` button did not route into the Workbench draft wizard. The screen remained on the Story Queue lead cards. Pressing Enter afterward did not start generation. After a full wait, the SQLite `drafts` table remained at 0 rows.

Because draft 1 never opened/generated in this run, I could not test the directive's intended wizard focus, no-double-fire, bottom Generate Draft button, repeat drafting, 5-story, editor, export, ZIP, or here.now publish gates.

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

- Required product commit: `a8c35fbb7e99ec7589c7699f73152893081208fa`
- Observed remote branch head: `a8c35fbb7e99ec7589c7699f73152893081208fa`
- Preferred NSIS installer: `test-comms/artifacts/20260629-rerun-full-e2e-a8c35fb/The Civic Desk_0.2.8_x64-setup.exe`
- Observed NSIS SHA256: `DF588D903A56ACB7DD2FC469D70BCB3DC872F830F9F4B73C5D0AA7B33193AEDE`
- Fallback MSI: `test-comms/artifacts/20260629-rerun-full-e2e-a8c35fb/The Civic Desk_0.2.8_x64_en-US.msi`
- Observed MSI SHA256: `2B7D9164ADB6DCA8F38AFD68B1DBF8FAA300E08BAA36A00B9648B05EC8621841`

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, ACTIVE_DIRECTIVE, and the active a8c35fb directive.
2. Verified product branch head and both installer hashes.
3. Clean wiped CivicNewspaper/app-owned runtime and model state.
4. Installed the matched NSIS installer silently.
5. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
6. Set app window to 1280x720 and monitored setup at 0, 30, 60, 120, 240, and 420 seconds.
7. Confirmed Local AI ready with qwen2.5:7b.
8. Read SQLite database in read-only mode after recovery: 19 leads, 0 drafts.
9. Navigated to Story Queue and used mouse-wheel scrolling inside the app content area to expose lead cards.
10. Clicked a visible `Draft` button on the Story Queue.
11. Captured screenshots before and after pressing Enter and after waiting.
12. Re-read SQLite database in read-only mode.

## Results

- Install and launch from NSIS: PASS
- Clean app-data/profile wipe: PASS
- First-run setup/recovery to Longmont content: PASS
- App-owned local AI runtime/model availability: PASS, app showed Local AI ready with qwen2.5:7b
- Sources/evidence/leads available after recovery: PASS, 6 sources, 27 evidence items, 19 leads
- 10+ reviewable leads: PASS by database count, 19 leads
- Draft click routes into Workbench drafting screen: FAIL/BLOCKED in this run
- Generate Draft focus/Enter/no-double-fire/bottom button: NOT REACHED
- Draft 1 persists: NOT REACHED, final drafts count 0
- Draft 2/repeat drafting to 5: NOT REACHED
- Writer/editor controls: NOT REACHED
- Export local output/ZIP: NOT REACHED
- here.now publish and HTTP 200 verification: NOT REACHED
- Ready for Scott to use for real Longmont publication next week: NO

## Evidence

All evidence is under `test-comms/artifacts/20260629-rerun-full-e2e-a8c35fb/`.

Key artifacts:

- `runa8-prewipe-state.json`
- `runa8-postwipe-paths.json`
- `runa8-launch.json`
- `runa8-monitor-log.json`
- `runa8-monitor-420.png` - Local AI ready, qwen2.5:7b
- `runa8-db-counts-after-recovery.json` - 19 leads, 0 drafts
- `runa8-05-wheel-leads-visible.png` - Story Queue lead cards visible with Draft button
- `runa8-06-draft1-wizard-real.png` - after visible Draft click, still on Story Queue lead card
- `runa8-07-after-enter-5s-real.png` - Enter did not start generation or show progress
- `runa8-08-after-draft1-real-wait.png` - still on Story Queue lead card after wait
- `runa8-db-after-real-draft1.json` - drafts 0
- `runa8-db-final.json` - final drafts 0

Final database excerpt:

```json
{
  "sources": 6,
  "evidence_items": 27,
  "leads": 19,
  "daily_scan_leads": 11,
  "daily_scan_runs": 1,
  "drafts": 0,
  "publish_runs": 0,
  "published_posts": 0,
  "verification_tasks": 3,
  "draft_sample": []
}
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker - Visible Story Queue Draft button did not route to draft wizard

Observed: With lead cards visible in Story Queue at 1280x720, clicking a visible `Draft` button did not open the Workbench draft wizard. The app remained on the Story Queue card. Pressing Enter afterward did not start generation, no `Generating Draft...` progress appeared, and the database remained at 0 drafts.

Expected: Clicking Draft on any lead should route into the Workbench draft wizard, focus Generate Draft, and allow Enter/click generation to persist draft 1.

Impact: The active directive cannot proceed past the first drafting gate. The a8c35fb-specific wizard focus/no-double-fire/bottom-button checks were not reached.

Repro:

1. Clean install a8c35fb NSIS artifact with matched hash.
2. Let setup reach Local AI ready.
3. Navigate to Story Queue.
4. Scroll until a lead card and `Draft` button are visible.
5. Click the visible `Draft` button.
6. Actual result: no route change to draft wizard; database remains `drafts = 0`.

## Request For Coder

Please recheck the Story Queue Draft action path in a8c35fb. In this cleanroom run, the visible Draft button did not route into the wizard at all, so the new wizard activation hardening could not be validated.
