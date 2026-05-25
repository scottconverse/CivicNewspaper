// src/components/SystemStatus.tsx
import React, { useState } from "react";
import { Cpu, Database, Cpu as ScraperIcon, ShieldAlert, Download } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";

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
      const path = await save({
        defaultPath: 'civicnews-diagnostics.json',
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (path) {
        setExportStatus("Exporting...");
        await invoke('export_diagnostics', { path });
        setExportStatus("Export successful!");
        setTimeout(() => setExportStatus(""), 3000);
      }
    } catch (e) {
      setExportStatus(`Export failed: ${e}`);
    }
  };

  return (
    <div className="card" id="system-status-panel" style={{ maxWidth: "600px", margin: "2rem auto" }}>
      <div className="flex-between" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem", marginBottom: "1rem" }}>
        <h3 className="card-title" style={{ borderBottom: "none", paddingBottom: 0, marginBottom: 0 }}>
          <Cpu size={20} style={{ marginRight: "0.5rem" }} />
          System Resources & Status
        </h3>
        <button className="btn btn-secondary" onClick={handleExportDiagnostics} style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <Download size={16} />
          Export Diagnostic Report
        </button>
      </div>
      
      {exportStatus && (
        <div style={{ marginBottom: "1rem", padding: "0.5rem", backgroundColor: "var(--bg-secondary)", borderRadius: "4px", fontSize: "0.9rem" }}>
          {exportStatus}
        </div>
      )}

      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        {/* Ollama Connection */}
        <div className="flex-between" style={{ padding: "0.5rem 0", borderBottom: "1px solid var(--border-color)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <Cpu size={16} style={{ color: "var(--text-secondary)" }} />
            <span>Ollama Inference Engine</span>
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
            <span>OSINT Scraper Agent</span>
          </div>
          <span className="badge badge-success" data-testid="scraper-status-text">Ready / Idle</span>
        </div>

        {/* Daily Scan */}
        <div className="flex-between" style={{ padding: "0.5rem 0", borderBottom: "1px solid var(--border-color)" }}>
          <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <ScraperIcon size={16} style={{ color: "var(--text-secondary)" }} />
            <span>Daily Scan</span>
          </div>
          <button className="btn btn-primary btn-sm" onClick={() => { invoke('run_daily_scan', { city: 'Local', state: 'ST', sinceHours: 24 }).catch(console.error); }}>
            Trigger Scan
          </button>
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
