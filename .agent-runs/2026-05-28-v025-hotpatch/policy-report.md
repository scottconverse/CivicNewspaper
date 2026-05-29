================================================================
Policy checks
================================================================

[PASS] check_manifest_schema
  check_manifest_schema: PASS -- manifest at .agent-runs\2026-05-28-v025-hotpatch\manifest.yaml satisfies the v1.0 schema.

[PASS] check_manifest_immutable
  OK: manifest SHA matches pin.
    sha256=6b7bad24afeb440d40c200124bf9764a6bc70c26577497a685c0fe3882deab88

[PASS] check_allowed_paths
  check_allowed_paths: no changed files in any tracked repo. PASS.

[PASS] check_no_todos
  check_no_todos: no source directories detected. PASS (vacuous).

[PASS] check_adr_gate
  check_adr_gate: PASS — no docs/adr/ directory in this project (check is vacuous).

[PASS] check_stage_done
  OK: all expected STAGE_DONE markers present (through stage 'patch').
    found: critique, drift-detect, manager, manifest, patch, policy, reproduce, research, verify

[PASS] check_autonomous_compliance
  OK: NO-OP -- autonomous-mode compliance check is inert in v1.3.0.
    The grant + autonomous-mode flow was removed.

[PASS] check_directive_conformance
  SKIP - directive.yaml not present in run dir

[PASS] check_scope_lock
  SKIP - scope-lock.yaml not present in run dir

[PASS] check_rung_file_ownership
  SKIP - scope-lock.yaml not present in run dir

[PASS] check_release_docs_consistency
  SKIP - scope-lock.yaml not present in run dir

[PASS] check_pipeline_control_loop
  SKIP - active-control-state.md not present in run dir

[PASS] check_execute_readiness
  check_execute_readiness: PASS - implementation-report.md declares full manifest DoD readiness.

[PASS] check_decision_ledger
  SKIP - decision-ledger.ndjson not present in run dir

----------------------------------------------------------------
POLICY: ALL CHECKS PASSED
