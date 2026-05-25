// src/components/Workbench.tsx
import React from "react";
import { CheckCircle, AlertTriangle, Info } from "lucide-react";
import { Lead, Draft, EvidenceItem, GuardrailsReport } from "../ipc";

interface WorkbenchProps {
  selectedLead: Lead | null;
  selectedDraft: Draft | null;
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
  onDeleteDraft: (id: number) => void;
  onDecision: (status: string) => void;
  isGeneratingSocial: boolean;
  socialPackResult: string;
  onSocialPackResultChange: (val: string) => void;
  onGenerateSocial: () => void;
  onUpdateDraftTitle: (title: string) => void;
  onUpdateDraftContent: (content: string) => void;
}

export const Workbench: React.FC<WorkbenchProps> = ({
  selectedLead,
  selectedDraft,
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
  onDeleteDraft,
  onDecision,
  isGeneratingSocial,
  socialPackResult,
  onSocialPackResultChange,
  onGenerateSocial,
  onUpdateDraftTitle,
  onUpdateDraftContent
}) => {

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
      <div className="wizard-container card" id="draft-wizard-panel">
        <h2>Drafting Article from Evidence</h2>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          Lead: <strong>{selectedLead.why}</strong>
        </p>

        <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
          <div>
            <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Article Format</label>
            <select value={draftFormat} onChange={(e) => onDraftFormatChange(e.target.value)} id="select-draft-format">
              <option value="brief">Brief (Under 200 words summary)</option>
              <option value="watch">Watch Alert (Highlights specific public safety or procurement issues)</option>
              <option value="explainer">Explainer (Detailed review of a policy or decision background)</option>
              <option value="investigation">Investigation (Highlights specific money Trails/risk linkages)</option>
              <option value="opinion">Editorial / Opinion Piece (Presents a structured argument)</option>
            </select>
          </div>

          <div>
            <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Custom Guidelines / Instructions (Optional)</label>
            <textarea
              placeholder="e.g. Focus specifically on the budget numbers, keep tone highly objective..."
              value={customSystemPrompt}
              onChange={(e) => onCustomSystemPromptChange(e.target.value)}
              style={{ height: "100px" }}
              id="textarea-custom-instructions"
            />
          </div>

          <div className="card" style={{ background: "var(--accent-light)", marginTop: "1rem" }}>
            <h4>Linked Records ({evidenceList.length})</h4>
            <div style={{ maxHeight: "150px", overflowY: "auto", marginTop: "0.5rem" }}>
              {evidenceList.map((item, idx) => (
                <div key={idx} style={{ padding: "0.25rem 0", fontSize: "0.85rem", borderBottom: "1px solid var(--border-color)" }}>
                  📄 <em>"{item.excerpt.slice(0, 100)}..."</em>
                </div>
              ))}
            </div>
          </div>

          <div className="flex-between" style={{ marginTop: "1.5rem" }}>
            <button className="btn btn-secondary" onClick={onCancelDraftWizard} disabled={generatingText} id="btn-cancel-draft">
              Cancel
            </button>
            <button className="btn btn-primary" onClick={onGenerateText} disabled={generatingText || (!ollamaOnline && !manualLlmMode)} id="btn-generate-draft">
              {generatingText ? "Generating Draft..." : "Generate Draft"}
            </button>
          </div>
          
          {!ollamaOnline && !manualLlmMode && (
            <div className="error-text" id="ollama-offline-warning">
              ⚠️ Local Ollama is offline. Open the "Ollama Wizard" tab to set up or use "Manual Mode" settings.
            </div>
          )}
        </div>
      </div>
    );
  }

  // If editing an existing Draft
  if (selectedDraft) {
    return (
      <div id="workbench-editor-panel">
        <div className="page-header" style={{ marginBottom: "1rem" }}>
          <div className="page-title">
            <h1>Story Editorial Workbench</h1>
            <p>Modify drafted content, review guardrails violations, and link citations to raw public evidence.</p>
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
                {guardrailsReport.is_clean ? (
                  <CheckCircle size={18} />
                ) : (
                  <AlertTriangle size={18} style={{ color: "var(--color-warning)" }} />
                )}
                <strong>
                  {guardrailsReport.is_clean 
                    ? "Pre-publication Guardrails Passed: No major issues detected." 
                    : "Verification Issues Detected:"}
                </strong>
              </div>
              <span style={{ fontSize: "0.8rem", textTransform: "uppercase" }}>
                {guardrailsReport.issues.length} issue(s)
              </span>
            </div>
            {!guardrailsReport.is_clean && (
              <div style={{ marginTop: "0.5rem" }} id="guardrails-issues-list">
                {guardrailsReport.issues.map((issue: any, idx: number) => (
                  <div key={idx} className={`guardrail-issue ${issue.severity}`}>
                    ⚠️ [Category: {issue.category.replace(/_/g, " ")}] {issue.message}
                  </div>
                ))}
              </div>
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
                <div style={{ display: "flex", gap: "1rem" }}>
                  <button 
                    className="btn btn-secondary btn-sm"
                    onClick={async () => {
                      if (!selectedDraft.content) return;
                      try {
                        import('../ipc').then(async ({ plainLanguageRewrite }) => {
                          const rewrite = await plainLanguageRewrite(selectedDraft.content);
                          onUpdateDraftContent(rewrite);
                        });
                      } catch (e) {
                        console.error(e);
                      }
                    }}
                  >
                    Plain Language Rewrite
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
                  <button className="btn btn-danger btn-sm" onClick={() => onDecision("killed")} id="btn-status-kill">
                    Kill Story
                  </button>
                  <button className="btn btn-primary btn-sm" onClick={() => onDecision("ready_to_publish")} id="btn-status-publish">
                    Approve for Static Publish
                  </button>
                </div>
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
            <h4 style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem" }}>Linked Public Records</h4>
            {evidenceList.length === 0 ? (
              <p className="help-text">No evidence documents are linked to this story draft.</p>
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
      </div>
    );
  }

  return (
    <div className="card text-center" style={{ padding: "3rem" }}>
      <Info size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
      <h3>No lead or draft selected</h3>
      <p className="help-text">Open a lead draft wizard or open a draft from the queue.</p>
    </div>
  );
};
