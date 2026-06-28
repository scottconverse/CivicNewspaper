# Tester Report: Source Intake Fixture Gate

Date: 2026-06-28T04:48Z
Tester machine: Windows 11 Home 10.0.26200, Intel Core i7-13620H, 15.7 GB RAM, Intel UHD Graphics + NVIDIA GeForce RTX 4050 Laptop GPU
Repo: `https://github.com/scottconverse/CivicNewspaper.git`
Product artifact already validated: `513341b`
Directive: `test-comms/directives/20260627-2218-coder-directive-source-intake-fixture-gate.md`
Verdict: **CLEAR WITH MINOR FINDINGS**

Private local user paths are redacted as `<USER>`.

## Fixture Hash

Artifact:

```text
test-comms/artifacts/source-fixtures-20260628/CivicNewspaper-source-intake-fixtures-20260628.zip
```

Expected SHA256:

```text
99DC4DEABC8FE8FCAF3AB4C57C1C65DA390E25A1D44A1DDBA1B1DA6D1C34C43C
```

Observed SHA256:

```text
99DC4DEABC8FE8FCAF3AB4C57C1C65DA390E25A1D44A1DDBA1B1DA6D1C34C43C
```

Hash result: **PASS**

## Steps Run

1. Pulled `test-comms/cleanroom-coder-tester` and reread README, protocol, tester prompt, and directives.
2. Verified and extracted the source fixture artifact.
3. Used the already-installed 513341b desktop app.
4. Opened **Sources**.
5. Opened **Bulk import**.
6. Used the actual Windows file picker from **Load file** for each required fixture.
7. Recorded the review counts and screenshots for each file.
8. Imported selected reviewed sources from two successful fixtures:
   - `colorado-source-list-edge-cases.xlsx`: 6 selected imported.
   - `colorado-source-list-messy.xlsx`: 7 selected imported after duplicate detection against the first import.
9. Confirmed imported sources appeared in the Sources list.
10. Read the app SQLite database read-only to distinguish stored URLs from table display/accessibility rendering.

## Per-File Results

| Fixture | Format | Result | Importable | Duplicate | Skipped | Selected | Reviewable rows? | Notes |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| `colorado-source-list-clean.csv` | CSV | PASS | 40 | 0 | 1 | 16 | Yes | Clean baseline loaded as row review, not one blob. |
| `colorado-source-list-human-notes.txt` | TXT | PASS | 35 | 1 | 11 | 14 | Yes | Bullets/prose produced reviewable URL candidates. |
| `colorado-source-list-messy.xlsx` | XLSX | PASS | 26 before imports; 24 after prior import | 1 before imports; 3 after prior import | 4 | 9 before imports; 7 after prior import | Yes | Multiple-column spreadsheet produced individual candidates. |
| `colorado-source-list-edge-cases.xlsx` | XLSX | PASS with minor normalization findings | 18 | 1 | 2 | 6 | Yes | Candidate cards exposed split URL, punctuation, duplicate, social, YouTube, malformed, and far-right URL cases. |
| `colorado-source-list-briefing.docx` | DOCX | PASS | 50 | 2 | 140 | 19 | Yes | Word document parsed through prose/table-style content into candidates. |
| `colorado-source-list-exported.pdf` | text PDF | PASS | 42 | 1 | 18 | 16 | Yes | Text PDF produced reviewable candidates. |
| `colorado-source-list-scanned-style.pdf` | image-backed PDF | PASS with minor stale-review finding | 0 new | n/a | n/a | n/a | n/a | App showed: `No readable text was found in this PDF. It may be scanned image-only and require OCR.` No crash. |
| `LOngmont URLS2.docx` | Longmont DOCX | PASS | 244 | 0 | 265 | 80 | Yes | Longmont-specific fixture produced many reviewable candidates; not imported because the directive asked for a small safe subset. |

## Import Verification

Imported subsets:

- Edge-case XLSX: app displayed `Bulk imported 6 source(s) successfully.`
- Messy XLSX: app displayed `Bulk imported 7 source(s) successfully.`

