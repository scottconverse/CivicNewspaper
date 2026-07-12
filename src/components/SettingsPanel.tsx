// src/components/SettingsPanel.tsx
import React, { useState } from "react";
import {
  CommunityProfile,
  GuardrailConfig,
  getGuardrailTerms,
  setGuardrailTerms,
  toUserMessage,
} from "../ipc";

// One editable word list (e.g. accusatory terms). Each word can be removed and
// can be toggled to "high-concern"; by default a match only warns.
const GuardrailWordList: React.FC<{
  title: string;
  hint: string;
  words: string[];
  isBlocking: (w: string) => boolean;
  onToggleBlocking: (w: string) => void;
  onRemove: (w: string) => void;
  onAdd: (w: string) => void;
}> = ({ title, hint, words, isBlocking, onToggleBlocking, onRemove, onAdd }) => {
  const [draft, setDraft] = useState("");
  const add = () => {
    const w = draft.trim();
    if (w) {
      onAdd(w);
      setDraft("");
    }
  };
  return (
    <div>
      <h4 style={{ marginBottom: "0.25rem" }}>{title}</h4>
      <p className="help-text" style={{ marginBottom: "0.75rem" }}>{hint}</p>
      <div style={{ display: "flex", flexWrap: "wrap", gap: "0.5rem", marginBottom: "0.75rem" }}>
        {words.length === 0 && (
          <span className="help-text">No words in this list - nothing will be flagged.</span>
        )}
        {words.map((w) => (
          <span
            key={w}
            style={{
              display: "inline-flex",
              alignItems: "center",
              gap: "0.4rem",
              background: "var(--bg-app)",
              border: "1px solid var(--border-color)",
              borderRadius: "999px",
              padding: "0.2rem 0.6rem",
              fontSize: "0.85rem",
            }}
          >
            <span>{w}</span>
            <label
              style={{
                display: "inline-flex",
                alignItems: "center",
                gap: "0.2rem",
                cursor: "pointer",
                color: isBlocking(w) ? "var(--color-error)" : "var(--text-secondary)",
              }}
              title="Ask for an editor note when this word is used (otherwise it only warns)"
            >
              <input
                type="checkbox"
                checked={isBlocking(w)}
                onChange={() => onToggleBlocking(w)}
                aria-label={`Mark the word "${w}" as high concern`}
              />
              high concern
            </label>
            <button
              type="button"
              aria-label={`Remove ${w}`}
              onClick={() => onRemove(w)}
              style={{
                border: "none",
                background: "transparent",
                cursor: "pointer",
                color: "var(--text-secondary)",
                fontSize: "1rem",
                lineHeight: 1,
              }}
            >
              x
            </button>
          </span>
        ))}
      </div>
      <div style={{ display: "flex", gap: "0.5rem" }}>
        <input
          type="text"
          value={draft}
          placeholder="Add a word..."
          onChange={(e) => setDraft(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              add();
            }
          }}
          style={{ flex: 1, padding: "0.4rem" }}
        />
        <button type="button" className="btn btn-secondary btn-sm" onClick={add}>
          Add
        </button>
      </div>
    </div>
  );
};

