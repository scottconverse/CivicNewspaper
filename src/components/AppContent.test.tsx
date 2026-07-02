// src/components/AppContent.test.tsx
import { act, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, test, expect, vi } from "vitest";
import { AppContent } from "./AppContent";

// Mock nested components to avoid deep dependencies
vi.mock("./SettingsPanel", () => ({
  SettingsPanel: () => <div data-testid="settings-panel">Settings Panel Mock</div>
}));
vi.mock("./PublishPanel", () => ({
  PublishPanel: () => <div data-testid="publish-panel">Publish Panel Mock</div>
}));
vi.mock("./LeadQueue", () => ({
  LeadQueue: () => <div data-testid="lead-queue">Lead Queue Mock</div>
}));
vi.mock("./DailyScanPage", () => ({
  DailyScanPage: ({ onOpenLead }: { onOpenLead: (lead: any) => void }) => (
    <button
      type="button"
      onClick={() => onOpenLead({ id: 44, title: "Public hearing" })}
    >
      Mock open scan lead
    </button>
  )
}));
vi.mock("./Workbench", () => ({
  Workbench: ({ selectedLead }: { selectedLead: any }) => (
    <div id={selectedLead ? "draft-wizard-panel" : "workbench-editor-panel"} tabIndex={-1} data-testid="workbench">
      Workbench Mock
    </div>
  )
}));

describe("AppContent Component Tests", () => {
  const makeMockApp = (activeTab: string) => ({
    activeTab,
    statusMessage: "",
    errorMessage: "",
    communityProfile: {},
    backupPathInput: "",
    publishPath: "",
    publishStep: 1,
    loading: false,
    handleSaveProfile: vi.fn(),
    handleBackupSave: vi.fn(),
    handleBackupRestore: vi.fn(),
    handlePublish: vi.fn(),
    setActiveTab: vi.fn(),
    handleOpenDraftWizard: vi.fn(),
    setStatusMessage: vi.fn(),
    setErrorMessage: vi.fn(),
    leads: [],
    drafts: [],
    sources: [],
    bulkImportReview: { accepted: [], duplicates: [], rejected: [] },
    discoveredCats: [],
    selectedDiscovered: [],
  });

  test("renders settings panel when activeTab is settings", () => {
    const mockApp = makeMockApp("settings");

    render(<AppContent app={mockApp} />);

    expect(screen.getByTestId("settings-panel")).toBeInTheDocument();
  });

  test("renders publish panel when activeTab is publish", () => {
    const mockApp = makeMockApp("publish");

    render(<AppContent app={mockApp} />);

    expect(screen.getByTestId("publish-panel")).toBeInTheDocument();
  });

  test("keeps setup progress status visible while a long-running action is loading", () => {
    vi.useFakeTimers();
    const mockApp = {
      ...makeMockApp("sources"),
      loading: true,
      statusMessage: "Adding starter sources for Longmont, CO. When this finishes, you will move to Daily Scan.",
      setStatusMessage: vi.fn(),
    };

    try {
      render(<AppContent app={mockApp} />);

      act(() => {
        vi.advanceTimersByTime(7000);
      });

      expect(mockApp.setStatusMessage).not.toHaveBeenCalledWith("");
      expect(screen.getByText(/Adding starter sources for Longmont/i)).toBeInTheDocument();
    } finally {
      vi.useRealTimers();
    }
  });

  test("reveals the draft wizard after a lead is selected", () => {
    vi.useFakeTimers();
    const scrollIntoView = vi.fn();
    const focus = vi.fn();
    const originalScrollIntoView = HTMLElement.prototype.scrollIntoView;
    const originalFocus = HTMLElement.prototype.focus;
    HTMLElement.prototype.scrollIntoView = scrollIntoView;
    HTMLElement.prototype.focus = focus;

    const mockApp = {
      ...makeMockApp("queue"),
      selectedLead: {
        id: 101,
        why: "A council packet contains a new contract.",
        detector_name: "Contract",
        confidence: "high",
        risk_level: "med",
        confirmation_checklist: "[]",
        created_at: "2026-06-28T00:00:00Z"
      },
      selectedDraft: null,
    };

    try {
      render(<AppContent app={mockApp} />);
      act(() => {
        vi.runOnlyPendingTimers();
      });

      expect(scrollIntoView).toHaveBeenCalledWith({ block: "start", behavior: "auto" });
      expect(focus).not.toHaveBeenCalled();
    } finally {
      HTMLElement.prototype.scrollIntoView = originalScrollIntoView;
      HTMLElement.prototype.focus = originalFocus;
      vi.useRealTimers();
    }
  });

  test("keeps Daily Scan fallback guidance visible in Story Queue when no matching lead exists", async () => {
    const mockApp = {
      ...makeMockApp("dailyScan"),
      latestScanId: 3,
      setActiveTab: vi.fn((tab: string) => {
        mockApp.activeTab = tab;
      }),
    };

    const { rerender } = render(<AppContent app={mockApp} />);
    await userEvent.click(screen.getByRole("button", { name: /Mock open scan lead/i }));

    expect(mockApp.setActiveTab).toHaveBeenCalledWith("queue");
    mockApp.activeTab = "queue";
    rerender(<AppContent app={mockApp} />);

    expect(screen.getByText(/Public hearing/)).toBeInTheDocument();
    expect(screen.getByText(/no disconnected draft was created/i)).toBeInTheDocument();
    expect(screen.getByTestId("lead-queue")).toBeInTheDocument();
  });
});
