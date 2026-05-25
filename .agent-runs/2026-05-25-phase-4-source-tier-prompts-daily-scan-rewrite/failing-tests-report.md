# Failing Tests Report

## Test Files Added
- `src-tauri/src/core/tests.rs` (Appended Phase 4 tests)
- `src/components/DailyScanResults.test.tsx` (New file)

## Contracts Asserted
- `test_source_tier_migration`: Asserts migration applies default `official_record` tier.
- `test_source_tier_backfill_media_lead`: Asserts a `media_lead` is correctly mapped to `news_reporting`.
- `test_list_prompts_returns_bundled`: Asserts `list_prompts()` returns at least 14 entries.
- `test_get_prompt_loads_aggregator`: Asserts the aggregator prompt is correctly read from resource.
- `test_daily_scan_parses_fixture_response`: Asserts `parse_daily_scan_leads` correctly extracts 1 lead from a fixture.
- `test_plain_language_rewrite_invokes_ollama`: Asserts rewrite logic function returns mocked value.
- `DailyScanResults renders 3 fixture leads with tier badges`: Asserts UI render of leads and badges.
- `DailyScanResults Open in Workbench fires right action`: Asserts click handler passes correct ID.

## Test Runner Output & Failure Reasons

### Frontend Tests (`vitest`)
```
 FAIL  src/components/DailyScanResults.test.tsx [ src/components/DailyScanResults.test.tsx ]
Error: Failed to resolve import "./DailyScanResults" from "src/components/DailyScanResults.test.tsx". Does the file exist?
```
**Reason for Failure:** The implementation file `DailyScanResults.tsx` does not exist yet. This is correct.

### Backend Tests (`cargo check --tests`)
```
error[E0432]: unresolved import `crate::core::prompts`
error[E0609]: no field `tier` on type `Source`
error[E0432]: unresolved import `crate::core::daily_scan`
error[E0425]: cannot find function `plain_language_rewrite_logic` in module `crate::tauri_cmds`
```
*(Exact rustc error lines omitted for brevity)*
**Reason for Failure:** The backend implementation is missing. The modules `prompts`, `daily_scan` do not exist, `tier` is missing on `Source`, and `plain_language_rewrite_logic` is missing. This is correct.
