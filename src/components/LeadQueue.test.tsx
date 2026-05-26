// src/components/LeadQueue.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { LeadQueue } from "./LeadQueue";
import { Lead } from "../ipc";

describe("LeadQueue Component Tests", () => {
  const fixtureLeads: Lead[] = [
    {
      id: 101,
      detector_name: "Money Threshold",
      why: "$350,000 budget approved",
      confidence: "high",
      risk_level: "high",
      confirmation_checklist: "[]",
      created_at: "2026-05-23T00:00:00Z"
    },
    {
      id: 102,
      detector_name: "Watchlist Hit",
      why: "John Doe contractor noted",
      confidence: "med",
      risk_level: "med",
      confirmation_checklist: "[]",
      created_at: "2026-05-23T00:00:00Z"
    },
    {
      id: 103,
      detector_name: "Decision / Vote",
      why: "Unanimous zoning approval",
      confidence: "low",
      risk_level: "low",
      confirmation_checklist: "[]",
      created_at: "2026-05-23T00:00:00Z"
    }
  ];

  test("renders 3 fixture leads with risk/status badges and triggers onSelect", () => {
    const handleSelect = vi.fn();

    render(
      <LeadQueue
        leads={fixtureLeads}
        drafts={[]}
        loading={false}
        onSelect={handleSelect}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={vi.fn()}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    // Assert all 3 leads are rendered by checking their text content
    expect(screen.getByText("$350,000 budget approved")).toBeInTheDocument();
    expect(screen.getByText("John Doe contractor noted")).toBeInTheDocument();
    expect(screen.getByText("Unanimous zoning approval")).toBeInTheDocument();

    // Check status/risk badges are present
    expect(screen.getByText("Risk: high")).toBeInTheDocument();
    expect(screen.getByText("Risk: med")).toBeInTheDocument();
    expect(screen.getByText("Risk: low")).toBeInTheDocument();

    // Click on the first lead card and assert callback fired
    const firstLeadCard = screen.getByTestId("lead-card-101");
    fireEvent.click(firstLeadCard);
    expect(handleSelect).toHaveBeenCalledWith(101);
  });
});
