import React from "react";
import { ArrowLeft, ArrowRight, Check, Download, HardDrive, RefreshCcw } from "lucide-react";
import modelsConfig from "../models.json";

interface AiModelPanelProps {
  ollamaOnline: boolean;
  systemRam: number;
  wizardModel: string;
  installedModels: string[];
  onWizardModelChange: (model: string) => void;
  pullingModel: boolean;
  pullProgressText: string[];
  onPullModel: () => void;
}

const modelSizes: Record<string, string> = (modelsConfig as any).sizes || {};

export const AiModelPanel: React.FC<AiModelPanelProps> = ({
  ollamaOnline,
  systemRam,
  wizardModel,
  installedModels,
  onWizardModelChange,
  pullingModel,
  pullProgressText,
  onPullModel,
}) => {
  const recommended = wizardModel || (systemRam >= 16 ? modelsConfig.high : systemRam >= 8 ? modelsConfig.medium : modelsConfig.low);
  const isInstalled = installedModels.some((model) => model === recommended || `${model}:latest` === recommended || model === `${recommended}:latest`);
  const progressLine = pullProgressText[pullProgressText.length - 1] || (ollamaOnline ? "Local AI service is ready." : "Local AI service is starting.");
  const progressMatch = progressLine.match(/(\d+(?:\.\d+)?)%/);
  const progressPercent = pullingModel ? Number(progressMatch?.[1] ?? 12) : 0;

  const options = [
    { model: modelsConfig.high, label: "16 GB computers", size: modelSizes[modelsConfig.high] || "about 9.3 GB" },
    { model: modelsConfig.medium, label: "8 GB computers", size: modelSizes[modelsConfig.medium] || "about 5.2 GB" },
    { model: modelsConfig.low, label: "lighter laptops", size: modelSizes[modelsConfig.low] || "about 2.5 GB" },
    ...installedModels.map((model) => ({
      model,
      label: "installed locally",
      size: modelSizes[model] || "already downloaded",
    })),
  ].filter((option, index, all) => all.findIndex((other) => other.model === option.model) === index);

  return (
    <div className="ai-model-panel">
      <div className="ai-model-intro">
        <h1>Set up your private AI</h1>
        <p>Everything runs on this computer. No accounts, no cloud, nothing leaves your desk.</p>
      </div>

      <div className="ai-stepper" aria-label="AI setup progress">
        {["Identity", "AI service", "Download model", "Defaults", "Done"].map((step, index) => (
          <div className="ai-step" key={step}>
            <span className={index < 2 ? "done" : index === 2 ? "active" : ""}>
              {index < 2 ? <Check size={16} /> : index + 1}
            </span>
            <strong>{step}</strong>
            {index < 4 && <i />}
          </div>
        ))}
      </div>

      <div className="card ai-download-card">
        <div className="ai-card-heading">
          <div className="ai-card-icon"><Download size={23} /></div>
          <div>
            <h2>Download your AI model</h2>
            <p>Step 3 of 5 - one-time download</p>
          </div>
        </div>

        <p className="ai-card-copy">We picked a model that fits your computer. This downloads once and then works completely offline. Large models can take 10-60+ minutes on slower connections.</p>

        <div className="ai-ram-callout">
          <HardDrive size={19} />
          <span><strong>{systemRam || "Unknown"} GB RAM detected.</strong> Recommended model: <strong>{recommended}</strong>.</span>
        </div>

        <div className="ai-model-progress">
          <div className="flex-between">
            <select value={recommended} onChange={(event) => onWizardModelChange(event.target.value)} aria-label="AI model">
              {options.map((option) => (
                <option value={option.model} key={option.model}>{option.model}</option>
              ))}
            </select>
            <span>{pullingModel ? progressLine : isInstalled ? "Installed and ready" : ollamaOnline ? "Ready to download" : "AI service offline"}</span>
          </div>
          <div className="ai-progress-track">
            <div style={{ width: `${Math.min(100, Math.max(0, progressPercent))}%` }} />
          </div>
          <div className="ai-progress-status">
            {pullingModel && <RefreshCcw className="animate-spin" size={15} />}
            <span>{pullingModel ? progressLine : isInstalled ? "Available on this computer" : `${modelSizes[recommended] || "One-time download"} - local after setup`}</span>
          </div>
          <p className="help-text" style={{ margin: "0.5rem 0 0 0" }}>
            You can leave this running, or cancel and resume later. If progress stops for several minutes, check internet access, restart Civic Desk, then retry.
          </p>
        </div>

        <div className="ai-options-label">Other options</div>
        <div className="ai-model-options">
          {options.filter((option) => option.model !== recommended).map((option) => (
            <button type="button" key={option.model} onClick={() => onWizardModelChange(option.model)}>
              <code>{option.model}</code>
              <span>{option.size} - {option.label}</span>
            </button>
          ))}
        </div>

        <div className="ai-actions">
          <button className="btn btn-secondary" type="button"><ArrowLeft size={16} />Back</button>
          <button className="btn btn-primary" type="button" onClick={onPullModel} disabled={pullingModel || isInstalled || !ollamaOnline}>
            {pullingModel ? "Downloading..." : isInstalled ? "Installed" : `Download ${recommended}`}
            <ArrowRight size={16} />
          </button>
        </div>
      </div>
    </div>
  );
};
