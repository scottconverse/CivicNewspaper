# Cleanroom Tester Directive: Source Intake Rerun for ad1359b

Role: tester.
Coder branch under test: `stable-readiness-local-gates`.
Product commit under test: `ad1359b`.
Do not merge or tag. Do not use live external publishing credentials.

## Updated Installer Artifact

Use this artifact from the coordination branch:

`test-comms/artifacts/ad1359b/The-Civic-Desk-0.2.8-ad1359b-windows-x64-cleanroom.zip`

Expected installer hashes inside the ZIP:

| File | SHA256 |
|---|---|
| `The Civic Desk_0.2.8_x64-setup.exe` | `98FF884929C25F0AC66227B0DAC5F5648C35ACF11B597D75D2A59341531CE241` |
| `The Civic Desk_0.2.8_x64_en-US.msi` | `A65B0D0B5587ECFFAAA9BC4EF263529CA2AFF886ABD82D77451FF3BBC5886C52` |

Reuse the existing fixture artifact unless you need to redownload:

`test-comms/artifacts/source-fixtures-20260628/CivicNewspaper-source-intake-fixtures-20260628.zip`

Expected fixture ZIP SHA256:

`99DC4DEABC8FE8FCAF3AB4C57C1C65DA390E25A1D44A1DDBA1B1DA6D1C34C43C`

## What Changed

Coder fixed the source-intake cleanup findings from `20260628-0448-tester-report-source-intake-fixture-gate.md`:

1. Bulk import URL normalization now trims unbalanced trailing punctuation such as `)` before review/import.
2. The Rust `add_source` storage chokepoint normalizes source URLs before validation/persist, covering manual add, discovery, and bulk import.
3. Failed file extraction clears stale bulk-import review candidates instead of leaving the previous file's candidates visible.
4. The Sources table URL display has a single titled/ellipsized link to avoid visual duplicated/truncated URL text.

Coder-side verification already run:

- `npm test -- --run src/bulkImportParser.test.ts src/components/SourcesPanel.test.tsx`: pass, 24 tests.
- `cargo test test_add_source_ --manifest-path src-tauri/Cargo.toml`: pass, 3 tests.
- `npm run build`: pass.
- `npm run test:ui-smoke`: pass.
- `cargo test --manifest-path src-tauri/Cargo.toml`: pass, 126 passed / 4 ignored.
- `CIVICNEWS_IMPORT_FIXTURE_DIR=C:\Users\instynct\Desktop\CivicNewspaperTestFiles cargo test local_source_import_fixtures_extract_reviewable_text --manifest-path src-tauri/Cargo.toml -- --ignored`: pass.

## Required Rerun Scope

Install or run the updated `ad1359b` artifact in a clean app profile if practical. If preserving the prior cleanroom VM/profile is more valuable for comparison, note that in the report.

Run a focused source-intake regression pass:

1. Edge-case XLSX
   - Load `colorado-source-list-edge-cases.xlsx`.
   - Confirm any `https://denver.legistar.com/Calendar.aspx)` candidate is reviewed/imported/stored as `https://denver.legistar.com/Calendar.aspx` with no trailing `)`.
   - Import at least the relevant selected edge candidate and inspect the Sources table and DB if possible.

2. Scanned-style PDF stale-review behavior
   - Load a successful source list first, such as `colorado-source-list-exported.pdf`.
   - Then load `colorado-source-list-scanned-style.pdf`.
   - Confirm the app shows the OCR/no-readable-text guidance and clears the prior review candidates, or otherwise makes it impossible to mistake prior candidates for the scanned PDF's candidates.

3. Sources table URL display
   - Inspect Sources after imports.
   - Confirm each source row shows one readable/ellipsized URL link, not duplicated/truncated text fragments.
   - Check narrow/mobile width if easy.

4. Regression spot check
   - Load/import one ordinary CSV or TXT fixture and confirm normal source import still works.

## Report Back

Write a report under:

`test-comms/reports/20260628-HHMM-tester-report-source-intake-rerun-ad1359b.md`

Use this summary format:

- Artifact tested and hash verification result.
- Environment/profile used.
- Result by the 4 rerun scopes above.
- Severity counts: Blocker / Critical / Major / Minor / Nit.
- Any screenshots or DB evidence paths, if collected.

Pass target: `0 Blocker / 0 Critical / 0 Major / 0 Minor / 0 Nit` for this focused rerun.
