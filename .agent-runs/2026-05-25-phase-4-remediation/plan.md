# Phase P - Implementation Plan

## 1. Approach
The strategy centers on addressing the 14 findings from the Phase 4 audit, combined with director decisions (D1-D5), using targeted fixes within the allowed scope. We will refactor `llm.rs` to introduce the `LlmClient` trait for testable dependency injection (D1). For the database layer, we will create two discrete migrations for schema updates (D3) and adjust `db.rs` and `daily_scan.rs` to correctly filter by `since_hours` (P4-003) and propagate a nullable `source_id` (P4-002, D5). We will add `DailyScanResults.tsx` to visualize the generated leads (P4-004) and update `Workbench.tsx` to handle rewrite errors safely with a confirmation modal (P4-007, P4-008, P4-013, D2). 

However, because several required files fall outside the manifest's `allowed_paths`, the Definition of Done is structurally infeasible without a replan. This plan bounds modifications strictly to the allowed paths and raises the missing file permissions as a REPLAN trigger.

## 2. Files to create
- `src-tauri/migrations/0006_daily_scan_lead_source_nullable.sql`: Migration to alter `daily_scan_leads.source_id` to be nullable (D3).
- `src-tauri/migrations/0007_source_tier_check.sql`: Migration to add `CHECK` constraint on `sources.tier` (D3).
- `src/components/DailyScanResults.tsx`: New component to display scan results (P4-004).
- `src/components/DailyScanResults.test.tsx`: Vitest suite asserting the new component renders correctly, including the aggregated badge for null source IDs (D5).
- `carried-debt.md`: Tracks deferred work, explicitly P5-001 for the plain-language diff modal (D2).

## 3. Files to modify
- `src-tauri/src/core/daily_scan.rs`: 
  - Update `parse_and_save_scan_response` to accept nullable `source_id` (P4-002, D5) and log insert errors (P4-009).
  - Apply `since_hours` to query `db::list_evidence_since` instead of arbitrary selection (P4-003).
  - Use `once_cell::sync::Lazy` for the city/state regex (P4-010).
  - Add orphan recovery and logging for nested errors during scan lifecycle (P4-014).
- `src-tauri/src/core/db.rs`: 
  - Change `DailyScanLead` `source_id` to `Option<i32>` and handle `NULL` in `list_daily_scan_leads` (D5).
  - Add `list_evidence_since(conn, hours)` helper (P4-003).
- `src-tauri/src/core/llm.rs`: 
  - Implement `pub trait LlmClient: Send + Sync` and `pub struct OllamaClient` wrapping local calls (D1).
- `src-tauri/src/core/prompts.rs`: 
  - No direct changes planned unless required to align with prompt resolution logic.
- `src-tauri/src/core/tests.rs`: 
  - Add the 6 missing tests (`test_source_tier_migration`, `test_source_tier_backfill_media_lead`, `test_list_prompts_returns_bundled`, `test_get_prompt_loads_aggregator`, `test_daily_scan_parses_fixture_response`, `test_plain_language_rewrite_invokes_ollama`).
  - Add prompt-schema-drift test that deserializes `aggregator.md` JSON into a `ScanResult` (D4).
- `src-tauri/src/core/migrations.rs`: 
  - Register `0006` and `0007` sequential migrations (D3).
- `src/components/Workbench.tsx`: 
  - Update plain-language rewrite button to use `await`, `try/catch`, loading state (P4-008), pass `draftFormat` (P4-013), and prompt `window.confirm` before updating draft (P4-007, D2).
- `src/ipc.ts`: 
  - Update `DailyScanLead` interface to `source_id?: number` (D5).
- `CHANGELOG.md`: 
  - Add Phase 4 feature list (P4-001).
- `SECURITY.md`: 
  - Add local-LLM-only documentation (P4-001).

*(Note: Every file listed above is within `allowed_paths`.)*

## 4. Test strategy
- **Migrations/Database**: `test_source_tier_migration` will apply `0006` and `0007` on an empty and populated DB. `test_source_tier_backfill_media_lead` checks legacy rows.
- **Prompts**: `test_list_prompts_returns_bundled` and `test_get_prompt_loads_aggregator` ensure resource paths resolve.
- **LLM/Daily Scan**: `test_daily_scan_parses_fixture_response` will supply a fixture LLM response and assert correctly filtered `since_hours` and extracted/null `source_id`.
- **Injection (D1)**: `test_plain_language_rewrite_invokes_ollama` will inject a `FakeLlmClient` to assert the target payload `(model, prompt, system)`.
- **Schema Drift (D4)**: Read `prompts/aggregator.md`, extract the JSON block, and assert successful deserialization into `ScanResult`.
- **Frontend**: `DailyScanResults.test.tsx` will assert rendering of the aggregated badge for rows with `source_id: undefined`.

## 5. Risks and Open Questions (REPLAN TRIGGER)
**BLOCKER**: The Definition of Done requires fixing issues in files that are not within `manifest.allowed_paths`, and one file explicitly forbidden:
1. **D1 (`LlmClient` Injection)**: Demands refactoring all call sites to use the injected client, but `src-tauri/src/tauri_cmds.rs` and `src-tauri/src/main.rs` are missing from `allowed_paths`, and `src-tauri/src/core/server.rs` is strictly forbidden. We cannot remove direct `call_local_ollama` references outside `llm.rs`.
2. **P4-004 & P4-005 (Scan UI / Location Hardcode)**: `src/useApp.ts` must be modified to read `CommunityProfile` and populate the results UI, but it is not in `allowed_paths`.
3. **P4-006 & P4-011 (Tier Validation / Command Registration)**: `src-tauri/src/tauri_cmds.rs` is required to add tier validation and expose `list_daily_scan_leads`.
4. **P4-012 (Walkthrough conflation)**: Requires modifying `walkthrough.md`, which is not in `allowed_paths`.

**STOP / REPLAN**: I am stopping execution and raising this as a REPLAN trigger. The operator must either expand `allowed_paths` to include these required files (and remove `server.rs` from `forbidden_paths`) or explicitly amend the Definition of Done.

## 6. Layered audit hooks
- **Per-commit careful-coding**: `cargo clippy --all-targets -- -D warnings` and `npx tsc --noEmit` locally ensure safe constructs and clean types.
- **Per-checkpoint sanity sweep**: `cargo test` and `npx vitest run` act as rapid regression nets after each localized change.
- **Per-rung audit-lite**: The pipeline will invoke `/audit-skills-antigravity:audit-lite` at the Critique phase, checking against the zero Blockers/Criticals requirement.

## 7. Definition of done
- All 14 findings remediated (Blocked pending REPLAN).
- `cargo test` reports >= 24 passing, including the 6 named tests and drift test.
- `npx vitest run` clean, including `DailyScanResults.test.tsx`.
- `cargo clippy` and `npx tsc` clean.
- `SECURITY.md` and `CHANGELOG.md` updated.
- `carried-debt.md` created with P5-001.
- Migrations `0006` and `0007` registered sequentially.
- `DailyScanLead.source_id` propagated as `Option<i32>`.
- `LlmClient` trait implemented without direct calls elsewhere (Blocked pending REPLAN).
- Audit-lite returns 0 Blockers, 0 Criticals.
- No files modified outside authorized scope.
