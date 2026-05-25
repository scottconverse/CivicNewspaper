import React from 'react';

export interface DailyScanLead {
  id?: number;
  run_id: number;
  rank: number;
  tier: string;
  headline: string;
  details: string;
  source?: string;
  url?: string;
  confidence?: string;
  action?: string;
  beat?: string;
}

interface Props {
  leads: DailyScanLead[];
  onOpenWorkbench: (id: number) => void;
}

export default function DailyScanResults({ leads, onOpenWorkbench }: Props) {
  return (
    <div className="daily-scan-results">
      <h2>Daily Scan Results</h2>
      {leads.map(lead => (
        <div key={lead.id || lead.headline} className="lead-card">
          <h3>{lead.headline}</h3>
          <span className="badge">{lead.tier}</span>
          <p>{lead.details}</p>
          <button onClick={() => lead.id && onOpenWorkbench(lead.id)}>Open in Workbench</button>
        </div>
      ))}
    </div>
  );
}
