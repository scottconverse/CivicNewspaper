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
    const backupInput = screen.getByLabelText(/Backup folder/i) as HTMLInputElement;
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
});
