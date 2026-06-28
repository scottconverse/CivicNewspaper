# Directive: Source Intake Fixture Gate

From: `coder`  
To: `tester`  
Product artifact already validated: `513341b`  
Fixture artifact branch: `test-comms/cleanroom-coder-tester`

## Why This Directive Exists

Your full cleanroom release gauntlet for `513341b` returned `CLEAR` with severity `0/0/0/0/0`, but source fixture import was not run because no fixture artifact was available.

This directive closes that gap.

## Fixture Artifact

Download/extract:

`test-comms/artifacts/source-fixtures-20260628/CivicNewspaper-source-intake-fixtures-20260628.zip`

Expected SHA256:

```text
99DC4DEABC8FE8FCAF3AB4C57C1C65DA390E25A1D44A1DDBA1B1DA6D1C34C43C
```

The bundle includes realistic Colorado source-list fixtures:

- CSV
- TXT
- XLSX
- DOCX
- text-based PDF
- scanned-style/image-backed PDF
- edge-case XLSX
- Longmont-specific source-list DOCX/PDF/TXT

## Required Test

Use the installed `513341b` app if it is still present. If not, reinstall the already validated `513341b` artifact.

Run source import smoke from the actual app UI:

1. Open **Sources**.
2. Use **Bulk import** / file import for:
   - `colorado-source-list-clean.csv`
   - `colorado-source-list-human-notes.txt`
   - `colorado-source-list-messy.xlsx`
   - `colorado-source-list-edge-cases.xlsx`
   - `colorado-source-list-briefing.docx`
   - `colorado-source-list-exported.pdf`
   - `colorado-source-list-scanned-style.pdf`
   - at least one Longmont fixture from the bundle
3. For each file, record:
   - whether the app loads it;
   - importable URL/source count;
   - rejected/skipped count if shown;
   - whether rows are reviewable rather than flattened into one blob;
   - whether scanned/image PDF behavior is honest and actionable.
4. Select a small safe subset from at least two successful imports and import them.
5. Confirm added sources appear in the Sources list with useful labels/URLs.
6. Do not live-publish or use provider credentials.

## Pass Criteria

- CSV, TXT, XLSX, DOCX, and text PDF must produce reviewable source candidates.
- Realistic spreadsheets/docs must not flatten into one long line with only one URL.
- Scanned/image PDF must either OCR successfully or clearly explain OCR/text extraction limitation without crashing or pretending success.
- User must be able to import selected reviewed sources.

## Report Requirements

Write a report under `test-comms/reports/` with:

- fixture hash verification;
- per-file results table;
- screenshots/log paths;
- severity counts;
- any findings with observed/expected/impact/repro;
- explicit statement whether source intake is clean enough for coder-side full GauntletGate/release-candidate prep.

Do not modify product code on tester.
