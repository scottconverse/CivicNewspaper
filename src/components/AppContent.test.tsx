// src/components/AppContent.test.tsx
import { render, screen } from "@testing-library/react";
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
});
