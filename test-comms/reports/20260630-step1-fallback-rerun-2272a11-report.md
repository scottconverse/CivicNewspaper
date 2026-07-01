# Tester Report - Step 1 fallback rerun 2272a11

Date: 2026-07-01T04:10:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: 2272a1114b340cdfc7276c5aafd88da4a4101a74
Directive: test-comms/directives/20260630-step1-fallback-rerun-2272a11.md
Result: FAIL / BLOCKED at first-run Step 1 Longmont field-fill gate

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376496586752 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: no retained test model state; prior `.ollama\models` path absent during clean wipe
- Models present: none verified through product flow because first-run setup blocked at Step 1

## Summary

The installer hash and byte size matched the directive. The product clean wipe and silent install completed. The installed app launched normally and rendered visible `AI Setup` Step 1 content without handle manipulation, so the window render gate passed.

The first required Step 1 fallback gate failed. Clicking `Longmont` did not populate the Publication Name, Editor Name, City, or State fields. Per directive, I stopped the workflow at that first-gate failure and did not continue into Next, AI setup, source discovery, scanning, drafting, approval, export, or here.now publish.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-step1-fallback-rerun-2272a11.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-step1-fallback-rerun-2272a11/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5629063`
   - SHA256: `600E99133A33939003AC2220D1AF0E423F4515EA5FBC7019094EFAD12ACC8E25`
3. Wrote and pushed visibility report `test-comms/reports/20260630-step1-fallback-rerun-2272a11-visibility.md`.
4. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Checked prior `.ollama\models` path; it was already absent.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
7. Waited 30 seconds with no handle manipulation and captured screenshot. Result: visible Step 1 window rendered.
8. Clicked `Longmont` at the visible starter-profile button.
9. Captured screenshot showing fields remained empty.
10. Captured DB snapshot and runtime diagnostics.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app process launch: PASS.
- Visible installed app window without manipulation: PASS.
- Natural first-run setup route: PASS to Step 1.
- Step 1 Longmont starter profile fills Publication Name, Editor Name, City, and State: FAIL.
- Step 1 visible Next advances to Step 2: BLOCKED because the Longmont field-fill gate failed first.
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

Evidence folder: `test-comms/evidence/20260630-step1-fallback-rerun-2272a11/`

- `cleanwipe-install-launch.log`
- `db-snapshot-after-longmont-click-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-normal-launch-after-30s.png`
- `screenshot-02-longmont-click-result.png`

Runtime diagnostics:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

DB snapshot after Longmont click:

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

### Blocker: Longmont starter profile does not populate Step 1 fields

Observed: The installed app rendered `AI Setup` Step 1 normally. Clicking the visible `Longmont` starter-profile button left the Publication Name, Editor Name, City, and State fields empty.

Expected: Clicking `Longmont` should fill Publication Name, Editor Name, City, and State while remaining on Step 1.

Impact: This fails the directive's first Step 1 gate and blocks the remaining setup and journalism-quality workflow.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-step1-fallback-rerun-2272a11/`.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
4. Wait for visible `AI Setup` Step 1.
5. Click `Longmont`.
6. Observe Publication Name, Editor Name, City, and State remain empty.

## Request For Coder

The packaged window render still passes. Please fix or instrument the Step 1 starter-profile click/fallback so `Longmont` populates all required identity fields. Once that first gate passes, issue another rerun directive for Longmont field-fill, Next-to-Step-2, and the downstream draft-safety gates.
