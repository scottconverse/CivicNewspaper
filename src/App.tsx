import { useApp } from "./useApp";
import { Layout } from "./components/Layout";
import { AppContent } from "./components/AppContent";
import "./App.css";

function App() {
  const app = useApp();

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