The Sources list then showed imported rows with labels and URLs, including:

- `trailing punctuation`
- `http variant`
- `https variant`
- `repeated row A`
- `city`
- `council`
- `Brighton agendas`
- `Denver council`
- `Aurora duplicate`
- `Longmont`
- `Boulder County`
- `Larimer County`

Read-only DB spot-check of `sources` confirmed 13 imported source rows. Stored URLs were mostly clean, for example:

```text
https://www.brightonco.gov/AgendaCenter
https://denver.legistar.com/Calendar.aspx
https://www.longmontcolorado.gov/
https://bouldercounty.gov/
https://www.larimer.gov/
```

## Evidence

Local evidence screenshots were captured under:

```text
work/installed-evidence-source-fixture-gate/
```

Key screenshots:

- `04-sources-open.png`
- `05-bulk-import-modal.png`
- `loaded-colorado-source-list-clean-csv.png`
- `result-colorado-source-list-human-notes-txt.png`
- `result-colorado-source-list-messy-xlsx.png`
- `result-colorado-source-list-edge-cases-xlsx.png`
- `result-colorado-source-list-briefing-docx.png`
- `result-colorado-source-list-exported-pdf.png`
- `result-colorado-source-list-scanned-style-pdf.png`
- `result-LOngmont-URLS2-docx.png`
- `09-review-candidates-lower.png`
- `11-edge-cases-import-button.png`
- `13-edge-imported.png`
- `16-messy-imported.png`

## Findings

Severity counts:

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 2
- Nit: 1

### Minor 1: Edge-case URL normalization still lets a trailing parenthesis persist

- Observed: Importing selected candidates from `colorado-source-list-edge-cases.xlsx` persisted `https://denver.legistar.com/Calendar.aspx)` with a trailing `)`.
- Expected: The review/import path should trim trailing punctuation before import.
- Impact: A scanned source may fail or fetch a bad URL if the imported URL includes punctuation from prose/table formatting.
- Repro: Bulk import `colorado-source-list-edge-cases.xlsx`, import the default selected subset, then inspect the Sources list or `sources.url` for `trailing punctuation`.

### Minor 2: Scanned PDF error leaves prior review candidates visible

- Observed: Loading `colorado-source-list-scanned-style.pdf` showed a clear error: `No readable text was found in this PDF. It may be scanned image-only and require OCR.` However, the previous text-PDF review list remained visible underneath.
- Expected: After a scanned/image PDF extraction failure, stale candidates should be cleared or visually marked as belonging to the prior file.
- Impact: A user could be confused and import the previous file's candidates after a failed scanned-PDF load.
- Repro: Load `colorado-source-list-exported.pdf`, then load `colorado-source-list-scanned-style.pdf` in the same Bulk Import modal.

### Nit 1: Sources table visually duplicates/truncates URL text in some rows

- Observed: The Sources table displayed some imported URLs as visually duplicated/truncated strings, for example an elided URL immediately followed by the same URL prefix.
- Expected: The table should show one readable URL string per source row.
- Impact: Mostly presentation/accessibility confusion. A read-only DB spot-check showed most stored URLs were not duplicated.
- Repro: Import the edge-case and messy selected subsets, then inspect the Sources list rows.

## Pass Criteria Assessment

- CSV produces reviewable source candidates: **PASS**
- TXT produces reviewable source candidates: **PASS**
- XLSX produces reviewable source candidates: **PASS**
- DOCX produces reviewable source candidates: **PASS**
- Text PDF produces reviewable source candidates: **PASS**
- Realistic spreadsheets/docs do not flatten into one long line: **PASS**
- Scanned/image PDF behavior is honest/actionable and does not crash: **PASS with minor stale-review caveat**
- User can import selected reviewed sources: **PASS**

## Release Readiness

Source intake is **clean enough for coder-side full GauntletGate/release-candidate prep**, with the two minor cleanup findings above recommended before or during RC hardening.

