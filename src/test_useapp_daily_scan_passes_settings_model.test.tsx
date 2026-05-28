// src/useApp.test.tsx
import { render, screen, act } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { useApp } from "./useApp";

// Mock tauri core invoke to return mock initial data
import { invoke } from "@tauri-apps/api/core";

// Helper component to test the hook
const TestComponent = () => {
  const app = useApp();
  return (
    <div>
      <span data-testid="active-tab">{app.activeTab}</span>
      <span data-testid="loading">{app.loading ? "yes" : "no"}</span>
      <span data-testid="new-source-type">{app.newSourceType}</span>
      <button data-testid="btn-change-tab" onClick={() => app.setActiveTab("sources")}>
        Change Tab
      </button>
    </div>
  );
};

describe("useApp Hook Tests", () => {
  test("initializes hook states correctly and handles navigation updates", async () => {
    // Mock invoke implementations
    vi.mocked(invoke).mockImplementation(async (cmd: string, ..._args: any[]) => {
      if (cmd === "get_queue") {
        return { leads: [], drafts: [] };
      }
      if (cmd === "get_sources") {
        return [];
      }
      if (cmd === "get_community_profile") {
        return {};
      }
      if (cmd === "list_paired_clients") {
        return [];
      }
      if (cmd === "get_system_ram") {
        return 16;
      }
      return null;
    });

    await act(async () => {
      render(<TestComponent />);
    });

    // Assert initial activeTab is queue
    expect(screen.getByTestId("active-tab")).toHaveTextContent("queue");
    expect(screen.getByTestId("new-source-type")).toHaveTextContent("primary_record");

    // Click button to change tab
    const btn = screen.getByTestId("btn-change-tab");
    act(() => {
      btn.click();
    });

    expect(screen.getByTestId("active-tab")).toHaveTextContent("sources");
  });

  test("test_useapp_daily_scan_end_to_end_model", async () => {
    const expectedModel = "phi3:mini";
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Brighton", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_setting" && args?.key === "model.selected") {
        return expectedModel;
      }
      if (cmd === "ollama_health") {
        return { reachable: true, models: [expectedModel], version: "0.1.0" };
      }
      if (cmd === "run_daily_scan") {
        return 1;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return null;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      await hookResult.handleDailyScan();
    });

    // The selected model is read from settings, checked against Ollama's
    // available models, and only then does the scan run. These assertions prove
    // that real IPC flow — not that a local constant equals itself.
    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
    expect(invoke).toHaveBeenCalledWith("ollama_health");
    expect(invoke).toHaveBeenCalledWith("run_daily_scan", { city: "Brighton", state: "CO", sinceHours: 24 });
  });

  test("test_useapp_daily_scan_blocks_when_selected_model_unavailable", async () => {
    // C-1 remediation: prove the selected model genuinely gates the scan.
    // The user selected gemma2:9b but Ollama only has phi3:mini, so the scan
    // must NOT run. If the model value were ignored, run_daily_scan would fire.
    const selectedModel = "gemma2:9b";
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Brighton", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_setting" && args?.key === "model.selected") {
        return selectedModel;
      }
      if (cmd === "ollama_health") {
        return { reachable: true, models: ["phi3:mini"], version: "0.1.0" };
      }
      if (cmd === "run_daily_scan") {
        return 1;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return null;
    };

    await act(async () => {
      render(<TestComp />);
    });

    // The invoke spy is shared across tests in this file; clear its call
    // history so the assertions below reflect only the handleDailyScan flow.
    vi.mocked(invoke).mockClear();

    await act(async () => {
      await hookResult.handleDailyScan();
    });

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
    expect(invoke).toHaveBeenCalledWith("ollama_health");
    expect(invoke).not.toHaveBeenCalledWith(
      "run_daily_scan",
      expect.anything()
    );
    expect(hookResult.errorMessage).toContain(selectedModel);
  });
});
