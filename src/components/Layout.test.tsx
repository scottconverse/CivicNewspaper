// src/components/Layout.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { Layout } from "./Layout";

describe("Layout Component Tests", () => {
  test("renders navigation tabs and triggers onTabChange", () => {
    const handleTabChange = vi.fn();
    
    render(
      <Layout
        activeTab="queue"
        onTabChange={handleTabChange}
        ollamaOnline={true}
        selectedDraft={null}
      >
        <div data-testid="children-content">Test Child</div>
      </Layout>
    );

    // Verify children and basic layouts render
    expect(screen.getByTestId("children-content")).toBeInTheDocument();
    expect(screen.getByText("Story Queue")).toBeInTheDocument();
    expect(screen.getByText("Sources Setup")).toBeInTheDocument();

    // Click tabs and verify callback
    const sourcesBtn = screen.getByText("Sources Setup");
    fireEvent.click(sourcesBtn);
    expect(handleTabChange).toHaveBeenCalledWith("sources");

    const onboardingBtn = screen.getByText("Ollama Wizard");
    fireEvent.click(onboardingBtn);
    expect(handleTabChange).toHaveBeenCalledWith("onboarding");
  });
});
