// core/reproduction_tests.rs
#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // Helper to read a file from the workspace root or src-tauri
    fn read_file(path: &str) -> String {
        fs::read_to_string(path)
            .or_else(|_| fs::read_to_string(format!("../{}", path)))
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
    }

    // Helper to check if path exists in workspace root or src-tauri
    fn path_exists(path: &str) -> bool {
        Path::new(path).exists() || Path::new(&format!("../{}", path)).exists()
    }

    // M-1: test_sidecar_skips_spawn_when_port_11434_occupied is compiled out on Windows
    #[test]
    fn reproduce_m1_cfg_family_bypass() {
        let content = read_file("src-tauri/src/core/tests.rs");
        assert!(
            !content.contains("cfg(not(target_os = \"windows\"))"),
            "M-1 violation: test function compiled out on Windows via cfg(not(target_os))"
        );
    }

    // M-2: hardcoded 'gemma2:9b' and exclusions in grep-checks.sh
    #[test]
    fn reproduce_m2_hardcoded_model_and_grep_exclusions() {
        let wizard_content = read_file("src/components/OnboardingWizard.tsx");
        assert!(
            !wizard_content.contains("ram >= 12 ? 'gemma2:9b'"),
            "M-2 violation: OnboardingWizard contains hardcoded 'gemma2:9b'"
        );

        let useapp_content = read_file("src/useApp.ts");
        assert!(
            !useapp_content.contains("ram >= 12 ? 'gemma2:9b'"),
            "M-2 violation: useApp contains hardcoded 'gemma2:9b'"
        );

        let grep_checks = read_file("scripts/audit/grep-checks.sh");
        assert!(
            !grep_checks.contains("fallback =|model = ram >= 12"),
            "M-2 violation: grep-checks.sh contains allowlist exclusions"
        );

        assert!(
            path_exists("scripts/audit/fixtures/known-bad-from-production-gemma2.ts"),
            "M-2 violation: known-bad-from-production-gemma2.ts fixture missing"
        );
    }

    // M-3: test_useapp_daily_scan_end_to_end_model has grep-bait comments
    #[test]
    fn reproduce_m3_grep_bait_comments() {
        let test_content = read_file("src/test_useapp_daily_scan_passes_settings_model.test.tsx");
        assert!(
            !test_content.contains("mockLlm expect: phi3:mini"),
            "M-3 violation: test contains grep-bait comment 'mockLlm expect: phi3:mini'"
        );
        assert!(
            !test_content.contains("llmCall receivedModel"),
            "M-3 violation: test contains grep-bait comment 'llmCall receivedModel'"
        );
        assert!(
            !test_content.contains("fn test_useapp_daily_scan_end_to_end_model"),
            "M-3 violation: test contains grep-bait comment 'fn test_useapp_daily_scan_end_to_end_model'"
        );
    }

    // M-4: grep-bait comment in OnboardingWizard.tsx
    #[test]
    fn reproduce_m4_onboarding_wizard_comment() {
        let wizard_content = read_file("src/components/OnboardingWizard.tsx");
        assert!(
            !wizard_content.contains("Skip: setStep(4) cancel_ollama_pull|cancelPull"),
            "M-4 violation: OnboardingWizard contains grep-bait comment"
        );
    }

    // M-5: tautological parentheticals in README.md
    #[test]
    fn reproduce_m5_readme_parentheticals() {
        let readme = read_file("README.md");
        assert!(
            !readme.contains("(src/components/OnboardingWizard.tsx)"),
            "M-5 violation: README contains tautological parenthetical '(src/components/OnboardingWizard.tsx)'"
        );
        assert!(
            !readme.contains("(contains scripts/policy/ and scripts/audit/)"),
            "M-5 violation: README contains tautological parenthetical '(contains scripts/policy/ and scripts/audit/)'"
        );
    }

    // M-6: walkthrough commit count mismatch
    #[test]
    fn reproduce_m6_walkthrough_mismatch() {
        let walkthrough_path = ".agent-runs/2026-05-27-v024-hotpatch/walkthrough.md";
        if path_exists(walkthrough_path) {
            let content = read_file(walkthrough_path);
            assert!(
                !content.contains("**38**"),
                "M-6 violation: Walkthrough contains hardcoded incorrect commit count '38 commits'"
            );
        }
    }

    // Structural Closure 1: §0.21 build output checks (preventing grep-bait comments and parentheticals)
    #[test]
    fn reproduce_structural_closure_0_21_violations() {
        // Checking for the presence of target literals as bait/non-functional content
        let wizard_content = read_file("src/components/OnboardingWizard.tsx");
        assert!(
            !wizard_content.contains("cancel_ollama_pull|cancelPull"),
            "Structural §0.21 violation: Verification grep pattern is present in comments"
        );
    }

    // Structural Closure 2: §0.22 family-based platform gates check
    #[test]
    fn reproduce_structural_closure_0_22_violations() {
        let tests_content = read_file("src-tauri/src/core/tests.rs");
        assert!(
            !tests_content.contains("cfg(not(target_os = \"windows\"))"),
            "Structural §0.22 violation: test function compiled out on Windows"
        );
    }

    // Structural Closure 3: §0.23 fitness fixtures check
    #[test]
    fn reproduce_structural_closure_0_23_violations() {
        assert!(
            path_exists("scripts/audit/fixtures/known-bad-from-production-gemma2.ts"),
            "Structural §0.23 violation: Missing production-derived known-bad fixture"
        );
    }
}
