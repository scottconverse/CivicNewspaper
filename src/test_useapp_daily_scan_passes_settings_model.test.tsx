// src/useApp.test.tsx
import { render, screen, act } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { sanitizeEvidenceCitations, useApp } from "./useApp";

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
  test("sanitizeEvidenceCitations preserves linked evidence and neutralizes unsupported IDs", () => {
    const text = "Valid [source](evidence: 7). Bad [source](evidence:999). Also bad evidence://123. Mixed Evidence:456.";
    const sanitized = sanitizeEvidenceCitations(text, [7]);
    expect(sanitized).toContain("evidence:7");
    expect(sanitized).not.toContain("evidence: 7");
    expect(sanitized).not.toContain("evidence:999");
    expect(sanitized).not.toContain("evidence://123");
    expect(sanitized).not.toContain("Evidence:456");
    expect(sanitized).toContain("unlinked-evidence-999");
    expect(sanitized).toContain("unlinked-evidence-123");
    expect(sanitized).toContain("unlinked-evidence-456");
  });

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

    // The invoke spy is shared across tests in this file; clear its call history
    // (the mount-time IPC calls) so the assertions below prove THIS
    // handleDailyScan call issued the invokes — matching the negative test.
    vi.mocked(invoke).mockClear();

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

  test("test_useapp_daily_scan_degrades_when_selected_model_unavailable", async () => {
    // Phase 9: Daily Scan must not dead-end when the selected model is missing.
    // It warns, then still runs the deterministic evidence/verification path.
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
    expect(invoke).toHaveBeenCalledWith("run_daily_scan", { city: "Brighton", state: "CO", sinceHours: 24 });
    expect(hookResult.statusMessage).toContain("Daily Scan complete");
  });

  test("imports discovered official sources with official tier", async () => {
    const calls: Array<{ cmd: string; args: any }> = [];
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      calls.push({ cmd, args });
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Brighton", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_setting") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.1.0" };
      if (cmd === "add_source") return 42;
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
      hookResult.handleToggleDiscoveredSource({
        name: "Brighton Council Agendas",
        url: "https://www.brightonco.gov/AgendaCenter",
        type: "primary_record",
      });
    });

    vi.mocked(invoke).mockClear();
    calls.length = 0;

    await act(async () => {
      await hookResult.handleImportDiscoveredSources();
    });

    expect(calls).toContainEqual({
      cmd: "add_source",
      args: {
        name: "Brighton Council Agendas",
        url: "https://www.brightonco.gov/AgendaCenter",
        type: "primary_record",
        tier: "official_record",
      },
    });
  });

  test("opening a lead moves drafting into the Workbench route", async () => {
    vi.mocked(invoke).mockImplementation(async (cmd: string) => {
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      hookResult.handleOpenDraftWizard({
        id: 12,
        detector_name: "Public Meeting Scheduled",
        why: "A Longmont meeting notice appeared.",
        confidence: "medium",
        risk_level: "low",
        confirmation_checklist: "[]",
        created_at: "2026-06-29T00:00:00Z",
      });
    });

    expect(screen.getByTestId("active-tab")).toHaveTextContent("workbench");
    expect(hookResult.selectedLead?.id).toBe(12);
    expect(invoke).toHaveBeenCalledWith("get_evidence", { leadId: 12 });
  });

  test("generating a draft persists it and opens the editor", async () => {
    const savedDrafts: any[] = [];
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: savedDrafts };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "generate_draft") {
        return "Headline: Council weighs land purchase\n\nNut graf: The council is reviewing a land purchase tied to a public agenda item.\n\nThe decision could affect nearby residents.";
      }
      if (cmd === "save_draft") {
        const draft = { ...args.draft, id: 501 };
        savedDrafts.push(draft);
        return 501;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      hookResult.handleOpenDraftWizard({
        id: 77,
        detector_name: "Public Meeting Scheduled",
        why: "A Longmont board packet mentions a land purchase.",
        confidence: "medium",
        risk_level: "medium",
        confirmation_checklist: "[]",
        created_at: "2026-06-29T00:00:00Z",
      });
    });

    vi.mocked(invoke).mockClear();

    await act(async () => {
      await hookResult.handleGenerateText();
    });

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
    expect(invoke).toHaveBeenCalledWith("ollama_health");
    expect(invoke).toHaveBeenCalledWith("generate_draft", {
      leadId: 77,
      format: "watch",
      systemPrompt: undefined,
    });
    expect(invoke).toHaveBeenCalledWith(
      "save_draft",
      expect.objectContaining({
        draft: expect.objectContaining({
          lead_id: 77,
          content: "The council is reviewing a land purchase tied to a public agenda item.\n\nThe decision could affect nearby residents.",
          status: "draft_generated",
        }),
      })
    );
    expect(invoke).toHaveBeenCalledWith("guardrails_check", { draftId: 501 });
    expect(hookResult.selectedLead).toBeNull();
    expect(hookResult.selectedDraft?.id).toBe(501);
    expect(hookResult.selectedDraft?.title).toBe("Council weighs land purchase");
    expect(hookResult.selectedDraft?.title).not.toMatch(/^Draft:/);
    expect(hookResult.selectedDraft?.content).not.toMatch(/Headline:|Nut graf:/i);
    expect(screen.getByTestId("active-tab")).toHaveTextContent("workbench");
  });

  test("generated draft normalization removes reporter-note scaffolding", async () => {
    const savedDrafts: any[] = [];
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: savedDrafts };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "generate_draft") {
        return "Headline: Council to review traffic safety\n\nEDITOR_NOTE: This needs more reporting.\n\nThe council will review traffic safety at an upcoming meeting [insert date if available].\n\nReporting Steps:\n- Call the clerk.\n- Verify the agenda.\n\nResidents can follow the next posted agenda for details. [Source](evidence:4)\n\n[End of Report]";
      }
      if (cmd === "save_draft") {
        const draft = { ...args.draft, id: 502 };
        savedDrafts.push(draft);
        return 502;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      hookResult.handleOpenDraftWizard({
        id: 78,
        detector_name: "Public Meeting Scheduled",
        why: "A Longmont traffic safety item appeared.",
        confidence: "medium",
        risk_level: "low",
        confirmation_checklist: "[]",
        created_at: "2026-06-29T00:00:00Z",
      });
    });

    await act(async () => {
      await hookResult.handleGenerateText();
    });

    const content = hookResult.selectedDraft?.content ?? "";
    expect(hookResult.selectedDraft?.title).toBe("Council to review traffic safety");
    expect(content).not.toMatch(/EDITOR_NOTE|Reporting Steps|End of Report|\[insert/i);
    expect(content).toContain("The council will review traffic safety at an upcoming meeting.");
    expect(content).toContain("Residents can follow the next posted agenda for details.");
  });

  test("drafts from watch or low-novelty leads open as needs-work with guardrails loaded", async () => {
    const savedDrafts: any[] = [];
    const guardrailsReport = {
      is_clean: true,
      issues: [
        { category: "Lead Readiness", message: "Watch item needs a current fact.", severity: "warning" },
      ],
    };
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: savedDrafts };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "generate_draft") return "A city page was updated and residents may want to watch it.";
      if (cmd === "save_draft") {
        const draft = { ...args.draft, id: 503 };
        savedDrafts.push(draft);
        return 503;
      }
      if (cmd === "guardrails_check") return guardrailsReport;
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      hookResult.handleOpenDraftWizard({
        id: 79,
        detector_name: "Monitoring Item",
        why: "A Longmont source page was fetched again.",
        confidence: "medium",
        risk_level: "low",
        confirmation_checklist: "[]",
        story_type: "watch",
        disposition: "watch",
        novelty_score: 1,
        created_at: "2026-06-29T00:00:00Z",
      });
    });

    await act(async () => {
      await hookResult.handleGenerateText();
    });

    expect(savedDrafts[0].status).toBe("needs_verification");
    expect(hookResult.selectedDraft?.status).toBe("needs_verification");
    expect(hookResult.guardrailsReport).toEqual(guardrailsReport);
    expect(hookResult.statusMessage).toContain("marked as needing more work");
  });

  test("can draft multiple leads sequentially without losing queue/workbench state", async () => {
    let nextDraftId = 700;
    const savedDrafts: any[] = [];
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: savedDrafts };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "generate_draft") return `Generated draft for lead ${args.leadId}.`;
      if (cmd === "save_draft") {
        const draft = { ...args.draft, id: nextDraftId++ };
        savedDrafts.push(draft);
        return draft.id;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    const leads = [1, 2, 3].map((id) => ({
      id,
      detector_name: "Public Meeting Scheduled",
      why: `Longmont test lead ${id}`,
      confidence: "medium",
      risk_level: "medium",
      confirmation_checklist: "[]",
      created_at: "2026-06-29T00:00:00Z",
    }));

    for (const lead of leads) {
      await act(async () => {
        hookResult.setActiveTab("queue");
        hookResult.setSelectedDraft(null);
        hookResult.handleOpenDraftWizard(lead);
      });

      expect(screen.getByTestId("active-tab")).toHaveTextContent("workbench");
      expect(hookResult.selectedLead?.id).toBe(lead.id);
      expect(hookResult.selectedDraft).toBeNull();

      await act(async () => {
        await hookResult.handleGenerateText();
      });

      expect(hookResult.selectedLead).toBeNull();
      expect(hookResult.selectedDraft?.lead_id).toBe(lead.id);
      expect(hookResult.selectedDraft?.title).toBe(`Longmont test lead ${lead.id}`);
      expect(hookResult.selectedDraft?.title).not.toMatch(/^Draft:/);
      expect(hookResult.selectedDraft?.content).toBe(`Generated draft for lead ${lead.id}.`);
      expect(screen.getByTestId("active-tab")).toHaveTextContent("workbench");
    }

    expect(savedDrafts.map((draft) => draft.lead_id)).toEqual([1, 2, 3]);
    expect(new Set(savedDrafts.map((draft) => draft.id)).size).toBe(3);
  });

  test("opening an existing draft reaches Workbench even if evidence refresh fails", async () => {
    const existingDraft = {
      id: 44,
      lead_id: 12,
      format: "watch",
      title: "Draft: Longmont budget item",
      content: "Existing draft body.",
      status: "draft_generated",
      verification_checklist: "[]",
    };

    vi.mocked(invoke).mockImplementation(async (cmd: string) => {
      if (cmd === "get_queue") return { leads: [], drafts: [existingDraft] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") throw new Error("evidence unavailable");
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      hookResult.setSelectedLead({
        id: 99,
        detector_name: "Decision / Vote",
        why: "Stale verification lead should not hijack an existing draft.",
        confidence: "medium",
        risk_level: "low",
        confirmation_checklist: "[]",
        created_at: "2026-06-29T00:00:00Z",
      });
    });

    await act(async () => {
      await hookResult.handleOpenDraftEditor(44);
    });

    expect(screen.getByTestId("active-tab")).toHaveTextContent("workbench");
    expect(hookResult.selectedDraft?.id).toBe(44);
    expect(hookResult.selectedDraft?.title).toBe("Draft: Longmont budget item");
    expect(hookResult.selectedLead).toBeNull();
    expect(hookResult.errorMessage).toContain("evidence unavailable");
  });

  test("complete onboarding seeds Longmont starter sources for first run", async () => {
    const settings = new Map<string, string | null>();
    const addSourceCalls: any[] = [];

    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "is_onboarding_complete") return false;
      if (cmd === "get_queue") return { leads: [], drafts: [] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "get_setting") return settings.get(args.key) ?? null;
      if (cmd === "set_setting") {
        settings.set(args.key, args.value);
        return null;
      }
      if (cmd === "add_source") {
        addSourceCalls.push(args);
        return addSourceCalls.length;
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="active-tab">{hookResult.activeTab}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });

    await act(async () => {
      await hookResult.completeOnboarding();
    });

    expect(settings.get("setup.first_run_intake")).toBe("consumed");
    expect(addSourceCalls).toHaveLength(19);
    expect(addSourceCalls.map(call => call.name)).toContain("Longmont Public Information");
    expect(addSourceCalls.map(call => call.name)).toContain("Longmont Leader local news");
    expect(addSourceCalls.find(call => call.name === "Longmont Leader local news")?.tier).toBe("news_reporting");
    expect(addSourceCalls.map(call => call.name)).toContain("Visit Longmont events");
    expect(addSourceCalls.map(call => call.name)).toContain("Downtown Longmont events");
    expect(addSourceCalls.map(call => call.name)).toContain("Longmont Public Safety Facebook");
    expect(addSourceCalls.map(call => call.name)).toContain("Longmont city YouTube");
    expect(screen.getByTestId("active-tab")).toHaveTextContent("dailyScan");
    expect(hookResult.statusMessage).toContain("starter Longmont source");
  });

  test("improve for publication preflights selected model and sanitizes unsupported evidence citations", async () => {
    const draft = {
      id: 610,
      lead_id: 77,
      format: "watch",
      title: "Original headline",
      content: "Original body. [Source](evidence:7)",
      status: "draft_generated",
      verification_checklist: "[]",
    };
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: [draft] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") {
        expect(args).toEqual({ leadId: 77 });
        return [{ id: 7, source_id: 1, fetched_at: "2026-06-30T00:00:00Z", excerpt: "Agenda", content_hash: "hash", entities: "[]" }];
      }
      if (cmd === "guardrails_check") return { is_clean: true, issues: [] };
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["qwen2.5:7b"], version: "0.6.0" };
      if (cmd === "llm_task") {
        return "Headline: Improved headline\n\nValid citation [Source](evidence:7). Unsupported citation [Bad](evidence:999).";
      }
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="status">{hookResult.statusMessage}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });
    await act(async () => {
      await hookResult.handleOpenDraftEditor(610);
    });
    vi.mocked(invoke).mockClear();

    await act(async () => {
      await hookResult.handleImproveForPublication();
    });

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
    expect(invoke).toHaveBeenCalledWith("ollama_health");
    expect(invoke).toHaveBeenCalledWith("llm_task", expect.any(Object));
    expect(hookResult.selectedDraft.title).toBe("Improved headline");
    expect(hookResult.selectedDraft.content).toContain("evidence:7");
    expect(hookResult.selectedDraft.content).toContain("unlinked-evidence-999");
    expect(hookResult.selectedDraft.content).not.toContain("evidence:999");
  });

  test("improve for publication does not call LLM when the selected model is unavailable", async () => {
    const draft = {
      id: 611,
      lead_id: 77,
      format: "watch",
      title: "Original headline",
      content: "Original body.",
      status: "draft_generated",
      verification_checklist: "[]",
    };
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: [draft] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "get_setting" && args?.key === "model.selected") return "qwen2.5:7b";
      if (cmd === "ollama_health") return { reachable: true, models: ["llama3.2:3b"], version: "0.6.0" };
      if (cmd === "llm_task") throw new Error("llm_task should not run");
      return null;
    });

    let hookResult: any;
    const TestComp = () => {
      hookResult = useApp();
      return <span data-testid="status">{hookResult.statusMessage}</span>;
    };

    await act(async () => {
      render(<TestComp />);
    });
    await act(async () => {
      await hookResult.handleOpenDraftEditor(611);
    });
    vi.mocked(invoke).mockClear();

    await act(async () => {
      await hookResult.handleImproveForPublication();
    });

    expect(invoke).toHaveBeenCalledWith("get_setting", { key: "model.selected" });
    expect(invoke).toHaveBeenCalledWith("ollama_health");
    expect(invoke).not.toHaveBeenCalledWith("llm_task", expect.any(Object));
    expect(hookResult.selectedDraft.title).toBe("Original headline");
    expect(hookResult.selectedDraft.content).toBe("Original body.");
    expect(hookResult.errorMessage).toContain("isn't downloaded yet");
  });

  test("approve publish saves visible edited draft before advancing status", async () => {
    const savedDrafts: any[] = [];
    const draft = {
      id: 620,
      lead_id: 77,
      format: "watch",
      title: "Original headline",
      content: "Original body.",
      status: "draft_generated",
      verification_checklist: "[]",
    };
    vi.mocked(invoke).mockImplementation(async (cmd: string, args: any) => {
      if (cmd === "get_queue") return { leads: [], drafts: savedDrafts.length ? savedDrafts : [draft] };
      if (cmd === "get_sources") return [];
      if (cmd === "get_community_profile") return { city: "Longmont", state: "CO" };
      if (cmd === "list_paired_clients") return [];
      if (cmd === "get_system_ram") return 16;
      if (cmd === "get_evidence") return [];
      if (cmd === "guardrails_check") return { is_clean: true, issues: [] };
      if (cmd === "get_setting" && args?.key === "identity.editor_name") return "Scott";
      if (cmd === "save_draft") {
        savedDrafts.push({ ...args.draft, id: 620 });
        return 620;
      }
      if (cmd === "attest_draft") return null;
      if (cmd === "story_decision") return null;
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
      await hookResult.handleOpenDraftEditor(620);
    });
    await act(async () => {
      hookResult.setSelectedDraft({
        ...hookResult.selectedDraft,
        title: "Edited headline",
        content: "Edited body the editor actually reviewed.",
      });
    });
    vi.mocked(invoke).mockClear();

    await act(async () => {
      await hookResult.handleApprovePublish("Reviewed story-quality warnings.");
    });

    expect(invoke).toHaveBeenCalledWith("save_draft", {
      draft: expect.objectContaining({
        id: 620,
        title: "Edited headline",
        content: "Edited body the editor actually reviewed.",
      }),
    });
    expect(invoke).toHaveBeenCalledWith("guardrails_check", { draftId: 620 });
    expect(invoke).toHaveBeenCalledWith("attest_draft", { id: 620, editor: "Scott" });
    expect(invoke).toHaveBeenCalledWith("story_decision", {
      id: 620,
      decision: "ready_to_publish",
      overrideReason: "Reviewed story-quality warnings.",
    });
    const saveOrder = vi.mocked(invoke).mock.invocationCallOrder.find((_, idx) => vi.mocked(invoke).mock.calls[idx][0] === "save_draft")!;
    const decisionOrder = vi.mocked(invoke).mock.invocationCallOrder.find((_, idx) => vi.mocked(invoke).mock.calls[idx][0] === "story_decision")!;
    expect(saveOrder).toBeLessThan(decisionOrder);
  });
});
