# Tester Report - Full E2E Workbench Route 8e4fcca

Date: 2026-06-29T03:51:05Z heartbeat run
Tester machine: msi\civic, Windows cleanroom tester
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: stable-readiness-local-gates
Directive: test-comms/directives/20260629-rerun-full-e2e-workbench-route-8e4fcca.md
Result: BLOCKED

## Summary

Clean install and first-run setup reached Story Queue with Local AI ready. The new Workbench routing behavior partially passed: clicking Draft on a visible lead moved into the Drafting Article / Workbench-style drafting screen with Generate Draft visible above the fold at 1280x720.

The run is blocked before draft 1 persists. Three activation attempts from that screen left the SQLite drafts table at 0 rows:

- Enter while on the drafting screen.
- Coordinate click at the visible Generate Draft button area.
- From focused Article Format control, Shift+Tab, Shift+Tab, Enter.

After the final attempt the app returned to the main navigation/Story Queue screen with Local AI ready, but the database still had 0 drafts. Because draft 1 did not persist, I could not proceed to draft 2, 5 stories, editor controls, export, ZIP, here.now publish, or HTTP verification.

Important branch note: the active directive names required product commit `8e4fcca6f3d762d32c892858fd56605bce971b4b`, but both `git rev-parse origin/stable-readiness-local-gates` and `git ls-remote origin refs/heads/stable-readiness-local-gates` observed branch head `8e4fcca694318d2dd121292e0686eb24ac6db98b`. Installer artifact hashes matched the directive.

## Environment

- Windows version: Windows 11 Home 10.0.26200
- CPU: Intel Core i7-13620H
- RAM: 15.7 GB
- GPU: Intel UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 346.4 GB
- Node: not used for product run
- Rust: not used for product run
- npm: not used for product run
- Ollama installed/running before clean wipe: ollama process was present before wipe; no global `ollama` command found after cleanroom setup checks
- Models present after product setup: app showed Local AI ready, qwen2.5:7b

## Product And Artifact Verification

- Required product commit in directive: `8e4fcca6f3d762d32c892858fd56605bce971b4b`
- Observed remote branch head: `8e4fcca694318d2dd121292e0686eb24ac6db98b`
- Preferred NSIS installer: `test-comms/artifacts/20260629-rerun-full-e2e-8e4fcca/The Civic Desk_0.2.8_x64-setup.exe`
- Observed NSIS SHA256: `D5D82D9A2BB736D54565ED737DB065B030CB4D83F7E5415451E5EAD0378BE191`
- Fallback MSI: `test-comms/artifacts/20260629-rerun-full-e2e-8e4fcca/The Civic Desk_0.2.8_x64_en-US.msi`
- Observed MSI SHA256: `9AF8CB74E0D1E80A775D053101824353A9877DA800D29C4B45024F7F5B25659E`

## Steps Run

1. Pulled coordination branch:
   `git fetch origin`
   `git switch test-comms/cleanroom-coder-tester`
   `git pull --ff-only origin test-comms/cleanroom-coder-tester`
2. Reread:
   `test-comms/README.md`
   `test-comms/protocol.md`
   `test-comms/prompts/tester-codex-desktop-prompt.md`
   `test-comms/ACTIVE_DIRECTIVE.md`
   `test-comms/directives/20260629-rerun-full-e2e-workbench-route-8e4fcca.md`
3. Verified product branch head:
   `git rev-parse origin/stable-readiness-local-gates`
   `git ls-remote origin refs/heads/stable-readiness-local-gates`
4. Verified installer hashes with `Get-FileHash -Algorithm SHA256`.
5. Recorded machine profile.
6. Clean wiped product/Ollama state:
   stopped civicnews, The Civic Desk, ollama, msiexec, WindowsTerminal, SearchHost, StartMenuExperienceHost when present;
   removed app data and app-owned runtime/model paths under LocalAppData, AppData, and user `.ollama`.
7. Installed NSIS silently:
   `Start-Process -FilePath "...The Civic Desk_0.2.8_x64-setup.exe" -ArgumentList "/S" -Wait`
8. Launched:
   `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`
9. Set app window to 1280x720 and captured progress at 0, 30, 60, 90, 120, 180, 240, 300, and 420 seconds.
10. Reached Story Queue / main nav with Local AI ready and qwen2.5:7b.
11. Read SQLite database in read-only mode:
   sources 6, evidence_items 27, leads 19, daily_scan_leads 11, daily_scan_runs 1, drafts 0, publish_runs 0, published_posts 0, verification_tasks 3.
