// src/components/AppContent.test.tsx
import { act, render, screen } from "@testing-library/react";
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
      expect(focus).toHaveBeenCalledWith({ preventScroll: true });
    } finally {
      HTMLElement.prototype.scrollIntoView = originalScrollIntoView;
      HTMLElement.prototype.focus = originalFocus;
      vi.useRealTimers();
    }
  });
});
