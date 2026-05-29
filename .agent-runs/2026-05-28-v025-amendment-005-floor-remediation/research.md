# Research Report — v0.2.5 Amendment 005 Floor Remediation

## Affected Modules
- `src-tauri/src/core/reproduction_tests.rs`: Implements selective platform-gate checks. Whitelist checks are to be updated to load whitelist data from the new `.agent-workflows/section2-auth.json` file.
- `.agent-workflows/section2-auth.json` (NEW file): The dedicated configuration file containing §2-AUTH whitelisted tests.
- `scripts/audit/mutations.json` (READ-ONLY): Checked for non-modification at the end of the run.

## Existing Patterns
- Platform-restricted unit tests (incompatible with Windows console-mode test runners due to Tauri dynamic link crashes) are skipped on Windows using standard `#[cfg_attr(target_os = "windows", ignore = "...")]` attributes.
- Inner test blocks execute only on non-Windows systems (`#[cfg(unix)]`), avoiding mock_app crashes.
- Verification tests in `reproduction_tests.rs` recursively parse test files for platform compile-out attributes and raise panics for any test using them without presence in the whitelist.

## Constraints from Antigravity.md
- **Task Cleanup**: Clean up background tasks before completing turns.
- **Order of Operations**: Branch from main, work in slices. Tag at rung close. Run tests and verify before merging.
- **Memory Precedence**: In active runs, chat gate keywords (`APPROVE` / `REVISE` / `REPLAN` / `BLOCK` / `VIEW`) are authoritative, overriding standard operator rules.

## Constraints from ADRs (System Architecture & Security Model)
- **Local Isolation**: Re-runs and integrations must co-exist on the local machine with security barriers (local Axum bound to `127.0.0.1:12053`, PIN-pairing, scope-locked paths).
- **Graceful Coexistence**: Sidecar port checks (`127.0.0.1:11434`) must gracefully skip spawning if the port is already occupied (coexisting with existing Ollama instances).

## Open Questions
- None. The scope of this amendment is explicitly restricted to closing findings F-5 and F-6.

## Checkpoint Research Verification

### 1. Examination of `reproduction_tests.rs` and Prior Remediation Bridge Files
- **Bridge File `2026-05-28T1507-antigravity-wb1-remediation-cfg-family.md`**: Shows the five compile-out syntaxes detected (cfg(unix), cfg(target_family = "unix"), cfg(any(target_os = "linux", target_os = "macos")), cfg(not(windows)), cfg(not(target_family = "windows"))).
- **Bridge File `2026-05-28T1507-antigravity-wb1-remediation-section2auth-enforcement.md`**: Outlines the initial selective whitelist check matching against `mutations.json`.
- **Bridge File `2026-05-28T1712-antigravity-wb1-remediation-cfg-family-whitelist-proof.md`**: Verifies that whitelisted tests using compile-out syntaxes do not trigger reproduction test panics (positive-direction proof).
- **Bridge File `2026-05-28T1712-antigravity-wb1-remediation-section2auth-decoupling.md`**: Demonstrates decoupling verification where the source is changed to a dedicated `section2-auth.json` list.

### 2. Brain Directory Amendments Verification
A scan of the brain directory `C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5` confirms the existence of the following `DIRECTIVE-v0*-AMENDMENT-<NNN>-*.md` files:
- `DIRECTIVE-v022-AMENDMENT-001-tauri-test-harness.md`
- `DIRECTIVE-v022-AMENDMENT-002-sidecar-test-harness.md`
- `DIRECTIVE-v025-AMENDMENT-004-sidecar-port-test.md`
- `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md`

*Note: No `DIRECTIVE-v0*-AMENDMENT-003-*.md` file exists in the brain directory.*

### 3. Test Authorization Mapping Table
The 8 platform-gated tests are verified against their authorizing directive/amendment files in the brain directory:

| # | Test Function Name | Authorizing Document File in Brain Directory | Status of Authorizing Document |
|---|---|---|---|
| 1 | `test_plain_language_rewrite_invokes_ollama` | `DIRECTIVE-v022-AMENDMENT-001-tauri-test-harness.md` | Valid and Exists |
| 2 | `test_daily_scan_command_does_not_panic_when_state_registered` | `DIRECTIVE-v022-AMENDMENT-001-tauri-test-harness.md` | Valid and Exists |
| 3 | `test_daily_scan_uses_settings_model_not_hardcoded` | `DIRECTIVE-v022-AMENDMENT-001-tauri-test-harness.md` | Valid and Exists |
| 4 | `test_ollama_sidecar_spawns_with_expected_pid_pattern` | `DIRECTIVE-v022-AMENDMENT-002-sidecar-test-harness.md` | Valid and Exists |
| 5 | `test_ollama_sidecar_terminates_cleanly_on_drop` | `DIRECTIVE-v022-AMENDMENT-002-sidecar-test-harness.md` | Valid and Exists |
| 6 | `test_pull_ollama_model_propagates_http_error` | `DIRECTIVE-v024-fix-all-28-plus-structural.md` (WE-3 Option A) | Valid and Exists |
| 7 | `test_cancel_ollama_pull_is_per_pull` | `DIRECTIVE-v024-fix-all-28-plus-structural.md` (WE-3 Option A) | Valid and Exists |
| 8 | `test_sidecar_skips_spawn_when_port_11434_occupied` | `DIRECTIVE-v025-AMENDMENT-004-sidecar-port-test.md` | Valid and Exists |

*Assertion Check:* Every test is associated with an existing, valid authorizing document in the brain directory. Tests 6 and 7, previously referenced under "Amendment 003" (which does not exist), are verified as authorized under section `WE-3` Option A of the base directive `DIRECTIVE-v024-fix-all-28-plus-structural.md`.

### 4. Verification of Other Tests
A global search of `src-tauri/src` for `cfg_attr`, `target_os`, and `target_family` confirms that **no other test functions** contain platform-specific compile-out or ignore annotations.

### 5. Documented Brain Directory File List
Files scanned in `C:\Users\scott\.gemini\antigravity\brain\0921da25-c18f-4fad-9ee3-f6ced44621f5`:
- `00-executive-audit.md`
- `DIRECTIVE-v021-hotpatch-lieproof.md`
- `DIRECTIVE-v022-AMENDMENT-001-tauri-test-harness.md`
- `DIRECTIVE-v022-AMENDMENT-002-sidecar-test-harness.md`
- `DIRECTIVE-v022-hotpatch-lieproof-2.md`
- `DIRECTIVE-v023-fix-all-37.md`
- `DIRECTIVE-v024-fix-all-28-plus-structural.md`
- `DIRECTIVE-v025-AMENDMENT-004-sidecar-port-test.md`
- `DIRECTIVE-v025-AMENDMENT-005-floor-remediation.md`
- `DIRECTIVE-v025-zero-zero-zero-zero-zero.md`
- `MASTER-PROMPT-civicnewspaper-v020-ship.md`
- `audit-lite-phase-4-2026-05-25.md`
- `audit-lite-phase-4-remediation-2026-05-26.md`
- `directive-phase-4-remediation.md`
- `directive.yaml`
- `implementation_plan.md`
- `task.md`
- `walkthrough.md`
- `macos_gatekeeper_warning_1779836738502.png`
- `windows_smartscreen_warning_1779836725063.png`
- Subdirectories: `.agents`, `.system_generated`, `audit-team-v021-claude`, `audit-team-v022-claude`, `audit-team-v023-claude`, `audit-team-v024-claude`, `scratch`, `stage-13-audit-team-claude`
