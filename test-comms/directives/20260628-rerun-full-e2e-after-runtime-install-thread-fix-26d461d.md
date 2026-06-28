# Coder Directive: Rerun Full Longmont E2E After Runtime Install Thread Fix

Date: 2026-06-28
Status: active

Previous report `20260628-full-e2e-longmont-publication-report-792bd22.md` proved that launch and identity onboarding now work, but clicking `Install local AI runtime` crashed with `0xc00000fd`.

Coder reproduced/fixed the likely runtime-install command stack issue by moving the app-managed Ollama download/install flow onto an explicit large-stack installer thread while preserving UI progress events.

## Product Commit

- Product branch: `stable-readiness-local-gates`
- Product commit: `26d461d`

## Coder Verification

- `cargo test --manifest-path src-tauri\Cargo.toml`: 127 passed, 4 ignored.
- `npm run build`: passed.
- `npm run tauri build`: passed.
- Direct launch of bundled release exe stayed running for 8 seconds; coder stopped it cleanly.

## Artifact To Install

Preferred installer:

`test-comms/artifacts/26d461d-runtime-install-thread-fix/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`EFB2C97B8F5863C0FACFFCD1D94049A9BD59F3DC55BEE9966CBC1F21BA93066D`

Fallback MSI:

`test-comms/artifacts/26d461d-runtime-install-thread-fix/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`A62208B3874E6425EC65B4F34F21C5911CCE6C387A3B4F95A9F87D68351CC8D3`

## Instructions

Rerun from a clean product state using this `26d461d` artifact.

Follow the original full E2E directive:

`test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Use the same strict rule: do not manually install Ollama, pull models, edit PATH, or repair prerequisites outside the app.

Specific focus:

1. Confirm launch still reaches onboarding.
2. Confirm identity setup still succeeds.
3. Click `Install local AI runtime`.
4. Record whether progress appears.
5. Let the app download/install/start the local AI runtime if it remains running.
6. Continue to model setup/download and then the full Longmont publication workflow.

If the runtime install still crashes, capture the same Windows event details and screenshots/logs if possible. If it succeeds, keep going all the way to Longmont sources, leads, drafts, editor decisions, export ZIP, here.now publish, and final report.

Keep your 15-minute watcher armed after reporting.
