# Implementation Plan — 2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite

## 1. Approach
We will implement the Phase 4 specification exactly as authored in the directive. The database schema will be extended with two migrations (`0004_source_tier.sql` and `0005_daily_scans.sql`) to introduce the `tier` taxonomy on sources and the tracking for daily scans. The Tauri backend will expose new commands (`run_daily_scan`, `plain_language_rewrite`, `list_prompts`, `get_prompt`) that wrap local Ollama calls and SQLite CRUD operations. The React frontend will be updated to display tier badges and add the new workbench actions. We will enforce strict security validations (DR-1 and DR-2) in the backend handlers.

## 2. Files to create
- `src-tauri/migrations/0004_source_tier.sql`: Alters `sources` table to add `tier` column and backfills `media_lead`.
- `src-tauri/migrations/0005_daily_scans.sql`: Creates `daily_scan_runs` and `daily_scan_leads` tables.
- `src-tauri/src/core/prompts.rs`: Handles loading bundled prompts from `src-tauri/prompts/` and defines Tauri commands.
- `src-tauri/src/core/daily_scan.rs`: Implements the `run_daily_scan` logic (fetching evidence, calling Ollama, parsing results, saving to DB).
- `src/components/DailyScanResults.tsx`: Displays the extracted leads in rank order with tier badges.
- `src/components/DailyScanResults.test.tsx`: Component tests with fixture leads.

## 3. Files to modify
- `src-tauri/src/core/db.rs`: Add `tier` to `Source`. Update `list_sources`, `insert_source`. Add structs/CRUD for Daily Scans.
- `src-tauri/src/core/tests.rs`: Add the 6 required integration tests (source tier, prompt loader, scan parser, plain language rewrite).
- `src-tauri/src/core/mod.rs`: Export `prompts` and `daily_scan`.
- `src-tauri/src/main.rs`: Register new Tauri commands.
- `src-tauri/src/tauri_cmds.rs`: Implement `plain_language_rewrite` and expose new commands.
- `src/components/SourcesPanel.tsx`: Add tier selector for sources.
- `src/components/SourcesPanel.test.tsx`: Update to support `tier`.
- `src/components/Workbench.tsx`: Add "Prompt Library" dropdown and "Rewrite for Newsletter" button.
- `src/components/Workbench.test.tsx`: Update with new buttons.
- `src/components/SystemStatus.tsx`: Add "Run Daily Scan" button.
- `src/components/SystemStatus.test.tsx`: Update with new button.
- `CHANGELOG.md`: Add Phase 4 feature notes.

## 4. Test strategy
- `test_source_tier_migration`: DB integration test asserting schema and constraints.
- `test_source_tier_backfill_media_lead`: DB integration test asserting backfill logic.
- `test_list_prompts_returns_bundled`: Integration test asserting all 14 prompts across 5 categories load.
- `test_get_prompt_loads_aggregator`: Integration test asserting the aggregator prompt contains the unfakeable anchor string.
- `test_daily_scan_parses_fixture_response`: Unit test mocking Ollama and validating the parser logic for structured leads.
- `test_plain_language_rewrite_invokes_ollama`: Unit test mocking Ollama for the rewrite function.
- UI tests for `DailyScanResults`, `Workbench`, `SystemStatus`, and `SourcesPanel` using Vitest to assert rendering and click handlers.

## 5. Risks
- **Missing Prompts (CRITICAL)**: The `src-tauri/prompts/` directory is missing. Mitigation: Block execution until the operator provides the bundled prompts directory from `civic-newsroom` or `civic-scanner`.
- **Path Traversal via get_prompt**: Mitigation: Implement strict allowlist validation (DR-1) against `list_prompts()` results.
- **Unbounded Evidence Pulls**: Mitigation: Enforce `0 < since_hours <= 168` in `run_daily_scan` (DR-2) before querying DB.
- **Ollama Timeout/OOM**: Mitigation: Truncate evidence context to ~32KB to fit within 9B model context windows safely.

## 6. Layered audit hooks
- Per-commit careful-coding: Code changes will be isolated to exactly the allowed_paths.
- Per-checkpoint sanity sweep: `cargo test` and Vitest runs locally.
- Per-rung audit-lite: The final PR will be verified by the dual-AI audit pipeline before merge.

## 7. Definition of done
cargo test includes the 6 new tests for source-tier, prompt loader, scan parser, and rewrite, all passing. Total tests >= 24. CI green on all DoDs. Pre-merge audit-lite returns zero Blockers and zero Criticals.
