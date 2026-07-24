# Repo-Local Agent Pipeline Retirement

Status: retired permanently on 2026-07-24.

## Decision

The repo-local self-grading agent pipeline used during the v0.2 development
cycle is retired. CivicNewspaper does not use it for development acceptance,
promotion, release decisions, or application runtime behavior.

The project will not add a sealed callable store merely to preserve that
pipeline. Its executable entry points, automatic promotion logic, mutable
thresholds, and obsolete phase-specific DoD gates have been removed instead.
GitHub Actions and maintainer review are the active project gates.

This retirement closes the risk tracked by issue #19 and carried-debt item
`P5-000`: executor-controlled validation code cannot approve current work
because the callable pipeline no longer exists or participates in any current
workflow.

## Current trust boundary

Current acceptance and release decisions are based on:

- workflows under `.github/workflows/`;
- executable Rust and frontend tests;
- measured coverage gates;
- release smoke, dependency, installer, signature, and cleanroom evidence;
- GitHub branch protection and maintainer review.

No file under `.agent-runs/`, `.agent-workflows/`, `audit-*`, or `forensic/`
is an active acceptance authority. Those paths are historical records only.
They cannot promote a commit or authorize a release.

## Preserved history

The following material remains for incident analysis and historical context:

- `.agent-runs/` — committed v0.2 run records;
- `.agent-workflows/scope-overrides.md` — historical override ledger;
- `.agent-workflows/section2-auth.json` — final empty authorization record;
- `forensic/` — pipeline-integrity incident reports;
- `audit-civicnews-2026-05-28/` — historical audit output.

Historical records may describe commands or states that no longer exist. They
must not be treated as current instructions.

## Conditions for any future replacement

Reintroducing a repo-local agent acceptance system requires a new design and a
new security review. At minimum, that design must:

1. keep validation code, policy, thresholds, and promotion authority outside
   executor-writable worktrees;
2. prevent the executor from writing its own verdict or approval;
3. fail closed when required evidence or validation inputs are absent;
4. run through maintainer-owned CI with reviewable logs;
5. document the trust boundary and threat model before activation.

This is a future-system requirement, not deferred work for the retired
pipeline. A future proposal must open a new issue and may not reactivate the
historical files.
