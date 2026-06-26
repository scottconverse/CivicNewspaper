import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DailyScanPage } from "./DailyScanPage";

vi.mock("./DailyScanResults", () => ({
  DailyScanResults: ({ scanId }: { scanId: number }) => <div>Results for {scanId}</div>,
}));

describe("DailyScanPage", () => {
  it("shows accurate local scan progress without fake percentages", () => {
    render(
      <DailyScanPage
        latestScanId={null}
        leadCount={3}
        draftCount={0}
        sourceCount={2}
        loading={true}
        ollamaOnline={true}
        dailyScanProgress={{
          stage: "generating",
          message: "Scanning batch 2 of 5.",
          model: "qwen2.5:7b",
          evidence_count: 17,
          batch_index: 2,
          batch_count: 5,
          saved_leads: 4,
        }}
        onRunScan={vi.fn()}
        onRefresh={vi.fn()}
      />
    );

    expect(screen.getByTestId("daily-scan-progress")).toBeInTheDocument();
    expect(screen.getByText("Scanning batch 2 of 5.")).toBeInTheDocument();
    expect(screen.getByText(/Model: qwen2.5:7b/)).toBeInTheDocument();
    expect(screen.getByText(/Evidence: 17/)).toBeInTheDocument();
    expect(screen.getByText(/Saved leads: 4/)).toBeInTheDocument();
    expect(screen.getByText(/Batch 2 of 5/)).toBeInTheDocument();
    expect(screen.queryByText(/%/)).not.toBeInTheDocument();
  });
});
