// src/components/PublishPanel.tsx
import React, { useState } from "react";
import { AlertTriangle, CheckCircle, ExternalLink, FileArchive, FileDown, FolderOpen, Rss, UploadCloud } from "lucide-react";
import type { PublishResult } from "../ipc";

interface PublishPanelProps {
  publishPath: string;
  publishResult: PublishResult | null;
  onPublishPathChange: (val: string) => void;
  publishStep: number;
  onPublishStepChange: (step: number) => void;
  loading: boolean;
  onPublish: () => void;
  onOpenLocalPath: (path: string) => void | Promise<void>;
  onOpenExternalUrl: (url: string) => void | Promise<void>;
  onChoosePublishPath: () => void;
  onRecordPublishDestination: (provider: string, publishedUrl: string, deploymentId?: string) => void | Promise<void>;
}

const PROVIDERS = [
  {
    id: "netlify",
    label: "Netlify Drop",
    url: "https://app.netlify.com/drop",
    guidance: "Open Netlify Drop, drag in the ZIP or output folder, then paste the live site URL here.",
  },
  {
    id: "cloudflare_pages",
    label: "Cloudflare Pages",
    url: "https://dash.cloudflare.com/",
    guidance: "Create or open a Pages project, upload the ZIP or folder, then paste the deployed URL here.",
  },
  {
    id: "github_pages",
    label: "GitHub Pages",
    url: "https://github.com/new",
    guidance: "Publish the folder through a repository Pages site, then paste the Pages URL here.",
  },
  {
    id: "substack",
    label: "Substack",
    url: "https://substack.com/home",
    guidance: "Paste the Substack draft into a post, publish it, then paste the public post URL here.",
  },
  {
    id: "wordpress",
    label: "WordPress",
    url: "https://wordpress.com/posts",
    guidance: "Create a post/page from the exported copy, publish it, then paste the public URL here.",
  },
  {
    id: "other",
    label: "Other host",
    url: "https://www.google.com/search?q=static+site+hosting",
    guidance: "Use any static host or local web server, then paste the public URL here.",
  },
];

