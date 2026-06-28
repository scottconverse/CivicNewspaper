// STEPS DEFINED HERE ARE DOCUMENTED IN docs/user_manual.md PART 1. Update both together.
import React, { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { appDataDir, join } from "@tauri-apps/api/path";
import { ChevronRight, Download, CheckCircle, RefreshCcw, AlertCircle } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import {
  getSystemRam,
  getSetting,
  setSetting,
  getCommunityProfile,
  saveCommunityProfile,
  ollamaHealth,
  installOllamaRuntime,
  pullOllamaModel,
  cancelOllamaPull,
  exportDiagnostics,
  setOnboardingComplete,
  isTauri,
  toUserMessage,
} from "../ipc";
import modelsConfig from "../models.json";
import { ConfirmModal } from "./ConfirmModal";

// Minimum system RAM (GB) for the low-RAM model to run at usable
// speed. Below this floor we still allow setup but warn the user that AI
// features may run slowly.
const LOW_RAM_FLOOR_GB = 8;

// QA-M3: a local model on CPU is slow even with adequate RAM, so caution at the
// medium/high tiers too — not just below the low-RAM floor.
const SLOW_CPU_CAUTION =
  "Heads up: the AI model runs on your CPU, so generating a draft or daily scan can take a minute or more — this is normal.";

// Approximate one-time download sizes, sourced from models.json so the wizard
// can disclose the size up front (UX-C1) instead of springing a multi-GB
// download on the user.
const modelSizes: Record<string, string> = (modelsConfig as any).sizes || {};
function downloadSizeFor(modelTag: string): string {
  return modelSizes[modelTag] || "a few GB";
}

const starterProfiles = [
  {
    label: "Longmont",
    pubName: "Longmont Civic Desk",
    editorName: "Local editor",
    organizationType: "single_person",
    city: "Longmont",
    state: "CO",
  },
  {
    label: "Brighton",
    pubName: "Brighton Civic Desk",
    editorName: "Local editor",
    organizationType: "single_person",
    city: "Brighton",
    state: "CO",
  },
  {
    label: "Denver",
    pubName: "Denver Civic Desk",
    editorName: "Local editor",
    organizationType: "single_person",
    city: "Denver",
    state: "CO",
  },
];

interface OnboardingWizardProps {
  ollamaOnline: boolean;
  systemRam: number;
  onComplete: () => void;
  initialStep?: number;
}

interface OllamaState {
  reachable: boolean;
  models: string[];
  version: string | null;
}

export const OnboardingWizard: React.FC<OnboardingWizardProps> = ({ 
  ollamaOnline, 
  systemRam, 
  onComplete, 
  initialStep 
}) => {
  const [step, setStep] = useState<number>(initialStep || 1);
  const [model, setModel] = useState<string>("");
  const [skipConfirm, setSkipConfirm] = useState<{
    title: string;
    message: string;
    confirmLabel: string;
    onConfirm: () => void | Promise<void>;
  } | null>(null);
  
  // Step 1 State
  const [pubName, setPubName] = useState("");
  const [editorName, setEditorName] = useState("");
  const [organizationType, setOrganizationType] = useState("single_person");
  const [city, setCity] = useState("");
  const [state, setState] = useState("");

  // Step 2 State
  const [health, setHealth] = useState<OllamaState | null>(null);
  const [checkingHealth, setCheckingHealth] = useState(false);
  const [sysRam, setSysRam] = useState<number>(systemRam || 0);
  const [healthTimeout, setHealthTimeout] = useState(false);
  const [retryCount, setRetryCount] = useState(0);
  const [exportStatus, setExportStatus] = useState("");
  const [runtimeInstalling, setRuntimeInstalling] = useState(false);
  const [runtimeProgress, setRuntimeProgress] = useState("");
  const [runtimePercent, setRuntimePercent] = useState<number | null>(null);
  const [runtimeError, setRuntimeError] = useState("");
  const autoRuntimeInstallAttempted = useRef(false);
  const userInteractedOnIdentityRef = useRef(false);
  const identityRescueAttemptedRef = useRef(false);
  const pubNameInputRef = useRef<HTMLInputElement | null>(null);
  const editorNameInputRef = useRef<HTMLInputElement | null>(null);
  const organizationTypeSelectRef = useRef<HTMLSelectElement | null>(null);
  const cityInputRef = useRef<HTMLInputElement | null>(null);
  const stateInputRef = useRef<HTMLInputElement | null>(null);

  // Step 3 State
  const [pullProgress, setPullProgress] = useState<string>("");
  const [pullPercent, setPullPercent] = useState<number | null>(null);
  const [pulling, setPulling] = useState(false);
  const [pullComplete, setPullComplete] = useState(false);
  const [pullError, setPullError] = useState<string>("");
  const [autoStartPull, setAutoStartPull] = useState(false);

  // Step 4 State
  const [publishPath, setPublishPath] = useState("");
  const [backupPath, setBackupPath] = useState("");

  // Init error surfacing state (WU-Nit-1)
  const [initError, setInitError] = useState<string | null>(null);
  const [setupNotice, setSetupNotice] = useState<string | null>(null);

  const steps = [
    { title: "Identity", desc: "Define your local news outlet name and mission." },
    { title: "AI Service Setup", desc: "Check the connection to the local AI service that runs on your computer." },
    { title: "Download AI Model", desc: "Download the local AI model. One-time setup — needs an internet connection." },
    { title: "Defaults", desc: "Configure publication directories and backup database paths." },
    { title: "Done", desc: "Onboarding completed. Ready to inspect local stories." }
  ];

  const applyIdentityValues = (values: {
    pubName: string;
    editorName: string;
    organizationType: string;
    city: string;
    state: string;
  }) => {
    setPubName(values.pubName);
    setEditorName(values.editorName);
    setOrganizationType(values.organizationType);
    setCity(values.city);
    setState(values.state);

    if (pubNameInputRef.current) pubNameInputRef.current.value = values.pubName;
    if (editorNameInputRef.current) editorNameInputRef.current.value = values.editorName;
    if (organizationTypeSelectRef.current) organizationTypeSelectRef.current.value = values.organizationType;
    if (cityInputRef.current) cityInputRef.current.value = values.city;
    if (stateInputRef.current) stateInputRef.current.value = values.state;
  };

  const currentIdentityValues = () => ({
    pubName: pubNameInputRef.current?.value ?? pubName,
    editorName: editorNameInputRef.current?.value ?? editorName,
    organizationType: organizationTypeSelectRef.current?.value ?? organizationType,
    city: cityInputRef.current?.value ?? city,
    state: stateInputRef.current?.value ?? state,
  });

  const persistIdentity = async (identity = currentIdentityValues()) => {
    setPubName(identity.pubName);
    setEditorName(identity.editorName);
    setOrganizationType(identity.organizationType);
    setCity(identity.city);
    setState(identity.state);

    await setSetting("identity.newsroom_name", identity.pubName);
    await setSetting("identity.editor_name", identity.editorName);
    await setSetting("identity.organization_type", identity.organizationType);
    await setSetting("identity.city", identity.city);
    await setSetting("identity.state", identity.state);

    try {
      const profile = await getCommunityProfile();
      await saveCommunityProfile({
        ...profile,
        site_title: identity.pubName.trim() || profile.site_title,
        organization_type: identity.organizationType,
        city: identity.city.trim() || profile.city,
        state: identity.state.trim() || profile.state,
      });
    } catch {
      /* non-fatal — identity settings above are still saved */
    }
  };

  // Initialize paths and ram
  useEffect(() => {
    async function init() {
      try {
        if (isTauri()) {
          const appData = await appDataDir();
          const pPath = await join(appData, "sites", "default");
          setPublishPath(pPath);

          const bPath = await join(appData, "backups");
          setBackupPath(bPath);
        } else {
          setPublishPath("C:\\CivicNews\\sites\\default");
          setBackupPath("C:\\CivicNews\\backups");
        }

        const ram = systemRam || await getSystemRam();
        setSysRam(ram);

        const selected = isTauri() ? await getSetting("model.selected") : null;
        if (selected) {
          setModel(selected);
        } else {
          const fallback = ram >= 16 ? modelsConfig.high : ram >= 8 ? modelsConfig.medium : modelsConfig.low;
          setModel(fallback);
        }

        if (ollamaOnline !== undefined) {
          setHealth({ reachable: ollamaOnline, models: [], version: null });
        }
      } catch (e: any) {
        console.error(e);
        setInitError(e?.message || String(e));
      }
    }
    init();
  }, [systemRam, ollamaOnline]);

  // Health check loop for Step 2 (WU-2)
  useEffect(() => {
    let intervalId: any;
    let timeoutId: any;
    let isFirst = true;

    if (step === 2) {
      setCheckingHealth(true);
      setHealthTimeout(false);
      
      const check = async () => {
        try {
          const result = await ollamaHealth();
          setHealth(result);
          
          if (result.reachable) {
            clearInterval(intervalId);
            clearTimeout(timeoutId);
            if (result.models.length > 0 && (!model || !result.models.includes(model))) {
              setModel(result.models[0]);
            }
          }
        } catch (e) {
          console.error(e);
        } finally {
          if (isFirst) {
            setCheckingHealth(false);
            isFirst = false;
          }
        }
      };

      check();
      intervalId = setInterval(check, 2000);

      timeoutId = setTimeout(() => {
        clearInterval(intervalId);
        setCheckingHealth(false);
        setHealthTimeout(true);
      }, 30000);
    }

    return () => {
      clearInterval(intervalId);
      clearTimeout(timeoutId);
    };
  }, [step, retryCount]);

  const handleExportDiagnostics = async () => {
    try {
      const path = await save({
        defaultPath: 'civicnews-diagnostics.json',
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (path) {
        setExportStatus("Exporting...");
        await exportDiagnostics(path);
        setExportStatus("Export successful!");
        setTimeout(() => setExportStatus(""), 3000);
      }
    } catch (e) {
      setExportStatus(`Export failed: ${toUserMessage(e)}`);
    }
  };

  const installRuntime = async (): Promise<boolean> => {
    setRuntimeInstalling(true);
    setRuntimeError("");
    setRuntimeProgress("Preparing local AI runtime install...");
    setRuntimePercent(0);
    setInitError(null);
    let unlisten: (() => void) | null = null;
    try {
      try {
        unlisten = await listen<{ stage: string; message: string; completed?: number | null; total?: number | null }>(
          "ollama-runtime-install-progress",
          (event) => {
            setRuntimeProgress(event.payload.message);
            if (event.payload.completed !== undefined && event.payload.completed !== null && event.payload.total) {
              setRuntimePercent((event.payload.completed / event.payload.total) * 100);
            } else if (event.payload.stage === "verify") {
              setRuntimePercent(100);
            } else if (event.payload.stage === "extract" || event.payload.stage === "start") {
              setRuntimePercent(null);
            }
          }
        );
      } catch (eventError) {
        console.warn("Runtime progress listener could not start; continuing install.", eventError);
        setRuntimeProgress("Starting local AI runtime install...");
      }
      await installOllamaRuntime();
      setRuntimeProgress("Local AI runtime is ready.");
      setRuntimePercent(100);
      setHealthTimeout(false);
      setRetryCount(c => c + 1);
      const result = await ollamaHealth();
      setHealth(result);
      return result.reachable;
    } catch (e) {
      const message = toUserMessage(e);
      setRuntimeError(message);
      setInitError(`Local AI runtime install failed: ${message}`);
      setRuntimeProgress("");
      return false;
    } finally {
      unlisten?.();
      setRuntimeInstalling(false);
    }
  };

  useEffect(() => {
    if (
      step !== 2 ||
      checkingHealth ||
      runtimeInstalling ||
      autoRuntimeInstallAttempted.current ||
      health?.reachable
    ) {
      return;
    }
    if (healthTimeout || health?.reachable === false) {
      autoRuntimeInstallAttempted.current = true;
      void installRuntime();
    }
  }, [step, checkingHealth, runtimeInstalling, healthTimeout, health?.reachable]);

  const formatStatus = (status: string): string => {
    const s = status.toLowerCase();
    if (s.includes("pulling manifest") || s.includes("pulling")) return "Initializing download...";
    if (s.includes("downloading")) return "Downloading model files...";
    if (s.includes("verifying")) return "Verifying model integrity...";
    if (s.includes("writing")) return "Completing setup...";
    if (s.includes("success")) return "Download complete!";
    if (s.includes("error")) return "Error downloading model.";
    return status;
  };

  const startPullModel = async () => {
    const modelToPull = model;
    if (!modelToPull) {
      setPullError("No model is selected yet. Go back and choose a model, then try again.");
      return;
    }

    setPulling(true);
    setPullProgress("Starting pull...");
    setPullPercent(0);
    setPullComplete(false);
    setPullError("");

    try {
      await listen<{ model: string; status: string; completed?: number; total?: number }>(
        "ollama-pull-progress",
        (event) => {
          setPullProgress(formatStatus(event.payload.status));
          if (
            event.payload.completed !== undefined &&
            event.payload.total !== undefined &&
            event.payload.total > 0
          ) {
            setPullPercent((event.payload.completed / event.payload.total) * 100);
          }
          if (
            event.payload.status === "success" ||
            event.payload.status.toLowerCase().includes("success")
          ) {
            setPullComplete(true);
            setPulling(false);
          }
          if (
            event.payload.status === "cancelled" ||
            event.payload.status.toLowerCase().includes("cancel")
          ) {
            setPullComplete(false);
            setPulling(false);
          }
        }
      );
      await listen<void>("ollama-pull-complete", () => {
        setPullProgress("Download complete!");
        setPullPercent(100);
        setPullComplete(true);
        setPulling(false);
      });
      await listen<string>("ollama-pull-error", (event) => {
        setPullError(
          `Download failed: ${event.payload || "The model service reported an error."} ` +
            `Check your internet connection, then click "Download ${modelToPull}" to try again.`
        );
        setPullProgress("");
        setPulling(false);
      });

      await pullOllamaModel(modelToPull);
      await setSetting("model.selected", modelToPull);
    } catch (e) {
      console.error(e);
      const reason = (e instanceof Error ? e.message : String(e)).trim();
      setPullError(
        `Download failed${reason ? `: ${reason}` : "."} ` +
          `Make sure the AI service is running and your internet connection is working, then click "Download ${modelToPull}" to try again.`
      );
      setPullProgress("");
      setPulling(false);
    }
  };

  useEffect(() => {
    if (step !== 3 || !autoStartPull || pulling || pullComplete) {
      return;
    }
    setAutoStartPull(false);
    void startPullModel();
  }, [step, autoStartPull, pulling, pullComplete, model]);

  const cancelPullModel = async () => {
    try {
      await cancelOllamaPull(model);
      setPulling(false);
      setPullComplete(false);
      setPullProgress("Pull cancelled.");
    } catch (e) {
      console.error(e);
    }
  };

  const handleNext = async () => {
    // QA-005: every branch persists settings over IPC, any of which can reject.
    // Without this guard a failed write left the wizard silently stuck (no
    // advance, no message). Surface failures via the existing initError banner.
    try {
      setInitError(null);
      if (step === 1) {
        setSetupNotice(null);
        await persistIdentity();
        setStep(2);
      } else if (step === 2) {
        if (health && health.reachable && health.models.includes(model)) {
          // Model is already installed, skip Step 3 and go directly to Step 4
          await setSetting("model.selected", model);
          setStep(4);
        } else if (!health?.reachable) {
          const ready = await installRuntime();
          if (ready) {
            setStep(3);
          }
        } else {
          setAutoStartPull(true);
          setStep(3);
        }
      } else if (step === 3) {
        const modelReady = pullComplete || Boolean(health?.models?.includes(model));
        if (!modelReady) {
          setSkipConfirm({
            title: "Skip the model download?",
            message: "Daily Scan and AI drafting will run in limited mode until you download a model from AI Model.",
            confirmLabel: "Skip download",
            onConfirm: async () => {
              await setSetting("model.selected", model);
              setStep(4);
            },
          });
          return;
        }
        await setSetting("model.selected", model);
        setStep(4);
      } else if (step === 4) {
        // Persist defaults
        await setSetting("paths.publish", publishPath);
        await setSetting("paths.backup", backupPath);

        setStep(5);
      } else if (step === 5) {
        await setOnboardingComplete(true);
        onComplete();
      }
    } catch (e) {
      console.error(e);
      setInitError(toUserMessage(e));
    }
  };

  const handleBack = () => {
    if (step > 1) setStep(prev => prev - 1);
  };

  useEffect(() => {
    if (step === 1) {
      window.setTimeout(() => pubNameInputRef.current?.focus(), 0);
    }
  }, [step]);

  useEffect(() => {
    if (step !== 1) return;

    const handleStarterRoute = () => {
      const hashParams = window.location.hash.startsWith("#")
        ? new URLSearchParams(window.location.hash.slice(1))
        : new URLSearchParams();
      const params = window.location.search
        ? new URLSearchParams(window.location.search)
        : hashParams;
      const starter = params.get("starter");
      const profile = starterProfiles.find(item => item.label.toLowerCase() === starter?.toLowerCase());
      if (!profile) return;

      applyIdentityValues(profile);
      window.history.replaceState(null, "", window.location.pathname);

      if (params.get("continueSetup") === "1") {
        void persistIdentity(profile)
          .then(() => setStep(2))
          .catch(e => {
            console.error(e);
            setInitError(toUserMessage(e));
          });
      }
    };

    handleStarterRoute();
    window.addEventListener("hashchange", handleStarterRoute);
    return () => window.removeEventListener("hashchange", handleStarterRoute);
  }, [step]);

  useEffect(() => {
    if (step !== 1 || identityRescueAttemptedRef.current) return;

    const markInteraction = () => {
      userInteractedOnIdentityRef.current = true;
    };

    window.addEventListener("pointerdown", markInteraction, true);
    window.addEventListener("click", markInteraction, true);
    window.addEventListener("keydown", markInteraction, true);
    window.addEventListener("input", markInteraction, true);

    const rescueTimer = window.setTimeout(() => {
      if (identityRescueAttemptedRef.current || userInteractedOnIdentityRef.current) return;

      const fieldsAreEmpty = !currentIdentityValues().pubName.trim()
        && !currentIdentityValues().editorName.trim()
        && !currentIdentityValues().city.trim()
        && !currentIdentityValues().state.trim();
      if (!fieldsAreEmpty) return;

      const profile = starterProfiles[0];
      identityRescueAttemptedRef.current = true;
      applyIdentityValues(profile);
      setSetupNotice("The setup screen did not receive input events, so The Civic Desk continued with a starter Longmont profile. You can edit identity later in Settings.");
      void persistIdentity(profile)
        .then(() => setStep(2))
        .catch(e => {
          console.error(e);
          setInitError(toUserMessage(e));
        });
    }, 12000);

    return () => {
      window.clearTimeout(rescueTimer);
      window.removeEventListener("pointerdown", markInteraction, true);
      window.removeEventListener("click", markInteraction, true);
      window.removeEventListener("keydown", markInteraction, true);
      window.removeEventListener("input", markInteraction, true);
    };
  }, [step]);

  return (
    <div className="wizard-container card" id="onboarding-wizard">
      {initError && (
        <div style={{ background: "rgba(239, 68, 68, 0.05)", borderLeft: "4px solid var(--color-error)", padding: "0.75rem", borderRadius: "4px", marginBottom: "1rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
          <AlertCircle size={16} style={{ color: "var(--color-error)" }} />
          <span style={{ fontSize: "0.85rem", color: "var(--color-error)" }}>Initialization Error: {initError}</span>
        </div>
      )}
      {setupNotice && (
        <div className="onboarding-notice" role="status">
          {setupNotice}
        </div>
      )}

      <div className="flex-between">
        <h2>AI Setup</h2>
        <span style={{ fontWeight: 600, fontSize: "0.9rem", color: "var(--text-secondary)" }}>
          Step {step} of {steps.length}
        </span>
      </div>

      <div className="progress-bar-container">
        <div 
          className="progress-bar" 
          style={{ width: `${(step / steps.length) * 100}%` }}
          data-testid="progress-bar"
        />
      </div>

      <div className="onboarding-step-body">
        <h3>{steps[step - 1].title}</h3>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          {steps[step - 1].desc}
        </p>

        {/* STEP 1: IDENTITY */}
        {step === 1 && (
          <div className="onboarding-identity-fields">
            <div className="onboarding-starter-profiles" aria-label="Starter profiles">
              <span>Starter profiles</span>
              <div>
                {starterProfiles.map(profile => (
                  <a
                    key={profile.label}
                    className="btn btn-secondary btn-sm"
                    href={`#starter=${encodeURIComponent(profile.label.toLowerCase())}&continueSetup=1`}
                    onClick={() => applyIdentityValues(profile)}
                  >
                    {profile.label}
                  </a>
                ))}
              </div>
            </div>
            <div className="onboarding-field">
              <label htmlFor="onboarding-publication-name">Publication Name</label>
              <input
                id="onboarding-publication-name"
                ref={pubNameInputRef}
                autoFocus
                type="text"
                placeholder="e.g. The Brighton Gazette"
                defaultValue={pubName}
                onInput={e => setPubName(e.currentTarget.value)}
                onChange={e => setPubName(e.target.value)}
              />
            </div>
            <div className="onboarding-field">
              <label htmlFor="onboarding-editor-name">Editor Name</label>
              <input
                id="onboarding-editor-name"
                ref={editorNameInputRef}
                type="text"
                placeholder="e.g. Jane Doe"
                defaultValue={editorName}
                onInput={e => setEditorName(e.currentTarget.value)}
                onChange={e => setEditorName(e.target.value)}
              />
            </div>
            <div className="onboarding-field">
              <label htmlFor="onboarding-organization-type">Publisher Type</label>
              <select
                id="onboarding-organization-type"
                ref={organizationTypeSelectRef}
                defaultValue={organizationType}
                onInput={e => setOrganizationType(e.currentTarget.value)}
                onChange={e => setOrganizationType(e.target.value)}
              >
                <option value="single_person">Single person</option>
                <option value="for_profit">For-profit publication</option>
                <option value="nonprofit">Nonprofit publication</option>
                <option value="private_org">Private organization</option>
                <option value="community_group">Community group</option>
                <option value="other">Other</option>
              </select>
            </div>
            <div className="onboarding-field-row">
              <div className="onboarding-field">
                <label htmlFor="onboarding-city">City</label>
                <input
                  id="onboarding-city"
                  ref={cityInputRef}
                  type="text"
                  placeholder="Brighton"
                  defaultValue={city}
                  onInput={e => setCity(e.currentTarget.value)}
                  onChange={e => setCity(e.target.value)}
                />
              </div>
              <div className="onboarding-field">
                <label htmlFor="onboarding-state">State</label>
                <input
                  id="onboarding-state"
                  ref={stateInputRef}
                  type="text"
                  placeholder="CO"
                  defaultValue={state}
                  onInput={e => setState(e.currentTarget.value)}
                  onChange={e => setState(e.target.value)}
                />
              </div>
            </div>
          </div>
        )}

        {/* STEP 2: AI SERVICE SETUP */}
        {step === 2 && (
          <div className="card">
            {checkingHealth ? (
              <div style={{ textAlign: "center", padding: "2rem 0" }}>
                <RefreshCcw className="animate-spin" size={32} style={{ color: "var(--accent-primary)", marginBottom: "1rem" }} />
                <p style={{ fontSize: "0.95rem" }}>Starting the local AI service...</p>
              </div>
            ) : (
              <>
                {health && (
                  <div className="flex-between" style={{ marginBottom: "1rem" }}>
                    <div>
                      <strong>Local AI Service Connection</strong>
                      <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>Local Ram: {sysRam} GB</p>
                    </div>
                    <span className={`status-dot ${health.reachable ? "online" : "offline"}`} />
                  </div>
                )}

                {/* Timeout State (WU-2) */}
                {healthTimeout && (
                  <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-error)" }}>Couldn't reach the AI service</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>
                      The private AI service did not start. First try restarting Civic Desk. If Windows or antivirus asked about this app, allow it, then retry. If it still fails, save a diagnostics file for support.
                    </p>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>
                      If this is a clean machine, Civic Desk can download and install its local AI runtime for you. This is a large one-time download and may take a while.
                    </p>
                    {runtimeError && (
                      <p style={{ fontSize: "0.85rem", color: "var(--color-error)", marginBottom: "0.5rem" }}>{runtimeError}</p>
                    )}
                    {runtimeProgress && (
                      <div style={{ marginBottom: "0.75rem" }}>
                        <div style={{ display: "flex", justifyContent: "space-between", fontSize: "0.85rem", marginBottom: "0.35rem" }}>
                          <span>{runtimeProgress}</span>
                          {runtimePercent !== null && <span>{runtimePercent.toFixed(1)}%</span>}
                        </div>
                        {runtimePercent !== null && (
                          <div className="progress-bar-container" style={{ background: "var(--border-color)", height: "8px", borderRadius: "4px" }}>
                            <div style={{ height: "100%", background: "var(--accent-primary)", width: `${runtimePercent}%`, transition: "width 0.2s" }} />
                          </div>
                        )}
                      </div>
                    )}
                    {exportStatus && (
                      <p style={{ fontSize: "0.85rem", color: "var(--accent-primary)", marginBottom: "0.5rem" }}>{exportStatus}</p>
                    )}
                    <div style={{ display: "flex", gap: "1rem", flexWrap: "wrap" }}>
                      <button type="button" className="btn btn-primary btn-sm" onClick={() => void installRuntime()} disabled={runtimeInstalling}>
                        <Download size={14} style={{ marginRight: "0.5rem" }} /> {runtimeInstalling ? "Installing..." : "Install local AI runtime"}
                      </button>
                      <button type="button" className="btn btn-primary btn-sm" onClick={() => { setHealthTimeout(false); setCheckingHealth(true); setRetryCount(c => c + 1); }}>
                        <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} /> Retry
                      </button>
                      <button type="button" className="btn btn-secondary btn-sm" onClick={handleExportDiagnostics}>
                        Save diagnostics file
                      </button>
                    </div>
                  </div>
                )}

                {!healthTimeout && health && !health.reachable && (
                  <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-error)" }}>Starting the local AI service</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>CivicNews includes a local AI service that runs on your computer. It may take a moment to start up. Once it's running, you'll download a model in the next step.</p>
                    <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginBottom: "1rem" }}>
                      On a clean machine, use the install button if the service does not become ready.
                    </p>
                    {runtimeError && (
                      <p style={{ fontSize: "0.85rem", color: "var(--color-error)", marginBottom: "0.5rem" }}>{runtimeError}</p>
                    )}
                    {runtimeProgress && (
                      <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginBottom: "0.5rem" }}>{runtimeProgress}</p>
                    )}
                    <div style={{ display: "flex", gap: "1rem", flexWrap: "wrap" }}>
                      <button type="button" className="btn btn-primary" onClick={() => void installRuntime()} disabled={runtimeInstalling}>
                        <Download size={14} style={{ marginRight: "0.5rem" }} />
                        {runtimeInstalling ? "Installing..." : "Install local AI runtime"}
                      </button>
                      <button type="button" className="btn btn-secondary" onClick={() => setRetryCount(c => c + 1)} disabled={checkingHealth}>
                        <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} />
                        {checkingHealth ? "Checking..." : "Check Initialization Status"}
                      </button>
                    </div>
                  </div>
                )}

                {/* Reachable, no models (WU-7 action hint) */}
                {!healthTimeout && health && health.reachable && health.models.length === 0 && (
                  <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-success)" }}>The AI service is ready. Download a recommended model?</h4>
                    <p style={{ fontSize: "0.9rem" }}>
                      Based on your {sysRam}GB of RAM, we recommend: <strong>{model}</strong> (one-time download, {downloadSizeFor(model)}, needs internet).
                    </p>
                    <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.5rem" }}>
                      This may take 10-60+ minutes depending on your connection. You can cancel and resume later from AI Model; already downloaded pieces are usually reused by the model service.
                    </p>
                    {sysRam > 0 && sysRam < LOW_RAM_FLOOR_GB ? (
                      <p
                        data-testid="low-ram-warning"
                        style={{ fontSize: "0.85rem", color: "var(--color-error)", marginTop: "0.5rem", display: "flex", alignItems: "flex-start", gap: "0.4rem" }}
                      >
                        <AlertCircle size={16} style={{ flexShrink: 0, marginTop: "0.1rem" }} />
                        Your system has {sysRam}GB of RAM, below the {LOW_RAM_FLOOR_GB}GB recommended for local AI. {model} will still run, but generation may be slow.
                      </p>
                    ) : (
                      <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.5rem", display: "flex", alignItems: "flex-start", gap: "0.4rem" }}>
                        <AlertCircle size={16} style={{ flexShrink: 0, marginTop: "0.1rem" }} />
                        {SLOW_CPU_CAUTION}
                      </p>
                    )}
                    <button
                      type="button"
                      className="btn btn-primary"
                      onClick={() => {
                        setAutoStartPull(true);
                        setStep(3);
                      }}
                      disabled={pulling}
                      style={{ marginTop: "1rem" }}
                    >
                      <Download size={16} style={{ marginRight: "0.5rem" }} />
                      {pulling ? "Downloading..." : `Download ${model}`}
                    </button>
                  </div>
                )}

                {/* Reachable with models (WU-4 use existing model) */}
                {!healthTimeout && health && health.reachable && health.models.length > 0 && (
                  <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-success)" }}>Use an existing model?</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "0.5rem" }}>
                      We detected the following models already installed on your computer. Select one to use it and skip downloading:
                    </p>
                    {/* installedModels from api/tags are selectable: Use existing model if you already have it. */}
                    <select 
                      value={model} 
                      onChange={e => setModel(e.target.value)}
                      style={{ width: "100%", padding: "0.5rem", borderRadius: "4px", border: "1px solid var(--border-color)", background: "var(--bg-card)", color: "var(--text-primary)" }}
                    >
                      {health.models.map(m => <option key={m} value={m}>{m}</option>)}
                      <option value="" disabled hidden>-- Or pull a recommended model --</option>
                    </select>
                  </div>
                )}
              </>
            )}
          </div>
        )}

        {/* STEP 3: DOWNLOAD AI MODEL */}
        {step === 3 && (
          <div>
            <div style={{ background: "var(--accent-light)", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
              <strong>AI Model: {model} (Recommended)</strong>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>
                CivicNews will download this local AI model now - a one-time download of about {downloadSizeFor(model)} that needs an internet connection. This may take 10-60+ minutes. After this, the AI runs fully offline on your computer.
              </p>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>
                It is safe to cancel and resume later from AI Model. If the download appears stuck for several minutes, check your internet connection, restart Civic Desk, and retry.
              </p>
            </div>
            
            {!pulling && !pullComplete && (
              <div>
                {pullError && (
                  <div
                    data-testid="pull-error"
                    style={{ marginBottom: "1rem", background: "rgba(239, 68, 68, 0.06)", borderLeft: "4px solid var(--color-error)", padding: "0.75rem", borderRadius: "4px" }}
                  >
                    <p style={{ fontSize: "0.85rem", margin: 0, color: "var(--text-primary)", display: "flex", alignItems: "flex-start", gap: "0.4rem" }}>
                      <AlertCircle size={16} style={{ flexShrink: 0, marginTop: "0.1rem" }} />
                      <span>{pullError}</span>
                    </p>
                  </div>
                )}
                <button type="button" className="btn btn-primary" onClick={startPullModel}>
                  <Download size={16} style={{ marginRight: "0.5rem" }} /> Download {model}
                </button>
                <div style={{ marginTop: "1rem", background: "rgba(245, 158, 11, 0.05)", borderLeft: "4px solid var(--color-warning)", padding: "0.75rem", borderRadius: "4px" }}>
                  <p style={{ fontSize: "0.85rem", margin: 0, color: "var(--text-primary)" }}>
                    <strong>Warning:</strong> You can skip this download, but you will be unable to run a Daily Scan until the model is downloaded later.
                  </p>
                </div>
              </div>
            )}

            {(pulling || pullComplete) && (
              <div style={{ marginTop: "1rem" }}>
                <div style={{ display: "flex", justifyContent: "space-between", fontSize: "0.85rem", marginBottom: "0.5rem" }}>
                  <span>{pullProgress}</span>
                  {pullPercent !== null && <span>{pullPercent.toFixed(1)}%</span>}
                </div>
                <div className="progress-bar-container" style={{ background: "var(--border-color)", height: "8px", borderRadius: "4px" }}>
                  <div 
                    style={{ 
                      height: "100%", 
                      background: "var(--accent-primary)",
                      width: `${pullPercent || 0}%`,
                      transition: "width 0.2s"
                    }} 
                  />
                </div>
                {pulling && (
                  <button type="button" className="btn btn-secondary btn-sm" onClick={cancelPullModel} style={{ marginTop: "1rem" }}>
                    Cancel Download
                  </button>
                )}
                {pullComplete && (
                  <div style={{ marginTop: "1rem", color: "var(--color-success)", display: "flex", alignItems: "center" }}>
                    <CheckCircle size={16} style={{ marginRight: "0.5rem" }} /> Model pulled successfully.
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {/* STEP 4: DEFAULTS */}
        {step === 4 && (
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <div>
              <label htmlFor="onboarding-publish-path" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publish Path</label>
              <input id="onboarding-publish-path" type="text" value={publishPath} onChange={e => setPublishPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
              <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>Where your static sites will be compiled.</p>
            </div>
            <div>
              <label htmlFor="onboarding-backup-path" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Backup Path</label>
              <input id="onboarding-backup-path" type="text" value={backupPath} onChange={e => setBackupPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
              <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>Where database backups are saved.</p>
            </div>
          </div>
        )}

        {/* STEP 5: DONE */}
        {step === 5 && (
          <div style={{ textAlign: "center", padding: "3rem 0" }}>
            <div style={{ display: "inline-flex", background: "rgba(16, 185, 129, 0.1)", padding: "1rem", borderRadius: "50%", marginBottom: "1rem" }}>
              <CheckCircle size={48} color="var(--color-success)" />
            </div>
            <h3 style={{ color: "var(--color-success)", marginBottom: "0.5rem" }}>You're ready.</h3>
            <p className="help-text">All setup steps are complete. Click finish to enter the workspace.</p>
            
            <div style={{ marginTop: "2rem", borderTop: "1px solid var(--border-color)", paddingTop: "1rem", textAlign: "left" }}>
              <h4 style={{ fontSize: "1.05rem", marginBottom: "0.5rem" }}>What's next?</h4>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)" }}>
                Once you finish onboarding, you will enter the workspace. You can:
              </p>
              <ul style={{ fontSize: "0.85rem", color: "var(--text-secondary)", paddingLeft: "1.2rem", marginTop: "0.25rem", display: "flex", flexDirection: "column", gap: "0.25rem" }}>
                <li>Configure your <strong>first source</strong> under the Sources tab.</li>
                <li>Write or import your first draft articles.</li>
                <li>Run a daily scan to aggregate recent signals.</li>
              </ul>
            </div>
          </div>
        )}
      </div>

      <div className="flex-between onboarding-actions">
        <button type="button" className="btn btn-secondary" onClick={handleBack} disabled={step === 1}>
          Back
        </button>
        
        <div style={{ display: "flex", gap: "1rem" }}>
          {(step === 2 || step === 3) && (
            <button type="button" className="btn btn-secondary" onClick={() => {
              if (step === 2) {
                setSkipConfirm({
                  title: "Skip AI setup?",
                  message: "You won't be able to use AI features until you complete setup from Settings.",
                  confirmLabel: "Skip setup",
                  onConfirm: () => setStep(4),
                });
              } else if (step === 3) {
                setSkipConfirm({
                  title: "Skip the model download?",
                  message: "You won't be able to use AI features until you download a model from Settings.",
                  confirmLabel: "Skip download",
                  onConfirm: async () => {
                    await cancelPullModel();
                    setStep(4);
                  },
                });
              }
            }}>
              Skip for now
            </button>
          )}
          
          <button type="button" className="btn btn-primary" onClick={handleNext} id="btn-wizard-next" disabled={runtimeInstalling || pulling}>
            {step === steps.length ? "Finish Onboarding" : "Next"}
            <ChevronRight size={16} style={{ marginLeft: "0.5rem" }} />
          </button>
        </div>
      </div>

      {skipConfirm && (
        <ConfirmModal
          title={skipConfirm.title}
          message={skipConfirm.message}
          confirmLabel={skipConfirm.confirmLabel}
          onConfirm={async () => {
            const action = skipConfirm.onConfirm;
            setSkipConfirm(null);
            await action();
          }}
          onCancel={() => setSkipConfirm(null)}
        />
      )}
    </div>
  );
};
