// src/components/Layout.tsx
import React from "react";
import {
  Bot,
  Cpu,
  FileText,
  Globe2,
  Link as LinkIcon,
  Newspaper,
  Radar,
  ShieldCheck,
  Rss,
  ScanSearch,
  Settings,
} from "lucide-react";

interface LayoutProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
  ollamaOnline: boolean;
  selectedDraft: any;
  kicker?: string;
  modelLabel?: string;
  aiSetupSkipped?: boolean;
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({
  activeTab,
  onTabChange,
  ollamaOnline,
  selectedDraft: _selectedDraft,
  kicker,
  modelLabel,
  aiSetupSkipped = false,
  children
}) => {
  const mainRef = React.useRef<HTMLElement | null>(null);
  const hasSelectedModel = Boolean(modelLabel && !/^no model selected$/i.test(modelLabel.trim()));
  const aiStatusLabel = aiSetupSkipped && !hasSelectedModel
    ? "AI limited mode"
    : !ollamaOnline
      ? "Local AI offline"
      : hasSelectedModel
      ? "Local AI ready"
      : "Choose an AI model";
  const aiStatusTone = aiSetupSkipped && !hasSelectedModel ? "needs-model" : !ollamaOnline ? "offline" : hasSelectedModel ? "ready" : "needs-model";
  const aiStatusClass = aiStatusTone === "ready" ? "online" : aiStatusTone === "needs-model" ? "warning" : "offline";
  const navGroups = [
    {
      label: "Newsroom",
      items: [
        { id: "queue", label: "Story Queue", icon: Newspaper },
        { id: "dailyScan", label: "Daily Scan", icon: ScanSearch },
        { id: "darkSignals", label: "Dark Signals", icon: Radar },
        { id: "verification", label: "Verification", icon: ShieldCheck },
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
        { id: "system", label: "System & Status", icon: Cpu },
      ],
    },
  ];

  const routeToTab = React.useCallback((tab: string) => {
    onTabChange(tab);
  }, [onTabChange]);

  React.useEffect(() => {
    const canUseWindowScrollTo =
      typeof window.scrollTo === "function" &&
      !navigator.userAgent.toLowerCase().includes("jsdom");
    try {
      if (canUseWindowScrollTo) {
        window.scrollTo({ top: 0, left: 0, behavior: "auto" });
      } else {
        document.documentElement.scrollTop = 0;
        document.documentElement.scrollLeft = 0;
        document.body.scrollTop = 0;
        document.body.scrollLeft = 0;
      }
    } catch {
      document.documentElement.scrollTop = 0;
      document.documentElement.scrollLeft = 0;
      document.body.scrollTop = 0;
      document.body.scrollLeft = 0;
    }
    const main = mainRef.current;
    if (main) {
      if (typeof main.scrollTo === "function") {
        main.scrollTo({ top: 0, left: 0, behavior: "auto" });
      } else {
        main.scrollTop = 0;
        main.scrollLeft = 0;
      }
    }
  }, [activeTab]);

  React.useEffect(() => {
    const tabIds = navGroups.flatMap((group) => group.items.map((item) => item.id));
    const tabByShortcut = new Map(tabIds.map((id, index) => [String(index + 1), id]));

    const handlePointerNavigation = (event: Event) => {
      const target = event.target as HTMLElement | null;
      const button = target?.closest<HTMLElement>("[data-nav-tab]");
      const tab = button?.dataset.navTab;
      if (!tab || !tabIds.includes(tab)) return;
      event.preventDefault();
      routeToTab(tab);
    };

    const handleKeyboardNavigation = (event: KeyboardEvent) => {
      if (!event.altKey && !event.ctrlKey) return;
      const tab = tabByShortcut.get(event.key);
      if (!tab) return;
      event.preventDefault();
      routeToTab(tab);
    };

    document.addEventListener("pointerdown", handlePointerNavigation, true);
    document.addEventListener("mousedown", handlePointerNavigation, true);
    document.addEventListener("click", handlePointerNavigation, true);
    document.addEventListener("keydown", handleKeyboardNavigation, true);

    return () => {
      document.removeEventListener("pointerdown", handlePointerNavigation, true);
      document.removeEventListener("mousedown", handlePointerNavigation, true);
      document.removeEventListener("click", handlePointerNavigation, true);
      document.removeEventListener("keydown", handleKeyboardNavigation, true);
    };
  }, [navGroups, routeToTab]);

  return (
    <div className="app-container">
      <aside className="sidebar">
        <div className="brand">
          <span className="brand-icon-shell">
            <Newspaper className="brand-icon" aria-hidden="true" />
          </span>
          <div>
            <span className="brand-name">The Civic Desk</span>
            <span className="brand-kicker">{kicker ?? "Local newsroom"}</span>
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
                        type="button"
                        data-nav-tab={item.id}
                        onPointerDown={() => routeToTab(item.id)}
                        onMouseDown={() => routeToTab(item.id)}
                        onClick={() => routeToTab(item.id)}
                        id={`nav-tab-${item.id}`}
                        aria-current={activeTab === item.id ? "page" : undefined}
                        title={`${item.label} (Alt+${navGroups.flatMap(group => group.items).findIndex(nav => nav.id === item.id) + 1})`}
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

        <div className={`sidebar-footer ai-status-${aiStatusTone}`}>
          <div className={`ollama-status-indicator ${aiStatusTone}`}>
            <span className={`status-dot ${aiStatusClass}`} />
            <div>
              <strong>{aiStatusLabel}</strong>
              <span>{modelLabel ?? "private"}</span>
            </div>
          </div>
        </div>
      </aside>

      <main className="main-content" ref={mainRef}>
        {children}
      </main>
    </div>
  );
};
