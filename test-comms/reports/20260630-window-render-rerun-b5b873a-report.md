# Tester Report - window render rerun b5b873a

Date: 2026-07-01T03:57:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: b5b873a0da6ee9712a8ca1633464c6ee261dd5fc
Directive: test-comms/directives/20260630-window-render-rerun-b5b873a.md
Result: FAIL / BLOCKED at first-run Step 1

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376540712960 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: no retained test model state; prior `.ollama\models` path absent during clean wipe
- Models present: none verified through product flow because first-run setup blocked at Step 1

## Summary

The installer hash and byte size matched the directive. The product clean wipe and silent install completed. The packaged window render gate passed: launching the installed EXE normally produced a visible `The Civic Desk` native window with visible `AI Setup` Step 1 content within the 30-second gate, without ShowWindow/MoveWindow/SetForegroundWindow/taskbar manipulation.

The run then failed at the next required gate. Clicking the `Longmont` starter profile did not fill the identity fields. Clicking the visible `Next` button did not advance to Step 2; the wizard remained on `AI Setup` Step 1. A second clear retry after dismissing an overlapping Codex notification reproduced the same Step 1 result. Because Step 1 did not advance, I did not continue into AI setup, source discovery, scan, draft generation, Workbench approval, static export, or here.now publish.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-window-render-rerun-b5b873a.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-window-render-rerun-b5b873a/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5629721`
   - SHA256: `A3FDF4BCA93EFBC77A085C5C96063F419DBA640C4B9CA8F913B053BBC5A5439D`
3. Wrote and pushed visibility report `test-comms/reports/20260630-window-render-rerun-b5b873a-visibility.md`.
4. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Checked prior `.ollama\models` path; it was already absent.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
7. Waited 30 seconds with no handle manipulation and captured screenshot. Result: visible Step 1 window rendered.
8. Clicked `Longmont`. Result: screen remained on Step 1; identity fields did not fill.
9. Clicked visible `Next`. Result: did not advance to Step 2.
10. Dismissed an overlapping Codex notification and retried `Longmont` then `Next`. Result: still stayed on Step 1.
11. Captured DB snapshot and runtime diagnostics.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app process launch: PASS.
- Visible installed app window without manipulation: PASS.
- Natural first-run setup route: PASS to Step 1.
- Step 1 Longmont starter profile fills fields and remains on Step 1: FAIL.
- Step 1 visible Next advances to Step 2: FAIL.
- Setup wording clarity: PARTIAL. Step 1 is readable, but later setup copy could not be reached.
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
- Target 5-10 reader-facing stories/briefs: BLOCKED at first-run Step 1.

## Evidence

Evidence folder: `test-comms/evidence/20260630-window-render-rerun-b5b873a/`

- `cleanwipe-install-launch.log`
- `db-snapshot-after-step1-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-normal-launch-after-30s.png`
- `screenshot-02-longmont-click-stays-step1.png`
- `screenshot-03-next-advances-step2.png`
- `screenshot-04-longmont-retry-clear.png`
- `screenshot-05-next-retry-result.png`

Important note on screenshot names: `screenshot-03-next-advances-step2.png` was named before inspection; it does not prove Step 2. The later inspected screenshots and DB state confirm the wizard remained blocked at Step 1.

Runtime diagnostics:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

DB snapshot after Step 1 failure:

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
- Minor: 1
- Nit: 0

### Blocker: Step 1 Longmont/Next gate does not work

Observed: The installed app rendered the first-run Step 1 UI normally. Clicking `Longmont` did not fill the optional identity fields. Clicking the visible `Next` button left the wizard on Step 1 rather than advancing to Step 2. Retrying after clearing an overlapping notification reproduced the same state.

Expected: Clicking `Longmont` should fill fields while staying on Step 1, and clicking `Next` should advance to Step 2 without disappearing, minimizing, or losing setup state.

Impact: This blocks first-run setup and prevents all downstream AI setup, source discovery, scanning, drafting, approval, export, publish, and output-quality validation.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-window-render-rerun-b5b873a/`.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
4. Wait for visible `AI Setup` Step 1.
5. Click `Longmont`.
6. Observe fields do not fill.
7. Click visible `Next`.
8. Observe the wizard remains on Step 1.

### Minor: Overlapping Codex notification obscured part of the first Step 1 attempt

Observed: A Codex desktop notification overlapped the lower right of the app during the first Longmont/Next attempt.

Expected: No external notification should obscure the app during a clean click attempt.

Impact: Low. I dismissed the notification and reproduced the Step 1 failure on a clear view, so this was not the root cause.

## Request For Coder

The packaged window render fix appears successful. Please fix or instrument first-run Step 1 button handling: Longmont starter button should populate the fields, and the visible Next button should advance to Step 2. Once that is fixed, issue another rerun directive for Step 1 plus the draft-safety gates.
