# CivicNewspaper Cleanroom E2E Attempt 4 - a094ce1

UTC report time: 2026-06-30T12:22:00Z

Verdict: FAIL

## Break Point

The run failed at Daily Scan before any leads were produced.

The typed first-run identity values were accepted on the visible Identity form before clicking Next. Evidence `03-identity-values-before-next.json` and `03b-identity-values-before-next.json` show:

- Publication Name: `Attempt Four Longmont Ledger`
- Editor Name: `A094 Tester Editor`
- City: `Longmont`
- State: `CO`

However, after first-run setup completed, the app shell displayed the location as:

`LONGMONT / CO94 TES`

Daily Scan then failed with:

`Something went wrong: Invalid city or state format`

Evidence `18-daily-scan-invalid-city-state-failure.json` captures the failure state:

- `SOURCES WATCHED`: 7
- `OPEN LEADS`: 0
- `DRAFTS IN DESK`: 0
- `AI STATUS`: Ready
- `SCAN PROGRESS`: `Something went wrong: Invalid city or state format`
- `Evidence: 0. Saved leads: 0.`

I did not manually edit product data to repair the corrupted city/state value because the directive is a cleanroom first-run/product-behavior test.

## Setup Results

- Product clean wipe: completed.
- NSIS install: completed.
- App launch: completed.
- Typed first-run identity values visible before Next: yes.
- Typed identity values accepted through the Identity step: partially. They advanced the app, but the resulting app location was corrupted from `CO` into `CO94 TES`.
- App-guided local AI setup: completed to `Local AI ready`.
- Model: `qwen2.5:7b`.
- Manual Ollama/model installation by tester: no.
- Source discovery/import: completed enough to import 7 sources through the app UI.
- Daily Scan: failed before producing leads because of invalid city/state format.

## Publish Results

- here.now URL: none.
- Local output path on tester: none produced.
- Copied output path: `test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/`
- ZIP path and SHA256: none produced.
- Leads: 0.
- Generated drafts: 0.
- Clean approved stories: 0.
- Published stories: 0.

## Draft and Topic Results

- Plain Draft leads: none produced.
- Draft anyway leads: none produced.
- Draft anyway leads used: none.
- Duplicate-topic clustering: not reached.
- Drafts clean before manual editing: not reached.
- Drafts with forbidden markers: not reached.

## Evidence Files

Evidence is under:

`test-comms/artifacts/20260630-cleanroom-e2e-a094ce1/tester-output/evidence/`

Key evidence:

- `00-clean-wipe-summary.json`
- `01-install-launch-summary.json`
- `02-first-launch.png`
- `02-first-launch-dom.json`
- `03-identity-values-before-next.json`
- `03-identity-values-typed-before-next.png`
- `03b-identity-values-before-next.json`
- `03b-identity-values-typed-before-next.png`
- `04-after-identity-next.json`
- `04-after-identity-next.png`
- `04b-after-identity-next.json`
- `04b-after-identity-next.png`
- `05-ai-service-setup-start.png`
- `06-ai-service-wait-summary.json`
- `06-ai-setup-current.png`
- `07-model-download-progress-00.png`
- `07-model-download-progress-06.png`
- `07-model-download-progress-12.png`
- `07-model-download-progress-18.png`
- `07-model-download-progress-24.png`
- `07-model-download-progress-30.png`
- `07-model-download-progress-36.png`
- `07-model-download-progress-42.png`
- `07-model-download-progress-48.png`
- `07-model-download-progress-54.png`
- `07-model-download-progress-60.png`
- `07-model-download-progress-66.png`
- `07-model-download-progress-72.png`
- `07-model-download-progress-78.png`
- `07-model-download-progress-84.png`
- `08-setup-step4-defaults.json`
- `08-setup-step4-defaults.png`
- `09-after-setup-complete.json`
- `09-after-setup-complete.png`
- `10-sources-before-discover.png`
- `11-sources-after-discover.png`
- `11-sources-discover-summary.json`
- `12-selected-sources-before-import.json`
- `12-selected-sources-before-import.png`
- `13-sources-after-import.json`
- `13-sources-after-import.png`
- `14-daily-scan-start.png`
- `15-daily-scan-progress-00.png`
- `15-daily-scan-progress-06.png`
- `15-daily-scan-progress-12.png`
- `15-daily-scan-progress-18.png`
- `15-daily-scan-progress-24.png`
- `15-daily-scan-progress-30.png`
- `15-daily-scan-progress-36.png`
- `15-daily-scan-progress-42.png`
- `15-daily-scan-progress-48.png`
- `18-daily-scan-invalid-city-state-failure.json`
- `18-daily-scan-invalid-city-state-failure.png`

## Watcher

The watcher remains armed.
