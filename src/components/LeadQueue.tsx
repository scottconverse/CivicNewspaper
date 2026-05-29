// src/components/LeadQueue.tsx
import React, { useState } from "react";
import { RefreshCw, Play, Trash2, Info, ChevronRight, Search } from "lucide-react";
import { Lead, Draft } from "../ipc";
import { DailyScanResults } from "./DailyScanResults";

interface LeadQueueProps {
  leads: Lead[];
  drafts: Draft[];
  loading: boolean;
  latestScanId?: number | null;
  selectedLeadId?: number | null;
  filter?: string;
  onSelect: (id: number) => void;
  onSyncList: () => void;
  onIngest: () => void;
  onDailyScan: () => void;
  onOpenDraftEditor: (draft: Draft) => void;
  onOpenCorrectionModal: (draftId: number) => void;
  onDeleteDraft: (id: number) => void;
}

export const LeadQueue: React.FC<LeadQueueProps> = ({
  leads,
  drafts,
  loading,
  latestScanId,
  selectedLeadId,
  filter = "",
  onSelect,
  onSyncList,
  onIngest,
  onDailyScan,
  onOpenDraftEditor,
  onOpenCorrectionModal,
  onDeleteDraft
}) => {
  const [queueSubTab, setQueueSubTab] = useState<"leads" | "drafts">("leads");
  const [filterText, setFilterText] = useState<string>(filter);
  const [sortBy, setSortBy] = useState<"risk" | "confidence" | "date">("date");

  // Filtering leads based on filter prop and local filter text
  const filteredLeads = leads.filter(lead => {
    const text = (lead.why + " " + lead.detector_name).toLowerCase();
    const searchString = filterText.toLowerCase();
    return text.includes(searchString);
  });

  // Sorting leads
  const sortedLeads = [...filteredLeads].sort((a, b) => {
    if (sortBy === "risk") {
      const rank: Record<string, number> = { high: 3, med: 2, low: 1 };
      return (rank[b.risk_level] || 0) - (rank[a.risk_level] || 0);
    } else if (sortBy === "confidence") {
      const rank: Record<string, number> = { high: 3, med: 2, low: 1 };
      return (rank[b.confidence] || 0) - (rank[a.confidence] || 0);
    } else {
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
    }
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "online": return "online";
      case "offline": return "offline";
      case "draft_generated": return "warning";
      case "ready_to_review": return "info";
      case "ready_to_publish": return "online";
      case "hold": return "warning";
      case "killed": return "offline";
      case "corrected": return "info";
      default: return "warning";
    }
  };

  return (
    <div>
      <div className="page-header">
        <div className="page-title">
          <h1>Daily Story Queue</h1>
          <p>Verify municipal leads, review drafted articles, and compile your local community gazette.</p>
        </div>
        <div className="btn-group">
          <button className="btn btn-secondary" onClick={onSyncList} disabled={loading} id="btn-sync-list">
            <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
            Sync List
          </button>
          <button className="btn btn-secondary" onClick={onDailyScan} disabled={loading} id="btn-daily-scan">
            <Play size={16} />
            Daily Scan
          </button>
          <button className="btn btn-primary" onClick={onIngest} disabled={loading} id="btn-scrape-detect">
            <Play size={16} />
            Scrape & Detect
          </button>
        </div>
      </div>

      {latestScanId && (
        <DailyScanResults scanId={latestScanId} onRunScan={onDailyScan} />
      )}

      <div className="queue-tabs">
        <button
          className={`queue-tab ${queueSubTab === "leads" ? "active" : ""}`}
          onClick={() => setQueueSubTab("leads")}
          id="queue-tab-leads"
        >
          Generated Leads <span className="badge badge-neutral">{leads.length}</span>
        </button>
        <button
          className={`queue-tab ${queueSubTab === "drafts" ? "active" : ""}`}
          onClick={() => setQueueSubTab("drafts")}
          id="queue-tab-drafts"
        >
          Editorial Workbench <span className="badge badge-neutral">{drafts.length}</span>
        </button>
      </div>

      {queueSubTab === "leads" ? (
        <div>
          {/* Filter and Sort controls */}
          <div className="card" style={{ display: "flex", gap: "1rem", alignItems: "center", padding: "1rem", marginBottom: "1rem" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "0.5rem", flexGrow: 1 }}>
              <Search size={18} style={{ color: "var(--text-secondary)" }} />
              <input
                type="text"
                placeholder="Filter leads..."
                value={filterText}
                onChange={(e) => setFilterText(e.target.value)}
                style={{ padding: "0.5rem", border: "1px solid var(--border-color)", borderRadius: "var(--radius-sm)" }}
                id="leads-filter-input"
              />
            </div>
            <div>
              <label style={{ marginRight: "0.5rem", fontSize: "0.9rem", fontWeight: 600 }}>Sort by:</label>
              <select 
                value={sortBy} 
                onChange={(e) => setSortBy(e.target.value as any)}
                style={{ padding: "0.5rem", width: "150px" }}
              >
                <option value="date">Date Created</option>
                <option value="risk">Risk Level</option>
                <option value="confidence">Confidence</option>
              </select>
            </div>
          </div>

          <div className="lead-grid">
            {sortedLeads.length === 0 ? (
              <div className="card text-center" style={{ gridColumn: "1 / -1", padding: "3rem" }}>
                <Info size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
                <h3>No unlinked leads available</h3>
                <p className="help-text">Click "Scrape & Detect" above to scrape primary sources and trigger OSINT alerts.</p>
              </div>
            ) : (
              sortedLeads.map((lead) => (
                <div 
                  key={lead.id} 
                  className={`card lead-card ${selectedLeadId === lead.id ? "selected-lead" : ""}`}
                  onClick={() => lead.id && onSelect(lead.id)}
                  style={{ cursor: "pointer", border: selectedLeadId === lead.id ? "2px solid var(--accent-primary)" : "" }}
                  data-testid={`lead-card-${lead.id}`}
                >
                  <div>
                    <div className="lead-header">
                      <span className={`badge ${
                        lead.risk_level === "high" ? "badge-error" : 
                        lead.risk_level === "med" ? "badge-warning" : "badge-info"
                      }`}>
                        Risk: {lead.risk_level}
                      </span>
                      <span className="help-text">{lead.detector_name}</span>
                    </div>
                    <h4 className="lead-why">{lead.why}</h4>
                    <div style={{ marginTop: "1rem" }}>
                      <span style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>
                        Confidence: <strong>{lead.confidence}</strong>
                      </span>
                    </div>
                  </div>
                  <div className="mt-2 text-right">
                    <button 
                      className="btn btn-secondary btn-sm"
                      onClick={(e) => {
                        e.stopPropagation();
                        if (lead.id) onSelect(lead.id);
                      }}
                    >
                      Draft Article <ChevronRight size={14} />
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      ) : (
        <div className="card">
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Title</th>
                  <th>Format</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {drafts.length === 0 ? (
                  <tr>
                    <td colSpan={4} className="text-center" style={{ padding: "3rem" }}>
                      No drafts generated yet. Select a Lead and click "Draft Article" to begin.
                    </td>
                  </tr>
                ) : (
                  drafts.map((draft) => (
                    <tr key={draft.id} data-testid={`draft-row-${draft.id}`}>
                      <td>
                        <strong>{draft.title}</strong>
                        {draft.correction_note && (
                          <div style={{ fontSize: "0.75rem", color: "var(--color-warning)", marginTop: "2px" }}>
                            ⚠️ Correction Registered: {draft.correction_note.slice(0, 50)}...
                          </div>
                        )}
                      </td>
                      <td>
                        <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                          {draft.format}
                        </span>
                      </td>
                      <td>
                        <span className={`badge badge-${getStatusColor(draft.status)}`}>
                          {draft.status.replace(/_/g, " ")}
                        </span>
                      </td>
                      <td>
                        <div className="btn-group">
                          <button className="btn btn-secondary btn-sm" onClick={() => onOpenDraftEditor(draft)} id={`btn-open-workbench-${draft.id}`}>
                            Open Workbench
                          </button>
                          <button className="btn btn-secondary btn-sm" onClick={() => onOpenCorrectionModal(draft.id!)} id={`btn-correction-${draft.id}`}>
                            Correction
                          </button>
                          <button 
                            className="btn btn-danger btn-sm" 
                            onClick={(e) => {
                              e.stopPropagation();
                              onDeleteDraft(draft.id!);
                            }}
                            id={`btn-delete-draft-${draft.id}`}
                            aria-label="Delete draft"
                          >
                            <Trash2 size={12} />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};
