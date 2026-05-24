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
  test("renders settings and publish panels when activeTab is settings", () => {
    const mockApp = {
      activeTab: "settings",
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
      handlePublish: vi.fn()
    };

    render(<AppContent app={mockApp} />);

    expect(screen.getByTestId("settings-panel")).toBeInTheDocument();
    expect(screen.getByTestId("publish-panel")).toBeInTheDocument();
  });
});
