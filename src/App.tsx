import { useState, useEffect } from "react";
import { isOnboardingComplete } from "./ipc";
import { useApp } from "./useApp";
import { Layout } from "./components/Layout";
import { AppContent } from "./components/AppContent";
import { OnboardingWizard } from "./components/OnboardingWizard";
import "./App.css";

function App() {
  const app = useApp();
  const [onboardingComplete, setOnboardingComplete] = useState<boolean | null>(null);

  useEffect(() => {
    isOnboardingComplete()
      .then(setOnboardingComplete)
      .catch((e) => {
        console.error(e);
        setOnboardingComplete(false);
      });
  }, []);

  if (onboardingComplete === null) {
    return null;
  }

  if (!onboardingComplete) {
    return (
      <div style={{ position: 'fixed', top: 0, left: 0, right: 0, bottom: 0, backgroundColor: 'var(--bg-app)', zIndex: 9999, overflow: 'auto' }}>
        <div style={{ padding: '2rem', maxWidth: '800px', margin: '0 auto' }}>
          <OnboardingWizard
            ollamaOnline={app.ollamaOnline}
            systemRam={app.systemRam}
            onComplete={() => setOnboardingComplete(true)}
          />
        </div>
      </div>
    );
  }

  return (
    <Layout
      activeTab={app.activeTab}
      onTabChange={(tab) => {
        app.setActiveTab(tab);
        app.setSelectedLead(null);
      }}
      ollamaOnline={app.ollamaOnline}
      selectedDraft={app.selectedDraft}
    >
      <AppContent app={app} />
    </Layout>
  );
}

export default App;
