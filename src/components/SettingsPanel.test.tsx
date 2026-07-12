// src/components/SettingsPanel.test.tsx
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/core";
import { describe, test, expect, vi } from "vitest";
import { SettingsPanel } from "./SettingsPanel";
import { CommunityProfile } from "../ipc";

describe("SettingsPanel Component Tests", () => {
  const mockProfile: CommunityProfile = {
    site_title: "Test Site",
    site_subtitle: "Testing only",
    about_text: "About us",
    ethics_text: "Ethics",
    how_we_report_text: "Process",
    organization_type: "single_person",
    footer_text: "",
    logo_url: "",
    accent_color: "#5a1818",
    layout_style: "classic",
    first_amendment_advisor_enabled: true,
    money_threshold: 50000,
    watchlist: [],
    city: "Test City",
    state: "TC"
  };

  test("backup path input is editable and save profile fires with the right path/data", async () => {
    const handleBackupPathChange = vi.fn();
    const handleSaveProfile = vi.fn();
    const backupPath = String.raw`C:\backup.db`;
    const newBackupPath = String.raw`C:\new-backup.db`;
    vi.mocked(invoke).mockResolvedValue({
      accusatory: [],
      legal: [],
      blocking: [],
    });

    render(
      <SettingsPanel
        communityProfile={mockProfile}
        onSaveProfile={handleSaveProfile}
        backupPathInput={backupPath}
        onBackupPathInputChange={handleBackupPathChange}
        onBackupSave={vi.fn()}
        onBackupRestore={vi.fn()}
      />
    );

    await waitFor(() => {
      expect(screen.queryByText(/Loading guardrail words/i)).not.toBeInTheDocument();
    });

    // 1. Verify backup path input is editable
    const backupInput = screen.getByLabelText(/Backup file/i) as HTMLInputElement;
    expect(backupInput.value).toBe(backupPath);
    
    fireEvent.change(backupInput, { target: { value: newBackupPath } });
    expect(handleBackupPathChange).toHaveBeenCalledWith(newBackupPath);

    // 2. Modify publication name and verify save profile fires with correct profile data
    const titleInput = screen.getByLabelText(/Publication name/i) as HTMLInputElement;
    fireEvent.change(titleInput, { target: { value: "Updated Observer" } });

    const saveBtn = screen.getByRole("button", { name: /Save identity/i });
    fireEvent.click(saveBtn);

    expect(handleSaveProfile).toHaveBeenCalledWith({
      ...mockProfile,
      site_title: "Updated Observer"
    });
    expect(await screen.findByRole("status")).toHaveTextContent("Identity saved.");
  });

  test("choose logo fills the profile logo field and shows a preview", async () => {
    vi.mocked(invoke).mockResolvedValue({
      accusatory: [],
      legal: [],
      blocking: [],
    });
    const handleChooseLogo = vi
      .fn()
      .mockResolvedValue("data:image/png;base64,iVBORw0KGgo=");

    render(
      <SettingsPanel
        communityProfile={mockProfile}
        onSaveProfile={vi.fn()}
        onChooseLogo={handleChooseLogo}
        backupPathInput=""
        onBackupPathInputChange={vi.fn()}
        onBackupSave={vi.fn()}
        onBackupRestore={vi.fn()}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: /Choose image/i }));

    expect(await screen.findByDisplayValue(/data:image\/png;base64/i)).toBeInTheDocument();
    expect(screen.getByAltText(/Logo preview/i)).toBeInTheDocument();
    expect(await screen.findByRole("status")).toHaveTextContent("Logo loaded. Save identity to publish it.");
  });

  test("reports a failed identity save instead of claiming success", async () => {
    vi.mocked(invoke).mockResolvedValue({
      accusatory: [],
      legal: [],
      blocking: [],
    });
    const handleSaveProfile = vi.fn().mockRejectedValue(new Error("disk write failed"));

    render(
      <SettingsPanel
        communityProfile={mockProfile}
        onSaveProfile={handleSaveProfile}
        backupPathInput=""
        onBackupPathInputChange={vi.fn()}
        onBackupSave={vi.fn()}
        onBackupRestore={vi.fn()}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: /Save identity/i }));

    expect(await screen.findByRole("status")).toHaveTextContent(
      "Save failed: Something went wrong: disk write failed",
    );
    expect(screen.queryByText("Identity saved.")).not.toBeInTheDocument();
  });

  test("allows only one identity save while a save is pending", async () => {
    vi.mocked(invoke).mockResolvedValue({
      accusatory: [],
      legal: [],
      blocking: [],
    });
    let finishSave: (() => void) | undefined;
    const pendingSave = new Promise<void>((resolve) => {
      finishSave = resolve;
    });
    const handleSaveProfile = vi.fn().mockReturnValue(pendingSave);

    render(
      <SettingsPanel
        communityProfile={mockProfile}
        onSaveProfile={handleSaveProfile}
        backupPathInput=""
        onBackupPathInputChange={vi.fn()}
        onBackupSave={vi.fn()}
        onBackupRestore={vi.fn()}
      />
    );

    const saveButton = screen.getByRole("button", { name: /Save identity/i });
    fireEvent.click(saveButton);
    fireEvent.click(saveButton);

    expect(handleSaveProfile).toHaveBeenCalledTimes(1);
    expect(saveButton).toBeDisabled();
    expect(screen.getByLabelText(/City/i)).toBeDisabled();
    finishSave?.();
    expect(await screen.findByText("Identity saved.")).toBeInTheDocument();
    await waitFor(() => expect(saveButton).not.toBeDisabled());
  });
});
