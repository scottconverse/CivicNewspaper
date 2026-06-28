# Checkpoint: 26d461d runtime install rerun

- Report time: 2026-06-28T09:04:58Z
- Tester: Codex desktop cleanroom tester
- Directive: `test-comms/directives/20260628-checkpoint-runtime-install-rerun-26d461d.md`
- Underlying rerun directive: `test-comms/directives/20260628-rerun-full-e2e-after-runtime-install-thread-fix-26d461d.md`
- Product branch: `stable-readiness-local-gates`
- Product commit: `26d461dd3507aead46d7bfba3c5310e8d4a7c54d`
- Current status: **runtime/model path passed; full E2E blocked later at draft save**

## Current Step

The rerun has progressed beyond runtime install and model setup. The app is currently open in the draft-generation flow after attempting to generate the first story draft.

The full report is:

```text
test-comms/reports/20260628-full-e2e-longmont-publication-report-26d461d.md
```

## Runtime / Model Status

- App running: **yes**
- Runtime download progress visible: **was visible and completed**
- Runtime install crash: **no crash in this build**
- App-local runtime process appeared: **yes**
- Model store appeared: **yes**
- Recommended model download: **completed**
- Model in app status: **Local AI ready / qwen2.5:7b**

## Later Blocker

After source import, Daily Scan, and Scrape & Detect, the first draft generation failed with:

```text
Draft generation failed: Something went wrong: invalid args `draft` for command `save_draft`: missing field `created_at`
```

This blocks editor review, export ZIP, and here.now publishing.

## Evidence

Screenshots are under:

```text
test-comms/artifacts/20260628-full-e2e-longmont-publication-26d461d/
```

Useful checkpoint screenshots:

- `04-after-click-install-runtime-3s.png` - app showed runtime download/install progress.
- `06-runtime-service-green.png` - runtime installed and local AI service ready.
- `09-model-download-progress-2min.png` - model download complete.
- `13-workspace-after-onboarding.png` - workspace opened with local AI ready.
- `33-generate-draft-result.png` - later draft save failure.

## Continuing?

The watcher remains armed, but this specific rerun is blocked at draft persistence until coder fixes or clarifies the `save_draft` `created_at` failure.
