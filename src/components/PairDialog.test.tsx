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

  test("clicking Generate Token, pasting token, and revoking paired client actions", () => {
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
    const generateBtn = screen.getByRole("button", { name: /Generate Pairing Token/i });
    fireEvent.click(generateBtn);
    expect(handleGeneratePin).toHaveBeenCalled();

    // 2. Pasted token input accepts 22 chars
    const pasteInput = screen.getByTestId("input-paste-token") as HTMLInputElement;
    fireEvent.change(pasteInput, { target: { value: "1234567890123456789012" } });
    expect(pasteInput.value).toBe("1234567890123456789012");
    expect(pasteInput.value.length).toBe(22);

    // 3. Revoke button on a paired client fires with correct id
    const revokeBtn = screen.getByRole("button", { name: /Revoke/i });
    fireEvent.click(revokeBtn);
    expect(handleRevoke).toHaveBeenCalledWith(7);
  });
});
