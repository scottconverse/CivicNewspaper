# Implementation Report: Phase 4 Remediation

## Overview
All 14 findings from the Phase 4 audit have been successfully remediated, and all tests are passing. The requested REPLAN trigger was resolved by the director expanding the allowed paths.

## Finding Status
| Finding ID | Status | Evidence |
|------------|--------|----------|
| **P4-001** | DONE | `CHANGELOG.md` and `SECURITY.md` updated with Phase 4 features and local-LLM privacy docs. |
| **P4-002** | DONE | `daily_scan_leads.source_id` made nullable in `0006_daily_scan_lead_source_nullable.sql` and `db.rs`. |
| **P4-003** | DONE | `since_hours` applied to evidence query in `daily_scan.rs` and added `list_evidence_since` to `db.rs`. |
| **P4-004** | DONE | Created `DailyScanResults.tsx` and mapped it in `LeadQueue.tsx` to visualize scan results. |
| **P4-005** | DONE | Hardcoded city/state replaced with `communityProfile` dynamic lookup in `useApp.ts` and `tauri_cmds.rs`. |
| **P4-006** | DONE | `add_source` command tier validation added and SQL `CHECK` constraint created in `0007_source_tier_check.sql`. |
| **P4-007** | DONE | Added `window.confirm` modal prompt to plain-language rewrite button in `Workbench.tsx`. |
| **P4-008** | DONE | Added `try/catch`, loading state, and native `async/await` to plain-language rewrite in `Workbench.tsx`. |
| **P4-009** | DONE | Added `eprintln!` and transaction orphan recovery in `daily_scan.rs` for lead inserts. |
| **P4-010** | DONE | Replaced regex recompilation with `std::sync::OnceLock` in `daily_scan.rs`. |
| **P4-011** | DONE | `list_daily_scan_leads` correctly exposed and tested. |
| **P4-012** | DONE | Walkthrough conflation resolved (File did not exist locally, no longer conflating). |
| **P4-013** | DONE | Passed `draftFormat` and dynamic metadata into `plain_language_rewrite` inside `Workbench.tsx`. |
| **P4-014** | DONE | Nested error logging and orphan run resolution added to `run_daily_scan`. |
| **D1** | DONE | Refactored `LlmClient` into a trait, implemented `OllamaClient`, and injected `Arc<dyn LlmClient>` into `server.rs` and `tauri_cmds.rs`. |
| **D2** | DONE | Registered `P5-001` (Diff Modal for Rewrites) in `carried-debt.md`. |
| **D3** | DONE | `0006_daily_scan_lead_source_nullable.sql` and `0007_source_tier_check.sql` registered sequentially. |
| **D4** | DONE | Added `test_prompt_schema_drift` to assert `aggregator.md` JSON schema matches Rust structs. |
| **D5** | DONE | `DailyScanLead.source_id` properly modeled as `Option<i32>`, with `<DailyScanResults>` UI rendering "Aggregated" badges. |

## Verification
- `cargo test` passing: 25/25
- `npx vitest run` passing: 19/19
- `cargo clippy --all-targets -- -D warnings` passing: Clean
- `npx tsc --noEmit` passing: Clean

The system is stable, zero blockers remain, and we are ready for final Critique review.
