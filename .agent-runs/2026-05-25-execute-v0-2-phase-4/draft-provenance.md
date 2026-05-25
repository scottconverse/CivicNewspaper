# Draft Provenance — 2026-05-25-execute-v0-2-phase-4

Drafted by manifest-drafter.

## Pre-flight checks

- `git status` indicates the working tree is clean and already on branch `v0.2-phase-4`, matching the user's branch intention.

## Sources walked

- `.agent-workflows/PROJECT_CONTROL_PLANE.md` — § "Active target" and § "v0.2 release plan" (used for advances_target, authorizing_source, and non_goals).
- `docs/spec/v0.2-phase-4.md` — The authoritative design spec (used for goal, allowed_paths, expected_outputs, definition_of_done, and director_notes).
- `Antigravity.md` — Project conventions.

## Field-by-field provenance

| Field | Source | Confidence |
|:------|:-------|:-----------|
| id | given by orchestrator | n/a |
| type | given by orchestrator | n/a |
| branch | user_description and git status | high |
| goal | docs/spec/v0.2-phase-4.md §1 | high |
| advances_target | .agent-workflows/PROJECT_CONTROL_PLANE.md:8 | high |
| authorizing_source | .agent-workflows/PROJECT_CONTROL_PLANE.md:8 | high |
| allowed_paths | docs/spec/v0.2-phase-4.md and user_description | high |
| forbidden_paths | .agent-workflows/PROJECT_CONTROL_PLANE.md and user_description | high |
| non_goals | .agent-workflows/PROJECT_CONTROL_PLANE.md and user_description | high |
| expected_outputs | docs/spec/v0.2-phase-4.md | high |
| required_gates | template default | n/a |
| risk | user_description | high |
| rollback_plan | user_description | medium |
| definition_of_done | docs/spec/v0.2-phase-4.md exit criteria | high |
| director_notes | docs/spec/v0.2-phase-4.md prerequisites and notes | high |

## Hand-required fields

None this run — fully auto-derivable from project artifacts.
