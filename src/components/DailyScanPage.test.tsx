import { render, screen } from "@testing-library/react";
import type { ComponentProps } from "react";
import { describe, expect, it, vi } from "vitest";
import { DailyScanPage } from "./DailyScanPage";

vi.mock("./DailyScanResults", () => ({
  DailyScanResults: ({ scanId }: { scanId: number }) => <div>Results for {scanId}</div>,
}));

function renderPage(overrides: Partial<ComponentProps<typeof DailyScanPage>> = {}) {
  const props: ComponentProps<typeof DailyScanPage> = {
    latestScanId: null,
    leadCount: 0,
    draftCount: 0,
    sourceCount: 2,
    loading: false,
    ollamaOnline: true,
    dailyScanProgress: null,
    onRunScan: vi.fn(),
    onRefresh: vi.fn(),
    onGoToSources: vi.fn(),
    ...overrides,
  };
  render(<DailyScanPage {...props} />);
  return props;
}

describe("DailyScanPage", () => {
  it("shows accurate local scan progress without fake percentages", () => {
    renderPage({
      leadCount: 3,
      loading: true,
      dailyScanProgress: {
        stage: "generating",
        message: "Scanning batch 2 of 5.",
        model: "phi4-mini:latest",
        evidence_count: 17,
        batch_index: 2,
        batch_count: 5,
        saved_leads: 4,
      },
    });

    expect(screen.getByTestId("daily-scan-progress")).toBeInTheDocument();
    expect(screen.getByText("Scanning batch 2 of 5.")).toBeInTheDocument();
    expect(screen.getByText(/Model: phi4-mini:latest/)).toBeInTheDocument();
    expect(screen.getByText(/Evidence: 17/)).toBeInTheDocument();
    expect(screen.getByText(/Saved leads: 4/)).toBeInTheDocument();
    expect(screen.getByText(/Batch 2 of 5/)).toBeInTheDocument();
    expect(screen.queryByText(/%/)).not.toBeInTheDocument();
  });

  it("labels the deterministic evidence stage before AI review", () => {
    renderPage({
      sourceCount: 1,
      loading: true,
      ollamaOnline: false,
      dailyScanProgress: {
        stage: "deterministic",
        message: "Extracting entities, detecting changes, and building verification tasks.",
        evidence_count: 5,
        saved_leads: 1,
      },
    });

    expect(screen.getByText("Evidence intelligence")).toBeInTheDocument();
    expect(screen.getByText(/Deterministic checks run first/)).toBeInTheDocument();
  });

  it("labels the source-fetch stage for one-button daily scans", () => {
    renderPage({
      loading: true,
      ollamaOnline: false,
      dailyScanProgress: {
        stage: "fetching",
        message: "Checking watched sources for fresh records before analysis.",
        evidence_count: 0,
        saved_leads: 0,
      },
    });

    expect(screen.getByText("Checking sources")).toBeInTheDocument();
    expect(screen.getByText("Checking watched sources for fresh records before analysis.")).toBeInTheDocument();
  });

  it("routes zero-source users to Sources instead of running an empty scan", () => {
    const runScan = vi.fn();
    const goToSources = vi.fn();

    renderPage({
      sourceCount: 0,
      ollamaOnline: false,
      onRunScan: runScan,
      onGoToSources: goToSources,
    });

    screen.getByRole("button", { name: "Go to Sources" }).click();
    expect(goToSources).toHaveBeenCalledTimes(1);
    expect(runScan).not.toHaveBeenCalled();
  });
});
