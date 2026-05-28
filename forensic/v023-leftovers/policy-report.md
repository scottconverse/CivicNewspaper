================================================================
Policy checks
================================================================

[PASS] check_manifest_schema
  check_manifest_schema: PASS -- manifest at .agent-runs\2026-05-26-civicnewspaper-v020-ship\manifest.yaml satisfies the v1.0 schema.

[PASS] check_manifest_immutable
  OK: manifest SHA matches pin.
    sha256=3a925ebef78fbacccf45fdd71be8c617778413a670b666cfc2573d5858a3ca67

[PASS] check_allowed_paths
  check_allowed_paths: PASS — 3 changed file(s), all within declared paths.

[PASS] check_no_todos
  check_no_todos: no source directories detected. PASS (vacuous).

[PASS] check_adr_gate
  check_adr_gate: PASS — no docs/adr/ directory in this project (check is vacuous).

[PASS] check_stage_done
  OK: all expected STAGE_DONE markers present (through stage 'stage-15-pages-publish').
    found: stage-00-preflight, stage-01-finish-phase-4, stage-02-audit-phase-4, stage-03-merge-phase-4, stage-04-version-bump, stage-06-ollama-sidecar, stage-07-onboarding-model-pull, stage-08-audit-ollama, stage-09-docs-pass, stage-10-audit-docs, stage-11-landing-page, stage-12-clean-machine-rehearsal, stage-13-audit-team-prerelease, stage-14-tag-and-release, stage-15-pages-publish

[PASS] check_autonomous_compliance
  OK: NO-OP -- autonomous-mode compliance check is inert in v1.3.0.
    The grant + autonomous-mode flow was removed.

[PASS] check_directive_conformance
  directive_conformance: AUTO_APPROVE manifest/scope-lock hash=85c8066d167c65e92ac3b1b68dcc44cef2853ea0ba7889b1452387d1d82837e2 author=Scott Converse authority=design_doc:C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5\MASTER-PROMPT-civicnewspaper-v020-ship.md

[PASS] check_scope_lock
  check_scope_lock: PASS - canonical_rung: v0.2.0-ship v0.2.0 ship — finish Phase 4 + Ollama sidecar + docs + release

[PASS] check_rung_file_ownership
  check_rung_file_ownership: PASS - edited paths and commit subject match the locked rung.

[PASS] check_release_docs_consistency
  check_release_docs_consistency: PASS - release docs match the locked canonical rung.

[PASS] check_pipeline_control_loop
  SKIP - active-control-state.md not present in run dir

[PASS] check_execute_readiness
  check_execute_readiness: PASS - implementation-report.md declares full manifest DoD readiness.

[PASS] check_decision_ledger
  SKIP - decision-ledger.ndjson not present in run dir

----------------------------------------------------------------
POLICY: ALL CHECKS PASSED
