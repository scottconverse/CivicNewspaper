// src/components/SourcesPanel.tsx
import React from "react";
import { FileUp, Plus, Trash2, RefreshCcw, Search, Upload } from "lucide-react";
import { Source, DiscoveredSource, DiscoveredSourceCategory } from "../ipc";
import { openExternalUrl, toUserMessage } from "../ipc";
import { BulkImportReview, credibilityForSource } from "../bulkImportParser";
import { Modal } from "./Modal";

interface SourcesPanelProps {
  sources: Source[];
  loading: boolean;
  newSourceName: string;
  onNewSourceNameChange: (val: string) => void;
  newSourceUrl: string;
  onNewSourceUrlChange: (val: string) => void;
  newSourceType: string;
  onNewSourceTypeChange: (val: string) => void;
  newSourceTier: string;
  onNewSourceTierChange: (val: string) => void;
  onAddSource: (e: React.FormEvent) => void;
  onDeleteSource: (id: number) => void;
  
  // Bulk import
  showBulkImportModal: boolean;
  onShowBulkImportModalChange: (val: boolean) => void;
  bulkImportText: string;
  onBulkImportTextChange: (val: string) => void;
  bulkImportType: string;
  onBulkImportTypeChange: (val: string) => void;
  bulkImportLoading: boolean;
  bulkImportReview: BulkImportReview;
  onBuildBulkImportReview: () => void;
  onToggleBulkImportItem: (id: string) => void;
  onChooseBulkImportFile: () => void;
  onBulkImport: (e: React.FormEvent) => void;
  
  // Discovery
  showDiscoveryModal: boolean;
  onShowDiscoveryModalChange: (val: boolean) => void;
  discoveryCity: string;
  onDiscoveryCityChange: (val: string) => void;
  discoveryState: string;
  onDiscoveryStateChange: (val: string) => void;
  discoveryLoading: boolean;
  onRunDiscovery: (e: React.FormEvent) => void;
  discoveredCats: DiscoveredSourceCategory[];
  selectedDiscovered: DiscoveredSource[];
  onToggleDiscoveredSource: (source: DiscoveredSource) => void;
  onImportDiscoveredSources: () => void;
  onClearDiscovered: () => void;
}

