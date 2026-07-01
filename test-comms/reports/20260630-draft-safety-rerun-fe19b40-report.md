# Tester Report - draft safety rerun fe19b40

Date: 2026-07-01T03:04:00Z
Tester machine: Windows Intel cleanroom laptop, 16 GB RAM, Intel UHD Graphics plus NVIDIA GeForce RTX 4050 Laptop GPU
Repo: https://github.com/scottconverse/CivicNewspaper
Product branch: main
Product commit represented by installer: fe19b407102d90256e14bb80fc63577a6fc16890
Directive: test-comms/directives/20260630-draft-safety-rerun-fe19b40.md
Result: FAIL / BLOCKED at first-run setup

## Environment

- Windows version: Windows 10 Home, build family 10.0.26100.1
- CPU: 13th Gen Intel(R) Core(TM) i7-13620H
- RAM: 16870060032 bytes
- GPU: Intel(R) UHD Graphics; NVIDIA GeForce RTX 4050 Laptop GPU
- Disk free: 372123074560 bytes on C: before install
- Node: not on PATH
- Rust: not on PATH
- npm: not on PATH
- Ollama installed/running before product run: not on PATH; no model state retained after clean wipe
- Models present: removed prior `C:\Users\civic\.ollama\models` during product clean wipe

## Summary

The installer hash and byte size matched the directive, and the installed app launched into natural first-run setup from a clean product profile. The run is blocked because the installed first-run setup UI could not be advanced through Step 1.

The first-run screen was visible at `AI Setup`, `Step 1 of 5`, with the Longmont starter profile and optional identity fields. Clicking the visible `Next` button caused the Civic Desk window to disappear/minimize, left the process running with a blank `MainWindowTitle`, and did not write any setup state beyond the default selected model. Keyboard-only activation of `Next` produced the same blocked state. Because setup never advanced, I could not truthfully test source discovery, scan depth, draft generation, Workbench approval blocking, static export, here.now publish, duplicate-topic audit, or public-output safety on this build.

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and confirmed `test-comms/ACTIVE_DIRECTIVE.md` points to `test-comms/directives/20260630-draft-safety-rerun-fe19b40.md`.
2. Verified installer:
   - Path: `test-comms/artifacts/20260630-draft-safety-rerun-fe19b40/The Civic Desk_0.3.1_x64-setup.exe`
   - Size: `5633326`
   - SHA256: `967F12990DC98B2B70498872DBD278E3514509E0A74E8E5B3F5C457B3B5E6D20`
3. Wrote and pushed visibility report `test-comms/reports/20260630-draft-safety-rerun-fe19b40-visibility.md`.
4. Performed a product clean wipe:
   - Stopped `civicnews`/`ollama` if present.
   - Ran prior `The Civic Desk\uninstall.exe /S`.
   - Removed Civic Desk app data under `AppData\Roaming\com.scottconverse.civicdesk` and `AppData\Local\com.scottconverse.civicdesk`.
   - Removed prior local CivicNews output paths if present.
   - Removed prior `.ollama\models` test model state.
5. Installed the directive NSIS installer silently with `/S`.
6. Launched `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
7. Captured natural first-run setup screen.
8. Tried to use Step 1 as an end user:
   - First attempt: click Longmont starter profile, click visible `Next`.
   - Second attempt after relaunch: fill optional publication/editor fields with synthetic input, click visible `Next`.
   - Third attempt: low-level input and clipboard paste into fields, then attempt to advance.
   - Fourth attempt: restore window and attempt keyboard-only tab/enter activation.
9. Confirmed after attempts that `civicdesk.db` still only contains `settings: model.selected = phi4-mini:latest`; sources, leads, drafts, and publish runs remain zero.

## Results

- Installer hash and size: PASS.
- Clean product wipe: PASS.
- Real installed app launch: PASS.
- Natural first-run setup route: PASS.
- Setup wording clarity: PARTIAL. Step 1 wording is understandable and says identity fields are optional, but I could not evaluate later steps.
- App-guided AI setup: BLOCKED. Setup could not advance past Step 1.
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
- Target 5-10 reader-facing stories/briefs: BLOCKED at first-run setup.

## Evidence

Evidence folder: `test-comms/evidence/20260630-draft-safety-rerun-fe19b40/`

- `cleanwipe-install.log`
- `db-snapshot-after-first-run-block.json`
- `runtime-diagnostics.txt`
- `screenshot-01-first-launch.png`
- `screenshot-02-first-run-visible.png`
- `screenshot-03-first-run-product-visible.png`
- `screenshot-04-setup-step2.png`
- `screenshot-04b-after-click.png`
- `screenshot-05-relaunch-after-window-loss.png`
- `screenshot-06-after-movewindow.png`
- `screenshot-07-taskbar-restore.png`
- `screenshot-08-taskbar-civic-click.png`
- `screenshot-09-relaunch-visible-check.png`
- `screenshot-10-setup-after-step1.png`
- `screenshot-11-step1-filled-check.png`
- `screenshot-12-lowlevel-input-check.png`
- `screenshot-13-after-msi-close.png`
- `screenshot-14-msi-closed-direct.png`
- `screenshot-15-after-next-click.png`
- `screenshot-16-keyboard-next-attempt.png`

Key DB snapshot after the blocker:

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

Runtime diagnostics after the blocker show:

```text
ProcessName      : civicnews
MainWindowTitle  :
Path             : C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe
```

## Findings

Severity counts:

- Blocker: 1
- Critical: 0
- Major: 0
- Minor: 1
- Nit: 0

### Blocker: First-run setup cannot advance past Step 1

Observed: From the natural first-run `AI Setup` Step 1 screen, clicking the visible `Next` button caused the Civic Desk window to disappear/minimize and left the process running without a visible main title. The database did not advance setup state. Keyboard-only activation also failed to advance setup state.

Expected: Clicking `Next` from Step 1 should advance to Step 2, especially because the visible identity fields are labeled optional.

Impact: Cleanroom first-run cannot reach AI setup, source discovery, scanning, draft generation, Workbench approval, export, or publish. This blocks the active directive before draft-safety behavior can be validated.

Repro:

1. Clean product state.
2. Install `The Civic Desk_0.3.1_x64-setup.exe` from the directive artifact.
3. Launch `C:\Users\civic\AppData\Local\The Civic Desk\civicnews.exe`.
4. On `AI Setup` Step 1, click `Longmont`.
5. Click the visible `Next` button.
6. Observe the app window disappears/minimizes and setup state does not advance.

### Minor: MSI Center utility stole focus during synthetic-input fallback

Observed: During a low-level input fallback, an unrelated MSI Center privacy dialog appeared and stole focus. I closed it and resumed the product window.

Expected: Environmental vendor utility should not affect normal manual use, but it did interfere with one fallback input attempt.

Impact: This is not the root cause because the same first-run blocker reproduced before and after the MSI dialog was closed, and the app DB remained unchanged.

Repro: Not treated as product repro; captured as environmental noise in screenshots.

## Request For Coder

Please fix or instrument first-run Step 1 so `Next` reliably advances from the installed app with optional identity fields blank. Once that is fixed, issue a rerun directive for the same draft-safety gates: source-bound fallback drafts, Workbench preflight blocking for lead-based drafts without inline citations, unsupported high-risk claim handling, static export, here.now publish, duplicate-topic audit, and public-output scans.
