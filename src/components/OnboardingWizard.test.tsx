import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, test, expect, vi, beforeEach, afterEach } from "vitest";
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

  afterEach(() => {
    vi.useRealTimers();
  });

  test("Happy path: completes onboarding and calls onComplete", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    // 16 GB RAM maps to the high tier per OnboardingWizard's
    // ram >= 16 ? high : ram >= 8 ? medium : low recommendation logic.
    const recommendedModel = 'qwen2.5:7b';
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [recommendedModel], version: "0.1.0" });
      if (cmd === "generate_pairing_pin") return Promise.resolve("ABCD-1234");
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    // Step 1
    expect(screen.getByText("Step 1 of 5")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 2
    await waitFor(() => expect(screen.getByText("Step 2 of 5")).toBeInTheDocument());
    // The RAM-based model recommendation renders via a separate async path from
    // the step indicator, so await it rather than asserting synchronously.
    await waitFor(() => expect(screen.getAllByText(new RegExp(recommendedModel)).length).toBeGreaterThan(0));
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 4 - the model is already installed, so the wizard skips the download step.
    await waitFor(() => expect(screen.getByText("Step 4 of 5")).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 5
    await waitFor(() => expect(screen.getByText("Step 5 of 5")).toBeInTheDocument());
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

    // Step 2 — unreachable AI service shows the "starting" notice
    await waitFor(() => expect(screen.getByText("Starting the local AI service")).toBeInTheDocument());
    expect(screen.getByRole("button", { name: /Skip for now/i })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Skip for now/i }));

    // Confirm the skip in the styled dialog (replaces native window.confirm)
    await waitFor(() => expect(screen.getByRole("button", { name: /Skip setup/i })).toBeInTheDocument());
    fireEvent.click(screen.getByRole("button", { name: /Skip setup/i }));

    // Should skip to step 4 because step 3 (pull model) is also skipped
    await waitFor(() => expect(screen.getByText("Step 4 of 5")).toBeInTheDocument());
  });

  test("offline AI setup install button invokes app-managed runtime install", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "install_ollama_runtime") return Promise.resolve();
      if (cmd === "ollama_health") return Promise.resolve({ reachable: false, models: [], version: null });
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    fireEvent.click(screen.getByRole("button", { name: /next/i }));
    await waitFor(() => expect(screen.getByText("Starting the local AI service")).toBeInTheDocument());

    fireEvent.click(screen.getByRole("button", { name: /Install local AI runtime/i }));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("install_ollama_runtime"));
  });

  test("offline AI setup auto-starts runtime install after health check settles", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    let resolveInstall: () => void = () => {};

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "install_ollama_runtime") {
        return new Promise<void>((resolve) => {
          resolveInstall = resolve;
        });
      }
      if (cmd === "ollama_health") return Promise.resolve({ reachable: false, models: [], version: null });
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("install_ollama_runtime"));
    expect(await screen.findByText(/Preparing local AI runtime install/i)).toBeInTheDocument();
    resolveInstall();
  });

  test("first-run onboarding uses a scrollable shell body and sticky actions", () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    expect(document.querySelector(".onboarding-step-body")).toBeInTheDocument();
    expect(document.querySelector(".onboarding-actions")).toBeInTheDocument();
  });

  test("identity step focuses publication name first", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    const user = userEvent.setup();

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    const publicationInput = screen.getByLabelText("Publication Name");
    await waitFor(() => expect(publicationInput).toHaveFocus());
    await user.type(publicationInput, "Longmont Ledger");

    expect(publicationInput).toHaveValue("Longmont Ledger");
    expect(screen.getByLabelText("Editor Name")).toHaveValue("");
  });

  test("identity publication name accepts click and keyboard entry", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    const user = userEvent.setup();

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    const publicationInput = screen.getByLabelText("Publication Name");
    await user.click(publicationInput);
    await user.keyboard("ABC");

    expect(publicationInput).toHaveValue("ABC");
  });

  test("identity starter profile advances Longmont setup without keyboard entry", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    const user = userEvent.setup();

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "set_setting") return Promise.resolve();
      if (cmd === "ollama_health") return Promise.resolve({ reachable: false, models: [], version: null });
      if (cmd === "get_community_profile") return Promise.resolve({
        site_title: "My Local Publication",
        organization_type: "single_person",
        city: "Brighton",
        state: "CO",
      });
      if (cmd === "save_community_profile") return Promise.resolve();
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    await user.click(screen.getByRole("link", { name: "Longmont" }));

    await waitFor(() => expect(screen.getByText("Step 2 of 5")).toBeInTheDocument());
    expect(invokeMock).toHaveBeenCalledWith("set_setting", {
      key: "identity.newsroom_name",
      value: "Longmont Civic Desk",
    });
  });

  test("native starter hash route advances to setup after load", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "set_setting") return Promise.resolve();
      if (cmd === "get_community_profile") return Promise.resolve({
        site_title: "My Local Publication",
        organization_type: "single_person",
        city: "Brighton",
        state: "CO",
      });
      if (cmd === "save_community_profile") return Promise.resolve();
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    window.location.hash = "starter=longmont&continueSetup=1";
    window.dispatchEvent(new HashChangeEvent("hashchange"));

    await waitFor(() => expect(screen.getByText("Step 2 of 5")).toBeInTheDocument());
    expect(invokeMock).toHaveBeenCalledWith("set_setting", {
      key: "identity.newsroom_name",
      value: "Longmont Civic Desk",
    });
  });

  test("identity setup auto-recovers when no input events arrive", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "set_setting") return Promise.resolve();
      if (cmd === "get_community_profile") return Promise.resolve({
        site_title: "My Local Publication",
        organization_type: "single_person",
        city: "Brighton",
        state: "CO",
      });
      if (cmd === "save_community_profile") return Promise.resolve();
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    await waitFor(() => expect(screen.getByText("Step 2 of 5")).toBeInTheDocument());
    expect(screen.getByRole("status")).toHaveTextContent("starter Longmont profile");
    expect(invokeMock).toHaveBeenCalledWith("set_setting", {
      key: "identity.newsroom_name",
      value: "Longmont Civic Desk",
    });
  });

  test("model setup auto-starts pull and completes onboarding when no input events arrive", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    const eventApi = await import("@tauri-apps/api/event");
    let completeCallback: any = null;

    vi.mocked(eventApi.listen).mockImplementation(((eventName: string, callback: any) => {
      if (eventName === "ollama-pull-complete") {
        completeCallback = callback;
      }
      return Promise.resolve(() => {});
    }) as any);

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "set_setting") return Promise.resolve();
      if (cmd === "ollama_health") return Promise.resolve({
        reachable: true,
        models: [],
        version: "0.1.0",
      });
      if (cmd === "pull_ollama_model") {
        window.setTimeout(() => completeCallback?.({ payload: undefined }), 0);
        return Promise.resolve();
      }
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} initialStep={2} />);

    await waitFor(() => expect(screen.getByText(/Download a recommended model/i)).toBeInTheDocument());

    await waitFor(() => expect(screen.getByText("Step 3 of 5")).toBeInTheDocument());
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("pull_ollama_model", { modelId: "qwen2.5:7b" }));
    await waitFor(() => expect(handleComplete).toHaveBeenCalled());
  });

  test("offline AI setup keeps Next disabled while runtime install is running", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "install_ollama_runtime") return new Promise(() => {});
      if (cmd === "ollama_health") return Promise.resolve({ reachable: false, models: [], version: null });
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("install_ollama_runtime"));
    expect(screen.getByRole("button", { name: /^next/i })).toBeDisabled();
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

    // Step 2 — service reachable but no models installed shows the ready/download prompt
    const recommendedModel = 'qwen2.5:7b';
    await waitFor(() => expect(screen.getByText(/Download a recommended model/i)).toBeInTheDocument());
    expect(screen.getAllByText(new RegExp(recommendedModel)).length).toBeGreaterThan(0);
    expect(screen.getByRole("button", { name: new RegExp(`Download ${recommendedModel}`, "i") })).toBeInTheDocument();
  });

  test("Ollama reachable with no models: Next starts the recommended model download", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;
    const recommendedModel = 'qwen2.5:7b';

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "get_setting") return Promise.resolve(null);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [], version: "0.1.0" });
      if (cmd === "pull_ollama_model") return new Promise(() => {});
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} />);

    fireEvent.click(screen.getByRole("button", { name: /next/i }));
    await waitFor(() => expect(screen.getByText(/Download a recommended model/i)).toBeInTheDocument());

    fireEvent.click(screen.getByRole("button", { name: /^next/i }));

    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("pull_ollama_model", { modelId: recommendedModel }));
    expect(await screen.findByText("Step 3 of 5")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /^next/i })).toBeDisabled();
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
    expect(screen.getByText("Step 3 of 5")).toBeInTheDocument();
    
    // Click pull recommended model button
    const recommendedModel = 'qwen2.5:7b';
    const pullBtn = await screen.findByRole("button", { name: new RegExp("Download " + recommendedModel, "i") });
    fireEvent.click(pullBtn);

    // Verify it called pull_ollama_model command
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("pull_ollama_model", { modelId: recommendedModel }));

    // Simulate progress event at 50%
    await waitFor(() => expect(progressCallback).not.toBeNull());
    
    const { act } = await import("@testing-library/react");
    act(() => {
      progressCallback({
        payload: {
          model: recommendedModel,
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
          model: recommendedModel,
          status: "success",
          completed: 100,
          total: 100,
        },
      });
    });

    expect(await screen.findByText("Model pulled successfully.")).toBeInTheDocument();
  });

  test("test_onboarding_step2_timeout_shows_retry", async () => {
    vi.useFakeTimers();
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return new Promise(() => {});
      return Promise.resolve();
    });

    const { act } = await import("@testing-library/react");

    render(<OnboardingWizard ollamaOnline={false} systemRam={16} onComplete={handleComplete} />);

    // Let the async init() effect (path/RAM/setting lookups) settle inside act
    // so its state updates don't land outside an act(...) boundary.
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    // handleNext() is async (awaits set_setting then setStep(2)); wrapping the
    // click in an async act flushes that chain — including the step-2 health
    // effect's initial state updates — before we assert.
    await act(async () => {
      fireEvent.click(screen.getByRole("button", { name: /next/i }));
    });

    await vi.waitFor(() => {
      expect(screen.getByText("Starting the local AI service...")).toBeInTheDocument();
    });

    act(() => {
      vi.advanceTimersByTime(30000);
    });

    await vi.waitFor(() => {
      expect(screen.getByText("Couldn't reach the AI service")).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /retry/i })).toBeInTheDocument();
      expect(screen.getByRole("button", { name: /Save diagnostics file/i })).toBeInTheDocument();
    });

    vi.useRealTimers();
  });

  test("test_onboarding_no_models_continue_button_advances_step", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [], version: "0.1.0" });
      if (cmd === "ollama_list_models") return Promise.resolve([]); // Mock returns [] (no models installed)
      return Promise.resolve();
    });

    // Render with initialStep={1}
    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} initialStep={1} />);

    // Step 1
    expect(screen.getByText("Step 1 of 5")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /next/i }));

    // Step 2
    // setStep(2) advances the wizard to step 2; expect step 2 is active
    await waitFor(() => expect(screen.getByText("Step 2 of 5")).toBeInTheDocument());

    // Locate the Next button in the footer and click it
    const nextBtn = screen.getByRole("button", { name: /next/i });
    expect(nextBtn).toBeInTheDocument();
    fireEvent.click(nextBtn);

    // Verify it advanced to Step 3
    await waitFor(() => expect(screen.getByText("Step 3 of 5")).toBeInTheDocument());
  });

  test("requires explicit confirmation before skipping model download from primary Next", async () => {
    const handleComplete = vi.fn();
    const invokeMock = tauriCore.invoke as any;

    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "get_system_ram") return Promise.resolve(16);
      if (cmd === "ollama_health") return Promise.resolve({ reachable: true, models: [], version: "0.1.0" });
      if (cmd === "get_setting") return Promise.resolve(null);
      return Promise.resolve();
    });

    render(<OnboardingWizard ollamaOnline={true} systemRam={16} onComplete={handleComplete} initialStep={3} />);

    expect(screen.getByText("Step 3 of 5")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /^next/i }));

    expect(await screen.findByText("Skip the model download?")).toBeInTheDocument();
    expect(screen.getByText(/Daily Scan and AI drafting will run in limited mode/i)).toBeInTheDocument();
    expect(screen.getByText("Step 3 of 5")).toBeInTheDocument();
  });
});
