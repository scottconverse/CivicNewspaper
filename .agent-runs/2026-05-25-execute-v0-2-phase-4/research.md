# Research Report for 2026-05-25-execute-v0-2-phase-4

## 1. Affected modules
- `src-tauri/migrations/0004_source_tier.sql` (New): Will add the `tier` column to the `sources` table and backfill existing rows. Exposes SQL schema modifications.
- `src-tauri/migrations/0005_daily_scans.sql` (New): Will add `daily_scan_runs` and `daily_scan_leads` tables, exposing schema for the new Daily Scan feature.
- `src-tauri/src/core/db.rs`: Currently exposes DB CRUD operations with structs like `Source` and `Lead`. Will be updated to include the `tier` field for `Source`, `from_scan_lead_id` for `Lead`, and new scan tables.
- `src-tauri/src/core/migrations.rs`: Currently holds the `MIGRATIONS` array up to `0003_settings`. Will expose `0004_source_tier` and `0005_daily_scans` for automatic schema application.
- `src-tauri/src/core/detectors.rs`: Exposes `run_detectors` and parses `ProfileConfig`. Unlocked only to handle the `tier` field cascade from the updated `Source` struct without widening its logic scope.
- `src-tauri/src/core/prompts.rs` (New): Will expose bundled prompt access (`list_prompts`, `get_prompt`) reading from the Tauri resources dir.
- `src-tauri/src/core/daily_scan.rs` (New): Will expose `run_daily_scan` logic to interface with `llm.rs` and the DB.
- `src-tauri/src/core/mod.rs`: Exports core modules; will need to expose `prompts` and `daily_scan`.
- `src-tauri/src/core/tests.rs`: Contains `cargo test` suite; will expose the 6 new tests explicitly requested by the Definition of Done.
- `src-tauri/src/lib.rs`: The Tauri application entrypoint exposing the `tauri::generate_handler!` macro. Will register `list_prompts`, `get_prompt`, `run_daily_scan`, `plain_language_rewrite`.
- `src-tauri/src/tauri_cmds.rs`: Exposes the Tauri frontend-to-backend bridge functions handling payload validation and DB locks.
- `src-tauri/tauri.conf.json`: Configures Tauri bundling. Exposes `bundle.resources` which will add `prompts/**/*`.
- `src/components/SourcesPanel.tsx`: Renders the Source management table and add-source form. Will integrate a tier selector.
- `src/components/LeadQueue.tsx`: Renders leads and drafts. Will render tier badges.
- `src/components/PublishPanel.tsx`: Exposes static site compilation triggers. May receive minor updates for tier logic compatibility.
- `src/components/Workbench.tsx`: Handles draft editing, LLM interactions. Exposes the main writing interface and will add Prompt Library and Rewrite for Newsletter features.
- `src/components/SystemStatus.tsx`: Displays system/app version and Ollama status.
- `src/components/DailyScanResults.tsx` (New) & `DailyScanResults.test.tsx` (New): Will expose the UI for viewing Daily Scan leads and tests for fixture interactions.
- `CHANGELOG.md`: Project's version history log.
- `SECURITY.md`: Defines security parameters. Will be updated to explicitly state that Daily Scan is local-LLM-only with no auto-upload and bundled prompts have no network access.
- `src-tauri/Cargo.toml` & `src-tauri/Cargo.lock`: Defines Rust dependencies.
- `src/App.tsx` & `src/useApp.ts`: Wires global React application state and routing.
- `package.json` & `package-lock.json`: Defines frontend JS dependencies.
- Frontend test files (`src/components/Workbench.test.tsx`, `SystemStatus.test.tsx`, `SourcesPanel.test.tsx`): Exposes current Vitest suites.

## 2. Existing patterns
1. **DB Struct CRUD Operations (`src-tauri/src/core/db.rs`)**: 
   Uses standard `rusqlite` parameters in query strings with struct mapping. For example, `pub fn list_sources(conn: &Connection) -> SqlResult<Vec<Source>>`. This pattern should be used for `daily_scan_leads` interactions.
2. **Tauri Command Injection (`src-tauri/src/tauri_cmds.rs`)**:
   Commands heavily utilize `db: tauri::State<'_, DbConn>` and unwrap it locally via `.lock().map_err(|_| "Failed to lock database".to_string())?`. This error stringifying pattern must be used for new commands (`run_daily_scan`, `get_prompt`).
3. **Migration Array Definitions (`src-tauri/src/core/migrations.rs`)**:
   Migrations are explicitly ordered in the `MIGRATIONS: &[(&str, &str)]` array utilizing the `include_str!` macro to load SQL files. The `run_migrations` function loops and checks against the `PRAGMA user_version`. We must cleanly insert `0004` and `0005` here.
