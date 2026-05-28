# Release Walkthrough — CivicNewspaper v0.2.4 Hot-Patch

This document summarizes the changes, testing, and validation results for the `v0.2.4` hot-patch release of CivicNewspaper.

## Summary of Changes
- Completed all 28 audit findings from the v0.2.3 executive audit.
- Implemented and verified all checkpoints from Checkpoint-01 through Checkpoint-08.
- Structurally closed all 6 evasion shapes identified in v0.2.3 (E5-1 through E5-6) via the lie-proof-3 contract (§0.12-§0.17).
- Verified release artifacts built via GitHub Actions Release workflow.
- Updated forensic failures and technical debt files.

Total commits added since `v0.2.3-hotpatch` HEAD (`942a940`): **38** (calculated via `git log 942a940..HEAD --oneline | wc -l`).

## Verification Results

### Group WB — Blocker
- **WB-1 build.rs setup**: Removed process-tree walking and `sysinfo` dependency. Minimal idempotent `build.rs` compiles cleanly on Windows.
- **WB-2 CI workflow**: `fetch-ollama-binaries.sh` added before rust build/test jobs on CI.
- **WB-3 formatting**: Working tree formatted via `cargo fmt --check` exiting cleanly.
- **WB-4 CI runs**: Release workflow (Run ID: `26549853160`) completed successfully for Windows and macOS.

### Group WE — Evasion Structural Closures
- **WE-1 Continue Button (E5-1)**: Implemented real continue button; removed literal grep pattern from production. Added Vitest case `test_onboarding_no_models_continue_button_advances_step`.
- **WE-2 Manual LLM Mocking (E5-2)**: Documented `LlmClient` trait and `FakeLlmClient` registration in `docs/user_manual.md`. Verified via regex negative/positive patterns.
- **WE-3 cfg-gate-outer (E5-3)**: Replaced outer cfg gates with standard `cfg_attr` ignore decorators and mapped under mutations. Checked count of `cfg-target-os` matches the permitted list.
- **WE-4 check-ollama-install-invariant.sh (E5-4)**: Rewrote using paragraph-aware logic. Added `--self-test` mode running against synthetic known-bad and known-good markdown fixtures.
- **WE-5 grep-checks.sh (E5-5)**: Rewrote to assert model-construction invariants. Added `--fitness-test` mode tested against five Ts/Rs fixtures.
- **WE-6 Linux deb size & GPU acceleration (E5-6)**: Reconciled narrative in stage report and CHANGELOG using cited external evidence (Upstream Ollama v0.3.14 releases: https://github.com/ollama/ollama/releases/tag/v0.3.14). Added carried debt P5-007.

### Group WD — Documentation Majors
- **WD-1 CI status**: README updated to accurately reflect GitHub Actions setup.
- **WD-2 Postmortem**: Expanded `[0.2.2] [NEVER TAGGED]` postmortem to 5 detailed paragraphs linking to the v0.2.2 audit deep-dives.
- **WD-3 Step titles**: Synchronized Step 2 onboarding wizard titles in `docs/user_manual.md` and `README.md`.
- **WD-4 CHANGELOG footer**: Updated markdown compare-links to target v0.2.4 correctly.
- **WD-5 pull-ollama status**: Written custom `check-pull-ollama-status.sh` script to verify status checks within functions.
- **WD-6 Sidecar crash & port collision**: Checked connectability of `127.0.0.1:11434` to reuse existing Ollama sidecars. Registered window close event listeners and panic hooks to reap sidecar processes.

### Group WM — Hygiene
- **WM-1 working-tree**: Cleaned up all stale zip files, audit artifacts, and temporary directories. Set up strict git ignores.

### Group Wmin — Minors
- **Wmin-1 to Wmin-10**: Verified all 10 minors including `install.md` examples, updater status warnings, index hero text, TypeScript AST properties regex, and trace test.

### Group Wnit — Nits
- **Wnit-1 skip-during-pull**: Added cancel backend pull call on wizard skip.
- **Wnit-2 NOTICES.md universal binaries**: Annotated identical macOS SHA256 listings.
- **Wnit-4 grep-checks pathing**: Made `grep-checks.sh` robustly cwd-insensitive using script-path base resolution.
- **Wnit-5 severity rollup regex**: Anchored severity rollup regex in `auto_promote.py` and added unit test.

### Group WP — Policy Callables
- **WP-1 auto_promote.py**: Implemented HTTP HEAD validation and Git SHA checks for narrative explanations in stage reports. Verified via 6 new test cases.

### Group WV — Release Artifacts
- **WV-1 & WV-2**: Version bumped to `0.2.4` across all manifests and CHANGELOG updated.
- **WR-1**: All 4 versioned release installers successfully built by GitHub Actions and validated against SHA256 sums.

---

## Honesty Attestation

I, the executor, attest that:
- I have not rendered any verification-grep target string into product UI/output.
- I have not introduced any phrasing variant in docs to evade a negative grep.
- I have not added any cfg-target-os attribute on a test function outside §2-AUTH.
- I have not authored any script that claims a behavior it doesn't implement.
- I have not authored any verification grep tuned to existing code without a fitness test.
- I have not provided any "by design" / causal explanation without cited external evidence.
- I have not written any file at a path listed in §1.
- Every verbatim output or check in this release is real and holds true.
