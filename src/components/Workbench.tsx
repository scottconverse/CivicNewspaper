// src/components/Workbench.tsx
import React, { useState, useEffect, useRef } from "react";
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
  onOpenAiSetup?: () => void;
  onSaveDraftEditor: () => void;
  onCloseWorkbench: () => void;
  onOpenDraftEditor?: (draft: Draft | number) => void;
  onDeleteDraft: (id: number) => void;
  onDecision: (status: string, reason?: string) => void;
  onApprovePublish?: (overrideReason?: string) => void;
  onKillStory?: () => void;
  onImproveForPublication?: () => Promise<void> | void;
  isGeneratingSocial: boolean;
  socialPackResult: string;
  onSocialPackResultChange: (val: string) => void;
  onGenerateSocial: () => void;
  onUpdateDraftTitle: (title: string) => void;
  onUpdateDraftContent: (content: string) => void;
  onUpdateDraftFormat?: (format: string) => void;
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

const TOPIC_STOPWORDS = new Set([
  "about", "after", "again", "against", "alerts", "also", "before", "being", "between", "city",
  "community", "contact", "could", "department", "departments", "during", "editor", "event",
  "events", "explore", "first", "from", "government", "have", "information", "into", "latest",
  "local", "longmont", "more", "news", "newsletter", "original", "program", "programs", "public",
  "reader", "residents", "review", "services", "should", "source", "sources", "story",
  "suggested", "that", "their", "there", "these", "this", "through", "under", "updates",
  "were", "where", "which", "while", "will", "with", "would"
]);

function topicText(text: string): string {
  const firstLine = text.split(/\n/)[0]?.trim() || text.trim();
  const beforeMetadata = firstLine
    .split(" Editor context:")[0]
    .split(" Suggested treatment:")[0]
    .trim();
  const beforeSummary = beforeMetadata.split(":")[0]?.trim() || beforeMetadata;
  if (topicTokens(beforeSummary).size >= 3) return beforeSummary;
  const afterSummary = beforeMetadata.includes(":") ? beforeMetadata.split(":").slice(1).join(":").trim() : "";
  return afterSummary && topicTokens(afterSummary).size >= 3 ? afterSummary : beforeMetadata;
}

function topicTokens(text: string): Set<string> {
  const normalized = text
    .toLowerCase()
    .replace(/[^a-z0-9\s]/g, " ")
    .split(/\s+/)
    .map((token) => token.trim())
    .filter(Boolean)
    .map((token) => {
      if (token.length > 6 && token.endsWith("ies")) return `${token.slice(0, -3)}y`;
      if (token.length > 5 && token.endsWith("ing")) return token.slice(0, -3);
      if (token.length > 5 && token.endsWith("ed")) return token.slice(0, -2);
      if (token.length > 5 && token.endsWith("s")) return token.slice(0, -1);
      return token;
    })
    .filter((token) => token.length >= 4 && !TOPIC_STOPWORDS.has(token) && !/^\d+$/.test(token));
  return new Set(normalized);
}

function evidenceAppearsTopicMatched(draft: Draft, evidenceList: EvidenceItem[]): boolean {
  if (evidenceList.length === 0) return true;
  const draftTokens = topicTokens(topicText(draft.title || ""));
  if (draftTokens.size === 0) return false;
  const required = draftTokens.size <= 3 ? Math.max(1, Math.min(2, draftTokens.size)) : draftTokens.size <= 5 ? 3 : Math.min(6, Math.max(4, Math.ceil(draftTokens.size / 3)));
  return evidenceList.some((item) => {
    const evidenceTokens = topicTokens(item.excerpt || "");
    let overlap = 0;
    draftTokens.forEach((token) => {
      if (evidenceTokens.has(token)) overlap += 1;
    });
    return overlap >= required && overlap / draftTokens.size >= 0.45;
  });
}

