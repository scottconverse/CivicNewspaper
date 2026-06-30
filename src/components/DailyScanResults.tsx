import React, { useCallback, useEffect, useState } from "react";
import { DailyScanLead, listDailyScanLeads, openExternalUrl, toUserMessage } from "../ipc";

interface Props {
  scanId: number;
  onRunScan?: () => void;
}

function clean(value?: string | null): string | null {
  const text = value?.trim();
  return text ? text : null;
}

function priorityLabel(value?: string | null): string {
  const priority = clean(value)?.toLowerCase();
  if (priority === "high") return "High priority";
  if (priority === "medium" || priority === "med") return "Medium priority";
  if (priority === "low") return "Low priority";
  return "Needs review";
}

function priorityClass(value?: string | null): string {
  const priority = clean(value)?.toLowerCase();
  if (priority === "high") return "badge-warning";
  if (priority === "medium" || priority === "med") return "badge-info";
  if (priority === "low") return "badge-neutral";
  return "badge-info";
}

function dispositionLabel(value?: string | null): string {
  const disposition = clean(value)?.toLowerCase();
  if (disposition === "ready_to_draft") return "Ready to draft";
  if (disposition === "needs_verification") return "Needs verification";
  if (disposition === "background") return "Background";
  if (disposition === "watch") return "Watch";
  return "Editor review";
}

function dispositionClass(value?: string | null): string {
  const disposition = clean(value)?.toLowerCase();
  if (disposition === "ready_to_draft") return "badge-success";
  if (disposition === "needs_verification") return "badge-warning";
  if (disposition === "background" || disposition === "watch") return "badge-neutral";
  return "badge-info";
}

function sourceContext(lead: DailyScanLead): string {
  const sourceName = clean(lead.source_name);
  const sourceType = clean(lead.source_type);
  if (sourceName && sourceType) return `${sourceName} / ${sourceType.replace(/_/g, " ")}`;
  if (sourceName) return sourceName;
  if (lead.source_id !== undefined && lead.source_id !== null) return `Watched source #${lead.source_id}`;
  return "Aggregated across watched sources";
}

