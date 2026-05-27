import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { documentDir, appDataDir, join } from "@tauri-apps/api/path";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ChevronRight, Download, CheckCircle, RefreshCcw, Copy, Info } from "lucide-react";

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

export const OnboardingWizard: React.FC<OnboardingWizardProps> = ({ onComplete, initialStep }) => {
  const [step, setStep] = useState<number>(initialStep || 1);
  
  // Step 1 State
  const [pubName, setPubName] = useState("");
  const [editorName, setEditorName] = useState("");
  const [city, setCity] = useState("");
  const [state, setState] = useState("");

  // Step 2 State
  const [health, setHealth] = useState<OllamaState | null>(null);
  const [checkingHealth, setCheckingHealth] = useState(false);
  const [sysRam, setSysRam] = useState<number>(0);

  // Step 3 State
  const [pullProgress, setPullProgress] = useState<string>("");
  const [pullPercent, setPullPercent] = useState<number | null>(null);
  const [pulling, setPulling] = useState(false);
  const [pullComplete, setPullComplete] = useState(false);

  // Step 4 State
  const [publishPath, setPublishPath] = useState("");
  const [backupPath, setBackupPath] = useState("");

  // Step 5 State
  const [pairingToken, setPairingToken] = useState("");
  const [copied, setCopied] = useState(false);

  const steps = [
    { title: "Identity", desc: "Define your local news outlet name and mission." },
    { title: "Ollama Health", desc: "Check connection with the local Ollama LLM endpoint." },
    { title: "Download AI Model", desc: "Downloading AI model (5.4 GB). This is a one-time setup." },
    { title: "Defaults", desc: "Configure publication directories and backup database paths." },
    { title: "Browser Pairing", desc: "Establish authorization pairing with the browser extensions." },
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

        const ram = await invoke<number>("get_system_ram");
        setSysRam(ram);
      } catch (e) {
        console.error(e);
      }
    }
    init();
  }, []);

  const runHealthCheck = async () => {
    setCheckingHealth(true);
    try {
      const result = await invoke<OllamaState>("ollama_health");
      setHealth(result);
    } catch (e) {
      console.error(e);
      setHealth({ reachable: false, models: [], version: null });
    } finally {
      setCheckingHealth(false);
    }
  };

  const getRecommendedModel = () => {
    if (sysRam >= 12) return ["gemma2", "9b"].join(":");
    if (sysRam >= 8) return "llama3:8b";
    return "phi3:mini";
  };

  const startPullModel = async () => {
    setPulling(true);
    setPullProgress("Starting pull...");
    setPullPercent(0);

    const model = getRecommendedModel();

    try {
      await listen<{ model: string; status: string; completed?: number; total?: number }>(
        "ollama-pull-progress",
        (event) => {
          setPullProgress(event.payload.status);
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
        }
      );

      await invoke("pull_ollama_model", { modelId: model });
      await invoke("set_setting", { key: "model.selected", value: model });
    } catch (e) {
      console.error(e);
      setPullProgress("Error pulling model.");
      setPulling(false);
    }
  };

  const generateToken = async () => {
    try {
      const token = await invoke<string>("generate_pairing_pin", { label: "onboarding" });
      setPairingToken(token);
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
      
      // Setup step 2
      runHealthCheck();
      setStep(2);
    } else if (step === 2) {
      setStep(3);
    } else if (step === 3) {
      const recommendedModel = getRecommendedModel();
      await invoke("set_setting", { key: "model.selected", value: recommendedModel });
      setStep(4);
    } else if (step === 4) {
      // Persist defaults
      await invoke("set_setting", { key: "paths.publish", value: publishPath });
      await invoke("set_setting", { key: "paths.backup", value: backupPath });
      
      // Setup step 5
      generateToken();
      setStep(5);
    } else if (step === 5) {
      setStep(6);
    } else if (step === 6) {
      await invoke("set_onboarding_complete", { value: true });
      onComplete();
    }
  };

  const handleBack = () => {
    if (step > 1) setStep(prev => prev - 1);
  };

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(pairingToken);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="wizard-container card" id="onboarding-wizard">
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

        {/* STEP 2: OLLAMA HEALTH */}
        {step === 2 && health && (
          <div className="card">
            <div className="flex-between" style={{ marginBottom: "1rem" }}>
              <div>
                <strong>Local Ollama Connection</strong>
                <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>Local Ram: {sysRam} GB</p>
              </div>
              <span className={`status-dot ${health.reachable ? "online" : "offline"}`} />
            </div>

            {!health.reachable && (
              <div style={{ background: "rgba(239, 68, 68, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                <h4 style={{ color: "var(--color-danger)" }}>Ollama Not Detected</h4>
                <p style={{ fontSize: "0.9rem", marginBottom: "1rem" }}>Ollama must be running locally to power the AI features.</p>
                <div style={{ display: "flex", gap: "1rem" }}>
                  <button className="btn btn-primary" onClick={() => openUrl("https://ollama.ai")}>
                    Install Ollama
                  </button>
                  <button className="btn btn-secondary" onClick={runHealthCheck} disabled={checkingHealth}>
                    <RefreshCcw size={14} style={{ marginRight: "0.5rem" }} />
                    {checkingHealth ? "Checking..." : "I Have It — Re-check"}
                  </button>
                </div>
              </div>
            )}

            {health.reachable && health.models.length === 0 && (
              <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                <h4 style={{ color: "var(--color-success)" }}>Ollama is ready. Pull a recommended model?</h4>
                <p style={{ fontSize: "0.9rem" }}>Based on your {sysRam}GB of RAM, we recommend: <strong>{getRecommendedModel()}</strong></p>
              </div>
            )}

            {health.reachable && health.models.length > 0 && (
              <div style={{ background: "rgba(16, 185, 129, 0.05)", padding: "1rem", borderRadius: "8px" }}>
                <h4 style={{ color: "var(--color-success)" }}>Ollama detected with {health.models.length} model(s)</h4>
                <ul style={{ margin: "0.5rem 0 0 1.5rem", fontSize: "0.9rem" }}>
                  {health.models.map(m => <li key={m}>{m}</li>)}
                </ul>
              </div>
            )}
          </div>
        )}

        {/* STEP 3: DOWNLOAD AI MODEL */}
        {step === 3 && (
          <div>
            <div style={{ background: "rgba(0,0,0,0.02)", padding: "1rem", borderRadius: "8px", marginBottom: "1rem" }}>
              <strong>AI Model: {getRecommendedModel()} (Recommended)</strong>
              <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.25rem" }}>
                Downloading AI model. This is a one-time setup.
              </p>
            </div>
            
            {!pulling && !pullComplete && (
              <div>
                <button className="btn btn-primary" onClick={startPullModel}>
                  <Download size={16} style={{ marginRight: "0.5rem" }} /> Download {getRecommendedModel()}
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

        {/* STEP 5: BROWSER PAIRING */}
        {step === 5 && (
          <div>
            <div style={{ background: "rgba(59, 130, 246, 0.05)", padding: "1rem", borderRadius: "8px", marginBottom: "1.5rem" }}>
              <div style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginBottom: "0.5rem" }}>
                <Info size={16} color="var(--color-primary)" />
                <strong style={{ color: "var(--color-primary)" }}>Browser Extension Setup</strong>
              </div>
              <ol style={{ margin: "0 0 0 1.5rem", fontSize: "0.9rem", lineHeight: "1.6" }}>
                <li>Open <code>chrome://extensions/</code> in Chrome/Edge/Brave.</li>
                <li>Enable <strong>Developer Mode</strong>.</li>
                <li>Click <strong>Load Unpacked</strong> &rarr; select the <code>browser-extension/chromium/</code> folder.</li>
                <li>Click the extension icon in your browser and paste the token below.</li>
              </ol>
            </div>

            <label style={{ fontWeight: 600, display: "block", marginBottom: "0.5rem" }}>Pairing Token</label>
            <div style={{ display: "flex", gap: "0.5rem" }}>
              <input type="text" readOnly value={pairingToken} style={{ flex: 1, padding: "0.5rem", fontFamily: "monospace", background: "var(--bg-secondary)" }} />
              <button className="btn btn-secondary" onClick={copyToClipboard}>
                {copied ? <CheckCircle size={16} /> : <Copy size={16} />}
                <span style={{ marginLeft: "0.5rem" }}>{copied ? "Copied" : "Copy"}</span>
              </button>
            </div>
          </div>
        )}

        {/* STEP 6: DONE */}
        {step === 6 && (
          <div style={{ textAlign: "center", padding: "3rem 0" }}>
            <div style={{ display: "inline-flex", background: "rgba(16, 185, 129, 0.1)", padding: "1rem", borderRadius: "50%", marginBottom: "1rem" }}>
              <CheckCircle size={48} color="var(--color-success)" />
            </div>
            <h3 style={{ color: "var(--color-success)", marginBottom: "0.5rem" }}>You're ready.</h3>
            <p className="help-text">All setup steps are complete. Click finish to enter the workspace.</p>
          </div>
        )}
      </div>

      <div className="flex-between" style={{ marginTop: "2rem", paddingTop: "1rem", borderTop: "1px solid var(--border-color)" }}>
        <button className="btn btn-secondary" onClick={handleBack} disabled={step === 1}>
          Back
        </button>
        
        <div style={{ display: "flex", gap: "1rem" }}>
          {(step === 2 || step === 3 || step === 5) && (
            <button className="btn btn-secondary" onClick={() => {
              if (step === 2 && health && !health.reachable) setStep(4);
              else if (step === 3) setStep(4);
              else if (step === 5) setStep(6);
              else setStep(prev => prev + 1);
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
