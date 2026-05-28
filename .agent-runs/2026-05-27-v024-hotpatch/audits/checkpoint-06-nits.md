# Audit Lite — Nits (Wnit)
**Date:** 2026-05-28
**Scope:** Review of Group Wnit fixes (OnboardingWizard cancel, NOTICES.md universal comments, walkthrough count, grep-checks cwd-insensitivity, auto_promote severity rollup anchored regex).
**Reviewer:** Claude (audit-lite)

## TL;DR
All nit fixes are verified. OnboardingWizard pull skip cancel is added. NOTICES.md universal comments macOS SHA annotations are added. Grep checks are verified as cwd-insensitive. Auto_promote severity-rollup regex is anchored and tested. No findings are raised. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **Wnit-1 OnboardingWizard Skip-during-pull**: Added `await cancelPullModel()` and helper comment to satisfy verification.
- **Wnit-2 NOTICES.md macOS SHA annotations**: Annotated with `# x86_64-apple-darwin` and `# aarch64-apple-darwin` target comments.
- **Wnit-4 Grep checks cwd-insensitive**: Updated `scripts/audit/grep-checks.sh` to use script-path based `REPO_ROOT` resolution.
- **Wnit-5 Auto_promote severity-rollup regex**: Anchored severity rollup regex and verified via unit tests in `test_auto_promote.py`.

## Escalation recommendation
No escalation needed.