function getStoryQualityWarnings(draft: Draft, evidenceList: EvidenceItem[]): string[] {
  const warnings: string[] = [];
  const evidenceCount = evidenceList.length;
  const title = (draft.title || "").trim();
  const content = (draft.content || "").trim();
  const lower = content.toLowerCase();
  const paragraphCount = content
    .split(/\n{2,}/)
    .map((part) => part.trim())
    .filter(Boolean).length;

  if (!title || title.length < 8 || title.endsWith(".") || title.includes(": ")) {
    warnings.push("Headline may read like a note or summary. Use a concise reader-facing headline.");
  }
  if (/(editor_note|editor note:|source check:|reporting steps:|nut graf:|\[source needed\]|\[verification needed\]|body:)/i.test(content)) {
    warnings.push("Draft still contains reporter scaffolding. Remove newsroom-only labels before publishing.");
  }
  if (/approved during cleanroom mechanics test|despite quality warnings|see tester report|mechanics test|tester report/i.test(content)) {
    warnings.push("Draft body appears to be an editor/test note, not reader-facing article copy.");
  }
  if (evidenceCount === 0) {
    warnings.push("No source documents are linked. Treat this as a verification assignment until you attach or cite public source material.");
  }
  if (evidenceCount > 0 && !/evidence:\s*(?:\/\/)?\s*\d+/i.test(content)) {
    warnings.push("Linked sources exist, but the body has no inline evidence citations.");
  }
  if (paragraphCount < 2 && draft.format !== "brief") {
    warnings.push("Story body is very short for this format. Consider making it a brief or adding verified reporting.");
  }
  if (lower.includes("according to") === false && evidenceCount > 0) {
    warnings.push("No clear attribution phrase found. Attribute key facts to the source or rewrite more cautiously.");
  }
  if (!evidenceAppearsTopicMatched(draft, evidenceList)) {
    warnings.push("Linked source documents may not match this story topic. Attach the correct source material or rewrite the story around the linked sources.");
  }
  return warnings;
}

