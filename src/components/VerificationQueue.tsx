import React from "react";
import { CheckCircle2, ClipboardCheck, RefreshCw, ShieldAlert } from "lucide-react";
import type { VerificationQueueSnapshot, VerificationTask } from "../ipc";

interface VerificationQueueProps {
  queue: VerificationQueueSnapshot | null;
  loading: boolean;
  onRefresh: () => void;
  onStatusChange: (
    task: VerificationTask,
    status: VerificationTask["status"],
    resultSummary?: string
  ) => void;
  onCreateLead: (darkSignalId: number) => void;
}

const statusLabel = (status: string) => status.replace(/_/g, " ");
const taskTypeLabel = (value: string) => value.replace(/_/g, " ");

const statusClass = (status: string) => {
  if (status === "needs_human" || status === "blocked") return "badge-warning";
  if (status === "resolved" || status === "auto_checked") return "badge-success";
  return "badge-info";
};

const impactClass = (impact: string) => {
  if (impact === "high") return "badge-error";
  if (impact === "medium") return "badge-warning";
  return "badge-info";
};

export const VerificationQueue: React.FC<VerificationQueueProps> = ({
  queue,
  loading,
  onRefresh,
  onStatusChange,
  onCreateLead,
}) => {
  const tasks = queue?.tasks ?? [];
  const needsHuman = tasks.filter(task => task.status === "needs_human").length;
  const autoChecked = tasks.filter(task => task.status === "auto_checked").length;
  const resolved = tasks.filter(task => task.status === "resolved").length;

  return (
    <div id="verification-queue">
      <div className="page-header">
        <div className="page-title">
          <p className="eyebrow">Reporting workbench</p>
          <h1>Verification Queue</h1>
          <p>Turn signals and leads into concrete checks ranked by effort, impact, and reporting urgency.</p>
        </div>
        <button className="btn btn-secondary" type="button" onClick={onRefresh} disabled={loading} id="btn-refresh-verification">
          <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
          Refresh tasks
        </button>
      </div>

      <div className="stat-grid">
        <div className="stat-card stat-red">
          <span>Needs human</span>
          <strong>{needsHuman}</strong>
        </div>
        <div className="stat-card stat-green">
          <span>Auto-checked</span>
          <strong>{autoChecked}</strong>
        </div>
        <div className="stat-card stat-blue">
          <span>Resolved</span>
          <strong>{resolved}</strong>
        </div>
        <div className="stat-card stat-amber">
          <span>New this refresh</span>
          <strong>{queue?.generated_count ?? 0}</strong>
        </div>
      </div>

      <div className="desk-banner">
        <ShieldAlert size={22} />
        <div>
          <strong>Automate the obvious, leave judgment to the editor</strong>
          <span>Low-effort checks can be marked automatically, but the queue keeps every unresolved verification choice visible for human review.</span>
        </div>
      </div>

      {tasks.length === 0 ? (
        <div className="card text-center" style={{ padding: "3rem" }}>
          <ClipboardCheck size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
          <h3>No verification tasks yet</h3>
          <p className="help-text">Run source collection and review Dark Signals. This queue will turn those signals into concrete reporting checks.</p>
        </div>
      ) : (
        <div className="verification-task-list">
          {tasks.map(task => (
            <article className="card verification-task-card" key={task.id ?? `${task.title}-${task.created_at}`}>
              <div className="flex-between" style={{ gap: "1rem", alignItems: "flex-start" }}>
                <div>
                  <div className="lead-header">
                    <span className={`badge ${statusClass(task.status)}`}>{statusLabel(task.status)}</span>
                    <span className={`badge ${impactClass(task.impact_level)}`}>{task.impact_level} impact</span>
                    <span className="badge badge-neutral">{task.effort_level} effort</span>
                    <span className="badge badge-info">score {Math.round(task.rank_score)}</span>
                  </div>
                  <h3 className="card-title" style={{ marginTop: "0.55rem" }}>{task.title}</h3>
                  <p className="help-text">{taskTypeLabel(task.check_type)}</p>
                </div>
                {task.status === "resolved" ? (
                  <CheckCircle2 size={22} style={{ color: "var(--color-success)", flexShrink: 0 }} />
                ) : (
                  <ClipboardCheck size={22} style={{ color: "var(--color-info)", flexShrink: 0 }} />
                )}
              </div>

              <p>{task.description}</p>

              <div className="signal-detail-grid">
                <div>
                  <strong>Target</strong>
                  <span>{task.target_label || "General verification"}</span>
                </div>
                <div>
                  <strong>Links</strong>
                  <span>
                    {[
                      task.dark_signal_id ? `signal #${task.dark_signal_id}` : "",
                      task.lead_id ? `lead #${task.lead_id}` : "",
                      task.draft_id ? `draft #${task.draft_id}` : "",
                      task.entity_id ? `entity #${task.entity_id}` : "",
                    ].filter(Boolean).join(" / ") || "unlinked"}
                  </span>
                </div>
                <div>
                  <strong>Updated</strong>
                  <span>{new Date(task.updated_at).toLocaleString()}</span>
                </div>
                <div>
                  <strong>Source URL</strong>
                  <span>{task.target_url || "none"}</span>
                </div>
              </div>

              {task.result_summary && (
                <div className="signal-section">
                  <strong>Result</strong>
                  <p className="help-text">{task.result_summary}</p>
                </div>
              )}

              <div className="btn-group verification-actions">
                {task.status !== "resolved" && (
                  <button
                    className="btn btn-primary btn-sm"
                    type="button"
                    onClick={() => onStatusChange(task, "resolved", "Marked resolved by editor.")}
                  >
                    Mark resolved
                  </button>
                )}
                {task.dark_signal_id && !task.lead_id && (
                  <button
                    className="btn btn-secondary btn-sm"
                    type="button"
                    onClick={() => onCreateLead(task.dark_signal_id!)}
                  >
                    Create story lead
                  </button>
                )}
                {task.status !== "needs_human" && task.status !== "resolved" && (
                  <button
                    className="btn btn-secondary btn-sm"
                    type="button"
                    onClick={() => onStatusChange(task, "needs_human", "Needs human verification.")}
                  >
                    Needs human
                  </button>
                )}
                {task.status !== "blocked" && task.status !== "resolved" && (
                  <button
                    className="btn btn-secondary btn-sm"
                    type="button"
                    onClick={() => onStatusChange(task, "blocked", "Blocked until more information is available.")}
                  >
                    Blocked
                  </button>
                )}
              </div>
            </article>
          ))}
        </div>
      )}
    </div>
  );
};
