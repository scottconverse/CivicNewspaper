// src/components/LeadQueue.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { LeadQueue } from "./LeadQueue";
import { Draft, Lead } from "../ipc";

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

  const fixtureDrafts: Draft[] = [
    {
      id: 201,
      lead_id: 101,
      format: "watch",
      title: "Budget Watch",
      content: "Draft content",
      status: "draft_generated",
      verification_checklist: "[]",
    },
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
    expect(handleSelect).toHaveBeenCalledWith(101, fixtureLeads[0]);
  });

  test("visible Draft button passes the rendered lead object to the parent", () => {
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

    fireEvent.click(screen.getByTestId("btn-draft-lead-102"));

    expect(handleSelect).toHaveBeenCalledWith(102, fixtureLeads[1]);
  });

  test("keeps drafts reachable when latest scan results exist", () => {
    const handleOpenDraft = vi.fn();

    render(
      <LeadQueue
        leads={fixtureLeads}
        drafts={fixtureDrafts}
        loading={false}
        latestScanId={9}
        onSelect={vi.fn()}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={handleOpenDraft}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    expect(screen.getByRole("button", { name: /Scan results/i })).toBeInTheDocument();
    fireEvent.click(document.getElementById("queue-tab-drafts")!);
    expect(screen.getByText("Budget Watch")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /^Open$/i }));
    expect(handleOpenDraft).toHaveBeenCalledWith(fixtureDrafts[0]);
  });

  test("opens an existing draft from a drafted lead instead of starting a duplicate", () => {
    const handleSelect = vi.fn();
    const handleOpenDraft = vi.fn();

    render(
      <LeadQueue
        leads={fixtureLeads}
        drafts={fixtureDrafts}
        loading={false}
        onSelect={handleSelect}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={handleOpenDraft}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    expect(screen.getByText("Draft exists")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Open draft/i }));

    expect(handleOpenDraft).toHaveBeenCalledWith(fixtureDrafts[0]);
    expect(handleSelect).not.toHaveBeenCalled();
  });
});
