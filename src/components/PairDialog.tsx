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
          <p className="help-text">Install the local browser extension, generate a code, then paste it into the extension popup. The code expires in 5 minutes.</p>

          <div className="setup-steps" aria-label="Browser extension setup steps">
            <div><strong>1.</strong> Click <span>Open extension folder</span>.</div>
            <div><strong>2.</strong> In Chrome or Edge, open Extensions, turn on Developer mode, and choose Load unpacked.</div>
            <div><strong>3.</strong> Select the opened <span>chromium</span> folder, then paste the code into the extension icon popup.</div>
          </div>

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
            {!generatedPin && !pairingLabel.trim() && (
              <p className="help-text" style={{ margin: "-0.5rem 0 0" }}>
                Enter a device label before generating a code.
              </p>
            )}
            {generatedPin && (
              <p className="help-text" style={{ margin: "-0.5rem 0 0" }}>
                Paste this active code into the extension popup. Generate a replacement code only if this one expires.
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
            <span>The bridge only talks to the Civic Desk app on this computer. Nothing is exposed to the internet.</span>
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
