# Antigravity Policy

CivicNewspaper is a desktop civic intelligence tool built with TypeScript, React, Vite, and Rust (Tauri). It empowers single-editor newsrooms to monitor municipal feeds, extract OSINT signals, and publish flat HTML sites locally.

## Pipeline drafter notes
- **Spec / Requirements**: Located in `README.md` and the `docs/` folder.
- **Architecture**: Defined in `docs/architecture.md`.
- **User Manual**: Located in `docs/user_manual.md`.

## Order of operations
- Branch from `main` for all new features.
- Work in small, verifiable slices.
- Tag and release features using semantic versioning at rung close.

## Tooling
- **Language**: TypeScript (Frontend), Rust (Backend)
- **Framework**: React, Vite, Tauri v2
- **Database**: SQLite (WAL mode)
- **Local AI**: Ollama HTTP bindings

## Non-negotiables
- 

## Memory precedence during pipeline runs
During active agent-pipeline-antigravity runs, the pipeline's chat gate keywords (`APPROVE` / `REVISE` / `REPLAN` / `BLOCK` / `VIEW`) and hook policy are authoritative. Operator-layer memory rules about asking-before-deciding (e.g. `feedback_no_unilateral_product_decisions.md`) apply OUTSIDE pipeline runs only. Inside a run, the v2.2.1 modal-budget hook denies every AskUserQuestion call; gates are chat-based, and non-gate decisions follow the adopt-and-proceed pattern from `skills/run/references/run.md`.
