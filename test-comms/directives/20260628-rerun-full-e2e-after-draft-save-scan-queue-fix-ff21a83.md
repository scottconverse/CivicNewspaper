# Coder Directive: Rerun Full Longmont E2E After Draft Save / Scan Queue Fix

Date: 2026-06-28
Status: active

Previous report `20260628-full-e2e-longmont-publication-report-26d461d.md` proved:

- clean install and launch work,
- app-managed Ollama runtime install works,
- model download works,
- Longmont source discovery/import works enough to proceed,
- Daily Scan runs with local AI,
- Scrape & Detect can produce Story Queue leads,
- but draft generation failed when saving a generated draft because the draft IPC payload lacked `created_at`.

Coder fixed the draft persistence issue and also made Daily Scan leads appear in the main draftable Story Queue.

## Product Commit

- Product branch: `stable-readiness-local-gates`
- Product commit: `ff21a83`

## Coder Verification

- `cargo test --manifest-path src-tauri\Cargo.toml`: 128 passed, 4 ignored.
- `npm run build`: passed.
- `npm run tauri build`: passed.
- Direct launch of bundled release exe stayed running for 8 seconds; coder stopped it cleanly.

## Artifact To Install

Preferred installer:

`test-comms/artifacts/ff21a83-draft-save-scan-leads/The Civic Desk_0.2.8_x64-setup.exe`

SHA256:

`879CC345B1A01D2673B525712BEA89258008877A953592284434AD8CFFEAEF02`

Fallback MSI:

`test-comms/artifacts/ff21a83-draft-save-scan-leads/The Civic Desk_0.2.8_x64_en-US.msi`

SHA256:

`70F411BC2E163BDE2BD6A68E9B139CE51E51C7272B4B6150A061E0F662A2EF6F`

## Instructions

Rerun from a clean product state using this `ff21a83` artifact.

Follow the original full E2E directive:

`test-comms/directives/20260628-full-e2e-cleanroom-longmont-publication.md`

Specific focus:

1. Confirm clean install, launch, onboarding, runtime install, and model download still pass.
2. Run Longmont source discovery/import.
3. Run Daily Scan.
4. Confirm Story Queue now shows draftable leads after Daily Scan without requiring Scrape & Detect as the only path.
5. Generate at least one draft and confirm it saves.
6. Continue to produce the required Longmont publication:
   - 10 to 25 leads if possible,
   - 5 to 10 reader-facing stories/briefs,
   - writer/editor workflow controls,
   - advisory review on at least one story,
   - export ZIP/publication package,
   - here.now anonymous publish,
   - final report and artifact path/URL.

Do not manually install Ollama or models. Do not hand-write story content outside the app.

If blocked again, report the exact break and keep your watcher armed.
