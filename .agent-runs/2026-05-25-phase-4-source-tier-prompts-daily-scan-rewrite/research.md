# Research Report — 2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite

## 1. Affected modules
- `src-tauri/src/core/db.rs`: Defines data structs like `Source` and `EvidenceItem`. CRUD logic will need `tier` for sources.
- `src-tauri/src/tauri_cmds.rs`: Front-line Tauri command handlers. We will add `run_daily_scan`, `plain_language_rewrite`, `list_prompts`, and `get_prompt` here.
- `src-tauri/src/core/prompts.rs` (new): Needs to be created to read from `src-tauri/prompts/` via `tauri::api::path::resolve_resource`.
- `src-tauri/src/core/daily_scan.rs` (new): Needs to be created to pull evidence, hydrate prompt, call Ollama, and save `daily_scan_runs`/`daily_scan_leads`.
- Components (`SourcesPanel`, `Workbench`, `SystemStatus`, `DailyScanResults`): React components that will wire up the new logic and render tier badges and scan results.

## 2. Existing patterns
- DB connection passing: `db: tauri::State<'_, DbConn>` followed by `let conn = db.lock().unwrap()`.
- Error handling: Tauri commands map internal errors to `Result<T, String>` using `.map_err(|e| e.to_string())`.
- LLM calls: `crate::core::llm::call_local_ollama(&model, &prompt, &system)` used in `generate_draft`.

## 3. Constraints from Antigravity.md
None specified. (Non-negotiables section is empty).

## 4. Constraints from ADRs
No `docs/adr/` directory exists.

## 5. Open questions

- **Prompt Library Sourcing**: The `src-tauri/prompts/` directory does not exist in this repo. The Phase 4 specification states I must either identify a sibling repo or escalate. *Recommendation: Escalate to operator to provide the bundled prompts (e.g., from `civic-newsroom` or `civic-scanner`) before writing code.*
- **get_prompt path validation**: `get_prompt` must validate the id against `list_prompts()` to prevent path-traversal. *Recommendation: implement a strict check that ensures `id` matches an enumerated entry exactly, returning an Error otherwise.*
- **since_hours bounds**: *Recommendation: `run_daily_scan` will explicitly enforce `if since_hours == 0 || since_hours > 168 { return Err(...) }` before proceeding.*
