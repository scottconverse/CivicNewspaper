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

  test("renders 3 fixture leads with risk/status badges and keeps card text passive", () => {
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

    // Lead text is selectable/readable; the explicit Draft button starts work.
    const firstLeadCard = screen.getByTestId("lead-card-101");
    fireEvent.click(firstLeadCard);
    expect(handleSelect).not.toHaveBeenCalled();
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

  test("surfaces recurring beat-memory context before drafting", () => {
    render(
      <LeadQueue
        leads={[
          {
            ...fixtureLeads[0],
            recurrence_count: 1,
            recurrence_note: "Similar topic 'budget portal' was first seen last run.",
            disposition: "background",
          },
        ]}
        drafts={[]}
        loading={false}
        onSelect={vi.fn()}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={vi.fn()}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    expect(screen.getByText("Seen before")).toBeInTheDocument();
    expect(screen.getByText(/Beat memory:/)).toBeInTheDocument();
    expect(screen.getByText(/budget portal/)).toBeInTheDocument();
    expect(screen.getByText("Background")).toBeInTheDocument();
  });

  test("empty queue with sources points users to Daily Scan", () => {
    render(
      <LeadQueue
        leads={[]}
        drafts={[]}
        loading={false}
        sourceCount={3}
        onSelect={vi.fn()}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={vi.fn()}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    expect(screen.getByText("No story leads yet")).toBeInTheDocument();
    expect(
      screen.getByText("Run Daily Scan to check your watched sources and build the first editor packet.")
    ).toBeInTheDocument();
    expect(screen.queryByText(/Scrape & Detect above/i)).not.toBeInTheDocument();
  });

  test("labels weak leads as verify-first instead of normal draft choices", () => {
    render(
      <LeadQueue
        leads={[
          {
            ...fixtureLeads[0],
            disposition: "ready_to_draft",
            story_type: "brief",
            novelty_score: 4,
          },
          {
            ...fixtureLeads[1],
            disposition: "watch",
            story_type: "watch",
            novelty_score: 2,
          },
          {
            ...fixtureLeads[2],
            disposition: "needs_verification",
            story_type: "verification",
            novelty_score: 2,
          },
        ]}
        drafts={[]}
        loading={false}
        onSelect={vi.fn()}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={vi.fn()}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    expect(screen.getByRole("button", { name: /^Draft \$350,000 budget approved/i })).toBeInTheDocument();
    expect(screen.getAllByRole("button", { name: /^Review/i })).toHaveLength(2);
  });

  test("drafts tab filters send-back, held, and cut workflow states", () => {
    const workflowDrafts: Draft[] = [
      ...fixtureDrafts,
      {
        id: 202,
        lead_id: 102,
        format: "brief",
        title: "Needs More Reporting",
        content: "Draft content",
        status: "needs_verification",
        verification_checklist: "[]",
      },
      {
        id: 203,
        lead_id: 103,
        format: "brief",
        title: "Held Story",
        content: "Draft content",
        status: "hold",
        verification_checklist: "[]",
      },
      {
        id: 204,
        format: "brief",
        title: "Cut Story",
        content: "Draft content",
        status: "killed",
        verification_checklist: "[]",
      },
    ];

    render(
      <LeadQueue
        leads={fixtureLeads}
        drafts={workflowDrafts}
        loading={false}
        onSelect={vi.fn()}
        onSyncList={vi.fn()}
        onIngest={vi.fn()}
        onDailyScan={vi.fn()}
        onOpenDraftEditor={vi.fn()}
        onOpenCorrectionModal={vi.fn()}
        onDeleteDraft={vi.fn()}
      />
    );

    fireEvent.click(document.getElementById("queue-tab-drafts")!);
    expect(screen.getByText("Needs More Reporting")).toBeInTheDocument();
    expect(screen.getByText("Held Story")).toBeInTheDocument();
    expect(screen.getByText("Cut Story")).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/Draft status/i), {
      target: { value: "needs_verification" },
    });
    expect(screen.getByText("Needs More Reporting")).toBeInTheDocument();
    expect(screen.queryByText("Held Story")).not.toBeInTheDocument();
    expect(screen.queryByText("Cut Story")).not.toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/Draft status/i), {
      target: { value: "killed" },
    });
    expect(screen.getByText("Cut Story")).toBeInTheDocument();
    expect(screen.queryByText("Needs More Reporting")).not.toBeInTheDocument();
  });
});
