// src/components/SourcesPanel.tsx
import React from "react";
import { Plus, Trash2 } from "lucide-react";
import { Source, DiscoveredSource, DiscoveredSourceCategory } from "../ipc";

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
  const getStatusColor = (status: string) => {
    return status === "online" ? "online" : "offline";
  };

  return (
    <div>
      <div className="page-header">
        <div className="page-title">
          <h1>Sources Manager</h1>
          <p>Configure feeds, portals, and records systems scanned by CivicNews' OSINT detectors.</p>
        </div>
        <div className="btn-group">
          <button 
            className="btn btn-secondary" 
            onClick={() => onShowBulkImportModalChange(true)} 
            style={{ marginRight: "0.5rem" }}
            id="btn-trigger-bulk-import"
          >
            <Plus size={16} />
            Bulk Import URLs
          </button>
          <button className="btn btn-primary" onClick={() => onShowDiscoveryModalChange(true)} id="btn-trigger-discovery">
            <Plus size={16} />
            Auto-Discover Town Feeds
          </button>
        </div>
      </div>

      <div className="sources-grid">
        {/* Left Pane: Sources List */}
        <div className="card">
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Source</th>
                  <th>URL</th>
                  <th>Type</th>
                  <th>Tier</th>
                  <th>Status</th>
                  <th>Scraped</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {sources.length === 0 ? (
                  <tr>
                    <td colSpan={6} className="text-center">No feeds or portals registered yet. Add one in the right panel.</td>
                  </tr>
                ) : (
                  sources.map((src) => (
                    <tr key={src.id} data-testid={`source-row-${src.id}`}>
                      <td><strong>{src.name}</strong></td>
                      <td style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>
                        <a href={src.url} target="_blank" rel="noreferrer" style={{ wordBreak: "break-all" }}>{src.url}</a>
                      </td>
                      <td>
                        <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                          {src.type.replace(/_/g, " ")}
                        </span>
                      </td>
                      <td>
                        <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                          {src.tier ? src.tier.replace(/_/g, " ") : "Community Signal"}
                        </span>
                      </td>
                      <td>
                        <span className={`status-dot ${getStatusColor(src.status)}`} />
                        <span style={{ marginLeft: "0.25rem", fontSize: "0.85rem" }}>{src.status}</span>
                      </td>
                      <td style={{ fontSize: "0.8rem" }}>
                        {src.last_scraped ? new Date(src.last_scraped).toLocaleDateString() : "Never"}
                      </td>
                      <td>
                        <button className="btn btn-danger btn-sm" onClick={() => onDeleteSource(src.id!)} id={`btn-delete-source-${src.id}`} aria-label="Delete source">
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

        {/* Right Pane: Add Source Form */}
        <div className="card">
          <h3 className="card-title">Register Portal/Feed</h3>
          <form onSubmit={onAddSource} style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="form-add-source">
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Source Name</label>
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
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Feed URL (RSS or static HTML)</label>
              <input
                type="url"
                placeholder="e.g. https://city.gov/agendas/rss"
                value={newSourceUrl}
                onChange={(e) => onNewSourceUrlChange(e.target.value)}
                required
                id="input-new-source-url"
              />
            </div>

            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Classification Type</label>
              <select value={newSourceType} onChange={(e) => onNewSourceTypeChange(e.target.value)} id="select-new-source-type">
                <option value="primary_record">Primary Record (Agendas, budgets, public notices)</option>
                <option value="official_comm">Official Communication (Press releases, announcements)</option>
                <option value="community_signal">Community Signal (Local forums, neighborhood boards)</option>
                <option value="media_lead">Media Lead (Newspapers, regional feeds)</option>
              </select>
            </div>

            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Source Tier</label>
              <select value={newSourceTier} onChange={(e) => onNewSourceTierChange(e.target.value)} id="select-new-source-tier">
                <option value="official_record">Official Record</option>
                <option value="news_reporting">News Reporting</option>
                <option value="community_signal">Community Signal</option>
              </select>
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
        <div className="modal-overlay" id="modal-bulk-import">
          <div className="modal-content" style={{ maxWidth: "600px", width: "90%", display: "flex", flexDirection: "column" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 style={{ margin: 0 }}>Bulk Import Sources</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => onShowBulkImportModalChange(false)}>Close</button>
            </div>
            
            <form onSubmit={onBulkImport} style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="form-bulk-import">
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Default Classification Type</label>
                <select value={bulkImportType} onChange={(e) => onBulkImportTypeChange(e.target.value)}>
                  <option value="primary_record">Primary Record (Agendas, budgets, public notices)</option>
                  <option value="official_comm">Official Communication (Press releases, announcements)</option>
                  <option value="community_signal">Community Signal (Local forums, neighborhood boards)</option>
                  <option value="media_lead">Media Lead (Newspapers, regional feeds)</option>
                </select>
              </div>

              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>
                  Source List (one per line)
                </label>
                <p className="help-text" style={{ margin: "0 0 0.5rem 0", fontSize: "0.8rem" }}>
                  Paste a list of URLs. You can optionally prefix with a name, e.g.,<br />
                  <code>Brighton Council, https://brightonco.gov/agenda</code><br />
                  If only a URL is pasted, we will automatically extract the name from its domain.
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

              <div style={{ display: "flex", justifyContent: "flex-end", gap: "1rem", marginTop: "1rem" }}>
                <button className="btn btn-secondary" type="button" onClick={() => onShowBulkImportModalChange(false)} disabled={bulkImportLoading}>
                  Cancel
                </button>
                <button className="btn btn-primary" type="submit" disabled={bulkImportLoading} id="btn-submit-bulk-import">
                  {bulkImportLoading ? "Importing..." : "Import List"}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Discovery Modal */}
      {showDiscoveryModal && (
        <div className="modal-overlay" id="modal-discovery">
          <div className="modal-content" style={{ maxWidth: "800px", width: "90%", maxHeight: "85vh", display: "flex", flexDirection: "column" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 style={{ margin: 0 }}>Town Setup & Source Auto-Discovery</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => onShowDiscoveryModalChange(false)}>Close</button>
            </div>
            
            <form onSubmit={onRunDiscovery} style={{ display: "flex", gap: "1rem", marginBottom: "1.5rem" }} id="form-run-discovery">
              <div style={{ flex: 1 }}>
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
                  <div className="animate-spin" style={{ display: "inline-block", border: "4px solid rgba(255,255,255,0.1)", borderTop: "4px solid var(--color-primary)", borderRadius: "50%", width: "40px", height: "40px", marginBottom: "1rem" }} />
                  <p>Searching DuckDuckGo for agendas, subreddits, library calendars, and local news...</p>
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
                    Select the feeds you want to import. We recommend keeping the primary record portals and your town's local newspaper or subreddit checked.
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
                            return (
                              <label key={cIdx} style={{ display: "flex", alignItems: "flex-start", gap: "0.75rem", cursor: "pointer", fontSize: "0.9rem" }}>
                                <input
                                  type="checkbox"
                                  checked={isChecked}
                                  onChange={() => onToggleDiscoveredSource(cand)}
                                  style={{ marginTop: "0.25rem" }}
                                />
                                <div>
                                  <div style={{ fontWeight: 600 }}>{cand.name}</div>
                                  <a href={cand.url} target="_blank" rel="noreferrer" style={{ fontSize: "0.8rem", color: "var(--color-primary)", wordBreak: "break-all" }}>
                                    {cand.url}
                                  </a>
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
          </div>
        </div>
      )}
    </div>
  );
};
