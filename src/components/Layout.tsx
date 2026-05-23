// src/components/Layout.tsx
import React from "react";
import { Newspaper, Rss, Cpu, Link as LinkIcon, FileText, Settings, BookOpen } from "lucide-react";

interface LayoutProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
  updateAvailable: any;
  ollamaOnline: boolean;
  selectedDraft: any;
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({
  activeTab,
  onTabChange,
  updateAvailable,
  ollamaOnline,
  selectedDraft,
  children
}) => {
  return (
    <div className="app-container">
      {/* Top Banner for Updates */}
      {updateAvailable && (
        <div style={{ background: '#3b82f6', color: 'white', padding: '8px', textAlign: 'center', fontWeight: 'bold' }} id="update-banner">
          Update available: {updateAvailable.version}. 
          <button 
            style={{ marginLeft: '10px', padding: '2px 8px', borderRadius: '4px', border: 'none', background: 'white', color: '#3b82f6', cursor: 'pointer' }}
            onClick={async () => {
              await updateAvailable.downloadAndInstall();
              const { relaunch } = await import('@tauri-apps/plugin-process');
              await relaunch();
            }}
          >
            Install & Restart
          </button>
        </div>
      )}

      {/* Sidebar Navigation */}
      <aside className="sidebar">
        <div className="brand">
          <Newspaper className="brand-icon" />
          <span className="brand-name">CivicNews</span>
        </div>
        <nav>
          <ul className="nav-links">
            <li>
              <button
                className={`nav-link ${activeTab === "queue" ? "active" : ""}`}
                onClick={() => onTabChange("queue")}
                id="nav-tab-queue"
              >
                <BookOpen size={18} />
                Story Queue
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "sources" ? "active" : ""}`}
                onClick={() => onTabChange("sources")}
                id="nav-tab-sources"
              >
                <Rss size={18} />
                Sources Setup
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "onboarding" ? "active" : ""}`}
                onClick={() => onTabChange("onboarding")}
                id="nav-tab-onboarding"
              >
                <Cpu size={18} />
                Ollama Wizard
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "pairing" ? "active" : ""}`}
                onClick={() => onTabChange("pairing")}
                id="nav-tab-pairing"
              >
                <LinkIcon size={18} />
                Browser Pairing
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "settings" ? "active" : ""}`}
                onClick={() => onTabChange("settings")}
                id="nav-tab-settings"
              >
                <Settings size={18} />
                Ethics & Backups
              </button>
            </li>
            {selectedDraft && (
              <li>
                <button
                  className={`nav-link ${activeTab === "workbench" ? "active" : ""}`}
                  onClick={() => onTabChange("workbench")}
                  id="nav-tab-workbench"
                >
                  <FileText size={18} />
                  Story Workbench
                </button>
              </li>
            )}
          </ul>
        </nav>

        <div className="sidebar-footer">
          <div className="ollama-status-indicator">
            <span className={`status-dot ${ollamaOnline ? "online" : "offline"}`} />
            Ollama Status: {ollamaOnline ? "Online" : "Offline"}
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="main-content">
        {children}
      </main>
    </div>
  );
};
