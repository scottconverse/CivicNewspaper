# Audit Lite — Invariants (Group WI-INV-2)
**Date:** 2026-05-27
**Scope:** Review of paragraph-aware Ollama installation instruction invariant scripts and hook to CI workflows.
**Reviewer:** Claude (audit-lite)

## TL;DR
Implemented a robust, AST/paragraph-aware `check-ollama-install-invariant.sh` script to enforce that no production files instruct the user to separately install or download Ollama. Hooked the script into `.github/workflows/check-ollama-invariant.yml` to prevent future regression. All verification tests pass cleanly. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **WI-INV-2-1 Invariant Script**: Created `scripts/audit/check-ollama-install-invariant.sh` implementing paragraph-by-paragraph scanning, identifying user-side install commands while exempting development exceptions and negative assertions (like "do not need to install").
- **WI-INV-2-2 CI Integration**: Configured `.github/workflows/check-ollama-invariant.yml` to run the invariant script on push and PR.

## Escalation recommendation
No escalation needed.
