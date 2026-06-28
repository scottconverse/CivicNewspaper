# Checkpoint: ff21a83 draft-save rerun

Directive: `test-comms/directives/20260628-checkpoint-ff21a83-draft-save-rerun.md`

Related full rerun directive: `test-comms/directives/20260628-rerun-full-e2e-after-draft-save-scan-queue-fix-ff21a83.md`

Status: **blocked after partial completion**

## Current step

The ff21a83 rerun reached Publishing after completing clean install, onboarding, runtime/model setup, Longmont source import, Daily Scan, Story Queue verification, draft generation, saved draft verification, advisor review, approval, and local package export.

The current blocking step is here.now anonymous publish. The connector test passed, but `Publish with connector` did not produce a visible URL, deployment ID, error, or connector publish DB row after two attempts.

## App/runtime state

- App was still running at the time of checkpoint.
- Bundled local AI runtime was still running.
- Runtime/model setup passed during this clean run; it was not manually installed.
- Model downloaded by the app: `qwen2.5:7b`.

## Counts

- Imported/added sources: 3
  - 2 online official-record sources
  - 1 offline source on fetch
- Evidence items: 10
- Daily Scan runs: 1 completed
- Daily Scan lead count: 8
- Story Queue lead count after Daily Scan: 8
- Drafts generated: 1
- Drafts saved: 1
- Approved/exported stories: 1
- Local export publish run: 1
  - articles: 1
  - files: 18
  - skipped: 0

## Draft generation/save status

Draft generation passed. The generated draft opened in Workbench and saved without the previous `created_at` persistence error.

DB draft status after approval:

- `status`: `ready_to_publish`
- `attested_by`: `Cleanroom Tester`
- `attested_at`: non-null

## Continuing/blocker

I am blocked on the here.now anonymous publish step. The watcher remains armed.

The full report has been written separately:

`test-comms/reports/20260628-full-e2e-longmont-publication-report-ff21a83.md`

## Useful screenshots/artifacts

Artifact folder:

`test-comms/artifacts/20260628-full-e2e-longmont-publication-ff21a83/`

Key screenshots:

- `18-story-queue-after-daily-scan.png`
- `20-draft-generation-result.png`
- `21-workbench-draft-saved.png`
- `22-advisor-review-result.png`
- `23-compile-receipt.png`
- `24-herenow-connector-passed-test-inert-publish.png`

Sanitized package artifacts:

- `site-package-ff21a83.zip`
- `publish-manifest-ff21a83.json`
