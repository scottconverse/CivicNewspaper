import { render, screen, fireEvent } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DarkSignalDesk } from "./DarkSignalDesk";
import type { CivicIntelligenceSnapshot } from "../ipc";

const snapshot: CivicIntelligenceSnapshot = {
  observations: [
    {
      id: 10,
      observation_type: "social_signal_found",
      source_id: 1,
      evidence_id: 22,
      title: "New social signal",
      summary: "Residents are discussing a parcel sale.",
      url: "https://example.test/thread",
      observed_at: "2026-06-27T12:00:00Z",
      content_hash: "hash-a",
      previous_hash: null,
      diff_summary: "New community signal",
      metadata_json: "{}",
      tier: "community_signal",
    },
  ],
  entities: [
    {
      id: 1,
      entity_type: "company",
      name: "Acme Development LLC",
      normalized_name: "acme development llc",
      first_seen_at: "2026-06-27T12:00:00Z",
      last_seen_at: "2026-06-27T12:00:00Z",
      mention_count: 2,
    },
  ],
  source_scores: [
    {
      source_id: 1,
      source_name: "Town forum",
      reliability_score: 72,
      usefulness_score: 84,
      fetch_successes: 3,
      fetch_failures: 1,
      new_items: 4,
      changed_items: 1,
      entity_hits: 6,
      dark_signal_hits: 2,
      last_fetch_at: "2026-06-27T12:00:00Z",
      updated_at: "2026-06-27T12:00:00Z",
    },
  ],
  dark_signals: [
    {
      id: 5,
      observation_id: 10,
      source_id: 1,
      title: "Review community signal from Town forum",
      summary: "Rumor says an out of state company bought land quietly.",
      origin: "Town forum",
      risk_level: "high",
      rank_score: 91,
      tier: "community_signal",
      evidence_policy: "editor_review_only",
      why_it_matters: "Community or weakly verified signals can reveal early civic risk.",
      verification_path: "Check land records and agenda packets before drafting.",
      publication_status: "review",
      created_at: "2026-06-27T12:00:00Z",
      updated_at: "2026-06-27T12:00:00Z",
      entities: [
        {
          id: 1,
          entity_type: "company",
          name: "Acme Development LLC",
          normalized_name: "acme development llc",
          first_seen_at: "2026-06-27T12:00:00Z",
          last_seen_at: "2026-06-27T12:00:00Z",
          mention_count: 2,
        },
      ],
    },
  ],
};

describe("DarkSignalDesk", () => {
  it("preserves and explains community signals instead of hiding them", () => {
    render(<DarkSignalDesk intelligence={snapshot} loading={false} onRefresh={vi.fn()} onCreateLead={vi.fn()} />);

    expect(screen.getByText("Dark Signal Desk")).toBeInTheDocument();
    expect(screen.getByText("Review community signal from Town forum")).toBeInTheDocument();
    expect(screen.getByText("editor review only")).toBeInTheDocument();
    expect(screen.getByText("Check land records and agenda packets before drafting.")).toBeInTheDocument();
    expect(screen.getByText("company: Acme Development LLC")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /create story lead/i })).toBeInTheDocument();
  });

  it("lets the editor refresh the desk", () => {
    const onRefresh = vi.fn();
    render(<DarkSignalDesk intelligence={snapshot} loading={false} onRefresh={onRefresh} onCreateLead={vi.fn()} />);

    fireEvent.click(screen.getByRole("button", { name: /refresh/i }));
    expect(onRefresh).toHaveBeenCalled();
  });

  it("lets the editor create a story lead from a signal", () => {
    const onCreateLead = vi.fn();
    render(<DarkSignalDesk intelligence={snapshot} loading={false} onRefresh={vi.fn()} onCreateLead={onCreateLead} />);

    fireEvent.click(screen.getByRole("button", { name: /create story lead/i }));
    expect(onCreateLead).toHaveBeenCalledWith(5);
  });
});
