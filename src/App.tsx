// src/App.tsx
import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { documentDir, join, resolveResource } from "@tauri-apps/api/path";
import {
  Newspaper,
  Rss,
  Cpu,
  Link as LinkIcon,
  Play,
  Trash2,
  Plus,
  AlertTriangle,
  Download,
  RefreshCw,
  FileText,
  CheckCircle,
  Info,
  ChevronRight,
  BookOpen,
  FileDown,
  Settings
} from "lucide-react";
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
  openLocalPath
} from "./ipc";

import "./App.css";

function App() {
  // Navigation
  const [activeTab, setActiveTab] = useState<string>("queue");

  // App Data
  const [sources, setSources] = useState<Source[]>([]);
  const [leads, setLeads] = useState<Lead[]>([]);
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const [pairedClients, setPairedClients] = useState<PairedClient[]>([]);
  const [communityProfile, setCommunityProfile] = useState<CommunityProfile | null>(null);

  // Queue View Toggles
  const [queueSubTab, setQueueSubTab] = useState<"leads" | "drafts">("leads");

  // Selection & Details
  const [selectedLead, setSelectedLead] = useState<Lead | null>(null);
  const [selectedDraft, setSelectedDraft] = useState<Draft | null>(null);
  const [evidenceList, setEvidenceList] = useState<EvidenceItem[]>([]);
  const [guardrailsReport, setGuardrailsReport] = useState<GuardrailsReport | null>(null);

  // Forms / Modals state
  const [newSourceName, setNewSourceName] = useState("");
  const [newSourceUrl, setNewSourceUrl] = useState("");
  const [newSourceType, setNewSourceType] = useState("primary_record");
  
  const [pairingLabel, setPairingLabel] = useState("");
  const [generatedPin, setGeneratedPin] = useState<string | null>(null);
  const [pinExpiryMsg, setPinExpiryMsg] = useState("");

  const [draftFormat, setDraftFormat] = useState("watch");
  const [customSystemPrompt, setCustomSystemPrompt] = useState("");
  const [generatingText, setGeneratingText] = useState(false);

  const [publishPath, setPublishPath] = useState("");
  const [backupPathInput, setBackupPathInput] = useState("");

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
  }, []);
  const [correctionNote, setCorrectionNote] = useState("");
  const [showCorrectionModal, setShowCorrectionModal] = useState(false);

  // Discovery / Guided Setup Wizard State
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
  const [wizardModel, setWizardModel] = useState("gemma2:9b");
  const [pullingModel, setPullingModel] = useState(false);
  const [pullProgressText, setPullProgressText] = useState<string[]>([]);
  const [manualLlmMode, setManualLlmMode] = useState(false);
  const [customLlmPrompt, setCustomLlmPrompt] = useState("");
  const [customLlmSystem, setCustomLlmSystem] = useState("You are a helpful assistant.");
  const [customLlmResult, setCustomLlmResult] = useState("");
  const [customLlmRunning, setCustomLlmRunning] = useState(false);

  // Global Status Feed
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const pullLogEndRef = useRef<HTMLDivElement>(null);

  // Initial Load
  useEffect(() => {
    loadInitialData();
    pollOllamaStatus();
    
    // Auto-detect recommended model based on RAM
    getSystemRam().then((ram) => {
      setSystemRam(ram);
      if (ram < 12) {
        setWizardModel("llama3.2:3b");
      } else {
        setWizardModel("gemma2:9b");
      }
    }).catch(console.error);

    // Setup event listeners for Ollama model pulling
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

  // Scroll to bottom of pull logs
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

  // Sources Actions
  const handleAddSource = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newSourceName || !newSourceUrl) return;
    try {
      setLoading(true);
      await addSource(newSourceName, newSourceUrl, newSourceType);
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

  // Auto-Discovery Actions
  const handleRunDiscovery = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!discoveryCity || !discoveryState) return;
    try {
      setDiscoveryLoading(true);
      setErrorMessage("");
      const results = await discoverSources(discoveryCity, discoveryState);
      setDiscoveredCats(results);
      // Pre-select all discovered candidates by default to make it easy for the user
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
          await addSource(item.name, item.url, item.type);
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
          await addSource(name, url, type);
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



  // Pairing Pin Actions
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

  // Community Profile Details
  const handleSaveProfile = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!communityProfile) return;
    try {
      setLoading(true);
      await saveCommunityProfile(communityProfile);
      setStatusMessage("Ethics standard and community profile updated.");
    } catch (e: any) {
      setErrorMessage(e.toString());
    } finally {
      setLoading(false);
    }
  };

  // Draft Creation & Decisions
  const handleOpenDraftWizard = (lead: Lead) => {
    setSelectedLead(lead);
    setSelectedDraft(null);
    setEvidenceList([]);
    setGuardrailsReport(null);
    // Fetch evidence immediately to preview it
    if (lead.id) {
      getEvidence(lead.id).then(setEvidenceList).catch(console.error);
    }
  };

  const handleGenerateText = async () => {
    if (!selectedLead || !selectedLead.id) return;
    try {
      setGeneratingText(true);
      setErrorMessage("");
      setStatusMessage("Asking local Ollama model to write a draft from evidence...");
      const text = await generateDraft(
        selectedLead.id,
        draftFormat,
        customSystemPrompt ? customSystemPrompt : undefined
      );

      // Create draft state locally
      const draftObj: Draft = {
        lead_id: selectedLead.id,
        format: draftFormat,
        title: `Draft: ${selectedLead.why.slice(0, 40)}...`,
        content: text,
        status: "draft_generated",
        verification_checklist: "[]"
      };

      // Save to SQLite right away
      const newId = await saveDraft(draftObj);
      draftObj.id = newId;

      setSelectedDraft(draftObj);
      setActiveTab("workbench");
      setStatusMessage("Draft generated successfully.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(`Ollama drafting failed: ${e.toString()}. (Do you have Ollama running, and did you pull the model?)`);
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
      // Trigger guardrails right away
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
      
      // Re-run guardrails check
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
      setStatusMessage("Static Newspaper compiled successfully! Open the folder to upload or preview index.html.");
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

  // Social Media Pack
  const handleGenerateSocial = async () => {
    if (!selectedDraft || !selectedDraft.content) return;
    try {
      setIsGeneratingSocial(true);
      setErrorMessage("");
      setStatusMessage("Asking Ollama to generate social media promo pack...");
      
      const systemPrompt = "You are a social media manager for a local news organization. Your job is to take a news story and create engaging social media posts tailored for Twitter/X (a short thread with #tags), Facebook (an engaging post with local context), and Reddit (a neutral, informative summary for a local subreddit). Separate each section clearly with headings.";
      
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

  // Backups
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
    if (!confirm("WARNING: This will replace the entire active database with the backup. Proceed?")) return;
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

  // Ollama Setup Assistant
  const handlePullModel = () => {
    if (!wizardModel) return;
    setPullingModel(true);
    setPullProgressText(["Initializing download..."]);
    pullModel(wizardModel).catch((e) => {
      setPullingModel(false);
      setErrorMessage(e.toString());
    });
  };

  // LLM Sandbox Task
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

  // Helper function to insert citation markdown
  const insertCitation = (evidenceId: number) => {
    const textarea = document.getElementById("draft-editor-textarea") as HTMLTextAreaElement;
    if (!textarea || !selectedDraft) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const text = textarea.value;

    const selectionText = text.substring(start, end);
    const citationText = selectionText 
      ? `[${selectionText}](evidence:${evidenceId})` 
      : `[Evidence #${evidenceId}](evidence:${evidenceId})`;

    const newContent = text.substring(0, start) + citationText + text.substring(end);
    
    setSelectedDraft({
      ...selectedDraft,
      content: newContent
    });

    // Reset selection range after state update
    setTimeout(() => {
      textarea.focus();
      textarea.setSelectionRange(start + citationText.length, start + citationText.length);
    }, 50);
  };

  // Get status color label
  const getStatusColor = (status: string) => {
    switch (status) {
      case "online": return "online";
      case "offline": return "offline";
      case "draft_generated": return "warning";
      case "ready_to_review": return "info";
      case "ready_to_publish": return "online";
      case "hold": return "warning";
      case "killed": return "offline";
      case "corrected": return "info";
      default: return "warning";
    }
  };

  return (
    <div className="app-container">
      {/* Sidebar Navigation */}
      <aside className="sidebar">
        <div className="brand">
          <Newspaper className="brand-icon" />
          <span className="brand-name">CivicNews</span>
        </div>

        <nav>
          <ul className="nav-links">
            <li>
              <button
                className={`nav-link ${activeTab === "queue" ? "active" : ""}`}
                onClick={() => { setActiveTab("queue"); setSelectedLead(null); }}
              >
                <BookOpen size={18} />
                Story Queue
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "sources" ? "active" : ""}`}
                onClick={() => { setActiveTab("sources"); setSelectedLead(null); }}
              >
                <Rss size={18} />
                Sources Setup
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "onboarding" ? "active" : ""}`}
                onClick={() => { setActiveTab("onboarding"); setSelectedLead(null); }}
              >
                <Cpu size={18} />
                Ollama Wizard
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "pairing" ? "active" : ""}`}
                onClick={() => { setActiveTab("pairing"); setSelectedLead(null); }}
              >
                <LinkIcon size={18} />
                Browser Pairing
              </button>
            </li>
            <li>
              <button
                className={`nav-link ${activeTab === "settings" ? "active" : ""}`}
                onClick={() => { setActiveTab("settings"); setSelectedLead(null); }}
              >
                <Settings size={18} />
                Ethics & Backups
              </button>
            </li>
            {selectedDraft && (
              <li>
                <button
                  className={`nav-link ${activeTab === "workbench" ? "active" : ""}`}
                  onClick={() => setActiveTab("workbench")}
                >
                  <FileText size={18} />
                  Story Workbench
                </button>
              </li>
            )}
          </ul>
        </nav>

        <div className="sidebar-footer">
          <div className="ollama-status-indicator">
            <span className={`status-dot ${ollamaOnline ? "online" : "offline"}`} />
            Ollama Status: {ollamaOnline ? "Online" : "Offline"}
          </div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="main-content">
        
        {/* Global Notifications */}
        {statusMessage && (
          <div className="card" style={{ borderLeft: "4px solid var(--color-success)", background: "rgba(16, 185, 129, 0.05)" }}>
            <div className="flex-between">
              <span style={{ fontSize: "0.9rem", color: "var(--text-primary)" }}>{statusMessage}</span>
              <button className="btn btn-secondary btn-sm" onClick={() => setStatusMessage("")}>Dismiss</button>
            </div>
          </div>
        )}

        {errorMessage && (
          <div className="card" style={{ borderLeft: "4px solid var(--color-error)", background: "rgba(239, 68, 68, 0.05)" }}>
            <div className="flex-between">
              <span style={{ fontSize: "0.9rem", color: "var(--color-error)" }}>{errorMessage}</span>
              <button className="btn btn-secondary btn-sm" onClick={() => setErrorMessage("")}>Dismiss</button>
            </div>
          </div>
        )}

        {/* Tab 1: Story Queue */}
        {activeTab === "queue" && !selectedLead && (
          <div>
            <div className="page-header">
              <div className="page-title">
                <h1>Daily Story Queue</h1>
                <p>Verify municipal leads, review drafted articles, and compile your local community gazette.</p>
              </div>
              <div className="btn-group">
                <button className="btn btn-secondary" onClick={loadInitialData} disabled={loading}>
                  <RefreshCw size={16} className={loading ? "animate-spin" : ""} />
                  Sync List
                </button>
                <button className="btn btn-primary" onClick={handleIngest} disabled={loading}>
                  <Play size={16} />
                  Scrape & Detect
                </button>
              </div>
            </div>

            <div className="queue-tabs">
              <button
                className={`queue-tab ${queueSubTab === "leads" ? "active" : ""}`}
                onClick={() => setQueueSubTab("leads")}
              >
                Generated Leads <span className="badge badge-neutral">{leads.length}</span>
              </button>
              <button
                className={`queue-tab ${queueSubTab === "drafts" ? "active" : ""}`}
                onClick={() => setQueueSubTab("drafts")}
              >
                Editorial Workbench <span className="badge badge-neutral">{drafts.length}</span>
              </button>
            </div>

            {queueSubTab === "leads" ? (
              <div className="lead-grid">
                {leads.length === 0 ? (
                  <div className="card text-center" style={{ gridColumn: "1 / -1", padding: "3rem" }}>
                    <Info size={36} style={{ color: "var(--text-muted)", marginBottom: "1rem" }} />
                    <h3>No unlinked leads available</h3>
                    <p className="help-text">Click "Scrape & Detect" above to scrape primary sources and trigger OSINT alerts.</p>
                  </div>
                ) : (
                  leads.map((lead) => (
                    <div key={lead.id} className="card lead-card">
                      <div>
                        <div className="lead-header">
                          <span className={`badge ${
                            lead.risk_level === "high" ? "badge-error" : 
                            lead.risk_level === "med" ? "badge-warning" : "badge-info"
                          }`}>
                            Risk: {lead.risk_level}
                          </span>
                          <span className="help-text">{lead.detector_name}</span>
                        </div>
                        <h4 className="lead-why">{lead.why}</h4>
                        <div style={{ marginTop: "1rem" }}>
                          <span style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>
                            Confidence: <strong>{lead.confidence}</strong>
                          </span>
                        </div>
                      </div>
                      <div className="mt-2 text-right">
                        <button className="btn btn-secondary btn-sm" onClick={() => handleOpenDraftWizard(lead)}>
                          Draft Article <ChevronRight size={14} />
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            ) : (
              <div className="card">
                <div className="table-container">
                  <table>
                    <thead>
                      <tr>
                        <th>Title</th>
                        <th>Format</th>
                        <th>Status</th>
                        <th>Actions</th>
                      </tr>
                    </thead>
                    <tbody>
                      {drafts.length === 0 ? (
                        <tr>
                          <td colSpan={4} className="text-center" style={{ padding: "3rem" }}>
                            No drafts generated yet. Select a Lead and click "Draft Article" to begin.
                          </td>
                        </tr>
                      ) : (
                        drafts.map((draft) => (
                          <tr key={draft.id}>
                            <td>
                              <strong>{draft.title}</strong>
                              {draft.correction_note && (
                                <div style={{ fontSize: "0.75rem", color: "var(--color-warning)", marginTop: "2px" }}>
                                  ⚠️ Correction Registered: {draft.correction_note.slice(0, 50)}...
                                </div>
                              )}
                            </td>
                            <td>
                              <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                                {draft.format}
                              </span>
                            </td>
                            <td>
                              <span className={`badge badge-${getStatusColor(draft.status)}`}>
                                {draft.status.replace(/_/g, " ")}
                              </span>
                            </td>
                            <td>
                              <div className="btn-group">
                                <button className="btn btn-secondary btn-sm" onClick={() => handleOpenDraftEditor(draft)}>
                                  Open Workbench
                                </button>
                                <button className="btn btn-secondary btn-sm" onClick={() => openCorrectionModal(draft.id!)}>
                                  Correction
                                </button>
                                <button className="btn btn-danger btn-sm" onClick={() => handleDeleteDraft(draft.id!)}>
                                  <Trash2 size={12} />
                                </button>
                              </div>
                            </td>
                          </tr>
                        ))
                      )}
                    </tbody>
                  </table>
                </div>
              </div>
            )}
          </div>
        )}

        {/* Lead Draft Wizard (Shown inside tab, when lead is selected) */}
        {activeTab === "queue" && selectedLead && (
          <div className="wizard-container card">
            <h2>Drafting Article from Evidence</h2>
            <p className="help-text" style={{ marginBottom: "1.5rem" }}>
              Lead: <strong>{selectedLead.why}</strong>
            </p>

            <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Article Format</label>
                <select value={draftFormat} onChange={(e) => setDraftFormat(e.target.value)}>
                  <option value="brief">Brief (Under 200 words summary)</option>
                  <option value="watch">Watch Alert (Highlights specific public safety or procurement issues)</option>
                  <option value="explainer">Explainer (Detailed review of a policy or decision background)</option>
                  <option value="investigation">Investigation (Highlights specific money Trails/risk linkages)</option>
                  <option value="opinion">Editorial / Opinion Piece (Presents a structured argument)</option>
                </select>
              </div>

              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Custom Guidelines / Instructions (Optional)</label>
                <textarea
                  placeholder="e.g. Focus specifically on the budget numbers, keep tone highly objective..."
                  value={customSystemPrompt}
                  onChange={(e) => setCustomSystemPrompt(e.target.value)}
                  style={{ height: "100px" }}
                />
              </div>

              <div className="card" style={{ background: "var(--accent-light)", marginTop: "1rem" }}>
                <h4>Linked Records ({evidenceList.length})</h4>
                <div style={{ maxHeight: "150px", overflowY: "auto", marginTop: "0.5rem" }}>
                  {evidenceList.map((item, idx) => (
                    <div key={idx} style={{ padding: "0.25rem 0", fontSize: "0.85rem", borderBottom: "1px solid var(--border-color)" }}>
                      📄 <em>"{item.excerpt.slice(0, 100)}..."</em>
                    </div>
                  ))}
                </div>
              </div>

              <div className="flex-between" style={{ marginTop: "1.5rem" }}>
                <button className="btn btn-secondary" onClick={() => setSelectedLead(null)} disabled={generatingText}>
                  Cancel
                </button>
                <button className="btn btn-primary" onClick={handleGenerateText} disabled={generatingText || (!ollamaOnline && !manualLlmMode)}>
                  {generatingText ? "Generating Draft..." : "Generate Draft"}
                </button>
              </div>
              
              {!ollamaOnline && !manualLlmMode && (
                <div className="error-text">
                  ⚠️ Local Ollama is offline. Open the "Ollama Wizard" tab to set up or use "Manual Mode" settings.
                </div>
              )}
            </div>
          </div>
        )}

        {/* Tab 2: Sources Setup */}
        {activeTab === "sources" && (
          <div>
            <div className="page-header">
              <div className="page-title">
                <h1>Sources Manager</h1>
                <p>Configure feeds, portals, and records systems scanned by CivicNews' OSINT detectors.</p>
              </div>
              <div className="btn-group">
                <button className="btn btn-secondary" onClick={() => setShowBulkImportModal(true)} style={{ marginRight: "0.5rem" }}>
                  <Plus size={16} />
                  Bulk Import URLs
                </button>
                <button className="btn btn-primary" onClick={() => setShowDiscoveryModal(true)}>
                  <Plus size={16} />
                  Auto-Discover Town Feeds
                </button>
              </div>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "1fr 380px", gap: "1.5rem" }}>
              {/* Left Pane: Sources List */}
              <div className="card">
                <div className="table-container">
                  <table>
                    <thead>
                      <tr>
                        <th>Source</th>
                        <th>URL</th>
                        <th>Type</th>
                        <th>Status</th>
                        <th>Scraped</th>
                        <th>Action</th>
                      </tr>
                    </thead>
                    <tbody>
                      {sources.length === 0 ? (
                        <tr>
                          <td colSpan={6} className="text-center">No feeds or portals registered yet. Add one in the right panel.</td>
                        </tr>
                      ) : (
                        sources.map((src) => (
                          <tr key={src.id}>
                            <td><strong>{src.name}</strong></td>
                            <td style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>
                              <a href={src.url} target="_blank" rel="noreferrer" style={{ wordBreak: "break-all" }}>{src.url}</a>
                            </td>
                            <td>
                              <span className="badge badge-neutral" style={{ textTransform: "capitalize" }}>
                                {src.type.replace(/_/g, " ")}
                              </span>
                            </td>
                            <td>
                              <span className={`status-dot ${getStatusColor(src.status)}`} />
                              <span style={{ marginLeft: "0.25rem", fontSize: "0.85rem" }}>{src.status}</span>
                            </td>
                            <td style={{ fontSize: "0.8rem" }}>
                              {src.last_scraped ? new Date(src.last_scraped).toLocaleDateString() : "Never"}
                            </td>
                            <td>
                              <button className="btn btn-danger btn-sm" onClick={() => handleDeleteSource(src.id!)}>
                                <Trash2 size={12} />
                              </button>
                            </td>
                          </tr>
                        ))
                      )}
                    </tbody>
                  </table>
                </div>
              </div>

              {/* Right Pane: Add Source Form */}
              <div className="card">
                <h3 className="card-title">Register Portal/Feed</h3>
                <form onSubmit={handleAddSource} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                  <div>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Source Name</label>
                    <input
                      type="text"
                      placeholder="e.g. City Council Agendas"
                      value={newSourceName}
                      onChange={(e) => setNewSourceName(e.target.value)}
                      required
                    />
                  </div>

                  <div>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Feed URL (RSS or static HTML)</label>
                    <input
                      type="url"
                      placeholder="e.g. https://city.gov/agendas/rss"
                      value={newSourceUrl}
                      onChange={(e) => setNewSourceUrl(e.target.value)}
                      required
                    />
                  </div>

                  <div>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Classification Type</label>
                    <select value={newSourceType} onChange={(e) => setNewSourceType(e.target.value)}>
                      <option value="primary_record">Primary Record (Agendas, budgets, public notices)</option>
                      <option value="official_comm">Official Communication (Press releases, announcements)</option>
                      <option value="community_signal">Community Signal (Local forums, neighborhood boards)</option>
                      <option value="media_lead">Media Lead (Newspapers, regional feeds)</option>
                    </select>
                  </div>

                  <button className="btn btn-primary" type="submit" disabled={loading}>
                    <Plus size={16} />
                    Add Source
                  </button>
                </form>
              </div>
            </div>
          </div>
        )}

        {/* Tab 3: Ollama Onboarding / Setup Assistant */}
        {activeTab === "onboarding" && (
          <div style={{ maxWidth: "800px", margin: "0 auto" }}>
            <div className="page-header">
              <div className="page-title">
                <h1>Local LLM Setup Wizard</h1>
                <p>CivicNews runs LLM inference locally via Ollama to keep your work completely private.</p>
              </div>
            </div>

            <div className="card">
              <div className="flex-between" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1.5rem" }}>
                <div>
                  <h3>Ollama Service Connection</h3>
                  <p className="help-text">Detected Local system RAM: <strong>{systemRam} GB</strong></p>
                </div>
                <div style={{ display: "flex", alignItems: "center", gap: "1rem" }}>
                  <span className={`status-dot ${ollamaOnline ? "online" : "offline"}`} />
                  <strong style={{ textTransform: "uppercase" }}>{ollamaOnline ? "Connected" : "Offline"}</strong>
                  <button className="btn btn-secondary btn-sm" onClick={pollOllamaStatus}>
                    <RefreshCw size={12} /> Check
                  </button>
                </div>
              </div>

              {!ollamaOnline ? (
                <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                  <div className="card" style={{ background: "rgba(239, 68, 68, 0.05)", borderColor: "rgba(239, 68, 68, 0.2)" }}>
                    <h4>Ollama is not running or not installed</h4>
                    <p className="help-text" style={{ marginTop: "0.5rem" }}>
                      To write draft articles automatically, CivicNews connects to Ollama on your local machine.
                    </p>
                    <ol style={{ marginLeft: "1.5rem", marginTop: "0.5rem", fontSize: "0.9rem" }}>
                      <li>Download & Install Ollama from <a href="https://ollama.com" target="_blank" rel="noreferrer">ollama.com</a>.</li>
                      <li>Launch the Ollama application.</li>
                      <li>Click "Check" above to connect CivicNews.</li>
                    </ol>
                  </div>
                  <div className="flex-between">
                    <label style={{ display: "flex", alignItems: "center", gap: "0.5rem", cursor: "pointer" }}>
                      <input type="checkbox" checked={manualLlmMode} onChange={(e) => setManualLlmMode(e.target.checked)} />
                      <span>Enable Manual Fallback Mode (No LLM drafting, direct text inputs only)</span>
                    </label>
                  </div>
                </div>
              ) : (
                <div>
                  <div className="card" style={{ background: "rgba(16, 185, 129, 0.05)", borderColor: "rgba(16, 185, 129, 0.2)" }}>
                    <h4>✓ Successfully Connected to Ollama</h4>
                    <p className="help-text" style={{ marginTop: "0.5rem" }}>
                      We recommend pulling <strong>gemma2:9b</strong> for systems with 16GB+ RAM, and <strong>llama3.2:3b</strong> for systems with 8GB RAM.
                    </p>
                  </div>

                  <div style={{ marginTop: "1.5rem" }}>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Model to Pull</label>
                    <div style={{ display: "flex", gap: "1rem" }}>
                      <select value={wizardModel} onChange={(e) => setWizardModel(e.target.value)} disabled={pullingModel}>
                        <option value="gemma2:9b">Gemma 2 (9B parameters - Recommended for 16GB+ RAM)</option>
                        <option value="llama3.2:3b">Llama 3.2 (3B parameters - Recommended for 8GB RAM)</option>
                        <option value="llama3:8b">Llama 3 (8B parameters)</option>
                        <option value="mistral:7b">Mistral (7B parameters)</option>
                      </select>
                      <button className="btn btn-primary" onClick={handlePullModel} disabled={pullingModel}>
                        <Download size={16} /> Pull Model
                      </button>
                    </div>
                  </div>

                  {pullProgressText.length > 0 && (
                    <div className="card" style={{ background: "#0b0f19", color: "#38bdf8", fontFamily: "monospace", fontSize: "0.85rem", marginTop: "1.5rem", padding: "1rem", overflow: "hidden" }}>
                      <h5 style={{ borderBottom: "1px solid rgba(255,255,255,0.1)", paddingBottom: "0.5rem", marginBottom: "0.5rem", color: "#94a3b8" }}>Ollama Pull Stream logs:</h5>
                      <div style={{ maxHeight: "200px", overflowY: "auto" }}>
                        {pullProgressText.map((line, idx) => (
                          <div key={idx}>{line}</div>
                        ))}
                        <div ref={pullLogEndRef} />
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>

            {ollamaOnline && (
              <div className="card mt-2">
                <h3 className="card-title">LLM Sandbox Test</h3>
                <form onSubmit={handleCustomLlmTask} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                  <div>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>System Instruction</label>
                    <input
                      type="text"
                      value={customLlmSystem}
                      onChange={(e) => setCustomLlmSystem(e.target.value)}
                    />
                  </div>
                  <div>
                    <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Prompt</label>
                    <textarea
                      placeholder="Write a greeting in Shakespearean style..."
                      value={customLlmPrompt}
                      onChange={(e) => setCustomLlmPrompt(e.target.value)}
                      style={{ height: "60px" }}
                      required
                    />
                  </div>
                  <button className="btn btn-secondary" type="submit" disabled={customLlmRunning}>
                    {customLlmRunning ? "Generating..." : "Run Inference Sandbox"}
                  </button>
                </form>
                {customLlmResult && (
                  <div className="card" style={{ marginTop: "1rem", background: "var(--accent-light)", whiteSpace: "pre-wrap", fontFamily: "var(--font-serif)" }}>
                    {customLlmResult}
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {/* Tab 4: Browser Pairing Panel */}
        {activeTab === "pairing" && (
          <div style={{ maxWidth: "850px", margin: "0 auto" }}>
            <div className="page-header">
              <div className="page-title">
                <h1>Browser Integration Pairing</h1>
                <p>Securely connect web browsers or external AI coding plugins (Codex, Agent Pipeline) to access story queues and evidence records.</p>
              </div>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1.5rem" }}>
              {/* Left Column: Generate Pin & Extension Setup */}
              <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
                <div className="card">
                  <h3 className="card-title">1. Browser Extension Setup</h3>
                  <p className="help-text" style={{ marginBottom: "1rem" }}>
                    Install the CivicNews browser extension to capture articles, send to queue, and highlight evidence as you browse.
                  </p>
                  <ol style={{ marginLeft: "1.5rem", marginBottom: "1rem", fontSize: "0.9rem", color: "var(--text-secondary)" }}>
                    <li>Open <strong>chrome://extensions</strong> in your browser.</li>
                    <li>Enable <strong>Developer Mode</strong> (top right).</li>
                    <li>Click the button below to open the extension folder, then <strong>drag and drop</strong> the folder into the extensions page.</li>
                  </ol>
                  <button className="btn btn-secondary" onClick={async () => {
                    try {
                      const extPath = await resolveResource("browser-extension");
                      openLocalPath(extPath);
                    } catch (e) {
                      console.error("Failed to resolve extension path", e);
                    }
                  }}>
                    <FileDown size={16} /> Open Extension Folder
                  </button>
                </div>

                <div className="card">
                  <h3 className="card-title">2. Pair Assistant</h3>
                  <p className="help-text">
                    Pairing enables extensions like Chrome/Safari to query local data via read-only APIs on port <code>12053</code>. Write operations are blocked.
                  </p>

                  <form onSubmit={handleGeneratePin} style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Client Label</label>
                      <input
                        type="text"
                        placeholder="e.g. My Chrome Extension"
                        value={pairingLabel}
                        onChange={(e) => setPairingLabel(e.target.value)}
                        required
                      />
                    </div>
                    <button className="btn btn-primary" type="submit">
                      Generate Pairing Pin
                    </button>
                  </form>

                  {generatedPin && (
                    <div>
                      <div className="pairing-pin-box">{generatedPin}</div>
                      <p className="help-text text-center" style={{ color: "var(--color-warning)" }}>{pinExpiryMsg}</p>
                    </div>
                  )}
                </div>
              </div>

              {/* Right Column: Paired Clients list */}
              <div className="card">
                <h3 className="card-title">Active Pairings</h3>
                <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                  {pairedClients.length === 0 ? (
                    <p className="help-text">No active browser or assistant pairings registered.</p>
                  ) : (
                    pairedClients.map((client) => (
                      <div key={client.id} className="card" style={{ padding: "1rem", marginBottom: "0", background: "var(--bg-app)" }}>
                        <div className="flex-between">
                          <div>
                            <strong>{client.label}</strong>
                            <div className="help-text" style={{ fontSize: "0.75rem" }}>
                              Token: <code>{client.token.slice(0, 12)}...</code>
                            </div>
                            <div className="help-text" style={{ fontSize: "0.75rem" }}>
                              Added: {client.created_at ? new Date(client.created_at).toLocaleDateString() : ""}
                            </div>
                          </div>
                          <button className="btn btn-danger btn-sm" onClick={() => handleRevokeClient(client.id!)}>
                            Revoke
                          </button>
                        </div>
                      </div>
                    ))
                  )}
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Tab 5: Ethics Profile & Backup settings */}
        {activeTab === "settings" && (
          <div style={{ maxWidth: "900px", margin: "0 auto" }}>
            <div className="page-header">
              <div className="page-title">
                <h1>Ethics Profile & Core Backups</h1>
                <p>Adjust guardrails parameters, define ethics policies, and backup the local SQLite database.</p>
              </div>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1.5rem" }}>
              {/* Profile setup */}
              <div className="card">
                <h3 className="card-title">Community Profile & Thresholds</h3>
                {communityProfile ? (
                  <form onSubmit={handleSaveProfile} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publication Name</label>
                      <input
                        type="text"
                        value={communityProfile.site_title}
                        onChange={(e) => setCommunityProfile({ ...communityProfile, site_title: e.target.value })}
                        required
                      />
                    </div>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Subtitle / Motto</label>
                      <input
                        type="text"
                        value={communityProfile.site_subtitle}
                        onChange={(e) => setCommunityProfile({ ...communityProfile, site_subtitle: e.target.value })}
                        required
                      />
                    </div>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Public Inquest Threshold ($)</label>
                      <input
                        type="number"
                        value={communityProfile.money_threshold}
                        onChange={(e) => setCommunityProfile({ ...communityProfile, money_threshold: parseFloat(e.target.value) || 0 })}
                        required
                      />
                      <p className="help-text">Contracts exceeding this triggers high-priority procurement warnings.</p>
                    </div>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Ethics Statement</label>
                      <textarea
                        value={communityProfile.ethics_text}
                        onChange={(e) => setCommunityProfile({ ...communityProfile, ethics_text: e.target.value })}
                        style={{ height: "80px" }}
                        required
                      />
                    </div>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Methodology (How We Report)</label>
                      <textarea
                        value={communityProfile.how_we_report_text}
                        onChange={(e) => setCommunityProfile({ ...communityProfile, how_we_report_text: e.target.value })}
                        style={{ height: "80px" }}
                        required
                      />
                    </div>
                    <button className="btn btn-primary" type="submit">
                      Save Profile & Policies
                    </button>
                  </form>
                ) : (
                  <p>Loading profile configurations...</p>
                )}
              </div>

              {/* Database Backups and Static Publish */}
              <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
                {/* Static Site Publishing Wizard */}
                <div className="card">
                  <h3 className="card-title">Static Compilation & Publishing Wizard</h3>
                  <p className="help-text">
                    Compile approved stories into a clean, standalone, responsive newspaper static directory.
                  </p>
                  
                  <div style={{ marginTop: "1rem" }}>
                    {publishStep === 1 && (
                      <div className="wizard-step">
                        <h4 style={{ marginBottom: "0.5rem" }}>Step 1: Choose Output Folder</h4>
                        <p className="help-text" style={{ marginBottom: "1rem" }}>
                          Select an empty directory on your computer where the generated HTML, CSS, and RSS files will be placed.
                        </p>
                        <div>
                          <input
                            type="text"
                            value={publishPath}
                            onChange={(e) => setPublishPath(e.target.value)}
                            placeholder="C:\my-local-news-site"
                            required
                          />
                        </div>
                        <div className="btn-group" style={{ marginTop: "1rem" }}>
                          <button 
                            className="btn btn-primary" 
                            onClick={() => setPublishStep(2)}
                            disabled={!publishPath}
                          >
                            Next: Compile <ChevronRight size={16} />
                          </button>
                        </div>
                      </div>
                    )}

                    {publishStep === 2 && (
                      <div className="wizard-step">
                        <h4 style={{ marginBottom: "0.5rem" }}>Step 2: Generate Files</h4>
                        <p className="help-text" style={{ marginBottom: "1rem" }}>
                          We will now compile your approved stories into static HTML files at <strong>{publishPath}</strong>.
                        </p>
                        <div className="btn-group" style={{ marginTop: "1rem" }}>
                          <button className="btn btn-secondary" onClick={() => setPublishStep(1)}>
                            Back
                          </button>
                          <button className="btn btn-primary" onClick={handlePublish} disabled={loading}>
                            <FileDown size={16} /> {loading ? "Compiling..." : "Compile Static Site"}
                          </button>
                        </div>
                      </div>
                    )}

                    {publishStep === 3 && (
                      <div className="wizard-step" style={{ background: "rgba(16, 185, 129, 0.05)", border: "1px solid rgba(16, 185, 129, 0.2)", padding: "1rem", borderRadius: "8px" }}>
                        <h4 style={{ marginBottom: "0.5rem", color: "var(--color-success)" }}>✓ Step 3: Publish to the Web</h4>
                        <p className="help-text" style={{ marginBottom: "1rem" }}>
                          Your site has been generated locally. To make it live for readers:
                        </p>
                        <ol style={{ marginLeft: "1.5rem", marginBottom: "1rem", fontSize: "0.9rem" }}>
                          <li>Open <a href="https://app.netlify.com/drop" target="_blank" rel="noreferrer">Netlify Drop</a> or <a href="https://vercel.com/new/drop" target="_blank" rel="noreferrer">Vercel Drop</a> in your browser. (These are free hosting platforms for static sites).</li>
                          <li>Click the button below to open your local output folder.</li>
                          <li>Drag and drop the entire <strong>{publishPath.split('\\').pop() || publishPath}</strong> folder directly into the browser window.</li>
                        </ol>
                        <div className="btn-group" style={{ marginTop: "1rem" }}>
                          <button className="btn btn-secondary" onClick={() => setPublishStep(1)}>
                            Start Over
                          </button>
                          <button className="btn btn-primary" onClick={() => openLocalPath(publishPath)}>
                            Open Folder in Explorer
                          </button>
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                {/* Backups Panel */}
                <div className="card">
                  <h3 className="card-title">Backup & Disaster Recovery</h3>
                  <p className="help-text">
                    Save the entire SQLite database containing paired clients, RSS sources, drafts, and evidence items to a single local file.
                  </p>
                  <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
                    <div>
                      <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Backup / Restore Path</label>
                      <input
                        type="text"
                        value={backupPathInput}
                        onChange={(e) => setBackupPathInput(e.target.value)}
                        required
                      />
                    </div>
                    <div className="btn-group">
                      <button className="btn btn-secondary" style={{ flexGrow: 1 }} onClick={handleBackupSave}>
                        Create Backup
                      </button>
                      <button className="btn btn-danger" style={{ flexGrow: 1 }} onClick={handleBackupRestore}>
                        Restore Backup
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Tab 6: Story Workbench / Review Panel */}
        {activeTab === "workbench" && selectedDraft && (
          <div>
            <div className="page-header" style={{ marginBottom: "1rem" }}>
              <div className="page-title">
                <h1>Story Editorial Workbench</h1>
                <p>Modify drafted content, review guardrails violations, and link citations to raw public evidence.</p>
              </div>
              <div className="btn-group">
                <button className="btn btn-secondary" onClick={() => setActiveTab("queue")}>
                  Back to Queue
                </button>
                <button className="btn btn-secondary" onClick={handleSaveDraftEditor} disabled={loading}>
                  Save Draft
                </button>
                <button className="btn btn-danger" onClick={() => handleDeleteDraft(selectedDraft.id!)}>
                  Delete
                </button>
              </div>
            </div>

            {/* Guardrails Check Report Alert */}
            {guardrailsReport && (
              <div className={`guardrails-panel ${guardrailsReport.is_clean ? "clean" : "issues"}`}>
                <div className="flex-between">
                  <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
                    {guardrailsReport.is_clean ? (
                      <CheckCircle size={18} />
                    ) : (
                      <AlertTriangle size={18} style={{ color: "var(--color-warning)" }} />
                    )}
                    <strong>
                      {guardrailsReport.is_clean 
                        ? "Pre-publication Guardrails Passed: No major issues detected." 
                        : "Verification Issues Detected:"}
                    </strong>
                  </div>
                  <span style={{ fontSize: "0.8rem", textTransform: "uppercase" }}>
                    {guardrailsReport.issues.length} issue(s)
                  </span>
                </div>
                {!guardrailsReport.is_clean && (
                  <div style={{ marginTop: "0.5rem" }}>
                    {guardrailsReport.issues.map((issue: any, idx: number) => (
                      <div key={idx} className={`guardrail-issue ${issue.severity}`}>
                        ⚠️ [Category: {issue.category.replace(/_/g, " ")}] {issue.message}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}

            <div className="workbench-container">
              {/* Editor Pane (Left) */}
              <div className="editor-pane">
                <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
                  <label style={{ fontWeight: 600, fontSize: "0.9rem" }}>Story Title</label>
                  <input
                    type="text"
                    value={selectedDraft.title}
                    onChange={(e) => setSelectedDraft({ ...selectedDraft, title: e.target.value })}
                    style={{ fontSize: "1.2rem", fontWeight: "600", fontFamily: "var(--font-serif)" }}
                  />
                </div>

                <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem", flexGrow: 1 }}>
                  <div className="flex-between">
                    <label style={{ fontWeight: 600, fontSize: "0.9rem" }}>Article Body (Markdown)</label>
                    <span className="help-text">Highlight text and click "Cite" in evidence pane to link.</span>
                  </div>
                  <textarea
                    id="draft-editor-textarea"
                    className="editor-textarea"
                    value={selectedDraft.content}
                    onChange={(e) => setSelectedDraft({ ...selectedDraft, content: e.target.value })}
                  />
                </div>

                <div className="card" style={{ padding: "1rem", background: "var(--bg-sidebar)" }}>
                  <div className="flex-between">
                    <div>
                      <span style={{ fontSize: "0.85rem", color: "var(--text-secondary)" }}>Current Status: </span>
                      <strong className={`badge badge-${getStatusColor(selectedDraft.status)}`} style={{ fontSize: "0.85rem" }}>
                        {selectedDraft.status.replace(/_/g, " ")}
                      </strong>
                    </div>
                    <div className="btn-group">
                      <button className="btn btn-secondary btn-sm" onClick={() => handleDecision("hold")}>
                        Hold
                      </button>
                      <button className="btn btn-danger btn-sm" onClick={() => handleDecision("killed")}>
                        Kill Story
                      </button>
                      <button className="btn btn-primary btn-sm" onClick={() => handleDecision("ready_to_publish")}>
                        Approve for Static Publish
                      </button>
                    </div>
                  </div>
                </div>

                {/* Social Media Generator */}
                <div className="card" style={{ padding: "1rem", marginTop: "1rem" }}>
                  <div className="flex-between" style={{ marginBottom: "0.5rem" }}>
                    <h4 style={{ margin: 0 }}>Social Media Promo Pack</h4>
                    <button 
                      className="btn btn-secondary btn-sm" 
                      onClick={handleGenerateSocial}
                      disabled={isGeneratingSocial || (!ollamaOnline && !manualLlmMode)}
                    >
                      {isGeneratingSocial ? "Generating..." : "Generate Posts"}
                    </button>
                  </div>
                  {socialPackResult && (
                    <textarea
                      className="editor-textarea"
                      style={{ height: "150px", marginTop: "0.5rem", fontSize: "0.85rem", fontFamily: "var(--font-serif)" }}
                      value={socialPackResult}
                      onChange={(e) => setSocialPackResult(e.target.value)}
                    />
                  )}
                  {!socialPackResult && !isGeneratingSocial && (
                    <p className="help-text" style={{ fontSize: "0.85rem", margin: 0 }}>
                      Generate optimized posts for Twitter/X, Facebook, and Reddit based on the current draft.
                    </p>
                  )}
                </div>
              </div>

              {/* Evidence Pane (Right) */}
              <div className="evidence-pane">
                <h4 style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem" }}>Linked Public Records</h4>
                {evidenceList.length === 0 ? (
                  <p className="help-text">No evidence documents are linked to this story draft.</p>
                ) : (
                  evidenceList.map((item) => (
                    <div key={item.id} className="evidence-item">
                      <div className="evidence-header">
                        <span>Citation ID: #{item.id}</span>
                        <span>Fetched: {new Date(item.fetched_at).toLocaleDateString()}</span>
                      </div>
                      <div className="evidence-excerpt">"{item.excerpt}"</div>
                      <div className="text-right">
                        <button className="btn btn-secondary btn-sm" onClick={() => insertCitation(item.id!)}>
                          Link Citation
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        )}

      </main>

      {/* Correction Modal */}
      {showCorrectionModal && (
        <div className="modal-overlay">
          <div className="modal-content">
            <h3 style={{ marginBottom: "1rem" }}>Register Story Correction</h3>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Entering a correction marks the story status as <code>corrected</code>, and appends a public retraction/correction note directly on the static site compiler output.
            </p>
            <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Correction Note</label>
                <textarea
                  placeholder="e.g. Corrected date of zoning hearing from June 5 to June 15..."
                  value={correctionNote}
                  onChange={(e) => setCorrectionNote(e.target.value)}
                  style={{ height: "120px" }}
                  required
                />
              </div>
              <div className="btn-group text-right" style={{ justifyContent: "flex-end" }}>
                <button className="btn btn-secondary" onClick={() => setShowCorrectionModal(false)}>
                  Cancel
                </button>
                <button className="btn btn-primary" onClick={handleRegisterCorrection}>
                  Register & Commit Note
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Bulk Import Modal */}
      {showBulkImportModal && (
        <div className="modal-overlay">
          <div className="modal-content" style={{ maxWidth: "600px", width: "90%", display: "flex", flexDirection: "column" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 style={{ margin: 0 }}>Bulk Import Sources</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => setShowBulkImportModal(false)}>Close</button>
            </div>
            
            <form onSubmit={handleBulkImport} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Default Classification Type</label>
                <select value={bulkImportType} onChange={(e) => setBulkImportType(e.target.value)}>
                  <option value="primary_record">Primary Record (Agendas, budgets, public notices)</option>
                  <option value="official_comm">Official Communication (Press releases, announcements)</option>
                  <option value="community_signal">Community Signal (Local forums, neighborhood boards)</option>
                  <option value="media_lead">Media Lead (Newspapers, regional feeds)</option>
                </select>
              </div>

              <div>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>
                  Source List (one per line)
                </label>
                <p className="help-text" style={{ margin: "0 0 0.5rem 0", fontSize: "0.8rem" }}>
                  Paste a list of URLs. You can optionally prefix with a name, e.g.,<br />
                  <code>Brighton Council, https://brightonco.gov/agenda</code><br />
                  If only a URL is pasted, we will automatically extract the name from its domain.
                </p>
                <textarea
                  placeholder="https://example.com/feed.xml&#10;Brighton School District, https://sd27j.org/board-agenda&#10;https://reddit.com/r/brightonco"
                  value={bulkImportText}
                  onChange={(e) => setBulkImportText(e.target.value)}
                  style={{ height: "200px", fontFamily: "monospace", fontSize: "0.85rem", background: "var(--bg-card)", border: "1px solid var(--border-color)", color: "var(--text-primary)", borderRadius: "4px", padding: "0.5rem" }}
                  required
                  disabled={bulkImportLoading}
                />
              </div>

              <div style={{ display: "flex", justifyContent: "flex-end", gap: "1rem", marginTop: "1rem" }}>
                <button className="btn btn-secondary" type="button" onClick={() => setShowBulkImportModal(false)} disabled={bulkImportLoading}>
                  Cancel
                </button>
                <button className="btn btn-primary" type="submit" disabled={bulkImportLoading}>
                  {bulkImportLoading ? "Importing..." : "Import List"}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Discovery Modal */}
      {showDiscoveryModal && (
        <div className="modal-overlay">
          <div className="modal-content" style={{ maxWidth: "800px", width: "90%", maxHeight: "85vh", display: "flex", flexDirection: "column" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", borderBottom: "1px solid var(--border-color)", paddingBottom: "1rem", marginBottom: "1rem" }}>
              <h3 style={{ margin: 0 }}>Town Setup & Source Auto-Discovery</h3>
              <button className="btn btn-secondary btn-sm" onClick={() => setShowDiscoveryModal(false)}>Close</button>
            </div>
            
            <form onSubmit={handleRunDiscovery} style={{ display: "flex", gap: "1rem", marginBottom: "1.5rem" }}>
              <div style={{ flex: 1 }}>
                <input
                  type="text"
                  placeholder="City Name (e.g. Brighton)"
                  value={discoveryCity}
                  onChange={(e) => setDiscoveryCity(e.target.value)}
                  required
                  disabled={discoveryLoading}
                />
              </div>
              <div style={{ width: "150px" }}>
                <input
                  type="text"
                  placeholder="State (e.g. CO)"
                  value={discoveryState}
                  onChange={(e) => setDiscoveryState(e.target.value)}
                  required
                  disabled={discoveryLoading}
                />
              </div>
              <button className="btn btn-primary" type="submit" disabled={discoveryLoading}>
                {discoveryLoading ? "Searching..." : "Auto-Find Feeds"}
              </button>
            </form>

            <div style={{ flex: 1, overflowY: "auto", paddingRight: "0.5rem" }}>
              {discoveryLoading && (
                <div style={{ textAlign: "center", padding: "3rem 0" }}>
                  <div className="animate-spin" style={{ display: "inline-block", border: "4px solid rgba(255,255,255,0.1)", borderTop: "4px solid var(--color-primary)", borderRadius: "50%", width: "40px", height: "40px", marginBottom: "1rem" }} />
                  <p>Searching DuckDuckGo for agendas, subreddits, library calendars, and local news...</p>
                  <p className="help-text" style={{ fontSize: "0.85rem", marginTop: "0.5rem" }}>Running priority checklist queries sequentially. This takes a few seconds.</p>
                </div>
              )}

              {!discoveryLoading && discoveredCats.length === 0 && (
                <div style={{ textAlign: "center", padding: "3rem 0", color: "var(--text-secondary)" }}>
                  <p>Enter your town's name and state above to auto-discover local civic feeds.</p>
                </div>
              )}

              {!discoveryLoading && discoveredCats.length > 0 && (
                <div style={{ display: "flex", flexDirection: "column", gap: "1.5rem" }}>
                  <p className="help-text">
                    Select the feeds you want to import. We recommend keeping the primary record portals and your town's local newspaper or subreddit checked.
                  </p>
                  {discoveredCats.map((cat, idx) => (
                    <div key={idx} className="card" style={{ padding: "1rem", background: "var(--bg-sidebar)", border: "1px solid var(--border-color)" }}>
                      <div className="flex-between" style={{ borderBottom: "1px solid var(--border-color)", paddingBottom: "0.5rem", marginBottom: "0.5rem" }}>
                        <h4 style={{ margin: 0 }}>{cat.category_name}</h4>
                        <span className="badge badge-neutral" style={{ fontSize: "0.75rem", textTransform: "capitalize" }}>
                          {cat.type.replace(/_/g, " ")}
                        </span>
                      </div>
                      
                      {cat.candidates.length === 0 ? (
                        <p className="help-text" style={{ fontStyle: "italic" }}>No candidate portals detected. You can add one manually later.</p>
                      ) : (
                        <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                          {cat.candidates.map((cand: any, cIdx: number) => {
                            const isChecked = selectedDiscovered.some(item => item.url === cand.url);
                            return (
                              <label key={cIdx} style={{ display: "flex", alignItems: "flex-start", gap: "0.75rem", cursor: "pointer", fontSize: "0.9rem" }}>
                                <input
                                  type="checkbox"
                                  checked={isChecked}
                                  onChange={() => handleToggleDiscoveredSource(cand)}
                                  style={{ marginTop: "0.25rem" }}
                                />
                                <div>
                                  <div style={{ fontWeight: 600 }}>{cand.name}</div>
                                  <a href={cand.url} target="_blank" rel="noreferrer" style={{ fontSize: "0.8rem", color: "var(--color-primary)", wordBreak: "break-all" }}>
                                    {cand.url}
                                  </a>
                                </div>
                              </label>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>

            {!discoveryLoading && discoveredCats.length > 0 && (
              <div style={{ borderTop: "1px solid var(--border-color)", paddingTop: "1rem", marginTop: "1rem", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                <span>Selected: <strong>{selectedDiscovered.length}</strong> sources</span>
                <div className="btn-group">
                  <button className="btn btn-secondary" onClick={() => {
                    setDiscoveredCats([]);
                    setSelectedDiscovered([]);
                  }}>
                    Clear
                  </button>
                  <button className="btn btn-primary" onClick={handleImportDiscoveredSources} disabled={selectedDiscovered.length === 0}>
                    Import Checked Sources
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export default App;

