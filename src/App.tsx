import { useApp } from "./useApp";
import { Layout } from "./components/Layout";
import { AppContent } from "./components/AppContent";
import { OnboardingWizard } from "./components/OnboardingWizard";
import "./App.css";

function App() {
  const app = useApp();

  // GG-C4: a brand-new user is walked through the guided setup before reaching
  // the workspace. `onboardingDone === null` means we're still checking.
  if (app.onboardingDone === null) {
    return (
      <div style={{ padding: "2rem", color: "var(--text-secondary)" }}>Loading The Civic Desk…</div>
    );
  }
  if (!app.onboardingDone) {
    return (
      <div style={{ maxWidth: 720, margin: "2rem auto", padding: "0 1rem" }}>
        <OnboardingWizard
          ollamaOnline={app.ollamaOnline}
          systemRam={app.systemRam}
          onComplete={app.completeOnboarding}
        />
      </div>
    );
  }

  // GG (de-hardcode the masthead): show the newsroom's configured location and
  // the model the user actually selected, not baked-in fixtures.
  const kicker =
    [app.communityProfile?.city, app.communityProfile?.state].filter(Boolean).join(" · ") ||
    "Local newsroom";
  const modelLabel = app.selectedModel || "No model selected";

  return (
    <Layout
      activeTab={app.activeTab}
      onTabChange={(tab) => {
        app.setActiveTab(tab);
        app.setSelectedLead(null);
      }}
      ollamaOnline={app.ollamaOnline}
      selectedDraft={app.selectedDraft}
      kicker={kicker}
      modelLabel={modelLabel}
    >
      <AppContent app={app} />
    </Layout>
  );
}

export default App;
