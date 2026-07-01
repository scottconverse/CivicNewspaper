# Tester Report - Step 1 prefill rerun e632108

Date: 2026-07-01T04:25:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: e63210807daa4edffd2f1f6d2daa0b3b0faf9c30
Directive: test-comms/directives/20260630-step1-prefill-rerun-e632108.md
Result: FAIL / BLOCKED at Step 1 Next-to-Step-2 gate

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 376498888704 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: no retained test model state; prior `.ollama\models` path absent during clean wipe
- Models present: none verified through product flow because first-run setup blocked before Step 2

## Summary

The installer hash and byte size matched the directive. The product clean wipe and silent install completed. The installed app launched normally and rendered visible `AI Setup` Step 1 content without handle manipulation.

The new no-input recovery worked: after waiting on Step 1, the app displayed a notice saying the Longmont starter profile was filled automatically because setup did not receive input. This satisfies the Step 1 field-fill recovery intent.

The next gate still failed. Clicking the visible `Next` control did not show Step 2; the app window disappeared and the desktop was visible. Per directive, I stopped at that first-gate failure and did not continue into AI setup, source discovery, scanning, drafting, approval, export, or here.now publish.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-step1-prefill-rerun-e632108.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-step1-prefill-rerun-e632108/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5631510`
   - SHA256: `08F780AFA4AAFFC36FE920C374816A2B44F01A4BA8F6B7F9322FBB833430E6ED`
3. Wrote and pushed visibility report `test-comms/reports/20260630-step1-prefill-rerun-e632108-visibility.md`.
4. Performed product clean wipe:
   - Stopped `civicnews` and `ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Checked prior `.ollama\models` path; it was already absent.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
7. Waited 30 seconds with no handle manipulation and captured screenshot.
8. Observed visible Step 1 plus the no-input recovery notice: `The Longmont starter profile was filled automatically because setup did not receive input. You can edit these fields before continuing.`
9. Clicked the visible `Next` control.
10. Captured screenshot showing the app window gone and desktop visible.
11. Captured DB snapshot and runtime diagnostics.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app process launch: PASS.
- Visible installed app window without manipulation: PASS.
- Natural first-run setup route: PASS to Step 1.
- Step 1 identity fields fill through Longmont or no-input recovery: PASS by no-input recovery notice.
- Step 1 visible Next advances to Step 2: FAIL.
- Setup wording clarity: PARTIAL. Step 1 recovery wording is understandable, but later setup copy could not be reached.
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
- Target 5-10 reader-facing stories/briefs: BLOCKED at first-run Step 1 Next.

## Evidence

Evidence folder: `test-comms/evidence/20260630-step1-prefill-rerun-e632108/`

- `cleanwipe-install-launch.log`
- `db-snapshot-after-next-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-normal-launch-after-30s.png`
- `screenshot-02-next-to-step2-result.png`

Runtime diagnostics:

```text
ProcessName      : civicnews
MainWindowTitle  : The Civic Desk
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

DB snapshot after failed Next:

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

### Blocker: Step 1 Next hides the app instead of advancing to Step 2

Observed: The installed app rendered Step 1 and the no-input Longmont recovery notice appeared. Clicking the visible `Next` control caused the app window to disappear, leaving the desktop visible, and Step 2 did not appear.

Expected: Clicking `Next` after the Step 1 recovery fills fields should advance to Step 2.

Impact: This blocks first-run setup and prevents the downstream AI setup, source discovery, scanning, drafting, approval, export, publish, and output-quality validation required by the directive.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from `test-comms/artifacts/20260630-step1-prefill-rerun-e632108/`.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe` normally.
4. Wait on Step 1 until the Longmont no-input recovery notice appears.
5. Click the visible `Next` control.
6. Observe the app window disappears and Step 2 does not appear.

## Request For Coder

The no-input Step 1 Longmont recovery now works. Please fix or instrument the Step 1 Next control so it advances to Step 2 after recovery-filled fields. Once that gate passes, issue another rerun directive for Step 2 and the downstream draft-safety/publication gates.
