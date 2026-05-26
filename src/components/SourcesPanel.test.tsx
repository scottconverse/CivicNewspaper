// src/components/SourcesPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { SourcesPanel } from "./SourcesPanel";
import { Source } from "../ipc";

describe("SourcesPanel Component Tests", () => {
  const mockSources: Source[] = [
    {
      id: 1,
      name: "City Council Feed",
      url: "https://city.gov/rss",
      type: "primary_record",
      tier: "official_record",
      status: "online",
      last_success_at: "2026-05-23T00:00:00Z",
      last_failed_at: undefined,
      last_scraped: "2026-05-23T00:00:00Z"
    }
  ];

  test("renders sources list and delete triggers callback", () => {
    const handleDeleteSource = vi.fn();
    const handleAddSource = vi.fn();

    render(
      <SourcesPanel
        sources={mockSources}
        loading={false}
        newSourceName=""
        onNewSourceNameChange={vi.fn()}
        newSourceUrl=""
        onNewSourceUrlChange={vi.fn()}
        newSourceType="primary_record"
        onNewSourceTypeChange={vi.fn()}
        newSourceTier="official_record"
        onNewSourceTierChange={vi.fn()}
        onAddSource={handleAddSource}
        onDeleteSource={handleDeleteSource}
        showBulkImportModal={false}
        onShowBulkImportModalChange={vi.fn()}
        bulkImportText=""
        onBulkImportTextChange={vi.fn()}
        bulkImportType="primary_record"
        onBulkImportTypeChange={vi.fn()}
        bulkImportLoading={false}
        onBulkImport={vi.fn()}
        showDiscoveryModal={false}
        onShowDiscoveryModalChange={vi.fn()}
        discoveryCity=""
        onDiscoveryCityChange={vi.fn()}
        discoveryState=""
        onDiscoveryStateChange={vi.fn()}
        discoveryLoading={false}
        onRunDiscovery={vi.fn()}
        discoveredCats={[]}
        selectedDiscovered={[]}
        onToggleDiscoveredSource={vi.fn()}
        onImportDiscoveredSources={vi.fn()}
        onClearDiscovered={vi.fn()}
      />
    );

    // Verify list renders the mock source
    expect(screen.getByText("City Council Feed")).toBeInTheDocument();
    expect(screen.getByText("https://city.gov/rss")).toBeInTheDocument();

    // Click delete and verify callback
    const deleteBtn = screen.getByLabelText("Delete source");
    fireEvent.click(deleteBtn);
    expect(handleDeleteSource).toHaveBeenCalledWith(1);
  });
});
