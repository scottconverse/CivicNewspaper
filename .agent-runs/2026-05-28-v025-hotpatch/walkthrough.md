# Walkthrough - v0.2.5 Hotpatch Release

This walkthrough documents the verified hotpatch changes applied on the `v0.2.5-hotpatch` branch to close the outstanding 18 audit findings.

## Changes Made

1. **Whitelisted platform-gated tests (Amendment 005)**: Relocated §2-AUTH whitelist to `.agent-workflows/section2-auth.json` containing the 8 authorized tests.
2. **Decoupled reproduction_tests.rs**: Fully decoupled the test suite from `mutations.json`.
3. **Occupied Port Sidecar skip (Amendment 004)**: Ensured occupied sidecar port test is gated correctly cross-platform.
4. **Onboarding wizard improvements**: Removed redundant Continue button, disabled empty selectable model placeholder.
5. **Version Bump**: Bumped to version `0.2.5` across `package.json`, `Cargo.toml`, and `tauri.conf.json`.
6. **CHANGELOG Postmortems**: Added postmortems for `[0.2.3]` and `[0.2.4] [NEVER TAGGED]`.

## Verification Results

### Automated Tests
- Vitest suite: 28 passed, 0 failed.
- Cargo suite: 35 passed, 0 failed, 8 ignored.
- TypeScript compiler and Rust formatting checks passed.
- Policy check script executed with exit code 0 (`POLICY: ALL CHECKS PASSED`).
