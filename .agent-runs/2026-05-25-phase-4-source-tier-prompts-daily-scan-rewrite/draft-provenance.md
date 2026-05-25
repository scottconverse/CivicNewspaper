# Draft Provenance — 2026-05-25-phase-4-source-tier-prompts-daily-scan-rewrite

Drafted by manifest-drafter at 2026-05-25T06:10:00Z.

## Pre-flight checks

- `git status` showed working tree dirty (only setup artifacts like `.gitignore` changes and `.agent-runs/`), not outside plausible scope. Checked out new branch `feature/phase-4-source-tier-prompts-daily-scan-rewrite`.
- `PROJECT_CONTROL_PLANE.md` found and active target verified.

## Sources walked

- `.agent-workflows/PROJECT_CONTROL_PLANE.md` — §Active target, §v0.2 release plan, §Cross-run invariants (used for advances_target, authorizing_source, non_goals, forbidden_paths)
- `docs/spec/v0.2-phase-4.md` — §Goal, §4a-4d, §Phase 4 tests, §Phase 4 done when (used for goal, allowed_paths, expected_outputs, definition_of_done, director_notes, risk, rollback_plan)
- `Antigravity.md` — §Tooling, §Order of operations (used for branch convention validation)

## Field-by-field provenance

| Field | Source | Confidence |
|:------|:-------|:-----------|
| id | given by orchestrator | n/a |
| type | given by orchestrator | n/a |
| branch | Git status / inferred convention | high |
| goal | `docs/spec/v0.2-phase-4.md` | high |
| allowed_paths | `docs/spec/v0.2-phase-4.md` + test file layout inference | high |
| forbidden_paths | `.agent-workflows/PROJECT_CONTROL_PLANE.md` (rust strict files + CI immutable files) | high |
| non_goals | `.agent-workflows/PROJECT_CONTROL_PLANE.md` §v0.2 release plan | high |
| expected_outputs | `docs/spec/v0.2-phase-4.md` + auto-added CHANGELOG | high |
| required_gates | template default | n/a |
| risk | inferred from schema migrations | high |
| rollback_plan | derived from schema migrations down-migration | high |
| definition_of_done | `docs/spec/v0.2-phase-4.md` §Phase 4 done when | high |
| director_notes | `docs/spec/v0.2-phase-4.md` | high |
| advances_target | `.agent-workflows/PROJECT_CONTROL_PLANE.md` §Active target | high |
| authorizing_source | `.agent-workflows/PROJECT_CONTROL_PLANE.md:8` | high |

## Hand-required fields

None this run — fully auto-derivable from project artifacts.
