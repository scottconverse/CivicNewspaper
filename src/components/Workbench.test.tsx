// src/components/Workbench.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { Workbench } from "./Workbench";
import { Lead, Draft, GuardrailsReport } from "../ipc";

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
});
