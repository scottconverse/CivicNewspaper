================================================================
Preflight checks (v1.2.0+)
================================================================

[PASS] check_manifest_schema
  check_manifest_schema: PASS -- manifest at .agent-runs\2026-05-28-v025-hotpatch\manifest.yaml satisfies the v1.0 schema.

[PASS] check_active_target
  WARN: OVERRIDE_ACCEPTED � advances_target='v0.2.5-hotpatch' bypasses the gate. Logged to .agent-workflows/scope-overrides.md. At the manifest gate, type OVERRIDE-CONFIRMED to proceed.

[PASS] check_manifest_paths
  OK: all manifest paths and citations resolve.

[PASS] check_autonomous_mode
  OK: HUMAN-MODE -- autonomous mode and grant system removed in v1.3.0.
    The v1.3.0 run skill uses modal AskUserQuestion gates and
    evidence-driven auto-promote. No grant needed.

[PASS] check_manifest_immutable_pin
  OK: manifest SHA-256 pinned to C:\Users\scott\Documents\antigravity\eager-archimedes\.agent-runs\2026-05-28-v025-hotpatch\manifest.sha
    sha256=6b7bad24afeb440d40c200124bf9764a6bc70c26577497a685c0fe3882deab88

----------------------------------------------------------------
PREFLIGHT: ALL CHECKS PASSED
  manifest schema OK; advances_target aligned with control plane; paths resolve; SHA pinned for cross-stage integrity check.
