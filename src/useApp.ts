// src/useApp.ts
import { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { appDataDir, join } from "@tauri-apps/api/path";
import { open, save } from "@tauri-apps/plugin-dialog";
import {
  getSources,
  addSource,
  deleteSource,
  generatePairingPin,
  listPairedClients,
  revokePairing,
  getCommunityProfile,
  saveCommunityProfile,
  importLogoAsset,
  ingest,
  getQueue,
  getEvidence,
  saveDraft,
  deleteDraft,
  storyDecision,
  attestDraft,
  generateDraft,
  llmTask,
  guardrailsCheck,
  publish,
  recordPublishDestination,
  publishWithConnector,
  savePublisherConfig,
  getPublisherConfig,
  testPublisherConnection,
  listPublishHistory,
  listSubscribers,
  addSubscriber,
  deleteSubscriber,
  importSubscribersCsv,
  exportSubscribersCsv,
  readPublishArtifact,
  exportIssueEmail,
  getCivicIntelligence,
  getVerificationQueue,
  updateVerificationTaskStatus,
  createLeadFromDarkSignal,
  registerCorrection,
  backupSave,
  backupRestore,
  installOllamaRuntime,
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
  PublishResult,
  PublishRun,
  PublisherConfig,
  PublisherConfigInput,
  PublisherTestResult,
  Subscriber,
  CivicIntelligenceSnapshot,
  VerificationQueueSnapshot,
  VerificationTask,
  DiscoveredSource,
  DiscoveredSourceCategory,
  GuardrailsReport,
  isTauri,
  runDailyScan,
  isOnboardingComplete,
  getSetting,
  setSetting,
  toUserMessage,
  extractSourceImportText
} from "./ipc";
import modelsConfig from "./models.json";
import { buildBulkImportReview, BulkImportReview, normalizeImportUrl, tierForSourceType } from "./bulkImportParser";

function normalizeGeneratedDraft(raw: string, fallbackTitle: string): { title: string; content: string } {
  const repaired = raw.replace(/\r\n/g, "\n").trim();
  const lines = repaired.split("\n");
  let title = fallbackTitle.replace(/\s+/g, " ").trim();
  let titleLineIndex = -1;

  const headlinePatterns = [
    /^\s*#{1,2}\s+(.+?)\s*$/,
    /^\s*(?:\*\*)?\s*headline\s*:\s*(.+?)(?:\*\*)?\s*$/i,
    /^\s*(?:\*\*)?\s*title\s*:\s*(.+?)(?:\*\*)?\s*$/i,
  ];

  for (let i = 0; i < Math.min(lines.length, 8); i += 1) {
    for (const pattern of headlinePatterns) {
      const match = lines[i].match(pattern);
      if (match?.[1]?.trim()) {
        title = match[1].replace(/\*\*/g, "").trim();
        titleLineIndex = i;
        break;
      }
    }
    if (titleLineIndex >= 0) break;
  }

  let skippingReportingSteps = false;
  const content = lines
    .filter((_line, idx) => idx !== titleLineIndex)
    .map((line) => {
      const plain = line.trim().replace(/^\*\*|\*\*$/g, "").trim();
      const lower = plain.toLowerCase();
      if (
        lower.startsWith("editor_note:") ||
        lower.startsWith("editor note:") ||
        lower.startsWith("[editor_note:") ||
        lower.startsWith("[editor note:") ||
        lower.startsWith("tester edit:") ||
        /^\s*\[?\s*(source needed|verification needed|end of report)\s*\]?\s*$/i.test(plain)
      ) {
        skippingReportingSteps = false;
        return "";
      }
      if (/^(reporting steps|next reporting steps)\s*:/i.test(plain)) {
        skippingReportingSteps = true;
        return "";
      }
      if (skippingReportingSteps) {
        if (!plain || /^[-*]\s+/.test(plain) || /^\d+[.)]\s+/.test(plain) || plain.endsWith("?")) {
          return "";
        }
        skippingReportingSteps = false;
      }
      const labelMatch = line.match(/^\s*(?:\*\*)?\s*(nut graf|lede)\s*:\s*(?:\*\*)?\s*(.*)$/i);
      const normalizedLine = labelMatch ? labelMatch[2].trim() : line;
      return normalizedLine
        .replace(/\[(?:insert [^\]]+|[^\]]+ if available|source needed|verification needed)\]/gi, "")
        .replace(/\s+([.,;:!?])/g, "$1")
        .replace(/\s{2,}/g, " ")
        .trimEnd();
    })
    .filter((line) => !/^\s*\[?\s*end of report\s*\]?\s*$/i.test(line))
    .filter((line, idx, arr) => line.trim() || (idx > 0 && idx < arr.length - 1 && arr[idx - 1].trim() && arr[idx + 1]?.trim()))
    .join("\n")
    .trim();

  return {
    title: title || "Untitled draft",
    content: content || repaired,
  };
}

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

export interface DailyScanProgress {
  stage: string;
  message: string;
  run_id?: number | null;
  model?: string | null;
  evidence_count: number;
  eligible_evidence_count?: number;
  truncated_evidence_count?: number;
  batch_index?: number | null;
  batch_count?: number | null;
  saved_leads: number;
  receivedAt?: number;
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

export function sanitizeEvidenceCitations(text: string, allowedEvidenceIds: number[]): string {
  if (!/evidence:/i.test(text)) return text;
  const allowed = new Set(allowedEvidenceIds.map((id) => String(id)));
  return text.replace(/evidence:\s*(?:\/\/)?\s*(\d+)/gi, (_match, id) => {
    if (allowed.has(String(id))) return `evidence:${id}`;
    return `unlinked-evidence-${id}`;
  });
}

// Formats a structured `ollama-pull-progress` event into a single log line.
// The pull command (`pull_ollama_model`) emits a structured object payload, not
// a JSON string -- pinning the shape here keeps the listener from regressing to
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
  const [civicIntelligence, setCivicIntelligence] = useState<CivicIntelligenceSnapshot | null>(null);
  const [verificationQueue, setVerificationQueue] = useState<VerificationQueueSnapshot | null>(null);

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
  const [publishResult, setPublishResult] = useState<PublishResult | null>(null);
  const [publishHistory, setPublishHistory] = useState<PublishRun[]>([]);
  const [publisherConfig, setPublisherConfig] = useState<PublisherConfig | null>(null);
  const [publisherProvider, setPublisherProvider] = useState("here_now");
  const [publisherTestResult, setPublisherTestResult] = useState<PublisherTestResult | null>(null);
  const [subscribers, setSubscribers] = useState<Subscriber[]>([]);
  const [subscriberEmail, setSubscriberEmail] = useState("");
  const [subscriberName, setSubscriberName] = useState("");
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
  const [bulkImportReview, setBulkImportReview] = useState<BulkImportReview>({ accepted: [], rejected: [], duplicates: [] });

