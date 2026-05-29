# Draft Provenance — 2026-05-28-v025-hotpatch

Drafted by manifest-drafter at 2026-05-28T04:20:00Z.

## Pre-flight checks

- `git status` — Clean working tree on branch `v0.2.4-hotpatch`. Since the requested branch is `v0.2.5-hotpatch`, the pipeline run will switch branches.
- Priority check — `.agent-workflows/PROJECT_CONTROL_PLANE.md` defines the active target as `v0.2 Phase 4 — Source Tier + Prompt Library + Daily Scan + Plain Language Rewrite`. The requested bugfix run `v0.2.5-hotpatch` does not align with the control plane, so the `override_active_target` escape valve is used.

## Sources walked

- `.agent-workflows/PROJECT_CONTROL_PLANE.md` — Read for priority gate.
- `.agent-runs/2026-05-27-v024-hotpatch/manifest.yaml` — Read to copy standard baseline paths and settings for a hotpatch release.
- Main Agent Instruction (system message) — Received explicit instructions from the caller agent to proceed with `v0.2.5-hotpatch` goals, definition of done, and target override reason.

## Field-by-field provenance

| Field | Source | Confidence |
|:------|:-------|:-----------|
| id | given by orchestrator | n/a |
| type | given by orchestrator | n/a |
| branch | derived from user_description | high |
| goal | main agent instructions | high |
| allowed_paths | copied from v0.2.4-hotpatch | high |
| forbidden_paths | copied from v0.2.4-hotpatch | high |
| non_goals | copied from v0.2.4-hotpatch | high |
| expected_outputs | main agent instructions | high |
| required_gates | copied from v0.2.4-hotpatch | high |
| risk | copied from v0.2.4-hotpatch | high |
| rollback_plan | copied from v0.2.4-hotpatch | high |
| definition_of_done | main agent instructions | high |
| director_notes | default empty | high |
| advances_target | derived from user_description | high |
| authorizing_source | default empty (active target override) | high |
| override_active_target | main agent instructions | high |

## Hand-required fields

None this run — all fields have been derived from existing project assets and the main agent's instructions.
