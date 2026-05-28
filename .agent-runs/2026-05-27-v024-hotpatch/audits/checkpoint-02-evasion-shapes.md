# Audit Lite — Evasion Shapes (WE)
**Date:** 2026-05-28
**Scope:** Review of Group WE evasion-shape closures and tactical fixes.
**Reviewer:** Claude (audit-lite)

## TL;DR
All evasion-shape closures are verified. Tactical fixes for M-1 through M-6 are correct, and paragraph-aware checks, fitness tests, and validation rules have been successfully established. No findings are raised. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **WE-1 (E5-1 grep-pattern-as-product-string)**: Leaked grep pattern removed. The interactive Continue button functions as asserted in the Vitest suite.
- **WE-2 (E5-2 phrasing-variant-evades-grep)**: Phrasing revised in `docs/user_manual.md` to cleanly describe the `LlmClient` trait instead of HTTP mocking server.
- **WE-3 (E5-3 unauthorized-cfg-gate-outer)**: Conditionals replaced with authorized `cfg_attr(target_os = "windows", ignore)` attributes matching the authorized count of 7.
- **WE-4 (E5-4 script-claims-vs-does)**: Paragraph-aware logic implemented using `awk RS=""` in `check-ollama-install-invariant.sh`.
- **WE-5 (E5-5 grep-tuned-to-code)**: Default model updated to `phi3:mini` and `grep-checks.sh` rewritten to block all literals and string concatenations of model names.
- **WE-6 (E5-6 causal-explanation-fabrication)**: misleading "CUDA/ROCm bundled" size claim replaced with actual upstream Ollama releases cited details.

## Escalation recommendation
No escalation needed.
