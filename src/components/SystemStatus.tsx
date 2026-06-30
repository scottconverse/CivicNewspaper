// src/components/SystemStatus.tsx
import React, { useState } from "react";
import { Cpu, Database, Cpu as ScraperIcon, ShieldAlert, Download } from "lucide-react";
import { exportDiagnostics, toUserMessage } from "../ipc";
import { downloadDir, join } from "@tauri-apps/api/path";

interface SystemStatusProps {
  ollamaOnline: boolean;
  dbVersion: string;
  appVersion: string;
}

export const SystemStatus: React.FC<SystemStatusProps> = ({
  ollamaOnline,
  dbVersion,
  appVersion
}) => {
  const [exportStatus, setExportStatus] = useState<string>("");

  const handleExportDiagnostics = async () => {
    try {
      setExportStatus("Exporting...");
      const downloads = await downloadDir();
      const path = await join(downloads, "civicnews-diagnostics.json");
      await exportDiagnostics(path);
      setExportStatus(`Exported diagnostics to ${path}`);
    } catch (e) {
      setExportStatus(`Export failed: ${toUserMessage(e)}`);
    }
  };

  return (
    <div className="card" id="system-status-panel" style={{ maxWidth: "600px", margin: "2rem auto" }}>
      <div className="flex-between" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem", marginBottom: "1rem" }}>
        <h1 className="card-title" style={{ borderBottom: "none", paddingBottom: 0, marginBottom: 0, fontSize: "1.1rem" }}>
          <Cpu size={20} style={{ marginRight: "0.5rem" }} />
          System Resources & Status
        </h1>
        <button className="btn btn-secondary" onClick={handleExportDiagnostics} aria-label="Export diagnostic report" style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <Download size={16} />
          Export Diagnostic Report
        </button>
      </div>
      
      {exportStatus && (
        <div style={{ marginBottom: "1rem", padding: "0.5rem", backgroundColor: "var(--bg-app)", borderRadius: "4px", fontSize: "0.9rem" }}>
          {exportStatus}
        </div>
      )}

      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        {/* Ollama Connection */}
        <div className="flex-between" style={{ padding: "0.5rem 0", borderBottom: "1px solid var(--border-color)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <Cpu size={16} style={{ color: "var(--text-secondary)" }} />
            <span>Local AI Service</span>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <span 
              className={`status-dot ${ollamaOnline ? "online" : "offline"}`} 
              data-testid="ollama-status-dot"
            />
            <span style={{ fontWeight: 600 }} data-testid="ollama-status-text">
              {ollamaOnline ? "Online" : "Offline"}
            </span>
          </div>
        </div>

        {/* DB Version */}
        <div className="flex-between" style={{ padding: "0.5rem 0", borderBottom: "1px solid var(--border-color)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <Database size={16} style={{ color: "var(--text-secondary)" }} />
            <span>SQLite Schema Version</span>
          </div>
          <strong data-testid="db-version-text">{dbVersion || "v1.1.0"}</strong>
        </div>

        {/* Scraper Status */}
        <div className="flex-between" style={{ padding: "0.5rem 0", borderBottom: "1px solid var(--border-color)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <ScraperIcon size={16} style={{ color: "var(--text-secondary)" }} />
            <span>Source Scanner</span>
          </div>
          {/* UX-n2: there's no live scraper-state signal in the frontend, so a
              hardcoded green "Ready" badge was a false status. State it honestly:
              the scanner runs on demand when you click "Scrape & Detect". */}
          <span className="badge badge-neutral" data-testid="scraper-status-text">Runs on demand</span>
        </div>

        {/* App Version */}
        <div className="flex-between" style={{ padding: "0.5rem 0" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <ShieldAlert size={16} style={{ color: "var(--text-secondary)" }} />
            <span>Build Release version</span>
          </div>
          <strong data-testid="app-version-text">v{appVersion}</strong>
        </div>
      </div>
    </div>
  );
};
