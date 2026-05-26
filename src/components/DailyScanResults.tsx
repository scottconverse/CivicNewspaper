import React, { useEffect, useState } from "react";
import { DailyScanLead, listDailyScanLeads } from "../ipc";

interface Props {
  scanId: number;
}

export const DailyScanResults: React.FC<Props> = ({ scanId }) => {
  const [leads, setLeads] = useState<DailyScanLead[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;
    setLoading(true);
    listDailyScanLeads(scanId)
      .then(data => {
        if (mounted) {
          setLeads(data);
          setLoading(false);
        }
      })
      .catch(err => {
        if (mounted) {
          setError(String(err));
          setLoading(false);
        }
      });
    return () => { mounted = false; };
  }, [scanId]);

  if (loading) return <div>Loading scan results...</div>;
  if (error) return <div className="error-text">Error loading results: {error}</div>;

  return (
    <div className="card mt-4" data-testid="daily-scan-results" style={{ marginTop: '1rem', padding: '1rem' }}>
      <h3>Scan Results for Scan #{scanId}</h3>
      {leads.length === 0 ? (
        <p>No leads found in this scan.</p>
      ) : (
        <ul style={{ listStyleType: 'none', padding: 0 }}>
          {leads.map((lead, idx) => (
            <li key={idx} style={{ marginBottom: '1rem', paddingBottom: '1rem', borderBottom: '1px solid var(--border-color)' }}>
              <h4 style={{ margin: '0 0 0.5rem 0' }}>{lead.title}</h4>
              <p style={{ margin: '0 0 0.5rem 0' }}>{lead.summary}</p>
              <div className="flex-between">
                <a href={lead.original_url} target="_blank" rel="noreferrer">Original Source</a>
                {lead.source_id === undefined || lead.source_id === null ? (
                  <span className="badge badge-info" data-testid="aggregated-badge">Aggregated across sources</span>
                ) : (
                  <span className="badge badge-secondary">Source ID: {lead.source_id}</span>
                )}
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};