12. Clicked a visible lead Draft button.
13. Confirmed the app routed to Drafting Article screen with Generate Draft visible above fold.
14. Attempted to generate draft three ways:
   Enter from the drafting screen;
   coordinate click on visible Generate Draft control;
   from Article Format focus, Shift+Tab, Shift+Tab, Enter.
15. After each timed attempt, captured UI and read SQLite in read-only mode.

## Results

- Install and launch from NSIS: PASS
- Clean app-data/profile wipe: PASS
- First-run setup/recovery to Longmont content: PASS
- App-owned local AI runtime/model availability: PASS, app showed Local AI ready with qwen2.5:7b
- Sources/evidence/leads available after recovery: PARTIAL PASS, 6 sources, 27 evidence items, 19 leads
- 10+ reviewable leads: PASS by database count, 19 leads
- Draft click routes into Workbench drafting screen: PASS for first visible lead
- Generate Draft persists draft 1: FAIL/BLOCKED, drafts remained 0 after three activation attempts
- Draft 2 on a different lead: NOT REACHED
- 5-10 stories/briefs: NOT REACHED
- Writer/editor controls: NOT REACHED
- Duplicate-draft handling: NOT REACHED
- Export local output/ZIP: NOT REACHED
- here.now publish and HTTP 200 verification: NOT REACHED
- Ready for Scott to use for real Longmont publication next week: NO

## Evidence

All evidence is under `test-comms/artifacts/20260629-rerun-full-e2e-8e4fcca/`.

- `run8e-monitor-log.json` - setup/runtime monitor snapshots
- `run8e-monitor-000.png`
- `run8e-monitor-030.png`
- `run8e-monitor-060.png`
- `run8e-monitor-090.png`
- `run8e-monitor-120.png`
- `run8e-monitor-180.png`
- `run8e-monitor-240.png`
- `run8e-monitor-300.png`
- `run8e-monitor-420.png` - Local AI ready, qwen2.5:7b
- `run8e-db-counts-after-recovery.json` - sources 6, evidence_items 27, leads 19, drafts 0
- `run8e-01-leads-visible.png` - Story Queue with visible Draft controls
- `run8e-02-draft1-workbench-wizard.png` - Drafting Article route with Generate Draft visible
- `run8e-03-after-draft1-generate.png` - after Enter attempt, still on drafting screen
- `run8e-db-after-draft1.json` - drafts 0
- `run8e-04-after-coordinate-generate.png` - coordinate click landed focus in Article Format field; no progress visible
- `run8e-db-after-coordinate-generate.json` - drafts 0
- `run8e-05-after-keyboard-generate.png` - app returned to main navigation / Story Queue, Local AI ready
- `run8e-db-after-keyboard-generate.json` - drafts 0, empty draft_sample

Database excerpt after final attempt:

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
- Major: 1
- Minor: 0
- Nit: 0

### Blocker - Draft generation route reveals but draft 1 does not persist

Observed: Clicking Draft on a visible Story Queue lead opened the Drafting Article route and showed Generate Draft above the fold. After three activation attempts and two timed waits of at least 90 seconds after direct button-focused attempts, the app had returned to Story Queue/main navigation and SQLite still reported 0 drafts.

Expected: Generate Draft should start visible progress, complete generation, and persist a draft row for the lead. The directive requires then returning to Story Queue, drafting a different lead, and repeating until at least 5 drafts/stories exist.

Impact: Full cleanroom publication flow cannot continue. No editor actions, output export, ZIP, here.now publish, or HTTP verification can be exercised.

Repro:

1. Clean install `The Civic Desk_0.2.8_x64-setup.exe` with the matched directive hash.
2. Launch clean profile and allow setup to reach Local AI ready, qwen2.5:7b.
3. In Story Queue, click Draft on a visible lead.
4. Confirm Drafting Article route with Generate Draft visible.
5. Attempt Generate Draft.
6. Inspect `drafts` in read-only SQLite database.
7. Actual result: `drafts = 0`.

### Major - Directive required full product SHA does not match observed branch head

Observed: Directive requires `8e4fcca6f3d762d32c892858fd56605bce971b4b`, while the remote product branch currently resolves to `8e4fcca694318d2dd121292e0686eb24ac6db98b`. The packaged NSIS/MSI artifact hashes did match the directive.

Expected: Required product commit and branch head should match exactly or the directive should say to test packaged artifacts regardless of branch-head mismatch.

Impact: Tester can validate the installer artifacts as supplied, but source/branch traceability is ambiguous.

## Request For Coder

Please fix or clarify the draft generation activation/persistence path from the Workbench route. The current build proves the first Draft click reaches the drafting route, but no draft row persists before the app returns to Story Queue/main navigation.

Please also clarify the full required product commit SHA in the directive versus the observed `stable-readiness-local-gates` branch head.
