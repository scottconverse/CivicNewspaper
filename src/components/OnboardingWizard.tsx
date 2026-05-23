import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface OnboardingWizardProps {
  onComplete: () => void;
}

export function OnboardingWizard({ onComplete }: OnboardingWizardProps) {
  const [step, setStep] = useState(1);
  const [modelType, setModelType] = useState<"local" | "remote">("local");
  const [pulling, setPulling] = useState(false);
  const [pullProgress, setPullProgress] = useState(0);

  const handleNext = async () => {
    if (step === 1) {
      if (modelType === "local") {
        setStep(2);
        // Start pull
        setPulling(true);
        try {
          // Fake progress for now, or actual invoke if we have pull_model
          for (let i = 0; i <= 100; i += 10) {
            setPullProgress(i);
            await new Promise((resolve) => setTimeout(resolve, 200));
          }
        } catch (e) {
          console.error(e);
        } finally {
          setPulling(false);
        }
      } else {
        setStep(3); // Skip pull for remote
      }
    } else if (step === 2) {
      setStep(3);
    } else if (step === 3) {
      localStorage.setItem("firstRunCompleted", "true");
      onComplete();
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-content" style={{ maxWidth: "500px" }}>
        {step === 1 && (
          <div>
            <h3>Welcome to CivicNews</h3>
            <p>To get started, choose your AI model type:</p>
            <div style={{ display: "flex", flexDirection: "column", gap: "1rem", marginTop: "1rem" }}>
              <label>
                <input
                  type="radio"
                  name="modelType"
                  value="local"
                  checked={modelType === "local"}
                  onChange={() => setModelType("local")}
                />
                Local Model (Ollama) - Private, secure, uses your PC's hardware.
              </label>
              <label>
                <input
                  type="radio"
                  name="modelType"
                  value="remote"
                  checked={modelType === "remote"}
                  onChange={() => setModelType("remote")}
                />
                Remote API (OpenAI/Anthropic) - Requires an API key in Settings.
              </label>
            </div>
          </div>
        )}

        {step === 2 && (
          <div>
            <h3>Downloading Local Model</h3>
            <p>Pulling the recommended model via Ollama...</p>
            <div style={{ width: "100%", background: "#eee", borderRadius: "4px", height: "20px", marginTop: "1rem" }}>
              <div style={{ width: `${pullProgress}%`, background: "var(--primary-color)", height: "100%", borderRadius: "4px", transition: "width 0.2s" }} />
            </div>
            <p style={{ textAlign: "center", marginTop: "0.5rem" }}>{pullProgress}%</p>
          </div>
        )}

        {step === 3 && (
          <div>
            <h3>Add Your First Source</h3>
            <p>CivicNews needs public records to start analyzing. You can add sources later in the Sources tab.</p>
            <p>Click Finish to explore the app.</p>
          </div>
        )}

        <div style={{ display: "flex", justifyContent: "flex-end", marginTop: "2rem" }}>
          <button className="btn btn-primary" onClick={handleNext} disabled={pulling}>
            {step === 3 ? "Finish Setup" : "Next"}
          </button>
        </div>
      </div>
    </div>
  );
}
