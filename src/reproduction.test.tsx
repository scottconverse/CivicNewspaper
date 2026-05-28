// src/reproduction.test.tsx
import { describe, test, expect } from "vitest";
import OnboardingWizardText from "./components/OnboardingWizard.tsx?raw";
import useAppText from "./useApp.ts?raw";

describe("Frontend Reproduction Tests", () => {
  // M-2: OnboardingWizard and useApp contain hardcoded 'gemma2:9b'
  test("reproduce_m2_hardcoded_model_onboarding_wizard", () => {
    expect(OnboardingWizardText).not.toContain("ram >= 12 ? 'gemma2:9b'");
  });

  test("reproduce_m2_hardcoded_model_use_app", () => {
    expect(useAppText).not.toContain("ram >= 12 ? 'gemma2:9b'");
  });

  // M-4: OnboardingWizard contains grep-bait comment
  test("reproduce_m4_onboarding_wizard_grep_bait_comment", () => {
    expect(OnboardingWizardText).not.toContain("Skip: setStep(4) cancel_ollama_pull|cancelPull");
  });

  // WMin-1: Redundant primary Continue button on Step 2 in reachable-no-models card
  test("reproduce_wmin_1_redundant_continue_button", () => {
    // Current codebase has both a button on the card and in the footer
    const hasContinueInCard = /<button[^>]*>[\s\n]*Continue[\s\n]*<\/button>/i.test(OnboardingWizardText);
    expect(hasContinueInCard).toBe(false);
  });

  // WNit-1: Option for Recommended Model select empty-value option
  test("reproduce_wnit_1_selectable_empty_option", () => {
    // Current codebase has: <option value="">-- Or pull a recommended model --</option>
    // Which is selectable and sets model to empty string, breaking step-3 pull.
    // It should be disabled or hidden.
    expect(OnboardingWizardText).not.toContain('<option value="">-- Or pull a recommended model --</option>');
  });
});

