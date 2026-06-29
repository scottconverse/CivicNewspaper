# Targeted ASCII UI Rerun - Product Commit 607b0f3

Tester role: cleanroom tester for CivicNewspaper on `msi\civic`.

Coordination branch: `test-comms/cleanroom-coder-tester`

Product branch: `stable-readiness-local-gates`

Product commit: `607b0f3bb79b97a4f7cbb0a2286a8722b9a78b34`

Artifact folder: `test-comms/artifacts/20260629-ascii-ui-rerun-607b0f3/`

Preferred NSIS installer: `test-comms/artifacts/20260629-ascii-ui-rerun-607b0f3/The Civic Desk_0.2.8_x64-setup.exe`

Expected NSIS SHA256: `B9AF797EE8CEDF81BDE8761BE3FAAE34DA1CE00D122F3227AA0258272611BD1B`

Fallback MSI installer: `test-comms/artifacts/20260629-ascii-ui-rerun-607b0f3/The Civic Desk_0.2.8_x64_en-US.msi`

Expected MSI SHA256: `C79A80C855CE2131BF599DD80A9A5BD65CB2BDC9C1BCBE2A33190E0410DDE83E`

Report path: `test-comms/reports/20260629-ascii-ui-rerun-607b0f3-report.md`

Evidence folder: `test-comms/reports/20260629-ascii-ui-rerun-607b0f3-evidence/`

## Why This Rerun Exists

The previous full duplicate rerun passed the public publication checks, but its evidence captured in-app mojibake in the sidebar: `LONGMONT Â· CO`.

This product commit replaces vulnerable UI separators and ellipsis characters with ASCII-safe text:

- `LONGMONT / CO` instead of a middle-dot separator.
- `...` instead of Unicode ellipsis in loading/saving UI.
- Source context labels use ` / ` instead of a middle-dot separator.

## Required Test

This is a targeted installer rerun, not a full new publication-quality cleanroom run unless the app forces it.

1. Fetch `test-comms/ACTIVE_DIRECTIVE.md` and verify this file is the active directive.
2. Verify the installer hash before installing.
3. Install from the preferred NSIS installer.
4. Launch the app.
5. Use a clean or existing test profile, whichever is faster, but ensure the identity is set to:
   - City: `Longmont`
   - State: `CO`
6. Capture screenshots/text evidence for:
   - First visible workspace after setup.
   - Sidebar/brand area with the city/state kicker.
   - Story Queue or Daily Scan lead list showing source context labels.
   - Any loading/saving state you can trigger without creating unnecessary new content.
7. Search extracted visible text and saved HTML/text artifacts for mojibake:
   - `Â`
   - `Ã`
   - `�`
   - `â`
8. Confirm the UI shows `LONGMONT / CO` or equivalent ASCII-safe city/state text, not `LONGMONT Â· CO`.
9. Confirm source context labels show ASCII-safe separators and no mojibake.

## Pass Criteria

Pass only if:

- Installer hash matches.
- App launches.
- Sidebar/city-state text has no mojibake.
- Source context labels have no mojibake.
- The visible UI scan finds zero mojibake hits.

Fail if:

- Any `Â`, `Ã`, `�`, or `â` appears in visible app UI.
- The app cannot launch.
- The installer hash does not match.

## Report Requirements

Write the report to:

`test-comms/reports/20260629-ascii-ui-rerun-607b0f3-report.md`

Include:

- PASS or FAIL.
- Installed artifact path and observed SHA256.
- Product commit tested.
- Screenshots/text evidence paths.
- Exact visible city/state text observed.
- Mojibake hit count and any hit snippets.

Commit reports/artifacts to the coordination branch with `[skip ci]`.
