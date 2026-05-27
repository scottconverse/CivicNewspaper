# Audit Lite — Tests and Runtime (Group WT)
**Date:** 2026-05-27
**Scope:** Scoped review of HTTP status success check on model pull, cancellation keying, warning comments for ignored tests, vitest end-to-end model checks, and grep-evasion script.
**Reviewer:** Claude (audit-lite)

## TL;DR
Verified HTTP status success checks, per-pull cancellation channels, and vitest test cases. Added MUTATION-RESISTANT warnings above ignore tests and created the grep-checks.sh quote-evasion script. All tests pass successfully. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **WT-1 Status Check**: Added status checking `resp.status().is_success()` to `pull_ollama_model` to handle HTTP errors properly, verified by `test_pull_ollama_model_propagates_http_error`.
- **WT-2 Per-pull Cancellation**: Replaced global atomic cancel with `CANCEL_PULL_MAP` watch channels keyed by model ID, verifying cancel works on a per-model basis in `test_cancel_ollama_pull_is_per_pull`.
- **WT-4 Vitest settings check**: Renamed frontend vitest check to `test_useapp_daily_scan_end_to_end_model` verifying the persistence checks for settings models.
- **WT-5 Ignore warnings**: Added the `MUTATION-RESISTANT` warning comments above the five ignored tests in `src-tauri/src/core/tests.rs`.
- **WT-6 Grep evasion guard**: Created `scripts/audit/grep-checks.sh` containing the quote-evasion regex checks, verifying that no unquoted/single-quoted literals slip through the checks.

## Escalation recommendation
No escalation needed.