  const [correctionDraftId, setCorrectionDraftId] = useState<number | null>(null);

  // Social Media Pack State
  const [socialPackResult, setSocialPackResult] = useState("");
  const [isGeneratingSocial, setIsGeneratingSocial] = useState(false);

  // Publishing Wizard
  const [publishStep, setPublishStep] = useState(1);

  // Ollama & Wizard
  const [ollamaOnline, setOllamaOnline] = useState(false);
  const [systemRam, setSystemRam] = useState<number>(8);
  // GG-C4: gate the first-run guided OnboardingWizard. null = still loading.
  const [onboardingDone, setOnboardingDone] = useState<boolean | null>(null);
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [aiSetupSkipped, setAiSetupSkipped] = useState(false);
  const [installedModels, setInstalledModels] = useState<string[]>([]);
  const [wizardModel, setWizardModel] = useState("");
  const [pullingModel, setPullingModel] = useState(false);
  const [pullProgressText, setPullProgressText] = useState<string[]>([]);
  const [manualLlmMode, setManualLlmMode] = useState(false);
  const [customLlmPrompt, setCustomLlmPrompt] = useState("");
  const [customLlmSystem, setCustomLlmSystem] = useState("You are a helpful assistant.");
  const [customLlmResult, setCustomLlmResult] = useState("");
  const [customLlmRunning, setCustomLlmRunning] = useState(false);

  const [latestScanId, setLatestScanId] = useState<number | null>(null);
  const [dailyScanProgress, setDailyScanProgress] = useState<DailyScanProgress | null>(null);

  // Real application version, read from the Tauri bundle at runtime.
  const [appVersion, setAppVersion] = useState("");

  // Global Status Feed
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const pullLogEndRef = useRef<HTMLDivElement>(null);

  const leadNeedsDraftCaution = (lead: Lead) => {
    const disposition = (lead.disposition ?? "review").toLowerCase();
    const storyType = (lead.story_type ?? "").toLowerCase();
    const novelty = lead.novelty_score ?? 0;
    return Boolean(
      (lead.recurrence_count ?? 0) > 0 ||
      storyType === "background" ||
      storyType === "watch" ||
      storyType === "verification" ||
      disposition === "background" ||
      disposition === "watch" ||
      disposition === "needs_verification" ||
      (novelty > 0 && novelty <= 2)
    );
  };

  const pickStarterDiscoveryCandidates = (categories: DiscoveredSourceCategory[]) => {
    const picked: DiscoveredSource[] = [];
    const seen = new Set<string>();
    const trustedTypes = new Set(["primary_record", "official_comm", "official_calendar", "media_lead", "community_signal", "community_calendar"]);

    for (const category of categories) {
      const officialFirst = [...category.candidates].sort((a, b) => {
        const aOfficial = a.type === "primary_record" || a.type === "official_comm" || a.type === "official_calendar";
        const bOfficial = b.type === "primary_record" || b.type === "official_comm" || b.type === "official_calendar";
        return Number(bOfficial) - Number(aOfficial);
      });
      for (const candidate of officialFirst) {
        const urlKey = normalizeImportUrl(candidate.url);
        if (!urlKey || seen.has(urlKey) || !trustedTypes.has(candidate.type)) continue;
        picked.push(candidate);
        seen.add(urlKey);
        break;
      }
    }

    return picked;
  };

  const addStarterSourcesForCommunity = async (city: string, state: string) => {
    let imported = 0;
    const categories = await discoverSources(city, state);
    const candidates = pickStarterDiscoveryCandidates(categories);
    setDiscoveredCats(categories);
    setSelectedDiscovered(candidates);

    for (const source of candidates) {
      try {
        await addSource(source.name, source.url, source.type, tierForSourceType(source.type));
        imported++;
      } catch (err) {
        console.warn("Starter source import skipped:", source.url, err);
      }
    }
    return imported;
  };

