// src/components/PairDialog.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { PairDialog } from "./PairDialog";
import { PairedClient } from "../ipc";

describe("PairDialog Component Tests", () => {
  const mockClients: PairedClient[] = [
    {
      id: 7,
      token: "abcdefghijklmnopqrstuv",
      label: "Main Chrome Extension",
      created_at: "2026-05-23T00:00:00Z",
      revoked: false
    }
  ];

  test("clicking Generate Token and revoking paired client fire their handlers", () => {
    const handleGeneratePin = vi.fn((e) => e.preventDefault());
    const handleRevoke = vi.fn();
    const handleLabelChange = vi.fn();

    render(
      <PairDialog
        pairingLabel="Test Label"
        onPairingLabelChange={handleLabelChange}
        generatedPin={null}
        pinExpiryMsg=""
        onGeneratePin={handleGeneratePin}
        pairedClients={mockClients}
        onRevokeClient={handleRevoke}
        onOpenExtensionFolder={vi.fn()}
      />
    );

    // 1. Submit form (clicking generate button)
    const generateBtn = screen.getByRole("button", { name: /Generate new code/i });
    fireEvent.click(generateBtn);
    expect(handleGeneratePin).toHaveBeenCalled();

    // 2. Revoke button on a paired client fires with correct id
    const revokeBtn = screen.getByRole("button", { name: /Revoke/i });
    fireEvent.click(revokeBtn);
    expect(handleRevoke).toHaveBeenCalledWith(7);
  });

  test("explains how to install the unpacked browser extension", () => {
    render(
      <PairDialog
        pairingLabel=""
        onPairingLabelChange={vi.fn()}
        generatedPin={null}
        pinExpiryMsg=""
        onGeneratePin={vi.fn()}
        pairedClients={[]}
        onRevokeClient={vi.fn()}
        onOpenExtensionFolder={vi.fn()}
      />
    );

    expect(screen.getByText(/Load unpacked/i)).toBeInTheDocument();
    expect(screen.getByText(/extension icon popup/i)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Open extension folder/i })).toBeInTheDocument();
  });
});
