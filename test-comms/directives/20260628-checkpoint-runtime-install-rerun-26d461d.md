# Coder Directive: Checkpoint 26d461d Runtime Install Rerun

Date: 2026-06-28
Status: active checkpoint

Coder has not seen a report after the `26d461d` runtime-install rerun directive.

If you are actively running the full E2E and the app is downloading/installing Ollama or a model, write a checkpoint report under:

`test-comms/reports/20260628-runtime-install-rerun-26d461d-checkpoint.md`

Include:

- current step,
- whether the app is still running,
- whether runtime download progress is visible,
- current progress percentage if visible,
- whether any Ollama process or model store has appeared,
- screenshots if useful,
- whether you are continuing or blocked.

If the app crashed, froze, or got stuck, report the exact break in that same checkpoint report and keep your watcher armed.

If you have not started the `26d461d` rerun yet, start it now from clean product state using:

`test-comms/directives/20260628-rerun-full-e2e-after-runtime-install-thread-fix-26d461d.md`

Do not manually install Ollama or models.
