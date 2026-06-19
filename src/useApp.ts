// src/useApp.ts
import { useState, useEffect, useRef } from "react";
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
  pullOllamaModel,
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
  runDailyScan,
  getSetting,
  setSetting,
  toUserMessage
} from "./ipc";
import modelsConfig from "./models.json";
import { parseBulkImportLine } from "./bulkImportParser";

export interface ConfirmDialogState {
  title: string;
  message: string;
  confirmLabel: string;
  danger: boolean;
  onConfirm: () => void | Promise<void>;
}

export interface OllamaPullProgress {
  model?: string;
  status?: string;
  completed?: number;
  total?: number;
}

// QA-mn1: exact model-tag match. A loose `.includes()` let `phi3:mini` "match"
// `phi3:medium` (or any tag sharing a prefix), so the pre-flight could pass while
// the actual generate call later fails with "model not found." Ollama implicitly
// appends `:latest` when a bare name is requested, so we treat `name` and
// `name:latest` as equivalent and otherwise require an exact tag match.
export function modelInstalled(selected: string, installed: string[]): boolean {
  const want = selected.includes(":") ? selected : `${selected}:latest`;
  return installed.some((m) => {
    const have = m.includes(":") ? m : `${m}:latest`;
    return have === want;
  });
}

// Formats a structured `ollama-pull-progress` event into a single log line.
// The pull command (`pull_ollama_model`) emits a structured object payload, not
// a JSON string — pinning the shape here keeps the listener from regressing to
// the old string-parsing path the consolidation removed.
export function formatPullProgressLine(payload: OllamaPullProgress): string {
  let line = payload.status || "Downloading...";
  if (payload.completed && payload.total && payload.total > 0) {
    const pct = Math.round((payload.completed / payload.total) * 100);
    line += ` (${pct}%)`;
  }
  return line;
}

