// src/components/Workbench.tsx
import React, { useState, useEffect } from "react";
import { CheckCircle, AlertTriangle, Info, FileText } from "lucide-react";
import { Lead, Draft, EvidenceItem, GuardrailsReport, plainLanguageRewrite, pressFreedomLegalReview } from "../ipc";
import { Modal } from "./Modal";

type DiffRow = { text: string; type: "same" | "removed" | "added" };

// Line-level diff via longest-common-subsequence so the modal can highlight
// exactly which lines the rewrite dropped (left, red) or introduced (right,
// green). The two panes scroll independently, so unequal lengths are fine.
export function computeLineDiff(
  original: string,
  rewritten: string
): { left: DiffRow[]; right: DiffRow[] } {
  const a = original.split("\n");
  const b = rewritten.split("\n");
  const n = a.length;
  const m = b.length;
  const lcs: number[][] = Array.from({ length: n + 1 }, () =>
    new Array(m + 1).fill(0)
  );
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      lcs[i][j] =
        a[i] === b[j]
          ? lcs[i + 1][j + 1] + 1
          : Math.max(lcs[i + 1][j], lcs[i][j + 1]);
    }
  }
  const left: DiffRow[] = [];
  const right: DiffRow[] = [];
  let i = 0;
  let j = 0;
  while (i < n && j < m) {
    if (a[i] === b[j]) {
      left.push({ text: a[i], type: "same" });
      right.push({ text: b[j], type: "same" });
      i++;
      j++;
    } else if (lcs[i + 1][j] >= lcs[i][j + 1]) {
      left.push({ text: a[i], type: "removed" });
      i++;
    } else {
      right.push({ text: b[j], type: "added" });
      j++;
    }
  }
  while (i < n) {
    left.push({ text: a[i], type: "removed" });
    i++;
  }
  while (j < m) {
    right.push({ text: b[j], type: "added" });
    j++;
  }
  return { left, right };
}

interface WorkbenchProps {
  selectedLead: Lead | null;
  selectedDraft: Draft | null;
  drafts?: Draft[];
  evidenceList: EvidenceItem[];
  guardrailsReport: GuardrailsReport | null;
  ollamaOnline: boolean;
  manualLlmMode: boolean;
  draftFormat: string;
  onDraftFormatChange: (val: string) => void;
  customSystemPrompt: string;
  onCustomSystemPromptChange: (val: string) => void;
  generatingText: boolean;
  onGenerateText: () => void;
  onCancelDraftWizard: () => void;
  onSaveDraftEditor: () => void;
  onCloseWorkbench: () => void;
  onOpenDraftEditor?: (draft: Draft) => void;
  onDeleteDraft: (id: number) => void;
  onDecision: (status: string) => void;
  onApprovePublish?: (overrideReason?: string) => void;
  onKillStory?: () => void;
  isGeneratingSocial: boolean;
  socialPackResult: string;
  onSocialPackResultChange: (val: string) => void;
  onGenerateSocial: () => void;
  onUpdateDraftTitle: (title: string) => void;
  onUpdateDraftContent: (content: string) => void;
  firstAmendmentAdvisorEnabled?: boolean;
}

function guardrailInstruction(issue: any): { title: string; action: string } {
  const category = String(issue.category ?? "").toLowerCase();
  const message = String(issue.message ?? "");
  if (category.includes("citation")) {
    return {
      title: "Consider adding a source link",
      action: "This passage looks like a factual claim without a linked source. Add a link if one exists, rewrite the claim, or leave it for the editor to approve.",
    };
  }
  if (category.includes("accusatory")) {
    return {
      title: "Attribute or soften accusatory wording",
      action: "Use careful attribution such as according to the filed record, add a citation, or rewrite the claim so it does not state an unsupported accusation.",
    };
  }
  if (category.includes("legal")) {
    return {
      title: "Use legally careful language",
      action: "Use alleged, charged with, or attributed language unless the public record supports a final finding.",
    };
  }
  if (category.includes("verbatim")) {
    return {
      title: "Rewrite source wording",
      action: "This wording is too close to the source. Rewrite it in your own words or format it as a direct quote with a citation.",
    };
  }
  return {
    title: "Review this guardrail issue",
    action: message || "Check this passage before approval.",
  };
}

