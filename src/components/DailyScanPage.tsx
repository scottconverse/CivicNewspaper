import React from "react";
import { Play, RefreshCw, ScanSearch, TrendingUp } from "lucide-react";
import { DailyScanResults } from "./DailyScanResults";

interface DailyScanPageProps {
  latestScanId?: number | null;
  leadCount: number;
  draftCount: number;
  sourceCount: number;
  loading: boolean;
  ollamaOnline: boolean;
  onRunScan: () => void;
  onRefresh: () => void;
}

export const DailyScanPage: React.FC<DailyScanPageProps> = ({
  latestScanId,
  leadCount,
  draftCount,
  sourceCount,
  loading,
  ollamaOnline,
  onRunScan,
  onRefresh,
}) => {
  const cards = [
    { label: "Sources Watched", value: sourceCount, tone: "blue" },
    { label: "Open Leads", value: leadCount, tone: "amber" },
    { label: "Drafts in Desk", value: draftCount, tone: "green" },
    { label: "AI Status", value: ollamaOnline ? "Ready" : "Offline", tone: ollamaOnline ? "green" : "red" },
  ];

  return (
    <div>
      <div className="page-header">
        <div className="page-title">
          <p className="eyebrow">Morning editor packet</p>
          <h1>Daily Scan</h1>
          <p>Run a local AI pass across recent records and surface the leads that deserve an editor's look.</p>
        </div>
        <div className="btn-group">
          <button className="btn btn-secondary" onClick={onRefresh} disabled={loading}>
            <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
            Refresh
          </button>
          <button className="btn btn-primary" onClick={onRunScan} disabled={loading} id="btn-daily-scan-route">
            <Play size={16} />
            Run Daily Scan
          </button>
        </div>
      </div>

      <div className="stat-grid">
        {cards.map((card) => (
          <div className={`stat-card stat-${card.tone}`} key={card.label}>
            <span>{card.label}</span>
            <strong>{card.value}</strong>
          </div>
        ))}
      </div>

      <section className="scan-summary-card">
        <div>
          <span className="scan-icon"><ScanSearch size={22} /></span>
        </div>
        <div>
          <p className="eyebrow">AI brief</p>
          <h2>{latestScanId ? `Latest scan #${latestScanId}` : "No scan has been run yet"}</h2>
          <p>
            Civic Desk keeps this pass local, private, and evidence-first. Use it to find the handful of records
            worth turning into leads, then move the strongest ones into the Story Queue.
          </p>
        </div>
        <TrendingUp className="scan-watermark" size={88} aria-hidden="true" />
      </section>

      {latestScanId ? (
        <DailyScanResults scanId={latestScanId} onRunScan={onRunScan} />
      ) : (
        <div className="card empty-state">
          <ScanSearch size={36} />
          <h3>Start today's scan</h3>
          <p className="help-text">The results panel will fill with surfaced civic leads after the first run.</p>
          <button className="btn btn-primary" onClick={onRunScan} disabled={loading}>
            <Play size={16} />
            Run Daily Scan
          </button>
        </div>
      )}
    </div>
  );
};