export function useApp() {
  // Navigation
  const [activeTab, setActiveTab] = useState<string>("queue");
  const [onboardingStep, setOnboardingStep] = useState<number>(1);

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

  // Generic confirmation dialog (replaces native window.confirm so destructive
  // actions get consequence-specific copy in the styled, accessible Modal).
  const [confirmDialog, setConfirmDialog] = useState<ConfirmDialogState | null>(null);

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
  }, []);

  // Initial Load
  useEffect(() => {
    loadInitialData();
    pollOllamaStatus();
    
    getSystemRam().then(async (ram) => {
      setSystemRam(ram);
      try {
        let model = await getSetting("model.selected");
        if (!model) {
          model = ram >= 16 ? modelsConfig.high : ram >= 8 ? modelsConfig.medium : modelsConfig.low;
          await setSetting("model.selected", model);
        }
        setWizardModel(model);
      } catch (err) {
        console.error("Failed to load or initialize selected model setting", err);
      }
    }).catch(console.error);

    const setupListeners = async () => {
      const progressUnlisten = await listen<OllamaPullProgress>("ollama-pull-progress", (event) => {
        const progressLine = formatPullProgressLine(event.payload);
        setPullProgressText(prev => [...prev.slice(-30), progressLine]);
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

      // ENG-Min-R1: the Rust backend emits `http-server-error` when the local
      // pairing server can't bind (e.g. port 12053 already in use). Without a
      // listener the failure is silent. Surface it through the existing error
      // banner so the user understands why browser pairing is unavailable.
      const serverErrorUnlisten = await listen<string>("http-server-error", (event) => {
        const detail = event.payload || "another app may be using port 12053.";
        setErrorMessage(`Browser pairing is unavailable: ${detail}`);
      });

      return () => {
        progressUnlisten();
        completeUnlisten();
        errorUnlisten();
        serverErrorUnlisten();
      };
    };

    const cleanupListeners = setupListeners();

    // QA-R2-M1: the bundled Ollama sidecar takes a moment to bind 127.0.0.1:11434
    // after launch, so the single mount poll can lose the cold-start race and
    // leave `ollamaOnline=false` stuck — disabling Generate Draft and showing
    // "AI Offline" on a healthy sidecar. Re-poll on an interval AND whenever the
    // window regains focus / becomes visible, so a transient offline state
    // self-heals without an app relaunch. Lightweight: pollOllamaStatus is a
    // single 2s-timeout health GET, and it never spams the pull flow (the pull's
    // own completion handler already triggers a status refresh).
    const statusInterval = window.setInterval(() => {
      pollOllamaStatus();
    }, 10000);

    const handleVisibility = () => {
      if (document.visibilityState === "visible") {
        pollOllamaStatus();
      }
    };
    window.addEventListener("focus", pollOllamaStatus);
    document.addEventListener("visibilitychange", handleVisibility);

    return () => {
      cleanupListeners.then(cleanup => cleanup && cleanup());
      window.clearInterval(statusInterval);
      window.removeEventListener("focus", pollOllamaStatus);
      document.removeEventListener("visibilitychange", handleVisibility);
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
        const model = await getSetting("model.selected");
        if (model) {
          setWizardModel(model);
        }
      } catch (err) {
        console.error("Failed to load selected model setting", err);
      }

      // QA-mn3: restore the most recent scan id so its results render on relaunch.
      try {
        const savedScanId = await getSetting("scan.latest_id");
        if (savedScanId) {
          const parsed = parseInt(savedScanId, 10);
          if (!Number.isNaN(parsed)) {
            setLatestScanId(parsed);
          }
        }
      } catch (err) {
        console.error("Failed to load latest scan id setting", err);
      }

      setErrorMessage("");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
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
      setStatusMessage("Scraping feeds and scanning for story leads... (this may take a few moments)");
      setErrorMessage("");
      const newLeadsCount = await ingest();
      setStatusMessage(`Ingest complete. Detected ${newLeadsCount} new lead(s).`);
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleDailyScan = async () => {
    try {
      setLoading(true);
      setStatusMessage("Checking AI model presence...");
      setErrorMessage("");

      const model = await getSetting("model.selected");
      if (!model) {
        setErrorMessage("Daily Scan requires a selected AI model, but none was configured.");
        setOnboardingStep(3);
        setActiveTab("onboarding");
        return;
      }
      const health = await ollamaHealth();
      if (!health.reachable || !modelInstalled(model, health.models)) {
        setErrorMessage(`Daily Scan requires the ${model} model, which was not found. Redirecting to model download setup...`);
        setOnboardingStep(3);
        setActiveTab("onboarding");
        return;
      }

      setStatusMessage("Running the daily scan across your collected evidence...");
      const city = communityProfile?.city || "Brighton";
      const state = communityProfile?.state || "CO";
      const scanId = await runDailyScan(city, state, 24);
      setLatestScanId(scanId);
      // QA-mn3: persist the latest scan id so the results view survives a
      // relaunch instead of disappearing until the next scan.
      try {
        await setSetting("scan.latest_id", String(scanId));
      } catch (err) {
        console.error("Failed to persist latest scan id", err);
      }
      setStatusMessage(`Daily Scan complete (Scan ID: ${scanId}).`);
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      for (const rawLine of lines) {
        const parsed = parseBulkImportLine(rawLine, bulkImportType);
        if (!parsed) continue;
        await addSource(parsed.name, parsed.url, parsed.type, "community_signal");
        importedCount++;
      }
      setStatusMessage(`Bulk imported ${importedCount} source(s) successfully.`);
      setShowBulkImportModal(false);
      setBulkImportText("");
      const s = await getSources();
      setSources(s);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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

      // QA-C1: mirror handleDailyScan's pre-flight. The Generate Draft button is
      // gated only on the sidecar being reachable, not on the selected model
      // actually being installed — so a user who skipped the model download could
      // click it and hit an opaque "model not found." Check model presence first
      // and route to the model-download step instead of failing cryptically.
      if (!manualLlmMode) {
        setStatusMessage("Checking AI model presence...");
        const model = await getSetting("model.selected");
        if (!model) {
          setErrorMessage("Generating a draft requires a selected AI model, but none was configured. Redirecting to model download setup...");
          setOnboardingStep(3);
          setActiveTab("onboarding");
          return;
        }
        const health = await ollamaHealth();
        if (!health.reachable || !modelInstalled(model, health.models)) {
          setErrorMessage(`Generating a draft requires the ${model} model, which isn't downloaded yet. Redirecting to model download setup...`);
          setOnboardingStep(3);
          setActiveTab("onboarding");
          return;
        }
      }

      setStatusMessage("Asking the local AI model to write a draft from evidence... (this may take a moment)");
      const text = await generateDraft(
        selectedLead.id,
        draftFormat,
        customSystemPrompt ? customSystemPrompt : undefined
      );

      // UX-m2: persist a clean working title instead of an ellipsis-truncated
      // stub like "Draft: City approves new zoning…". Use the full lead summary,
      // collapsed to a single line; the editor lets the user rename it.
      const cleanTitle = selectedLead.why.replace(/\s+/g, " ").trim();
      const draftObj: Draft = {
        lead_id: selectedLead.id,
        format: draftFormat,
        title: cleanTitle ? `Draft: ${cleanTitle}` : "Untitled draft",
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
      setErrorMessage(`Draft generation failed: ${toUserMessage(e)}`);
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  // UX-m5: "Kill Story" is destructive and was a single unguarded click, unlike
  // draft delete which is confirmed. Route it through the same confirm dialog.
  const handleKillStory = () => {
    if (!selectedDraft || !selectedDraft.id) return;
    setConfirmDialog({
      title: "Kill this story?",
      message:
        "Killing this story marks it as killed and removes it from the publishing pipeline. You can reopen it later, but any in-progress review state is cleared.",
      confirmLabel: "Kill story",
      danger: true,
      onConfirm: () => handleDecision("killed"),
    });
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
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const closeConfirmDialog = () => setConfirmDialog(null);

  const handleConfirmDialogConfirm = async () => {
    const action = confirmDialog?.onConfirm;
    setConfirmDialog(null);
    if (action) await action();
  };

  const performDeleteDraft = async (id: number) => {
    try {
      setLoading(true);
      await deleteDraft(id);
      setSelectedDraft(null);
      setActiveTab("queue");
      setStatusMessage("Draft deleted.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteDraft = (id: number) => {
    setConfirmDialog({
      title: "Delete draft?",
      message:
        "This draft will be permanently deleted. This can't be undone.",
      confirmLabel: "Delete draft",
      danger: true,
      onConfirm: () => performDeleteDraft(id),
    });
  };

  const handleGenerateSocial = async () => {
    if (!selectedDraft || !selectedDraft.content) return;
    try {
      setIsGeneratingSocial(true);
      setErrorMessage("");
      setStatusMessage("Generating a social media promo pack...");
      
      const systemPrompt = "You are a social media manager for a local news organization.";
      const promptText = `Please create a social media pack for this story:\n\nTitle: ${selectedDraft.title}\n\nContent:\n${selectedDraft.content}`;
      
      const result = await llmTask(promptText, systemPrompt);
      setSocialPackResult(result);
      setStatusMessage("Social media pack generated!");
    } catch (e: any) {
      setErrorMessage(`Failed to generate social posts: ${toUserMessage(e)}`);
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
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const performBackupRestore = async () => {
    try {
      setLoading(true);
      setErrorMessage("");
      await backupRestore(backupPathInput);
      setStatusMessage("Database successfully restored from backup.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleBackupRestore = () => {
    if (!backupPathInput) return;
    setConfirmDialog({
      title: "Restore from backup?",
      message:
        "Restoring this backup will replace all current drafts, sources, and settings. This can't be undone.",
      confirmLabel: "Restore backup",
      danger: true,
      onConfirm: performBackupRestore,
    });
  };

  const handlePullModel = () => {
    if (!wizardModel) return;
    setPullingModel(true);
    setPullProgressText(["Initializing download..."]);
    pullOllamaModel(wizardModel).catch((e) => {
      setPullingModel(false);
      setErrorMessage(toUserMessage(e));
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
      setErrorMessage(toUserMessage(e));
    } finally {
      setCustomLlmRunning(false);
    }
  };

  return {
    activeTab,
    setActiveTab,
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
    confirmDialog,
    closeConfirmDialog,
    handleConfirmDialogConfirm,
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
    handleKillStory,
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
