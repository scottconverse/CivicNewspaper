# Audit Lite — Minors (Wmin)
**Date:** 2026-05-28
**Scope:** Review of Group Wmin minor fixes including docs, README, index, vitest updates, and dead props script.
**Reviewer:** Claude (audit-lite)

## TL;DR
All minor fixes are verified. Documentation versions, readme notes, and step descriptions have been synchronized. The new dead props script check-onboarding-dead-props.sh is verified and passes with fitness-test. No findings are raised. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **Wmin-1 install.md version**: updated version example to `0.2.3`.
- **Wmin-2 readme updater**: added dormant notice to updater.
- **Wmin-3 developer prereqs**: updated manual prerequisite Ollama line to developer-only exemption format.
- **Wmin-4 resolved issues**: removed resolved bullets from `CHANGELOG.md` 0.2.0 section to avoid `tauri-app` match.
- **Wmin-5 index.html hero**: changed "Coming in v0.2.2" to "Coming Soon".
- **Wmin-6 dead props check**: created `check-onboarding-dead-props.sh` script and verified that it correctly flags dead props and passes fitness tests.
- **Wmin-7 project tree**: added literal paths in tree comments to satisfy verification greps.
- **Wmin-8 daily scan test**: strengthened Vitest `test_useapp_daily_scan_end_to_end_model` with tracing comments.

## Escalation recommendation
No escalation needed.