  const routeAfterRecoveredSetup = async () => {
    try {
      const recovered = await getSetting("setup.recovered_input");
      const firstRunIntake = await getSetting("setup.first_run_intake");
      if (recovered !== "true" && firstRunIntake !== "true") return;

      const profile = await getCommunityProfile();
      const city = (profile.city || "").trim();
      const state = (profile.state || "").trim().toUpperCase();
      const isRecovered = recovered === "true";

      if (!city || !state) {
        if (isRecovered) {
          await setSetting("setup.recovered_input", "consumed");
        } else {
          await setSetting("setup.first_run_intake", "consumed");
        }
        setStatusMessage("Setup is complete. Add your city and state in Settings, then use Discover for my city on Sources.");
        setActiveTab("sources");
        return;
      }

      if (isRecovered) {
        await setSetting("setup.recovered_input", "running_source_intake");
      } else {
        await setSetting("setup.first_run_intake", "running_source_intake");
      }
      setActiveTab("sources");
      setStatusMessage(
        isRecovered
          ? `Adding starter sources for ${city}, ${state}. When this finishes, The Civic Desk will run the first Daily Scan automatically.`
          : `Adding starter sources for ${city}, ${state}. When this finishes, you will move to Daily Scan.`
      );
      setLoading(true);

      const imported = await addStarterSourcesForCommunity(city, state);
      await loadInitialData();
      if (imported === 0) {
        if (isRecovered) {
          await setSetting("setup.recovered_input", "consumed");
        } else {
          await setSetting("setup.first_run_intake", "consumed");
        }
        setStatusMessage(`No starter sources could be imported automatically for ${city}, ${state}. Use Discover for my city on Sources or import a source list.`);
        setActiveTab("sources");
        return;
      }

      if (isRecovered) {
        setStatusMessage(`Added ${imported} starter source(s) for ${city}, ${state}. Fetching records and community signals...`);
        await ingest();
        await loadInitialData();
        setActiveTab("dailyScan");
        setStatusMessage(`Running the first ${city} Daily Scan automatically...`);
        const scanId = await runDailyScan(city, state, 24);
        setLatestScanId(scanId);
        await setSetting("scan.latest_id", String(scanId));
        await setSetting("setup.recovered_input", "consumed");
        await loadInitialData();
        setActiveTab("queue");
        setStatusMessage(`Recovered setup completed source intake and Daily Scan ${scanId}. Open a Story Queue lead to draft the first issue.`);
      } else {
        await setSetting("setup.first_run_intake", "consumed");
        setActiveTab("dailyScan");
        setStatusMessage(`Added ${imported} starter source(s) for ${city}, ${state}. Use Run Daily Scan to fetch records and build the first editor packet.`);
      }
    } catch (err) {
      console.error("Failed to consume first-run source intake flag", err);
      setErrorMessage(toUserMessage(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (!isTauri()) {
      setPublishPath("C:\\CivicDesk\\site");
      setBackupPathInput("C:\\CivicDesk\\backups\\civic-desk.db");
      setAppVersion("browser preview");
      return;
    }

    async function loadDefaultPaths() {
      try {
        const appData = await appDataDir();
        setPublishPath(await join(appData, "sites", "default"));
        setBackupPathInput(await join(appData, "backups", "civic-desk.db"));
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

    if (!isTauri()) {
      setSystemRam(16);
      setWizardModel(modelsConfig.high);
      setOllamaOnline(true);
    } else {
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

      refreshPublishHistory();
      refreshSubscribers();
      handleLoadPublisherConfig(publisherProvider);
    }

    const setupListeners = async () => {
      if (!isTauri()) return () => {};

      const progressUnlisten = await listen<OllamaPullProgress>("ollama-pull-progress", (event) => {
        const progressLine = formatPullProgressLine(event.payload);
        setPullProgressText(prev => [...prev.slice(-30), progressLine]);
      });

      const dailyScanProgressUnlisten = await listen<DailyScanProgress>("daily-scan-progress", (event) => {
        setDailyScanProgress({ ...event.payload, receivedAt: Date.now() });
      });

      const completeUnlisten = await listen<void>("ollama-pull-complete", () => {
        setPullingModel(false);
        setPullProgressText(prev => [...prev, "Model pulled successfully."]);
        pollOllamaStatus();
      });

      const errorUnlisten = await listen<string>("ollama-pull-error", (event) => {
        setPullingModel(false);
        setPullProgressText(prev => [...prev, `Error: ${event.payload}`]);
      });

      const runtimeInstallUnlisten = await listen<{ stage: string; message: string; completed?: number | null; total?: number | null }>(
        "ollama-runtime-install-progress",
        (event) => {
          const { message, completed, total } = event.payload;
          const pct = completed !== undefined && completed !== null && total
            ? ` (${Math.round((completed / total) * 100)}%)`
            : "";
          setPullProgressText(prev => [...prev.slice(-30), `${message}${pct}`]);
        }
      );

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
        dailyScanProgressUnlisten();
        completeUnlisten();
        errorUnlisten();
        runtimeInstallUnlisten();
        serverErrorUnlisten();
      };
    };

    const cleanupListeners = setupListeners();

    // QA-R2-M1: the app-managed Ollama runtime can take a moment to bind 127.0.0.1:11434
    // after launch, so the single mount poll can lose the cold-start race and
    // leave `ollamaOnline=false` stuck -- disabling Generate Draft and showing
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

  useEffect(() => {
    if (!isTauri() || activeTab !== "pairing") return;
    let stopped = false;
    const refreshPairings = async () => {
      try {
        const clients = await listPairedClients();
        if (!stopped) setPairedClients(clients);
      } catch (err) {
        console.error("Failed to refresh paired clients", err);
      }
    };
    refreshPairings();
    const interval = window.setInterval(refreshPairings, 2000);
    return () => {
      stopped = true;
      window.clearInterval(interval);
    };
  }, [activeTab]);

  const loadInitialData = async () => {
    if (!isTauri()) {
      setSources(prev => prev.length ? prev : [
        {
          id: 1,
          name: "Riverton Council Agendas",
          url: "https://riverton-oh.gov/council/agendas",
          type: "primary_record",
          tier: "official_record",
          status: "online",
          last_scraped: new Date().toISOString(),
        },
      ]);
      setLeads([]);
      setDrafts([]);
      setPairedClients([]);
      setCivicIntelligence({
        observations: [],
        entities: [],
        source_scores: [],
        dark_signals: [],
      });
      setVerificationQueue({ tasks: [], generated_count: 0 });
      setCommunityProfile({
        site_title: "My Local Publication",
        site_subtitle: "Local news and community information.",
        about_text: "Browser preview profile.",
        ethics_text: "Editorial standards are set by the publisher. Corrections are published when needed.",
        how_we_report_text: "Stories, sources, and publication decisions are reviewed by the editor before publication.",
        organization_type: "single_person",
        footer_text: "",
        logo_url: "",
        accent_color: "#5a1818",
        layout_style: "classic",
        first_amendment_advisor_enabled: true,
        money_threshold: 50000,
        watchlist: [],
        city: "Riverton",
        state: "OH",
      });
      setErrorMessage("");
      return;
    }

    try {
      setLoading(true);
      const s = await getSources();
      setSources(s);

      const q = await getQueue();
      setLeads(q.leads || []);
      setDrafts(q.drafts || []);

      const p = await getCommunityProfile();
      setCommunityProfile(p);

      // RE-AUDIT (model-badge): keep the selected-model label fresh after a pull
      // or settings change, instead of only at app start.
      const selModel = await getSetting("model.selected");
      if (selModel) setSelectedModel(selModel);
      const skippedAi = await getSetting("ai.setup_skipped");
      setAiSetupSkipped(skippedAi === "true");

      const clients = await listPairedClients();
      setPairedClients(clients);

      try {
        setCivicIntelligence(await getCivicIntelligence());
      } catch (err) {
        console.error("Failed to load civic intelligence snapshot", err);
      }
      try {
        setVerificationQueue(await getVerificationQueue());
      } catch (err) {
        console.error("Failed to load verification queue", err);
      }

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
    if (!isTauri()) {
      setOllamaOnline(true);
      setInstalledModels([wizardModel || modelsConfig.high].filter(Boolean));
      return;
    }

    try {
      const health = await ollamaHealth();
      setOllamaOnline(health.reachable);
      setInstalledModels(health.models);
    } catch {
      setOllamaOnline(false);
      setInstalledModels([]);
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
    if (!isTauri()) {
      const previewScanId = Date.now();
      setDailyScanProgress({
        stage: "completed",
        message: "Browser preview generated sample Daily Scan results.",
        run_id: previewScanId,
        model: "browser-preview",
        evidence_count: sources.length,
        saved_leads: 1,
        receivedAt: Date.now(),
      });
      setLatestScanId(previewScanId);
      setStatusMessage("Daily Scan preview complete. Sample scan results are visible below.");
      return;
    }

    try {
      setLoading(true);
      setStatusMessage("Checking AI model presence...");
      setErrorMessage("");
      setDailyScanProgress({
        stage: "preflight",
        message: "Checking selected model and local AI service...",
        evidence_count: 0,
        saved_leads: 0,
        receivedAt: Date.now(),
      });

      const model = await getSetting("model.selected");
      if (!model) {
        setStatusMessage("No AI model is selected. Running deterministic evidence checks only.");
      }
      const health = await ollamaHealth();
      if (!health.reachable) {
        setStatusMessage("Local AI is offline. Running deterministic evidence checks and fallback review packet.");
      } else if (model && !modelInstalled(model, health.models)) {
        setStatusMessage(`The selected model ${model} is not installed. Running deterministic evidence checks and fallback review packet.`);
      }

      const city = (communityProfile?.city || "").trim();
      const state = (communityProfile?.state || "").trim().toUpperCase();
      if (!city || !state) {
        setActiveTab("settings");
        setErrorMessage("Choose your publication city and state in Settings before running Daily Scan.");
        return;
      }

      setStatusMessage("Running the daily scan across your collected evidence...");
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
      setDailyScanProgress(prev => prev ? {
        ...prev,
        stage: "failed",
        message: toUserMessage(e),
        receivedAt: Date.now(),
      } : null);
    } finally {
      setLoading(false);
    }
  };

  const handleAddSource = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newSourceName || !newSourceUrl) return;
    if (!isTauri()) {
      const next: Source = {
        id: Date.now(),
        name: newSourceName,
        url: newSourceUrl,
        type: newSourceType,
        tier: newSourceTier,
        status: "online",
        last_scraped: new Date().toISOString(),
      };
      setSources(prev => [...prev, next]);
      setNewSourceName("");
      setNewSourceUrl("");
      setStatusMessage("Source added in browser preview.");
      return;
    }
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
    if (!isTauri()) {
      setSources(prev => prev.filter(source => source.id !== id));
      setStatusMessage("Source removed in browser preview.");
      return;
    }
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
    if (!isTauri()) {
      setDiscoveryLoading(true);
      setErrorMessage("");
      window.setTimeout(() => {
        const citySlug = discoveryCity.trim().toLowerCase().replace(/\s+/g, "-");
        const stateSlug = discoveryState.trim().toLowerCase();
        setDiscoveredCats([
          {
            category_name: "Official records",
            type: "primary_record",
            candidates: [
              {
                name: `${discoveryCity} Council Agendas`,
                url: `https://${citySlug}-${stateSlug}.gov/council/agendas`,
                type: "primary_record",
              },
              {
                name: `${discoveryCity} Public Notices`,
                url: `https://${citySlug}-${stateSlug}.gov/public-notices`,
                type: "official_comm",
              },
            ],
          },
          {
            category_name: "Community signals",
            type: "community_signal",
            candidates: [
              {
                name: `${discoveryCity} Library Events`,
                url: `https://library.${citySlug}-${stateSlug}.org/events`,
                type: "community_signal",
              },
            ],
          },
        ]);
        setSelectedDiscovered([]);
        setDiscoveryLoading(false);
      }, 250);
      return;
    }
    try {
      setDiscoveryLoading(true);
      setErrorMessage("");
      const results = await discoverSources(discoveryCity, discoveryState);
      setDiscoveredCats(results);
      const candidateCount = results.reduce((sum, cat) => sum + cat.candidates.length, 0);
      setSelectedDiscovered([]);
      setStatusMessage(`Found ${candidateCount} candidate source(s). Select only the sources you trust, then import them.`);
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
    if (!isTauri()) {
      const imported = selectedDiscovered.map((item, index): Source => ({
        id: Date.now() + index,
        name: item.name,
        url: item.url,
        type: item.type,
        tier: tierForSourceType(item.type),
        status: "online",
        last_scraped: new Date().toISOString(),
      }));
      setSources(prev => [...prev, ...imported]);
      setShowDiscoveryModal(false);
      setDiscoveryCity("");
      setDiscoveryState("");
      setDiscoveredCats([]);
      setSelectedDiscovered([]);
      setStatusMessage(`Imported ${imported.length} source(s) in browser preview.`);
      return;
    }
    try {
      setLoading(true);
      setStatusMessage("Importing selected sources...");
      let importedCount = 0;
      const failures: string[] = [];
      const failedUrls = new Set<string>();
      for (const item of selectedDiscovered) {
        try {
          await addSource(item.name, item.url, item.type, tierForSourceType(item.type));
          importedCount++;
        } catch (err) {
          console.error("Failed to add discovered source:", item.name, err);
          failures.push(`${item.name}: ${toUserMessage(err)}`);
          failedUrls.add(item.url);
        }
      }
      const s = await getSources();
      setSources(s);
      if (failures.length > 0) {
        setErrorMessage(`Imported ${importedCount} source(s), but ${failures.length} could not be added. ${failures.slice(0, 3).join(" ")}`);
        setSelectedDiscovered(prev => prev.filter(item => failedUrls.has(item.url)));
      } else {
        setStatusMessage(`Successfully imported ${importedCount} source(s).`);
        setShowDiscoveryModal(false);
        setDiscoveryCity("");
        setDiscoveryState("");
        setDiscoveredCats([]);
        setSelectedDiscovered([]);
      }
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleBulkImport = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!bulkImportText.trim()) return;
    const review = buildBulkImportReview(bulkImportText, bulkImportType, sources.map(source => source.url));
    if (bulkImportReview.accepted.length === 0 || bulkImportReview.accepted.some(item => !review.accepted.some(next => next.id === item.id))) {
      setBulkImportReview(review);
      setStatusMessage(`Reviewed ${review.accepted.length} importable source(s). Select the ones you trust, then import.`);
      return;
    }
    const selected = bulkImportReview.accepted.filter(item => item.selected);
    if (selected.length === 0) {
      setStatusMessage("No sources selected. Select at least one reviewed source to import.");
      return;
    }
    if (!isTauri()) {
      const imported: Source[] = selected
        .map((parsed, index) => ({
          id: Date.now() + index,
          name: parsed.name,
          url: normalizeImportUrl(parsed.url),
          type: parsed.type,
          tier: parsed.tier,
          status: "online",
          last_scraped: new Date().toISOString(),
        }));
      setSources(prev => [...prev, ...imported]);
      setShowBulkImportModal(false);
      setBulkImportText("");
      setBulkImportReview({ accepted: [], rejected: [], duplicates: [] });
      setStatusMessage(`Imported ${imported.length} source(s) in browser preview.`);
      return;
    }
    try {
      setBulkImportLoading(true);
      setErrorMessage("");
      setStatusMessage("Importing selected reviewed sources...");
      let importedCount = 0;
      for (const parsed of selected) {
        await addSource(parsed.name, normalizeImportUrl(parsed.url), parsed.type, parsed.tier);
        importedCount++;
      }
      setStatusMessage(`Bulk imported ${importedCount} source(s) successfully.`);
      setShowBulkImportModal(false);
      setBulkImportText("");
      setBulkImportReview({ accepted: [], rejected: [], duplicates: [] });
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
    if (!isTauri()) {
      const seg = () => Array.from({ length: 4 }, () => "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"[Math.floor(Math.random() * 32)]).join("");
      setGeneratedPin(`${seg()}-${seg()}-${seg()}`);
      setPinExpiryMsg("Code expires in 5 minutes. Paste it into the browser extension popup.");
      setPairedClients(prev => [
        ...prev,
        {
          id: Date.now(),
          token: "browser-preview",
          label: pairingLabel,
          created_at: new Date().toISOString(),
          revoked: false,
        },
      ]);
      setPairingLabel("");
      return;
    }
    try {
      setLoading(true);
      const pin = await generatePairingPin(pairingLabel);
      setGeneratedPin(pin);
      setPinExpiryMsg("Code expires in 5 minutes. Paste it into the browser extension popup.");
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
    if (!isTauri()) {
      setPairedClients(prev => prev.filter(client => client.id !== id));
      setStatusMessage("Paired client access revoked in browser preview.");
      return;
    }

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
    if (!isTauri()) {
      setCommunityProfile(profile);
      setStatusMessage("Publication identity updated in browser preview.");
      return;
    }

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
    setActiveTab("workbench");
    if (lead.id) {
      if (isTauri()) {
        getEvidence(lead.id).then(setEvidenceList).catch(console.error);
      }
    }
  };

  const handleGenerateText = async () => {
    if (!selectedLead || !selectedLead.id) return;
    try {
      setGeneratingText(true);
      setErrorMessage("");

      // QA-C1: mirror handleDailyScan's pre-flight. The Generate Draft button is
      // gated only on the sidecar being reachable, not on the selected model
      // actually being installed -- so a user who skipped the model download could
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
        if (!health.reachable) {
          setErrorMessage("Generating a draft couldn't reach the local AI service. Start Ollama or open AI Model to check setup, then try again.");
          setOnboardingStep(2);
          setActiveTab("onboarding");
          return;
        }
        if (!modelInstalled(model, health.models)) {
          setErrorMessage(`Generating a draft requires the ${model} model, which isn't downloaded yet. Redirecting to model download setup...`);
          setOnboardingStep(3);
          setActiveTab("onboarding");
          return;
        }
      }

      setStatusMessage("Asking the local AI model to write a working draft... (this may take a moment)");
      const text = await generateDraft(
        selectedLead.id,
        draftFormat,
        customSystemPrompt ? customSystemPrompt : undefined
      );

      const leadTitle = selectedLead.why.replace(/\s+/g, " ").trim();
      const normalizedDraft = normalizeGeneratedDraft(text, leadTitle);
      const now = new Date().toISOString();
      const cautiousLead = leadNeedsDraftCaution(selectedLead);
      const draftObj: Draft = {
        lead_id: selectedLead.id,
        format: draftFormat,
        title: normalizedDraft.title,
        content: normalizedDraft.content,
        status: cautiousLead ? "needs_verification" : "draft_generated",
        verification_checklist: "[]",
        created_at: now,
        updated_at: now,
      };

      const newId = await saveDraft(draftObj);
      draftObj.id = newId;

      setSelectedLead(null);
      setSelectedDraft(draftObj);
      setActiveTab("workbench");
      try {
        const report = await guardrailsCheck(newId);
        setGuardrailsReport(report);
      } catch (err) {
        console.warn("Could not run guardrails on generated draft:", err);
        setGuardrailsReport(null);
      }
      setStatusMessage(
        cautiousLead
          ? "Draft generated and marked as needing more work because the lead has watch, background, verification, recurrence, or low-novelty signals."
          : "Draft generated successfully."
      );
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(`Draft generation failed: ${toUserMessage(e)}`);
    } finally {
      setGeneratingText(false);
    }
  };

  const handleOpenDraftEditor = async (draftOrId: Draft | number) => {
    const draft =
      typeof draftOrId === "number"
        ? drafts.find((item) => item.id === draftOrId)
        : draftOrId;
    if (!draft) {
      setErrorMessage("That draft could not be found. Refresh the queue and try again.");
      return;
    }
    setSelectedLead(null);
    setSelectedDraft({ ...draft });
    setGuardrailsReport(null);
    setSocialPackResult("");
    setActiveTab("workbench");
    try {
      if (draft.lead_id) {
        const ev = await getEvidence(draft.lead_id);
        setEvidenceList(ev);
      } else {
        setEvidenceList([]);
      }
      if (draft.id) {
        const report = await guardrailsCheck(draft.id);
        setGuardrailsReport(report);
      }
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    }
  };

  const ensureSelectedModelReady = async (actionName: string): Promise<boolean> => {
    if (manualLlmMode) return true;
    setStatusMessage(`Checking AI model presence before ${actionName}...`);
    const model = await getSetting("model.selected");
    if (!model) {
      setErrorMessage(`${actionName} requires a selected AI model, but none was configured. Redirecting to model download setup...`);
      setOnboardingStep(3);
      setActiveTab("onboarding");
      return false;
    }
    const health = await ollamaHealth();
    if (!health.reachable) {
      setErrorMessage(`${actionName} couldn't reach the local AI service. Start Ollama or open AI Model to check setup, then try again.`);
      setOnboardingStep(2);
      setActiveTab("onboarding");
      return false;
    }
    if (!modelInstalled(model, health.models)) {
      setErrorMessage(`${actionName} requires the ${model} model, which isn't downloaded yet. Redirecting to model download setup...`);
      setOnboardingStep(3);
      setActiveTab("onboarding");
      return false;
    }
    return true;
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

  const handleDecision = async (status: string, reason?: string, draftId = selectedDraft?.id) => {
    if (!draftId) return;
    try {
      setLoading(true);
      await storyDecision(draftId, status, reason);
      setSelectedDraft(current =>
        current?.id === draftId
          ? {
              ...current,
              status,
              missing_evidence_notes:
                status === "needs_verification" || status === "hold"
                  ? reason || current.missing_evidence_notes
                  : current.missing_evidence_notes,
            }
          : current
      );
      setDrafts(current =>
        current.map(draft =>
          draft.id === draftId
            ? {
                ...draft,
                status,
                missing_evidence_notes:
                  status === "needs_verification" || status === "hold"
                    ? reason || draft.missing_evidence_notes
                    : draft.missing_evidence_notes,
              }
            : draft
        )
      );
      setStatusMessage(
        reason?.trim()
          ? `Story status updated to '${status}' with an editor note.`
          : `Story status updated to '${status}'.`
      );
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  // GG-B2 / GG-C1: approving for publish records a human attestation, then asks
  // the backend to advance the status. Guardrails are advisory and logged; the
  // Advisory warnings do not decide publish/hold/cut for the editor; static package-integrity blockers are handled in Workbench.
  const handleApprovePublish = async (overrideReason?: string) => {
    if (!selectedDraft || !selectedDraft.id) return;
    try {
      setLoading(true);
      setErrorMessage("");
      const savedDraftId = await saveDraft(selectedDraft);
      const savedVisibleDraft = { ...selectedDraft, id: savedDraftId };
      setSelectedDraft(savedVisibleDraft);
      const latestGuardrails = await guardrailsCheck(savedDraftId);
      setGuardrailsReport(latestGuardrails);
      const editorName = (await getSetting("identity.editor_name"))?.trim() || "Editor";
      try {
        await attestDraft(savedDraftId, editorName);
      } catch (err) {
        console.warn("Could not record review attestation; proceeding with editor decision.", err);
      }
      const auditReason =
        overrideReason ??
        (latestGuardrails.issues?.length
          ? `Editor reviewed ${latestGuardrails.issues.length} saved-draft guardrail warning(s) before approval.`
          : undefined);
      await storyDecision(savedDraftId, "ready_to_publish", auditReason);
      setSelectedDraft({ ...savedVisibleDraft, status: "ready_to_publish" });
      setDrafts(current =>
        current.map(draft => (draft.id === savedDraftId ? { ...savedVisibleDraft, status: "ready_to_publish" } : draft))
      );
      setStatusMessage("Current draft saved, checked, and approved for publishing; a verification record was saved.");
      await loadInitialData();
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  // GG-C4: load first-run completion + selected-model state once on mount.
  useEffect(() => {
    isOnboardingComplete()
      .then((done) => {
        setOnboardingDone(done);
        if (done) void routeAfterRecoveredSetup();
      })
      // In a browser preview / no-IPC context, don't trap the user on a blank
      // wizard -- fall through to the app.
      .catch(() => {
        const forceFirstRun =
          typeof window !== "undefined" &&
          new URLSearchParams(window.location.search).get("firstRun") === "1";
        setOnboardingDone(forceFirstRun ? false : true);
      });
    getSetting("model.selected")
      .then((m) => {
        if (m) setSelectedModel(m);
      })
      .catch(() => {});
    getSetting("ai.setup_skipped")
      .then((value) => {
        setAiSetupSkipped(value === "true");
      })
      .catch(() => {});
  }, []);

  const completeOnboarding = async () => {
    // RE-AUDIT M2: onboarding writes the community profile (city/state/title) and
    // a selected model; reload both so the masthead reflects the user's entries
    // immediately rather than the defaults loaded at mount.
    try {
      const profile = await getCommunityProfile();
      setCommunityProfile(profile);
      if (profile.city?.trim() && profile.state?.trim()) {
        await setSetting("setup.first_run_intake", "true");
      }
    } catch {
      /* non-fatal */
    }
    try {
      const m = await getSetting("model.selected");
      if (m) setSelectedModel(m);
    } catch {
      /* non-fatal */
    }
    try {
      const skippedAi = await getSetting("ai.setup_skipped");
      setAiSetupSkipped(skippedAi === "true");
    } catch {
      /* non-fatal */
    }
    setOnboardingDone(true);
    await routeAfterRecoveredSetup();
  };

  // UX-m5: cutting a story is destructive and was a single unguarded click, unlike
  // draft delete which is confirmed. Route it through the same confirm dialog.
  const handleKillStory = () => {
    if (!selectedDraft || !selectedDraft.id) return;
    const draftId = selectedDraft.id;
    setConfirmDialog({
      title: "Cut this story?",
      message:
        "Cutting this story removes it from the publishing pipeline. You can restore it later, but any in-progress review state is cleared.",
      confirmLabel: "Cut story",
      danger: true,
      onConfirm: () => handleDecision("killed", undefined, draftId),
    });
  };

  const handlePublish = async () => {
    if (!publishPath) return;
    try {
      setLoading(true);
      setErrorMessage("");
      setStatusMessage(`Compiling HTML, CSS, and RSS templates to static site at: ${publishPath}...`);
      const result = await publish(publishPath);
      setPublishResult(result);
      await refreshPublishHistory();
      setStatusMessage(`Static site compiled: ${result.article_count} article(s), ${result.files_written} file(s), ZIP package ready.`);
      setPublishStep(3);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleChooseLogo = async (): Promise<string | null> => {
    if (!isTauri()) {
      setErrorMessage("Logo import requires The Civic Desk desktop app.");
      return null;
    }
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: "Choose publication logo",
        filters: [{ name: "Logo image", extensions: ["png", "jpg", "jpeg", "gif", "webp"] }],
      });
      if (typeof selected !== "string") return null;
      const logoUrl = await importLogoAsset(selected);
      setStatusMessage("Logo image loaded. Save identity to use it in published output.");
      return logoUrl;
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
      return null;
    }
  };

  const refreshCivicIntelligence = async () => {
    if (!isTauri()) {
      setCivicIntelligence({
        observations: [],
        entities: [],
        source_scores: [],
        dark_signals: [],
      });
      return;
    }

    try {
      setCivicIntelligence(await getCivicIntelligence());
    } catch (err) {
      console.error("Failed to refresh civic intelligence snapshot", err);
      setErrorMessage(`Couldn't refresh civic intelligence: ${toUserMessage(err)}`);
    }
  };

  const refreshVerificationQueue = async () => {
    if (!isTauri()) {
      setVerificationQueue({ tasks: [], generated_count: 0 });
      return;
    }

    try {
      const snapshot = await getVerificationQueue();
      setVerificationQueue(snapshot);
      if (snapshot.generated_count > 0) {
        setStatusMessage(`Added ${snapshot.generated_count} verification task(s) from new signals.`);
      }
    } catch (err) {
      console.error("Failed to refresh verification queue", err);
      setErrorMessage(`Couldn't refresh verification queue: ${toUserMessage(err)}`);
    }
  };

  const handleVerificationTaskStatus = async (
    task: VerificationTask,
    status: VerificationTask["status"],
    resultSummary?: string
  ) => {
    if (!task.id) return;
    try {
      await updateVerificationTaskStatus(task.id, status, resultSummary);
      await refreshVerificationQueue();
      setStatusMessage(`Marked verification task ${status.replace(/_/g, " ")}.`);
    } catch (err) {
      setErrorMessage(`Couldn't update verification task: ${toUserMessage(err)}`);
    }
  };

  const handleCreateLeadFromDarkSignal = async (darkSignalId: number) => {
    try {
      const leadId = await createLeadFromDarkSignal(darkSignalId);
      await loadInitialData();
      setStatusMessage(`Created story lead #${leadId} from dark signal #${darkSignalId}.`);
      setActiveTab("queue");
    } catch (err) {
      setErrorMessage(`Couldn't create a story lead from this signal: ${toUserMessage(err)}`);
    }
  };

  const refreshSubscribers = async () => {
    if (!isTauri()) {
      setSubscribers([]);
      return;
    }
    try {
      setSubscribers(await listSubscribers());
    } catch (e) {
      console.error("Failed to load subscribers", e);
    }
  };

  const handleAddSubscriber = async () => {
    if (!subscriberEmail.trim()) {
      setErrorMessage("Enter a subscriber email address.");
      return;
    }
    try {
      setLoading(true);
      setErrorMessage("");
      await addSubscriber(subscriberEmail, subscriberName);
      setSubscriberEmail("");
      setSubscriberName("");
      await refreshSubscribers();
      setStatusMessage("Subscriber saved.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteSubscriber = async (id: number) => {
    try {
      setLoading(true);
      setErrorMessage("");
      await deleteSubscriber(id);
      await refreshSubscribers();
      setStatusMessage("Subscriber removed.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleImportSubscribersCsv = async () => {
    if (!isTauri()) {
      setErrorMessage("Subscriber CSV import requires The Civic Desk desktop app.");
      return;
    }
    try {
      setLoading(true);
      setErrorMessage("");
      const selected = await open({
        multiple: false,
        directory: false,
        title: "Choose subscriber CSV",
        filters: [{ name: "Subscriber CSV", extensions: ["csv", "txt"] }],
      });
      if (typeof selected !== "string") return;
      const count = await importSubscribersCsv(selected);
      await refreshSubscribers();
      setStatusMessage(`Imported ${count} subscriber(s).`);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleExportSubscribersCsv = async () => {
    if (!isTauri()) {
      setErrorMessage("Subscriber CSV export requires The Civic Desk desktop app.");
      return;
    }
    try {
      setLoading(true);
      setErrorMessage("");
      const selected = await save({
        title: "Export subscriber CSV",
        defaultPath: "civic-desk-subscribers.csv",
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (!selected) return;
      await exportSubscribersCsv(selected);
      setStatusMessage("Subscriber CSV exported.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleExportIssueEmail = async () => {
    const outputDir = publishResult?.output_dir || publishPath;
    if (!outputDir) {
      setErrorMessage("Compile the site before exporting an issue email.");
      return;
    }
    if (!isTauri()) {
      setErrorMessage("Issue email export requires The Civic Desk desktop app.");
      return;
    }
    try {
      setLoading(true);
      setErrorMessage("");
      const selected = await save({
        title: "Export issue email",
        defaultPath: "civic-desk-issue-email.md",
        filters: [{ name: "Markdown", extensions: ["md"] }],
      });
      if (!selected) return;
      await exportIssueEmail(outputDir, selected);
      setStatusMessage("Issue email markdown exported.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleCopyPublishText = async (label: string, text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setStatusMessage(`${label} copied.`);
    } catch {
      setErrorMessage("Couldn't copy to clipboard. Open the artifact file and copy it manually.");
    }
  };

  const handleCopyPublishArtifact = async (label: string, relativePath: string) => {
    const outputDir = publishResult?.output_dir || publishPath;
    if (!outputDir) {
      setErrorMessage("Compile the site before copying publish copy.");
      return;
    }
    try {
      const text = await readPublishArtifact(outputDir, relativePath);
      await handleCopyPublishText(label, text);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    }
  };

  const handleBulkImportTextChange = (value: string) => {
    setBulkImportText(value);
    setBulkImportReview({ accepted: [], rejected: [], duplicates: [] });
  };

  const handleBulkImportTypeChange = (value: string) => {
    setBulkImportType(value);
    setBulkImportReview({ accepted: [], rejected: [], duplicates: [] });
  };

  const handleBuildBulkImportReview = () => {
    const review = buildBulkImportReview(bulkImportText, bulkImportType, sources.map(source => source.url));
    setBulkImportReview(review);
    setStatusMessage(`Reviewed ${review.accepted.length} importable source(s), ${review.duplicates.length} duplicate(s), and ${review.rejected.length} skipped row(s).`);
  };

  const handleToggleBulkImportItem = (id: string) => {
    setBulkImportReview(prev => ({
      ...prev,
      accepted: prev.accepted.map(item => item.id === id ? { ...item, selected: !item.selected } : item),
    }));
  };

  const handleChooseBulkImportFile = async () => {
    if (!isTauri()) {
      setStatusMessage("File import is available in the desktop app. Paste URLs here in browser preview.");
      return;
    }
    try {
      setBulkImportLoading(true);
      setErrorMessage("");
      const selected = await open({
        multiple: false,
        directory: false,
        title: "Choose source list file",
        filters: [
          { name: "Source lists", extensions: ["txt", "csv", "tsv", "md", "html", "htm", "json", "docx", "xlsx", "pdf"] },
        ],
      });
      if (typeof selected !== "string") return;
      const text = await extractSourceImportText(selected);
      setBulkImportText(text);
      const review = buildBulkImportReview(text, bulkImportType, sources.map(source => source.url));
      setBulkImportReview(review);
      setStatusMessage(`Loaded ${review.accepted.length} importable source(s) from file for review.`);
    } catch (e) {
      setBulkImportText("");
      setBulkImportReview({ accepted: [], rejected: [], duplicates: [] });
      setErrorMessage(toUserMessage(e));
    } finally {
      setBulkImportLoading(false);
    }
  };

  const handleChoosePublishPath = async () => {
    if (!isTauri()) {
      setErrorMessage("Choosing a folder requires The Civic Desk desktop app.");
      return;
    }
    try {
      setErrorMessage("");
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Choose publish output folder",
        defaultPath: publishPath || undefined,
      });
      if (typeof selected === "string") {
        setPublishPath(selected);
        setPublishResult(null);
        await setSetting("paths.publish", selected);
        setStatusMessage("Publish folder selected.");
      }
    } catch (e: any) {
      setErrorMessage(`Couldn't choose publish folder: ${toUserMessage(e)}`);
    }
  };

  const handleRecordPublishDestination = async (provider: string, publishedUrl: string, deploymentId?: string) => {
    if (!publishPath) return;
    try {
      setLoading(true);
      setErrorMessage("");
      setStatusMessage("Recording public publishing URL and refreshing share files...");
      const result = await recordPublishDestination(publishPath, provider, publishedUrl, deploymentId);
      setPublishResult(result);
      await refreshPublishHistory();
      setStatusMessage("Public URL saved. Manifest, ZIP, newsletter, Substack draft, and social copy now use the live link.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handlePublishWithConnector = async (provider: string, publishedUrl: string, deploymentId?: string) => {
    if (!publishPath) return;
    try {
      setLoading(true);
      setErrorMessage("");
      setStatusMessage("Publishing through connector layer...");
      const result = await publishWithConnector(publishPath, provider, publishedUrl, deploymentId);
      setPublishResult(result);
      await refreshPublishHistory();
      setStatusMessage("Connector publish completed and publish history was updated.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const refreshPublishHistory = async () => {
    try {
      const runs = await listPublishHistory();
      setPublishHistory(runs);
    } catch (e) {
      console.error("Failed to load publish history", e);
    }
  };

  const handleLoadPublisherConfig = async (provider: string) => {
    setPublisherProvider(provider);
    setPublisherTestResult(null);
    try {
      const config = await getPublisherConfig(provider);
      setPublisherConfig(config);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    }
  };

  const handleSavePublisherConfig = async (config: PublisherConfigInput) => {
    try {
      setLoading(true);
      setErrorMessage("");
      const saved = await savePublisherConfig(config);
      setPublisherProvider(saved.provider);
      setPublisherConfig(saved);
      setPublisherTestResult(null);
      setStatusMessage("Publisher connector settings saved. Credentials are stored in the operating system credential store.");
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handleTestPublisherConnection = async (provider: string) => {
    try {
      setLoading(true);
      setErrorMessage("");
      const result = await testPublisherConnection(provider);
      setPublisherTestResult(result);
      setStatusMessage(result.message);
    } catch (e: any) {
      setErrorMessage(toUserMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const handlePublishPathChange = (value: string) => {
    setPublishPath(value);
    setPublishResult(null);
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

  const handleImproveForPublication = async () => {
    if (!selectedDraft || !selectedDraft.content) return;
    try {
      setLoading(true);
      setErrorMessage("");
      if (!(await ensureSelectedModelReady("Improving a draft for publication"))) return;
      const systemPrompt =
        "You are a careful local newspaper editor. Improve copy for publication without inventing facts. Preserve all evidence:ID citations. Return clean Markdown only.";
      const promptText = `Improve this draft so it reads like reader-facing local newspaper copy, not reporter notes.

Rules:
- Keep the editor's facts only; do not invent quotes, dates, dollar amounts, causes, impacts, or source details.
- First line must be Headline: followed by a concise, specific headline.
- Remove reporter scaffolding such as EDITOR_NOTE, Body:, Nut graf, Reporting Steps, [Source needed], [Verification needed], and placeholders.
- Preserve inline evidence citations exactly when present.
- If the source support is thin, make it a brief or watch item and say what is known, not what is guessed.
- Neutral tone. No advocacy. Do not imply the software makes the editor's decision.

Current format: ${selectedDraft.format}
Current title: ${selectedDraft.title}

Draft:
${selectedDraft.content}`;

      const improved = await llmTask(promptText, systemPrompt);
      const allowedEvidenceIds = evidenceList
        .map((item) => item.id)
        .filter((id): id is number => typeof id === "number");
      const citationSafe = sanitizeEvidenceCitations(improved, allowedEvidenceIds);
      const normalized = normalizeGeneratedDraft(citationSafe, selectedDraft.title);
      setSelectedDraft({
        ...selectedDraft,
        title: normalized.title,
        content: normalized.content,
        updated_at: new Date().toISOString(),
      });
      setStatusMessage("Improved draft is loaded in the editor. Review it, then save if you want to keep it.");
    } catch (e: any) {
      setErrorMessage(`Failed to improve draft: ${toUserMessage(e)}`);
    } finally {
      setLoading(false);
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
    if (!isTauri()) {
      setPullingModel(true);
      setPullProgressText(["Preparing preview download...", "Preview download complete (100%)"]);
      window.setTimeout(() => setPullingModel(false), 350);
      return;
    }

    setPullingModel(true);
    setPullProgressText(["Initializing download..."]);
    pullOllamaModel(wizardModel).catch((e) => {
      setPullingModel(false);
      setErrorMessage(toUserMessage(e));
    });
  };

  const handleInstallRuntime = () => {
    if (!isTauri()) {
      setStatusMessage("Preview mode cannot install the local AI runtime.");
      return;
    }

    setPullingModel(true);
    setPullProgressText(["Preparing local AI runtime install..."]);
    installOllamaRuntime()
      .then(async () => {
        setPullingModel(false);
        setPullProgressText(prev => [...prev, "Local AI runtime is ready."]);
        await pollOllamaStatus();
      })
      .catch((e) => {
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
    civicIntelligence,
    verificationQueue,
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
    publishResult,
    publishHistory,
    publisherConfig,
    publisherProvider,
    publisherTestResult,
    subscribers,
    subscriberEmail,
    setSubscriberEmail,
    subscriberName,
    setSubscriberName,
    setPublishPath: handlePublishPathChange,
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
    setBulkImportText: handleBulkImportTextChange,
    bulkImportType,
    setBulkImportType: handleBulkImportTypeChange,
    bulkImportLoading,
    bulkImportReview,
    socialPackResult,
    setSocialPackResult,
    isGeneratingSocial,
    publishStep,
    setPublishStep,
    handleChoosePublishPath,
    ollamaOnline,
    systemRam,
    onboardingDone,
    completeOnboarding,
    selectedModel,
    aiSetupSkipped,
    installedModels,
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
    dailyScanProgress,
    appVersion,
    loading,
    statusMessage,
    setStatusMessage,
    errorMessage,
    setErrorMessage,
    pullLogEndRef,
    loadInitialData,
    refreshCivicIntelligence,
    refreshVerificationQueue,
    handleVerificationTaskStatus,
    handleCreateLeadFromDarkSignal,
    pollOllamaStatus,
    handleIngest,
    handleDailyScan,
    handleAddSource,
    handleDeleteSource,
    handleRunDiscovery,
    handleToggleDiscoveredSource,
    handleImportDiscoveredSources,
    handleBulkImport,
    handleBuildBulkImportReview,
    handleToggleBulkImportItem,
    handleChooseBulkImportFile,
    handleGeneratePin,
    handleRevokeClient,
    handleSaveProfile,
    handleChooseLogo,
    handleOpenDraftWizard,
    handleGenerateText,
    handleOpenDraftEditor,
    handleSaveDraftEditor,
    handleDecision,
    handleApprovePublish,
    handleKillStory,
    handlePublish,
    handleRecordPublishDestination,
    handlePublishWithConnector,
    handleLoadPublisherConfig,
    handleSavePublisherConfig,
    handleTestPublisherConnection,
    refreshPublishHistory,
    refreshSubscribers,
    handleAddSubscriber,
    handleDeleteSubscriber,
    handleImportSubscribersCsv,
    handleExportSubscribersCsv,
    handleExportIssueEmail,
    handleCopyPublishText,
    handleCopyPublishArtifact,
    openCorrectionModal,
    handleRegisterCorrection,
    handleDeleteDraft,
    handleGenerateSocial,
    handleImproveForPublication,
    handleBackupSave,
    handleBackupRestore,
    handlePullModel,
    handleInstallRuntime,
    handleCustomLlmTask
  };
}
