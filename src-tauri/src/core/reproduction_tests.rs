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

    fn verify_no_unauthorized_platform_gates(file_path: &str) {
        let auth_content = read_file(".agent-workflows/section2-auth.json");
        let auth: serde_json::Value = serde_json::from_str(&auth_content).unwrap();
        let whitelist: std::collections::HashSet<String> = auth
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["test"].as_str().unwrap().to_string())
            .collect();

        let content = read_file(file_path);
        let re_test = regex::Regex::new(r"(?m)(?:async\s+)?fn\s+(test_[a-z0-9_]+)\s*\(").unwrap();
        let re_gate = regex::Regex::new(r#"(?i)cfg(_attr)?\s*\(\s*(?:[^)]*unix|[^)]*linux|[^)]*macos|[^)]*not\s*\(\s*windows|[^)]*not\s*\(\s*target_os\s*=\s*"windows"|[^)]*not\s*\(\s*target_family\s*=\s*"windows"|target_os\s*=\s*"windows"\s*,\s*ignore)"#).unwrap();

        for cap in re_test.captures_iter(&content) {
            let mat = cap.get(0).unwrap();
            let name = cap.get(1).unwrap().as_str().to_string();
            let fn_start_idx = mat.start();

            // Find attributes (preceding text up to previous '}' or start of file)
            let preceding = &content[..fn_start_idx];
            let prev_br = preceding.rfind('}').unwrap_or(0);
            let attributes = preceding[prev_br..].to_string();

            // Find body starting from first '{' after fn_start_idx
            let after_fn = &content[fn_start_idx..];
            if let Some(brace_start) = after_fn.find('{') {
                let body_start = fn_start_idx + brace_start;
                let mut brace_count = 0;
                let mut body_end = None;
                for (i, c) in content[body_start..].char_indices() {
                    if c == '{' {
                        brace_count += 1;
                    } else if c == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            body_end = Some(body_start + i + 1);
                            break;
                        }
                    }
                }
                if let Some(end) = body_end {
                    let body = content[body_start..end].to_string();

                    let has_gate_in_attributes = re_gate.is_match(&attributes);
                    let has_gate_in_body = re_gate.is_match(&body);

                    if has_gate_in_attributes || has_gate_in_body {
                        assert!(
                            whitelist.contains(&name),
                            "M-1/§0.22 violation: Test function '{}' in {} contains unauthorized platform gate (attributes: '{}', body: '{}')",
                            name,
                            file_path,
                            attributes.trim(),
                            body.trim()
                        );
                    }
                }
            }
        }
    }

    // M-1: test_sidecar_skips_spawn_when_port_11434_occupied is compiled out on Windows
    #[test]
    fn reproduce_m1_cfg_family_bypass() {
        verify_no_unauthorized_platform_gates("src-tauri/src/core/tests.rs");
        verify_no_unauthorized_platform_gates("src-tauri/src/core/server_tests.rs");
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
        verify_no_unauthorized_platform_gates("src-tauri/src/core/tests.rs");
        verify_no_unauthorized_platform_gates("src-tauri/src/core/server_tests.rs");
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
