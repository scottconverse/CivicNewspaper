// src/components/Layout.tsx
import React from "react";
import {
  Bot,
  FileText,
  Globe2,
  Link as LinkIcon,
  Newspaper,
  Rss,
  ScanSearch,
  Settings,
} from "lucide-react";

interface LayoutProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
  ollamaOnline: boolean;
  selectedDraft: any;
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({
  activeTab,
  onTabChange,
  ollamaOnline,
  selectedDraft: _selectedDraft,
  children
}) => {
  const navGroups = [
    {
      label: "Newsroom",
      items: [
        { id: "queue", label: "Story Queue", icon: Newspaper },
        { id: "dailyScan", label: "Daily Scan", icon: ScanSearch },
        { id: "workbench", label: "Workbench", icon: FileText },
      ],
    },
    {
      label: "Set up",
      items: [
        { id: "sources", label: "Sources", icon: Rss },
        { id: "onboarding", label: "AI Model", icon: Bot },
        { id: "publish", label: "Publishing", icon: Globe2 },
      ],
    },
    {
      label: "Tools",
      items: [
        { id: "pairing", label: "Browser Pairing", icon: LinkIcon },
        { id: "settings", label: "Ethics & Backups", icon: Settings },
      ],
    },
  ];

  return (
    <div className="app-container">
      <aside className="sidebar">
        <div className="brand">
          <span className="brand-icon-shell">
            <Newspaper className="brand-icon" aria-hidden="true" />
          </span>
          <div>
            <span className="brand-name">The Civic Desk</span>
            <span className="brand-kicker">RIVERTON - OHIO</span>
          </div>
        </div>
        <nav>
          {navGroups.map((group) => (
            <div className="nav-group" key={group.label}>
              <div className="nav-group-label">{group.label}</div>
              <ul className="nav-links">
                {group.items.map((item) => {
                  const Icon = item.icon;
                  return (
                    <li key={item.id}>
                      <button
                        className={`nav-link ${activeTab === item.id ? "active" : ""}`}
                        onClick={() => onTabChange(item.id)}
                        id={`nav-tab-${item.id}`}
                      >
                        <Icon size={18} />
                        {item.label}
                      </button>
                    </li>
                  );
                })}
              </ul>
            </div>
          ))}
        </nav>

        <div className="sidebar-footer">
          <div className="ollama-status-indicator">
            <span className={`status-dot ${ollamaOnline ? "online" : "offline"}`} />
            <div>
              <strong>{ollamaOnline ? "Local AI ready" : "Local AI offline"}</strong>
              <span>qwen3:14b - private</span>
            </div>
          </div>
        </div>
      </aside>

      <main className="main-content">
        {children}
      </main>
    </div>
  );
};