// Editor-owned master word lists that drive the pre-publish guardrails. Warn-only
// by default; the editor can mark specific words as high-concern warnings.
const GuardrailEditor: React.FC = () => {
  const [config, setConfig] = useState<GuardrailConfig | null>(null);
  const [loadError, setLoadError] = useState<string>("");
  const [saveStatus, setSaveStatus] = useState<string>("");

  React.useEffect(() => {
    getGuardrailTerms()
      .then(setConfig)
      .catch((e) => setLoadError(toUserMessage(e)));
  }, []);

  if (loadError) {
    return (
      <div className="card" id="card-guardrail-words">
        <h3 className="card-title">Story guardrails</h3>
        <p className="help-text">{loadError}</p>
      </div>
    );
  }
  if (!config) {
    return (
      <div className="card" id="card-guardrail-words">
        <h3 className="card-title">Story guardrails</h3>
        <p className="help-text">Loading guardrail words...</p>
      </div>
    );
  }

  const isBlocking = (w: string) =>
    config.blocking.some((b) => b.toLowerCase() === w.toLowerCase());
  const toggleBlocking = (w: string) =>
    setConfig({
      ...config,
      blocking: isBlocking(w)
        ? config.blocking.filter((b) => b.toLowerCase() !== w.toLowerCase())
        : [...config.blocking, w],
    });
  const removeFrom = (key: "accusatory" | "legal", w: string) =>
    setConfig({
      ...config,
      [key]: config[key].filter((x) => x !== w),
      blocking: config.blocking.filter((b) => b.toLowerCase() !== w.toLowerCase()),
    });
  const addTo = (key: "accusatory" | "legal", w: string) => {
    if (config[key].some((x) => x.toLowerCase() === w.toLowerCase())) return;
    setConfig({ ...config, [key]: [...config[key], w] });
  };

  const save = async () => {
    try {
      setSaveStatus("Saving...");
      await setGuardrailTerms(config);
      setSaveStatus("Saved. Stories checked from now on use these words.");
      setTimeout(() => setSaveStatus(""), 4000);
    } catch (e) {
      setSaveStatus(`Save failed: ${toUserMessage(e)}`);
    }
  };

  return (
    <div className="card" id="card-guardrail-words">
      <h3 className="card-title">Story guardrails</h3>
      <p className="help-text">
        Drafts are scanned for these words. By default a match only <strong>warns</strong> the
        editor. Tick <strong>high concern</strong> on a word to ask for an editor note before
        approval. The app ranks and records concerns; it does not decide what the editor may see
        or publish. Changes apply to stories checked from now on.
      </p>
      <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem", marginTop: "1rem" }}>
        <GuardrailWordList
          title="Accusatory terms"
          hint="Flagged when used in a paragraph with no linked source."
          words={config.accusatory}
          isBlocking={isBlocking}
          onToggleBlocking={toggleBlocking}
          onRemove={(w) => removeFrom("accusatory", w)}
          onAdd={(w) => addTo("accusatory", w)}
        />
        <GuardrailWordList
          title="Charge / legal terms"
          hint={'Flagged when used without "alleged" / "allegedly" (presumption of innocence).'}
          words={config.legal}
          isBlocking={isBlocking}
          onToggleBlocking={toggleBlocking}
          onRemove={(w) => removeFrom("legal", w)}
          onAdd={(w) => addTo("legal", w)}
        />
      </div>
      <div style={{ marginTop: "1.25rem", display: "flex", alignItems: "center", gap: "1rem" }}>
        <button type="button" className="btn btn-primary" onClick={save} id="btn-save-guardrails">
          Save guardrail words
        </button>
        {saveStatus && <span className="help-text">{saveStatus}</span>}
      </div>
    </div>
  );
};

interface SettingsPanelProps {
  communityProfile: CommunityProfile | null;
  onSaveProfile: (profile: CommunityProfile) => void;
  onChooseLogo?: () => Promise<string | null>;
  backupPathInput: string;
  onBackupPathInputChange: (val: string) => void;
  onBackupSave: () => void;
  onBackupRestore: () => void;
}

