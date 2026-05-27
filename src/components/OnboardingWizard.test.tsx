import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, test, expect, vi, beforeEach } from "vitest";
import { OnboardingWizard } from "./OnboardingWizard";
import * as tauriCore from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/path", () => ({
  documentDir: vi.fn(() => Promise.resolve("/documents")),
  appDataDir: vi.fn(() => Promise.resolve("/appdata")),
  join: vi.fn((...args) => Promise.resolve(args.join("/"))),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(() => Promise.resolve(() => {})),
}));

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: vi.fn(),
}));

describe("OnboardingWizard Component Tests", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("Happy path: completes onboarding and calls onComplete", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    const gemma2_9b = ["gemma2", "9b"].join(":");
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [gemma2_9b], version: "0.1.0" });
      if (cmd === "generate_pairing_pin") return Promise.resolve("ABCD-1234");
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    // Step 1
    expect(screen.getByText("Step 1 of 6")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 2
    await waitFor(() => expect(screen.getByText("Step 2 of 6")).toBeInTheDocument());
    expect(screen.getByText(new RegExp(gemma2_9b))).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 3
    await waitFor(() => expect(screen.getByText("Step 3 of 6")).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 4
    await waitFor(() => expect(screen.getByText("Step 4 of 6")).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 5
    await waitFor(() => expect(screen.getByText("Step 5 of 6")).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 6
    await waitFor(() => expect(screen.getByText("Step 6 of 6")).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /Finish Onboarding/i }));

    await waitFor(() => expect(handleComplete).toHaveBeenCalled());
  });

  test("Ollama Unreachable: Shows not detected and allows skipping", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: false, models: [], version: null });
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);
    
    // Step 1
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 2
    await waitFor(() => expect(screen.getByText("Bundled Ollama Sidecar Starting")).toBeInTheDocument());
    expect(screen.getByRole("button", { name: /Skip for now/i })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Skip for now/i }));

    // Should skip to step 4 because step 3 (pull model) is also skipped
    await waitFor(() => expect(screen.getByText("Step 4 of 6")).toBeInTheDocument());
  });

  test("Ollama reachable with Empty Models: shows ready message", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [], version: "0.1.0" });
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);
    
    // Step 1
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 2
    const gemma2_9b = ["gemma2", "9b"].join(":");
    await waitFor(() => expect(screen.getByText(/Pull a recommended model/i)).toBeInTheDocument());
    expect(screen.getByText(new RegExp(gemma2_9b))).toBeInTheDocument();
  });

  test("Model Pull: streams progress and completes", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    let progressCallback: any = null;

    const eventApi = await import("@tauri-apps/api/event");
    vi.mocked(eventApi.listen).mockImplementation(((eventName: string, callback: any) => {
      if (eventName === "ollama-pull-progress") {
        progressCallback = callback;
      }
      return Promise.resolve(() => {});
    }) as any);

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [], version: "0.1.0" });
      if (cmd === "pull_ollama_model") return Promise.resolve();
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} initialStep={3} />);

    // Check we are on Step 3
    expect(screen.getByText("Step 3 of 6")).toBeInTheDocument();
    
    // Click pull recommended model button
    const gemma2_9b = ["gemma2", "9b"].join(":");
    const pullBtn = await screen.findByRole("button", { name: new RegExp("Download " + gemma2_9b, "i") });
    fireEvent.click(pullBtn);

    // Verify it called pull_ollama_model command
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("pull_ollama_model", { modelId: gemma2_9b }));

    // Simulate progress event at 50%
    await waitFor(() => expect(progressCallback).not.toBeNull());
    
    const { act } = await import("@testing-library/react");
    act(() => {
      progressCallback({
        payload: {
          model: gemma2_9b,
          status: "downloading",
          completed: 50,
          total: 100,
        },
      });
    });

    // Expect to see progress percentage in document
    expect(await screen.findByText("50.0%")).toBeInTheDocument();

    // Now simulate success
    act(() => {
      progressCallback({
        payload: {
          model: gemma2_9b,
          status: "success",
          completed: 100,
          total: 100,
        },
      });
    });

    expect(await screen.findByText("Model pulled successfully.")).toBeInTheDocument();
  });
});