4. **LLM Invocation via Reqwest (`src-tauri/src/core/llm.rs`)**:
   `call_local_ollama(model: &str, prompt: &str, system: &str) -> Result<String, Box<dyn Error + Send + Sync>>` handles timeouts and HTTP connections for generation. The new `run_daily_scan` must lean on this existing `pub` function natively.

## 3. Constraints from Antigravity.md
> "During active agent-pipeline-antigravity runs, the pipeline's chat gate keywords (`APPROVE` / `REVISE` / `REPLAN` / `BLOCK` / `VIEW`) and hook policy are authoritative. Operator-layer memory rules about asking-before-deciding (e.g. `feedback_no_unilateral_product_decisions.md`) apply OUTSIDE pipeline runs only. Inside a run, the v2.2.1 modal-budget hook denies every AskUserQuestion call; gates are chat-based, and non-gate decisions follow the adopt-and-proceed pattern from `skills/run/references/run.md`."

> "Branch from main, work in slices. Tag at rung close. Run tests and verify before merging."

## 4. Constraints from ADRs
The project does not have a `docs/adr/` directory. Therefore, no ADR constraints apply to this work.

## 5. Open questions

### Q1: Handling `src-tauri/prompts/` presence and string requirement
The directory `src-tauri/prompts/aggregator` already exists and contains `01-daily-scan.md`. The string `CIVIC NEWSROOM - DAILY CIVIC NEWS AGGREGATOR` is confirmed present on line 9 of the file.
- **Trade-off Matrix**:
  - *Option A (Load as-is)*: Directly expose this file via `get_prompt`. High safety, low implementation overhead.
  - *Option B (Parse/Trim)*: Strip headers from the markdown file before returning to frontend. Might break strict string matching requirements in testing.
- **Recommendation**: Proceed with Option A. Defer to the director if trimming is strictly required.

### Q2: Widen scope in `detectors.rs` vs modifying strictly `Source` cascade
- **Trade-off Matrix**:
  - *Option A (Strict Field Propagation)*: Modify `Source` instantiation in `detectors.rs` with `tier: source.tier.clone()` but keep logic otherwise untouched. Zero risk of breaking current heuristics.
  - *Option B (Utilize Tier in Detection)*: Add specific detector logic (e.g., skip parsing for low tiers). Violates scope lock.
- **Recommendation**: Strictly use Option A to comply with scope locks.

### Q3: Validation pattern in `get_prompt` against `list_prompts`
- **Trade-off Matrix**:
  - *Option A (Runtime dynamic check)*: Fetch the directory listing inside `get_prompt` to validate the `id` exists. Adds disk IO penalty per call but perfectly reflects filesystem.
  - *Option B (Hardcoded enum list)*: Define `VALID_PROMPT_IDS = ["aggregator"]` explicitly as instructed by the manifest (`list_prompts | VALID_PROMPT_IDS`). Best compatibility with the DoD constraints.
- **Recommendation**: Use Option B (Hardcoded Enum/Array) to satisfy the string grep DoD requirement cleanly.

### Q4: Verification of `audit-skills-antigravity` plugin
- **Trade-off Matrix**:
  - *Option A (Check during execution)*: The executor handles checking the skill presence. If not found, the run blocks. Ensures high compliance.
  - *Option B (Ignore)*: Fails the director's specific directive.
- **Recommendation**: Implement Option A. The executor MUST invoke `/audit-skills-antigravity:audit-lite --help` via a command task and evaluate the exit code. Final decision deferred to director.

### Q5: `call_local_ollama` accessibility
Confirmed: `pub async fn call_local_ollama` is actively available in `src-tauri/src/core/llm.rs` without modifying `llm.rs`.
- **Trade-off Matrix**:
  - *Option A (Direct Integration)*: Invoke `crate::core::llm::call_local_ollama` directly from `tauri_cmds.rs` for the new `plain_language_rewrite` and `daily_scan.rs`. Complies natively.
  - *Option B (Refactor `llm.rs` wrapper)*: Add wrapper logic. Violates scope-lock on `llm.rs`.
- **Recommendation**: Use Option A.

### Q6: First push branch tracking (`--force-with-lease` vs `-u`)
- **Trade-off Matrix**:
  - *Option A (Force push)*: Could result in git errors since upstream doesn't track `v0.2-phase-4`.
  - *Option B (`git push -u origin v0.2-phase-4`)*: Explicitly requested in the manifest to avoid the failure mode encountered in Phase 3.
- **Recommendation**: Executor must explicitly run `git push -u origin v0.2-phase-4`.

### Q7: `MIGRATIONS` numeric order registration
- **Trade-off Matrix**:
  - *Option A (Add to array sequentially)*: Register `0004_source_tier` and `0005_daily_scans` immediately after `0003_settings`. Required for tests to pass.
  - *Option B (Out of order)*: Fails compilation and DoD checks.
- **Recommendation**: Implement Option A strictly.