export const SourcesPanel: React.FC<SourcesPanelProps> = ({
  sources,
  loading,
  newSourceName,
  onNewSourceNameChange,
  newSourceUrl,
  onNewSourceUrlChange,
  newSourceType,
  onNewSourceTypeChange,
  newSourceTier,
  onNewSourceTierChange,
  onAddSource,
  onDeleteSource,
  showBulkImportModal,
  onShowBulkImportModalChange,
  bulkImportText,
  onBulkImportTextChange,
  bulkImportType,
  onBulkImportTypeChange,
  bulkImportLoading,
  bulkImportReview,
  onBuildBulkImportReview,
  onToggleBulkImportItem,
  onChooseBulkImportFile,
  onBulkImport,
  showDiscoveryModal,
  onShowDiscoveryModalChange,
  discoveryCity,
  onDiscoveryCityChange,
  discoveryState,
  onDiscoveryStateChange,
  discoveryLoading,
  onRunDiscovery,
  discoveredCats,
  selectedDiscovered,
  onToggleDiscoveredSource,
  onImportDiscoveredSources,
  onClearDiscovered
}) => {
  const [linkError, setLinkError] = React.useState("");

  const handleOpenUrl = async (url: string, event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    setLinkError("");
    try {
      await openExternalUrl(url);
    } catch (err) {
      setLinkError(toUserMessage(err));
    }
  };

  const getStatusColor = (status: string) => {
    if (status === "online") return "online";
    if (status === "quiet") return "warning";
    return "offline";
  };

  const formatStatus = (status: string) => {
    if (status === "online") return "Online";
    if (status === "quiet") return "Quiet";
    if (status === "offline") return "Offline";
    return status ? status.replace(/_/g, " ") : "Unknown";
  };

  const formatSourceKind = (value: string) => {
    if (value === "primary_record") return "Primary";
    if (value === "official_comm") return "Official";
    if (value === "community_signal") return "Watch";
    if (value === "media_lead") return "Media";
    return value.replace(/_/g, " ");
  };

  const formatTier = (value?: string) => {
    if (value === "official_record") return "Primary";
    if (value === "news_reporting") return "Secondary";
    if (value === "community_signal") return "Watch";
    return value ? value.replace(/_/g, " ") : "Watch";
  };

  const credibilityBadgeClass = (credibility: string) => {
    if (credibility === "Official record") return "badge-success";
    if (credibility === "Search helper") return "badge-warning";
    if (credibility === "Community signal") return "badge-info";
    return "badge-neutral";
  };

  const selectedBulkCount = bulkImportReview.accepted.filter(item => item.selected).length;

  return (
    <div>
      <div className="page-header">
        <div className="page-title">
          <h1>Sources</h1>
          <p>The municipal feeds and record portals The Civic Desk watches for you.</p>
        </div>
        <div className="btn-group">
          <button 
            className="btn btn-secondary" 
            onClick={() => onShowDiscoveryModalChange(true)}
            style={{ marginRight: "0.5rem" }}
            id="btn-trigger-discovery"
          >
            <Search size={16} />
            Discover for my city
          </button>
          <button className="btn btn-secondary" onClick={() => onShowBulkImportModalChange(true)} id="btn-trigger-bulk-import">
            <Upload size={16} />
            Bulk import
          </button>
        </div>
      </div>

      <div className="sources-grid">
        {linkError && (
          <div className="card" role="alert" style={{ gridColumn: "1 / -1", borderLeft: "4px solid var(--color-error)" }}>
            <span className="error-text">Couldn't open source link: {linkError}</span>
          </div>
        )}
        <div className="card source-list-card">
          <div className="table-container">
            <table className="sources-table">
              <thead>
                <tr>
                  <th>Source</th>
                  <th>Tier</th>
                  <th>Last check</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                {sources.length === 0 ? (
                  <tr>
                    <td colSpan={4} className="text-center">No feeds or portals registered yet. Add one in the right panel.</td>
                  </tr>
                ) : (
                  sources.map((src) => (
                    <tr key={src.id} data-testid={`source-row-${src.id}`}>
                      <td>
                        <div className="source-cell">
                          <span className={`status-dot ${getStatusColor(src.status)}`} />
                          <div className="source-copy">
                            <div>
                              <strong>{src.name}</strong>
                              <span className="source-type-chip">{formatSourceKind(src.type)}</span>
                              <span className="source-type-chip">{formatStatus(src.status)}</span>
                            </div>
                            <a
                              className="source-url-link"
                              href={src.url}
                              title={src.url}
                              onClick={(event) => handleOpenUrl(src.url, event)}
                            >
                              {src.url}
                            </a>
                          </div>
                        </div>
                      </td>
                      <td>
                        <span className="badge badge-neutral">{formatTier(src.tier)}</span>
                      </td>
                      <td className={src.status === "offline" ? "source-last source-last-error" : "source-last"}>
                        {src.last_scraped ? new Date(src.last_scraped).toLocaleDateString() : "Never"}
                      </td>
                      <td>
                        <button className="icon-button danger" onClick={() => onDeleteSource(src.id!)} id={`btn-delete-source-${src.id}`} aria-label="Delete source">
                          <Trash2 size={12} />
                        </button>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>

        <div className="card">
          <h3 className="card-title">Add a source</h3>
          <p className="help-text">Paste a city webpage or RSS feed. The Civic Desk figures out the rest.</p>
          <form onSubmit={onAddSource} style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="form-add-source">
            <div>
              <label htmlFor="input-new-source-name" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Name</label>
              <input
                type="text"
                placeholder="e.g. City Council Agendas"
                value={newSourceName}
                onChange={(e) => onNewSourceNameChange(e.target.value)}
                required
                id="input-new-source-name"
              />
            </div>

            <div>
              <label htmlFor="input-new-source-url" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>URL</label>
              <input
                type="url"
                placeholder="e.g. https://city.gov/agendas/rss"
                value={newSourceUrl}
                onChange={(e) => onNewSourceUrlChange(e.target.value)}
                required
                id="input-new-source-url"
              />
            </div>

            <div className="form-grid-two">
              <div>
                <label htmlFor="select-new-source-type" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Type</label>
                <select value={newSourceType} onChange={(e) => onNewSourceTypeChange(e.target.value)} id="select-new-source-type">
                  <option value="primary_record">Auto-detect</option>
                  <option value="official_comm">RSS</option>
                  <option value="community_signal">HTML</option>
                  <option value="media_lead">Media</option>
                </select>
              </div>

              <div>
                <label htmlFor="select-new-source-tier" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Tier</label>
                <select value={newSourceTier} onChange={(e) => onNewSourceTierChange(e.target.value)} id="select-new-source-tier">
                  <option value="official_record">Primary</option>
                  <option value="news_reporting">Secondary</option>
                  <option value="community_signal">Watch</option>
                </select>
              </div>
            </div>

            <button className="btn btn-primary" type="submit" disabled={loading} id="btn-submit-add-source">
              <Plus size={16} />
              Add Source
            </button>
          </form>
        </div>
      </div>

      {/* Bulk Import Modal */}
      {showBulkImportModal && (
        <Modal
          id="modal-bulk-import"
          labelledBy="bulk-import-title"
          contentStyle={{ maxWidth: "600px", width: "90%", display: "flex", flexDirection: "column" }}
          onClose={() => onShowBulkImportModalChange(false)}
        >
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 id="bulk-import-title" style={{ margin: 0 }}>Bulk Import Sources</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => onShowBulkImportModalChange(false)}>Close</button>
            </div>
            
            <form onSubmit={onBulkImport} style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="form-bulk-import">
              <div className="card" style={{ padding: "0.85rem", background: "var(--bg-sidebar)" }}>
                <strong>Import review</strong>
                <p className="help-text" style={{ margin: "0.25rem 0 0 0" }}>
                  Paste URLs or load a source-list file. Civic Desk reviews the rows first, flags duplicates and search-helper links, and imports only the checked sources.
                </p>
              </div>

              <div>
                <label htmlFor="select-bulk-import-type" style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Default Classification Type</label>
                <select id="select-bulk-import-type" value={bulkImportType} onChange={(e) => onBulkImportTypeChange(e.target.value)}>
                  <option value="primary_record">Primary Record (Agendas, budgets, public notices)</option>
                  <option value="official_comm">Official Communication (Press releases, announcements)</option>
                  <option value="community_signal">Community Signal (Local forums, neighborhood boards)</option>
                  <option value="media_lead">Media Lead (Newspapers, regional feeds)</option>
                </select>
              </div>

              <div>
                <label htmlFor="textarea-bulk-import" style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>
                  Source list
                </label>
                <p className="help-text" style={{ margin: "0 0 0.5rem 0", fontSize: "0.8rem" }}>
                  Paste a list or load CSV, TSV, TXT, DOCX, XLSX, or text-based PDF files. You can optionally prefix with a name, e.g.,<br />
                  <code>Brighton Council, https://brightonco.gov/agenda</code><br />
                  Image-only scanned PDFs may need OCR before URLs can be read.
                </p>
                <textarea
                  placeholder="https://example.com/feed.xml&#10;Brighton School District, https://sd27j.org/board-agenda&#10;https://reddit.com/r/brightonco"
                  value={bulkImportText}
                  onChange={(e) => onBulkImportTextChange(e.target.value)}
                  style={{ height: "200px", fontFamily: "monospace", fontSize: "0.85rem", background: "var(--bg-card)", border: "1px solid var(--border-color)", color: "var(--text-primary)", borderRadius: "4px", padding: "0.5rem" }}
                  required
                  disabled={bulkImportLoading}
                  id="textarea-bulk-import"
                />
              </div>

              <div className="btn-group">
                <button className="btn btn-secondary" type="button" onClick={onChooseBulkImportFile} disabled={bulkImportLoading}>
                  <FileUp size={16} />
                  Load file
                </button>
                <button className="btn btn-secondary" type="button" onClick={onBuildBulkImportReview} disabled={bulkImportLoading || !bulkImportText.trim()}>
                  Review list
                </button>
              </div>

              {(bulkImportReview.accepted.length > 0 || bulkImportReview.rejected.length > 0 || bulkImportReview.duplicates.length > 0) && (
                <div className="card" style={{ padding: "1rem" }} data-testid="bulk-import-review">
                  <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
                    <div>
                      <h4 style={{ margin: "0 0 0.25rem 0" }}>Review before importing</h4>
                      <p className="help-text" style={{ margin: 0 }}>
                        {bulkImportReview.accepted.length} importable, {bulkImportReview.duplicates.length} duplicate, {bulkImportReview.rejected.length} skipped. Selected: {selectedBulkCount}.
                      </p>
                    </div>
                    <span className="badge badge-info">{selectedBulkCount} checked</span>
                  </div>

                  {bulkImportReview.accepted.length > 0 && (
                    <div style={{ display: "grid", gap: "0.75rem", marginTop: "1rem" }}>
                      {bulkImportReview.accepted.map(item => (
                        <label key={item.id} style={{ display: "grid", gridTemplateColumns: "auto 1fr", gap: "0.75rem", alignItems: "flex-start" }}>
                          <input
                            type="checkbox"
                            checked={item.selected}
                            onChange={() => onToggleBulkImportItem(item.id)}
                            style={{ marginTop: "0.25rem" }}
                          />
                          <div>
                            <div className="flex-between" style={{ gap: "0.5rem", alignItems: "flex-start" }}>
                              <strong>{item.name}</strong>
                              <span className={`badge ${credibilityBadgeClass(item.credibility)}`}>{item.credibility}</span>
                            </div>
                            <a href={item.url} onClick={(event) => handleOpenUrl(item.url, event)} style={{ wordBreak: "break-all" }}>{item.url}</a>
                            <p className="help-text" style={{ margin: "0.25rem 0 0 0" }}>
                              {item.review_note} Type: {formatSourceKind(item.type)}. Tier: {formatTier(item.tier)}.
                            </p>
                          </div>
                        </label>
                      ))}
                    </div>
                  )}

                  {(bulkImportReview.duplicates.length > 0 || bulkImportReview.rejected.length > 0) && (
                    <details style={{ marginTop: "1rem" }}>
                      <summary>Skipped rows</summary>
                      <ul style={{ marginBottom: 0 }}>
                        {[...bulkImportReview.duplicates, ...bulkImportReview.rejected].map(item => (
                          <li key={`${item.row}-${item.text}`}>
                            Row {item.row}: {item.reason}
                          </li>
                        ))}
                      </ul>
                    </details>
                  )}
                </div>
              )}

              <div style={{ display: "flex", justifyContent: "flex-end", gap: "1rem", marginTop: "1rem" }}>
                <button className="btn btn-secondary" type="button" onClick={() => onShowBulkImportModalChange(false)} disabled={bulkImportLoading}>
                  Cancel
                </button>
                <button className="btn btn-primary" type="submit" disabled={bulkImportLoading || (bulkImportReview.accepted.length > 0 && selectedBulkCount === 0)} id="btn-submit-bulk-import">
                  {bulkImportLoading ? "Importing..." : bulkImportReview.accepted.length > 0 ? "Import Checked Sources" : "Review List"}
                </button>
              </div>
            </form>
        </Modal>
      )}

      {/* Discovery Modal */}
      {showDiscoveryModal && (
        <Modal
          id="modal-discovery"
          labelledBy="discovery-title"
          contentStyle={{ maxWidth: "800px", width: "90%", maxHeight: "85vh", display: "flex", flexDirection: "column" }}
          onClose={() => onShowDiscoveryModalChange(false)}
        >
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 id="discovery-title" style={{ margin: 0 }}>Town Setup & Source Auto-Discovery</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => onShowDiscoveryModalChange(false)}>Close</button>
            </div>
            
            <form onSubmit={onRunDiscovery} style={{ display: "flex", gap: "1rem", marginBottom: "1.5rem" }} id="form-run-discovery">
              <div style={{ flex: 1 }}>
                <label htmlFor="input-discovery-city" className="sr-only">City name</label>
                <input
                  type="text"
                  placeholder="City Name (e.g. Brighton)"
                  value={discoveryCity}
                  onChange={(e) => onDiscoveryCityChange(e.target.value)}
                  required
                  disabled={discoveryLoading}
                  id="input-discovery-city"
                />
              </div>
              <div style={{ width: "150px" }}>
                <label htmlFor="input-discovery-state" className="sr-only">State</label>
                <input
                  type="text"
                  placeholder="State (e.g. CO)"
                  value={discoveryState}
                  onChange={(e) => onDiscoveryStateChange(e.target.value)}
                  required
                  disabled={discoveryLoading}
                  id="input-discovery-state"
                />
              </div>
              <button className="btn btn-primary" type="submit" disabled={discoveryLoading} id="btn-submit-discovery">
                {discoveryLoading ? "Searching..." : "Auto-Find Feeds"}
              </button>
            </form>

            <div style={{ flex: 1, overflowY: "auto", paddingRight: "0.5rem" }}>
              {discoveryLoading && (
                <div style={{ textAlign: "center", padding: "3rem 0" }} id="discovery-loading-indicator">
                  {/* UX-m4: unified spinner idiom (lucide RefreshCcw + .animate-spin),
                      matching the onboarding wizard, instead of a hand-rolled div. */}
                  <RefreshCcw className="animate-spin" size={40} style={{ color: "var(--accent-primary)", marginBottom: "1rem" }} />
                  <p>Searching for agendas, subreddits, library calendars, and local news...</p>
                  <p className="help-text" style={{ fontSize: "0.85rem", marginTop: "0.5rem" }}>Running priority checklist queries sequentially. This takes a few seconds.</p>
                </div>
              )}

              {!discoveryLoading && discoveredCats.length === 0 && (
                <div style={{ textAlign: "center", padding: "3rem 0", color: "var(--text-secondary)" }}>
                  <p>Enter your town's name and state above to auto-discover local civic feeds.</p>
                </div>
              )}

              {!discoveryLoading && discoveredCats.length > 0 && (
                <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }} id="discovery-results-container">
                  <p className="help-text">
                    Select only the sources you trust. Search links are helpers for review, not verified feeds.
                  </p>
                  {discoveredCats.map((cat, idx) => (
                    <div key={idx} className="card" style={{ padding: "1rem", background: "var(--bg-sidebar)", border: "1px solid var(--border-color)" }}>
                      <div className="flex-between" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem", marginBottom: "0.5rem" }}>
                        <h4 style={{ margin: 0 }}>{cat.category_name}</h4>
                        <span className="badge badge-neutral" style={{ fontSize: "0.75rem", textTransform: "capitalize" }}>
                          {cat.type.replace(/_/g, " ")}
                        </span>
                      </div>
                      
                      {cat.candidates.length === 0 ? (
                        <p className="help-text" style={{ fontStyle: "italic" }}>No candidate portals detected. You can add one manually later.</p>
                      ) : (
                        <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                          {cat.candidates.map((cand: any, cIdx: number) => {
                            const isChecked = selectedDiscovered.some(item => item.url === cand.url);
                            const credibility = credibilityForSource(cand);
                            return (
                              <label key={cIdx} style={{ display: "flex", alignItems: "flex-start", gap: "0.75rem", cursor: "pointer", fontSize: "0.9rem" }}>
                                <input
                                  type="checkbox"
                                  checked={isChecked}
                                  onChange={() => onToggleDiscoveredSource(cand)}
                                  style={{ marginTop: "0.25rem" }}
                                />
                                <div>
                                  <div style={{ display: "flex", gap: "0.5rem", alignItems: "center", flexWrap: "wrap" }}>
                                    <strong>{cand.name}</strong>
                                    <span className={`badge ${credibilityBadgeClass(credibility.credibility)}`}>{credibility.credibility}</span>
                                  </div>
                                  <a href={cand.url} onClick={(event) => handleOpenUrl(cand.url, event)} style={{ fontSize: "0.8rem", color: "var(--accent-primary)", wordBreak: "break-all" }}>
                                    {cand.url}
                                  </a>
                                  <p className="help-text" style={{ margin: "0.2rem 0 0 0" }}>{credibility.note}</p>
                                </div>
                              </label>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>

            {!discoveryLoading && discoveredCats.length > 0 && (
              <div style={{ borderTop: "1px solid var(--border-color)", paddingTop: "1rem", marginTop: "1rem", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                <span>Selected: <strong>{selectedDiscovered.length}</strong> sources</span>
                <div className="btn-group">
                  <button className="btn btn-secondary" onClick={onClearDiscovered}>
                    Clear
                  </button>
                  <button className="btn btn-primary" onClick={onImportDiscoveredSources} disabled={selectedDiscovered.length === 0} id="btn-import-discovered">
                    Import Checked Sources
                  </button>
                </div>
              </div>
            )}
        </Modal>
      )}
    </div>
  );
};
