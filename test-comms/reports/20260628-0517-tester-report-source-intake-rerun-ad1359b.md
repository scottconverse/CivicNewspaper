# Tester Report: Source Intake Rerun for ad1359b

Date: 2026-06-28T05:17Z
Tester machine: Windows 11-class Windows 10 Home 10.0.26100/26200 family, Intel Core i7-13620H, 15.7 GB RAM, Intel UHD Graphics + NVIDIA GeForce RTX 4050 Laptop GPU
Repo: `https://github.com/scottconverse/CivicNewspaper.git`
Coder branch under test: `stable-readiness-local-gates`
Product commit under test: `ad1359b`
Directive: `test-comms/directives/20260628-0458-coder-directive-source-intake-rerun-ad1359b.md`
Verdict: **PASS / CLEAR**

Private local user paths are redacted as `<USER>`.

## Artifact And Hashes

Artifact tested:

```text
test-comms/artifacts/ad1359b/The-Civic-Desk-0.2.8-ad1359b-windows-x64-cleanroom.zip
```

Installed with the NSIS setup executable from the ZIP.

| File | Expected SHA256 | Observed SHA256 | Result |
| --- | --- | --- | --- |
| `The Civic Desk_0.2.8_x64-setup.exe` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` | PASS |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` | PASS |

Fixture ZIP:

```text
test-comms/artifacts/source-fixtures-20260628/CivicNewspaper-source-intake-fixtures-20260628.zip
```

Expected and observed SHA256:

```text
99DC4DEABC8FE8FCAF3AB4C57C1C65DA390E25A1D44A1DDBA1B1DA6D1C34C43C
```

Fixture hash result: **PASS**

## Environment/Profile Used

- Installed app path: `<USER>\AppData\Local\The Civic Desk\civicnews.exe`
- App data/database path: package-local app data for `com.scottconverse.civicdesk`, redacted.
- Profile choice: started from the prior cleanroom user profile, installed `ad1359b`, then moved only the app-owned `civicdesk.db` files into an app-data backup folder to avoid old 513341b source rows polluting this focused rerun. Onboarding was completed again with local-only values and AI setup skipped.
- Ollama: command-line `ollama` not present on PATH; app showed Local AI offline.
- No live publishing and no provider credentials used.

## Rerun Scope Results

### 1. Edge-case XLSX URL normalization

Result: **PASS**

Loaded:

```text
colorado-source-list-edge-cases.xlsx
```

Observed review counts:

- 18 importable
- 1 duplicate
- 2 skipped
- 6 selected

The raw fixture text still contained:

```text
https://denver.legistar.com/Calendar.aspx).
```

The reviewed candidate link showed:

```text
https://denver.legistar.com/Calendar.aspx
```

Imported the checked edge-case subset. The Sources table showed one link for `trailing punctuation`:

```text
https://denver.legistar.com/Calendar.aspx
```

Read-only DB spot-check confirmed:

```text
(1, 'trailing punctuation', 'https://denver.legistar.com/Calendar.aspx')
```

No trailing `)` was reviewed, imported, or stored.

### 2. Scanned-style PDF stale-review behavior

Result: **PASS**

Loaded successful text PDF first:

```text
colorado-source-list-exported.pdf
```

Observed review counts after existing edge-case imports:

- 37 importable
- 6 duplicate
- 18 skipped
- 11 selected

Then loaded:

```text
colorado-source-list-scanned-style.pdf
```

Observed error:

```text
Something went wrong: No readable text was found in this PDF. It may be scanned image-only and require OCR.
```

The prior text-PDF review panel was cleared. The modal returned to the source-list input state with Load file/Review List controls, so prior candidates were not left visible as if they belonged to the scanned PDF.

### 3. Sources table URL display

Result: **PASS**

After imports, the table showed one titled/ellipsized link per row. Accessibility text exposed one link element and one `Description` URL per source row, for example:

```text
link https://denver.legistar.com/Calendar.aspx Description: https://denver.legistar.com/Calendar.aspx
```

No duplicated/truncated URL fragments were observed in the table. A quick narrow/hover display check showed the elided URL with a single tooltip, not a duplicated inline fragment.

### 4. Regression spot check: ordinary CSV import

Result: **PASS**

Loaded:

```text
colorado-source-list-clean.csv
```

Observed review counts after existing edge-case imports:

- 35 importable
- 5 duplicate
- 1 skipped
- 11 selected

Imported the checked CSV subset. The app displayed:

```text
Bulk imported 11 source(s) successfully.
```

Final read-only DB spot-check:

- 17 total source rows.
- Edge-case rows remained clean.
- CSV rows were imported normally, including Brighton, Denver, Aurora, Boulder County, Longmont, and Larimer sources.

## Evidence

Local screenshot evidence captured under:

```text
work/installed-evidence-source-rerun-ad1359b/
```

Key screenshots:

- `01-edge-imported-sources.png`
- `02-text-pdf-review-before-scanned.png`
- `03-scanned-pdf-cleared-review.png`
- `04-clean-csv-review.png`
- `05-clean-csv-imported-sources.png`
- `06-url-display-check.png`

## Severity Counts

- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Request For Coder

No fixes requested from tester for this focused rerun. The pass target is met: `0 Blocker / 0 Critical / 0 Major / 0 Minor / 0 Nit`.