export const DailyScanResults: React.FC<Props> = ({ scanId, onRunScan }) => {
  const [leads, setLeads] = useState<DailyScanLead[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [linkError, setLinkError] = useState<string | null>(null);

  const load = useCallback(() => {
    let mounted = true;
    setLoading(true);
    setError(null);
    listDailyScanLeads(scanId)
      .then(data => {
        if (mounted) {
          setLeads(data);
          setLoading(false);
        }
      })
      .catch(err => {
        if (mounted) {
          setError(toUserMessage(err));
          setLoading(false);
        }
      });
    return () => { mounted = false; };
  }, [scanId]);

  useEffect(() => load(), [load]);

  const handleOpenOriginalSource = async (url: string, event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    setLinkError(null);
    try {
      await openExternalUrl(url);
    } catch (err) {
      setLinkError(toUserMessage(err));
    }
  };

  if (loading) {
    return (
      <div className="card mt-4" data-testid="daily-scan-results-loading" style={{ marginTop: '1rem', padding: '1rem' }} aria-busy="true">
        <h3>Scan Results for Scan #{scanId}</h3>
        <div aria-hidden="true">
          {[0, 1, 2].map(i => (
            <div
              key={i}
              className="skeleton-line"
              style={{
                height: '1rem',
                marginBottom: '0.75rem',
                borderRadius: '4px',
                background: 'var(--border-color)',
                width: i === 2 ? '60%' : '100%',
              }}
            />
          ))}
        </div>
        <span className="sr-only">Loading scan results...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="card mt-4" data-testid="daily-scan-results-error" style={{ marginTop: '1rem', padding: '1rem' }}>
        <h3>Scan Results for Scan #{scanId}</h3>
        <p className="error-text">Couldn't load scan results: {error}</p>
        <button className="btn btn-secondary btn-sm" onClick={load} data-testid="daily-scan-retry">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="card mt-4" data-testid="daily-scan-results" style={{ marginTop: '1rem', padding: '1rem' }}>
      <h3>Scan Results for Scan #{scanId}</h3>
      {leads.length === 0 ? (
        <div data-testid="daily-scan-empty">
          <p>No new leads found in this scan.</p>
          {onRunScan && (
            <button className="btn btn-secondary btn-sm" onClick={onRunScan} data-testid="daily-scan-run-again">
              Run scan again
            </button>
          )}
        </div>
      ) : (
        <>
        {linkError && <p className="error-text">Couldn't open original source: {linkError}</p>}
        <ul style={{ listStyleType: 'none', padding: 0 }}>
          {leads.map((lead, idx) => (
            <li key={idx} style={{ marginBottom: '1rem', paddingBottom: '1rem', borderBottom: '1px solid var(--border-color)' }}>
              <article className="scan-lead-card" data-testid="daily-scan-lead-card">
                <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
                  <div>
                    <p className="eyebrow" style={{ marginBottom: "0.35rem" }}>{sourceContext(lead)}</p>
                    <h4 style={{ margin: '0 0 0.5rem 0' }}>{lead.title}</h4>
                  </div>
                  <div style={{ display: "flex", gap: "0.4rem", flexWrap: "wrap", justifyContent: "flex-end" }}>
                    {clean(lead.story_type) && (
                      <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                        {lead.story_type}
                      </span>
                    )}
                    <span className={`badge ${dispositionClass(lead.disposition)}`}>{dispositionLabel(lead.disposition)}</span>
                    <span className={`badge ${priorityClass(lead.priority)}`}>{priorityLabel(lead.priority)}</span>
                  </div>
                </div>

                <p style={{ margin: '0 0 0.75rem 0' }}>{lead.summary ?? "No summary available for this lead."}</p>
                {(clean(lead.what_changed) || clean(lead.publishability_note) || lead.novelty !== undefined) && (
                  <div className="scan-quality-panel" style={{ display: "grid", gap: "0.35rem", margin: "0 0 0.85rem 0" }}>
                    {clean(lead.what_changed) && (
                      <p className="help-text" style={{ margin: 0 }}>
                        <strong>Why now:</strong> {clean(lead.what_changed)}
                      </p>
                    )}
                    {lead.novelty !== undefined && lead.novelty !== null && (
                      <p className="help-text" style={{ margin: 0 }}>
                        <strong>Novelty:</strong> {lead.novelty}/5
                      </p>
                    )}
                    {clean(lead.publishability_note) && (
                      <p className="help-text" style={{ margin: 0 }}>
                        <strong>Before publishing:</strong> {clean(lead.publishability_note)}
                      </p>
                    )}
                  </div>
                )}

                <div style={{ display: "grid", gap: "0.65rem", marginBottom: "0.85rem" }}>
                  <div>
                    <strong>Why this was flagged</strong>
                    <p className="help-text" style={{ margin: "0.2rem 0 0 0" }}>
                      {clean(lead.why_flagged) ?? "The scan found evidence that may deserve an editor's review."}
                    </p>
                  </div>
                  <div>
                    <strong>Suggested next step</strong>
                    <p className="help-text" style={{ margin: "0.2rem 0 0 0" }}>
                      {clean(lead.suggested_next_step) ?? "Open the source, confirm the details, then decide whether this should become a story lead."}
                    </p>
                  </div>
                </div>

                <div className="flex-between">
                  {lead.original_url ? (
                    <a href={lead.original_url} onClick={(event) => handleOpenOriginalSource(lead.original_url, event)}>Open source and review</a>
                  ) : (
                    <span className="help-text">No original source URL</span>
                  )}
                  {lead.source_id === undefined || lead.source_id === null ? (
                    <span className="badge badge-info" data-testid="aggregated-badge">Aggregated across sources</span>
                  ) : (
                    <span className="badge badge-neutral">Source #{lead.source_id}</span>
                  )}
                </div>
              </article>
            </li>
          ))}
        </ul>
        </>
      )}
    </div>
  );
};
