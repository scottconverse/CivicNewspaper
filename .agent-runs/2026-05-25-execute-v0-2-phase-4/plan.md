# Implementation Plan for v0.2 Phase 4

## 1. Approach
The approach follows an additive, strict-boundary pattern. We will introduce new database tables and UI components without disrupting existing security constraints or locked modules.
- **Database & Schema**: Introduce `tier` on sources and daily scan tracking tables via two sequential SQL migrations. The Rust `db.rs` models will be expanded, and `migrations.rs` will map these sequentially to ensure schema updates happen on application launch.
- **Tauri Backend Capabilities**: Expose two new domain modules: `prompts.rs` and `daily_scan.rs`. `prompts.rs` will read bundled markdown prompts from the Tauri resources folder, enforcing an explicit whitelist (`VALID_PROMPT_IDS` enum array) per the binding director decisions. `daily_scan.rs` will orchestrate scan logic locally, utilizing the existing `pub async fn call_local_ollama` in `llm.rs` without modifying `llm.rs` itself. `tauri_cmds.rs` will wrap these domain calls as Tauri commands and enforce stringent input validation (regex checks for city/state and bounds on `since_hours`).
- **Frontend Extensions**: Introduce `DailyScanResults.tsx` for reviewing scan output. We will update `SourcesPanel` and `LeadQueue` to display and manipulate the new source `tier` data, and enhance `Workbench` to accommodate Prompt Library selections and a "Rewrite for Newsletter" feature.
This strategy satisfies the manifest's Definition of Done and strictly adheres to the scope-locks and director guidelines (such as pushing with `-u origin` and leaving the aggregator prompt completely unmodified).

## 2. Files to create
- `src-tauri/migrations/0004_source_tier.sql`: SQL schema to add `tier` column to `sources` table and backfill existing rows.
- `src-tauri/migrations/0005_daily_scans.sql`: SQL schema to create `daily_scan_runs` and `daily_scan_leads` tables.
- `src-tauri/src/core/prompts.rs`: Exposes `list_prompts` and `get_prompt` capabilities using `resolve_resource` and an explicit `VALID_PROMPT_IDS` array to strictly validate bundled prompt access.
- `src-tauri/src/core/daily_scan.rs`: Contains the `run_daily_scan` business logic, bridging the DB models with `call_local_ollama` invocations.
- `src/components/DailyScanResults.tsx`: React component to render the results of daily scans.
- `src/components/DailyScanResults.test.tsx`: Vitest suite verifying the daily scan UI interactions.

## 3. Files to modify
All of the following paths fall strictly within `manifest.allowed_paths`:
- `src-tauri/src/core/db.rs`: Update the `Source` struct to include a `tier` field; add CRUD methods for new scan tables.
- `src-tauri/src/core/migrations.rs`: Include `0004_source_tier` and `0005_daily_scans` strictly in numeric order within the `MIGRATIONS` array, loaded via `include_str!`.
- `src-tauri/src/core/detectors.rs`: Update `Source` instantiation with `tier: source.tier.clone()` strictly for field propagation. (No widening of logic scope per director decisions).
- `src-tauri/src/core/mod.rs`: Add `pub mod prompts;` and `pub mod daily_scan;`.
- `src-tauri/src/core/tests.rs`: Append the 6 mandated tests asserting migration success and feature logic constraints.
- `src-tauri/src/lib.rs`: Register `list_prompts`, `get_prompt`, `run_daily_scan`, `plain_language_rewrite` in `tauri::generate_handler!`.
- `src-tauri/src/tauri_cmds.rs`: Define or expose Tauri handlers. Include `VALID_PROMPT_IDS` grep-able validation in `get_prompt` handler. Add regex bounds checking (`^[A-Za-z][A-Za-z .'-]{0,63}$`) for `city` and `state`, and numeric bounds `0 < since_hours <= 168` for `run_daily_scan`. Expose `plain_language_rewrite` that directly calls `crate::core::llm::call_local_ollama`.
- `src-tauri/tauri.conf.json`: Add `"prompts/**/*"` to the `bundle.resources` array.
- `src/components/SourcesPanel.tsx`: Incorporate a tier selector into the sources management interface.
- `src/components/LeadQueue.tsx`: Add tier badge rendering logic for sources.
- `src/components/Workbench.tsx`: Add Prompt Library fetching logic and integrate the "Rewrite for Newsletter" command.
- `src/App.tsx` & `src/useApp.ts`: Connect `DailyScanResults` routing and central application state.
- `CHANGELOG.md`: Record new capabilities (Source Tier, Prompt Library, Daily Scan, Plain Language Rewrite).
- `SECURITY.md`: Introduce a section explicitly confirming Daily Scan is a local-LLM-only process and bundled prompts have no network access.
- *(Optional adjustments as required within `allowed_paths`: `src/components/PublishPanel.tsx`, `src/components/SystemStatus.tsx`, `package.json`, etc.)*

