// src/components/SettingsPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
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

  test("backup path input is editable and save profile fires with the right path/data", () => {
    const handleBackupPathChange = vi.fn();
    const handleSaveProfile = vi.fn();

    render(
      <SettingsPanel
        communityProfile={mockProfile}
        onSaveProfile={handleSaveProfile}
        backupPathInput="C:\backup.db"
        onBackupPathInputChange={handleBackupPathChange}
        onBackupSave={vi.fn()}
        onBackupRestore={vi.fn()}
      />
    );

    // 1. Verify backup path input is editable
    const backupInput = screen.getByLabelText(/Backup \/ Restore Path/i) as HTMLInputElement;
    expect(backupInput.value).toBe("C:\\backup.db");
    
    fireEvent.change(backupInput, { target: { value: "C:\\new-backup.db" } });
    expect(handleBackupPathChange).toHaveBeenCalledWith("C:\\new-backup.db");

    // 2. Modify publication name and verify save profile fires with correct profile data
    const titleInput = screen.getByLabelText(/Publication Name/i) as HTMLInputElement;
    fireEvent.change(titleInput, { target: { value: "Updated Observer" } });

    const saveBtn = screen.getByRole("button", { name: /Save Profile & Policies/i });
    fireEvent.click(saveBtn);

    expect(handleSaveProfile).toHaveBeenCalledWith({
      ...mockProfile,
      site_title: "Updated Observer"
    });
  });
});
