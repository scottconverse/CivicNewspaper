// src/components/OnboardingWizard.tsx
import React, { useState } from "react";
import { ChevronRight, Download } from "lucide-react";

interface OnboardingWizardProps {
  ollamaOnline: boolean;
  systemRam: number;
  onComplete: () => void;
}

export const OnboardingWizard: React.FC<OnboardingWizardProps> = ({
  ollamaOnline,
  systemRam,
  onComplete
}) => {
  const [step, setStep] = useState<number>(1);
  const [publicationName, setPublicationName] = useState<string>("");

  const steps = [
    { title: "Identity", desc: "Define your local news outlet name and mission." },
    { title: "Ollama", desc: "Connect with the local Ollama LLM endpoint." },
    { title: "Pull Model", desc: "Download recommended models based on system specifications." },
    { title: "Defaults", desc: "Configure publication directories and backup database paths." },
    { title: "Pairing", desc: "Establish authorization pairing with the browser extensions." },
    { title: "Done", desc: "Onboarding completed. Ready to inspect local stories." }
  ];

  const handleNext = () => {
    if (step < steps.length) {
      setStep(prev => prev + 1);
    } else {
      onComplete();
    }
  };

  const handleBack = () => {
    if (step > 1) {
      setStep(prev => prev - 1);
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

      <div style={{ marginTop: "2rem", minHeight: "180px" }}>
        <h3>{steps[step - 1].title}</h3>
        <p className="help-text" style={{ marginBottom: "1.5rem" }}>
          {steps[step - 1].desc}
        </p>

        {step === 1 && (
          <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
            <label style={{ fontWeight: 600 }} htmlFor="input-onboarding-pubname">Publication Name</label>
            <input 
              id="input-onboarding-pubname"
              type="text" 
              placeholder="e.g. The Brighton Gazette" 
              value={publicationName}
              onChange={(e) => setPublicationName(e.target.value)}
            />
          </div>
        )}

        {step === 2 && (
          <div className="card" style={{ background: ollamaOnline ? "rgba(16, 185, 129, 0.05)" : "rgba(239, 68, 68, 0.05)" }}>
            <div className="flex-between">
              <div>
                <strong>Local Ollama Connection</strong>
                <p style={{ fontSize: "0.8rem", color: "var(--text-secondary)" }}>Local Ram: {systemRam} GB</p>
              </div>
              <span className={`status-dot ${ollamaOnline ? "online" : "offline"}`} />
            </div>
          </div>
        )}

        {step === 3 && (
          <div>
            <p style={{ fontSize: "0.9rem" }}>We recommend pulling Llama 3.2 (3B) or Gemma 2 (9B).</p>
            <button className="btn btn-secondary btn-sm" style={{ marginTop: "1rem" }} disabled>
              <Download size={14} /> Pull Recommended Model
            </button>
          </div>
        )}

        {step === 4 && (
          <div>
            <p style={{ fontSize: "0.9rem" }}>Your default site files compile to your Documents/civicnews-site directory.</p>
          </div>
        )}

        {step === 5 && (
          <div>
            <p style={{ fontSize: "0.9rem" }}>Pair extensions using authorization tokens to retrieve story lists.</p>
          </div>
        )}

        {step === 6 && (
          <div style={{ textAlign: "center", padding: "1rem 0" }}>
            <strong style={{ color: "var(--color-success)" }}>✓ All verification items are set up successfully.</strong>
            <p className="help-text" style={{ marginTop: "0.5rem" }}>You are ready to begin writing high-quality local coverage.</p>
          </div>
        )}
      </div>

      <div className="flex-between" style={{ marginTop: "2rem" }}>
        <button 
          className="btn btn-secondary" 
          onClick={handleBack} 
          disabled={step === 1}
        >
          Back
        </button>
        <button 
          className="btn btn-primary" 
          onClick={handleNext}
          id="btn-wizard-next"
        >
          {step === steps.length ? "Finish Onboarding" : "Next"}
          <ChevronRight size={16} />
        </button>
      </div>
    </div>
  );
};
