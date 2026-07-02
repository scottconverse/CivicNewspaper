import { useApp } from "./useApp";
import { Layout } from "./components/Layout";
import { AppContent } from "./components/AppContent";
import { OnboardingWizard } from "./components/OnboardingWizard";
import "./App.css";

function App() {
  const app = useApp();
  const buildId = import.meta.env.VITE_CIVICNEWS_BUILD_ID || "dev";

  // GG-C4: a brand-new user is walked through the guided setup before reaching
  // the workspace. `onboardingDone === null` means we're still checking.
  if (app.onboardingDone === null) {
    return (
      <>
        <div id="civicnews-build-id" data-build-id={buildId} hidden />
        <div style={{ padding: "2rem", color: "var(--text-secondary)" }}>Loading The Civic Desk...</div>
      </>
    );
  }
  if (!app.onboardingDone) {
    return (
      <div className="onboarding-shell">
        <div id="civicnews-build-id" data-build-id={buildId} hidden />
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
    [app.communityProfile?.city, app.communityProfile?.state].filter(Boolean).join(" / ") ||
    "Local newsroom";
  const modelLabel = app.selectedModel || "No model selected";

  return (
    <>
      <div id="civicnews-build-id" data-build-id={buildId} hidden />
      <Layout
        activeTab={app.activeTab}
        onTabChange={(tab) => {
          app.setErrorMessage("");
          app.setStatusMessage("");
          app.setActiveTab(tab);
          app.setSelectedLead(null);
          app.setSelectedDraft(null);
        }}
        ollamaOnline={app.ollamaOnline}
        selectedDraft={app.selectedDraft}
        kicker={kicker}
        modelLabel={modelLabel}
        aiSetupSkipped={app.aiSetupSkipped}
      >
        <AppContent app={app} />
      </Layout>
    </>
  );
}

export default App;
