# Implementation Plan — v0.2.5 Amendment 005 Floor Remediation

This plan outlines the approach, file alterations, testing strategy, risks, and compliance hooks to resolve findings F-5 and F-6 under the narrow scope of Amendment 005.

## 1. Approach

The core objective is to decouple the §2-AUTH whitelist from `scripts/audit/mutations.json` (F-6) by relocating it to a dedicated file `.agent-workflows/section2-auth.json`, and ensuring the platform compile-out test suite (`reproduce_m1_cfg_family_bypass`) correctly recognizes whitelisted tests without panicking (F-5).

We will implement this using the following strategies:
- **Dedicated Configuration File Adapter**: We will create `.agent-workflows/section2-auth.json` containing the metadata for the eight platform-gated tests. This list is derived from Amendments 001, 002, 004, and base directive v0.2.4 (since no Amendment 003 exists). Citing nonexistent amendment numbers is avoided, in strict compliance with research checks.
- **Function-Level Verification Logic**: In `src-tauri/src/core/reproduction_tests.rs`, we will implement the parser method `verify_no_unauthorized_platform_gates`. This helper reads the test file line-by-line or uses a regular expression to locate test functions (`(?:async\s+)?fn\s+(test_[a-z0-9_]+)`). For each test function, it grabs its attributes and body, then matches them against a platform gate regex (`cfg(unix)`, `cfg(target_family = "unix")`, `cfg(any(...))`, `cfg(not(windows))`, `cfg(not(target_family = "windows"))`, and `target_os = "windows", ignore`).
- **Whitelist Verification Integration**: Any test matching a platform gate MUST be present in the `.agent-workflows/section2-auth.json` whitelist. If a platform gate is found on an unauthorized test, the reproduction test will panic.

## 2. Files to Create

### Module: `.agent-workflows`
- `.agent-workflows/section2-auth.json`: Holds a JSON array of the 8 authorized platform-gated tests, including fields for `test`, `file`, `amendment`, and `justification`.

## 3. Files to Modify

### Module: `src-tauri`
- `src-tauri/src/core/reproduction_tests.rs`:
  - **Location**: Implement/modify `verify_no_unauthorized_platform_gates` and update test assertions inside `reproduce_m1_cfg_family_bypass` and `reproduce_structural_closure_0_22_violations`.
  - **Reason**: Update the whitelist source to read from `.agent-workflows/section2-auth.json` instead of `scripts/audit/mutations.json`, and execute the multi-syntax compile-out check against `src-tauri/src/core/tests.rs` and `src-tauri/src/core/server_tests.rs`.
  - **Policy Check**: This path is allowed under `manifest.allowed_paths`.

## 4. Test Strategy

The test writer will run the following verification checks:
1. **Automated Verification Test**:
   - `cargo test --manifest-path src-tauri/Cargo.toml -- reproduce_m1_cfg_family_bypass` and `reproduce_structural_closure_0_22_violations` must pass, validating that the 8 authorized tests are properly recognized and do not trigger panics.
2. **Whitelist-Positive-Direction Proof (F-5 / WB-AMD5-1)**:
   - Temporarily apply all 5 compile-out syntaxes to the authorized tests:
     - Syntax 1: `#[cfg(unix)]`
     - Syntax 2: `#[cfg(target_family = "unix")]`
     - Syntax 3: `#[cfg(any(target_os = "linux", target_os = "macos"))]`
     - Syntax 4: `#[cfg(not(windows))]`
     - Syntax 5: `#[cfg(not(target_family = "windows"))]`
   - Verify `cargo test -- reproduce_m1_cfg_family_bypass` passes for all 5. Restore tests cleanly afterwards.
3. **Decoupling Verification Proof (F-6 / WB-AMD5-2)**:
   - Temporarily modify a test name inside `scripts/audit/mutations.json`.
   - Verify that `cargo test -- reproduce_m1_cfg_family_bypass` still passes, proving that the verification logic is fully decoupled and does not rely on `mutations.json` contents. Restore `mutations.json` to its pristine state.

## 5. Risks

| Risk Description | Severity | Specific Code Mitigation |
|---|---|---|
| **Platform Gate Bypass via Syntax Evasion** (Regex does not match slight spacing or capitalization changes in compile-out attributes) | **High** | We utilize a case-insensitive `(?i)` regex pattern matching all variations with arbitrary spacing `\s*` and nested characters `[^)]*` inside the attributes and function body scopes. |
| **Path Resolution Failure in Test Runners** (Tests fail depending on where the `cargo test` command is run from: workspace root or `src-tauri/` directory) | **Medium** | The `read_file` utility function checks path existence at both the relative path and `../<path>` to resolve correct locations. |
| **Incorrect / Missing Test Authorization** (An unauthorized compile-out test is introduced, or an authorized one is omitted) | **Medium** | The `section2-auth.json` whitelist strictly lists the 8 authorized tests and maps them to verified, existing amendment files, which is checked during run verification. |

## 6. Layered Audit Hooks

- **Per-commit careful-coding**: Work items WB-AMD5-1 and WB-AMD5-2 will be developed in separate, distinct commits. `git diff` will be ran and analyzed before each commit.
- **Per-checkpoint sanity sweep**: Upon completing each work item, the executor will generate detailed, timestamped HR-1 compliant bridge files:
  - `<ts>-antigravity-amd5-wb1-whitelist-positive-proof.md`
  - `<ts>-antigravity-amd5-wb2-section2auth-decoupling.md`
- **Per-rung audit-lite**: At the conclusion of the run, the walkthrough file `.agent-runs/2026-05-28-v025-amendment-005-floor-remediation/walkthrough.md` will list all commits, SHAs, modified files, verification results, and §0-clause compliance attestations.

## 7. Definition of Done

The definition of done is defined as:
> **DoD**: The whitelist is relocated to `.agent-workflows/section2-auth.json` with all eight authorized tests configured, all tests pass, and `mutations.json` remains unmodified.

### Verifiable Deliverables:
1. **Configuration**: `.agent-workflows/section2-auth.json` contains exactly 8 entries matching the authorized platform-gated tests.
2. **Untouched mutations.json**: `git diff scripts/audit/mutations.json` returns empty.
3. **Rust Tests Pass**: `cargo test --manifest-path src-tauri/Cargo.toml` executes and passes all tests successfully.
4. **Source Cleanliness**: `grep -r "mutations.json" src-tauri/src/core/reproduction_tests.rs` returns zero occurrences.
5. **Logs & Walkthrough**: Walks through commits and registers compliance logs.
