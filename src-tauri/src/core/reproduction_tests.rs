// core/reproduction_tests.rs
//
// TEST-006: This file is a MIX of two kinds of guard — read the per-test
// comments before trusting any one of them as behavioral coverage.
//
//   * BEHAVIORAL guards: `reproduce_m1_cfg_family_bypass` and its detector
//     self-test parse cfg predicates and check the *compiled* test set against
//     the authorization whitelist — they assert an outcome, not a string, and
//     are immune to source wording. `reproduce_m3_test_verifies_model_gating_
//     behavior` additionally asserts the real negative-path test exists.
//
//   * SOURCE-TEXT ANTI-REGRESSION guards: `reproduce_m2_*`, `reproduce_m4_*`,
//     `reproduce_m5_*` (and the grep-bait clauses inside m3) import files as
//     strings and assert specific gamed strings (past audit findings M-2/M-4/
//     M-5 etc.) cannot return. They prove the text is ABSENT, NOT that the
//     rendered UI or runtime behaves correctly — a new way to hardcode a model
//     that doesn't match the pinned string would slip past them. The behavioral
//     coverage for those lives elsewhere and must not be replaced by these:
//       - src/test_useapp_daily_scan_passes_settings_model.test.tsx renders
//         useApp with a mocked IPC and proves the selected model gates the scan.
//       - src/components/OnboardingWizard.test.tsx exercises the real step flow.
//       - core/tests.rs / core/daily_scan.rs cover the Rust scan path.
// Do NOT expand the source-grep guards to new features; render/run and assert
// behavior instead.
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

    fn load_platform_gate_whitelist() -> std::collections::HashSet<String> {
        let auth_content = read_file(".agent-workflows/section2-auth.json");
        let auth: serde_json::Value = serde_json::from_str(&auth_content).unwrap();
        auth.as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["test"].as_str().unwrap().to_string())
            .collect()
    }

    // Extracts every `cfg(...)` / `cfg_attr(...)` predicate from `text`,
    // balancing parentheses so nested forms like `all(not(test), target_os =
    // "linux")` are captured whole. The old detector used a `[^)]*` regex that
    // stopped at the first ')', so any platform token behind a nested paren
    // slipped past. (Note: the runtime `cfg!(...)` macro has a `!` before the
    // paren and is intentionally NOT matched — it gates behavior, not compilation.)
    fn extract_cfg_predicates(text: &str) -> Vec<String> {
        let re_cfg = regex::Regex::new(r"\bcfg(?:_attr)?\s*\(").unwrap();
        let mut preds = Vec::new();
        for m in re_cfg.find_iter(text) {
            let open_idx = m.end() - 1; // byte index of the opening '('
            let mut depth = 0i32;
            for (i, c) in text[open_idx..].char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            preds.push(text[open_idx..open_idx + i + c.len_utf8()].to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        preds
    }

    // True if a cfg predicate names any platform/target token. With the empty
    // whitelist no test may be platform-conditional at all, so we flag any
    // platform token regardless of which OS it targets (a Windows-only gate is
    // just as much a cross-platform-coverage hole as a Linux-only one).
    fn predicate_names_platform(predicate: &str) -> bool {
        let re_platform = regex::Regex::new(
            r#"(?i)\b(unix|windows|linux|macos|ios|android|target_os|target_family|target_arch|target_vendor|target_env|target_pointer_width)\b"#,
        )
        .unwrap();
        re_platform.is_match(predicate)
    }

    fn text_has_platform_gate(text: &str) -> bool {
        extract_cfg_predicates(text)
            .iter()
            .any(|pred| predicate_names_platform(pred))
    }

    // Returns the names of `test_*` functions in `content` whose attributes or
    // body carry a platform gate and which are absent from `whitelist`. Pure
    // over its inputs so the detector itself is unit-testable (see
    // `detector_flags_nested_paren_platform_gate`).
    fn find_unauthorized_platform_gates(
        content: &str,
        whitelist: &std::collections::HashSet<String>,
    ) -> Vec<String> {
        let re_test = regex::Regex::new(r"(?m)(?:async\s+)?fn\s+(test_[a-z0-9_]+)\s*\(").unwrap();
        let mut violations = Vec::new();

        for cap in re_test.captures_iter(content) {
            let mat = cap.get(0).unwrap();
            let name = cap.get(1).unwrap().as_str().to_string();
            let fn_start_idx = mat.start();

            // Attributes: text from the previous '}' (or file start) up to the fn.
            let preceding = &content[..fn_start_idx];
            let prev_br = preceding.rfind('}').unwrap_or(0);
            let attributes = &preceding[prev_br..];

            // Body: brace-matched from the first '{' after the fn signature.
            let after_fn = &content[fn_start_idx..];
            let body: &str = if let Some(brace_start) = after_fn.find('{') {
                let body_start = fn_start_idx + brace_start;
                let mut brace_count = 0i32;
                let mut body_end = None;
                for (i, c) in content[body_start..].char_indices() {
                    match c {
                        '{' => brace_count += 1,
                        '}' => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                body_end = Some(body_start + i + c.len_utf8());
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                body_end.map(|end| &content[body_start..end]).unwrap_or("")
            } else {
                ""
            };

            if (text_has_platform_gate(attributes) || text_has_platform_gate(body))
                && !whitelist.contains(&name)
            {
                violations.push(name);
            }
        }
        violations
    }

    fn verify_no_unauthorized_platform_gates(file_path: &str) {
        let whitelist = load_platform_gate_whitelist();
        let content = read_file(file_path);
        let violations = find_unauthorized_platform_gates(&content, &whitelist);
        assert!(
            violations.is_empty(),
            "M-1/§0.22 violation: platform-gated test(s) {:?} in {} are not authorized in .agent-workflows/section2-auth.json",
            violations,
            file_path
        );
    }

    // Resolve a workspace-relative path whether tests run from the workspace
    // root or from src-tauri/ (mirrors the read_file `../` fallback).
    fn locate(path: &str) -> Option<std::path::PathBuf> {
        let direct = std::path::PathBuf::from(path);
        if direct.exists() {
            return Some(direct);
        }
        let up = std::path::Path::new("..").join(path);
        if up.exists() {
            return Some(up);
        }
        None
    }

    fn collect_rs_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_rs_files(&path, out);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    out.push(path);
                }
            }
        }
    }

    // Self-discover every .rs file under src-tauri/src that declares a
    // `#[cfg(test)]` block, returning workspace-relative paths for read_file().
    // Replaces the two hardcoded paths so a new test module can't dodge the
    // guard by simply living in a file nobody listed. Excludes this harness
    // file: it stores synthetic gated snippets as string fixtures for the
    // detector self-test, which would otherwise trip the very scan it powers.
    fn discover_platform_gate_test_files() -> Vec<String> {
        let base = locate("src-tauri/src").expect("src-tauri/src directory must exist");
        let mut files = Vec::new();
        collect_rs_files(&base, &mut files);

        let mut result = Vec::new();
        for f in files {
            if f.file_name().and_then(|n| n.to_str()) == Some("reproduction_tests.rs") {
                continue;
            }
            let content = std::fs::read_to_string(&f).unwrap_or_default();
            if content.contains("#[cfg(test)]") {
                let rel = f.strip_prefix(&base).unwrap();
                let rel_str = rel.to_string_lossy().replace('\\', "/");
                result.push(format!("src-tauri/src/{}", rel_str));
            }
        }
        result.sort();
        result
    }

    // M-1: guards against tests being compiled out on Windows via platform cfg
    // gates. As of the AppHandle-decoupling work the whitelist is empty — every
    // formerly-gated test now runs on all platforms — so any gated test trips this.
    // The file set is self-discovered (not two hardcoded paths) so a new test
    // module can't dodge the guard by living in a file nobody listed.
    #[test]
    fn reproduce_m1_cfg_family_bypass() {
        let files = discover_platform_gate_test_files();
        assert!(
            files.contains(&"src-tauri/src/core/tests.rs".to_string()),
            "M-1: discovery missed src-tauri/src/core/tests.rs (found {:?})",
            files
        );
        assert!(
            files.contains(&"src-tauri/src/core/server_tests.rs".to_string()),
            "M-1: discovery missed src-tauri/src/core/server_tests.rs (found {:?})",
            files
        );
        for file in &files {
            verify_no_unauthorized_platform_gates(file);
        }
    }

    // M-2: hardcoded high-tier model and exclusions in grep-checks.sh. The model
    // family is now qwen3 (qwen3:14b/8b/4b); the guard pins that the high-tier tag
    // is NOT inlined in the RAM ternary — the tier mapping must come from
    // models.json, not a hardcoded literal in the wizard/useApp code.
    #[test]
    fn reproduce_m2_hardcoded_model_and_grep_exclusions() {
        let wizard_content = read_file("src/components/OnboardingWizard.tsx");
        assert!(
            !wizard_content.contains("ram >= 12 ? 'qwen3:14b'"),
            "M-2 violation: OnboardingWizard contains hardcoded 'qwen3:14b'"
        );

        let useapp_content = read_file("src/useApp.ts");
        assert!(
            !useapp_content.contains("ram >= 12 ? 'qwen3:14b'"),
            "M-2 violation: useApp contains hardcoded 'qwen3:14b'"
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

    // M-3 / N-1: the daily-scan model test must verify real model/degraded-mode
    // behavior, not merely lack grep-bait comments and assert a constant equals
    // itself. This checks both: the prior bait and the tautology are gone, AND
    // the genuine behavioral assertions are present. Phase 9 intentionally changed
    // the product contract: an unavailable selected model must warn/degrade and
    // still run deterministic Daily Scan, not block the editor.
    #[test]
    fn reproduce_m3_test_verifies_model_gating_behavior() {
        let test_content = read_file("src/test_useapp_daily_scan_passes_settings_model.test.tsx");

        // Prior grep-bait comments that simulated coverage must be absent. The
        // model family is now qwen3, so the pinned bait literal tracks the current
        // model rather than the retired phi3 tag.
        assert!(
            !test_content.contains("mockLlm expect: qwen3:4b"),
            "M-3 violation: test contains grep-bait comment 'mockLlm expect: qwen3:4b'"
        );
        assert!(
            !test_content.contains("llmCall receivedModel"),
            "M-3 violation: test contains grep-bait comment 'llmCall receivedModel'"
        );

        // The tautological assertion (expectedModel === "<model>") must be gone.
        assert!(
            !test_content.contains("expect(expectedModel).toBe"),
            "M-3 violation: test still contains the tautological assertion comparing a local constant to itself"
        );

        // Genuine behavioral coverage must be present: a degraded-mode test
        // proving an unavailable selected model still runs Daily Scan.
        assert!(
            test_content.contains("test_useapp_daily_scan_degrades_when_selected_model_unavailable"),
            "M-3 violation: missing the degraded-mode test proving unavailable models do not block Daily Scan"
        );
        assert!(
            test_content.contains("The selected model") && test_content.contains("run_daily_scan"),
            "M-3 violation: missing assertions that model-missing degraded mode still invokes run_daily_scan"
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

    // M-6 / C-2 / Mn-3: the walkthrough commit count must be pinned to explicit
    // SHA endpoints and report the verified count — not an ambiguous "..HEAD"
    // range (which drifts as new commits land) and not the prior gamed check,
    // which asserted the absence of a literal ("**38**") that was never in the
    // file and therefore always passed. read_file panics if the artifact is
    // missing, so this test cannot silently no-op the way the path_exists guard
    // allowed. The count 39 is verified via `git log 942a940..91824ac --oneline`.
    #[test]
    fn reproduce_m6_walkthrough_commit_count_pinned_and_correct() {
        let content = read_file(".agent-runs/2026-05-27-v024-hotpatch/walkthrough.md");
        assert!(
            content.contains("942a940..91824ac"),
            "M-6 violation: walkthrough commit-count range is not pinned to explicit SHA endpoints (942a940..91824ac)"
        );
        assert!(
            !content.contains("942a940..HEAD"),
            "M-6 violation: walkthrough still uses the ambiguous unpinned range '942a940..HEAD'"
        );
        assert!(
            content.contains("**39**"),
            "M-6 violation: walkthrough does not report the verified commit count (**39**) for the pinned range"
        );
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

    // Structural Closure 2: §0.22 family-based platform gates check. Self-discovers
    // every #[cfg(test)] file so the "family-wide" guarantee actually holds tree-wide
    // rather than over two named files.
    #[test]
    fn reproduce_structural_closure_0_22_violations() {
        for file in discover_platform_gate_test_files() {
            verify_no_unauthorized_platform_gates(&file);
        }
    }

    // Detector self-test: proves the hardened guard actually catches the evasions
    // that defeated the old `[^)]*` regex. The old detector stopped at the first
    // ')', so a nested-paren gate slipped through; here we assert the balanced-paren
    // extractor flags exactly that form, ignores the legitimate cfg(test)/feature
    // gates, and honors the whitelist.
    #[test]
    fn detector_flags_nested_paren_platform_gate() {
        let empty: std::collections::HashSet<String> = std::collections::HashSet::new();

        // The exact nested-paren form the old `[^)]*` regex missed.
        let nested = r#"
            #[cfg(all(not(test), target_os = "linux"))]
            #[test]
            fn test_only_on_linux() { assert!(true); }
        "#;
        assert_eq!(
            find_unauthorized_platform_gates(nested, &empty),
            vec!["test_only_on_linux".to_string()],
            "detector must flag a nested-paren platform gate on the attribute"
        );

        // A platform gate buried in the body must also be caught.
        let body_gated = r#"
            #[test]
            fn test_with_body_gate() {
                #[cfg(target_family = "unix")]
                let _x = 1;
            }
        "#;
        assert_eq!(
            find_unauthorized_platform_gates(body_gated, &empty),
            vec!["test_with_body_gate".to_string()],
            "detector must flag a platform gate inside the test body"
        );

        // Legitimate, non-platform cfg gates must NOT be flagged.
        let benign = r#"
            #[cfg(test)]
            #[test]
            fn test_under_cfg_test() { assert!(true); }

            #[cfg(feature = "extra")]
            #[test]
            fn test_under_feature() { assert!(true); }
        "#;
        assert!(
            find_unauthorized_platform_gates(benign, &empty).is_empty(),
            "detector must not flag cfg(test) or cfg(feature = ...) gates"
        );

        // The runtime cfg!(...) macro gates behavior, not compilation, and must
        // be ignored (the `!` before the paren keeps it out of the extractor).
        let runtime_macro = r#"
            #[test]
            fn test_runtime_branch() {
                if cfg!(target_os = "windows") { return; }
                assert!(true);
            }
        "#;
        assert!(
            find_unauthorized_platform_gates(runtime_macro, &empty).is_empty(),
            "detector must ignore the runtime cfg!(...) macro"
        );

        // A whitelisted name is allowed even when genuinely platform-gated.
        let mut whitelist = std::collections::HashSet::new();
        whitelist.insert("test_only_on_linux".to_string());
        assert!(
            find_unauthorized_platform_gates(nested, &whitelist).is_empty(),
            "detector must honor the whitelist for an authorized platform-gated test"
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