## 4. Test strategy
- **Backend Tests (Rust)** (`src-tauri/src/core/tests.rs`):
  1. `test_source_tier_migration`: Creates a DB without `0004`, adds a source, runs the migration, and asserts `tier` column exists and receives a valid default tier.
  2. `test_source_tier_backfill_media_lead`: Verifies that a legacy media lead maps correctly to a valid tier.
  3. `test_list_prompts_returns_bundled`: Asserts `list_prompts` returns `["aggregator"]`.
  4. `test_get_prompt_loads_aggregator`: Asserts `get_prompt("aggregator")` succeeds and strictly contains `CIVIC NEWSROOM - DAILY CIVIC NEWS AGGREGATOR`.
  5. `test_daily_scan_parses_fixture_response`: Mocks an LLM response passed to `run_daily_scan` and asserts it accurately saves a `daily_scan_leads` entry in the DB.
  6. `test_plain_language_rewrite_invokes_ollama`: Asserts the Tauri command links to `llm.rs` logic via dependency injection/fixture.
- **Frontend Tests (Vitest)** (`src/components/DailyScanResults.test.tsx`):
  - Mounts `DailyScanResults` with 3 fixture leads.
  - Verifies tier badges are rendered accurately.
  - Clicks "Open in Workbench" and asserts the appropriate mock handler is invoked with the specific lead ID.
  - *(Increases Vitest total count by at least 2).*

## 5. Risks
1. **Unregistered Migrations Crashing Launch**: If `0005_daily_scans` is missing from `MIGRATIONS`, Rust compilation will succeed, but runtime tests will fail on missing tables. *Mitigation:* Explicit sequential registration inside `MIGRATIONS` array and runtime evaluation inside `test_source_tier_migration`.
2. **Path Traversal on Prompt Loading**: `get_prompt` could be exploited to read sensitive filesystem files. *Mitigation:* We use a rigid `VALID_PROMPT_IDS` whitelist validation (Option B from director decisions) inside the command body prior to resolving paths.
3. **Regex ReDoS in Parameter Validation**: A bad regex inside `run_daily_scan` could consume massive CPU. *Mitigation:* Combine a strictly bounded regex string `^[A-Za-z][A-Za-z .'-]{0,63}$` with basic length constraints before regex evaluation.
4. **Scope Lock Violations via Convenience Tweaks**: Implementing scan heuristics in `detectors.rs` or adding wrappers inside `llm.rs` will fail the policy gate. *Mitigation:* Adhering explicitly to strict field propagation in `detectors.rs` (`tier: source.tier.clone()`) and directly invoking `crate::core::llm::call_local_ollama`.

## 6. Layered audit hooks
- **Per-commit careful-coding**: The AI agent will skip any changes inside scope-locked areas, isolating code footprint exclusively inside unlocked target files.
- **Per-checkpoint sanity sweep**: Standard `cargo test` and `vitest run` are invoked continuously. The executor will also strictly enforce pushing with `git push -u origin v0.2-phase-4` to prevent pipeline stalls.
- **Per-rung audit-lite**: The executor will run a dummy command `/audit-skills-antigravity:audit-lite --help` directly on the local machine prior to conducting the formal audit-lite sweep to confirm tool availability.
- The `audit-lite` sweep guarantees zero Blocker/Critical findings based on the modified `SECURITY.md` statements.

## 7. Definition of done
- The `cargo test` suite includes the 6 specific new tests named in the manifest, all passing.
- `cargo test --all` total test count >= 24.
- CI is entirely green across all DoDs.
- Pre-merge audit-lite returns 0 Blockers and 0 Criticals.
- Manual smoke tests pass for Daily Scan and Rewrite for Newsletter.
- `src-tauri/src/core/migrations.rs` explicitly registers both `0004_source_tier` and `0005_daily_scans` in sequential numeric order.
- `SECURITY.md` explicitly documents the Daily Scan as local-LLM-only and bundled prompts as network-free.
- No files outside `allowed_paths` are modified; scope-locked files show a literal 0 content diff against `origin/main`.
- Vitest test count grows by at least 2 cases, fully validating the Daily Scan results view and tier badges.
