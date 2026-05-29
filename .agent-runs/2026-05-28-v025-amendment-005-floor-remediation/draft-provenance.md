# Draft Provenance — 2026-05-28-v025-amendment-005-floor-remediation

Drafted by manifest-drafter at 2026-05-28T17:32:00Z.

## Pre-flight checks

- `git status` — Currently on branch `v0.2.5-hotpatch`. Working tree is dirty with uncommitted changes in `README.md`, `src-tauri/Cargo.lock`, `src-tauri/src/core/mod.rs`, `src/components/OnboardingWizard.test.tsx`, `src/test_useapp_daily_scan_passes_settings_model.test.tsx`, `.agent-runs/2026-05-27-v024-hotpatch/walkthrough.md`, and untracked files `src/reproduction.test.tsx` and `.agent-runs/2026-05-28-v025-hotpatch/`. Since these changes lie outside this run's allowed paths, they must be committed, stashed, or reverted before the pipeline runs, except as explicitly directed by the pre-condition checkpoint which handles F-5/F-6 remnants.
- Priority check — `.agent-workflows/PROJECT_CONTROL_PLANE.md` defines the active target as `v0.2 Phase 4 — Source Tier + Prompt Library + Daily Scan + Plain Language Rewrite`. The requested feature run `2026-05-28-v025-amendment-005-floor-remediation` does not align with the control plane, so the `override_active_target` escape valve is used.

## Sources walked

- `.agent-workflows/PROJECT_CONTROL_PLANE.md` — Read for priority gate.
- `C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5\DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` — Read to copy goals, expected outputs, rollback plan, non-goals, and definition of done.
- `C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5\directive.yaml` — Read to copy run ID, branch, allowed/forbidden paths, and gates.
- `.pipelines/manifest-template.yaml` — Read for template structure, required gates, risk, and target repos.

## Field-by-field provenance

| Field | Source | Confidence |
|:------|:-------|:-----------|
| id | `directive.yaml` | high |
| type | Run parameters | high |
| branch | `directive.yaml` | high |
| goal | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` | high |
| allowed_paths | `directive.yaml` | high |
| forbidden_paths | `directive.yaml` | high |
| non_goals | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` | high |
| expected_outputs | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` | high |
| required_gates | `manifest-template.yaml` + `directive.yaml` | high |
| risk | `manifest-template.yaml` | high |
| rollback_plan | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` | high |
| definition_of_done | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` | high |
| director_notes | `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md` (empty) | high |
| advances_target | `PROJECT_CONTROL_PLANE.md` | high |
| authorizing_source | `PROJECT_CONTROL_PLANE.md` | high |
| override_active_target | Run parameters | high |
| target_repos | `manifest-template.yaml` (empty) | high |

## Hand-required fields

None this run — all fields have been derived from existing project assets, directives, and run parameters.
