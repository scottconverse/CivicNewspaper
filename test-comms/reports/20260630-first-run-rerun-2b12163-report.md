# Tester Report - first-run rerun 2b12163

Date: 2026-07-01T03:25:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 2b12163f9f8791d847fb17ede820a24817896339
Directive: test-comms/directives/20260630-first-run-rerun-2b12163.md
Result: FAIL / BLOCKED before Step 1 interaction

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376533483520 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: no retained test model state; prior `.ollama\models` path absent during clean wipe
- Models present: none verified through product UI because installed UI did not render

## Summary

The installer hash and byte size matched the directive, and silent install completed. The run is blocked earlier than the previous `fe19b40` run: the installed app process starts and exposes a native window handle titled `The Civic Desk`, but the app window does not visibly render on the desktop. Restoring, moving, foregrounding, killing, and relaunching the installed app still left only the desktop visible.

Because the first-run UI never rendered, I could not verify the directive's first gate: clicking the Longmont starter profile and then clicking the visible Next button. I also could not test AI setup, source discovery, scan depth, draft generation, Workbench approval blockers, static export, here.now publish, duplicate-topic audit, or public-output safety.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-first-run-rerun-2b12163.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-first-run-rerun-2b12163/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5632010`
   - SHA256: `798D5FA706D7EF4A8284ACF0ED7012367D17B1806E32779467C4B1D76B8521A2`
3. Wrote and pushed visibility report `test-comms/reports/20260630-first-run-rerun-2b12163-visibility.md`.
4. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Checked prior `.ollama\models` path; it was already absent.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Captured desktop screenshot with `civicnews` process running but no visible Civic Desk window.
8. Used native window APIs to show, move, and foreground the window handle.
9. Captured a second screenshot; still no visible Civic Desk window.
10. Killed and relaunched `civicnews`, then showed/moved/foregrounded the new handle.
11. Captured a third screenshot; still no visible Civic Desk window.
12. Captured DB snapshot and runtime diagnostics.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app process launch: PASS.
- Visible installed app window: FAIL.
- Natural first-run setup route: BLOCKED. The UI did not render.
- Step 1 Longmont starter profile stays on Step 1: BLOCKED.
- Step 1 visible Next advances to Step 2: BLOCKED.
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
- Target 5-10 reader-facing stories/briefs: BLOCKED before Step 1 interaction.

## Evidence

Evidence folder: `test-comms/evidence/20260630-first-run-rerun-2b12163/`

- `cleanwipe-install.log`
- `db-snapshot-after-invisible-window-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-first-run-step1.png`
- `screenshot-02-window-restore.png`
- `screenshot-03-relaunch-window-check.png`

Runtime diagnostics after relaunch:

```text
ProcessName      : civicnews
Id               : 24328
MainWindowTitle  : The Civic Desk
MainWindowHandle : 16122022
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

DB snapshot after launch:

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

### Blocker: Installed app starts but renders no visible first-run window

Observed: After clean wipe and install, launching `civicnews.exe` creates a running process and a native `The Civic Desk` window handle. The actual app content does not render visibly. `ShowWindow`, `MoveWindow`, and `SetForegroundWindow` placed the handle at normal screen coordinates, but screenshots still show only the desktop. Killing and relaunching the process reproduced the same state.

Expected: The installed app should show the first-run setup UI so the tester can click Longmont and advance with the visible Next button.

Impact: This blocks all user-facing first-run validation and prevents the draft-safety/publication gates in the directive from being tested.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-first-run-rerun-2b12163/`.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
4. Observe `civicnews` process and native title/handle exist, but no Civic Desk UI is visible.
5. Restore/move/foreground or relaunch the app.
6. Observe no visible app UI and unchanged setup DB state.

## Request For Coder

Please fix or instrument installed-app launch rendering before the Step 1 Next fix can be validated. Once the first-run UI visibly renders again, issue a rerun directive for the same first-run Step 1 gate plus the draft-safety gates.
