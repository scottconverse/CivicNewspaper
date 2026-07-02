# Tester Report - Setup Visibility Rerun fbfb904

Date: 2026-07-01
Tester machine: Windows 11 Home, MSI Cyborg 15 A13VE, Intel Core i7-13620H, Intel UHD + NVIDIA GeForce RTX 4050 Laptop GPU, 16 GB RAM
Repo: `https://github.com/scottconverse/CivicNewspaper`
Coordination branch: `test-comms/cleanroom-coder-tester`
Product branch: `main`
Product commit represented by installer: `fbfb90464590d0f643cdb8189ee0a44ef5597a5b`
Directive: `test-comms/directives/20260702-setup-visibility-rerun-fbfb904.md`

## Environment

- Windows version: Windows 11 Home
- CPU: Intel Core i7-13620H
- RAM: 16 GB
- GPU: Intel UHD Graphics, NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: recorded in `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/machine-profile.txt`
- Node: not used for this installed-app run
- Rust: not used for this installed-app run
- npm: not used for this installed-app run
- Ollama installed/running: no product-owned `ollama` readiness confirmed
- Models present: database setting `model.selected = phi4-mini:latest`; no ready dashboard/model confirmed in UI

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread `test-comms/ACTIVE_DIRECTIVE.md`, `test-comms/README.md`, `test-comms/protocol.md`, `test-comms/prompts/tester-codex-desktop-prompt.md`, and the active directive.
2. Verified the installer artifact:
   - Path: `test-comms/artifacts/20260702-setup-visibility-rerun-fbfb904/The Civic Desk_0.3.1_x64-setup.exe`
   - SHA256: `D6ABA5A6D17D46AD466BB745D02D9DB3EB3AA5986150A5CC4B17248905A93BF4`
   - Size: `5661647`
3. Stopped stale product processes, uninstalled the prior app, removed app state, and installed only the directive NSIS artifact.
4. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally and verified a visible desktop app window titled `The Civic Desk`.
5. Entered Longmont cleanroom identity fields through the UI. The setup view required resizing/repositioning/scrolling to expose lower identity fields and `Next`.
6. Filled visible city/state fields as `Longmont` and `CO`, then clicked `Next`.
7. Observed the app disappear/exit instead of advancing to AI Service Setup.
8. Relaunched normally one time as requested. The app returned to AI Setup step 1 with identity not restored and no saved setup step.
9. Captured final database/window/process snapshot and stopped before source discovery because setup was blocked.

## Results

Overall result: BLOCKED.

The setup visibility recovery fix did not pass cleanroom validation. The app stayed visible while editing AI Setup identity fields, but after the required Longmont city/state values were entered and `Next` was clicked, the app did not advance to AI Service Setup. The process was no longer present immediately after the click. A normal relaunch returned to AI Setup step 1 with no identity restore and no saved setup step.

Because setup did not complete:

- Source discovery did not run.
- Daily Scan did not run.
- Story Queue was not available.
- The city-news chrome/source-grounding audit was not reachable.
- No draft, Workbench, export, ZIP/package, or here.now publish was attempted.

## Database Counts

At the blocked final snapshot:

- sources: `0`
- daily_scan_runs: `0`
- daily_scan_leads: `0`
- Story Queue leads: `0`
- evidence_items: `0`
- lead_evidence: `0`
- drafts: `0`
- publish_runs: `0`
- published_posts: `0`

Settings observed:

- `model.selected`: `phi4-mini:latest`
- no `identity.*` settings
- no `onboarding.step`

## Required Audits

- AI Setup identity Next visible advance: BLOCKED. `Next` did not advance to AI Service Setup.
- Relaunch recovery: BLOCKED. Relaunch returned to AI Setup step 1 without restored identity or setup step.
- Product-owned runtime/model setup: not reached.
- Evidence-linkage audit for ready-to-draft leads: not reached.
- City news/category/index chrome rescue audit for `https://www.longmontcolorado.gov/news`: not reached.
- Tourism/calendar navigation rescue audit: not reached.
- City-site navigation rescue audit: not reached.
- Summer Reading prior-failure audit: not reached.
- Draft/Workbench/editor workflow: not reached.
- ZIP/package path: none produced.
- here.now URL: none produced.
- Output quality audit: not applicable because no output was generated.

## Evidence

- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/install-clean-launch.log`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/machine-profile.txt`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/blocked-final-db-window-snapshot.json`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-01-after-launch.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-02-after-longmont-starter.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-07-identity-entered-before-next.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-11-city-state-next-visible.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-23-bottom-next-shifted-left.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-24-after-click-next.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-25-after-city-state-next.png`
- `test-comms/evidence/20260702-setup-visibility-rerun-fbfb904/screenshot-26-one-relaunch-recovery.png`

## Findings

Severity counts:

- Blocker: `1`
- Critical: `0`
- Major: `0`
- Minor: `0`
- Nit: `0`

### Blocker - Identity Next exits/disappears before AI Service Setup and relaunch does not restore identity

Observed: On clean install, AI Setup identity was visible. After entering a Longmont cleanroom identity and clicking `Next`, the app did not advance to AI Service Setup. The `civicnews` process was gone immediately after the click. A normal relaunch returned to AI Setup step 1 with blank/restarter identity state. The database contained only `model.selected = phi4-mini:latest`; no `identity.*` values and no `onboarding.step`.

Expected: Identity values should persist on pointer press, the setup step should advance to AI Service Setup, and relaunch recovery should restore identity and/or the saved step if a renderer/window transition fails.

Impact: Cleanroom users cannot complete first-run setup, so the required Longmont source discovery, Daily Scan, Story Queue, draft, export, and publish flow cannot run.

Repro: Clean wipe state, install `The Civic Desk_0.3.1_x64-setup.exe` from `20260702-setup-visibility-rerun-fbfb904`, launch, fill AI Setup identity including `Longmont` and `CO`, click `Next`, then relaunch.

## Request For Coder

Please fix or further instrument the AI Setup identity step transition in build `fbfb904`. The app still cannot reliably advance from identity to AI Service Setup, and the intended persistence/relaunch recovery did not leave `identity.*` or `onboarding.step` in the database.
