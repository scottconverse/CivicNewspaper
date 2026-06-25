// src/components/PublishPanel.tsx
import React, { useState } from "react";
import { AlertTriangle, CheckCircle, FileDown, FolderOpen, UploadCloud } from "lucide-react";

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
    <div id="publish-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Publishing</h1>
          <p>Compile your approved stories into a ready-to-host website folder.</p>
        </div>
      </div>

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

      <div className="publish-grid">
        <div className="card publish-compile-card">
          <h3 className="card-title">Compile your gazette</h3>
          <label htmlFor="input-publish-path" style={{ fontWeight: 600, display: "block", marginBottom: "0.35rem" }}>Output folder</label>
          <div className="path-row">
            <input
              type="text"
              value={publishPath}
              onChange={(e) => {
                setError("");
                onPublishPathChange(e.target.value);
              }}
              placeholder="C:\\CivicDesk\\site"
              required
              id="input-publish-path"
            />
            <button className="btn btn-secondary" type="button" onClick={() => publishPath && onOpenLocalPath(publishPath)}>
              <FolderOpen size={16} />
              Browse
            </button>
          </div>

          <div className="publish-step-list">
            {[
              ["Render approved stories to HTML", "stories"],
              ["Copy styles and assets", "files"],
              ["Generate RSS feed", "feed.xml"],
              ["Append correction notices", "notices"],
            ].map(([label, meta]) => (
              <div className="publish-step-row" key={label}>
                <CheckCircle size={19} />
                <span>{label}</span>
                <code>{meta}</code>
              </div>
            ))}
          </div>

          <button className="btn btn-primary btn-full" onClick={publishStep === 1 ? handleNextClick : handleCompileClick} disabled={loading} id={publishStep === 1 ? "btn-publish-next" : "btn-publish-compile"}>
            <FileDown size={16} />
            {loading ? "Compiling..." : "Compile site"}
          </button>
        </div>

        <div className="publish-side">
          <div className="card">
            <div className="last-compiled">
              <span className={publishStep === 3 ? "status-dot online" : "status-dot warning"} />
              <strong>{publishStep === 3 ? "Last compiled" : "Ready to compile"}</strong>
            </div>
            <p className="help-text">
              {publishStep === 3 ? "Generated locally. Open the folder and drag it into your hosting provider when you are ready." : "The Civic Desk will render a static site folder from approved stories and correction notices."}
            </p>
            <div className="btn-group">
              <button className="btn btn-secondary" onClick={() => publishPath && onOpenLocalPath(publishPath)} id="btn-publish-open-folder">Open folder</button>
              <button className="btn btn-secondary" onClick={() => onPublishStepChange(1)} id="btn-publish-restart">Reset</button>
            </div>
          </div>

          <div className="publish-next-card">
            <UploadCloud size={20} />
            <p><strong>Next step:</strong> drag this folder into Netlify or a GitHub Pages repo to put it online.</p>
          </div>
        </div>
      </div>
    </div>
  );
};
