// src/components/OnboardingWizard.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { OnboardingWizard } from "./OnboardingWizard";

describe("OnboardingWizard Component Tests", () => {
  test("onboarding wizard step 1 renders, Next button advances to step 2, step counter updates", () => {
    const handleComplete = vi.fn();

    render(
      <OnboardingWizard
        ollamaOnline={true}
        systemRam={16}
        onComplete={handleComplete}
      />
    );

    // Assert step 1 renders correctly
    expect(screen.getByText("Step 1 of 6")).toBeInTheDocument();
    expect(screen.getByText("Identity")).toBeInTheDocument();
    expect(screen.getByLabelText(/Publication Name/i)).toBeInTheDocument();

    // Click Next to advance to Step 2
    const nextBtn = screen.getByRole("button", { name: /next/i });
    fireEvent.click(nextBtn);

    // Assert step 2 renders correctly and counter updates
    expect(screen.getByText("Step 2 of 6")).toBeInTheDocument();
    expect(screen.getByText("Ollama")).toBeInTheDocument();
    expect(screen.queryByLabelText(/Publication Name/i)).not.toBeInTheDocument();
  });
});
