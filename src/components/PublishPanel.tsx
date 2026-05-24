// src/components/PublishPanel.tsx
import React, { useState } from "react";
import { FileDown, ChevronRight, AlertTriangle } from "lucide-react";

interface PublishPanelProps {
  publishPath: string;
  onPublishPathChange: (val: string) => void;
  publishStep: number;
  onPublishStepChange: (step: number) => void;
  loading: boolean;
  onPublish: () => void;
  onOpenLocalPath: (path: string) => void;
}

export const PublishPanel: React.FC<PublishPanelProps> = ({
  publishPath,
  onPublishPathChange,
  publishStep,
  onPublishStepChange,
  loading,
  onPublish,
  onOpenLocalPath
}) => {
  const [error, setError] = useState<string>("");

  const handleCompileClick = () => {
    if (!publishPath.trim()) {
      setError("Output path cannot be empty.");
      return;
    }
    setError("");
    onPublish();
  };

  const handleNextClick = () => {
    if (!publishPath.trim()) {
      setError("Output path cannot be empty.");
      return;
    }
    setError("");
    onPublishStepChange(2);
  };

  return (
    <div className="card" id="publish-panel-container">
      <h3 className="card-title">Static Compilation & Publishing Wizard</h3>
      <p className="help-text">
        Compile approved stories into a clean, standalone, responsive newspaper static directory.
      </p>

      {error && (
        <div 
          className="error-text" 
          style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginTop: "0.5rem" }} 
          data-testid="validation-error"
          id="publish-validation-error"
        >
          <AlertTriangle size={16} />
          {error}
        </div>
      )}
      
      <div style={{ marginTop: "1rem" }}>
        {publishStep === 1 && (
          <div className="wizard-step" id="publish-step-1">
            <h4 style={{ marginBottom: "0.5rem" }}>Step 1: Choose Output Folder</h4>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Select an empty directory on your computer where the generated HTML, CSS, and RSS files will be placed.
            </p>
            <div>
              <input
                type="text"
                value={publishPath}
                onChange={(e) => {
                  setError("");
                  onPublishPathChange(e.target.value);
                }}
                placeholder="C:\my-local-news-site"
                required
                id="input-publish-path"
              />
            </div>
            <div className="btn-group" style={{ marginTop: "1rem" }}>
              <button 
                className="btn btn-primary" 
                onClick={handleNextClick}
                id="btn-publish-next"
              >
                Next: Compile <ChevronRight size={16} />
              </button>
            </div>
          </div>
        )}

        {publishStep === 2 && (
          <div className="wizard-step" id="publish-step-2">
            <h4 style={{ marginBottom: "0.5rem" }}>Step 2: Generate Files</h4>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              We will now compile your approved stories into static HTML files at <strong>{publishPath}</strong>.
            </p>
            <div className="btn-group" style={{ marginTop: "1rem" }}>
              <button className="btn btn-secondary" onClick={() => onPublishStepChange(1)} id="btn-publish-back">
                Back
              </button>
              <button className="btn btn-primary" onClick={handleCompileClick} disabled={loading} id="btn-publish-compile">
                <FileDown size={16} /> {loading ? "Compiling..." : "Compile Static Site"}
              </button>
            </div>
          </div>
        )}

        {publishStep === 3 && (
          <div className="wizard-step" style={{ background: "rgba(16, 185, 129, 0.05)", border: "1px solid rgba(16, 185, 129, 0.2)", padding: "1rem", borderRadius: "8px" }} id="publish-step-3">
            <h4 style={{ marginBottom: "0.5rem", color: "var(--color-success)" }}>✓ Step 3: Publish to the Web</h4>
            <p className="help-text" style={{ marginBottom: "1rem" }}>
              Your site has been generated locally. To make it live for readers:
            </p>
            <ol style={{ marginLeft: "1.5rem", marginBottom: "1rem", fontSize: "0.9rem" }}>
              <li>Open <a href="https://app.netlify.com/drop" target="_blank" rel="noreferrer">Netlify Drop</a> or <a href="https://vercel.com/new/drop" target="_blank" rel="noreferrer">Vercel Drop</a> in your browser.</li>
              <li>Click the button below to open your local output folder.</li>
              <li>Drag and drop the entire folder directly into the browser window.</li>
            </ol>
            <div className="btn-group" style={{ marginTop: "1rem" }}>
              <button className="btn btn-secondary" onClick={() => onPublishStepChange(1)} id="btn-publish-restart">
                Start Over
              </button>
              <button className="btn btn-primary" onClick={() => onOpenLocalPath(publishPath)} id="btn-publish-open-folder">
                Open Folder in Explorer
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