export const PublishPanel: React.FC<PublishPanelProps> = ({
  publishPath,
  publishResult,
  onPublishPathChange,
  publishStep,
  onPublishStepChange,
  loading,
  onPublish,
  onOpenLocalPath,
  onOpenExternalUrl,
  onChoosePublishPath,
  onRecordPublishDestination
}) => {
  const [error, setError] = useState<string>("");
  const [provider, setProvider] = useState("netlify");
  const [publishedUrl, setPublishedUrl] = useState("");
  const [deploymentId, setDeploymentId] = useState("");

  const selectedProvider = PROVIDERS.find(item => item.id === provider) ?? PROVIDERS[0];

  const artifactPath = (relativePath: string) => {
    const separator = publishPath.includes("\\") ? "\\" : "/";
    return `${publishPath.replace(/[\\/]+$/, "")}${separator}${relativePath}`;
  };

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

  const handleRecordDestinationClick = () => {
    if (!publishResult) {
      setError("Compile the site before saving a public URL.");
      return;
    }
    if (!publishedUrl.trim()) {
      setError("Public URL cannot be empty.");
      return;
    }
    setError("");
    onRecordPublishDestination(provider, publishedUrl, deploymentId);
  };

  const primaryLabel = publishStep === 1 ? "Review compile checklist" : "Compile site";

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
            <button className="btn btn-secondary" type="button" onClick={onChoosePublishPath} id="btn-publish-browse">
              <FolderOpen size={16} />
              Browse
            </button>
          </div>

          <div className="publish-step-list">
            {[
              ["Compile approved stories", "HTML"],
              ["Preview local website", "index.html"],
              ["Export hosting package", "ZIP"],
              ["Publish to a host", "next"],
              ["Share with residents", "posts"],
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
            {loading ? "Compiling..." : primaryLabel}
          </button>
        </div>

        <div className="publish-side">
          <div className="card">
            <div className="last-compiled">
              <span className={publishStep === 3 ? "status-dot online" : "status-dot warning"} />
              <strong>{publishStep === 3 ? "Last compiled" : publishStep === 2 ? "Ready for final compile" : "Review before compiling"}</strong>
            </div>
            <p className="help-text">
              {publishStep === 3
                ? "Generated locally. Use the ZIP for Netlify or Cloudflare Pages, or commit the folder contents to GitHub Pages."
                : publishStep === 2
                  ? "Click Compile site to write the static website files to the output folder."
                  : "Choose an output folder, then review the compile checklist before writing files."}
            </p>
            <div className="btn-group">
              <button className="btn btn-secondary" onClick={() => publishPath && onOpenLocalPath(publishPath)} id="btn-publish-open-folder">Open folder</button>
              <button className="btn btn-secondary" onClick={() => onPublishStepChange(1)} id="btn-publish-restart">Reset</button>
            </div>
          </div>

          {publishResult && (
            <div className="card publish-result-card" aria-label="Compile receipt">
              <h3 className="card-title">Compile receipt</h3>
              <div className="publish-metrics">
                <div>
                  <strong>{publishResult.article_count}</strong>
                  <span>articles</span>
                </div>
                <div>
                  <strong>{publishResult.files_written}</strong>
                  <span>files</span>
                </div>
                <div>
                  <strong>{publishResult.skipped_count}</strong>
                  <span>skipped</span>
                </div>
              </div>
              <div className="artifact-list">
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.zip_path))}>
                  <FileArchive size={16} />
                  ZIP package
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.newsletter_path))}>
                  <FileDown size={16} />
                  Newsletter
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.substack_path))}>
                  <FileDown size={16} />
                  Substack draft
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.share_package_path))}>
                  <UploadCloud size={16} />
                  Share package
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.facebook_post_path))}>
                  <UploadCloud size={16} />
                  Facebook copy
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.subreddit_post_path))}>
                  <UploadCloud size={16} />
                  Subreddit post
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.nextdoor_post_path))}>
                  <UploadCloud size={16} />
                  Nextdoor copy
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.rss_path))}>
                  <Rss size={16} />
                  RSS
                </button>
              </div>
              {publishResult.article_count === 0 && (
                <p className="help-text">No approved stories were included. Approve an attested story in Workbench, then compile again.</p>
              )}
            </div>
          )}

          <div className="publish-next-card">
            <UploadCloud size={20} />
            <p><strong>Next step:</strong> publish the ZIP or folder to Netlify, Cloudflare Pages, or GitHub Pages. Use the newsletter and share package to tell residents where to read it.</p>
          </div>

          {publishResult && (
            <div className="card publish-destination-card">
              <h3 className="card-title">Publish destination</h3>
              <p className="help-text">{selectedProvider.guidance}</p>
              <label htmlFor="select-publish-provider" style={{ fontWeight: 600, display: "block", marginBottom: "0.35rem" }}>Provider</label>
              <select id="select-publish-provider" value={provider} onChange={event => setProvider(event.target.value)}>
                {PROVIDERS.map(item => <option key={item.id} value={item.id}>{item.label}</option>)}
              </select>
              <div className="btn-group" style={{ marginTop: "0.75rem" }}>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenExternalUrl(selectedProvider.url)}>
                  <ExternalLink size={16} />
                  Open {selectedProvider.label}
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.zip_path))}>
                  <FileArchive size={16} />
                  Open ZIP
                </button>
              </div>
              <label htmlFor="input-published-url" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Public URL</label>
              <input
                id="input-published-url"
                type="url"
                value={publishedUrl}
                onChange={event => {
                  setError("");
                  setPublishedUrl(event.target.value);
                }}
                placeholder="https://your-town-news.example.com"
              />
              <label htmlFor="input-deployment-id" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Deployment ID or note</label>
              <input
                id="input-deployment-id"
                type="text"
                value={deploymentId}
                onChange={event => setDeploymentId(event.target.value)}
                placeholder="optional"
              />
              <button className="btn btn-primary btn-full" type="button" onClick={handleRecordDestinationClick} disabled={loading} style={{ marginTop: "0.9rem" }}>
                <CheckCircle size={16} />
                Save public URL
              </button>
              {publishResult.published_url && (
                <p className="help-text">Saved live URL: <a href={publishResult.published_url}>{publishResult.published_url}</a></p>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
