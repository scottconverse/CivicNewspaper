# Tester Report - Step 1 button rerun f8da868

Date: 2026-07-01T04:40:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: f8da86833407ee8889978cd5cd52582a0846630e
Directive: test-comms/directives/20260630-step1-button-rerun-f8da868.md
Result: FAIL / BLOCKED at visible-window gate

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376478564352 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: no retained test model state; prior `.ollama\models` path absent during clean wipe
- Models present: none verified through product flow because the visible-window gate failed

## Summary

The installer hash and byte size matched the directive. The product clean wipe and silent install completed. However, this build failed the required visible-window gate before Step 1 could be tested.

After launching `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally and waiting 30 seconds with no handle manipulation, the desktop screenshot showed no visible Civic Desk app content. The `civicnews` process was running and Windows reported `MainWindowTitle: The Civic Desk`, but the app window was not visible on the desktop.

Per directive, I stopped the workflow and did not use ShowWindow, MoveWindow, SetForegroundWindow, taskbar tricks, or handle manipulation to force visibility. Step 1 prefill, Next-to-Step-2, identity DB persistence, AI setup, source discovery, scan, drafting, export, and here.now publish were not tested.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-step1-button-rerun-f8da868.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-step1-button-rerun-f8da868/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5629565`
   - SHA256: `B27AF20ED8A30685F4928A23475E0BB959959D26FA23EF9CD91535548CF71AE4`
3. Wrote and pushed visibility report `test-comms/reports/20260630-step1-button-rerun-f8da868-visibility.md`.
4. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Checked prior `.ollama\models` path; it was already absent.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
7. Waited 30 seconds with no handle manipulation.
8. Captured desktop screenshot showing no visible Civic Desk app content.
9. Captured DB snapshot and runtime diagnostics.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app process launch: PASS.
- Visible installed app window without manipulation: FAIL.
- Natural first-run setup route: BLOCKED.
- Step 1 identity fields fill through Longmont or no-input recovery: BLOCKED.
- Step 1 visible Next advances to Step 2: BLOCKED.
- Identity settings persisted before leaving Step 1: BLOCKED.
- Setup wording clarity: BLOCKED.
- App-guided AI setup: BLOCKED.
- Source discovery/addition: BLOCKED.
- Scan: BLOCKED.
- Enough-leads behavior: BLOCKED.
- Local AI draft path: BLOCKED.
- Writer/editor workflow: BLOCKED.
- Workbench block for lead-based draft with linked sources but no inline evidence citation: BLOCKED.
- Unsupported high-risk claim safety: BLOCKED.
- Static export/package: BLOCKED.
- here.now publish: BLOCKED.
- Public output quality scan: BLOCKED.
- Duplicate-topic audit: BLOCKED.
- Target 5-10 reader-facing stories/briefs: BLOCKED at visible-window gate.

## Evidence

Evidence folder: `test-comms/evidence/20260630-step1-button-rerun-f8da868/`

- `cleanwipe-install-launch.log`
- `db-snapshot-after-window-gate-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-normal-launch-after-30s.png`

Runtime diagnostics:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk
MainWindowHandle : 3015832
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

DB snapshot after failed window gate:

```json
{
  "settings": [
    {
      "key": "model.selected",
      "value": "phi4-mini:latest"
    }
  ],
  "sources": 0,
  "leads": 0,
  "drafts": 0
}
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

### Blocker: Installed app process starts but no visible app content appears

Observed: After clean wipe and install, launching the installed EXE normally produced a running `civicnews` process and `The Civic Desk` main window title/handle, but no visible app content appeared on the desktop within 30 seconds.

Expected: The installed app should render a visible desktop window without tester manipulation.

Impact: This blocks the first-run Step 1 button fix validation and all downstream Longmont workflow gates.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-step1-button-rerun-f8da868/`.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
4. Wait 30 seconds without handle manipulation.
5. Observe no visible Civic Desk app content despite a running `civicnews` process and `The Civic Desk` main window title.

## Request For Coder

This build regressed or reintroduced the visible-window gate failure seen before. Please restore the packaged visible-window behavior, then reissue a rerun directive for the Step 1 real-button fix and identity persistence gates.
