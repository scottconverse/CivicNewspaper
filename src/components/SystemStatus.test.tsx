// src/components/SystemStatus.test.tsx
import { render, screen } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { SystemStatus } from "./SystemStatus";

describe("SystemStatus Component Tests", () => {
  test("renders Online status when ollamaOnline is true", () => {
    render(
      <SystemStatus
        ollamaOnline={true}
        modelLabel="phi4-mini:latest"
        dbVersion="1.0"
        appVersion="0.1.1"
      />
    );

    const statusText = screen.getByTestId("ollama-status-text");
    const statusDot = screen.getByTestId("ollama-status-dot");

    expect(statusText.textContent).toBe("Ready");
    expect(statusDot).toHaveClass("online");
    expect(statusDot).not.toHaveClass("offline");
  });

  test("renders choose-model status when service is online without a selected model", () => {
    render(
      <SystemStatus
        ollamaOnline={true}
        modelLabel="No model selected"
        dbVersion="1.0"
        appVersion="0.1.1"
      />
    );

    const statusText = screen.getByTestId("ollama-status-text");
    const statusDot = screen.getByTestId("ollama-status-dot");

    expect(statusText.textContent).toBe("Choose model");
    expect(statusDot).toHaveClass("warning");
    expect(statusDot).not.toHaveClass("online");
  });

  test("offers a direct model setup action when the service is online without a selected model", () => {
    const openAiSetup = vi.fn();
    render(
      <SystemStatus
        ollamaOnline={true}
        modelLabel="No model selected"
        dbVersion="1.0"
        appVersion="0.1.1"
        onOpenAiSetup={openAiSetup}
      />
    );

    screen.getByRole("button", { name: "Set up model" }).click();
    expect(openAiSetup).toHaveBeenCalledTimes(1);
  });

  test("renders Offline status when ollamaOnline is false", () => {
    render(
      <SystemStatus
        ollamaOnline={false}
        dbVersion="1.0"
        appVersion="0.1.1"
      />
    );

    const statusText = screen.getByTestId("ollama-status-text");
    const statusDot = screen.getByTestId("ollama-status-dot");

    expect(statusText.textContent).toBe("Offline");
    expect(statusDot).toHaveClass("offline");
    expect(statusDot).not.toHaveClass("online");
  });
});
