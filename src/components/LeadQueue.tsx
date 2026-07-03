// src/components/LeadQueue.tsx
import React, { useEffect, useState } from "react";
import { RefreshCw, Play, Trash2, Info, ChevronRight, Search, AlertTriangle, ShieldCheck } from "lucide-react";
import { Lead, Draft } from "../ipc";
import { DailyScanResults } from "./DailyScanResults";

interface LeadQueueProps {
  leads: Lead[];
  drafts: Draft[];
  loading: boolean;
  latestScanId?: number | null;
  selectedLeadId?: number | null;
  filter?: string;
  sourceCount?: number;
  onGoToSources?: () => void;
  onSelect: (id: number, lead?: Lead) => void;
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
  sourceCount,
  onGoToSources,
  onSelect,
  onSyncList,
  onIngest,
  onDailyScan,
  onOpenDraftEditor,
  onOpenCorrectionModal,
  onDeleteDraft
}) => {
  const [queueSubTab, setQueueSubTab] = useState<"leads" | "drafts" | "scan">("leads");
  const [filterText, setFilterText] = useState<string>(filter);
  const [sortBy, setSortBy] = useState<"risk" | "confidence" | "date">("date");
  const [draftStatusFilter, setDraftStatusFilter] = useState<string>("all");

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
      case "needs_verification": return "warning";
      case "killed": return "offline";
      case "corrected": return "info";
      default: return "warning";
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case "draft_generated": return "Drafting";
      case "ready_to_review": return "Ready for review";
      case "ready_to_publish": return "Approved for publishing";
      case "needs_verification": return "Sent back / needs work";
      case "hold": return "On hold";
      case "killed": return "Cut";
      case "published": return "Published";
      case "corrected": return "Corrected";
      default: return status.replace(/_/g, " ");
    }
  };

  const getDispositionLabel = (disposition?: string) => {
    switch ((disposition ?? "review").toLowerCase()) {
      case "ready_to_draft": return "Ready to draft";
      case "needs_verification": return "Needs verification";
      case "background": return "Background";
      case "watch": return "Watch";
      default: return "Editor review";
    }
  };

  const getDispositionColor = (disposition?: string) => {
    switch ((disposition ?? "review").toLowerCase()) {
      case "ready_to_draft": return "success";
      case "needs_verification": return "warning";
      case "background":
      case "watch": return "neutral";
      default: return "info";
    }
  };

  const highRiskCount = leads.filter((lead) => lead.risk_level === "high").length;
  const filteredDrafts = drafts.filter((draft) => draftStatusFilter === "all" || draft.status === draftStatusFilter);
  const recurrenceLabel = (count?: number) => {
    if (count === undefined || count === null || count < 1) return null;
    return count === 1 ? "Seen before" : `Seen ${count} times before`;
  };
  const leadNeedsDraftCaution = (lead: Lead) => {
    const disposition = (lead.disposition ?? "review").toLowerCase();
    const storyType = (lead.story_type ?? "").toLowerCase();
    const novelty = lead.novelty_score ?? 0;
    return Boolean(
      (lead.recurrence_count ?? 0) > 0 ||
      storyType === "background" ||
      storyType === "watch" ||
      storyType === "verification" ||
      disposition === "background" ||
      disposition === "watch" ||
      disposition === "needs_verification" ||
      (novelty > 0 && novelty <= 2)
    );
  };
  const draftByLeadId = new Map<number, Draft>();
  for (const draft of drafts) {
    if (draft.lead_id && !draftByLeadId.has(draft.lead_id)) {
      draftByLeadId.set(draft.lead_id, draft);
    }
  }

  useEffect(() => {
    if (!latestScanId && queueSubTab === "scan") {
      setQueueSubTab("leads");
    }
  }, [latestScanId, queueSubTab]);

  return (
    <div>
      <div className="page-header">
        <div className="page-title">
          <h1>Story Queue</h1>
          <p>Civic signals your sources surfaced, ready to verify and turn into plain-language stories.</p>
        </div>
        <div className="btn-group">
          <button className="btn btn-secondary" onClick={onSyncList} disabled={loading} id="btn-sync-list">
            <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
            Refresh
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

      <div className="stat-grid">
        <div className="stat-card stat-blue">
          <span>New leads</span>
          <strong>{leads.length}</strong>
        </div>
        <button
          className="stat-card stat-amber stat-card-button"
          type="button"
          onClick={() => setQueueSubTab("drafts")}
          aria-label={`Open ${drafts.length} drafts`}
        >
          <span>In drafting</span>
          <strong>{drafts.length}</strong>
        </button>
        <div className="stat-card stat-red">
          <span>High priority</span>
          <strong>{highRiskCount}</strong>
        </div>
        <div className="stat-card stat-green">
          <span>Sources</span>
          <strong>{sourceCount ?? 0}</strong>
        </div>
      </div>

      <div className="desk-banner">
        <ShieldCheck size={22} />
        <div>
          <strong>Local-first workflow</strong>
          <span>Scan sources, draft from linked evidence, run guardrails, then publish a static paper.</span>
        </div>
      </div>

      <div className="queue-tabs">
        <button
          className={`queue-tab ${queueSubTab === "leads" ? "active" : ""}`}
          onClick={() => setQueueSubTab("leads")}
          id="queue-tab-leads"
        >
          Leads <span className="badge badge-neutral">{leads.length}</span>
        </button>
        <button
          className={`queue-tab ${queueSubTab === "drafts" ? "active" : ""}`}
          onClick={() => setQueueSubTab("drafts")}
          id="queue-tab-drafts"
        >
          Drafts <span className="badge badge-neutral">{drafts.length}</span>
        </button>
        {latestScanId && (
          <button
            className={`queue-tab ${queueSubTab === "scan" ? "active" : ""}`}
            onClick={() => setQueueSubTab("scan")}
            id="queue-tab-scan-results"
          >
            Scan results <span className="badge badge-neutral">latest</span>
          </button>
        )}
      </div>

      {queueSubTab === "scan" && latestScanId ? (
        <DailyScanResults scanId={latestScanId} onRunScan={onDailyScan} />
      ) : queueSubTab === "leads" ? (
        <div>
          {/* Filter and Sort controls */}
          <div className="card queue-filter-card">
            <div style={{ display: "flex", alignItems: "center", gap: "0.5rem", flexGrow: 1 }}>
              <Search size={18} style={{ color: "var(--text-secondary)" }} />
              <label htmlFor="leads-filter-input" className="sr-only">Filter leads</label>
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
              <label htmlFor="leads-sort-select" style={{ marginRight: "0.5rem", fontSize: "0.9rem", fontWeight: 600 }}>Sort by:</label>
              <select 
                value={sortBy} 
                onChange={(e) => setSortBy(e.target.value as any)}
                style={{ padding: "0.5rem", width: "150px" }}
                id="leads-sort-select"
              >
                <option value="date">Date Created</option>
                <option value="risk">Risk Level</option>
                <option value="confidence">Confidence</option>
              </select>
            </div>
          </div>

          <div className="lead-grid">
            {sortedLeads.length === 0 ? (
              sourceCount === 0 ? (
                // UX-M5 / QA-M1: with zero sources, "Scrape & Detect" is a no-op.
                // Point the user at the real next step — add a source — instead.
                <div className="card text-center" style={{ gridColumn: "1 / -1", padding: "3rem" }}>
                  <Info size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
                  <h3>Add your first source</h3>
                  <p className="help-text" style={{ marginBottom: "1rem" }}>
                    The Civic Desk scans the feeds and record portals you add. Add a source to start finding local story leads.
                  </p>
                  {onGoToSources && (
                    <button className="btn btn-primary" onClick={onGoToSources} id="btn-empty-go-to-sources">
                      Go to Sources
                    </button>
                  )}
                </div>
              ) : (
                <div className="card text-center" style={{ gridColumn: "1 / -1", padding: "3rem" }}>
                  <Info size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
                  <h3>No story leads yet</h3>
                  <p className="help-text">Run Daily Scan to check your watched sources and build the first editor packet.</p>
                </div>
              )
            ) : (
              sortedLeads.map((lead) => {
                const existingDraft = lead.id ? draftByLeadId.get(lead.id) : undefined;
                const disposition = (lead.disposition ?? "review").toLowerCase();
                const draftLabel = existingDraft
                  ? "Open draft"
                  : disposition === "needs_verification"
                    ? "Verify first"
                    : leadNeedsDraftCaution(lead)
                    ? "Draft anyway"
                    : "Draft";
                const openLeadOrDraft = () => {
                  if (existingDraft) {
                    onOpenDraftEditor(existingDraft);
                  } else if (lead.id) {
                    onSelect(lead.id, lead);
                  }
                };
                return (
                  <div
                    key={lead.id}
                    className={`card lead-card ${selectedLeadId === lead.id ? "selected-lead" : ""}`}
                    style={{ borderColor: selectedLeadId === lead.id ? "var(--accent-primary)" : undefined }}
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
                      {existingDraft && (
                        <span className={`badge badge-${getStatusColor(existingDraft.status)}`}>
                          Draft exists
                        </span>
                      )}
                      {lead.story_type && (
                        <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                          {lead.story_type}
                        </span>
                      )}
                      <span className={`badge badge-${getDispositionColor(lead.disposition)}`}>
                        {getDispositionLabel(lead.disposition)}
                      </span>
                      {recurrenceLabel(lead.recurrence_count) && (
                        <span className="badge badge-warning">
                          {recurrenceLabel(lead.recurrence_count)}
                        </span>
                      )}
                      <span className="help-text">{lead.detector_name}</span>
                    </div>
                    <h4 className="lead-why">{lead.why}</h4>
                    {lead.recurrence_note && (
                      <p className="help-text" style={{ margin: "0.75rem 0 0 0" }}>
                        <strong>Beat memory:</strong> {lead.recurrence_note}
                      </p>
                    )}
                    {(lead.novelty_score !== undefined || lead.novelty_reason) && (
                      <p className="help-text" style={{ margin: "0.75rem 0 0 0" }}>
                        {lead.novelty_score !== undefined && <>Novelty {lead.novelty_score}/5. </>}
                        {lead.novelty_reason}
                      </p>
                    )}
                    <div style={{ marginTop: "1rem" }}>
                      <span style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>
                        Confidence: <strong>{lead.confidence}</strong>
                      </span>
                    </div>
                  </div>
                  <div className="mt-2 text-right">
                    <button 
                      type="button"
                      className="btn btn-primary btn-sm lead-draft-action"
                      data-testid={`btn-draft-lead-${lead.id}`}
                      aria-label={`${draftLabel} ${lead.why}`}
                      onClick={(e) => {
                        e.stopPropagation();
                        openLeadOrDraft();
                      }}
                    >
                      {draftLabel} <ChevronRight size={14} />
                    </button>
                  </div>
                </div>
                );
              })
            )}
          </div>
        </div>
      ) : (
        <div className="card">
          <div className="queue-filter-card" style={{ marginBottom: "1rem" }}>
            <div>
              <label htmlFor="draft-status-filter" style={{ marginRight: "0.5rem", fontSize: "0.9rem", fontWeight: 600 }}>
                Draft status:
              </label>
              <select
                id="draft-status-filter"
                value={draftStatusFilter}
                onChange={(e) => setDraftStatusFilter(e.target.value)}
                style={{ padding: "0.5rem", width: "220px" }}
              >
                <option value="all">All drafts</option>
                <option value="draft_generated">Drafting</option>
                <option value="needs_verification">Sent back / needs work</option>
                <option value="ready_to_review">Ready for review</option>
                <option value="ready_to_publish">Approved for publishing</option>
                <option value="hold">On hold</option>
                <option value="killed">Cut</option>
              </select>
            </div>
          </div>
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
                      No drafts generated yet. Select a lead and choose Draft, Verify first, or Draft anyway to begin.
                    </td>
                  </tr>
                ) : filteredDrafts.length === 0 ? (
                  <tr>
                    <td colSpan={4} className="text-center" style={{ padding: "3rem" }}>
                      No drafts match this status filter.
                    </td>
                  </tr>
                ) : (
                  filteredDrafts.map((draft) => (
                    <tr key={draft.id} data-testid={`draft-row-${draft.id}`}>
                      <td>
                        <strong>{draft.title}</strong>
                        {draft.correction_note && (
                          <div style={{ fontSize: "0.75rem", color: "var(--color-warning)", marginTop: "2px", display: "flex", alignItems: "center", gap: "0.25rem" }}>
                            <AlertTriangle size={12} /> Correction Registered: {draft.correction_note.slice(0, 50)}...
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
                          {getStatusLabel(draft.status)}
                        </span>
                      </td>
                      <td>
                        <div className="btn-group">
                          <button className="btn btn-secondary btn-sm" onClick={() => onOpenDraftEditor(draft)} id={`btn-open-workbench-${draft.id}`}>
                            Open
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