export const SettingsPanel: React.FC<SettingsPanelProps> = ({
  communityProfile,
  onSaveProfile,
  onChooseLogo,
  backupPathInput,
  onBackupPathInputChange,
  onBackupSave,
  onBackupRestore
}) => {
  const [profileForm, setProfileForm] = useState<CommunityProfile | null>(communityProfile);
  const [profileSaveStatus, setProfileSaveStatus] = useState<string>("");

  const applyNeutralStarterCopy = () => {
    if (!profileForm) return;
    setProfileForm({
      ...profileForm,
      site_title: profileForm.site_title || "My Local Publication",
      site_subtitle: "Local news and community information.",
      about_text: "A locally edited publication for this community.",
      ethics_text: "Editorial standards are set by the publisher. Corrections are published when needed.",
      how_we_report_text: "Stories, sources, and publication decisions are reviewed by the editor before publication.",
      footer_text: "",
    });
    setProfileSaveStatus("Neutral starter copy loaded. Review it, then save.");
  };

  React.useEffect(() => {
    setProfileForm(communityProfile);
  }, [communityProfile]);

  const handleProfileSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (profileForm) {
      try {
        setProfileSaveStatus("Saving...");
        await Promise.resolve(onSaveProfile(profileForm));
        setProfileSaveStatus("Identity saved.");
        setTimeout(() => setProfileSaveStatus(""), 4000);
      } catch (error) {
        setProfileSaveStatus(`Save failed: ${toUserMessage(error)}`);
      }
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
          <div className="card-title-row">
            <h3 className="card-title">Publication identity</h3>
            <button
              className="btn btn-primary btn-sm"
              type="submit"
              form="form-save-profile"
              disabled={!profileForm}
            >
              Save now
            </button>
          </div>
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
                  <label htmlFor="input-profile-subtitle" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publication tagline</label>
                  <input
                    type="text"
                    value={profileForm.site_subtitle}
                    onChange={(e) => setProfileForm({ ...profileForm, site_subtitle: e.target.value })}
                    required
                    id="input-profile-subtitle"
                  />
                </div>
                <div>
                  <label htmlFor="input-profile-organization-type" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publisher type</label>
                  <select
                    value={profileForm.organization_type || "single_person"}
                    onChange={(e) => setProfileForm({ ...profileForm, organization_type: e.target.value })}
                    id="input-profile-organization-type"
                  >
                    <option value="single_person">Single person</option>
                    <option value="for_profit">For-profit publication</option>
                    <option value="nonprofit">Nonprofit publication</option>
                    <option value="private_org">Private organization</option>
                    <option value="community_group">Community group</option>
                    <option value="other">Other</option>
                  </select>
                </div>
                <div>
                  <label htmlFor="input-profile-city" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>City</label>
                  <input
                    type="text"
                    value={profileForm.city}
                    onChange={(e) => setProfileForm({ ...profileForm, city: e.target.value })}
                    required
                    id="input-profile-city"
                  />
                </div>
                <div>
                  <label htmlFor="input-profile-state" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>State</label>
                  <input
                    type="text"
                    value={profileForm.state}
                    onChange={(e) => setProfileForm({ ...profileForm, state: e.target.value })}
                    required
                    id="input-profile-state"
                  />
                </div>
                <div>
                  <label htmlFor="input-profile-layout" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Site layout</label>
                  <select
                    value={profileForm.layout_style || "classic"}
                    onChange={(e) => setProfileForm({ ...profileForm, layout_style: e.target.value })}
                    id="input-profile-layout"
                  >
                    <option value="classic">Classic newspaper</option>
                    <option value="compact">Compact</option>
                    <option value="wide">Wide</option>
                    <option value="modern">Modern</option>
                  </select>
                </div>
                <div>
                  <label htmlFor="input-profile-accent" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Accent color</label>
                  <input
                    type="color"
                    value={profileForm.accent_color || "#5a1818"}
                    onChange={(e) => setProfileForm({ ...profileForm, accent_color: e.target.value })}
                    id="input-profile-accent"
                  />
                </div>
              </div>

              <div>
                <label htmlFor="input-profile-threshold" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>High-cost contract alert threshold</label>
                <input
                  type="number"
                  value={profileForm.money_threshold}
                  onChange={(e) => setProfileForm({ ...profileForm, money_threshold: parseFloat(e.target.value) || 0 })}
                  required
                  id="input-profile-threshold"
                />
              </div>

              {/* The about / ethics / how-we-report copy that appears on the
                  published site is editable here; Civic Desk does not inject
                  business-model, ad-policy, sourcing, or AI-disclosure claims. */}
              <div>
                <label htmlFor="input-profile-logo" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Logo image URL</label>
                <div style={{ display: "flex", gap: "0.5rem", alignItems: "center" }}>
                  <input
                    type="text"
                    value={profileForm.logo_url || ""}
                    onChange={(e) => setProfileForm({ ...profileForm, logo_url: e.target.value })}
                    id="input-profile-logo"
                    placeholder="https://example.com/logo.png"
                  />
                  <button
                    type="button"
                    className="btn btn-secondary"
                    id="btn-choose-logo"
                    onClick={async () => {
                      if (!onChooseLogo) return;
                      const logoUrl = await onChooseLogo();
                      if (logoUrl) {
                        setProfileForm({ ...profileForm, logo_url: logoUrl });
                        setProfileSaveStatus("Logo loaded. Save identity to publish it.");
                      }
                    }}
                  >
                    Choose image
                  </button>
                  {profileForm.logo_url && (
                    <button
                      type="button"
                      className="btn btn-secondary"
                      id="btn-clear-logo"
                      onClick={() => setProfileForm({ ...profileForm, logo_url: "" })}
                    >
                      Clear
                    </button>
                  )}
                </div>
                {profileForm.logo_url && (
                  <img
                    src={profileForm.logo_url}
                    alt="Logo preview"
                    style={{ display: "block", maxWidth: "220px", maxHeight: "90px", objectFit: "contain", marginTop: "0.75rem" }}
                  />
                )}
                <p className="help-text" style={{ marginTop: "0.25rem" }}>
                  Optional. Choose a local image or paste an HTTPS logo URL. Leave blank for a text-only masthead.
                </p>
              </div>

              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-profile-about">About this site</label>
                <textarea
                  value={profileForm.about_text}
                  onChange={(e) => setProfileForm({ ...profileForm, about_text: e.target.value })}
                  id="input-profile-about"
                  style={{ width: "100%", minHeight: "60px" }}
                />
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-profile-ethics">Ethics statement</label>
                <textarea
                  value={profileForm.ethics_text}
                  onChange={(e) => setProfileForm({ ...profileForm, ethics_text: e.target.value })}
                  id="input-profile-ethics"
                  style={{ width: "100%", minHeight: "60px" }}
                />
              </div>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-profile-howwereport">How we report (published on your site)</label>
                <textarea
                  value={profileForm.how_we_report_text}
                  onChange={(e) => setProfileForm({ ...profileForm, how_we_report_text: e.target.value })}
                  id="input-profile-howwereport"
                  style={{ width: "100%", minHeight: "70px" }}
                />
                <p className="help-text" style={{ marginTop: "0.25rem" }}>
                  This text is published as written. Add AI, ad, ownership, corrections, or sourcing disclosures only if they match your publication.
                </p>
              </div>

              <div>
                <label htmlFor="input-profile-footer" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Footer / legal note</label>
                <textarea
                  value={profileForm.footer_text || ""}
                  onChange={(e) => setProfileForm({ ...profileForm, footer_text: e.target.value })}
                  id="input-profile-footer"
                  style={{ width: "100%", minHeight: "55px" }}
                  placeholder="Optional copyright, ownership, sponsorship, ad, or contact note."
                />
                <p className="help-text" style={{ marginTop: "0.25rem" }}>
                  The app will not invent copyright, ownership, ad-policy, or AI-disclosure language.
                </p>
              </div>

              <label
                htmlFor="chk-first-amendment-advisor"
                style={{ display: "flex", alignItems: "flex-start", gap: "0.5rem", color: "var(--text-secondary)" }}
              >
                <input
                  id="chk-first-amendment-advisor"
                  type="checkbox"
                  checked={profileForm.first_amendment_advisor_enabled !== false}
                  onChange={(e) => setProfileForm({ ...profileForm, first_amendment_advisor_enabled: e.target.checked })}
                  style={{ marginTop: "0.15rem" }}
                />
                <span>
                  Show the First Amendment advisor in the Workbench.
                  <span className="help-text" style={{ display: "block" }}>
                    Advisory only. It warns and asks good questions; it does not decide what can be shown to the editor or published.
                  </span>
                </span>
              </label>

              <div style={{ display: "flex", alignItems: "center", gap: "1rem", flexWrap: "wrap" }}>
                <button className="btn btn-primary" type="submit" id="btn-save-profile">
                  Save identity
                </button>
                <button className="btn btn-secondary" type="button" onClick={applyNeutralStarterCopy} id="btn-neutral-starter-copy">
                  Use neutral starter copy
                </button>
                {profileSaveStatus && <span className="help-text" role="status">{profileSaveStatus}</span>}
              </div>
            </form>
          ) : (
            <p id="profile-loading-text">Loading profile configurations...</p>
          )}
        </div>

        <GuardrailEditor />

        <div className="card" id="card-backup-disaster">
          <h3 className="card-title">Backups</h3>
          <p className="help-text">
            All your leads, drafts, and sources live in one file on this computer. Back it up somewhere safe.
          </p>
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }} htmlFor="input-backup-path">Backup file</label>
              <p className="help-text" style={{ margin: "0 0 0.5rem" }}>
                Enter a complete <code>.db</code> filename in Downloads or the Civic Desk data folder.
              </p>
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
