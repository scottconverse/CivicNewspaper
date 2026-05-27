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
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Brighton", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_setting" && args?.key === "model.selected") {
        return "phi3:mini";
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

    await act(async () => {
      await hookResult.handleDailyScan();
    });

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
  });
});
