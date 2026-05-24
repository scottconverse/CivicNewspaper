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
    <div style={{ maxWidth: "900px", margin: "0 auto" }} id="settings-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Ethics Profile & Core Backups</h1>
          <p>Adjust guardrails parameters, define ethics policies, and backup the local SQLite database.</p>
        </div>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1.5rem" }}>
        {/* Profile setup */}
        <div className="card" id="card-ethics-profile">
          <h3 className="card-title">Community Profile & Thresholds</h3>
          {profileForm ? (
            <form onSubmit={handleProfileSubmit} style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="form-save-profile">
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-profile-title">Publication Name</label>
                <input
                  type="text"
                  value={profileForm.site_title}
                  onChange={(e) => setProfileForm({ ...profileForm, site_title: e.target.value })}
                  required
                  id="input-profile-title"
                />
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Subtitle / Motto</label>
                <input
                  type="text"
                  value={profileForm.site_subtitle}
                  onChange={(e) => setProfileForm({ ...profileForm, site_subtitle: e.target.value })}
                  required
                  id="input-profile-subtitle"
                />
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Public Inquest Threshold ($)</label>
                <input
                  type="number"
                  value={profileForm.money_threshold}
                  onChange={(e) => setProfileForm({ ...profileForm, money_threshold: parseFloat(e.target.value) || 0 })}
                  required
                  id="input-profile-threshold"
                />
                <p className="help-text">Contracts exceeding this triggers high-priority procurement warnings.</p>
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Ethics Statement</label>
                <textarea
                  value={profileForm.ethics_text}
                  onChange={(e) => setProfileForm({ ...profileForm, ethics_text: e.target.value })}
                  style={{ height: "80px" }}
                  required
                  id="textarea-profile-ethics"
                />
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Methodology (How We Report)</label>
                <textarea
                  value={profileForm.how_we_report_text}
                  onChange={(e) => setProfileForm({ ...profileForm, how_we_report_text: e.target.value })}
                  style={{ height: "80px" }}
                  required
                  id="textarea-profile-methodology"
                />
              </div>
              <button className="btn btn-primary" type="submit" id="btn-save-profile">
                Save Profile & Policies
              </button>
            </form>
          ) : (
            <p id="profile-loading-text">Loading profile configurations...</p>
          )}
        </div>

        {/* Database Backups settings */}
        <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
          {/* Backups Panel */}
          <div className="card" id="card-backup-disaster">
            <h3 className="card-title">Backup & Disaster Recovery</h3>
            <p className="help-text">
              Save the entire SQLite database containing paired clients, RSS sources, drafts, and evidence items to a single local file.
            </p>
            <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-backup-path">Backup / Restore Path</label>
                <input
                  type="text"
                  value={backupPathInput}
                  onChange={(e) => onBackupPathInputChange(e.target.value)}
                  required
                  id="input-backup-path"
                />
              </div>
              <div className="btn-group">
                <button className="btn btn-secondary" style={{ flexGrow: 1 }} onClick={onBackupSave} id="btn-backup-save">
                  Create Backup
                </button>
                <button className="btn btn-danger" style={{ flexGrow: 1 }} onClick={onBackupRestore} id="btn-backup-restore">
                  Restore Backup
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
