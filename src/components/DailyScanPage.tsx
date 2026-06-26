import React from "react";
import { Play, RefreshCw, ScanSearch, TrendingUp } from "lucide-react";
import { DailyScanResults } from "./DailyScanResults";
import { DailyScanProgress } from "../useApp";

interface DailyScanPageProps {
  latestScanId?: number | null;
  leadCount: number;
  draftCount: number;
  sourceCount: number;
  loading: boolean;
  ollamaOnline: boolean;
  dailyScanProgress?: DailyScanProgress | null;
  onRunScan: () => void;
  onRefresh: () => void;
}

function progressStageLabel(stage: string): string {
  switch (stage) {
    case "preflight":
      return "Checking setup";
    case "preparing":
      return "Preparing records";
    case "generating":
      return "Local AI review";
    case "parsing":
      return "Repairing response";
    case "saving":
      return "Saving leads";
    case "fallback":
      return "Building review packet";
    case "complete":
      return "Complete";
    case "failed":
      return "Needs attention";
    default:
      return stage.replace(/_/g, " ");
  }
}

export const DailyScanPage: React.FC<DailyScanPageProps> = ({
  latestScanId,
  leadCount,
  draftCount,
  sourceCount,
  loading,
  ollamaOnline,
  dailyScanProgress,
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

      {dailyScanProgress && (
        <section className="card" data-testid="daily-scan-progress" style={{ marginTop: "1rem" }}>
          <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
            <div>
              <p className="eyebrow">Scan progress</p>
              <h3 style={{ marginTop: 0 }}>{dailyScanProgress.message}</h3>
              <p className="help-text" style={{ marginBottom: 0 }}>
                {dailyScanProgress.model ? `Model: ${dailyScanProgress.model}. ` : ""}
                Evidence: {dailyScanProgress.evidence_count}.
                {" "}Saved leads: {dailyScanProgress.saved_leads}.
                {dailyScanProgress.batch_index && dailyScanProgress.batch_count
                  ? ` Batch ${dailyScanProgress.batch_index} of ${dailyScanProgress.batch_count}.`
                  : ""}
              </p>
              {dailyScanProgress.stage !== "complete" && dailyScanProgress.stage !== "failed" && (
                <p className="help-text" style={{ margin: "0.35rem 0 0 0" }}>
                  Local scans move in stages rather than a fake percent. It is normal for the AI review stage to take a while on CPU-only machines.
                </p>
              )}
            </div>
            <span className={`badge ${dailyScanProgress.stage === "failed" ? "badge-warning" : dailyScanProgress.stage === "complete" ? "badge-success" : "badge-info"}`}>
              {progressStageLabel(dailyScanProgress.stage)}
            </span>
          </div>
        </section>
      )}

      {latestScanId ? (
        <DailyScanResults scanId={latestScanId} onRunScan={onRunScan} />
      ) : (
        <div className="card empty-state">
          <ScanSearch size={36} />
          <h3>Start today's scan</h3>
          <p className="help-text">The results panel will fill with surfaced civic leads after the first run. If this is a fresh setup, add sources and run Scrape & Detect first so there are records to review.</p>
          <button className="btn btn-primary" onClick={onRunScan} disabled={loading}>
            <Play size={16} />
            Run Daily Scan
          </button>
        </div>
      )}
    </div>
  );
};
