# Draft Provenance — 2026-05-25-phase-4-remediation

Drafted by manifest-drafter at 2026-05-26T05:28:00Z.

## Pre-flight checks

- `git status` — Clean working tree on branch `v0.2-phase-4`.
- Priority check — `.agent-workflows/PROJECT_CONTROL_PLANE.md` confirms active target is Phase 4.

## Sources walked

- `.agent-workflows/PROJECT_CONTROL_PLANE.md` — Read for priority gate, active target, and cross-run invariant forbidden paths.
- `C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5\directive-phase-4-remediation.md` — The user's provided directive. Read for goal, allowed paths, non-goals, expected outputs, DoD, and director notes.
- `docs/spec/v0.2-phase-4.md` — Read to cross-reference Phase 4 files mentioned in the directive.
- File system tree (`src-tauri/src/`, etc.) — Walked to map high-level module references into exact `allowed_paths`.

## Field-by-field provenance

| Field | Source | Confidence |
|:------|:-------|:-----------|
| id | given by orchestrator | n/a |
| type | given by orchestrator | n/a |
| branch | directive-phase-4-remediation.md | high |
| goal | directive-phase-4-remediation.md § Objective | high |
| allowed_paths | directive-phase-4-remediation.md + docs/spec/v0.2-phase-4.md | high |
| forbidden_paths | PROJECT_CONTROL_PLANE.md § Cross-run invariants | high |
| non_goals | directive-phase-4-remediation.md § 1 & 6 | high |
| expected_outputs | directive-phase-4-remediation.md § 5 | high |
| required_gates | template default + execute_readiness from directive's implementation-report requirement | high |
| risk | inferred from schema migrations and LLM interactions | medium |
| rollback_plan | inferred from SQLite and migration needs | medium |
| definition_of_done | directive-phase-4-remediation.md § 5 (all 14 findings remediated + zero blockers/criticals) | high |
| director_notes | directive-phase-4-remediation.md § 4 & 7 | high |
| advances_target | PROJECT_CONTROL_PLANE.md § Active target | high |
| authorizing_source | PROJECT_CONTROL_PLANE.md:8 | high |

## Hand-required fields

None this run — fully auto-derivable from project artifacts and the explicit directive.
