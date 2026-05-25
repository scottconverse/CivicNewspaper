================================================================
Preflight checks (v1.2.0+)
================================================================

[PASS] check_manifest_schema
  check_manifest_schema: PASS -- manifest at .agent-runs\2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite\manifest.yaml satisfies the v1.0 schema.

[PASS] check_active_target
  OK: ALIGNED — manifest.advances_target='v0.2 Phase 4 — Source Tier + Prompt Library + Daily Scan + Plain Language Rewrite' matches control plane active target='v0.2 Phase 4 — Source Tier + Prompt Library + Daily Scan + Plain Language Rewrite' at C:\Users\scott\Documents\antigravity\eager-archimedes\.agent-workflows\PROJECT_CONTROL_PLANE.md

[PASS] check_manifest_paths
  OK: all manifest paths and citations resolve.

[PASS] check_autonomous_mode
  OK: HUMAN-MODE -- autonomous mode and grant system removed in v1.3.0.
    The v1.3.0 run skill uses modal AskUserQuestion gates and
    evidence-driven auto-promote. No grant needed.

[PASS] check_manifest_immutable_pin
  OK: manifest SHA-256 pinned to C:\Users\scott\Documents\antigravity\eager-archimedes\.agent-runs\2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite\manifest.sha
    sha256=0571e29939a9e4884470a76f8cd5973e52d39dac8a79cc2b6baa5d6b476c5031

----------------------------------------------------------------
PREFLIGHT: ALL CHECKS PASSED
  manifest schema OK; advances_target aligned with control plane; paths resolve; SHA pinned for cross-stage integrity check.
