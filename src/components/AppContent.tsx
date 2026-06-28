// src/components/AppContent.tsx
import React from "react";
import { LeadQueue } from "./LeadQueue";
import { Workbench } from "./Workbench";
import { PairDialog } from "./PairDialog";
import { SettingsPanel } from "./SettingsPanel";
import { PublishPanel } from "./PublishPanel";
import { SourcesPanel } from "./SourcesPanel";
import { DailyScanPage } from "./DailyScanPage";
import { DarkSignalDesk } from "./DarkSignalDesk";
import { VerificationQueue } from "./VerificationQueue";
import { AiModelPanel } from "./AiModelPanel";
import { Modal } from "./Modal";
import { ConfirmModal } from "./ConfirmModal";
import { BetaNotice } from "./BetaNotice";
import { SystemStatus } from "./SystemStatus";
import { getBrowserExtensionPath, openExternalUrl, openLocalPath, toUserMessage } from "../ipc";

interface AppContentProps {
  app: any;
}

export const AppContent: React.FC<AppContentProps> = ({ app }) => {
  const [extensionFolderStatus, setExtensionFolderStatus] = React.useState("");
  const [extensionFolderPath, setExtensionFolderPath] = React.useState("");

  // UX-C2: auto-dismiss success banners so a stale "success" message doesn't
  // linger across unrelated navigation. Errors are left until the user dismisses
  // them (or the next action clears errorMessage) so failures aren't missed.
  React.useEffect(() => {
    if (!app.statusMessage) return;
    const t = setTimeout(() => app.setStatusMessage(""), 6000);
    return () => clearTimeout(t);
  }, [app.statusMessage]);

  return (
    <>
      {/* First-run beta notice (#12): unsigned-beta / SmartScreen disclosure */}
      <BetaNotice />

      {/* Global confirmation dialog (destructive actions) */}
      {app.confirmDialog && (
        <ConfirmModal
          title={app.confirmDialog.title}
          message={app.confirmDialog.message}
          confirmLabel={app.confirmDialog.confirmLabel}
          danger={app.confirmDialog.danger}
          onConfirm={app.handleConfirmDialogConfirm}
          onCancel={app.closeConfirmDialog}
        />
      )}

      {/* Global Notifications */}
      {app.statusMessage && (
        <div className="card" role="status" aria-live="polite" style={{ borderLeft: "4px solid var(--color-success)", background: "rgba(16, 185, 129, 0.05)" }}>
          <div className="flex-between">
            <span style={{ fontSize: "0.9rem", color: "var(--text-primary)" }}>{app.statusMessage}</span>
            <button className="btn btn-secondary btn-sm" onClick={() => app.setStatusMessage("")}>Dismiss</button>
          </div>
        </div>
      )}

      {app.errorMessage && (
        <div className="card" role="alert" aria-live="assertive" style={{ borderLeft: "4px solid var(--color-error)", background: "rgba(239, 68, 68, 0.05)" }}>
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
            drafts={app.drafts}
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
            onOpenDraftEditor={app.handleOpenDraftEditor}
            onDeleteDraft={app.handleDeleteDraft}
            onDecision={app.handleDecision}
            onApprovePublish={app.handleApprovePublish}
            onKillStory={app.handleKillStory}
            isGeneratingSocial={app.isGeneratingSocial}
            socialPackResult={app.socialPackResult}
            onSocialPackResultChange={app.setSocialPackResult}
            onGenerateSocial={app.handleGenerateSocial}
            onUpdateDraftTitle={(title) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, title })}
            onUpdateDraftContent={(content) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, content })}
            firstAmendmentAdvisorEnabled={app.communityProfile?.first_amendment_advisor_enabled !== false}
          />
        ) : (
          <LeadQueue
            leads={app.leads}
            drafts={app.drafts}
            loading={app.loading}
            latestScanId={app.latestScanId}
            sourceCount={app.sources.length}
            onGoToSources={() => app.setActiveTab("sources")}
            onSelect={(id) => {
              const lead = app.leads.find((item: any) => item.id === id);
              if (lead) app.handleOpenDraftWizard(lead);
            }}
            onSyncList={app.loadInitialData}
            onIngest={app.handleIngest}
            onDailyScan={app.handleDailyScan}
            onOpenDraftEditor={app.handleOpenDraftEditor}
            onOpenCorrectionModal={app.openCorrectionModal}
            onDeleteDraft={app.handleDeleteDraft}
          />
        )
      )}

      {app.activeTab === "dailyScan" && (
        <DailyScanPage
          latestScanId={app.latestScanId}
          leadCount={app.leads.length}
          draftCount={app.drafts.length}
          sourceCount={app.sources.length}
          loading={app.loading}
          ollamaOnline={app.ollamaOnline}
          dailyScanProgress={app.dailyScanProgress}
          onRunScan={app.handleDailyScan}
          onRefresh={app.loadInitialData}
          onGoToSources={() => app.setActiveTab("sources")}
        />
      )}

      {app.activeTab === "darkSignals" && (
        <DarkSignalDesk
          intelligence={app.civicIntelligence}
          loading={app.loading}
          onRefresh={app.refreshCivicIntelligence}
          onCreateLead={app.handleCreateLeadFromDarkSignal}
        />
      )}

      {app.activeTab === "verification" && (
        <VerificationQueue
          queue={app.verificationQueue}
          loading={app.loading}
          onRefresh={app.refreshVerificationQueue}
          onStatusChange={app.handleVerificationTaskStatus}
          onCreateLead={app.handleCreateLeadFromDarkSignal}
        />
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
          bulkImportReview={app.bulkImportReview}
          onBuildBulkImportReview={app.handleBuildBulkImportReview}
          onToggleBulkImportItem={app.handleToggleBulkImportItem}
          onChooseBulkImportFile={app.handleChooseBulkImportFile}
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
        <AiModelPanel
          ollamaOnline={app.ollamaOnline}
          systemRam={app.systemRam}
          wizardModel={app.wizardModel}
          installedModels={app.installedModels}
          onWizardModelChange={app.setWizardModel}
          pullingModel={app.pullingModel}
          pullProgressText={app.pullProgressText}
          onPullModel={app.handlePullModel}
          onRetryStatus={app.pollOllamaStatus}
          onOpenSystem={() => app.setActiveTab("system")}
        />
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
          extensionFolderStatus={extensionFolderStatus}
          extensionFolderPath={extensionFolderPath}
          onOpenExtensionFolder={async () => {
            try {
              const extPath = await getBrowserExtensionPath();
              setExtensionFolderPath(extPath);
              await openLocalPath(extPath);
              const msg = "Extension folder handoff requested. If File Explorer did not come forward, use this path.";
              setExtensionFolderStatus(msg);
              app.setStatusMessage(msg);
            } catch (e) {
              const message = `Couldn't open the browser extension folder: ${toUserMessage(e)}`;
              setExtensionFolderStatus(message);
              app.setErrorMessage(message);
            }
          }}
        />
      )}

      {app.activeTab === "settings" && (
        <SettingsPanel
          communityProfile={app.communityProfile}
          onSaveProfile={app.handleSaveProfile}
          onChooseLogo={app.handleChooseLogo}
          backupPathInput={app.backupPathInput}
          onBackupPathInputChange={app.setBackupPathInput}
          onBackupSave={app.handleBackupSave}
          onBackupRestore={app.handleBackupRestore}
        />
      )}

      {/* GG-M3: mount the previously-dead SystemStatus so a returning user has a
          reachable status/diagnostics surface (the only place to export a
          diagnostic report after onboarding). */}
      {app.activeTab === "system" && (
        <SystemStatus
          ollamaOnline={app.ollamaOnline}
          dbVersion=""
          appVersion={app.appVersion}
        />
      )}

      {app.activeTab === "publish" && (
        <PublishPanel
          publishPath={app.publishPath}
          publishResult={app.publishResult}
          publishHistory={app.publishHistory}
          communityProfile={app.communityProfile}
          onOpenSettings={() => app.setActiveTab("settings")}
          publisherConfig={app.publisherConfig}
          publisherProvider={app.publisherProvider}
          publisherTestResult={app.publisherTestResult}
          subscribers={app.subscribers}
          subscriberEmail={app.subscriberEmail}
          subscriberName={app.subscriberName}
          onSubscriberEmailChange={app.setSubscriberEmail}
          onSubscriberNameChange={app.setSubscriberName}
          onPublishPathChange={app.setPublishPath}
          publishStep={app.publishStep}
          onPublishStepChange={(step) => {
            app.setPublishStep(step);
            if (step === 2) {
              app.setStatusMessage("Review the compile checklist, then click Compile site to write files.");
            }
          }}
          loading={app.loading}
          onPublish={app.handlePublish}
          onOpenLocalPath={async (path, label) => {
            try {
              await openLocalPath(path);
              app.setStatusMessage(label ? `Opened ${label}.` : "Opened folder.");
            } catch (e) {
              app.setErrorMessage(`Couldn't open folder: ${toUserMessage(e)}`);
            }
          }}
          onOpenExternalUrl={async (url) => {
            try {
              await openExternalUrl(url);
              app.setStatusMessage("Opened publishing destination.");
            } catch (e) {
              app.setErrorMessage(`Couldn't open publishing destination: ${toUserMessage(e)}`);
            }
          }}
          onChoosePublishPath={app.handleChoosePublishPath}
          onRecordPublishDestination={app.handleRecordPublishDestination}
          onPublishWithConnector={app.handlePublishWithConnector}
          onLoadPublisherConfig={app.handleLoadPublisherConfig}
          onSavePublisherConfig={app.handleSavePublisherConfig}
          onTestPublisherConnection={app.handleTestPublisherConnection}
          onAddSubscriber={app.handleAddSubscriber}
          onDeleteSubscriber={app.handleDeleteSubscriber}
          onImportSubscribersCsv={app.handleImportSubscribersCsv}
          onExportSubscribersCsv={app.handleExportSubscribersCsv}
          onExportIssueEmail={app.handleExportIssueEmail}
          onCopyPublishText={app.handleCopyPublishText}
          onCopyPublishArtifact={app.handleCopyPublishArtifact}
        />
      )}

      {app.activeTab === "workbench" && (
        <Workbench
          selectedLead={app.selectedLead}
          selectedDraft={app.selectedDraft}
          drafts={app.drafts}
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
          onOpenDraftEditor={app.handleOpenDraftEditor}
          onDeleteDraft={app.handleDeleteDraft}
          onDecision={app.handleDecision}
          onApprovePublish={app.handleApprovePublish}
          onKillStory={app.handleKillStory}
          isGeneratingSocial={app.isGeneratingSocial}
          socialPackResult={app.socialPackResult}
          onSocialPackResultChange={app.setSocialPackResult}
          onGenerateSocial={app.handleGenerateSocial}
          onUpdateDraftTitle={(title) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, title })}
          onUpdateDraftContent={(content) => app.selectedDraft && app.setSelectedDraft({ ...app.selectedDraft, content })}
          firstAmendmentAdvisorEnabled={app.communityProfile?.first_amendment_advisor_enabled !== false}
        />
      )}

      {/* Correction Modal */}
      {app.showCorrectionModal && (
        <Modal labelledBy="correction-modal-title" onClose={() => app.setShowCorrectionModal(false)}>
            <h3 id="correction-modal-title" style={{ marginBottom: "1rem" }}>Register Story Correction</h3>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Entering a correction marks the story status as <code>corrected</code>, and appends a public retraction note directly on the static site compiler output.
            </p>
            <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
              <div>
                <label htmlFor="textarea-correction-note" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Correction Note</label>
                <textarea
                  placeholder="e.g. Corrected date of zoning hearing from June 5 to June 15..."
                  value={app.correctionNote}
                  onChange={(e) => app.setCorrectionNote(e.target.value)}
                  style={{ height: "120px" }}
                  required
                  id="textarea-correction-note"
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
        </Modal>
      )}
    </>
  );
};
