// src/components/PairDialog.tsx
import React from "react";
import { KeyRound, Lock, Monitor, Puzzle, Smartphone } from "lucide-react";
import { PairedClient } from "../ipc";

interface PairDialogProps {
  pairingLabel: string;
  onPairingLabelChange: (val: string) => void;
  generatedPin: string | null;
  pinExpiryMsg: string;
  onGeneratePin: (e: React.FormEvent) => void;
  pairedClients: PairedClient[];
  onRevokeClient: (id: number) => void;
  onOpenExtensionFolder: () => void;
}

export const PairDialog: React.FC<PairDialogProps> = ({
  pairingLabel,
  onPairingLabelChange,
  generatedPin,
  pinExpiryMsg,
  onGeneratePin,
  pairedClients,
  onRevokeClient,
  onOpenExtensionFolder
}) => {
  return (
    <div id="pairing-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Browser pairing</h1>
          <p>Connect the Civic Desk browser extension so you can send pages here while you read.</p>
        </div>
      </div>

      <div className="pairing-grid" id="pairing-grid">
        <div className="card" id="card-pair-token-generator">
          <h3 className="card-title">Pair a new device</h3>
          <p className="help-text">Generate a code, then type it into the extension. The code expires in 5 minutes.</p>

          <form onSubmit={onGeneratePin} style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }} id="form-generate-pin">
            <label className="sr-only" htmlFor="input-pairing-label">Device label</label>
            <input
              type="text"
              placeholder="e.g. Chrome - newsroom laptop"
              value={pairingLabel}
              onChange={(e) => onPairingLabelChange(e.target.value)}
              required
              id="input-pairing-label"
            />
            {!pairingLabel.trim() && (
              <p className="help-text" style={{ margin: "-0.5rem 0 0" }}>
                Enter a device label before generating a code.
              </p>
            )}

            <div id="pin-display-box" className="pairing-pin-box" data-testid="pin-display">
              {generatedPin || "---- ---- ----"}
              <span>{generatedPin ? pinExpiryMsg : "No active code"}</span>
            </div>

            <button className="btn btn-primary btn-full" type="submit" id="btn-submit-generate-pin" disabled={!pairingLabel.trim()}>
              <KeyRound size={16} />
              Generate new code
            </button>
          </form>

          <button className="btn btn-secondary btn-full" onClick={onOpenExtensionFolder} id="btn-open-extension-folder">
            <Puzzle size={16} />
            Open extension folder
          </button>

          <div className="pairing-lockout">
            <Lock size={18} />
            <span>Pairing only works on this computer. Nothing is exposed to the internet.</span>
          </div>
        </div>

        <div className="card" id="card-paired-clients-list">
          <h3 className="card-title">Paired devices</h3>
          <div className="paired-list" id="active-pairings-list">
            {pairedClients.length === 0 ? (
              <p className="help-text">No active browser or assistant pairings registered.</p>
            ) : (
              pairedClients.map((client) => (
                <div key={client.id} className="paired-row" data-testid={`paired-client-${client.id}`}>
                  <div className="paired-icon">{client.label.toLowerCase().includes("phone") ? <Smartphone size={18} /> : <Monitor size={18} />}</div>
                  <div>
                    <strong>{client.label}</strong>
                    <span>Paired {client.created_at ? new Date(client.created_at).toLocaleDateString() : "recently"}</span>
                  </div>
                  <button className="text-button danger" onClick={() => onRevokeClient(client.id!)} id={`btn-revoke-${client.id}`}>
                    Revoke
                  </button>
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
