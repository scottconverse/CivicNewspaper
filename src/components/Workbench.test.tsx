// src/components/Workbench.test.tsx
import type { ComponentProps } from "react";
import { render, screen, fireEvent, waitFor, within } from "@testing-library/react";
import { describe, test, expect, vi, beforeEach } from "vitest";
import { Workbench } from "./Workbench";
import { Lead, Draft, GuardrailsReport, plainLanguageRewrite } from "../ipc";

vi.mock("../ipc", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../ipc")>();
  return { ...actual, plainLanguageRewrite: vi.fn() };
});

describe("Workbench Component Tests", () => {
  const mockLead: Lead = {
    id: 42,
    detector_name: "Procurement",
    why: "Unusual expense spike",
    confidence: "high",
    risk_level: "high",
    confirmation_checklist: "[]",
    created_at: "2026-05-23T00:00:00Z"
  };

  const mockDraft: Draft = {
    id: 123,
    lead_id: 42,
    format: "watch",
    title: "Suspicious Spending",
    content: "Content with citations",
    status: "draft_generated",
    verification_checklist: "[]"
  };

  const renderEditor = (overrides: Partial<ComponentProps<typeof Workbench>> = {}) =>
    render(
      <Workbench
        selectedLead={null}
        selectedDraft={mockDraft}
        evidenceList={[]}
        guardrailsReport={null}
        ollamaOnline={true}
        manualLlmMode={false}
        draftFormat="watch"
        onDraftFormatChange={vi.fn()}
        customSystemPrompt=""
        onCustomSystemPromptChange={vi.fn()}
        generatingText={false}
        onGenerateText={vi.fn()}
        onCancelDraftWizard={vi.fn()}
        onSaveDraftEditor={vi.fn()}
        onCloseWorkbench={vi.fn()}
        onDeleteDraft={vi.fn()}
        onDecision={vi.fn()}
        isGeneratingSocial={false}
        socialPackResult=""
        onSocialPackResultChange={vi.fn()}
        onGenerateSocial={vi.fn()}
        onUpdateDraftTitle={vi.fn()}
        onUpdateDraftContent={vi.fn()}
        {...overrides}
      />
    );

  beforeEach(() => {
    vi.mocked(plainLanguageRewrite).mockReset();
  });

  test("renders selectedLead and clicking Generate Draft fires action callback", () => {
    const handleGenerateText = vi.fn();

    render(
      <Workbench
        selectedLead={mockLead}
        selectedDraft={null}
        evidenceList={[]}
        guardrailsReport={null}
        ollamaOnline={true}
        manualLlmMode={false}
        draftFormat="watch"
        onDraftFormatChange={vi.fn()}
        customSystemPrompt=""
        onCustomSystemPromptChange={vi.fn()}
        generatingText={false}
        onGenerateText={handleGenerateText}
        onCancelDraftWizard={vi.fn()}
        onSaveDraftEditor={vi.fn()}
        onCloseWorkbench={vi.fn()}
        onDeleteDraft={vi.fn()}
        onDecision={vi.fn()}
        isGeneratingSocial={false}
        socialPackResult=""
        onSocialPackResultChange={vi.fn()}
        onGenerateSocial={vi.fn()}
        onUpdateDraftTitle={vi.fn()}
        onUpdateDraftContent={vi.fn()}
      />
    );

    expect(screen.getByText(/Unusual expense spike/i)).toBeInTheDocument();
    
    const generateBtn = screen.getByRole("button", { name: /Generate Draft/i });
    fireEvent.click(generateBtn);
    expect(handleGenerateText).toHaveBeenCalled();
  });

  test("renders selectedDraft and contains flagged-claim CSS class when guardrailsReport has issues", () => {
    const mockReport: GuardrailsReport = {
      is_clean: false,
      issues: [
        {
          category: "Accusatory Language",
          message: "Uncorroborated accusation",
          severity: "error"
        }
      ]
    };

    render(
      <Workbench
        selectedLead={null}
        selectedDraft={mockDraft}
        evidenceList={[]}
        guardrailsReport={mockReport}
        ollamaOnline={true}
        manualLlmMode={false}
        draftFormat="watch"
        onDraftFormatChange={vi.fn()}
        customSystemPrompt=""
        onCustomSystemPromptChange={vi.fn()}
        generatingText={false}
        onGenerateText={vi.fn()}
        onCancelDraftWizard={vi.fn()}
        onSaveDraftEditor={vi.fn()}
        onCloseWorkbench={vi.fn()}
        onDeleteDraft={vi.fn()}
        onDecision={vi.fn()}
        isGeneratingSocial={false}
        socialPackResult=""
        onSocialPackResultChange={vi.fn()}
        onGenerateSocial={vi.fn()}
        onUpdateDraftTitle={vi.fn()}
        onUpdateDraftContent={vi.fn()}
      />
    );

    // Assert that the title input renders the correct title
    expect(screen.getByDisplayValue("Suspicious Spending")).toBeInTheDocument();

    // Verify guardrail panel renders and has 'flagged-claim' class
    const reportCard = screen.getByTestId("guardrails-report-card");
    expect(reportCard).toHaveClass("flagged-claim");
    expect(screen.getByText(/Uncorroborated accusation/i)).toBeInTheDocument();
  });

  test("rewrite opens a diff modal instead of overwriting the draft in place", async () => {
    vi.mocked(plainLanguageRewrite).mockResolvedValue("Plain simple text\nsecond line");
    const onUpdateDraftContent = vi.fn();

    renderEditor({ onUpdateDraftContent });

    fireEvent.click(screen.getByRole("button", { name: /Plain Language Rewrite/i }));

    // Modal appears showing both the original and the rewrite; nothing applied yet.
    expect(await screen.findByText(/Review Plain Language Rewrite/i)).toBeInTheDocument();
    expect(plainLanguageRewrite).toHaveBeenCalledWith("Content with citations", "watch");
    const originalPane = within(document.getElementById("diff-pane-original")!);
    const rewritePane = within(document.getElementById("diff-pane-rewrite")!);
    expect(originalPane.getByText("Content with citations")).toBeInTheDocument();
    expect(rewritePane.getByText("Plain simple text")).toBeInTheDocument();
    expect(onUpdateDraftContent).not.toHaveBeenCalled();
  });

  test("accepting the diff applies the rewrite and closes the modal", async () => {
    vi.mocked(plainLanguageRewrite).mockResolvedValue("Plain simple text");
    const onUpdateDraftContent = vi.fn();

    renderEditor({ onUpdateDraftContent });

    fireEvent.click(screen.getByRole("button", { name: /Plain Language Rewrite/i }));
    await screen.findByText(/Review Plain Language Rewrite/i);

    fireEvent.click(screen.getByRole("button", { name: /Accept Rewrite/i }));

    expect(onUpdateDraftContent).toHaveBeenCalledWith("Plain simple text");
    await waitFor(() =>
      expect(screen.queryByText(/Review Plain Language Rewrite/i)).not.toBeInTheDocument()
    );
  });

  test("rejecting the diff discards the rewrite and leaves the draft untouched", async () => {
    vi.mocked(plainLanguageRewrite).mockResolvedValue("Plain simple text");
    const onUpdateDraftContent = vi.fn();

    renderEditor({ onUpdateDraftContent });

    fireEvent.click(screen.getByRole("button", { name: /Plain Language Rewrite/i }));
    await screen.findByText(/Review Plain Language Rewrite/i);

    fireEvent.click(screen.getByRole("button", { name: /Reject/i }));

    expect(onUpdateDraftContent).not.toHaveBeenCalled();
    await waitFor(() =>
      expect(screen.queryByText(/Review Plain Language Rewrite/i)).not.toBeInTheDocument()
    );
  });

  test("shows an in-flight indicator while rewriting, then opens the modal", async () => {
    let resolveRewrite: (v: string) => void = () => {};
    vi.mocked(plainLanguageRewrite).mockReturnValue(
      new Promise<string>((resolve) => {
        resolveRewrite = resolve;
      })
    );

    renderEditor();

    fireEvent.click(screen.getByRole("button", { name: /Plain Language Rewrite/i }));

    // Button reflects the in-flight state before the promise settles.
    expect(await screen.findByRole("button", { name: /Rewriting/i })).toBeDisabled();

    resolveRewrite("Plain simple text");

    expect(await screen.findByText(/Review Plain Language Rewrite/i)).toBeInTheDocument();
  });

  test("surfaces an error in-context when the rewrite fails and opens no modal", async () => {
    const errSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    vi.mocked(plainLanguageRewrite).mockRejectedValue(new Error("Ollama offline"));

    renderEditor();

    fireEvent.click(screen.getByRole("button", { name: /Plain Language Rewrite/i }));

    expect(await screen.findByText(/Ollama offline/i)).toBeInTheDocument();
    expect(screen.queryByText(/Review Plain Language Rewrite/i)).not.toBeInTheDocument();
    // Button returns to its idle, enabled state.
    expect(screen.getByRole("button", { name: /Plain Language Rewrite/i })).toBeEnabled();

    errSpy.mockRestore();
  });

  test("disables the rewrite button when the draft has no content", () => {
    renderEditor({ selectedDraft: { ...mockDraft, content: "" } });
    expect(screen.getByRole("button", { name: /Plain Language Rewrite/i })).toBeDisabled();
  });
});
