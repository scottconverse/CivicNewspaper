import React from "react";
import { AlertTriangle, Eye, RefreshCw, ShieldCheck } from "lucide-react";
import type { CivicEntity, CivicIntelligenceSnapshot } from "../ipc";

interface DarkSignalDeskProps {
  intelligence: CivicIntelligenceSnapshot | null;
  loading: boolean;
  onRefresh: () => void;
  onCreateLead: (darkSignalId: number) => void;
}

const riskClass = (risk: string) => {
  if (risk === "high") return "badge-error";
  if (risk === "medium") return "badge-warning";
  return "badge-info";
};

const formatKind = (value: string) => value.replace(/_/g, " ");

const entityLabel = (entity: CivicEntity) => `${formatKind(entity.entity_type)}: ${entity.name}`;

export const DarkSignalDesk: React.FC<DarkSignalDeskProps> = ({
  intelligence,
  loading,
  onRefresh,
  onCreateLead,
}) => {
  const signals = intelligence?.dark_signals ?? [];
  const observations = intelligence?.observations ?? [];
  const sourceScores = intelligence?.source_scores ?? [];
  const entities = intelligence?.entities ?? [];
  const highRisk = signals.filter(signal => signal.risk_level === "high").length;

  return (
    <div id="dark-signal-desk">
      <div className="page-header">
        <div className="page-title">
          <h1>Dark Signal Desk</h1>
          <p>Community and weak-signal leads ranked for review, never hidden from the editor.</p>
        </div>
        <button className="btn btn-secondary" type="button" onClick={onRefresh} disabled={loading} id="btn-refresh-dark-signals">
          <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
          Refresh
        </button>
      </div>

      <div className="stat-grid">
        <div className="stat-card stat-red">
          <span>High risk</span>
          <strong>{highRisk}</strong>
        </div>
        <div className="stat-card stat-blue">
          <span>Total signals</span>
          <strong>{signals.length}</strong>
        </div>
        <div className="stat-card stat-green">
          <span>Entities tracked</span>
          <strong>{entities.length}</strong>
        </div>
        <div className="stat-card stat-amber">
          <span>Sources scored</span>
          <strong>{sourceScores.length}</strong>
        </div>
      </div>

      <div className="desk-banner">
        <ShieldCheck size={22} />
        <div>
          <strong>Rank, explain, preserve</strong>
          <span>Tier C and community signals stay visible here for verification, but they are marked editor-review-only until confirmed.</span>
        </div>
      </div>

      {signals.length === 0 ? (
        <div className="card text-center" style={{ padding: "3rem" }}>
          <Eye size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
          <h3>No dark signals yet</h3>
          <p className="help-text">Add community, social, media, and official sources, then run Scrape & Detect. Signals will appear here with ranking and verification paths.</p>
        </div>
      ) : (
        <div className="dark-signal-grid">
          {signals.map(signal => (
            <article className="card dark-signal-card" key={signal.id ?? `${signal.origin}-${signal.created_at}`}>
              <div className="flex-between" style={{ gap: "1rem", alignItems: "flex-start" }}>
                <div>
                  <div className="lead-header">
                    <span className={`badge ${riskClass(signal.risk_level)}`}>{signal.risk_level} risk</span>
                    <span className="badge badge-neutral">score {Math.round(signal.rank_score)}</span>
                    <span className="badge badge-warning">{signal.evidence_policy.replace(/_/g, " ")}</span>
                  </div>
                  <h3 className="card-title" style={{ marginTop: "0.55rem" }}>{signal.title}</h3>
                </div>
                <AlertTriangle size={22} style={{ color: "var(--color-warning)", flexShrink: 0 }} />
              </div>
              <p>{signal.summary}</p>
              <div className="signal-detail-grid">
                <div>
                  <strong>Origin</strong>
                  <span>{signal.origin}</span>
                </div>
                <div>
                  <strong>Status</strong>
                  <span>{signal.publication_status.replace(/_/g, " ")}</span>
                </div>
                <div>
                  <strong>Tier</strong>
                  <span>{signal.tier.replace(/_/g, " ")}</span>
                </div>
                <div>
                  <strong>Created</strong>
                  <span>{new Date(signal.created_at).toLocaleString()}</span>
                </div>
              </div>
              <div className="signal-section">
                <strong>Why it matters</strong>
                <p className="help-text">{signal.why_it_matters}</p>
              </div>
              <div className="signal-section">
                <strong>Verification path</strong>
                <p className="help-text">{signal.verification_path}</p>
              </div>
              {signal.entities.length > 0 && (
                <div className="entity-chip-list">
                  {signal.entities.map(entity => (
                    <span className="badge badge-info" key={`${signal.id}-${entity.entity_type}-${entity.normalized_name}`}>
                      {entityLabel(entity)}
                    </span>
                  ))}
                </div>
              )}
              {signal.id && (
                <div className="btn-group verification-actions">
                  <button
                    className="btn btn-primary btn-sm"
                    type="button"
                    onClick={() => onCreateLead(signal.id!)}
                  >
                    Create story lead
                  </button>
                </div>
              )}
            </article>
          ))}
        </div>
      )}

      <div className="intel-grid">
        <section className="card">
          <h3 className="card-title">Recent observations</h3>
          {observations.length === 0 ? (
            <p className="help-text">No observations recorded yet.</p>
          ) : (
            <div className="compact-list">
              {observations.slice(0, 8).map(obs => (
                <div className="compact-row" key={obs.id ?? `${obs.title}-${obs.observed_at}`}>
                  <strong>{obs.title}</strong>
                  <span>{formatKind(obs.observation_type)} - {new Date(obs.observed_at).toLocaleString()}</span>
                  {obs.diff_summary && <small>{obs.diff_summary}</small>}
                </div>
              ))}
            </div>
          )}
        </section>

        <section className="card">
          <h3 className="card-title">Source performance</h3>
          {sourceScores.length === 0 ? (
            <p className="help-text">No source scores yet.</p>
          ) : (
            <div className="compact-list">
              {sourceScores.slice(0, 8).map(score => (
                <div className="compact-row" key={score.source_id}>
                  <strong>{score.source_name}</strong>
                  <span>Reliability {Math.round(score.reliability_score)} - Usefulness {Math.round(score.usefulness_score)}</span>
                  <small>{score.new_items} new - {score.changed_items} changed - {score.dark_signal_hits} dark signals</small>
                </div>
              ))}
            </div>
          )}
        </section>
      </div>
    </div>
  );
};