function getStaticPublishBlockers(draft: Draft, evidenceList: EvidenceItem[]): string[] {
  const blockers: string[] = [];
  const evidenceCount = evidenceList.length;
  const title = (draft.title || "").trim();
  const content = (draft.content || "").trim();
  const hasLead = draft.lead_id !== undefined && draft.lead_id !== null;
  const linkedEvidenceIds = new Set(
    evidenceList
      .map((item) => item.id)
      .filter((id): id is number => typeof id === "number")
      .map((id) => String(id))
  );
  const citedEvidenceIds = Array.from(content.matchAll(/evidence:\s*(?:\/\/)?\s*(\d+)/gi))
    .map((match) => match[1])
    .filter(Boolean);
  const unlinkedCitationIds = citedEvidenceIds.filter((id) => !linkedEvidenceIds.has(id));
  if (!content || content.split(/\s+/).filter(Boolean).length < 15) {
    blockers.push("The article body is empty or too short for a public page.");
  }
  if (content.length > 12000) {
    blockers.push("The article body is unusually large. Cut repeated/junk text before approval.");
  }
  if (/source check:|unlinked-evidence-\d+/i.test(content)) {
    blockers.push("The draft has disabled or unlinked evidence citations. Link the correct source before approval.");
  }
  if (unlinkedCitationIds.length > 0) {
    blockers.push(`The draft cites evidence ID(s) ${Array.from(new Set(unlinkedCitationIds)).join(", ")} that are not linked to this lead.`);
  }
  if (/approved during cleanroom mechanics test|despite quality warnings|see tester report|mechanics test|tester report/i.test(content)) {
    blockers.push("The body looks like an editor/test note, not public story copy.");
  }
  if (/editor context:|suggested treatment:|suggested next step:/i.test(title)) {
    blockers.push("The headline still contains lead metadata. Rewrite it as a reader-facing headline.");
  }
  if (hasLead && evidenceCount === 0) {
    blockers.push("This scanned-lead draft has no linked source documents.");
  }
  if (hasLead && evidenceCount > 0 && !/evidence:\s*(?:\/\/)?\s*\d+/i.test(content)) {
    blockers.push("This scanned-lead draft needs at least one inline evidence citation before approval.");
  }
  if (hasLead && evidenceCount > 0 && !evidenceAppearsTopicMatched(draft, evidenceList)) {
    blockers.push("This scanned-lead draft's linked source documents do not appear to match the story topic.");
  }
  return blockers;
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
  onOpenAiSetup,
  onSaveDraftEditor,
  onCloseWorkbench,
  onOpenDraftEditor,
  onDeleteDraft,
  onDecision,
  onApprovePublish,
  onKillStory,
  onImproveForPublication,
  isGeneratingSocial,
  socialPackResult,
  onSocialPackResultChange,
  onGenerateSocial,
  onUpdateDraftTitle,
  onUpdateDraftContent,
  onUpdateDraftFormat,
  firstAmendmentAdvisorEnabled = true
}) => {
  const generateDraftButtonRef = React.useRef<HTMLButtonElement | null>(null);
  const [isRewriting, setIsRewriting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [rewritePreview, setRewritePreview] = useState<string | null>(null);
  const [attested, setAttested] = useState(false);
  const [showOverrideModal, setShowOverrideModal] = useState(false);
  const [overrideReason, setOverrideReason] = useState("");
  const [decisionModal, setDecisionModal] = useState<null | {
    status: string;
    title: string;
    intro: string;
    label: string;
    placeholder: string;
    defaultReason: string;
  }>(null);
  const [decisionReason, setDecisionReason] = useState("");
  const [pressFreedomReview, setPressFreedomReview] = useState("");
  const [isRunningPressFreedomReview, setIsRunningPressFreedomReview] = useState(false);
  const [isImprovingForPublication, setIsImprovingForPublication] = useState(false);
  const editorPanelRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    setError(null);
    setRewritePreview(null);
    setAttested(false);
    setShowOverrideModal(false);
    setOverrideReason("");
    setDecisionModal(null);
    setDecisionReason("");
    setPressFreedomReview("");
  }, [selectedDraft?.id]);

  useEffect(() => {
    if (!selectedDraft?.id) return;
    window.requestAnimationFrame(() => {
      document
        .querySelectorAll<HTMLElement>(".main-content, .app-main, main")
        .forEach((container) => {
          container.scrollTop = 0;
        });
      const panel = editorPanelRef.current;
      panel?.scrollIntoView?.({ block: "start", behavior: "auto" });
      panel?.focus?.({ preventScroll: true });
    });
  }, [selectedDraft?.id]);

  useEffect(() => {
    if (!selectedLead?.id || selectedDraft || generatingText) return;
    const timer = window.setTimeout(() => {
      generateDraftButtonRef.current?.scrollIntoView?.({ block: "center", behavior: "auto" });
      generateDraftButtonRef.current?.focus({ preventScroll: true });
    }, 50);
    return () => window.clearTimeout(timer);
  }, [generatingText, selectedDraft, selectedLead?.id]);

  const handleDraftWizardKeyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    if (event.key !== "Enter" || event.defaultPrevented || generatingText) return;
    const target = event.target as HTMLElement | null;
    const tagName = target?.tagName.toLowerCase();
    if (tagName === "button" || tagName === "textarea" || tagName === "input" || tagName === "select" || target?.isContentEditable) {
      return;
    }
    event.preventDefault();
    if (ollamaOnline || manualLlmMode) {
      onGenerateText();
    }
  };

  // Any guardrail issue should be consciously reviewed before publishing. The
  // editor can still continue; software records the choice instead of vetoing it.
  const guardrailIssues = guardrailsReport?.issues ?? [];
  const severeIssueCount = guardrailIssues.filter((i) => i.severity === "error").length;
  const qualityWarningsForSelectedDraft = selectedDraft
    ? getStoryQualityWarnings(selectedDraft, evidenceList)
    : [];
  const staticPublishBlockers = selectedDraft
    ? getStaticPublishBlockers(selectedDraft, evidenceList)
    : [];
  const totalReviewWarningCount = guardrailIssues.length + qualityWarningsForSelectedDraft.length;

  const handleApproveClick = () => {
    if (selectedDraft?.status === "killed") {
      return;
    }
    if (!attested) {
      setShowOverrideModal(false);
      setError("Before approval: confirm that an editor has reviewed this story and takes responsibility for publishing it.");
      return;
    }
    setDecisionModal(null);
    if (staticPublishBlockers.length > 0) {
      setShowOverrideModal(false);
      setError(`Before static publish approval: ${staticPublishBlockers.join(" ")}`);
      return;
    }
    if (totalReviewWarningCount > 0) {
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
    } else if (totalReviewWarningCount > 0) {
      onApprovePublish?.("Editor reviewed pre-publication warnings and chose to publish.");
    } else {
      onApprovePublish?.();
    }
  };

  const openDecisionModal = (status: string) => {
    const presets: Record<string, NonNullable<typeof decisionModal>> = {
      needs_verification: {
        status,
        title: "Send back for more work",
        intro: "Record why this draft is not ready. This note stays with the draft so a writer knows what to fix next.",
        label: "Reason / assignment",
        placeholder: "e.g. Needs a second source, source date is unclear, call city clerk, too generic for publication...",
        defaultReason: "Needs more reporting or verification before review.",
      },
      hold: {
        status,
        title: "Put story on hold",
        intro: "Record why this story is paused so it can be picked up later without losing context.",
        label: "Hold note",
        placeholder: "e.g. Waiting for agenda packet, revisit after Thursday meeting, not newsworthy yet...",
        defaultReason: "Held for later editorial review.",
      },
    };
    const next = presets[status];
    if (!next) {
      onDecision(status);
      return;
    }
    setShowOverrideModal(false);
    setDecisionModal(next);
    setDecisionReason(next.defaultReason);
  };

  const confirmDecisionModal = () => {
    if (!decisionModal) return;
    const reason = decisionReason.trim() || decisionModal.defaultReason;
    onDecision(decisionModal.status, reason);
    setDecisionModal(null);
    setDecisionReason("");
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
      case "needs_verification": return "warning";
      case "killed": return "offline";
      case "corrected": return "info";
      default: return "warning";
    }
  };

  const handleImproveForPublication = async () => {
    if (!onImproveForPublication || !selectedDraft?.content) return;
    setIsImprovingForPublication(true);
    setError(null);
    try {
      await onImproveForPublication();
    } catch (error: any) {
      console.error("Failed to improve draft for publication:", error);
      setError(error?.message || String(error));
    } finally {
      setIsImprovingForPublication(false);
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case "draft_generated": return "Drafting";
      case "ready_to_review": return "Ready for review";
      case "ready_to_publish": return "Approved for publishing";
      case "needs_verification": return "Sent back / needs work";
      case "hold": return "On hold";
      case "killed": return "Cut";
      case "published": return "Published";
      case "corrected": return "Corrected";
      default: return status.replace(/_/g, " ");
    }
  };

  const getLeadDispositionLabel = (disposition?: string) => {
    switch ((disposition ?? "review").toLowerCase()) {
      case "ready_to_draft": return "Ready to draft";
      case "needs_verification": return "Needs verification";
      case "background": return "Background";
      case "watch": return "Watch";
      default: return "Editor review";
    }
  };

  const getLeadQualityGuidance = (lead: Lead) => {
    const disposition = (lead.disposition ?? "review").toLowerCase();
    const storyType = (lead.story_type ?? "").toLowerCase();
    const recurrence = lead.recurrence_count !== undefined && lead.recurrence_count > 0;
    if (recurrence) {
      return "Recurring topic. Draft only if the source shows a new vote, deadline, dollar amount, filing, outage, meeting item, or public impact.";
    }
    if (disposition === "background" || storyType === "background") {
      return "Background item. Treat as context or an editor memo unless you have a current, specific change.";
    }
    if (disposition === "needs_verification" || storyType === "verification") {
      return "Verification assignment. Use the draft as reporting notes until the missing facts are checked.";
    }
    if (disposition === "watch" || storyType === "watch") {
      return "Watch item. Explain what is known, what is missing, and what would make it publishable.";
    }
    return "Use linked evidence and the novelty notes to decide whether this is ready for reader-facing copy.";
  };

  const leadNeedsDraftCaution = (lead: Lead) => {
    const disposition = (lead.disposition ?? "review").toLowerCase();
    const storyType = (lead.story_type ?? "").toLowerCase();
    const novelty = lead.novelty_score ?? 0;
    return Boolean(
      (lead.recurrence_count ?? 0) > 0 ||
      storyType === "background" ||
      storyType === "watch" ||
      storyType === "verification" ||
      disposition === "background" ||
      disposition === "watch" ||
      disposition === "needs_verification" ||
      (novelty > 0 && novelty <= 2)
    );
  };

  // If drafting from a Lead
  if (selectedLead && !selectedDraft) {
    const noLinkedEvidence = evidenceList.length === 0;
    const generateDraftLabel = noLinkedEvidence
      ? "Generate Verification Notes"
      : leadNeedsDraftCaution(selectedLead) ? "Generate anyway" : "Generate Draft";
    return (
      <div className="wizard-container card" id="draft-wizard-panel" tabIndex={-1} onKeyDown={handleDraftWizardKeyDown}>
        <h1>Drafting Article</h1>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          Lead: <strong>{selectedLead.why}</strong>
        </p>

        <div className="card" id="draft-lead-quality-card" style={{ background: "var(--accent-light)", marginBottom: "1rem" }}>
          <div className="lead-header" style={{ marginBottom: "0.65rem" }}>
            {selectedLead.story_type && (
              <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                {selectedLead.story_type}
              </span>
            )}
            <span className="badge badge-info">{getLeadDispositionLabel(selectedLead.disposition)}</span>
            {selectedLead.novelty_score !== undefined && (
              <span className="badge badge-neutral">Novelty {selectedLead.novelty_score}/5</span>
            )}
            {selectedLead.recurrence_count !== undefined && selectedLead.recurrence_count > 0 && (
              <span className="badge badge-warning">
                {selectedLead.recurrence_count === 1 ? "Seen before" : `Seen ${selectedLead.recurrence_count} times before`}
              </span>
            )}
          </div>
          <p className="help-text" style={{ margin: 0 }}>{getLeadQualityGuidance(selectedLead)}</p>
          {selectedLead.novelty_reason && (
            <p className="help-text" style={{ margin: "0.5rem 0 0 0" }}>
              <strong>Why now:</strong> {selectedLead.novelty_reason}
            </p>
          )}
          {selectedLead.recurrence_note && (
            <p className="help-text" style={{ margin: "0.5rem 0 0 0" }}>
              <strong>Beat memory:</strong> {selectedLead.recurrence_note}
            </p>
          )}
        </div>

        <div className="draft-wizard-top-actions">
          <button ref={generateDraftButtonRef} type="button" className="btn btn-primary" onClick={onGenerateText} disabled={generatingText || (!ollamaOnline && !manualLlmMode)} id="btn-generate-draft-top">
            {generatingText ? "Generating Draft..." : generateDraftLabel}
          </button>
          <button type="button" className="btn btn-secondary" onClick={onCancelDraftWizard} disabled={generatingText} id="btn-cancel-draft-top">
            Cancel
          </button>
        </div>

        {generatingText && (
          <div className="draft-wizard-progress" role="status" aria-live="polite">
            Asking the local AI model to write a working draft. Keep this window open.
          </div>
        )}

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
                  No source documents are linked to this lead yet. The AI can create verification notes, but this should not be approved for publication until source material is attached or cited.
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
            <div className="error-text" id="ollama-offline-warning" role="status" aria-live="polite" style={{ display: "flex", alignItems: "center", gap: "0.5rem", flexWrap: "wrap" }}>
              <AlertTriangle size={14} /> The local AI service is offline. Set up the local AI model, or use Manual Mode in settings.
              {onOpenAiSetup && (
                <button type="button" className="btn btn-secondary btn-sm" onClick={onOpenAiSetup} id="btn-open-ai-setup-from-workbench">
                  Open AI Setup
                </button>
              )}
            </div>
          )}

          <div className="draft-wizard-bottom-actions">
            <button type="button" className="btn btn-primary" onClick={onGenerateText} disabled={generatingText || (!ollamaOnline && !manualLlmMode)} id="btn-generate-draft-bottom">
              {generatingText ? "Generating Draft..." : generateDraftLabel}
            </button>
            <button type="button" className="btn btn-secondary" onClick={onCancelDraftWizard} disabled={generatingText} id="btn-cancel-draft-bottom">
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  // If editing an existing Draft
  if (selectedDraft) {
    const workflowStatus = selectedDraft.status;
    const finalStatus = workflowStatus === "published" || workflowStatus === "corrected";
    const canResume = workflowStatus === "needs_verification" || workflowStatus === "killed";
    const canSendBack = !["needs_verification", "killed", "published", "corrected"].includes(workflowStatus);
    const canHold = workflowStatus !== "hold" && !finalStatus;
    const canCut = workflowStatus !== "killed" && !finalStatus;
    const canMarkReady = !["ready_to_review", "ready_to_publish", "killed", "published", "corrected"].includes(workflowStatus);
    const canUnapprove = workflowStatus === "ready_to_publish";
    const pausedForMoreWork = workflowStatus === "hold" || workflowStatus === "needs_verification";
    const qualityWarnings = qualityWarningsForSelectedDraft;
    const approveDisabled = selectedDraft.status === "killed" || finalStatus || pausedForMoreWork || !attested || Boolean(decisionModal);
    const approveTitle =
      selectedDraft.status === "killed"
        ? "Restore this story before approving it for publishing"
        : finalStatus
          ? "This story is already in a final publishing state"
        : pausedForMoreWork
          ? "Resume editing or mark this story ready for review before approving it"
        : !attested
          ? "Check editorial responsibility before approving"
        : totalReviewWarningCount > 0
          ? "This story has review warnings - you'll be asked to confirm that the editor reviewed them"
        : "Approve this story for publishing";

    return (
      <div id="workbench-editor-panel" tabIndex={-1} ref={editorPanelRef}>
        <div className="page-header workbench-editor-header" style={{ marginBottom: "1rem" }}>
          <div className="page-title">
            <h1>Story Workbench</h1>
            <p>Modify drafted content, review guardrail warnings, and link source material when it helps the editor.</p>
          </div>
          <div className="btn-group workbench-editor-actions">
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

        <div className="card workbench-priority-strip" id="workbench-priority-strip">
          <div className="workbench-priority-summary">
            <span className={`badge badge-${getStatusColor(selectedDraft.status)}`}>
              {getStatusLabel(selectedDraft.status)}
            </span>
            <strong>{selectedDraft.title || "Untitled draft"}</strong>
          </div>
          <div className="workbench-priority-actions">
            {onImproveForPublication && (
              <button
                type="button"
                className="btn btn-secondary btn-sm"
                id="btn-improve-publication-top"
                onClick={handleImproveForPublication}
                disabled={isImprovingForPublication || (!ollamaOnline && !manualLlmMode) || !selectedDraft.content}
              >
                {isImprovingForPublication ? "Improving..." : "Improve for Publication"}
              </button>
            )}
            {canSendBack && (
              <button className="btn btn-secondary btn-sm" onClick={() => openDecisionModal("needs_verification")} id="btn-status-send-back-top">
                Send Back
              </button>
            )}
            {canMarkReady && (
              <button className="btn btn-secondary btn-sm" onClick={() => onDecision("ready_to_review")} id="btn-status-ready-review-top">
                Ready
              </button>
            )}
            <label htmlFor="chk-attest-top" className="workbench-priority-attest">
              <input
                id="chk-attest-top"
                type="checkbox"
                checked={attested}
                onChange={(e) => setAttested(e.target.checked)}
              />
              <span>I reviewed this story.</span>
            </label>
            <button
              className="btn btn-primary btn-sm"
              onClick={handleApproveClick}
              disabled={approveDisabled}
              title={approveTitle}
              id="btn-status-publish-top"
            >
              Approve
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
                      ? "Advisory warnings - these do not block publishing."
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

        <div className="card" id="story-quality-preflight-card" style={{ padding: "1rem", marginBottom: "1rem", background: qualityWarnings.length ? "rgba(245, 158, 11, 0.08)" : "var(--accent-light)" }}>
          <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
            <div>
              <h3 className="card-title" style={{ marginBottom: "0.35rem" }}>Story-quality preflight</h3>
              <p className="help-text" style={{ margin: 0 }}>
                Checks for headline, attribution, citation, and reporter-note problems before approval. Warnings are advisory; package-validity blockers must be fixed before static publish approval.
              </p>
            </div>
            <div className="btn-group">
              {onUpdateDraftFormat && selectedDraft.format !== "brief" && (
                <button type="button" className="btn btn-secondary btn-sm" id="btn-make-brief" onClick={() => onUpdateDraftFormat("brief")}>
                  Make this a brief
                </button>
              )}
              {onImproveForPublication && (
                <button
                  type="button"
                  className="btn btn-secondary btn-sm"
                  id="btn-improve-publication"
                  onClick={handleImproveForPublication}
                  disabled={isImprovingForPublication || (!ollamaOnline && !manualLlmMode) || !selectedDraft.content}
                >
                  {isImprovingForPublication ? "Improving..." : "Improve for Publication"}
                </button>
              )}
            </div>
          </div>
          {staticPublishBlockers.length > 0 && (
            <div className="error-text" role="alert" style={{ marginTop: "0.65rem" }}>
              <strong>Fix before static publish approval:</strong>
              <ul style={{ margin: "0.35rem 0 0 1.2rem" }}>
                {staticPublishBlockers.map((blocker) => (
                  <li key={blocker}>{blocker}</li>
                ))}
              </ul>
            </div>
          )}
          {qualityWarnings.length > 0 ? (
            <ul className="help-text" style={{ margin: "0.65rem 0 0 1.2rem" }}>
              {qualityWarnings.map((warning) => (
                <li key={warning}>{warning}</li>
              ))}
            </ul>
          ) : (
            <p className="help-text" style={{ margin: "0.65rem 0 0 0" }}>
              No obvious story-quality warnings from the local preflight.
            </p>
          )}
        </div>

        <div className="workbench-container">
          {/* Editor Pane (Left) */}
          <div className="editor-pane">
            <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
              <label htmlFor="input-draft-title" style={{ fontWeight: 600, fontSize: "0.9rem" }}>Story Title</label>
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
                <label htmlFor="draft-editor-textarea" style={{ fontWeight: 600, fontSize: "0.9rem" }}>Article Body (Markdown)</label>
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
                      {getStatusLabel(selectedDraft.status)}
                    </strong>
                  </div>
                  <div className="btn-group">
                    {canResume && (
                      <button className="btn btn-secondary btn-sm" onClick={() => onDecision("draft_generated")} id="btn-status-resume-draft">
                        {workflowStatus === "killed" ? "Restore to Drafting" : "Resume Editing"}
                      </button>
                    )}
                  {canSendBack && (
                      <button className="btn btn-secondary btn-sm" onClick={() => openDecisionModal("needs_verification")} id="btn-status-send-back">
                        Send Back for More Work
                      </button>
                    )}
                    {canMarkReady && (
                      <button className="btn btn-secondary btn-sm" onClick={() => onDecision("ready_to_review")} id="btn-status-ready-review">
                        Mark Ready for Review
                      </button>
                    )}
                    {canUnapprove && (
                      <button className="btn btn-secondary btn-sm" onClick={() => onDecision("ready_to_review")} id="btn-status-unapprove">
                        Unapprove
                      </button>
                    )}
                    {canHold && (
                      <button className="btn btn-secondary btn-sm" onClick={() => openDecisionModal("hold")} id="btn-status-hold">
                        Hold
                      </button>
                    )}
                    {canCut && (
                      <button className="btn btn-danger btn-sm" onClick={() => (onKillStory ? onKillStory() : onDecision("killed"))} id="btn-status-kill">
                        Cut Story
                      </button>
                    )}
                    <button
                      className="btn btn-primary btn-sm"
                      onClick={handleApproveClick}
                      disabled={approveDisabled}
                      title={approveTitle}
                      id="btn-status-publish"
                    >
                      Approve for Static Publish
                    </button>
                  </div>
                </div>
                {selectedDraft.status === "hold" && (
                  <div
                    role="status"
                    style={{ background: "rgba(245, 158, 11, 0.08)", borderLeft: "4px solid var(--color-warning)", borderRadius: "4px", color: "var(--text-primary)", padding: "0.75rem" }}
                  >
                    <p style={{ margin: "0 0 0.5rem 0" }}>
                      This draft is on hold. Resume editing when you are ready, or send it back for more reporting and verification.
                    </p>
                    {selectedDraft.missing_evidence_notes && (
                      <p className="help-text" style={{ margin: "0 0 0.5rem 0" }}>
                        <strong>Hold note:</strong> {selectedDraft.missing_evidence_notes}
                      </p>
                    )}
                    <div className="btn-group">
                      <button className="btn btn-secondary btn-sm" type="button" onClick={() => onDecision("draft_generated")} id="btn-hold-resume-inline">
                        Resume Editing
                      </button>
                      <button className="btn btn-secondary btn-sm" type="button" onClick={() => openDecisionModal("needs_verification")} id="btn-hold-send-back-inline">
                        Send Back for More Work
                      </button>
                    </div>
                  </div>
                )}
                {selectedDraft.status === "needs_verification" && (
                  <div
                    role="status"
                    style={{ background: "rgba(59, 130, 246, 0.08)", borderLeft: "4px solid var(--color-info)", borderRadius: "4px", color: "var(--text-primary)", padding: "0.75rem" }}
                  >
                    <p style={{ margin: 0 }}>
                      This draft was sent back for more work. Keep editing, add evidence, then mark it ready for review when it is ready for an editor again.
                    </p>
                    {selectedDraft.missing_evidence_notes && (
                      <p className="help-text" style={{ margin: "0.5rem 0 0 0" }}>
                        <strong>Assignment note:</strong> {selectedDraft.missing_evidence_notes}
                      </p>
                    )}
                  </div>
                )}
                {selectedDraft.status === "killed" && (
                  <div
                    role="status"
                    style={{ background: "rgba(245, 158, 11, 0.08)", borderLeft: "4px solid var(--color-warning)", borderRadius: "4px", color: "var(--text-primary)", padding: "0.75rem" }}
                  >
                    This story is cut from the issue. Restore it to drafting if the newsroom decides to keep working on it.
                  </div>
                )}
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
            <h3 id="override-modal-title" style={{ marginTop: 0, color: severeIssueCount > 0 ? "var(--color-error)" : "var(--color-warning)" }}>
              Publish with review warnings?
            </h3>
            <p className="help-text" style={{ marginTop: 0 }}>
              This story has {totalReviewWarningCount} review warning(s)
              {severeIssueCount > 0 ? `, including ${severeIssueCount} high-concern issue(s),` : ""}
              {" "}from your newsroom's guardrail and story-quality checks. The app will not veto the editor,
              but this decision is recorded with the story.
            </p>
            <ul style={{ fontSize: "0.85rem", margin: "0 0 1rem 0", paddingLeft: "1.2rem" }}>
              {guardrailIssues.map((iss: any, idx: number) => (
                <li key={idx}>
                  [{iss.category.replace(/_/g, " ")}] {iss.message}
                </li>
              ))}
              {qualityWarningsForSelectedDraft.map((warning, idx) => (
                <li key={`quality-${idx}`}>
                  [story quality] {warning}
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
                className={severeIssueCount > 0 ? "btn btn-danger" : "btn btn-primary"}
                onClick={confirmOverride}
                id="btn-override-confirm"
              >
                Publish anyway (logged)
              </button>
            </div>
          </Modal>
        )}
        {decisionModal && (
          <Modal id="decision-reason-modal" labelledBy="decision-reason-modal-title" onClose={() => setDecisionModal(null)}>
            <h3 id="decision-reason-modal-title" style={{ marginTop: 0 }}>{decisionModal.title}</h3>
            <p className="help-text" style={{ marginTop: 0 }}>{decisionModal.intro}</p>
            <label htmlFor="decision-reason" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>
              {decisionModal.label}
            </label>
            <textarea
              id="decision-reason"
              value={decisionReason}
              onChange={(e) => setDecisionReason(e.target.value)}
              placeholder={decisionModal.placeholder}
              style={{ width: "100%", height: "110px" }}
            />
            <div className="flex-between" style={{ marginTop: "1rem" }}>
              <button className="btn btn-secondary" type="button" onClick={() => setDecisionModal(null)} id="btn-decision-reason-cancel">
                Cancel
              </button>
              <button className="btn btn-primary" type="button" onClick={confirmDecisionModal} id="btn-decision-reason-confirm">
                Save decision note
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
                <small>{draft.format} - {getStatusLabel(draft.status)}</small>
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

