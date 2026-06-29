# Targeted ASCII UI Rerun Report - 607b0f3

Status: PASS

Directive: `test-comms/directives/20260629-ascii-ui-rerun-607b0f3.md`

Product branch: `stable-readiness-local-gates`

Product commit tested: `607b0f3bb79b97a4f7cbb0a2286a8722b9a78b34`

Evidence folder: `test-comms/reports/20260629-ascii-ui-rerun-607b0f3-evidence/`

## Installed Artifact

Preferred NSIS installer:

`test-comms/artifacts/20260629-ascii-ui-rerun-607b0f3/The Civic Desk_0.2.8_x64-setup.exe`

Observed NSIS SHA256:

`B9AF797EE8CEDF81BDE8761BE3FAAE34DA1CE00D122F3227AA0258272611BD1B`

Expected NSIS SHA256:

`B9AF797EE8CEDF81BDE8761BE3FAAE34DA1CE00D122F3227AA0258272611BD1B`

Fallback MSI hash was also checked:

`C79A80C855CE2131BF599DD80A9A5BD65CB2BDC9C1BCBE2A33190E0410DDE83E`

The NSIS installer was used. MSI fallback was not needed.

Evidence:

- `installer-hashes.json`
- `install-result.json`
- `launch-result.json`

## Visible UI Observations

The app launched successfully with the existing Longmont test profile.

Exact visible city/state text observed in the sidebar:

`LONGMONT / CO`

The previous middle-dot mojibake rendering did not appear in the captured visible text.

Captured surfaces:

- `01-first-visible-workspace.png`
- `02-story-queue-sidebar.png`
- `03-daily-scan-labels.png`
- `04-after-refresh-loading-state.png`

Text evidence:

- `01-first-visible-workspace.txt`
- `02-story-queue-sidebar.txt`
- `03-daily-scan-labels.txt`
- `04-after-refresh-loading-state.txt`

Daily Scan/source context examples used ASCII-safe separators, including:

- `CITY CLERK'S OFFICE / PUBLIC NOTICE`
- `CITY WEBSITE / OFFICIAL UPDATE`
- `CITY OF LONGMONT PUBLIC INFORMATION DEPARTMENT / COMMUNITY SIGNAL`

## Mojibake Scan

The visible UI scanner checked for the required bad byte/display patterns by codepoint:

- `U+00C3 U+201A`
- `U+00C3 U+0192`
- `U+00EF U+00BF U+00BD`
- `U+00C3 U+00A2`

Visible UI scan result:

- Hit count: 0
- Hit snippets: none

Saved artifact scan result:

- Text/JSON artifact files scanned: 8
- Hit count: 0
- Hit snippets: none

Evidence:

- `visible-ui-scan.json`
- `saved-artifact-mojibake-scan.json`

## Final Result

PASS.

The installer hash matched, the app launched, the sidebar rendered `LONGMONT / CO`, source context labels used ASCII-safe separators, and the visible UI plus saved text/JSON artifacts had zero mojibake hits for the required scan patterns.
