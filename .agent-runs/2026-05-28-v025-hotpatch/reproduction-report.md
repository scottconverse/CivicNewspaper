# Reproduction Report: v0.2.5-hotpatch

This report documents the reproduction phase of the `v0.2.5-hotpatch` run. It verifies that the 6 Major findings (M-1 to M-6) and 3 structural closures described in the v0.2.5 directive are present and fail as expected on the current codebase prior to applying hotpatch fixes.

## 1. Reproduction Setup & Test Suites

Two test suites were verified in the codebase to target all issues in Rust (backend) and TypeScript/React (frontend):
- **Rust Reproduction Suite:** Located at `src-tauri/src/core/reproduction_tests.rs` (registered in `src-tauri/src/core/mod.rs`).
- **Vitest Reproduction Suite:** Located at `src/reproduction.test.tsx`.

Both test suites contain assertion checks that explicitly verify the absence of the security issues, platform gates, grep-bait, and hardcoded values. If the codebase contains any of these violations, the assertions fail, thereby successfully reproducing the issues.

---

## 2. Rust Reproduction Test Failures (`cargo test`)

Executed inside `src-tauri/` via:
`cargo test core::reproduction_tests::tests`

### Execution Results
All 9 tests failed on target assertions:

```text
running 9 tests
test core::reproduction_tests::tests::reproduce_structural_closure_0_23_violations ... FAILED
test core::reproduction_tests::tests::reproduce_m6_walkthrough_mismatch ... FAILED
test core::reproduction_tests::tests::reproduce_m1_cfg_family_bypass ... FAILED
test core::reproduction_tests::tests::reproduce_m4_onboarding_wizard_comment ... FAILED
test core::reproduction_tests::tests::reproduce_m2_hardcoded_model_and_grep_exclusions ... FAILED
test core::reproduction_tests::tests::reproduce_structural_closure_0_22_violations ... FAILED
test core::reproduction_tests::tests::reproduce_structural_closure_0_21_violations ... FAILED
test core::reproduction_tests::tests::reproduce_m3_grep_bait_comments ... FAILED
test core::reproduction_tests::tests::reproduce_m5_readme_parentheticals ... FAILED
```

### Detailed Failure Outputs

#### M-1: Cfg-Family Bypass (`reproduce_m1_cfg_family_bypass`)
- **Assertion Fail Message:** `M-1 violation: test_sidecar_skips_spawn_when_port_11434_occupied wraps its body in #[cfg(unix)]`
- **Context:** The test function `test_sidecar_skips_spawn_when_port_11434_occupied` in `src-tauri/src/core/tests.rs` wraps its entire logic in `#[cfg(unix)]`, allowing it to compile as a no-op on Windows.

#### M-2: Hardcoded Models & Grep Exclusions (`reproduce_m2_hardcoded_model_and_grep_exclusions`)
- **Assertion Fail Message:** `M-2 violation: OnboardingWizard contains hardcoded 'gemma2:9b'`
- **Context:** Hardcoded references to `'gemma2:9b'` exist in `OnboardingWizard.tsx` and `useApp.ts`, and `grep-checks.sh` contains allowlist exemptions for these occurrences. Also, `known-bad-from-production-gemma2.ts` is missing.

#### M-3: Grep-Bait Comments in Daily Scan Test (`reproduce_m3_grep_bait_comments`)
- **Assertion Fail Message:** `M-3 violation: test contains grep-bait comment 'mockLlm expect: phi3:mini'`
- **Context:** `src/test_useapp_daily_scan_passes_settings_model.test.tsx` contains comments matching `mockLlm expect: phi3:mini` and others, instead of asserting the actual flow behavior.

#### M-4: Grep-Bait Comment in Wizard Skip Flow (`reproduce_m4_onboarding_wizard_comment`)
- **Assertion Fail Message:** `M-4 violation: OnboardingWizard contains grep-bait comment`
- **Context:** A non-functional verification target comment `// Skip: setStep(4) cancel_ollama_pull|cancelPull` is written inside `src/components/OnboardingWizard.tsx`.

#### M-5: README Tautological Parentheticals (`reproduce_m5_readme_parentheticals`)
- **Assertion Fail Message:** `M-5 violation: README contains tautological parenthetical '(src/components/OnboardingWizard.tsx)'`
- **Context:** The `README.md` file contains redundant tree paths in parentheses (e.g. `(src/components/OnboardingWizard.tsx)` and `(contains scripts/policy/ and scripts/audit/)`).

#### M-6: Walkthrough Metadata Fabrication (`reproduce_m6_walkthrough_mismatch`)
- **Assertion Fail Message:** `M-6 violation: Walkthrough contains hardcoded incorrect commit count '38 commits'`
- **Context:** The v0.2.4 walkthrough file hardcodes the commit count to `**38**`, which drifts from reality.

#### Structural Closure 1: No verification grep targets in output (`reproduce_structural_closure_0_21_violations`)
- **Assertion Fail Message:** `Structural §0.21 violation: Verification grep pattern is present in comments`
- **Context:** The literal regex pattern matches are found in comments in `OnboardingWizard.tsx`.

#### Structural Closure 2: Complete cfg gate parity (`reproduce_structural_closure_0_22_violations`)
- **Assertion Fail Message:** `Structural §0.22 violation: Test body is gated by unix platform gate instead of target_os ignore`
- **Context:** Checks that test bodies do not use `#[cfg(unix)]` wrapper scopes.

#### Structural Closure 3: Verbatim production line fixtures (`reproduce_structural_closure_0_23_violations`)
- **Assertion Fail Message:** `Structural §0.23 violation: Missing production-derived known-bad fixture`
- **Context:** Checks that `scripts/audit/fixtures/known-bad-from-production-gemma2.ts` exists.

---

## 3. Vitest Reproduction Test Failures (`npx vitest`)

Executed inside repo root via:
`npx vitest run src/reproduction.test.tsx`

### Execution Results
All 5 tests failed on target assertions:

```text
 ❯ src/reproduction.test.tsx (5 tests | 5 failed)
   × reproduce_m2_hardcoded_model_onboarding_wizard
   × reproduce_m2_hardcoded_model_use_app
   × reproduce_m4_onboarding_wizard_grep_bait_comment
   × reproduce_wmin_1_redundant_continue_button
   × reproduce_wnit_1_selectable_empty_option
```

### Detailed Failure Outputs

- **`reproduce_m2_hardcoded_model_onboarding_wizard`**:
  * **Failure**: Found `'gemma2:9b'` hardcoded as a fallback in `OnboardingWizard.tsx`.
- **`reproduce_m2_hardcoded_model_use_app`**:
  * **Failure**: Found `'gemma2:9b'` hardcoded as a fallback in `useApp.ts`.
- **`reproduce_m4_onboarding_wizard_grep_bait_comment`**:
  * **Failure**: Found comment `// Skip: setStep(4) cancel_ollama_pull|cancelPull` in `OnboardingWizard.tsx`.
- **`reproduce_wmin_1_redundant_continue_button`**:
  * **Failure**: Found redundant primary "Continue" button on Step 2 in reachable-no-models card while the footer also has a primary button.
- **`reproduce_wnit_1_selectable_empty_option`**:
  * **Failure**: Found selectable option `<option value="">-- Or pull a recommended model --</option>` inside the select dropdown.

---

## Conclusion
The reproduction suite successfully validates that all the described Majors, Minors, Nits, and Structural Closures have failing tests demonstrating the vulnerabilities/deficiencies on the current codebase. The reproduce stage is complete.
