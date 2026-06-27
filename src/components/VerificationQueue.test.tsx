import { render, screen, fireEvent } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { VerificationQueue } from "./VerificationQueue";
import type { VerificationQueueSnapshot } from "../ipc";

const queue: VerificationQueueSnapshot = {
  generated_count: 2,
  tasks: [
    {
      id: 1,
      dark_signal_id: 5,
      observation_id: 10,
      lead_id: null,
      draft_id: null,
      entity_id: 7,
      check_type: "entity_lookup",
      title: "Verify company: Acme Development LLC",
      description: "Look up this company in official records.",
      target_label: "company: Acme Development LLC",
      target_url: null,
      status: "needs_human",
      effort_level: "medium",
      impact_level: "high",
      rank_score: 94,
      result_summary: null,
      created_at: "2026-06-27T12:00:00Z",
      updated_at: "2026-06-27T12:00:00Z",
    },
    {
      id: 2,
      dark_signal_id: 5,
      observation_id: 10,
      lead_id: null,
      draft_id: null,
      entity_id: null,
      check_type: "source_reachability",
      title: "Confirm source access: Town forum",
      description: "Open the original source.",
      target_label: "Town forum",
      target_url: "https://forum.example.test/thread",
      status: "auto_checked",
      effort_level: "low",
      impact_level: "high",
      rank_score: 100,
      result_summary: "Source was online during the most recent fetch.",
      created_at: "2026-06-27T12:00:00Z",
      updated_at: "2026-06-27T12:00:00Z",
    },
  ],
};

describe("VerificationQueue", () => {
  it("shows ranked verification work with links back to signal context", () => {
    render(<VerificationQueue queue={queue} loading={false} onRefresh={vi.fn()} onStatusChange={vi.fn()} onCreateLead={vi.fn()} />);

    expect(screen.getByText("Verification Queue")).toBeInTheDocument();
    expect(screen.getByText("Verify company: Acme Development LLC")).toBeInTheDocument();
    expect(screen.getAllByText("high impact").length).toBeGreaterThan(0);
    expect(screen.getByText("signal #5 / entity #7")).toBeInTheDocument();
    expect(screen.getByText("Source was online during the most recent fetch.")).toBeInTheDocument();
  });

  it("lets the editor resolve a task", () => {
    const onStatusChange = vi.fn();
    render(<VerificationQueue queue={queue} loading={false} onRefresh={vi.fn()} onStatusChange={onStatusChange} onCreateLead={vi.fn()} />);

    fireEvent.click(screen.getAllByRole("button", { name: /mark resolved/i })[0]);
    expect(onStatusChange).toHaveBeenCalledWith(queue.tasks[0], "resolved", "Marked resolved by editor.");
  });

  it("lets the editor create a story lead from an unlinked signal task", () => {
    const onCreateLead = vi.fn();
    render(<VerificationQueue queue={queue} loading={false} onRefresh={vi.fn()} onStatusChange={vi.fn()} onCreateLead={onCreateLead} />);

    fireEvent.click(screen.getAllByRole("button", { name: /create story lead/i })[0]);
    expect(onCreateLead).toHaveBeenCalledWith(5);
  });
});
