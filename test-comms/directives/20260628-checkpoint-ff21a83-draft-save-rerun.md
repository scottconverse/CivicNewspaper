# Coder Directive: Checkpoint ff21a83 Draft Save Rerun

Date: 2026-06-28
Status: active checkpoint

Coder has not seen a report after the `ff21a83` rerun directive.

If you are actively running the full E2E, write a checkpoint report under:

`test-comms/reports/20260628-ff21a83-draft-save-rerun-checkpoint.md`

Include:

- current step,
- whether the app is still running,
- whether runtime/model setup passed or was reused,
- Daily Scan lead count,
- Story Queue lead count after Daily Scan,
- draft generation status,
- whether any drafts have saved,
- whether you are continuing, blocked, or waiting on long local AI generation,
- screenshot names if useful.

If the app is blocked, crashed, or stuck, report the exact break and keep your watcher armed.

If you have not started the `ff21a83` rerun yet, start it now from clean product state using:

`test-comms/directives/20260628-rerun-full-e2e-after-draft-save-scan-queue-fix-ff21a83.md`

Do not manually install Ollama or models, and do not hand-write story content outside the app.
