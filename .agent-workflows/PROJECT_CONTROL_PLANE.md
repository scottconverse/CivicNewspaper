# CivicNewspaper — Project Control Plane

Canonical source of "what work is active right now." The agent-pipeline-antigravity
plugin reads this file at preflight to enforce the priority-drift gate.

## Active target

**v0.2 Phase 4 — Source Tier + Prompt Library + Daily Scan + Plain Language Rewrite**

Authoritative spec: `_audits/dev-directives/v0.2-hardening.md` § Phase 4 (lines 311–439).
Canonical in-repo spec for active run: [`docs/spec/v0.2-phase-4.md`](../docs/spec/v0.2-phase-4.md).
The dev's `/agent-pipeline-antigravity:run start` invocation drafts its own `.agent-runs/<run-id>/manifest.yaml` + `scope-lock.yaml` from the description prose. Operator reviews the drafted manifest at the chat manifest gate and replies APPROVE or REVISE.

## v0.2 release plan

The v0.2 milestone is decomposed into 8 phases. Each phase is one pipeline run
on its own branch, against its own pre-approved directive in `.agent-runs/`.

Per a re-scoping decision on 2026-05-25, the remaining phases collapse into
three pipeline runs to balance autonomy against scope-lock granularity:

| Run | Phases | Status | Run ID |
|---|---|---|---|
| — | Phase 1 (Frontend rebuild) | merged | (pre-pipeline) |
| — | Phase 2 (Onboarding wizard) | merged | (pre-pipeline) |
| — | Phase 3 (Diagnostic export) | merged | (pre-pipeline) |
| **A** | **Phase 4** (Source Tier + Prompt Library + Daily Scan + Plain Language) | **active** | `2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite` |
| B | Phases 5 + 7 (Integrity Audit + Pending Stories + Provenance Sidebar) | not started | t.b.d. |
| C | Phases 6 + 8 (Sourcing Connectors + Validation/Release) | not started | t.b.d. |

Run B and Run C directives will be authored after Run A merges.

## Cross-run invariants

These constraints carry across every pipeline run in the v0.2 milestone:

1. **Backend Rust scope-lock.** The following files are operator-owned and may
   not be modified except in a run whose scope-lock explicitly unlocks them:
   `src-tauri/src/core/auth.rs`, `scraper.rs`, `detectors.rs`, `compiler.rs`,
   `guardrails.rs`, `llm.rs`, `backups.rs`, `server.rs`, `discovery.rs`,
   `diagnostics.rs`. Phase 4 unlocks `db.rs` only.

2. **CI gate immutability.** `scripts/verify-v0.2-phase-*-dod.sh`,
   `scripts/.dod-phase-*.sha256`, and `.github/workflows/ci.yml` are
   operator-owned. Runs that need to change a DoD must escalate, not silence.

3. **v0.1.1 security invariants.** `auth.rs`'s 127.0.0.1-only Host check and
   absent-Origin rejection. `compiler.rs`'s XSS test (`test_compiler_xss_safe`)
   with its `&lt;script` assertion. Strict CSP. These survive every run.

4. **Test-count monotonicity.** Each phase's DoD asserts `>= baseline`, not
   equality. Tests may be added by future phases; deletion below baseline fails.
   Anti-padding is enforced by named-test-presence checks, not count equality.

5. **Tauri IPC input validation.** Any new `#[tauri::command]` taking a
   renderer-supplied path must canonicalize and prefix-check against an allowed
   root (typically `app_data_dir()` or `download_dir()`). Any taking an
   identifier must validate against a known set or strict regex. The frontend
   dialog/picker is UI affordance, not a security boundary.

## Provenance

- Phase decomposition derived from `_audits/dev-directives/v0.2-hardening.md`,
  the canonical 8-phase spec (~70KB) committed by the operator on 2026-05-23.
- 3-run re-scoping decision documented in chat 2026-05-25 after recognizing
  the agent-pipeline-antigravity plugin's run-level autonomy was the right
  granularity (not per-phase shepherding).
