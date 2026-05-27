# Audit Lite — Binaries (WB group)
**Date:** 2026-05-27
**Scope:** Scoped review of Linux .deb package size investigation and decoupling of the test fixture from production bundles.
**Reviewer:** Claude (audit-lite)

## TL;DR
Linux binary size is confirmed to be 1.07 GB by design upstream due to embedded acceleration runtimes. Decoupled test fixtures from production bundles and successfully routed tests to standard process spawns pointing to `src-tauri/tests/fixtures/`. No findings. Ship.

## Severity rollup
- Blocker: 0
- Critical: 0
- Major: 0
- Minor: 0
- Nit: 0

## Findings
None.

## What's working
- **WB-1 Investigation**: Documented the ~1.07 GB Linux binary size in `.agent-runs/2026-05-26-civicnewspaper-v020-ship/stage-WB-1-report.md` confirming it matches upstream Ollama design.
- **WB-2 Decoupled Test Fixtures**: `test-ollama-fixture` binaries moved to `src-tauri/tests/fixtures/` and removed from `tauri.conf.json:bundle.externalBin`, ensuring they are excluded from production installers.
- **SidecarChild implementation**: Spawns standard command processes for sidecar mocking in test builds while keeping Tauri sidecars in production, compiling without warnings or clippy errors.

## Escalation recommendation
No escalation needed.
