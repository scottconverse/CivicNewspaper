# 2aba587 Full E2E Checkpoint

Directive: `test-comms/directives/20260628-rerun-full-e2e-after-herenow-preview-fix-2aba587.md`  
Checkpoint request: `test-comms/directives/20260628-checkpoint-2aba587-full-e2e-rerun.md`  
Status: complete

## Watcher

Watcher is still armed. I will continue checking for new directives on the heartbeat cadence unless coder explicitly ends the watcher.

## Artifact Verification

Found and hash-verified the preferred installer:

- `test-comms/artifacts/2aba587-herenow-preview-publish/The Civic Desk_0.2.8_x64-setup.exe`
- SHA256 observed: `E698D542096C179AEC46A73AC9E68DB984823C6A8C964FB02AF72A018D524D1D`

Also observed the fallback MSI hash matched the directive:

- `D4F368B6F0BB07AEE8F36486AE6303C88946C1A1C45AC7349F0A703DDD84776D`

## Current Step Reached

The full 2aba587 cleanroom rerun is complete. Full report:

`test-comms/reports/20260628-full-e2e-longmont-publication-report-2aba587.md`

## Product Flow Status

- Installation: complete
- Launch: complete
- First-run onboarding: complete
- App-managed runtime install/start: complete
- App-managed model download: complete, `qwen2.5:7b`
- Source discovery/import: complete, 4 sources imported
- Daily Scan: complete, 8 reviewable leads
- Drafting/editorial: complete, 5 ready-to-publish drafts and 1 held draft
- Static compile/export: complete, 5 articles, 22 files, 0 skipped
- here.now verification: complete
- here.now publish: complete

Public preview URL:

https://emerald-island-gevx.here.now

## Blockers / Notes

No blocker remains for the 2aba587 directive.

Observed notes:

- Daily Scan produced 8 leads, below the 10-lead target, after expansion/import of 4 sources.
- Queue behavior allowed repeated drafting of the same lead, producing duplicate Vision Zero draft records.
- One imported source, `Longmont Public Safety`, was offline during scan.

Screenshots and package artifacts are under:

`test-comms/artifacts/20260628-full-e2e-longmont-publication-2aba587/`
