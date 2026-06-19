// src/components/PairDialog.tsx
import React from "react";
import { FileDown } from "lucide-react";
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
    <div style={{ maxWidth: "850px", margin: "0 auto" }} id="pairing-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Browser Pairing</h1>
          <p>Securely connect web browsers or external AI coding plugins (Codex, Agent Pipeline) to access story queues and evidence records.</p>
        </div>
      </div>

      <div className="pairing-grid" id="pairing-grid">
        {/* Left Column: Generate Pin & Extension Setup */}
        <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
          <div className="card" id="card-extension-setup">
            <h3 className="card-title">1. Browser Extension Setup</h3>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Install the CivicNews browser extension to capture articles, send to queue, and highlight evidence as you browse.
            </p>
            <ol style={{ marginLeft: "1.5rem", marginBottom: "1rem", fontSize: "0.9rem", color: "var(--text-secondary)" }}>
              <li>Open <strong>chrome://extensions</strong> in your browser.</li>
              <li>Enable <strong>Developer Mode</strong> (top right).</li>
              <li>Click the button below to open the extension folder, then <strong>drag and drop</strong> the folder into the extensions page.</li>
            </ol>
            <button className="btn btn-secondary" onClick={onOpenExtensionFolder} id="btn-open-extension-folder">
              <FileDown size={16} /> Open Extension Folder
            </button>
          </div>

          <div className="card" id="card-pair-token-generator">
            <h3 className="card-title">2. Pair Assistant</h3>
            <p className="help-text">
              Pairing enables Chrome extensions to query local data via read-only APIs on port <code>12053</code>. Write operations are blocked.
            </p>

            <form onSubmit={onGeneratePin} style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }} id="form-generate-pin">
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Client Label</label>
                <input
                  type="text"
                  placeholder="e.g. My Chrome Extension"
                  value={pairingLabel}
                  onChange={(e) => onPairingLabelChange(e.target.value)}
                  required
                  id="input-pairing-label"
                />
              </div>
              <button className="btn btn-primary" type="submit" id="btn-submit-generate-pin">
                Generate Pairing Token
              </button>
            </form>

            {generatedPin && (
              <div id="pin-display-box">
                <div className="pairing-pin-box" style={{ wordBreak: "break-all", fontSize: "1rem" }} data-testid="pin-display">{generatedPin}</div>
                <p className="help-text text-center" style={{ color: "var(--color-warning)" }} id="pin-expiry-text">{pinExpiryMsg}</p>
              </div>
            )}
          </div>
        </div>

        {/* Right Column: Paired Clients list */}
        <div className="card" id="card-paired-clients-list">
          <h3 className="card-title">Active Pairings</h3>
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }} id="active-pairings-list">
            {pairedClients.length === 0 ? (
              <p className="help-text">No active browser or assistant pairings registered.</p>
            ) : (
              pairedClients.map((client) => (
                <div key={client.id} className="card" style={{ padding: "1rem", marginBottom: "0", background: "var(--bg-app)" }} data-testid={`paired-client-${client.id}`}>
                  <div className="flex-between">
                    <div>
                      <strong>{client.label}</strong>
                      <div className="help-text" style={{ fontSize: "0.75rem" }}>
                        Token: <code>{client.token.slice(0, 12)}...</code>
                      </div>
                      <div className="help-text" style={{ fontSize: "0.75rem" }}>
                        Added: {client.created_at ? new Date(client.created_at).toLocaleDateString() : ""}
                      </div>
                    </div>
                    <button className="btn btn-danger btn-sm" onClick={() => onRevokeClient(client.id!)} id={`btn-revoke-${client.id}`}>
                      Revoke
                    </button>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