export const Workbench: React.FC<WorkbenchProps> = ({
  selectedLead,
  selectedDraft,
  drafts = [],
  evidenceList,
  guardrailsReport,
  ollamaOnline,
  manualLlmMode,
  draftFormat,
  onDraftFormatChange,
  customSystemPrompt,
  onCustomSystemPromptChange,
  generatingText,
  onGenerateText,
  onCancelDraftWizard,
  onSaveDraftEditor,
  onCloseWorkbench,
  onOpenDraftEditor,
  onDeleteDraft,
  onDecision,
  onApprovePublish,
  onKillStory,
  isGeneratingSocial,
  socialPackResult,
  onSocialPackResultChange,
  onGenerateSocial,
  onUpdateDraftTitle,
  onUpdateDraftContent,
  firstAmendmentAdvisorEnabled = true
}) => {
  const [isRewriting, setIsRewriting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [rewritePreview, setRewritePreview] = useState<string | null>(null);
  const [attested, setAttested] = useState(false);
  const [showOverrideModal, setShowOverrideModal] = useState(false);
  const [overrideReason, setOverrideReason] = useState("");
  const [pressFreedomReview, setPressFreedomReview] = useState("");
  const [isRunningPressFreedomReview, setIsRunningPressFreedomReview] = useState(false);

  useEffect(() => {
    setError(null);
    setRewritePreview(null);
    setAttested(false);
    setShowOverrideModal(false);
    setOverrideReason("");
    setPressFreedomReview("");
  }, [selectedDraft?.id]);

  // Error-severity issues = words the newsroom marked as especially sensitive.
  // They ask for an override note, but the editor still decides.
  const errorIssues = guardrailsReport?.issues.filter((i) => i.severity === "error") ?? [];

  const handleApproveClick = () => {
    if (errorIssues.length > 0) {
      setShowOverrideModal(true);
    } else {
      onApprovePublish?.();
    }
  };

  const confirmOverride = () => {
    const reason = overrideReason.trim();
    setShowOverrideModal(false);
    if (reason) {
      onApprovePublish?.(reason);
    } else {
      onApprovePublish?.();
    }
  };

  const handlePressFreedomReview = async () => {
    if (!selectedDraft?.id) return;
    setIsRunningPressFreedomReview(true);
    setError(null);
    try {
      const review = await pressFreedomLegalReview(selectedDraft.id);
      setPressFreedomReview(review);
    } catch (error: any) {
      console.error("Failed to run press-freedom review:", error);
      setError(error?.message || String(error));
    } finally {
      setIsRunningPressFreedomReview(false);
    }
  };

  const handleInsertCitation = (evidenceId: number) => {
    const textarea = document.getElementById("draft-editor-textarea") as HTMLTextAreaElement;
    if (!textarea || !selectedDraft) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const text = textarea.value;

    const selectionText = text.substring(start, end);
    const citationText = selectionText 
      ? `[${selectionText}](evidence:${evidenceId})` 
      : `[Evidence #${evidenceId}](evidence:${evidenceId})`;

    const newContent = text.substring(0, start) + citationText + text.substring(end);
    onUpdateDraftContent(newContent);

    // Reset selection range after state update
    setTimeout(() => {
      textarea.focus();
      textarea.setSelectionRange(start + citationText.length, start + citationText.length);
    }, 50);
  };

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

  // If drafting from a Lead
  if (selectedLead && !selectedDraft) {
    return (
      <div className="wizard-container card" id="draft-wizard-panel" tabIndex={-1}>
        <h2>Drafting Article</h2>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          Lead: <strong>{selectedLead.why}</strong>
        </p>

        <div className="draft-wizard-top-actions">
          <button className="btn btn-primary" onClick={onGenerateText} disabled={generatingText || (!ollamaOnline && !manualLlmMode)} id="btn-generate-draft-top">
            {generatingText ? "Generating Draft..." : "Generate Draft"}
          </button>
          <button className="btn btn-secondary" onClick={onCancelDraftWizard} disabled={generatingText} id="btn-cancel-draft-top">
            Cancel
          </button>
        </div>

        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
          <div>
            <label htmlFor="select-draft-format" style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Article Format</label>
            <select value={draftFormat} onChange={(e) => onDraftFormatChange(e.target.value)} id="select-draft-format">
              <option value="brief">Brief (Under 200 words summary)</option>
              <option value="watch">Watch Alert (Highlights specific public safety or procurement issues)</option>
              <option value="explainer">Explainer (Detailed review of a policy or decision background)</option>
              <option value="investigation">Investigation (Traces spending and connections behind a story)</option>
              <option value="opinion">Editorial / Opinion Piece (Presents a structured argument)</option>
            </select>
          </div>

          <div>
            <label htmlFor="textarea-custom-instructions" style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Custom Guidelines / Instructions (Optional)</label>
            <textarea
              placeholder="e.g. Focus specifically on the budget numbers, keep tone highly objective..."
              value={customSystemPrompt}
              onChange={(e) => onCustomSystemPromptChange(e.target.value)}
              style={{ height: "100px" }}
              id="textarea-custom-instructions"
            />
          </div>

          <div className="card" style={{ background: "var(--accent-light)", marginTop: "1rem" }}>
            <h4>Linked Sources ({evidenceList.length})</h4>
            <div style={{ maxHeight: "150px", overflowY: "auto", marginTop: "0.5rem" }}>
              {evidenceList.length === 0 ? (
                // UX-M3: the wizard previously showed an empty linked-source
                // box with no explanation. Mirror the editor's empty state.
                <p className="help-text" style={{ display: "flex", alignItems: "flex-start", gap: "0.4rem", margin: 0 }}>
                  <AlertTriangle size={14} style={{ flexShrink: 0, marginTop: "0.15rem", color: "var(--color-warning)" }} />
                  No source documents are linked to this lead yet. You can still generate a working draft; mark anything that needs verification before publishing.
                </p>
              ) : (
                evidenceList.map((item, idx) => (
                  <div key={idx} style={{ padding: "0.25rem 0", fontSize: "0.85rem", borderBottom: "1px solid var(--border-color)", display: "flex", alignItems: "center", gap: "0.25rem" }}>
                    <FileText size={14} /> <em>"{item.excerpt.slice(0, 100)}..."</em>
                  </div>
                ))
              )}
            </div>
          </div>

          {!ollamaOnline && !manualLlmMode && (
            <div className="error-text" id="ollama-offline-warning" style={{ display: "flex", alignItems: "center", gap: "0.25rem" }}>
              <AlertTriangle size={14} /> The local AI service is offline. Open the "AI Model" tab to set it up, or use "Manual Mode" in settings.
            </div>
          )}
        </div>
      </div>
    );
  }

  // If editing an existing Draft
  if (selectedDraft) {
    return (
      <div id="workbench-editor-panel" tabIndex={-1}>
        <div className="page-header" style={{ marginBottom: "1rem" }}>
          <div className="page-title">
            <h1>Story Workbench</h1>
            <p>Modify drafted content, review guardrail warnings, and link source material when it helps the editor.</p>
          </div>
          <div className="btn-group">
            <button className="btn btn-secondary" onClick={onCloseWorkbench} id="btn-close-workbench">
              Back to Queue
            </button>
            <button className="btn btn-secondary" onClick={onSaveDraftEditor} disabled={generatingText} id="btn-save-draft">
              Save Draft
            </button>
            <button className="btn btn-danger" onClick={() => onDeleteDraft(selectedDraft.id!)} id="btn-delete-workbench-draft">
              Delete
            </button>
          </div>
        </div>

        {/* Guardrails Check Report Alert */}
        {guardrailsReport && (
          <div 
            className={`guardrails-panel ${guardrailsReport.is_clean ? "clean" : "issues"} ${!guardrailsReport.is_clean ? "flagged-claim" : ""}`}
            id="guardrails-report-card"
            data-testid="guardrails-report-card"
          >
            <div className="flex-between">
              <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
                {guardrailsReport.is_clean && guardrailsReport.issues.length === 0 ? (
                  <CheckCircle size={18} />
                ) : (
                  <AlertTriangle
                    size={18}
                    style={{ color: guardrailsReport.is_clean ? "var(--color-warning)" : "var(--color-error)" }}
                  />
                )}
                <strong style={{ color: guardrailsReport.is_clean ? undefined : "var(--color-error)" }}>
                  {!guardrailsReport.is_clean
                    ? "Review before publishing - these are warnings, not software vetoes."
                    : guardrailsReport.issues.length > 0
                      ? "Advisory warnings â€” these do not block publishing."
                      : "Pre-publication guardrails passed: no issues detected."}
                </strong>
              </div>
              <span style={{ fontSize: "0.8rem", textTransform: "uppercase" }}>
                {guardrailsReport.issues.length} issue(s)
              </span>
            </div>
            {guardrailsReport.issues.length > 0 && (
              <div style={{ marginTop: "0.5rem" }} id="guardrails-issues-list">
                {guardrailsReport.issues.map((issue: any, idx: number) => {
                  const isError = issue.severity === "error";
                  const instruction = guardrailInstruction(issue);
                  return (
                    <div
                      key={idx}
                      className={`guardrail-issue ${issue.severity}`}
                      style={{ color: isError ? "var(--color-error)" : "var(--text-secondary)" }}
                    >
                      <AlertTriangle
                        size={14}
                        style={{
                          marginRight: "0.25rem",
                          verticalAlign: "middle",
                          color: isError ? "var(--color-error)" : "var(--color-warning)",
                        }}
                      />
                      <strong>{isError ? "Review carefully" : "Warns"}</strong> - {instruction.title}
                      <p className="help-text" style={{ margin: "0.25rem 0 0 1.4rem" }}>{instruction.action}</p>
                      <details style={{ margin: "0.35rem 0 0 1.4rem" }}>
                        <summary>Technical details</summary>
                        <p className="help-text" style={{ margin: "0.25rem 0 0 0" }}>[{String(issue.category).replace(/_/g, " ")}] {issue.message}</p>
                      </details>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        )}

        {firstAmendmentAdvisorEnabled && (
          <div className="card" id="first-amendment-advisor-card" style={{ padding: "1rem", marginBottom: "1rem", background: "var(--accent-light)" }}>
            <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
              <div>
                <h3 className="card-title" style={{ marginBottom: "0.35rem" }}>Press-freedom / legal-risk advisor</h3>
                <p className="help-text" style={{ margin: 0 }}>
                  Advisory only. Run this when you want risk notes, verification paths, and wording options. The publisher decides what to investigate, edit, hold, or publish.
                </p>
              </div>
              <button
                className="btn btn-secondary btn-sm"
                id="btn-press-freedom-review"
                onClick={handlePressFreedomReview}
                disabled={isRunningPressFreedomReview || !selectedDraft.id || !selectedDraft.content}
                title={!selectedDraft.content ? "Add draft text before running the advisory review" : "Run an optional press-freedom and legal-risk review"}
              >
                {isRunningPressFreedomReview ? "Reviewing..." : "Run Advisor"}
              </button>
            </div>
            <ul className="help-text" style={{ margin: "0.5rem 0 0 1.2rem" }}>
              <li>Distinguish verified facts, allegations, opinion, and unanswered questions.</li>
              <li>Use extra care with private individuals, minors, medical details, addresses, and active legal matters.</li>
              <li>Keep notes on source reliability and why publishing serves the community.</li>
            </ul>
            {pressFreedomReview && (
              <textarea
                className="editor-textarea"
                id="textarea-press-freedom-review"
                aria-label="Press-freedom legal-risk advisor result"
                value={pressFreedomReview}
                onChange={(e) => setPressFreedomReview(e.target.value)}
                style={{ height: "220px", marginTop: "0.75rem", fontSize: "0.85rem", fontFamily: "var(--font-mono)" }}
              />
            )}
          </div>
        )}

        <div className="workbench-container">
          {/* Editor Pane (Left) */}
          <div className="editor-pane">
            <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
              <label style={{ fontWeight: 600, fontSize: "0.9rem" }}>Story Title</label>
              <input
                type="text"
                value={selectedDraft.title}
                onChange={(e) => onUpdateDraftTitle(e.target.value)}
                style={{ fontSize: "1.2rem", fontWeight: "600", fontFamily: "var(--font-serif)" }}
                id="input-draft-title"
              />
            </div>

            <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem", flexGrow: 1 }}>
              <div className="flex-between">
                <label style={{ fontWeight: 600, fontSize: "0.9rem" }}>Article Body (Markdown)</label>
                <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
                  {error && (
                    <span className="error-text" role="alert" style={{ fontSize: "0.85rem", display: "flex", alignItems: "center", gap: "0.25rem" }}>
                      <AlertTriangle size={14} /> {error}
                    </span>
                  )}
                  <button
                    className="btn btn-secondary btn-sm"
                    disabled={isRewriting || !selectedDraft.content}
                    title={!selectedDraft.content ? "Add draft text before rewriting" : "Rewrite the draft in plainer language"}
                    id="btn-plain-language-rewrite"
                    onClick={async () => {
                      setIsRewriting(true);
                      setError(null);
                      try {
                        const rewrite = await plainLanguageRewrite(selectedDraft.content, selectedDraft.format);
                        setRewritePreview(rewrite);
                      } catch (error: any) {
                        console.error("Failed to rewrite draft:", error);
                        setError(error?.message || String(error));
                      } finally {
                        setIsRewriting(false);
                      }
                    }}
                  >
                    {isRewriting ? "Rewriting..." : "Plain Language Rewrite"}
                  </button>
                  <span className="help-text">Highlight text and click "Cite" in evidence pane to link.</span>
                </div>
              </div>
              <textarea
                id="draft-editor-textarea"
                className="editor-textarea"
                value={selectedDraft.content}
                onChange={(e) => onUpdateDraftContent(e.target.value)}
              />
            </div>

            <div className="card" style={{ padding: "1rem", background: "var(--bg-sidebar)" }}>
              <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                <div className="flex-between">
                  <div>
                    <span style={{ fontSize: "0.85rem", color: "var(--text-secondary)" }}>Current Status: </span>
                    <strong className={`badge badge-${getStatusColor(selectedDraft.status)}`} style={{ fontSize: "0.85rem" }}>
                      {selectedDraft.status.replace(/_/g, " ")}
                    </strong>
                  </div>
                  <div className="btn-group">
                    <button className="btn btn-secondary btn-sm" onClick={() => onDecision("hold")} id="btn-status-hold">
                      Hold
                    </button>
                    <button className="btn btn-danger btn-sm" onClick={() => (onKillStory ? onKillStory() : onDecision("killed"))} id="btn-status-kill">
                      Kill Story
                    </button>
                    <button
                      className="btn btn-primary btn-sm"
                      onClick={handleApproveClick}
                      title={
                        !attested
                          ? "Approve and record editorial responsibility"
                          : errorIssues.length > 0
                            ? "This story has sensitive warnings - you'll be asked to confirm an override note"
                            : "Approve this story for publishing"
                      }
                      id="btn-status-publish"
                    >
                      Approve for Static Publish
                    </button>
                  </div>
                </div>
                <label
                  htmlFor="chk-attest"
                  style={{ display: "flex", alignItems: "flex-start", gap: "0.5rem", fontSize: "0.85rem", cursor: "pointer", color: "var(--text-secondary)" }}
                >
                  <input
                    id="chk-attest"
                    type="checkbox"
                    checked={attested}
                    onChange={(e) => setAttested(e.target.checked)}
                    style={{ marginTop: "0.15rem" }}
                  />
                  <span>
                    I have reviewed this story and take responsibility for publishing it.
                    <em> This reminder is for the editor; the app does not make the publication decision.</em>
                  </span>
                </label>
              </div>
            </div>

            {/* Social Media Generator */}
            <div className="card" style={{ padding: "1rem", marginTop: "1rem" }}>
              <div className="flex-between" style={{ marginBottom: "0.5rem" }}>
                <h4 style={{ margin: 0 }}>Social Media Promo Pack</h4>
                <button 
                  className="btn btn-secondary btn-sm" 
                  onClick={onGenerateSocial}
                  disabled={isGeneratingSocial || (!ollamaOnline && !manualLlmMode)}
                  id="btn-generate-social"
                >
                  {isGeneratingSocial ? "Generating..." : "Generate Posts"}
                </button>
              </div>
              {socialPackResult && (
                <textarea
                  className="editor-textarea"
                  style={{ height: "150px", marginTop: "0.5rem", fontSize: "0.85rem", fontFamily: "var(--font-serif)" }}
                  value={socialPackResult}
                  onChange={(e) => onSocialPackResultChange(e.target.value)}
                  id="textarea-social-result"
                />
              )}
              {!socialPackResult && !isGeneratingSocial && (
                <p className="help-text" style={{ fontSize: "0.85rem", margin: 0 }}>
                  Generate optimized posts for Twitter/X, Facebook, and Reddit based on the current draft.
                </p>
              )}
            </div>
          </div>

          {/* Evidence Pane (Right) */}
          <div className="evidence-pane" id="evidence-citation-pane">
            <h4 style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem" }}>Linked Sources</h4>
            {evidenceList.length === 0 ? (
              <p className="help-text">No source documents are linked to this story draft.</p>
            ) : (
              evidenceList.map((item) => (
                <div key={item.id} className="evidence-item" data-testid={`evidence-item-${item.id}`}>
                  <div className="evidence-header">
                    <span>Citation ID: #{item.id}</span>
                    <span>Fetched: {new Date(item.fetched_at).toLocaleDateString()}</span>
                  </div>
                  <div className="evidence-excerpt">"{item.excerpt}"</div>
                  <div className="text-right">
                    <button 
                      className="btn btn-secondary btn-sm" 
                      onClick={() => handleInsertCitation(item.id!)}
                      id={`btn-cite-${item.id}`}
                    >
                      Link Citation
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

        {rewritePreview !== null && (() => {
          const { left, right } = computeLineDiff(selectedDraft.content || "", rewritePreview);
          return (
            <Modal
              id="rewrite-diff-modal"
              labelledBy="rewrite-diff-modal-title"
              contentClassName="modal-content-wide"
              onClose={() => setRewritePreview(null)}
            >
                <h3 id="rewrite-diff-modal-title" style={{ marginTop: 0 }}>Review Plain Language Rewrite</h3>
                <p className="help-text" style={{ marginTop: 0 }}>
                  Compare the original draft with the AI rewrite. Removed lines are marked on the left, new lines on the right. Accept to replace the draft, or reject to keep the original.
                </p>
                <div className="diff-container">
                  <div className="diff-pane" id="diff-pane-original">
                    <div className="diff-pane-header">Original</div>
                    {left.map((row, idx) => (
                      <span key={idx} className={row.type === "removed" ? "diff-line diff-line-removed" : "diff-line"}>
                        <span className="diff-gutter" aria-hidden="true">{row.type === "removed" ? "-" : " "}</span>
                        {row.text || " "}
                      </span>
                    ))}
                  </div>
                  <div className="diff-pane" id="diff-pane-rewrite">
                    <div className="diff-pane-header">Plain Language Rewrite</div>
                    {right.map((row, idx) => (
                      <span key={idx} className={row.type === "added" ? "diff-line diff-line-added" : "diff-line"}>
                        <span className="diff-gutter" aria-hidden="true">{row.type === "added" ? "+" : " "}</span>
                        {row.text || " "}
                      </span>
                    ))}
                  </div>
                </div>
                <div className="flex-between" style={{ marginTop: "1rem" }}>
                  <button className="btn btn-secondary" id="btn-reject-rewrite" onClick={() => setRewritePreview(null)}>
                    Reject
                  </button>
                  <button
                    className="btn btn-primary"
                    id="btn-accept-rewrite"
                    onClick={() => {
                      onUpdateDraftContent(rewritePreview);
                      setRewritePreview(null);
                    }}
                  >
                    Accept Rewrite
                  </button>
                </div>
            </Modal>
          );
        })()}

        {showOverrideModal && (
          <Modal id="override-modal" labelledBy="override-modal-title" onClose={() => setShowOverrideModal(false)}>
            <h3 id="override-modal-title" style={{ marginTop: 0, color: "var(--color-error)" }}>
              Publish with sensitive warnings?
            </h3>
            <p className="help-text" style={{ marginTop: 0 }}>
              This story has {errorIssues.length} high-concern issue(s) from your newsroom's guardrail list.
              The app will not veto the editor, but your reason is recorded with the story.
            </p>
            <ul style={{ fontSize: "0.85rem", margin: "0 0 1rem 0", paddingLeft: "1.2rem" }}>
              {errorIssues.map((iss: any, idx: number) => (
                <li key={idx}>
                  [{iss.category.replace(/_/g, " ")}] {iss.message}
                </li>
              ))}
            </ul>
            <label htmlFor="override-reason" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>
              Editor note (optional)
            </label>
            <textarea
              id="override-reason"
              value={overrideReason}
              onChange={(e) => setOverrideReason(e.target.value)}
              placeholder="e.g. Verified against the filed indictment; charge language is accurate and attributed."
              style={{ width: "100%", height: "90px" }}
            />
            <div className="flex-between" style={{ marginTop: "1rem" }}>
              <button className="btn btn-secondary" onClick={() => setShowOverrideModal(false)} id="btn-override-cancel">
                Cancel
              </button>
              <button
                className="btn btn-danger"
                onClick={confirmOverride}
                id="btn-override-confirm"
              >
                Publish anyway (logged)
              </button>
            </div>
          </Modal>
        )}
      </div>
    );
  }

  return (
    <div className="card workbench-picker-card" id="workbench-draft-picker">
      <div className="workbench-picker-heading">
        <Info size={36} style={{ color: "var(--text-muted)" }} />
        <div>
          <h3>No lead or draft selected</h3>
          <p className="help-text">Open a draft below, or return to the queue to start a new story from a lead.</p>
        </div>
      </div>

      {drafts.length > 0 ? (
        <div className="draft-picker-list" aria-label="Workbench draft picker">
          {drafts.map((draft) => (
            <button
              type="button"
              key={draft.id ?? draft.title}
              className="draft-picker-row"
              onClick={() => onOpenDraftEditor?.(draft)}
              id={draft.id ? `btn-workbench-picker-open-${draft.id}` : undefined}
            >
              <span>
                <strong>{draft.title || "Untitled draft"}</strong>
                <small>{draft.format} - {draft.status.replace(/_/g, " ")}</small>
              </span>
              <span className={`badge badge-${getStatusColor(draft.status)}`}>Open</span>
            </button>
          ))}
        </div>
      ) : (
        <p className="help-text">No drafts exist yet. Start from a lead in the Story Queue.</p>
      )}

      <button className="btn btn-secondary" type="button" onClick={onCloseWorkbench} id="btn-workbench-empty-back">
        Back to Story Queue
      </button>
    </div>
  );
};

