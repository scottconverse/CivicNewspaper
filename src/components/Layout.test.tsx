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
    expect(screen.getByText("Dark Signals")).toBeInTheDocument();
    expect(screen.getByText("Verification")).toBeInTheDocument();
    expect(screen.getByText("Sources")).toBeInTheDocument();

    // Click tabs and verify callback
    const sourcesBtn = screen.getByText("Sources");
    fireEvent.click(sourcesBtn);
    expect(handleTabChange).toHaveBeenCalledWith("sources");

    const onboardingBtn = screen.getByText("AI Model");
    fireEvent.click(onboardingBtn);
    expect(handleTabChange).toHaveBeenCalledWith("onboarding");

    const publishBtn = screen.getByText("Publishing");
    fireEvent.click(publishBtn);
    expect(handleTabChange).toHaveBeenCalledWith("publish");
  });

  test("native pointer and keyboard fallbacks navigate tabs", () => {
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

    fireEvent.pointerDown(screen.getByRole("button", { name: "Daily Scan" }));
    expect(handleTabChange).toHaveBeenCalledWith("dailyScan");

    fireEvent.keyDown(document, { key: "6", altKey: true });
    expect(handleTabChange).toHaveBeenCalledWith("sources");

    fireEvent.keyDown(document, { key: "8", ctrlKey: true });
    expect(handleTabChange).toHaveBeenCalledWith("publish");
  });

  test("does not claim local AI is ready when no model is selected", () => {
    render(
      <Layout
        activeTab="queue"
        onTabChange={vi.fn()}
        ollamaOnline={true}
        selectedDraft={null}
        modelLabel="No model selected"
      >
        <div>Test Child</div>
      </Layout>
    );

    expect(screen.getByText("Choose an AI model")).toBeInTheDocument();
    expect(screen.getByText("Choose an AI model").closest(".sidebar-footer")).toHaveClass("ai-status-needs-model");
    expect(screen.getByText("Choose an AI model").closest(".ollama-status-indicator")).toHaveClass("needs-model");
    expect(screen.queryByText("Local AI ready")).not.toBeInTheDocument();
  });

  test("shows limited mode after the user intentionally skips AI setup", () => {
    render(
      <Layout
        activeTab="queue"
        onTabChange={vi.fn()}
        ollamaOnline={false}
        selectedDraft={null}
        modelLabel="No model selected"
        aiSetupSkipped={true}
      >
        <div>Test Child</div>
      </Layout>
    );

    expect(screen.getByText("AI limited mode")).toBeInTheDocument();
    expect(screen.getByText("AI limited mode").closest(".sidebar-footer")).toHaveClass("ai-status-needs-model");
    expect(screen.queryByText("Local AI offline")).not.toBeInTheDocument();
  });
});
