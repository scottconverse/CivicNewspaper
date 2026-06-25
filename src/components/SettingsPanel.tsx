// src/components/SettingsPanel.tsx
import React, { useState } from "react";
import { CommunityProfile } from "../ipc";

interface SettingsPanelProps {
  communityProfile: CommunityProfile | null;
  onSaveProfile: (profile: CommunityProfile) => void;
  backupPathInput: string;
  onBackupPathInputChange: (val: string) => void;
  onBackupSave: () => void;
  onBackupRestore: () => void;
}

export const SettingsPanel: React.FC<SettingsPanelProps> = ({
  communityProfile,
  onSaveProfile,
  backupPathInput,
  onBackupPathInputChange,
  onBackupSave,
  onBackupRestore
}) => {
  const [profileForm, setProfileForm] = useState<CommunityProfile | null>(communityProfile);

  React.useEffect(() => {
    setProfileForm(communityProfile);
  }, [communityProfile]);

  const handleProfileSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (profileForm) {
      onSaveProfile(profileForm);
    }
  };

  return (
    <div id="settings-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Ethics &amp; Backups</h1>
          <p>Your publication identity and where your records are kept safe.</p>
        </div>
      </div>

      <div className="settings-stack">
        <div className="card" id="card-ethics-profile">
          <h3 className="card-title">Publication identity</h3>
          {profileForm ? (
            <form onSubmit={handleProfileSubmit} id="form-save-profile">
              <div className="settings-form-grid">
                <div>
                  <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-profile-title">Publication name</label>
                  <input
                    type="text"
                    value={profileForm.site_title}
                    onChange={(e) => setProfileForm({ ...profileForm, site_title: e.target.value })}
                    required
                    id="input-profile-title"
                  />
                </div>
                <div>
                  <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Editor</label>
                  <input
                    type="text"
                    value={profileForm.site_subtitle}
                    onChange={(e) => setProfileForm({ ...profileForm, site_subtitle: e.target.value })}
                    required
                    id="input-profile-subtitle"
                  />
                </div>
                <div>
                  <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>City</label>
                  <input
                    type="text"
                    value={profileForm.city}
                    onChange={(e) => setProfileForm({ ...profileForm, city: e.target.value })}
                    required
                    id="input-profile-city"
                  />
                </div>
                <div>
                  <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>State</label>
                  <input
                    type="text"
                    value={profileForm.state}
                    onChange={(e) => setProfileForm({ ...profileForm, state: e.target.value })}
                    required
                    id="input-profile-state"
                  />
                </div>
              </div>

              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>High-cost contract alert threshold</label>
                <input
                  type="number"
                  value={profileForm.money_threshold}
                  onChange={(e) => setProfileForm({ ...profileForm, money_threshold: parseFloat(e.target.value) || 0 })}
                  required
                  id="input-profile-threshold"
                />
              </div>

              <button className="btn btn-primary" type="submit" id="btn-save-profile">
                Save identity
              </button>
            </form>
          ) : (
            <p id="profile-loading-text">Loading profile configurations...</p>
          )}
        </div>

        <div className="card" id="card-backup-disaster">
          <h3 className="card-title">Backups</h3>
          <p className="help-text">
            All your leads, drafts, and sources live in one file on this computer. Back it up somewhere safe.
          </p>
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-backup-path">Backup folder</label>
              <input
                type="text"
                value={backupPathInput}
                onChange={(e) => onBackupPathInputChange(e.target.value)}
                required
                id="input-backup-path"
              />
            </div>
            <div className="btn-group">
              <button
                className="btn btn-primary"
                onClick={onBackupSave}
                disabled={!backupPathInput.trim()}
                title={!backupPathInput.trim() ? "Enter a backup file path first" : "Create a backup at the path above"}
                id="btn-backup-save"
              >
                Back up now
              </button>
              <button
                className="btn btn-secondary"
                onClick={onBackupRestore}
                disabled={!backupPathInput.trim()}
                title={!backupPathInput.trim() ? "Enter a backup file path first" : "Restore from the backup file at the path above"}
                id="btn-backup-restore"
              >
                Restore from backup
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
