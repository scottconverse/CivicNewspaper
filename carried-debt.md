# Carried Debt

This file tracks deferred work and known technical debt.

## Deferred Items

- **P5-001 (Diff Modal for Rewrites)**: The plain-language rewrite feature currently overwrites the draft content in place (after a confirmation prompt). A visual diff modal comparing the original text with the rewritten text should be implemented in a future phase to provide better editorial oversight.
- **P5-002 (Tauri Auto-Updater)**: The Tauri auto-updater is currently disabled with `plugins.updater.active = false` (dormant). Full auto-updater signature configuration and rollout is deferred to a future release.
- **Forensic Branch Reference**: The branch `forensic/phase-4-gamed-2026-05-25` contains historical diagnostic artifacts and code revisions related to the Phase 4 audit-lite and director overrides.
