// src/components/SystemStatus.test.tsx
import { render, screen } from "@testing-library/react";
import { describe, test, expect } from "vitest";
import { SystemStatus } from "./SystemStatus";

describe("SystemStatus Component Tests", () => {
  test("renders Online status when ollamaOnline is true", () => {
    render(
      <SystemStatus
        ollamaOnline={true}
        dbVersion="1.0"
        appVersion="0.1.1"
      />
    );

    const statusText = screen.getByTestId("ollama-status-text");
    const statusDot = screen.getByTestId("ollama-status-dot");

    expect(statusText.textContent).toBe("Online");
    expect(statusDot).toHaveClass("online");
    expect(statusDot).not.toHaveClass("offline");
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
