// src/components/SystemStatus.tsx
import React from "react";
import { Cpu, Database, Cpu as ScraperIcon, ShieldAlert } from "lucide-react";

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
  return (
    <div className="card" id="system-status-panel" style={{ maxWidth: "600px", margin: "2rem auto" }}>
      <h3 className="card-title" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem", marginBottom: "1rem" }}>
        <Cpu size={20} style={{ marginRight: "0.5rem" }} />
        System Resources & Status
      </h3>
      
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
