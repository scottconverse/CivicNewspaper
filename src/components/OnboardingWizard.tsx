// STEPS DEFINED HERE ARE DOCUMENTED IN docs/user_manual.md PART 1. Update both together.
import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { documentDir, appDataDir, join } from "@tauri-apps/api/path";
import { ChevronRight, Download, CheckCircle, RefreshCcw, AlertCircle } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";

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
  
  // Step 1 State
  const [pubName, setPubName] = useState("");
  const [editorName, setEditorName] = useState("");
  const [city, setCity] = useState("");
  const [state, setState] = useState("");

  // Step 2 State
  const [health, setHealth] = useState<OllamaState | null>(null);
  const [checkingHealth, setCheckingHealth] = useState(false);
  const [sysRam, setSysRam] = useState<number>(systemRam || 0);
  const [healthTimeout, setHealthTimeout] = useState(false);
  const [retryCount, setRetryCount] = useState(0);
  const [exportStatus, setExportStatus] = useState("");

  // Step 3 State
  const [pullProgress, setPullProgress] = useState<string>("");
  const [pullPercent, setPullPercent] = useState<number | null>(null);
  const [pulling, setPulling] = useState(false);
  const [pullComplete, setPullComplete] = useState(false);

  // Step 4 State
  const [publishPath, setPublishPath] = useState("");
  const [backupPath, setBackupPath] = useState("");

  // Init error surfacing state (WU-Nit-1)
  const [initError, setInitError] = useState<string | null>(null);

  const steps = [
    { title: "Identity", desc: "Define your local news outlet name and mission." },
    { title: "AI Service Setup", desc: "Check connection with the local Ollama LLM endpoint." },
    { title: "Download AI Model", desc: "Downloading AI model. This is a one-time setup." },
    { title: "Defaults", desc: "Configure publication directories and backup database paths." },
    { title: "Done", desc: "Onboarding completed. Ready to inspect local stories." }
  ];

  // Initialize paths and ram
  useEffect(() => {
    async function init() {
      try {
        const docDir = await documentDir();
        const pPath = await join(docDir, "CivicNews", "sites", "default");
        setPublishPath(pPath);

        const appData = await appDataDir();
        const bPath = await join(appData, "backups");
        setBackupPath(bPath);

        const ram = systemRam || await invoke<number>("get_system_ram");
        setSysRam(ram);

        const selected = await invoke<string | null>("get_setting", { key: "model.selected" });
        if (selected) {
          setModel(selected);
        } else {
          const fallback = ram >= 12 ? 'gemma2:9b' : ram >= 8 ? 'llama3:8b' : 'phi3:mini';
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
          const result = await invoke<OllamaState>("ollama_health");
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
        await invoke('export_diagnostics', { path });
        setExportStatus("Export successful!");
        setTimeout(() => setExportStatus(""), 3000);
      }
    } catch (e: any) {
      setExportStatus(`Export failed: ${e?.message || String(e)}`);
    }
  };

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
    setPulling(true);
    setPullProgress("Starting pull...");
    setPullPercent(0);
    setPullComplete(false);

    const modelToPull = model;

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

      await invoke("pull_ollama_model", { modelId: modelToPull });
      await invoke("set_setting", { key: "model.selected", value: modelToPull });
    } catch (e) {
      console.error(e);
      setPullProgress("Error pulling model.");
      setPulling(false);
    }
  };

  const cancelPullModel = async () => {
    try {
      await invoke("cancel_ollama_pull", { model: model });
      setPulling(false);
      setPullComplete(false);
      setPullProgress("Pull cancelled.");
    } catch (e) {
      console.error(e);
    }
  };

  const handleNext = async () => {
    if (step === 1) {
      // Persist identity settings
      await invoke("set_setting", { key: "identity.newsroom_name", value: pubName });
      await invoke("set_setting", { key: "identity.editor_name", value: editorName });
      await invoke("set_setting", { key: "identity.city", value: city });
      await invoke("set_setting", { key: "identity.state", value: state });
      
      setStep(2);
    } else if (step === 2) {
      if (health && health.reachable && health.models.includes(model)) {
        // Model is already installed, skip Step 3 and go directly to Step 4
        await invoke("set_setting", { key: "model.selected", value: model });
        setStep(4);
      } else {
        setStep(3);
      }
    } else if (step === 3) {
      await invoke("set_setting", { key: "model.selected", value: model });
      setStep(4);
    } else if (step === 4) {
      // Persist defaults
      await invoke("set_setting", { key: "paths.publish", value: publishPath });
      await invoke("set_setting", { key: "paths.backup", value: backupPath });
      
      setStep(5);
    } else if (step === 5) {
      await invoke("set_onboarding_complete", { value: true });
      onComplete();
    }
  };

  const handleBack = () => {
    if (step > 1) setStep(prev => prev - 1);
  };

  return (
    <div className="wizard-container card" id="onboarding-wizard">
      {initError && (
        <div style={{ background: "rgba(239, 68, 68, 0.05)", borderLeft: "4px solid var(--color-danger)", padding: "0.75rem", borderRadius: "4px", marginBottom: "1rem", display: "flex", alignItems: "center", gap: "0.5rem" }}>
          <AlertCircle size={16} style={{ color: "var(--color-danger)" }} />
          <span style={{ fontSize: "0.85rem", color: "var(--color-danger)" }}>Initialization Error: {initError}</span>
        </div>
      )}

      <div className="flex-between">
        <h2>Setup Wizard</h2>
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

      <div style={{ marginTop: "2rem", minHeight: "300px" }}>
        <h3>{steps[step - 1].title}</h3>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          {steps[step - 1].desc}
        </p>

        {/* STEP 1: IDENTITY */}
        {step === 1 && (
          <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publication Name</label>
              <input type="text" placeholder="e.g. The Brighton Gazette" value={pubName} onChange={e => setPubName(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
            </div>
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Editor Name</label>
              <input type="text" placeholder="e.g. Jane Doe" value={editorName} onChange={e => setEditorName(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
            </div>
            <div style={{ display: "flex", gap: "1rem" }}>
              <div style={{ flex: 1 }}>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>City</label>
                <input type="text" placeholder="Brighton" value={city} onChange={e => setCity(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
              </div>
              <div style={{ flex: 1 }}>
                <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>State</label>
                <input type="text" placeholder="CO" value={state} onChange={e => setState(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
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
                <p style={{ fontSize: "0.95rem" }}>Checking Ollama sidecar initialization status...</p>
              </div>
            ) : (
              <>
                {health && (
                  <div className="flex-between" style={{ marginBottom: "1rem" }}>
                    <div>
                      <strong>Local Ollama Connection</strong>
                      <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>Local Ram: {sysRam} GB</p>
                    </div>
                    <span className={`status-dot ${health.reachable ? "online" : "offline"}`} />
                  </div>
                )}

                {/* Timeout State (WU-2) */}
                {healthTimeout && (
                  <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-danger)" }}>Couldn't reach the AI service</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>
                      The Ollama sidecar took too long to respond. This might be due to system resources or path permissions.
                    </p>
                    {exportStatus && (
                      <p style={{ fontSize: "0.85rem", color: "var(--accent-primary)", marginBottom: "0.5rem" }}>{exportStatus}</p>
                    )}
                    <div style={{ display: "flex", gap: "1rem" }}>
                      <button className="btn btn-primary btn-sm" onClick={() => { setHealthTimeout(false); setCheckingHealth(true); setRetryCount(c => c + 1); }}>
                        <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} /> Retry
                      </button>
                      <button className="btn btn-secondary btn-sm" onClick={handleExportDiagnostics}>
                        Open diagnostic log
                      </button>
                    </div>
                  </div>
                )}

                {!healthTimeout && health && !health.reachable && (
                  <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-danger)" }}>Bundled Ollama Sidecar Starting</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>The application includes a bundled Ollama sidecar to run AI features completely offline. It may take a moment to initialize.</p>
                    <div style={{ display: "flex", gap: "1rem" }}>
                      <button className="btn btn-secondary" onClick={() => setRetryCount(c => c + 1)} disabled={checkingHealth}>
                        <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} />
                        {checkingHealth ? "Checking..." : "Check Initialization Status"}
                      </button>
                    </div>
                  </div>
                )}

                {/* Reachable, no models (WU-7 action hint) */}
                {!healthTimeout && health && health.reachable && health.models.length === 0 && (
                  <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-success)" }}>Ollama is ready. Pull a recommended model?</h4>
                    <p style={{ fontSize: "0.9rem" }}>Based on your {sysRam}GB of RAM, we recommend: <strong>{model}</strong></p>
                    <button 
                      className="btn btn-primary" 
                      onClick={handleNext}
                      style={{ marginTop: "1rem" }}
                    >
                      Continue
                    </button>
                  </div>
                )}

                {/* Reachable with models (WU-4 use existing model) */}
                {!healthTimeout && health && health.reachable && health.models.length > 0 && (
                  <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                    <h4 style={{ color: "var(--color-success)" }}>Use an existing model?</h4>
                    <p style={{ fontSize: "0.9rem", marginBottom: "0.5rem" }}>
                      We detected the following installed models in your local Ollama. Select one to use it and skip downloading:
                    </p>
                    {/* installedModels from api/tags are selectable: Use existing model if you already have it. */}
                    <select 
                      value={model} 
                      onChange={e => setModel(e.target.value)}
                      style={{ width: "100%", padding: "0.5rem", borderRadius: "4px", border: "1px solid var(--border-color)", background: "var(--bg-primary)", color: "var(--text-primary)" }}
                    >
                      {health.models.map(m => <option key={m} value={m}>{m}</option>)}
                      <option value="">-- Or pull a recommended model --</option>
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
            <div style={{ background: "rgba(0,0,0,0.02)", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
              <strong>AI Model: {model} (Recommended)</strong>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>
                Downloading AI model. This is a one-time setup.
              </p>
            </div>
            
            {!pulling && !pullComplete && (
              <div>
                <button className="btn btn-primary" onClick={startPullModel}>
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
                      background: "var(--color-primary)", 
                      width: `${pullPercent || 0}%`,
                      transition: "width 0.2s"
                    }} 
                  />
                </div>
                {pulling && (
                  <button className="btn btn-secondary btn-sm" onClick={cancelPullModel} style={{ marginTop: "1rem" }}>
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
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Publish Path</label>
              <input type="text" value={publishPath} onChange={e => setPublishPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
              <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>Where your static sites will be compiled.</p>
            </div>
            <div>
              <label style={{ fontWeight: 600, display: "block", marginBottom: "0.25rem" }}>Backup Path</label>
              <input type="text" value={backupPath} onChange={e => setBackupPath(e.target.value)} style={{ width: "100%", padding: "0.5rem" }} />
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

      <div className="flex-between" style={{ marginTop: "2rem", paddingTop: "1rem", borderTop: "1px solid var(--border-color)" }}>
        <button className="btn btn-secondary" onClick={handleBack} disabled={step === 1}>
          Back
        </button>
        
        <div style={{ display: "flex", gap: "1rem" }}>
          {(step === 2 || step === 3) && (
            <button className="btn btn-secondary" onClick={async () => {
              if (step === 2) {
                const confirmSkip = window.confirm("Skip Ollama setup? You won't be able to use AI features until you complete setup from Settings.");
                if (!confirmSkip) return;
                setStep(4);
              } else if (step === 3) {
                const confirmSkip = window.confirm("Skip the model download? You won't be able to use AI features until you download a model from Settings.");
                if (!confirmSkip) return;
                // Skip: setStep(4) cancel_ollama_pull|cancelPull
                await cancelPullModel();
                setStep(4);
              }
            }}>
              Skip for now
            </button>
          )}
          
          <button className="btn btn-primary" onClick={handleNext} id="btn-wizard-next">
            {step === steps.length ? "Finish Onboarding" : "Next"}
            <ChevronRight size={16} style={{ marginLeft: "0.5rem" }} />
          </button>
        </div>
      </div>
    </div>
  );
};
