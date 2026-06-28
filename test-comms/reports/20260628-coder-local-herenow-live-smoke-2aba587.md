# Coder-side local here.now live smoke: 2aba587

Role: coder-side verification, not a cleanroom tester pass

Product branch: `stable-readiness-local-gates`

Product commit: `2aba587`

Result: **PASS for live here.now anonymous connector path from a compiled output folder**

This is not the final cleanroom product acceptance run. The cleanroom tester still needs to rerun the full Longmont E2E directive and produce the 5+ item paper from the installed app. This smoke only verifies that the here.now publisher path can create a live anonymous preview from a valid compiled CivicNewspaper output folder.

## Input

Used prior cleanroom exported package:

`test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/site-package-ff21a83.zip`

Expanded locally to:

`C:\Users\instynct\Desktop\CODE\civicnewspaper\.agent-runs\herenow-live-2aba587`

Important note: the expanded folder initially lacked `site-package.zip`; publisher validation correctly rejected it as incomplete. I copied the ZIP back into the expanded folder as `site-package.zip` and reran the smoke.

## Command

Ran the ignored live gate:

`cargo test --manifest-path src-tauri\Cargo.toml local_herenow_anonymous_publishes_compiled_site -- --ignored --nocapture`

Environment:

- `CIVIC_DESK_HERENOW_OUTPUT_DIR=C:\Users\instynct\Desktop\CODE\civicnewspaper\.agent-runs\herenow-live-2aba587`
- `CIVIC_DESK_HERENOW_RECEIPT=C:\Users\instynct\Desktop\CODE\civicnewspaper\.agent-runs\herenow-live-2aba587-receipt.json`

## Result

The live gate passed.

Receipt:

```json
{
  "deployment_id": "slug=onyx-bodhi-h7cd;version=01KW6X7Q1Y9K9536VAKCP8SPN9;created_slug=onyx-bodhi-h7cd",
  "message": "Published a temporary here.now preview. Save an API key for permanent sites. Expires at 2026-06-29T10:43:36.126Z.",
  "provider": "here_now",
  "published_url": "https://onyx-bodhi-h7cd.here.now"
}
```

URL:

`https://onyx-bodhi-h7cd.here.now`

## Remaining cleanroom requirement

Tester still needs to complete:

- clean install of `2aba587`
- full UI-driven Longmont run
- app-managed AI runtime/model setup
- source discovery/import
- Daily Scan
- 5+ approved reader-facing items
- local ZIP export
- here.now publish from the installed app UI
- final human-readable report with output path and URL
