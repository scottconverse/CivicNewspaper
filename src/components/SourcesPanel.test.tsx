// src/components/SourcesPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { SourcesPanel } from "./SourcesPanel";
import { Source } from "../ipc";
import { BulkImportReview } from "../bulkImportParser";

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

  const emptyReview: BulkImportReview = { accepted: [], rejected: [], duplicates: [] };

  const renderPanel = (overrides: Partial<React.ComponentProps<typeof SourcesPanel>> = {}) => {
    const props: React.ComponentProps<typeof SourcesPanel> = {
      sources: mockSources,
      loading: false,
      newSourceName: "",
      onNewSourceNameChange: vi.fn(),
      newSourceUrl: "",
      onNewSourceUrlChange: vi.fn(),
      newSourceType: "primary_record",
      onNewSourceTypeChange: vi.fn(),
      newSourceTier: "official_record",
      onNewSourceTierChange: vi.fn(),
      onAddSource: vi.fn(),
      onDeleteSource: vi.fn(),
      showBulkImportModal: false,
      onShowBulkImportModalChange: vi.fn(),
      bulkImportText: "",
      onBulkImportTextChange: vi.fn(),
      bulkImportType: "primary_record",
      onBulkImportTypeChange: vi.fn(),
      bulkImportLoading: false,
      bulkImportReview: emptyReview,
      onBuildBulkImportReview: vi.fn(),
      onToggleBulkImportItem: vi.fn(),
      onChooseBulkImportFile: vi.fn(),
      onBulkImport: vi.fn(),
      showDiscoveryModal: false,
      onShowDiscoveryModalChange: vi.fn(),
      discoveryCity: "",
      onDiscoveryCityChange: vi.fn(),
      discoveryState: "",
      onDiscoveryStateChange: vi.fn(),
      discoveryLoading: false,
      onRunDiscovery: vi.fn(),
      discoveredCats: [],
      selectedDiscovered: [],
      onToggleDiscoveredSource: vi.fn(),
      onImportDiscoveredSources: vi.fn(),
      onClearDiscovered: vi.fn(),
      ...overrides,
    };
    render(<SourcesPanel {...props} />);
    return props;
  };

  test("renders sources list and delete triggers callback", () => {
    const handleDeleteSource = vi.fn();
    renderPanel({ onDeleteSource: handleDeleteSource });

    // Verify list renders the mock source
    expect(screen.getByText("City Council Feed")).toBeInTheDocument();
    expect(screen.getByText("https://city.gov/rss")).toBeInTheDocument();
    expect(screen.getByText("Online")).toBeInTheDocument();

    // Click delete and verify callback
    const deleteBtn = screen.getByLabelText("Delete source");
    fireEvent.click(deleteBtn);
    expect(handleDeleteSource).toHaveBeenCalledWith(1);
  });

  test("opens bulk import URLs modal and submits pasted URLs", () => {
    const handleBulkImport = vi.fn((e: React.FormEvent) => e.preventDefault());
    const handleBulkImportTextChange = vi.fn();

    renderPanel({
      showBulkImportModal: true,
      bulkImportText: "https://city.gov/rss",
      onBulkImportTextChange: handleBulkImportTextChange,
      onBulkImport: handleBulkImport,
    });

    expect(screen.getByRole("dialog", { name: "Bulk Import Sources" })).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Source list"), {
      target: { value: "https://county.gov/notices" },
    });
    expect(handleBulkImportTextChange).toHaveBeenCalledWith("https://county.gov/notices");

    fireEvent.click(screen.getByRole("button", { name: "Review List" }));
    expect(handleBulkImport).toHaveBeenCalledTimes(1);
  });

  test("renders bulk import review and toggles reviewed rows", () => {
    const handleToggle = vi.fn();
    renderPanel({
      showBulkImportModal: true,
      bulkImportText: "City, https://city.gov/rss",
      bulkImportReview: {
        accepted: [{
          id: "1-https://city.gov/rss",
          row: 1,
          name: "City",
          url: "https://city.gov/rss",
          type: "primary_record",
          tier: "official_record",
          credibility: "Official record",
          review_note: "Likely a primary civic source.",
          selected: true,
        }],
        duplicates: [{ row: 2, text: "https://city.gov/rss", reason: "Duplicate URL already in this import or source list." }],
        rejected: [{ row: 3, text: "not a url", reason: "No valid http(s) URL found." }],
      },
      onToggleBulkImportItem: handleToggle,
    });

    expect(screen.getByTestId("bulk-import-review")).toBeInTheDocument();
    expect(screen.getByText("Official record")).toBeInTheDocument();
    expect(screen.getByText(/1 importable, 1 duplicate, 1 skipped/i)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("checkbox"));
    expect(handleToggle).toHaveBeenCalledWith("1-https://city.gov/rss");
  });

  test("opens find URLs modal and runs source discovery", () => {
    const handleRunDiscovery = vi.fn((e: React.FormEvent) => e.preventDefault());
    const handleCityChange = vi.fn();
    const handleStateChange = vi.fn();

    renderPanel({
      showDiscoveryModal: true,
      discoveryCity: "Riverton",
      onDiscoveryCityChange: handleCityChange,
      discoveryState: "OH",
      onDiscoveryStateChange: handleStateChange,
      onRunDiscovery: handleRunDiscovery,
    });

    expect(screen.getByRole("dialog", { name: "Town Setup & Source Auto-Discovery" })).toBeInTheDocument();
    fireEvent.change(screen.getByPlaceholderText("City Name (e.g. Brighton)"), {
      target: { value: "Dayton" },
    });
    fireEvent.change(screen.getByPlaceholderText("State (e.g. CO)"), {
      target: { value: "PA" },
    });
    expect(handleCityChange).toHaveBeenCalledWith("Dayton");
    expect(handleStateChange).toHaveBeenCalledWith("PA");

    fireEvent.click(screen.getByRole("button", { name: "Auto-Find Feeds" }));
    expect(handleRunDiscovery).toHaveBeenCalledTimes(1);
  });
});
