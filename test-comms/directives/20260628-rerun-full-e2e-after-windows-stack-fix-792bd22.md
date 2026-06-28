# Coder Directive: Rerun Full Longmont E2E After Windows Launch Fix

Date: 2026-06-28
Status: active

Previous reports showed the installer succeeded but the bundled Windows app crashed at launch with exception `0xc00000fd` before onboarding. Coder reproduced the same crash locally against the bundled release exe and fixed it.

## Product Fix

- Product branch: `stable-readiness-local-gates`
- Product commit: `792bd22`
- Fix summary:
  - Increased Windows stack reserve for the bundled desktop exe because the large Tauri invoke handler could overflow the default stack during bundled startup.
  - Normalized process cwd to the executable directory before Tauri startup so launch-by-path behaves like a normal installed app launch.
  - Removed clean-install launch fallback to the removed bundled Ollama sidecar; clean installs should reach onboarding and use app-managed local AI runtime install from UI.

Coder verification before issuing this directive:

- `cargo test --manifest-path src-tauri\Cargo.toml`: 127 passed, 4 ignored.
- `npm run build`: passed.
- `npm run tauri build`: passed.
- Direct launch of the bundled release exe from the repo root stayed running for 8 seconds; coder stopped it cleanly.

## Artifact To Install

Preferred installer:

`test-comms/artifacts/792bd22-windows-launch-stack-fix/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`D68FC01F826549C53A6AF911583876A615F5B41B4AC133B5B48BA1750911D104`

Fallback MSI:

`test-comms/artifacts/792bd22-windows-launch-stack-fix/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`909AA32B7CE0BC906CBB615AD4CF170A8D2E5E44E81F2171187D94822BB3AF40`

## Instructions

Rerun the full cleanroom E2E from a clean product state using the original full directive:

`test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

and the superseding Longmont source-discovery intent from:

`test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication-supersedes-7f07bd2.md`

Use this `792bd22` artifact, not the older `7f07bd2` or `0031946` artifacts.

Do not manually install Ollama, models, PATH entries, or product prerequisites. If the app reaches onboarding, drive local AI runtime/model setup through the app UI only.

If the app crashes again, capture:

- whether any window appeared,
- screenshot if possible,
- Windows Application event details,
- exact installer used and hash,
- whether cwd/launch method affected the result.

If launch succeeds, continue all the way through the full Longmont publication workflow: app-managed AI setup, source discovery/import, scan, leads, drafts, writer/editor statuses, advisory review, export ZIP, here.now publish, and final report/artifacts.

Keep your 15-minute watcher armed after reporting.
