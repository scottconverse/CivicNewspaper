# Next-Codex implementation handoff — v0.3.2 candidate

Copy the prompt below into a new Codex instance after it has cloned this repository and checked out the current handoff PR branch; use `main` only after this PR has merged. The prompt deliberately separates what is proven locally from what is still required before a tag or hosted release.

```text
You are taking over CivicNewspaper, a Windows-focused Tauri 2 + React/TypeScript local-newsroom application.

Repository: https://github.com/scottconverse/CivicNewspaper
Working directory after clone: <your-clone>/civicnewspaper
Until PR #32 is merged, check out `agent/v032-candidate-handoff` after fetching it. If the PR has merged, use current `origin/main` instead.
Current product line: v0.3.2 Windows public beta. Do NOT create or move a Git tag, publish or replace a GitHub Release asset, trigger a GitHub Pages deployment, or treat the current local candidate as the hosted download unless the owner explicitly asks.

First, orient to the actual checkout rather than this note:

1. Run `git fetch origin --prune`, `git status --short --branch`, `git log --oneline -8`, and `git remote -v`.
2. Read `AGENTS.md` if present, then read `README.md`, `docs/release-readiness.md`, `docs/manual-smoke.md`, `docs/release-evidence/v0.3.2.json`, and `docs/release-evidence/v0.3.2-local-isolated-package-report.md`.
3. Read `package.json`, `src-tauri/Cargo.toml`, `scripts/release-smoke.ps1`, `scripts/packaged-first-run-walkthrough.ps1`, and `scripts/packaged-webview-driver.mjs` before changing release, packaging, or first-run code.
4. Treat the current branch/commit and files as authoritative. Do not assume old handoff hashes, release bodies, or Pages content are current.

What this candidate changed and proved locally:

- Replaced coordinate/SendKeys first-run automation with accessible Playwright-over-WebView2-CDP automation. `scripts/packaged-webview-driver.mjs` drives first-run by accessible names, intentionally selects `Skip for now`, proves `onboarding_complete` and `ai.setup_skipped`, and reaches usable zero-source Daily Scan guidance.
- Extended `scripts/packaged-first-run-walkthrough.ps1` to produce hash-backed isolated package evidence and to run a live-model installed-package core flow: controlled municipal sources -> Daily Scan -> linked-evidence lead -> generated/saved draft -> Workbench reload.
- Added a direct Rust Tauri IPC regression for `generate_and_save_draft` and protected its presence with `scripts/coverage-gate.test.ts`.
- Added enforced frontend coverage configuration and CI/release coverage requirements.
- Updated README, release evidence, install/readiness/manual docs, API navigation, and generated HTML to distinguish the locally proved candidate from the older GitHub-hosted v0.3.2 artifact.

Local evidence recorded in the repository:

- `docs/release-evidence/v0.3.2-local-isolated-package-report.md` binds the candidate commit, installer SHA256/size, isolated package checks, local model, receipts, and honest public-beta limits.
- `docs/release-evidence/v0.3.2.json` is the machine-readable local/hosted provenance contract.
- `docs/release-readiness.md` contains the normal and stable release-smoke commands plus the boundary that this evidence does not itself push, tag, upload an artifact, or deploy Pages.

Local verification already achieved on the candidate machine:

- `npm test -- --run`: 27 files / 241 tests passed.
- `npm run test:coverage`: passed at 62.29% statements, 68.94% branches, 57.34% functions, and 63.45% lines, above configured floors.
- `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, release-doc policy check, design-token check, and managed Playwright UI smoke passed.
- An installed NSIS package with an intentionally unreachable local-AI endpoint passed the fresh-profile/skip/onboarding/zero-source flow.
- A second isolated installed package with `phi4-mini:latest` passed live-source scan, linked evidence, draft persistence, and Workbench reload.

Known work and guardrails for the next coding pass:

1. Security/data-integrity: `publish`, subscriber export, issue-email export, and backup creation still accept raw renderer-supplied destinations. Reuse/extend the canonicalized destination validation already used by diagnostics. Add tests for valid allowed roots and rejected sibling, traversal, and symlink-escape paths. Do not silently classify this as fixed without a real regression test.
2. Exhaustive UI wiring: current Playwright evidence covers primary navigation, responsive views, first-run, and core editorial flow, but not every conditional button/link. If the requirement is literal exhaustive coverage, add a state-aware control/link manifest and receipt that records each safe action, expected-disabled action, native-dialog exclusion, or external-link landing result. Do not claim the present smoke is exhaustive.
3. Rust test-host limitation: on the previous Windows host, `cargo test --locked --lib -- --list` and the focused `registered_ipc_command_generates_and_reloads_linked_draft` test compiled but exited before discovery with `0xc0000139 (STATUS_ENTRYPOINT_NOT_FOUND)`. Diagnose the host DLL/runtime mismatch or use a green hosted CI run for the exact revision. The installed-package E2E proof is complementary, not a substitute for executable Rust tests.
4. Hosted-state boundary: the GitHub release still serves the older `ba49af4...` / `1D6E650...` artifact. Local docs deliberately say so. Do not replace that asset or claim it matches the newer local candidate without explicit owner approval. GitHub Pages deployment is manual for this repo; do not deploy it without explicit owner approval.
5. Keep audit artifacts (`gate-*`, `audit-*`, `.agent-runs`) local/ignored. Durable release-candidate evidence belongs in the tracked release-evidence docs, not in transient audit folders.

Required engineering discipline:

- Preserve unrelated work. Check `git status` before every edit and stage explicit files only.
- Read before changing; establish a relevant baseline; use test-first RED -> GREEN for logic/security fixes; never weaken existing tests to obtain green.
- Run narrow checks first, then full affected checks. For UI work, verify desktop/mobile behavior, console state, empty/error/loading states, keyboard and accessible names.
- Before any push, run a secrets scan, inspect `.gitignore`, run the full relevant suite, and update user-facing docs/CHANGELOG when behavior changes. Use a PR and merge only green required checks; do not bypass protections.

Suggested first task:

Implement the shared safe write-destination validator for publish/export/backup IPC commands with failing regression tests first. Then run focused Rust tests, frontend tests, coverage, the packaged walkthrough as appropriate, and a scoped audit. Keep the existing local candidate evidence separate from any later artifact/release decision.
```
