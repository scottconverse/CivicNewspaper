// STEPS DEFINED HERE ARE DOCUMENTED IN docs/user_manual.md PART 1. Update both together.
import React, { useState, useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import { join } from "@tauri-apps/api/path";
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
  revealMainWindowForSetup,
  getResolvedAppDataDir,
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
// medium/high tiers too - not just below the low-RAM floor.
const SLOW_CPU_CAUTION =
  "Heads up: the AI model runs on your CPU, so generating a draft or daily scan can take a minute or more - this is normal.";
const MODEL_DOWNLOAD_RESCUE_MS = import.meta.env.MODE === "test" ? 50 : 6000;
const MODEL_READY_RESCUE_MS = import.meta.env.MODE === "test" ? 50 : 5000;
const FINAL_SETUP_RESCUE_MS = import.meta.env.MODE === "test" ? 50 : 3000;
const RUNTIME_INSTALL_RESCUE_MS = import.meta.env.MODE === "test" ? 50 : 4000;

// Approximate one-time download sizes, sourced from models.json so the wizard
// can disclose the size up front (UX-C1) instead of springing a multi-GB
// download on the user.
const modelSizes: Record<string, string> = (modelsConfig as any).sizes || {};
function downloadSizeFor(modelTag: string): string {
  return modelSizes[modelTag] || "a few GB";
}

function sanitizeStateCode(value: string): string {
  return value.replace(/[^A-Za-z]/g, "").slice(0, 2).toUpperCase();
}

function modelInstalled(selected: string, installed: string[]): boolean {
  const want = selected.includes(":") ? selected : `${selected}:latest`;
  return installed.some((m) => {
    const have = m.includes(":") ? m : `${m}:latest`;
    return have === want;
  });
}

async function markAiSetupSkipped(): Promise<void> {
  await setSetting("ai.setup_skipped", "true");
}

async function clearAiSetupSkipped(): Promise<void> {
  await setSetting("ai.setup_skipped", "false");
}

const starterProfiles = [
  {
    label: "Longmont",
    pubName: "Longmont Civic Desk",
    editorName: "Local Editor",
    organizationType: "single_person",
    city: "Longmont",
    state: "CO",
  },
  {
    label: "Brighton",
    pubName: "Brighton Civic Desk",
    editorName: "Local Editor",
    organizationType: "single_person",
    city: "Brighton",
    state: "CO",
  },
  {
    label: "Denver",
    pubName: "Denver Civic Desk",
    editorName: "Local Editor",
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
  const modelDownloadRescueAttemptedRef = useRef(false);
  const runtimeInstallRescueAttemptedRef = useRef(false);
  const identityAdvanceInFlightRef = useRef(false);
  const pubNameInputRef = useRef<HTMLInputElement | null>(null);
  const editorNameInputRef = useRef<HTMLInputElement | null>(null);
  const organizationTypeSelectRef = useRef<HTMLSelectElement | null>(null);
  const cityInputRef = useRef<HTMLInputElement | null>(null);
  const stateInputRef = useRef<HTMLInputElement | null>(null);
  const primaryActionRef = useRef<HTMLButtonElement | null>(null);
  const stepTwoSkipButtonRef = useRef<HTMLButtonElement | null>(null);
  const modelDownloadButtonRef = useRef<HTMLButtonElement | null>(null);

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
  const [setupRecoveryActive, setSetupRecoveryActive] = useState(false);

  const steps = [
    { title: "Identity", desc: "Define your local news outlet name and mission." },
    { title: "AI Service Setup", desc: "Check the connection to the local AI service that runs on your computer." },
    { title: "Download AI Model", desc: "Download the local AI model. One-time setup - needs an internet connection." },
    { title: "Defaults", desc: "Configure publication directories and backup database paths." },
    { title: "Done", desc: "Onboarding completed. Ready to inspect local stories." }
  ];

  const saveSetting = async (key: string, value: string) => {
    if (!isTauri()) return;
    await setSetting(key, value);
  };

  const ensureDefaultPathSettings = async (defaultPublishPath: string, defaultBackupPath: string) => {
    if (!isTauri()) return;
    const savedPublishPath = await getSetting("paths.publish");
    const savedBackupPath = await getSetting("paths.backup");
    if (!savedPublishPath) {
      await saveSetting("paths.publish", defaultPublishPath);
    }
    if (!savedBackupPath) {
      await saveSetting("paths.backup", defaultBackupPath);
    }
  };

  const saveOnboardingDone = async () => {
    if (!isTauri()) return;
    await setOnboardingComplete(true);
  };

  const revealSetupWindow = async () => {
    if (!isTauri()) return;
    try {
      await revealMainWindowForSetup();
    } catch {
      /* non-fatal - setup can continue if the shell reveal command is unavailable */
    }
  };

  const goToStep = async (nextStep: number) => {
    await saveSetting("onboarding.step", String(nextStep));
    setStep(nextStep);
    void revealSetupWindow();
  };

  const applyIdentityValues = (values: {
    pubName: string;
    editorName: string;
    organizationType: string;
    city: string;
    state: string;
  }) => {
    if (pubNameInputRef.current) pubNameInputRef.current.value = values.pubName;
    if (editorNameInputRef.current) editorNameInputRef.current.value = values.editorName;
    if (organizationTypeSelectRef.current) organizationTypeSelectRef.current.value = values.organizationType;
    if (cityInputRef.current) cityInputRef.current.value = values.city;
    if (stateInputRef.current) stateInputRef.current.value = values.state;
    setPubName(values.pubName);
    setEditorName(values.editorName);
    setOrganizationType(values.organizationType);
    setCity(values.city);
    setState(values.state);
  };

  const markIdentityInteraction = () => {
    setSetupNotice(null);
  };

  const currentIdentityValues = () => ({
    pubName: pubNameInputRef.current?.value ?? pubName,
    editorName: editorNameInputRef.current?.value ?? editorName,
    organizationType: organizationTypeSelectRef.current?.value ?? organizationType,
    city: cityInputRef.current?.value ?? city,
    state: stateInputRef.current?.value ?? state,
  });

  const persistIdentity = async (identity = currentIdentityValues()) => {
    const normalizedIdentity = {
      ...identity,
      pubName: identity.pubName.trim(),
      editorName: identity.editorName.trim(),
      city: identity.city.trim(),
      state: sanitizeStateCode(identity.state),
    };

    setPubName(normalizedIdentity.pubName);
    setEditorName(normalizedIdentity.editorName);
    setOrganizationType(normalizedIdentity.organizationType);
    setCity(normalizedIdentity.city);
    setState(normalizedIdentity.state);

    await saveSetting("identity.newsroom_name", normalizedIdentity.pubName);
    await saveSetting("identity.editor_name", normalizedIdentity.editorName);
    await saveSetting("identity.organization_type", normalizedIdentity.organizationType);
    await saveSetting("identity.city", normalizedIdentity.city);
    await saveSetting("identity.state", normalizedIdentity.state);

    try {
      const profile = await getCommunityProfile();
      await saveCommunityProfile({
        ...profile,
        site_title: normalizedIdentity.pubName || profile.site_title,
        organization_type: normalizedIdentity.organizationType,
        city: normalizedIdentity.city || profile.city,
        state: normalizedIdentity.state || profile.state,
      });
    } catch {
      /* non-fatal - identity settings above are still saved */
    }
  };

  const advanceIdentityStep = async (identity = currentIdentityValues()) => {
    if (identityAdvanceInFlightRef.current || step !== 1) return;
    identityAdvanceInFlightRef.current = true;
    try {
      setInitError(null);
      setSetupNotice(null);
      await persistIdentity(identity);
      await goToStep(2);
    } catch (e) {
      console.error(e);
      setInitError(toUserMessage(e));
      identityAdvanceInFlightRef.current = false;
    }
  };

  const prePersistIdentityOnPress = () => {
    if (step !== 1) return;
    const identity = currentIdentityValues();
    void (async () => {
      try {
        await persistIdentity(identity);
        await revealSetupWindow();
      } catch (e) {
        console.error(e);
        setInitError(toUserMessage(e));
      }
    })();
  };

  // Initialize paths and ram
  useEffect(() => {
    async function init() {
      try {
        if (isTauri()) {
          const appData = await getResolvedAppDataDir();
          const pPath = await join(appData, "sites", "default");
          setPublishPath(pPath);

          const bPath = await join(appData, "backups");
          setBackupPath(bPath);
          await ensureDefaultPathSettings(pPath, bPath);
        } else {
          setPublishPath("C:\\CivicNews\\sites\\default");
          setBackupPath("C:\\CivicNews\\backups");
        }

        const ram = systemRam || await getSystemRam();
        setSysRam(ram);
        const fallback = ram >= 16 ? modelsConfig.high : ram >= 8 ? modelsConfig.medium : modelsConfig.low;

        const selected = isTauri() ? await getSetting("model.selected") : null;
        if (selected) {
          setModel(selected);
        } else {
          setModel(fallback);
        }

        if (isTauri()) {
          const savedPubName = await getSetting("identity.newsroom_name");
          const savedEditorName = await getSetting("identity.editor_name");
          const savedOrganizationType = await getSetting("identity.organization_type");
          const savedCity = await getSetting("identity.city");
          const savedState = await getSetting("identity.state");
          if (savedPubName || savedEditorName || savedCity || savedState) {
            applyIdentityValues({
              pubName: savedPubName || "",
              editorName: savedEditorName || "",
              organizationType: savedOrganizationType || "single_person",
              city: savedCity || "",
              state: savedState || "",
            });
          }

          if (initialStep === undefined) {
            const savedStep = await getSetting("onboarding.step");
            const restoredStep = Number.parseInt(savedStep || "", 10);
            if (Number.isInteger(restoredStep) && restoredStep >= 2 && restoredStep <= steps.length) {
              setStep(restoredStep);
              void revealSetupWindow();
            }
          }
        }

        if (ollamaOnline !== undefined) {
          setHealth({
            reachable: ollamaOnline || !isTauri(),
            models: isTauri() ? [] : [fallback],
            version: null,
          });
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
      if (!isTauri()) {
        setHealth({
          reachable: true,
          models: [model || modelsConfig.high],
          version: "browser-preview",
        });
        setCheckingHealth(false);
        setHealthTimeout(false);
        return;
      }
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
        } catch {
          setHealth({ reachable: false, models: [], version: null });
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
      await clearAiSetupSkipped();
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
      !setupRecoveryActive ||
      runtimeInstallRescueAttemptedRef.current ||
      step !== 2 ||
      checkingHealth ||
      runtimeInstalling ||
      !health ||
      health.reachable
    ) {
      return;
    }

    const timer = window.setTimeout(() => {
      if (
        !setupRecoveryActive ||
        runtimeInstallRescueAttemptedRef.current ||
        step !== 2 ||
        runtimeInstalling ||
        !health ||
        health.reachable
      ) {
        return;
      }

      runtimeInstallRescueAttemptedRef.current = true;
      setSetupNotice("Local AI setup is taking a little while. The Civic Desk is continuing the runtime installation automatically.");
      void installRuntime().then((ready) => {
        if (ready) {
          setSetupNotice("The local AI runtime is ready. The Civic Desk is starting the recommended model download automatically.");
          setAutoStartPull(true);
          void goToStep(3);
        }
      });
    }, RUNTIME_INSTALL_RESCUE_MS);

    return () => window.clearTimeout(timer);
  }, [setupRecoveryActive, step, checkingHealth, runtimeInstalling, health?.reachable]);

  useEffect(() => {
    if (
      step !== 2 ||
      modelDownloadRescueAttemptedRef.current ||
      checkingHealth ||
      runtimeInstalling ||
      !health?.reachable ||
      health.models.length > 0
    ) {
      return;
    }

    const rescueTimer = window.setTimeout(() => {
      if (
        modelDownloadRescueAttemptedRef.current ||
        step !== 2 ||
        !health?.reachable ||
        health.models.length > 0
      ) {
        return;
      }

      modelDownloadRescueAttemptedRef.current = true;
      setSetupRecoveryActive(true);
      setSetupNotice("The model download is taking a little while, so The Civic Desk is continuing it automatically.");
      setAutoStartPull(true);
      void goToStep(3);
    }, MODEL_DOWNLOAD_RESCUE_MS);

    return () => window.clearTimeout(rescueTimer);
  }, [step, checkingHealth, runtimeInstalling, health?.reachable, health?.models?.length]);

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
            void clearAiSetupSkipped();
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
        void saveSetting("model.selected", modelToPull);
        void clearAiSetupSkipped();
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

  const stepTwoNeedsRuntimeInstall = step === 2 && Boolean(health && !health.reachable);

  useEffect(() => {
    if (step !== 3 || !setupRecoveryActive || pullError) {
      return;
    }

    let cancelled = false;
    const finishRecoveredSetup = async (latestHealth: OllamaState | null) => {
      setPullProgress("Download complete!");
      setPullPercent(100);
      setPullComplete(true);
      setPulling(false);
      await saveSetting("model.selected", model);
      if (cancelled) return;
      if (latestHealth) {
        setHealth(latestHealth);
      }
      setSetupNotice("The recommended model is installed. Setup is continuing automatically.");
      await goToStep(4);
    };

    const checkModelReady = async () => {
      try {
        if (pullComplete) {
          await finishRecoveredSetup(null);
          return;
        }

        const latestHealth = await ollamaHealth();
        if (cancelled) return;
        setHealth(latestHealth);
        if (latestHealth.reachable && modelInstalled(model, latestHealth.models)) {
          await finishRecoveredSetup(latestHealth);
        }
      } catch (e) {
        console.error(e);
      }
    };

    const interval = window.setInterval(() => void checkModelReady(), MODEL_READY_RESCUE_MS);
    void checkModelReady();
    return () => {
      cancelled = true;
      window.clearInterval(interval);
    };
  }, [step, setupRecoveryActive, pullComplete, pullError, model]);

  useEffect(() => {
    if (!setupRecoveryActive || step !== 4) {
      return;
    }

    const timer = window.setTimeout(() => {
      void (async () => {
        try {
          await saveSetting("paths.publish", publishPath);
          await saveSetting("paths.backup", backupPath);
          setSetupNotice("Default folders were saved. Setup is continuing automatically.");
          await goToStep(5);
        } catch (e) {
          console.error(e);
          setInitError(toUserMessage(e));
        }
      })();
    }, FINAL_SETUP_RESCUE_MS);

    return () => window.clearTimeout(timer);
  }, [setupRecoveryActive, step, publishPath, backupPath]);

  useEffect(() => {
    if (!setupRecoveryActive || step !== 5) {
      return;
    }

    const timer = window.setTimeout(() => {
      void (async () => {
        try {
          await persistIdentity();
          await saveOnboardingDone();
          await saveSetting("setup.recovered_input", "true");
          onComplete();
        } catch (e) {
          console.error(e);
          setInitError(toUserMessage(e));
        }
      })();
    }, FINAL_SETUP_RESCUE_MS);

    return () => window.clearTimeout(timer);
  }, [setupRecoveryActive, step, onComplete]);

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
        await advanceIdentityStep();
      } else if (step === 2) {
        if (health && health.reachable && modelInstalled(model, health.models)) {
          // Model is already installed, skip Step 3 and go directly to Step 4
          await saveSetting("model.selected", model);
          await clearAiSetupSkipped();
          await goToStep(4);
        } else if (!health?.reachable) {
          setSetupNotice("Install the local AI runtime or choose Skip for now before continuing.");
          setHealthTimeout(true);
          return;
        } else {
          setAutoStartPull(true);
          await goToStep(3);
        }
      } else if (step === 3) {
        const modelReady = pullComplete || Boolean(health && modelInstalled(model, health.models));
        if (!modelReady) {
          setAutoStartPull(true);
          return;
        }
        await saveSetting("model.selected", model);
        await clearAiSetupSkipped();
        await goToStep(4);
      } else if (step === 4) {
        // Persist defaults
        await saveSetting("paths.publish", publishPath);
        await saveSetting("paths.backup", backupPath);

        await goToStep(5);
      } else if (step === 5) {
        // Re-commit identity at the final boundary. Installed WebView recovery
        // can advance later setup steps without React input events, so the
        // values selected on step 1 must still be durable when setup finishes.
        await persistIdentity();
        await saveOnboardingDone();
        if (setupRecoveryActive) {
          await saveSetting("setup.recovered_input", "true");
        }
        onComplete();
      }
    } catch (e) {
      console.error(e);
      setInitError(toUserMessage(e));
    }
  };

  const requestSkipAiSetup = () => {
    if (step === 2) {
      setSkipConfirm({
        title: "Skip AI setup?",
        message: "AI drafting and AI-assisted review will stay limited until you complete setup from AI Model. Deterministic source checks can still run.",
        confirmLabel: "Continue without AI",
        onConfirm: async () => {
          await markAiSetupSkipped();
          await goToStep(4);
        },
      });
    } else if (step === 3) {
      setSkipConfirm({
        title: "Skip the model download?",
        message: "AI drafting and AI-assisted review will stay limited until you download a model from AI Model. Deterministic source checks can still run.",
        confirmLabel: "Continue without model",
        onConfirm: async () => {
          await markAiSetupSkipped();
          await cancelPullModel();
          await goToStep(4);
        },
      });
    }
  };

  const handleBack = () => {
    if (step > 1) void goToStep(step - 1);
  };

  useEffect(() => {
    if (step !== 2) return;
    window.setTimeout(() => {
      if (stepTwoNeedsRuntimeInstall) {
        stepTwoSkipButtonRef.current?.focus();
      } else {
        primaryActionRef.current?.focus();
      }
    }, 0);
  }, [step, stepTwoNeedsRuntimeInstall]);

  useEffect(() => {
    if (step === 1) {
      identityAdvanceInFlightRef.current = false;
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
      const shouldContinue = params.get("continueSetup") === "1";
      const profile = starterProfiles.find(item => item.label.toLowerCase() === starter?.toLowerCase());
      if (!profile && !shouldContinue) return;

      window.history.replaceState(null, "", window.location.pathname);

      const identity = profile ?? currentIdentityValues();
      if (profile) {
        applyIdentityValues(profile);
      }

      if (shouldContinue) {
        void (async () => {
          try {
            setInitError(null);
            setSetupNotice(null);
            await advanceIdentityStep(identity);
          } catch (e) {
            console.error(e);
            setInitError(toUserMessage(e));
          }
        })();
      }

    };

    handleStarterRoute();
    window.addEventListener("hashchange", handleStarterRoute);
    return () => window.removeEventListener("hashchange", handleStarterRoute);
  }, [step, pubName, editorName, organizationType, city, state]);

  useEffect(() => {
    if (!primaryActionRef.current) return;

    const button = primaryActionRef.current;
    const nativeIdentityAdvance = (event: Event) => {
      if (step !== 1 || button.disabled) return;
      event.preventDefault();
      event.stopPropagation();
      void advanceIdentityStep(currentIdentityValues());
    };
    const nativeIdentityEnterAdvance = (event: KeyboardEvent) => {
      if (step !== 1 || event.key !== "Enter" || button.disabled) return;
      event.preventDefault();
      event.stopPropagation();
      void advanceIdentityStep(currentIdentityValues());
    };

    // Some installed WebView paths have accepted native field edits while
    // dropping React's delegated identity-step handlers. Keep only the first
    // setup handoff native-owned; later setup steps use their normal React
    // handlers to avoid stale-step native listeners blocking Finish.
    button.addEventListener("pointerdown", nativeIdentityAdvance, { capture: true });
    button.addEventListener("mousedown", nativeIdentityAdvance, { capture: true });
    button.addEventListener("click", nativeIdentityAdvance, { capture: true });
    document.addEventListener("keydown", nativeIdentityEnterAdvance, { capture: true });
    return () => {
      button.removeEventListener("pointerdown", nativeIdentityAdvance, { capture: true });
      button.removeEventListener("mousedown", nativeIdentityAdvance, { capture: true });
      button.removeEventListener("click", nativeIdentityAdvance, { capture: true });
      document.removeEventListener("keydown", nativeIdentityEnterAdvance, { capture: true });
    };
  }, [step, pubName, editorName, organizationType, city, state, health, pulling, pullComplete, model, runtimeInstalling, publishPath, backupPath]);

  useEffect(() => {
    if (step !== 1) return;

    const handleNativeStarter = (event: Event) => {
      const target = event.target;
      if (!(target instanceof Element)) return;
      const button = target.closest<HTMLButtonElement>("[data-starter-profile]");
      if (!button) return;
      const profile = starterProfiles.find(item => item.label === button.dataset.starterProfile);
      if (!profile) return;
      event.preventDefault();
      event.stopPropagation();
      markIdentityInteraction();
      applyIdentityValues(profile);
      void advanceIdentityStep(profile);
    };

    document.addEventListener("pointerdown", handleNativeStarter, { capture: true });
    document.addEventListener("mousedown", handleNativeStarter, { capture: true });
    document.addEventListener("click", handleNativeStarter, { capture: true });
    return () => {
      document.removeEventListener("pointerdown", handleNativeStarter, { capture: true });
      document.removeEventListener("mousedown", handleNativeStarter, { capture: true });
      document.removeEventListener("click", handleNativeStarter, { capture: true });
    };
  }, [step]);

  useEffect(() => {
    if (!modelDownloadButtonRef.current) return;

    const button = modelDownloadButtonRef.current;
    const nativeStartPull = (event: Event) => {
      if (button.disabled) return;
      event.preventDefault();
      event.stopPropagation();
      void startPullModel();
    };

    button.addEventListener("click", nativeStartPull, { capture: true });
    return () => {
      button.removeEventListener("click", nativeStartPull, { capture: true });
    };
  }, [step, model, pulling, pullComplete]);

  useEffect(() => {
    if (step !== steps.length) return;
    window.setTimeout(() => primaryActionRef.current?.focus(), 0);
  }, [step]);

  return (
    <div className="wizard-container card" id="onboarding-wizard">
      {initError && (
        <div role="alert" aria-live="assertive" style={{ background: "rgba(239, 68, 68, 0.05)", borderLeft: "4px solid var(--color-error)", padding: "0.75rem", borderRadius: "4px", marginBottom: "1rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
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
        <h2>Workspace Setup</h2>
        <div style={{ display: "flex", alignItems: "center", gap: "0.75rem", flexWrap: "wrap", justifyContent: "flex-end" }}>
          <span style={{ fontWeight: 600, fontSize: "0.9rem", color: "var(--text-secondary)" }}>
            Step {step} of {steps.length}
          </span>
        </div>
      </div>

      <div
        className="progress-bar-container"
        role="progressbar"
        aria-label="Setup progress"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={(step / 5) * 100}
      >
        <div 
          className="progress-bar" 
          style={{ width: `${(step / steps.length) * 100}%` }}
          data-testid="progress-bar"
        />
      </div>

      <div className="onboarding-step-body">
        <h3>{steps[step - 1].title}</h3>
        <p className="help-text" style={{ marginBottom: "0.85rem" }}>
          {steps[step - 1].desc}
        </p>
        {stepTwoNeedsRuntimeInstall && (
          <div className="onboarding-notice onboarding-decision-note" id="onboarding-runtime-required-note" role="status">
            To continue, install the local AI runtime or choose <strong>Skip for now</strong>. Skipping is supported: source checks still work, and you can finish AI setup later from AI Model.
          </div>
        )}

        {/* STEP 1: IDENTITY */}
        {step === 1 && (
          <div className="onboarding-identity-fields">
            <div className="onboarding-starter-profiles" aria-label="Starter profiles">
              <span>Starter profiles</span>
              <div>
                {starterProfiles.map(profile => (
                  <button
                    key={profile.label}
                    type="button"
                    className="btn btn-secondary btn-sm"
                    data-starter-profile={profile.label}
                    onClick={() => {
                      markIdentityInteraction();
                      applyIdentityValues(profile);
                      void advanceIdentityStep(profile);
                    }}
                  >
                    {profile.label}
                  </button>
                ))}
              </div>
            </div>
            <p className="help-text" style={{ margin: 0 }}>
              These identity fields are optional during setup and can be edited later in Settings before you publish.
            </p>
            <div className="onboarding-field">
              <label htmlFor="onboarding-publication-name">Publication Name <span className="help-text">(optional)</span></label>
              <input
                id="onboarding-publication-name"
                aria-label="Publication Name"
                ref={pubNameInputRef}
                autoFocus
                type="text"
                placeholder="e.g. The Brighton Gazette"
                defaultValue={pubName}
                onInput={e => {
                  markIdentityInteraction();
                  setPubName(e.currentTarget.value);
                }}
                onChange={e => {
                  markIdentityInteraction();
                  setPubName(e.target.value);
                }}
              />
            </div>
            <div className="onboarding-field">
              <label htmlFor="onboarding-editor-name">Editor Name <span className="help-text">(optional)</span></label>
              <input
                id="onboarding-editor-name"
                aria-label="Editor Name"
                ref={editorNameInputRef}
                type="text"
                placeholder="e.g. Jane Doe"
                defaultValue={editorName}
                onInput={e => {
                  markIdentityInteraction();
                  setEditorName(e.currentTarget.value);
                }}
                onChange={e => {
                  markIdentityInteraction();
                  setEditorName(e.target.value);
                }}
              />
            </div>
            <div className="onboarding-field">
              <label htmlFor="onboarding-organization-type">Publisher Type</label>
              <select
                id="onboarding-organization-type"
                ref={organizationTypeSelectRef}
                defaultValue={organizationType}
                  onInput={e => {
                    markIdentityInteraction();
                    setOrganizationType(e.currentTarget.value);
                  }}
                  onChange={e => {
                    markIdentityInteraction();
                    setOrganizationType(e.target.value);
                  }}
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
                  onInput={e => {
                    markIdentityInteraction();
                    setCity(e.currentTarget.value);
                  }}
                  onChange={e => {
                    markIdentityInteraction();
                    setCity(e.target.value);
                  }}
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
                  maxLength={2}
                  autoCapitalize="characters"
                  onInput={e => {
                    markIdentityInteraction();
                    const nextState = sanitizeStateCode(e.currentTarget.value);
                    e.currentTarget.value = nextState;
                    setState(nextState);
                  }}
                  onChange={e => {
                    markIdentityInteraction();
                    const nextState = sanitizeStateCode(e.target.value);
                    e.target.value = nextState;
                    setState(nextState);
                  }}
                />
              </div>
            </div>
          </div>
        )}

        {/* STEP 2: AI SERVICE SETUP */}
        {step === 2 && (
          <div className="onboarding-ai-step">
            <div className="onboarding-step-two-actions" role="group" aria-label="AI setup choices">
              <div>
                <strong>Choose how to continue</strong>
                <p className="help-text">
                  You can install local AI now, or skip it and finish setup. Source checks still work without AI.
                </p>
              </div>
              <div className="onboarding-step-two-buttons">
                {health && !health.reachable && (
                  <button
                    type="button"
                    className="btn btn-primary"
                    onClick={() => void installRuntime()}
                    disabled={runtimeInstalling}
                  >
                    <Download size={14} style={{ marginRight: "0.5rem" }} />
                    {runtimeInstalling ? "Installing..." : "Install local AI runtime"}
                  </button>
                )}
                {health && health.reachable && health.models.length === 0 && (
                  <button
                    type="button"
                    className="btn btn-primary"
                    onClick={() => {
                      setAutoStartPull(true);
                      void goToStep(3);
                    }}
                    disabled={pulling}
                  >
                    <Download size={14} style={{ marginRight: "0.5rem" }} />
                    {pulling ? "Downloading..." : `Download ${model}`}
                  </button>
                )}
                {health && health.reachable && health.models.length > 0 && (
                  <button
                    type="button"
                    ref={primaryActionRef}
                    className="btn btn-primary"
                    onClick={() => void handleNext()}
                  >
                    Use selected model
                    <ChevronRight size={14} style={{ marginLeft: "0.5rem" }} />
                  </button>
                )}
                {health && !health.reachable && (
                  <button
                    type="button"
                    className="btn btn-secondary"
                    onClick={() => setRetryCount(c => c + 1)}
                    disabled={checkingHealth || runtimeInstalling}
                  >
                    <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} />
                    {checkingHealth ? "Checking..." : "Retry"}
                  </button>
                )}
                <button
                  type="button"
                  ref={stepTwoSkipButtonRef}
                  className="btn btn-secondary"
                  onClick={requestSkipAiSetup}
                >
                  Skip for now
                </button>
              </div>
            </div>

            <div className="card onboarding-ai-status-card">
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
                      <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>Local RAM: {sysRam} GB</p>
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
                        <div
                          role="status"
                          aria-live="polite"
                          style={{ display: "flex", justifyContent: "space-between", fontSize: "0.85rem", marginBottom: "0.35rem" }}
                        >
                          <span>{runtimeProgress}</span>
                          {runtimePercent !== null && <span>{runtimePercent.toFixed(1)}%</span>}
                        </div>
                        {runtimePercent !== null && (
                          <div
                            className="progress-bar-container"
                            role="progressbar"
                            aria-label="Local AI runtime install progress"
                            aria-valuemin={0}
                            aria-valuemax={100}
                            aria-valuenow={Math.round(runtimePercent)}
                            style={{ background: "var(--border-color)", height: "8px", borderRadius: "4px" }}
                          >
                            <div style={{ height: "100%", background: "var(--accent-primary)", width: `${runtimePercent}%`, transition: "width 0.2s" }} />
                          </div>
                        )}
                      </div>
                    )}
                    {exportStatus && (
                      <p style={{ fontSize: "0.85rem", color: "var(--accent-primary)", marginBottom: "0.5rem" }}>{exportStatus}</p>
                    )}
                    <details>
                      <summary>Need help from support?</summary>
                      <button type="button" className="btn btn-secondary btn-sm" onClick={handleExportDiagnostics} style={{ marginTop: "0.75rem" }}>
                        Save diagnostics file
                      </button>
                    </details>
                  </div>
                )}

                {!healthTimeout && health && !health.reachable && (
                  <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-error)" }}>Starting the local AI service</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>The Civic Desk includes a local AI service that runs on your computer. It may take a moment to start up. Once it's running, you'll download a model in the next step.</p>
                    <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginBottom: "1rem" }}>
                      On a clean machine, use the install button if the service does not become ready.
                    </p>
                    {runtimeError && (
                      <p style={{ fontSize: "0.85rem", color: "var(--color-error)", marginBottom: "0.5rem" }}>{runtimeError}</p>
                    )}
                    {runtimeProgress && (
                      <p role="status" aria-live="polite" style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginBottom: "0.5rem" }}>{runtimeProgress}</p>
                    )}
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
                  </div>
                )}

                {/* Reachable with models (WU-4 use existing model) */}
                {!healthTimeout && health && health.reachable && health.models.length > 0 && (
                  <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-success)" }}>Use an existing model?</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "0.5rem" }}>
                      We detected the following models already installed on your computer. Select one to use it and skip downloading:
                    </p>
                    <label htmlFor="onboarding-installed-model" className="sr-only">Installed model</label>
                    {/* installedModels from api/tags are selectable: Use existing model if you already have it. */}
                    <select
                      id="onboarding-installed-model"
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
          </div>
        )}

        {/* STEP 3: DOWNLOAD AI MODEL */}
        {step === 3 && (
          <div>
            <div style={{ background: "var(--accent-light)", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
              <strong>AI Model: {model} (Recommended)</strong>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>
                The Civic Desk will download this local AI model now - a one-time download of about {downloadSizeFor(model)} that needs an internet connection. This may take 10-60+ minutes. After this, the AI runs fully offline on your computer.
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
                <button type="button" ref={modelDownloadButtonRef} className="btn btn-primary" onClick={startPullModel} disabled={pulling}>
                  <Download size={16} style={{ marginRight: "0.5rem" }} /> Download {model}
                </button>
                <div style={{ marginTop: "1rem", background: "rgba(245, 158, 11, 0.05)", borderLeft: "4px solid var(--color-warning)", padding: "0.75rem", borderRadius: "4px" }}>
                  <p style={{ fontSize: "0.85rem", margin: 0, color: "var(--text-primary)" }}>
                    <strong>Warning:</strong> You can skip this download. Daily Scan can still run deterministic evidence checks, but AI drafting and AI-assisted lead review will stay limited until you download a model.
                  </p>
                </div>
              </div>
            )}

            {(pulling || pullComplete) && (
              <div style={{ marginTop: "1rem" }}>
                <div
                  role="status"
                  aria-live="polite"
                  style={{ display: "flex", justifyContent: "space-between", fontSize: "0.85rem", marginBottom: "0.5rem" }}
                >
                  <span>{pullProgress}</span>
                  {pullPercent !== null && <span>{pullPercent.toFixed(1)}%</span>}
                </div>
                <div
                  className="progress-bar-container"
                  role="progressbar"
                  aria-label="AI model download progress"
                  aria-valuemin={0}
                  aria-valuemax={100}
                  aria-valuenow={Math.round(pullPercent || 0)}
                  style={{ background: "var(--border-color)", height: "8px", borderRadius: "4px" }}
                >
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
            <div className="card" style={{ margin: 0 }}>
              <h3 style={{ marginTop: 0, marginBottom: "0.4rem" }}>Use the recommended local folders</h3>
              <p className="help-text" style={{ marginBottom: 0 }}>
                The Civic Desk will save finished publication files and backups in your Windows user profile.
                You can change these later in Settings.
              </p>
            </div>
            <details>
              <summary style={{ cursor: "pointer", fontWeight: 700 }}>Advanced: choose exact folders</summary>
              <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
                <div>
                  <label htmlFor="onboarding-publish-path" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publication folder</label>
                  <input id="onboarding-publish-path" type="text" value={publishPath} onChange={e => setPublishPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
                  <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>Finished HTML sites and ZIP review packages are saved here.</p>
                </div>
                <div>
                  <label htmlFor="onboarding-backup-path" style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Backup file</label>
                  <input id="onboarding-backup-path" type="text" value={backupPath} onChange={e => setBackupPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
                  <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>Database backup copies are saved here.</p>
                </div>
              </div>
            </details>
          </div>
        )}

        {/* STEP 5: DONE */}
        {step === 5 && (
          <div data-testid="onboarding-done-step" style={{ textAlign: "center", padding: "0.5rem 0 5.25rem" }}>
            <div style={{ display: "inline-flex", background: "rgba(16, 185, 129, 0.1)", padding: "0.625rem", borderRadius: "50%", marginBottom: "0.5rem" }}>
              <CheckCircle size={40} color="var(--color-success)" />
            </div>
            <h3 style={{ color: "var(--color-success)", marginBottom: "0.5rem" }}>Workspace ready.</h3>
            <p className="help-text">
              You can enter the workspace now. If you skipped local AI setup, drafting and AI-assisted review stay limited until you finish AI Model setup.
            </p>
            
            <div style={{ marginTop: "0.75rem", borderTop: "1px solid var(--border-color)", paddingTop: "0.75rem", textAlign: "left" }}>
              <h4 style={{ fontSize: "1rem", marginBottom: "0.375rem" }}>What's next?</h4>
              <ul style={{ fontSize: "0.86rem", color: "var(--text-secondary)", paddingLeft: "1.2rem", marginTop: 0, display: "flex", flexDirection: "column", gap: "0.3rem" }}>
                <li><strong>Starter sources may be added automatically</strong> when you enter the workspace.</li>
                <li><strong>Daily Scan is next</strong> once the starter-source step finishes, or you can add sources manually.</li>
                <li><strong>Review leads in Story Queue</strong>, then draft and edit in Workbench.</li>
                <li><strong>Finish AI Model setup</strong> later if you skipped it here.</li>
              </ul>
            </div>
          </div>
        )}
      </div>

      <div className="flex-between onboarding-actions">
        {step === 1 ? (
          <span className="help-text">Choose a starter profile or continue with the fields shown.</span>
        ) : (
          <button type="button" className="btn btn-secondary" onClick={handleBack} disabled={step === 1}>
            Back
          </button>
        )}

        <div style={{ display: "flex", gap: "1rem" }}>
          {step === 3 && (
            <button type="button" className="btn btn-secondary" onClick={requestSkipAiSetup}>
              Skip for now
            </button>
          )}

          <button
            type="button"
            ref={primaryActionRef}
            className="btn btn-primary"
            onPointerDown={step === 1 ? prePersistIdentityOnPress : undefined}
            onClick={handleNext}
            id="btn-wizard-next"
            aria-label={step === 1 ? "Continue setup Next" : undefined}
            disabled={runtimeInstalling || pulling || stepTwoNeedsRuntimeInstall}
            aria-describedby={stepTwoNeedsRuntimeInstall ? "onboarding-runtime-required-note" : undefined}
            title={stepTwoNeedsRuntimeInstall ? "Install the local AI runtime or choose Skip for now before continuing." : undefined}
          >
            {step === 1
              ? "Continue setup"
              : step === 3 && !pulling && !pullComplete && !(health && modelInstalled(model, health.models))
                ? "Start download"
                : step === steps.length
                  ? "Finish Onboarding"
                  : "Next"}
            <ChevronRight size={16} style={{ marginLeft: "0.5rem" }} />
          </button>
        </div>
      </div>

      {skipConfirm && (
        <ConfirmModal
          title={skipConfirm.title}
          message={skipConfirm.message}
          confirmLabel={skipConfirm.confirmLabel}
          cancelLabel="Keep setting up AI"
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
