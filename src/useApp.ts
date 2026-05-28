// src/useApp.ts
import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { documentDir, join } from "@tauri-apps/api/path";
import {
  getSources,
  addSource,
  deleteSource,
  generatePairingPin,
  listPairedClients,
  revokePairing,
  getCommunityProfile,
  saveCommunityProfile,
  ingest,
  getQueue,
  getEvidence,
  saveDraft,
  deleteDraft,
  storyDecision,
  generateDraft,
  llmTask,
  guardrailsCheck,
  publish,
  registerCorrection,
  backupSave,
  backupRestore,
  checkOllama,
  pullModel,
  ollamaHealth,
  getSystemRam,
  discoverSources,
  Source,
  EvidenceItem,
  Lead,
  Draft,
  PairedClient,
  CommunityProfile,
  DiscoveredSource,
  DiscoveredSourceCategory,
  GuardrailsReport,
  runDailyScan
} from "./ipc";
import modelsConfig from "./models.json";

export function useApp() {
  // Navigation
  const [activeTab, setActiveTab] = useState<string>("queue");
  const [onboardingStep, setOnboardingStep] = useState<number>(1);

  // Updater
  const [updateAvailable, setUpdateAvailable] = useState<any>(null);

  // App Data
  const [sources, setSources] = useState<Source[]>([]);
  const [leads, setLeads] = useState<Lead[]>([]);
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const [pairedClients, setPairedClients] = useState<PairedClient[]>([]);
  const [communityProfile, setCommunityProfile] = useState<CommunityProfile | null>(null);

  // Selection & Details
  const [selectedLead, setSelectedLead] = useState<Lead | null>(null);
  const [selectedDraft, setSelectedDraft] = useState<Draft | null>(null);
  const [evidenceList, setEvidenceList] = useState<EvidenceItem[]>([]);
  const [guardrailsReport, setGuardrailsReport] = useState<GuardrailsReport | null>(null);

  // Forms / Modals state
  const [newSourceName, setNewSourceName] = useState("");
  const [newSourceUrl, setNewSourceUrl] = useState("");
  const [newSourceType, setNewSourceType] = useState("primary_record");
  const [newSourceTier, setNewSourceTier] = useState("community_signal");
  
  const [pairingLabel, setPairingLabel] = useState("");
  const [generatedPin, setGeneratedPin] = useState<string | null>(null);
  const [pinExpiryMsg, setPinExpiryMsg] = useState("");

  const [draftFormat, setDraftFormat] = useState("watch");
  const [customSystemPrompt, setCustomSystemPrompt] = useState("");
  const [generatingText, setGeneratingText] = useState(false);

  const [publishPath, setPublishPath] = useState("");
  const [backupPathInput, setBackupPathInput] = useState("");

  const [correctionNote, setCorrectionNote] = useState("");
  const [showCorrectionModal, setShowCorrectionModal] = useState(false);

  // Discovery State
  const [showDiscoveryModal, setShowDiscoveryModal] = useState(false);
  const [discoveryCity, setDiscoveryCity] = useState("");
  const [discoveryState, setDiscoveryState] = useState("");
  const [discoveryLoading, setDiscoveryLoading] = useState(false);
  const [discoveredCats, setDiscoveredCats] = useState<DiscoveredSourceCategory[]>([]);
  const [selectedDiscovered, setSelectedDiscovered] = useState<DiscoveredSource[]>([]);

  // Bulk Import State
  const [showBulkImportModal, setShowBulkImportModal] = useState(false);
  const [bulkImportText, setBulkImportText] = useState("");
  const [bulkImportType, setBulkImportType] = useState("primary_record");
  const [bulkImportLoading, setBulkImportLoading] = useState(false);

  const [correctionDraftId, setCorrectionDraftId] = useState<number | null>(null);

  // Social Media Pack State
  const [socialPackResult, setSocialPackResult] = useState("");
  const [isGeneratingSocial, setIsGeneratingSocial] = useState(false);

  // Publishing Wizard
  const [publishStep, setPublishStep] = useState(1);

  // Ollama & Wizard
  const [ollamaOnline, setOllamaOnline] = useState(false);
  const [systemRam, setSystemRam] = useState<number>(8);
  const [wizardModel, setWizardModel] = useState("");
  const [pullingModel, setPullingModel] = useState(false);
  const [pullProgressText, setPullProgressText] = useState<string[]>([]);
  const [manualLlmMode, setManualLlmMode] = useState(false);
  const [customLlmPrompt, setCustomLlmPrompt] = useState("");
  const [customLlmSystem, setCustomLlmSystem] = useState("You are a helpful assistant.");
  const [customLlmResult, setCustomLlmResult] = useState("");
  const [customLlmRunning, setCustomLlmRunning] = useState(false);

  const [latestScanId, setLatestScanId] = useState<number | null>(null);

  // Real application version, read from the Tauri bundle at runtime.
  const [appVersion, setAppVersion] = useState("");

  // Global Status Feed
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const pullLogEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function loadDefaultPaths() {
      try {
        const docDir = await documentDir();
        setPublishPath(await join(docDir, "civicnews-site"));
        setBackupPathInput(await join(docDir, "civicnews-backup.db"));
      } catch (err) {
        console.error("Failed to resolve default paths", err);
      }
    }
    loadDefaultPaths();

    async function loadAppVersion() {
      try {
        const { getVersion } = await import("@tauri-apps/api/app");
        setAppVersion(await getVersion());
      } catch (err) {
        console.error("Failed to resolve app version", err);
      }
    }
    loadAppVersion();

    async function checkForUpdates() {
      try {
        const { check } = await import('@tauri-apps/plugin-updater');
        const update = await check();
        if (update) {
          setUpdateAvailable(update);
        }
      } catch (err) {
        console.error("Updater check failed", err);
      }
    }
    checkForUpdates();
  }, []);

  // Initial Load
  useEffect(() => {
    loadInitialData();
    pollOllamaStatus();
    
    getSystemRam().then(async (ram) => {
      setSystemRam(ram);
      try {
        let model = await invoke<string | null>("get_setting", { key: "model.selected" });
        if (!model) {
          model = ram >= 12 ? modelsConfig.high : ram >= 8 ? modelsConfig.medium : modelsConfig.low;
          await invoke("set_setting", { key: "model.selected", value: model });
        }
        setWizardModel(model);
      } catch (err) {
        console.error("Failed to load or initialize selected model setting", err);
      }
    }).catch(console.error);

    const setupListeners = async () => {
      const progressUnlisten = await listen<string>("ollama-pull-progress", (event) => {
        try {
          const parsed = JSON.parse(event.payload);
          let progressLine = parsed.status || "Downloading...";
          if (parsed.completed && parsed.total) {
            const pct = Math.round((parsed.completed / parsed.total) * 100);
            progressLine += ` (${pct}%)`;
          }
          setPullProgressText(prev => [...prev.slice(-30), progressLine]);
        } catch {
          setPullProgressText(prev => [...prev.slice(-30), event.payload]);
        }
      });

      const completeUnlisten = await listen<void>("ollama-pull-complete", () => {
        setPullingModel(false);
        setPullProgressText(prev => [...prev, "✓ Model pulled successfully!"]);
        pollOllamaStatus();
      });

      const errorUnlisten = await listen<string>("ollama-pull-error", (event) => {
        setPullingModel(false);
        setPullProgressText(prev => [...prev, `Error: ${event.payload}`]);
      });

      return () => {
        progressUnlisten();
        completeUnlisten();
        errorUnlisten();
      };
    };

    const cleanupListeners = setupListeners();
    return () => {
      cleanupListeners.then(cleanup => cleanup && cleanup());
    };
  }, []);

  useEffect(() => {
    if (pullLogEndRef.current) {
      pullLogEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [pullProgressText]);

  const loadInitialData = async () => {
    try {
      setLoading(true);
      const s = await getSources();
      setSources(s);

      const q = await getQueue();
      setLeads(q.leads || []);
      setDrafts(q.drafts || []);

      const p = await getCommunityProfile();
      setCommunityProfile(p);

      const clients = await listPairedClients();
      setPairedClients(clients);

      try {
        const model = await invoke<string | null>("get_setting", { key: "model.selected" });
        if (model) {
          setWizardModel(model);
        }
      } catch (err) {
        console.error("Failed to load selected model setting", err);
      }

      setErrorMessage("");
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const pollOllamaStatus = async () => {
    try {
      const status = await checkOllama();
      setOllamaOnline(status);
    } catch {
      setOllamaOnline(false);
    }
  };

  const handleIngest = async () => {
    try {
      setLoading(true);
      setStatusMessage("Scraping feeds and running OSINT detectors... (this may take a few moments)");
      setErrorMessage("");
      const newLeadsCount = await ingest();
      setStatusMessage(`Ingest complete. Detected ${newLeadsCount} new lead(s).`);
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleDailyScan = async () => {
    try {
      setLoading(true);
      setStatusMessage("Checking AI model presence...");
      setErrorMessage("");

      const model = await invoke<string | null>("get_setting", { key: "model.selected" });
      if (!model) {
        setErrorMessage("Daily Scan requires a selected AI model, but none was configured.");
        setOnboardingStep(3);
        setActiveTab("onboarding");
        return;
      }
      const health = await ollamaHealth();
      if (!health.reachable || !health.models.some(m => m.includes(model))) {
        setErrorMessage(`Daily Scan requires the ${model} model, which was not found. Redirecting to model download setup...`);
        setOnboardingStep(3);
        setActiveTab("onboarding");
        return;
      }

      setStatusMessage("Running daily scan on evidence using the aggregator prompt...");
      const city = communityProfile?.city || "Brighton";
      const state = communityProfile?.state || "CO";
      const scanId = await runDailyScan(city, state, 24);
      setLatestScanId(scanId);
      setStatusMessage(`Daily Scan complete (Scan ID: ${scanId}).`);
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleAddSource = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newSourceName || !newSourceUrl) return;
    try {
      setLoading(true);
      await addSource(newSourceName, newSourceUrl, newSourceType, newSourceTier);
      setNewSourceName("");
      setNewSourceUrl("");
      setStatusMessage("Source added successfully.");
      const s = await getSources();
      setSources(s);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteSource = async (id: number) => {
    try {
      setLoading(true);
      await deleteSource(id);
      setStatusMessage("Source deleted.");
      const s = await getSources();
      setSources(s);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleRunDiscovery = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!discoveryCity || !discoveryState) return;
    try {
      setDiscoveryLoading(true);
      setErrorMessage("");
      const results = await discoverSources(discoveryCity, discoveryState);
      setDiscoveredCats(results);
      const allDiscovered: DiscoveredSource[] = [];
      results.forEach(cat => {
        cat.candidates.forEach(cand => {
          allDiscovered.push(cand);
        });
      });
      setSelectedDiscovered(allDiscovered);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setDiscoveryLoading(false);
    }
  };

  const handleToggleDiscoveredSource = (source: DiscoveredSource) => {
    setSelectedDiscovered(prev => {
      const exists = prev.some(item => item.url === source.url);
      if (exists) {
        return prev.filter(item => item.url !== source.url);
      } else {
        return [...prev, source];
      }
    });
  };

  const handleImportDiscoveredSources = async () => {
    try {
      setLoading(true);
      setStatusMessage("Importing selected sources...");
      let importedCount = 0;
      for (const item of selectedDiscovered) {
        try {
          await addSource(item.name, item.url, item.type, "community_signal");
          importedCount++;
        } catch (err) {
          console.error("Failed to add discovered source:", item.name, err);
        }
      }
      setStatusMessage(`Successfully imported ${importedCount} source(s).`);
      setShowDiscoveryModal(false);
      setDiscoveryCity("");
      setDiscoveryState("");
      setDiscoveredCats([]);
      setSelectedDiscovered([]);
      const s = await getSources();
      setSources(s);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleBulkImport = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!bulkImportText.trim()) return;
    try {
      setBulkImportLoading(true);
      setErrorMessage("");
      setStatusMessage("Bulk importing sources...");
      const lines = bulkImportText.split("\n");
      let importedCount = 0;
      for (let line of lines) {
        line = line.trim();
        if (!line) continue;

        let name = "";
        let url = "";
        let type = bulkImportType;

        if (line.includes(",")) {
          const parts = line.split(",").map(p => p.trim());
          if (parts.length >= 2) {
            if (parts[0].startsWith("http://") || parts[0].startsWith("https://")) {
              url = parts[0];
              name = parts[1];
              if (parts.length >= 3 && parts[2]) {
                type = parts[2];
              }
            } else {
              name = parts[0];
              url = parts[1];
              if (parts.length >= 3 && parts[2]) {
                type = parts[2];
              }
            }
          }
        } else {
          url = line;
          try {
            const parsedUrl = new URL(url);
            name = parsedUrl.hostname.replace("www.", "");
          } catch {
            name = url;
          }
        }

        if (url.startsWith("http://") || url.startsWith("https://")) {
          const validTypes = ["primary_record", "official_comm", "community_signal", "media_lead"];
          if (!validTypes.includes(type)) {
            type = bulkImportType;
          }
          await addSource(name, url, type, "community_signal");
          importedCount++;
        }
      }
      setStatusMessage(`Bulk imported ${importedCount} source(s) successfully.`);
      setShowBulkImportModal(false);
      setBulkImportText("");
      const s = await getSources();
      setSources(s);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setBulkImportLoading(false);
    }
  };

  const handleGeneratePin = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!pairingLabel) return;
    try {
      setLoading(true);
      const pin = await generatePairingPin(pairingLabel);
      setGeneratedPin(pin);
      setPinExpiryMsg("PIN expires in 5 minutes. Enter this PIN in your browser extension config.");
      setPairingLabel("");
      const clients = await listPairedClients();
      setPairedClients(clients);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleRevokeClient = async (id: number) => {
    try {
      setLoading(true);
      await revokePairing(id);
      setStatusMessage("Paired client access revoked.");
      const clients = await listPairedClients();
      setPairedClients(clients);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleSaveProfile = async (profile: CommunityProfile) => {
    try {
      setLoading(true);
      await saveCommunityProfile(profile);
      setCommunityProfile(profile);
      setStatusMessage("Ethics standard and community profile updated.");
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleOpenDraftWizard = (lead: Lead) => {
    setSelectedLead(lead);
    setSelectedDraft(null);
    setEvidenceList([]);
    setGuardrailsReport(null);
    if (lead.id) {
      getEvidence(lead.id).then(setEvidenceList).catch(console.error);
    }
  };

  const handleGenerateText = async () => {
    if (!selectedLead || !selectedLead.id) return;
    try {
      setGeneratingText(true);
      setErrorMessage("");
      setStatusMessage("Asking local Ollama model to write a draft from evidence... (this may take a moment)");
      const text = await generateDraft(
        selectedLead.id,
        draftFormat,
        customSystemPrompt ? customSystemPrompt : undefined
      );

      const draftObj: Draft = {
        lead_id: selectedLead.id,
        format: draftFormat,
        title: `Draft: ${selectedLead.why.slice(0, 40)}...`,
        content: text,
        status: "draft_generated",
        verification_checklist: "[]"
      };

      const newId = await saveDraft(draftObj);
      draftObj.id = newId;

      setSelectedDraft(draftObj);
      setActiveTab("workbench");
      setStatusMessage("Draft generated successfully.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(`Ollama drafting failed: ${e.toString()}.`);
    } finally {
      setGeneratingText(false);
    }
  };

  const handleOpenDraftEditor = async (draft: Draft) => {
    setSelectedDraft(draft);
    setSelectedLead(null);
    setGuardrailsReport(null);
    setSocialPackResult("");
    try {
      if (draft.lead_id) {
        const ev = await getEvidence(draft.lead_id);
        setEvidenceList(ev);
      } else {
        setEvidenceList([]);
      }
      setActiveTab("workbench");
      if (draft.id) {
        const report = await guardrailsCheck(draft.id);
        setGuardrailsReport(report);
      }
    } catch (e: any) {
      setErrorMessage(e.toString());
    }
  };

  const handleSaveDraftEditor = async () => {
    if (!selectedDraft) return;
    try {
      setLoading(true);
      setErrorMessage("");
      const id = await saveDraft(selectedDraft);
      setSelectedDraft({ ...selectedDraft, id });
      setStatusMessage("Draft saved.");
      
      const report = await guardrailsCheck(id);
      setGuardrailsReport(report);
      
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleDecision = async (status: string) => {
    if (!selectedDraft || !selectedDraft.id) return;
    try {
      setLoading(true);
      await storyDecision(selectedDraft.id, status);
      setSelectedDraft({ ...selectedDraft, status });
      setStatusMessage(`Story status updated to '${status}'.`);
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handlePublish = async () => {
    if (!publishPath) return;
    try {
      setLoading(true);
      setErrorMessage("");
      setStatusMessage(`Compiling HTML, CSS, and RSS templates to static site at: ${publishPath}...`);
      await publish(publishPath);
      setStatusMessage("Static Newspaper compiled successfully!");
      setPublishStep(3);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const openCorrectionModal = (draftId: number) => {
    setCorrectionDraftId(draftId);
    setCorrectionNote("");
    setShowCorrectionModal(true);
  };

  const handleRegisterCorrection = async () => {
    if (correctionDraftId === null || !correctionNote) return;
    try {
      setLoading(true);
      await registerCorrection(correctionDraftId, correctionNote);
      setShowCorrectionModal(false);
      setStatusMessage("Correction added and appended to public log.");
      if (selectedDraft && selectedDraft.id === correctionDraftId) {
        setSelectedDraft({
          ...selectedDraft,
          status: "corrected",
          correction_note: correctionNote
        });
      }
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteDraft = async (id: number) => {
    if (!confirm("Are you sure you want to delete this draft?")) return;
    try {
      setLoading(true);
      await deleteDraft(id);
      setSelectedDraft(null);
      setActiveTab("queue");
      setStatusMessage("Draft deleted.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleGenerateSocial = async () => {
    if (!selectedDraft || !selectedDraft.content) return;
    try {
      setIsGeneratingSocial(true);
      setErrorMessage("");
      setStatusMessage("Asking Ollama to generate social media promo pack...");
      
      const systemPrompt = "You are a social media manager for a local news organization.";
      const promptText = `Please create a social media pack for this story:\n\nTitle: ${selectedDraft.title}\n\nContent:\n${selectedDraft.content}`;
      
      const result = await llmTask(promptText, systemPrompt);
      setSocialPackResult(result);
      setStatusMessage("Social media pack generated!");
    } catch (e: any) {
      setErrorMessage(`Failed to generate social posts: ${e.toString()}`);
    } finally {
      setIsGeneratingSocial(false);
    }
  };

  const handleBackupSave = async () => {
    if (!backupPathInput) return;
    try {
      setLoading(true);
      setErrorMessage("");
      await backupSave(backupPathInput);
      setStatusMessage(`Backup database successfully written to: ${backupPathInput}`);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handleBackupRestore = async () => {
    if (!backupPathInput) return;
    if (!confirm("WARNING: Proceed?")) return;
    try {
      setLoading(true);
      setErrorMessage("");
      await backupRestore(backupPathInput);
      setStatusMessage("Database successfully restored from backup.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  const handlePullModel = () => {
    if (!wizardModel) return;
    setPullingModel(true);
    setPullProgressText(["Initializing download..."]);
    pullModel(wizardModel).catch((e) => {
      setPullingModel(false);
      setErrorMessage(e.toString());
    });
  };

  const handleCustomLlmTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!customLlmPrompt) return;
    try {
      setCustomLlmRunning(true);
      setErrorMessage("");
      setCustomLlmResult("");
      const result = await llmTask(customLlmPrompt, customLlmSystem);
      setCustomLlmResult(result);
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setCustomLlmRunning(false);
    }
  };

  return {
    activeTab,
    setActiveTab,
    updateAvailable,
    sources,
    leads,
    drafts,
    pairedClients,
    communityProfile,
    selectedLead,
    setSelectedLead,
    selectedDraft,
    setSelectedDraft,
    evidenceList,
    guardrailsReport,
    newSourceName,
    setNewSourceName,
    newSourceUrl,
    setNewSourceUrl,
    newSourceType,
    setNewSourceType,
    newSourceTier,
    setNewSourceTier,
    pairingLabel,
    setPairingLabel,
    generatedPin,
    pinExpiryMsg,
    draftFormat,
    setDraftFormat,
    customSystemPrompt,
    setCustomSystemPrompt,
    generatingText,
    publishPath,
    setPublishPath,
    backupPathInput,
    setBackupPathInput,
    correctionNote,
    setCorrectionNote,
    showCorrectionModal,
    setShowCorrectionModal,
    showDiscoveryModal,
    setShowDiscoveryModal,
    discoveryCity,
    setDiscoveryCity,
    discoveryState,
    setDiscoveryState,
    discoveryLoading,
    discoveredCats,
    selectedDiscovered,
    showBulkImportModal,
    setShowBulkImportModal,
    bulkImportText,
    setBulkImportText,
    bulkImportType,
    setBulkImportType,
    bulkImportLoading,
    socialPackResult,
    setSocialPackResult,
    isGeneratingSocial,
    publishStep,
    setPublishStep,
    ollamaOnline,
    systemRam,
    wizardModel,
    setWizardModel,
    pullingModel,
    pullProgressText,
    onboardingStep,
    setOnboardingStep,
    manualLlmMode,
    setManualLlmMode,
    customLlmPrompt,
    setCustomLlmPrompt,
    customLlmSystem,
    setCustomLlmSystem,
    customLlmResult,
    customLlmRunning,
    latestScanId,
    appVersion,
    loading,
    statusMessage,
    setStatusMessage,
    errorMessage,
    setErrorMessage,
    pullLogEndRef,
    loadInitialData,
    pollOllamaStatus,
    handleIngest,
    handleDailyScan,
    handleAddSource,
    handleDeleteSource,
    handleRunDiscovery,
    handleToggleDiscoveredSource,
    handleImportDiscoveredSources,
    handleBulkImport,
    handleGeneratePin,
    handleRevokeClient,
    handleSaveProfile,
    handleOpenDraftWizard,
    handleGenerateText,
    handleOpenDraftEditor,
    handleSaveDraftEditor,
    handleDecision,
    handlePublish,
    openCorrectionModal,
    handleRegisterCorrection,
    handleDeleteDraft,
    handleGenerateSocial,
    handleBackupSave,
    handleBackupRestore,
    handlePullModel,
    handleCustomLlmTask
  };
}
