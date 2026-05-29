import React, { useCallback, useEffect, useState } from "react";
import { DailyScanLead, listDailyScanLeads, toUserMessage } from "../ipc";

interface Props {
  scanId: number;
  onRunScan?: () => void;
}

export const DailyScanResults: React.FC<Props> = ({ scanId, onRunScan }) => {
  const [leads, setLeads] = useState<DailyScanLead[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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
        <span className="sr-only">Loading scan results…</span>
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
        <ul style={{ listStyleType: 'none', padding: 0 }}>
          {leads.map((lead, idx) => (
            <li key={idx} style={{ marginBottom: '1rem', paddingBottom: '1rem', borderBottom: '1px solid var(--border-color)' }}>
              <h4 style={{ margin: '0 0 0.5rem 0' }}>{lead.title}</h4>
              <p style={{ margin: '0 0 0.5rem 0' }}>{lead.summary ?? "No summary available for this lead."}</p>
              <div className="flex-between">
                <a href={lead.original_url} target="_blank" rel="noopener noreferrer">Original Source</a>
                {lead.source_id === undefined || lead.source_id === null ? (
                  <span className="badge badge-info" data-testid="aggregated-badge">Aggregated across sources</span>
                ) : (
                  <span className="badge badge-neutral">Source ID: {lead.source_id}</span>
                )}
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};
