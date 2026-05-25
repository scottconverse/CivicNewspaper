// src/components/AppContent.tsx
import React from "react";
import { LeadQueue } from "./LeadQueue";
import { Workbench } from "./Workbench";
import { OnboardingWizard } from "./OnboardingWizard";
import { PairDialog } from "./PairDialog";
import { SettingsPanel } from "./SettingsPanel";
import { PublishPanel } from "./PublishPanel";
import { SystemStatus } from "./SystemStatus";
import { SourcesPanel } from "./SourcesPanel";
import { resolveResource } from "@tauri-apps/api/path";
import { openLocalPath } from "../ipc";

interface AppContentProps {
  app: any;
}

export const AppContent: React.FC<AppContentProps> = ({ app }) => {
  return (
    <>
      {/* Global Notifications */}
      {app.statusMessage && (
        <div className="card" style={{ borderLeft: "4px solid var(--color-success)", background: "rgba(16, 185, 129, 0.05)" }}>
          <div className="flex-between">
            <span style={{ fontSize: "0.9rem", color: "var(--text-primary)" }}>{app.statusMessage}</span>
            <button className="btn btn-secondary btn-sm" onClick={() => app.setStatusMessage("")}>Dismiss</button>
          </div>
        </div>
      )}

      {app.errorMessage && (
        <div className="card" style={{ borderLeft: "4px solid var(--color-error)", background: "rgba(239, 68, 68, 0.05)" }}>
          <div className="flex-between">
            <span style={{ fontSize: "0.9rem", color: "var(--color-error)" }}>{app.errorMessage}</span>
            <button className="btn btn-secondary btn-sm" onClick={() => app.setErrorMessage("")}>Dismiss</button>
          </div>
        </div>
      )}

      {/* Routing Views */}
      {app.activeTab === "queue" && (
        app.selectedLead ? (
          <Workbench
            selectedLead={app.selectedLead}
            selectedDraft={app.selectedDraft}
            evidenceList={app.evidenceList}
            guardrailsReport={app.guardrailsReport}
            ollamaOnline={app.ollamaOnline}
            manualLlmMode={app.manualLlmMode}
            draftFormat={app.draftFormat}
            onDraftFormatChange={app.setDraftFormat}
            customSystemPrompt={app.customSystemPrompt}
            onCustomSystemPromptChange={app.setCustomSystemPrompt}
            generatingText={app.generatingText}
            onGenerateText={app.handleGenerateText}
            onCancelDraftWizard={() => app.setSelectedLead(null)}
            onSaveDraftEditor={app.handleSaveDraftEditor}
            onCloseWorkbench={() => {
              app.setActiveTab("queue");
              app.setSelectedDraft(null);
            }}
            onDeleteDraft={app.handleDeleteDraft}
            onDecision={app.handleDecision}
            isGeneratingSocial={app.isGeneratingSocial}
            socialPackResult={app.socialPackResult}
            onSocialPackResultChange={app.setSocialPackResult}
            onGenerateSocial={app.handleGenerateSocial}
            onUpdateDraftTitle={(title) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, title })}
            onUpdateDraftContent={(content) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, content })}
          />
        ) : (
          <LeadQueue
            leads={app.leads}
            drafts={app.drafts}
            loading={app.loading}
            onSelect={app.handleOpenDraftWizard}
            onSyncList={app.loadInitialData}
            onIngest={app.handleIngest}
            onDailyScan={app.handleDailyScan}
            onOpenDraftEditor={app.handleOpenDraftEditor}
            onOpenCorrectionModal={app.openCorrectionModal}
            onDeleteDraft={app.handleDeleteDraft}
          />
        )
      )}

      {app.activeTab === "sources" && (
        <SourcesPanel
          sources={app.sources}
          loading={app.loading}
          newSourceName={app.newSourceName}
          onNewSourceNameChange={app.setNewSourceName}
          newSourceUrl={app.newSourceUrl}
          onNewSourceUrlChange={app.setNewSourceUrl}
          newSourceType={app.newSourceType}
          onNewSourceTypeChange={app.setNewSourceType}
          newSourceTier={app.newSourceTier}
          onNewSourceTierChange={app.setNewSourceTier}
          onAddSource={app.handleAddSource}
          onDeleteSource={app.handleDeleteSource}
          showBulkImportModal={app.showBulkImportModal}
          onShowBulkImportModalChange={app.setShowBulkImportModal}
          bulkImportText={app.bulkImportText}
          onBulkImportTextChange={app.setBulkImportText}
          bulkImportType={app.bulkImportType}
          onBulkImportTypeChange={app.setBulkImportType}
          bulkImportLoading={app.bulkImportLoading}
          onBulkImport={app.handleBulkImport}
          showDiscoveryModal={app.showDiscoveryModal}
          onShowDiscoveryModalChange={app.setShowDiscoveryModal}
          discoveryCity={app.discoveryCity}
          onDiscoveryCityChange={app.setDiscoveryCity}
          discoveryState={app.discoveryState}
          onDiscoveryStateChange={app.setDiscoveryState}
          discoveryLoading={app.discoveryLoading}
          onRunDiscovery={app.handleRunDiscovery}
          discoveredCats={app.discoveredCats}
          selectedDiscovered={app.selectedDiscovered}
          onToggleDiscoveredSource={app.handleToggleDiscoveredSource}
          onImportDiscoveredSources={app.handleImportDiscoveredSources}
          onClearDiscovered={() => {
            app.setDiscoveredCats([]);
            app.setSelectedDiscovered([]);
          }}
        />
      )}

      {app.activeTab === "onboarding" && (
        <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
          <OnboardingWizard
            ollamaOnline={app.ollamaOnline}
            systemRam={app.systemRam}
            onComplete={() => app.setActiveTab("queue")}
          />
          <SystemStatus
            ollamaOnline={app.ollamaOnline}
            dbVersion="v1.1.0"
            appVersion="0.1.1"
          />
        </div>
      )}

      {app.activeTab === "pairing" && (
        <PairDialog
          pairingLabel={app.pairingLabel}
          onPairingLabelChange={app.setPairingLabel}
          generatedPin={app.generatedPin}
          pinExpiryMsg={app.pinExpiryMsg}
          onGeneratePin={app.handleGeneratePin}
          pairedClients={app.pairedClients}
          onRevokeClient={app.handleRevokeClient}
          onOpenExtensionFolder={async () => {
            try {
              const extPath = await resolveResource("browser-extension");
              openLocalPath(extPath);
            } catch (e) {
              console.error("Failed to resolve extension path", e);
            }
          }}
        />
      )}

      {app.activeTab === "settings" && (
        <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
          <SettingsPanel
            communityProfile={app.communityProfile}
            onSaveProfile={app.handleSaveProfile}
            backupPathInput={app.backupPathInput}
            onBackupPathInputChange={app.setBackupPathInput}
            onBackupSave={app.handleBackupSave}
            onBackupRestore={app.handleBackupRestore}
          />
          <PublishPanel
            publishPath={app.publishPath}
            onPublishPathChange={app.setPublishPath}
            publishStep={app.publishStep}
            onPublishStepChange={app.setPublishStep}
            loading={app.loading}
            onPublish={app.handlePublish}
            onOpenLocalPath={openLocalPath}
          />
        </div>
      )}

      {app.activeTab === "workbench" && app.selectedDraft && (
        <Workbench
          selectedLead={app.selectedLead}
          selectedDraft={app.selectedDraft}
          evidenceList={app.evidenceList}
          guardrailsReport={app.guardrailsReport}
          ollamaOnline={app.ollamaOnline}
          manualLlmMode={app.manualLlmMode}
          draftFormat={app.draftFormat}
          onDraftFormatChange={app.setDraftFormat}
          customSystemPrompt={app.customSystemPrompt}
          onCustomSystemPromptChange={app.setCustomSystemPrompt}
          generatingText={app.generatingText}
          onGenerateText={app.handleGenerateText}
          onCancelDraftWizard={() => app.setSelectedLead(null)}
          onSaveDraftEditor={app.handleSaveDraftEditor}
          onCloseWorkbench={() => {
            app.setActiveTab("queue");
            app.setSelectedDraft(null);
          }}
          onDeleteDraft={app.handleDeleteDraft}
          onDecision={app.handleDecision}
          isGeneratingSocial={app.isGeneratingSocial}
          socialPackResult={app.socialPackResult}
          onSocialPackResultChange={app.setSocialPackResult}
          onGenerateSocial={app.handleGenerateSocial}
          onUpdateDraftTitle={(title) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, title })}
          onUpdateDraftContent={(content) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, content })}
        />
      )}

      {/* Correction Modal */}
      {app.showCorrectionModal && (
        <div className="modal-overlay">
          <div className="modal-content">
            <h3 style={{ marginBottom: "1rem" }}>Register Story Correction</h3>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Entering a correction marks the story status as <code>corrected</code>, and appends a public retraction note directly on the static site compiler output.
            </p>
            <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Correction Note</label>
                <textarea
                  placeholder="e.g. Corrected date of zoning hearing from June 5 to June 15..."
                  value={app.correctionNote}
                  onChange={(e) => app.setCorrectionNote(e.target.value)}
                  style={{ height: "120px" }}
                  required
                />
              </div>
              <div className="btn-group text-right" style={{ justifyContent: "flex-end" }}>
                <button className="btn btn-secondary" onClick={() => app.setShowCorrectionModal(false)}>
                  Cancel
                </button>
                <button className="btn btn-primary" onClick={app.handleRegisterCorrection}>
                  Register & Commit Note
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
