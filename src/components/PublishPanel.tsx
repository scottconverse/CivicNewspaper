// src/components/PublishPanel.tsx
import React, { useEffect, useState } from "react";
import { AlertTriangle, CheckCircle, Copy, ExternalLink, FileArchive, FileDown, FolderOpen, Mail, Rss, Trash2, UploadCloud, UserPlus } from "lucide-react";
import type { CommunityProfile, PublishResult, PublishRun, PublisherConfig, PublisherConfigInput, PublisherTestResult, Subscriber } from "../ipc";

interface PublishPanelProps {
  publishPath: string;
  publishResult: PublishResult | null;
  publishHistory: PublishRun[];
  publisherConfig: PublisherConfig | null;
  publisherProvider: string;
  publisherTestResult: PublisherTestResult | null;
  subscribers: Subscriber[];
  subscriberEmail: string;
  subscriberName: string;
  onSubscriberEmailChange: (value: string) => void;
  onSubscriberNameChange: (value: string) => void;
  onPublishPathChange: (val: string) => void;
  publishStep: number;
  onPublishStepChange: (step: number) => void;
  loading: boolean;
  onPublish: () => void;
  onOpenLocalPath: (path: string, label?: string) => void | Promise<void>;
  onOpenExternalUrl: (url: string) => void | Promise<void>;
  onChoosePublishPath: () => void;
  onRecordPublishDestination: (provider: string, publishedUrl: string, deploymentId?: string) => void | Promise<void>;
  onPublishWithConnector: (provider: string, publishedUrl: string, deploymentId?: string) => void | Promise<void>;
  onLoadPublisherConfig: (provider: string) => void | Promise<void>;
  onSavePublisherConfig: (config: PublisherConfigInput) => void | Promise<void>;
  onTestPublisherConnection: (provider: string) => void | Promise<void>;
  onAddSubscriber: () => void | Promise<void>;
  onDeleteSubscriber: (id: number) => void | Promise<void>;
  onImportSubscribersCsv: () => void | Promise<void>;
  onExportSubscribersCsv: () => void | Promise<void>;
  onExportIssueEmail: () => void | Promise<void>;
  onCopyPublishText: (label: string, text: string) => void | Promise<void>;
  onCopyPublishArtifact: (label: string, relativePath: string) => void | Promise<void>;
  communityProfile?: CommunityProfile | null;
  onOpenSettings?: () => void;
  approvedDraftCount?: number;
}

const PROVIDERS = [
  {
    id: "here_now",
    label: "here.now",
    url: "https://here.now/dashboard",
    guidance: "Publish instantly with here.now. Use a free API key for permanent civic newspaper sites, or publish a 24-hour preview without an account.",
  },
  {
    id: "github_pages",
    label: "GitHub Pages",
    url: "https://github.com/new",
    guidance: "Use GitHub Pages if you want a durable public archive in your own repository.",
  },
  {
    id: "netlify",
    label: "Netlify",
    url: "https://app.netlify.com/",
    guidance: "Technical option: publishes the generated ZIP to an existing Netlify site using a personal access token.",
  },
  {
    id: "cloudflare_pages",
    label: "Cloudflare Pages",
    url: "https://dash.cloudflare.com/",
    guidance: "Assisted beta option: export the generated folder or ZIP, deploy it in Cloudflare Pages yourself, then save the public URL here.",
  },
  {
    id: "substack",
    label: "Substack",
    url: "https://substack.com/home",
    guidance: "Substack does not offer a supported public publishing API. Use the generated Substack draft, then save the public URL.",
  },
  {
    id: "wordpress",
    label: "WordPress",
    url: "https://wordpress.com/posts",
    guidance: "Beta hold: direct API publishing is disabled until draft-first publishing, rollback, and live connector proof are complete. Export the ZIP/static folder or record a manually published URL.",
  },
  {
    id: "other",
    label: "Other host",
    url: "https://www.google.com/search?q=static+site+hosting",
    guidance: "Use any static host or local web server, then paste the public URL here.",
  },
];

const SETUP_GUIDES: Record<string, { credential: string; target: string; permission: string; verify: string }> = {
  here_now: {
    credential: "Optional. Publish a 24-hour preview with no account, or save a here.now API key for permanent sites.",
    target: "Leave the slug blank to create a new site, or enter an existing here.now slug to update that site.",
    permission: "An API key owns permanent sites. Anonymous previews are temporary and can be claimed in here.now.",
    verify: "Test connection checks the saved API key when present; otherwise it confirms preview publishing is available.",
  },
  netlify: {
    credential: "Create a Netlify personal access token in User settings -> Applications.",
    target: "Copy the site ID from Site configuration -> General -> Site details.",
    permission: "The token must be able to deploy the selected site.",
    verify: "Test connection checks the site ID and token before upload.",
  },
  github_pages: {
    credential: "Create a GitHub fine-grained token for the target repository.",
    target: "Use owner/repo, a Pages branch such as gh-pages, and either root or docs as the folder path.",
    permission: "The token needs repository Contents read/write. Pages will be configured from the branch on publish.",
    verify: "Test connection checks repository access; publish creates the branch if needed.",
  },
  cloudflare_pages: {
    credential: "No Cloudflare credential is used by this public beta.",
    target: "Create or choose a Cloudflare Pages project in Cloudflare, then upload the exported folder or ZIP there.",
    permission: "Use Cloudflare's own dashboard or CLI outside the app.",
    verify: "After Cloudflare publishes the site, paste the public URL here and save it.",
  },
  wordpress: {
    credential: "No WordPress credential is used by this public beta.",
    target: "Export the folder or ZIP, publish through WordPress manually or your own workflow, then paste the public URL here.",
    permission: "Direct WordPress API publishing is disabled until draft-first publishing, rollback, and live connector proof are complete.",
    verify: "Saving the public URL records where readers can find the manually published issue.",
  },
  substack: {
    credential: "No supported public publishing API is available.",
    target: "Use the generated Substack draft artifact.",
    permission: "Publish in Substack, then save the public URL here.",
    verify: "The connector records the public URL after assisted publishing.",
  },
  other: {
    credential: "Use the host's own publishing flow.",
    target: "Upload the folder or ZIP, then copy the public URL.",
    permission: "No app credential is stored for this connector.",
    verify: "Save public URL updates the manifest and share package.",
  },
};

export const PublishPanel: React.FC<PublishPanelProps> = ({
  publishPath,
  publishResult,
  publishHistory,
  publisherConfig,
  publisherProvider,
  publisherTestResult,
  subscribers,
  subscriberEmail,
  subscriberName,
  onSubscriberEmailChange,
  onSubscriberNameChange,
  onPublishPathChange,
  publishStep,
  onPublishStepChange,
  loading,
  onPublish,
  onOpenLocalPath,
  onOpenExternalUrl,
  onChoosePublishPath,
  onRecordPublishDestination,
  onPublishWithConnector,
  onLoadPublisherConfig,
  onSavePublisherConfig,
  onTestPublisherConnection,
  onAddSubscriber,
  onDeleteSubscriber,
  onImportSubscribersCsv,
  onExportSubscribersCsv,
  onExportIssueEmail,
  onCopyPublishText,
  onCopyPublishArtifact,
  communityProfile,
  onOpenSettings,
  approvedDraftCount = 0
}) => {
  const [error, setError] = useState<string>("");
  const [provider, setProvider] = useState(publisherProvider || "here_now");
  const [displayName, setDisplayName] = useState("My publication on here.now");
  const [siteUrl, setSiteUrl] = useState("");
  const [projectHint, setProjectHint] = useState("");
  const [siteId, setSiteId] = useState("");
  const [accountId, setAccountId] = useState("");
  const [repo, setRepo] = useState("");
  const [branch, setBranch] = useState("");
  const [pathPrefix, setPathPrefix] = useState("");
  const [username, setUsername] = useState("");
  const [credential, setCredential] = useState("");
  const [clearCredential, setClearCredential] = useState(false);
  const [publishedUrl, setPublishedUrl] = useState("");
  const [deploymentId, setDeploymentId] = useState("");

  const selectedProvider = PROVIDERS.find(item => item.id === provider) ?? PROVIDERS[0];
  const setupGuide = SETUP_GUIDES[provider] ?? SETUP_GUIDES.other;
  const connectorTestPassed = publisherTestResult?.provider === provider && publisherTestResult.ok;
  const connectorDisabled = provider === "cloudflare_pages" || provider === "wordpress";
  const hereNowSlugRequiresCredential = provider === "here_now" && siteId.trim().length > 0;
  const connectorPublishAllowed =
    !connectorDisabled && (provider === "here_now" ? !hereNowSlugRequiresCredential || connectorTestPassed : connectorTestPassed);
  const leadArticle = publishResult?.articles?.[0];
  const substackHeadline = leadArticle?.title || publishResult?.issue_id || "The Civic Desk update";
  const substackDeck = publishResult
    ? `${publishResult.article_count} local update${publishResult.article_count === 1 ? "" : "s"} ready to share.`
    : "Local updates ready to share.";
  const activeSubscriberCount = subscribers.filter(subscriber => subscriber.status === "active").length;
  const publicationName = communityProfile?.site_title?.trim() || "";
  const hasStarterPublicationName =
    !publicationName || publicationName.toLowerCase() === "my local publication";
  const publicationNameRequiredMessage =
    "Choose and save a real publication name before compiling or publishing.";
  const hasApprovedStories = approvedDraftCount > 0;
  const noApprovedStoriesMessage =
    "No approved stories are ready to publish. Approve at least one story or brief in Workbench before compiling a public package.";

  useEffect(() => {
    setProvider(publisherProvider || "here_now");
  }, [publisherProvider]);

  useEffect(() => {
    setDisplayName(publisherConfig?.display_name || selectedProvider.label);
    setSiteUrl(publisherConfig?.site_url || "");
    setProjectHint(publisherConfig?.project_hint || "");
    setSiteId(publisherConfig?.site_id || "");
    setAccountId(publisherConfig?.account_id || "");
    setRepo(publisherConfig?.repo || "");
    setBranch(publisherConfig?.branch || (provider === "github_pages" ? "gh-pages" : provider === "cloudflare_pages" ? "main" : ""));
    setPathPrefix(publisherConfig?.path_prefix || "");
    setUsername(publisherConfig?.username || "");
    setCredential("");
    setClearCredential(false);
  }, [publisherConfig, selectedProvider.label, provider]);

  const artifactPath = (relativePath: string) => {
    const separator = publishPath.includes("\\") ? "\\" : "/";
    return `${publishPath.replace(/[\\/]+$/, "")}${separator}${relativePath}`;
  };

  const handleCompileClick = () => {
    if (!publishPath.trim()) {
      setError("Output path cannot be empty.");
      return;
    }
    if (hasStarterPublicationName) {
      setError(publicationNameRequiredMessage);
      return;
    }
    if (!hasApprovedStories) {
      setError(noApprovedStoriesMessage);
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
    if (hasStarterPublicationName) {
      setError(publicationNameRequiredMessage);
      return;
    }
    if (!hasApprovedStories) {
      setError(noApprovedStoriesMessage);
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
    if (hasStarterPublicationName) {
      setError(publicationNameRequiredMessage);
      return;
    }
    if (publishResult.article_count === 0) {
      setError(noApprovedStoriesMessage);
      return;
    }
    setError("");
    onRecordPublishDestination(provider, publishedUrl, deploymentId);
  };

  const handleConnectorPublishClick = () => {
    if (!publishResult) {
      setError("Compile the site before publishing.");
      return;
    }
    if (hasStarterPublicationName) {
      setError(publicationNameRequiredMessage);
      return;
    }
    if (publishResult.article_count === 0) {
      setError(noApprovedStoriesMessage);
      return;
    }
    if (hereNowSlugRequiresCredential && !connectorTestPassed) {
      setError("Save and test a here.now API key before publishing to an existing slug. Leave the slug blank for an anonymous preview.");
      return;
    }
    setError("");
    onPublishWithConnector(provider, publishedUrl, deploymentId);
  };

  const handleProviderChange = (nextProvider: string) => {
    setProvider(nextProvider);
    setError("");
    onLoadPublisherConfig(nextProvider);
  };

  const handleSaveConnectorClick = () => {
    setError("");
    onSavePublisherConfig({
      provider,
      display_name: displayName,
      site_url: siteUrl || null,
      project_hint: projectHint || null,
      site_id: siteId || null,
      account_id: accountId || null,
      repo: repo || null,
      branch: branch || null,
      path_prefix: pathPrefix || null,
      username: providerUsesCredential ? username || null : null,
      credential: credential || null,
      clear_credential: clearCredential,
    });
  };

  const providerCredentialLabel = () => {
    if (provider === "here_now") return "here.now API key";
    if (provider === "wordpress") return "No WordPress credential used in this beta";
    if (provider === "github_pages") return "GitHub fine-grained token";
    if (provider === "cloudflare_pages") return "No Cloudflare credential used in this beta";
    if (provider === "netlify") return "Netlify personal access token";
    return "API token or credential";
  };

  const providerUsesCredential = !["wordpress", "cloudflare_pages", "substack", "other"].includes(provider);

  const providerCredentialPlaceholder = publisherConfig?.has_credential
    ? "credential saved; enter a new one to replace"
    : provider === "here_now"
      ? siteId.trim()
        ? "required for an existing here.now slug"
        : "optional; leave blank for anonymous preview"
      : provider === "cloudflare_pages"
      ? "Cloudflare API publishing is disabled in this beta"
      : provider === "substack"
      ? "Substack has no supported publishing API"
      : "paste the provider credential here";

  const primaryLabel = publishStep === 1 ? "Review compile checklist" : "Compile site";
  const publisherTypeLabel: Record<string, string> = {
    single_person: "Single person",
    for_profit: "For-profit publication",
    nonprofit: "Nonprofit publication",
    private_org: "Private organization",
    community_group: "Community group",
    other: "Other",
  };
  const identityWarnings = [
    hasStarterPublicationName
      ? "Publication name still uses starter text. Public publishing is paused until you choose one."
      : "",
    !communityProfile?.about_text?.trim() ? "About text is empty." : "",
    !communityProfile?.ethics_text?.trim() ? "Ethics statement is empty." : "",
    !communityProfile?.footer_text?.trim() ? "Footer/legal note is blank; that is allowed, but nothing will be invented for you." : "",
  ].filter(Boolean);

  return (
    <div id="publish-panel-container">
      <div className="page-header">
        <div className="page-title">
          <h1>Publishing</h1>
          <p>Publish instantly with here.now. Use GitHub Pages if you want a public archive in your own repository.</p>
        </div>
      </div>

      {error && (
        <div 
          className="error-text" 
          style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginTop: "0.5rem" }} 
          data-testid="validation-error"
          id="publish-validation-error"
          role="alert"
          aria-live="assertive"
        >
          <AlertTriangle size={16} />
          {error}
        </div>
      )}

      <div className="publish-grid">
        <div className="card publish-compile-card">
          <div className="preflight-card" id="publication-identity-review" style={{ marginBottom: "1rem", paddingBottom: "1rem", borderBottom: "1px solid var(--border-color)" }}>
            <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
              <div>
                <h3 className="card-title" style={{ marginBottom: "0.35rem" }}>Public site identity review</h3>
                <p className="help-text" style={{ margin: 0 }}>
                  Review this before compile. The app publishes only the identity and policy copy you choose.
                </p>
              </div>
              {onOpenSettings && (
                <button className="btn btn-secondary btn-sm" type="button" onClick={onOpenSettings} id="btn-edit-publish-identity">
                  Edit identity
                </button>
              )}
            </div>
            <dl className="help-text" style={{ display: "grid", gridTemplateColumns: "max-content 1fr", gap: "0.35rem 0.75rem", margin: "0.75rem 0 0" }}>
              <dt>Publication</dt>
              <dd>{communityProfile?.site_title || "Not set"}</dd>
              <dt>Tagline</dt>
              <dd>{communityProfile?.site_subtitle || "Not set"}</dd>
              <dt>Publisher type</dt>
              <dd>{publisherTypeLabel[communityProfile?.organization_type || ""] || "Not set"}</dd>
              <dt>Layout</dt>
              <dd>{communityProfile?.layout_style || "classic"} / {communityProfile?.accent_color || "#5a1818"}</dd>
              <dt>Logo</dt>
              <dd>{communityProfile?.logo_url ? "Configured" : "Text masthead"}</dd>
              <dt>Footer note</dt>
              <dd>{communityProfile?.footer_text ? "Configured" : "Blank"}</dd>
              <dt>Advisor</dt>
              <dd>{communityProfile?.first_amendment_advisor_enabled === false ? "Off" : "On"}</dd>
            </dl>
            {identityWarnings.length > 0 && (
              <ul className="help-text" style={{ margin: "0.75rem 0 0 1.2rem" }}>
                {identityWarnings.map((warning) => <li key={warning}>{warning}</li>)}
              </ul>
            )}
            {hasStarterPublicationName && (
              <p role="alert" className="error-text" style={{ margin: "0.75rem 0 0" }}>
                {publicationNameRequiredMessage}
              </p>
            )}
          </div>
          <h3 className="card-title">Compile your gazette</h3>
          <div className={hasApprovedStories ? "setup-guide" : "error-text"} aria-label="Approved story readiness" style={{ marginBottom: "0.85rem" }}>
            <strong>{approvedDraftCount}</strong> approved stor{approvedDraftCount === 1 ? "y" : "ies"} ready for the public package.
            {!hasApprovedStories && " Move at least one draft to approved/ready-to-publish before compiling."}
          </div>
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
              aria-describedby={error ? "publish-validation-error" : undefined}
            />
            <button className="btn btn-secondary" type="button" onClick={onChoosePublishPath} id="btn-publish-browse">
              <FolderOpen size={16} />
              Browse
            </button>
          </div>

          <div className="publish-step-list">
            {[
              { label: "Approve at least one story", meta: `${approvedDraftCount}`, complete: hasApprovedStories },
              { label: "Compile approved stories", meta: "HTML", complete: !!publishResult },
              { label: "Preview local website", meta: "index.html", complete: !!publishResult },
              { label: "Export hosting package", meta: "ZIP", complete: !!publishResult?.zip_path },
              { label: "Publish to a host", meta: publishResult?.published_url ? "live URL" : "next", complete: !!publishResult?.published_url },
              { label: "Share with residents", meta: "posts", complete: !!publishResult?.published_url },
            ].map(({ label, meta, complete }) => (
              <div className="publish-step-row" key={label}>
                {complete ? <CheckCircle size={19} /> : <AlertTriangle size={19} />}
                <span>{label}</span>
                <code>{complete ? meta : "pending"}</code>
              </div>
            ))}
          </div>

          <button className="btn btn-primary btn-full" onClick={publishStep === 1 ? handleNextClick : handleCompileClick} disabled={loading || !hasApprovedStories} id={publishStep === 1 ? "btn-publish-next" : "btn-publish-compile"}>
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
                ? "Generated locally. Publish instantly with here.now, or use GitHub Pages for a repository-backed archive."
                : publishStep === 2
                  ? "Click Compile site to write the static website files to the output folder."
                  : "Choose an output folder, then review the compile checklist before writing files."}
            </p>
            <div className="btn-group">
              <button className="btn btn-secondary" onClick={() => publishPath && onOpenLocalPath(publishPath, "output folder")} id="btn-publish-open-folder">Open folder</button>
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
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.zip_path), "site-package.zip")}>
                  <FileArchive size={16} />
                  ZIP package
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.newsletter_path), "newsletter.md")}>
                  <FileDown size={16} />
                  Newsletter
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.substack_path), "substack.md")}>
                  <FileDown size={16} />
                  Substack draft
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.share_package_path), "share-package.md")}>
                  <UploadCloud size={16} />
                  Share package
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.facebook_post_path), "facebook-post.txt")}>
                  <UploadCloud size={16} />
                  Facebook copy
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.subreddit_post_path), "subreddit-post.md")}>
                  <UploadCloud size={16} />
                  Subreddit post
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.nextdoor_post_path), "nextdoor-post.txt")}>
                  <UploadCloud size={16} />
                  Nextdoor copy
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.rss_path), "feed.xml")}>
                  <Rss size={16} />
                  RSS
                </button>
              </div>
              {publishResult.article_count === 0 && (
                <p className="help-text">No approved stories were included. Approve a story in Workbench, then compile again.</p>
              )}
            </div>
          )}

          {publishResult && (
            <div className="card newsletter-card" aria-label="Newsletter and Substack tools">
              <h3 className="card-title">Newsletter and Substack</h3>
              <p className="help-text">Use the generated draft as the source of truth, then save the public URL after posting.</p>
              <div className="artifact-list">
                <button className="btn btn-secondary" type="button" onClick={() => onOpenExternalUrl("https://substack.com/home")}>
                  <ExternalLink size={16} />
                  Open Substack editor
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onCopyPublishText("Substack headline", substackHeadline)}>
                  <Copy size={16} />
                  Copy headline
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onCopyPublishText("Substack deck", substackDeck)}>
                  <Copy size={16} />
                  Copy deck
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onCopyPublishArtifact("Substack body", publishResult.substack_path)}>
                  <Copy size={16} />
                  Copy post body
                </button>
                <button className="btn btn-secondary" type="button" onClick={onExportIssueEmail}>
                  <Mail size={16} />
                  Export issue email template
                </button>
              </div>
            </div>
          )}

          <div className="card subscriber-card" aria-label="Subscriber list">
            <h3 className="card-title">Subscriber list</h3>
            <p className="help-text">{activeSubscriberCount} active subscriber{activeSubscriberCount === 1 ? "" : "s"} stored locally.</p>
            <label htmlFor="input-subscriber-email" style={{ fontWeight: 600, display: "block", marginBottom: "0.35rem" }}>Email</label>
            <input
              id="input-subscriber-email"
              type="email"
              value={subscriberEmail}
              onChange={event => onSubscriberEmailChange(event.target.value)}
              placeholder="reader@example.com"
            />
            <label htmlFor="input-subscriber-name" style={{ fontWeight: 600, display: "block", marginTop: "0.7rem", marginBottom: "0.35rem" }}>Name</label>
            <input
              id="input-subscriber-name"
              type="text"
              value={subscriberName}
              onChange={event => onSubscriberNameChange(event.target.value)}
              placeholder="optional"
            />
            <div className="btn-group" style={{ marginTop: "0.75rem" }}>
              <button className="btn btn-secondary" type="button" onClick={onAddSubscriber} disabled={loading}>
                <UserPlus size={16} />
                Add
              </button>
              <button className="btn btn-secondary" type="button" onClick={onImportSubscribersCsv} disabled={loading}>
                <FileDown size={16} />
                Import CSV
              </button>
              <button className="btn btn-secondary" type="button" onClick={onExportSubscribersCsv} disabled={loading}>
                <UploadCloud size={16} />
                Export CSV
              </button>
            </div>
            {subscribers.length === 0 ? (
              <p className="help-text">No subscribers yet.</p>
            ) : (
              <div className="subscriber-list">
                {subscribers.slice(0, 5).map(subscriber => (
                  <div className="subscriber-row" key={subscriber.id || subscriber.email}>
                    <span>
                      <strong>{subscriber.email}</strong>
                      {subscriber.name ? <small>{subscriber.name}</small> : null}
                    </span>
                    {subscriber.id && (
                      <button className="btn btn-secondary btn-sm" type="button" onClick={() => onDeleteSubscriber(subscriber.id!)} aria-label={`Remove ${subscriber.email}`}>
                        <Trash2 size={14} />
                      </button>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="publish-next-card">
            <UploadCloud size={20} />
            <p><strong>Next step:</strong> publish instantly with here.now. Use GitHub Pages for a durable public archive, and use the newsletter and share package to tell residents where to read it.</p>
          </div>

          {publishResult && (
            <div className="card publish-destination-card">
              <h3 className="card-title">Publisher connector</h3>
              <p className="help-text">{selectedProvider.guidance}</p>
              <div className="setup-guide" aria-label="Connector setup guide">
                <p><strong>Account setup:</strong> {setupGuide.credential}</p>
                <p><strong>Target:</strong> {setupGuide.target}</p>
                <p><strong>Required access:</strong> {setupGuide.permission}</p>
                <p><strong>Check:</strong> {setupGuide.verify}</p>
              </div>
              <label htmlFor="select-publish-provider" style={{ fontWeight: 600, display: "block", marginBottom: "0.35rem" }}>Provider</label>
              <select id="select-publish-provider" value={provider} onChange={event => handleProviderChange(event.target.value)}>
                {PROVIDERS.map(item => <option key={item.id} value={item.id}>{item.label}</option>)}
              </select>
              <div className="publish-primary-action-panel">
                <label htmlFor="input-published-url" style={{ fontWeight: 600, display: "block", marginBottom: "0.35rem" }}>Public URL</label>
                <input
                  id="input-published-url"
                  type="url"
                  value={publishedUrl}
                  onChange={event => {
                    setError("");
                    setPublishedUrl(event.target.value);
                  }}
                  placeholder="optional for API connectors; required for manual save"
                />
                <label htmlFor="input-deployment-id" style={{ fontWeight: 600, display: "block", marginTop: "0.75rem", marginBottom: "0.35rem" }}>Deployment ID or note</label>
                <input
                  id="input-deployment-id"
                  type="text"
                  value={deploymentId}
                  onChange={event => setDeploymentId(event.target.value)}
                  placeholder="optional"
                />
                <div className="btn-group publish-action-row" style={{ marginTop: "0.75rem" }}>
                  <button className="btn btn-primary" type="button" onClick={handleRecordDestinationClick} disabled={loading}>
                    <CheckCircle size={16} />
                    Save public URL
                  </button>
                  <button className="btn btn-secondary" type="button" onClick={handleConnectorPublishClick} disabled={loading || !connectorPublishAllowed} aria-describedby={error ? "publish-validation-error" : undefined}>
                    <UploadCloud size={16} />
                    {provider === "here_now" ? "Publish to here.now" : "Publish with connector"}
                  </button>
                </div>
                {connectorDisabled && (
                  <p className="help-text" style={{ marginTop: "0.35rem" }}>
                    {provider === "wordpress"
                      ? "WordPress API publishing is disabled in this public beta. Export the folder or ZIP, publish through WordPress manually, then save the public URL here."
                      : "Cloudflare API publishing is disabled in this public beta. Export the folder or ZIP, deploy through Cloudflare, then save the public URL here."}
                  </p>
                )}
                {!connectorDisabled && !connectorPublishAllowed && (
                  <p className="help-text" style={{ marginTop: "0.35rem" }}>
                    Test the selected connector before publishing. You can still save a public URL manually.
                  </p>
                )}
                {provider === "here_now" && !connectorTestPassed && (
                  <p className="help-text" style={{ marginTop: "0.35rem" }}>
                    {hereNowSlugRequiresCredential
                      ? "This slug is an account-owned here.now target. Save and test an API key before publishing to it, or clear the slug for a temporary anonymous preview."
                      : "here.now can publish a temporary anonymous preview without a saved API key. Save and test an API key only for permanent account-owned sites."}
                  </p>
                )}
                {publishResult.published_url && (
                  <p className="help-text">Saved live URL: <a href={publishResult.published_url}>{publishResult.published_url}</a></p>
                )}
              </div>
              <label htmlFor="input-publisher-name" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Connector name</label>
              <input
                id="input-publisher-name"
                type="text"
                value={displayName}
                onChange={event => setDisplayName(event.target.value)}
              />
              <label htmlFor="input-publisher-site-url" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Default public URL</label>
              <input
                id="input-publisher-site-url"
                type="url"
                value={siteUrl}
                onChange={event => setSiteUrl(event.target.value)}
                placeholder="https://your-town-news.example.com"
              />
              {provider === "here_now" && (
                <>
                  <label htmlFor="input-publisher-site-id" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>here.now slug</label>
                  <input
                    id="input-publisher-site-id"
                    type="text"
                    value={siteId}
                    onChange={event => {
                      setError("");
                      setSiteId(event.target.value);
                    }}
                    placeholder="blank creates a new site; enter a slug to update"
                  />
                  <p className="help-text" style={{ marginTop: "0.35rem" }}>
                    Leave the slug blank for the live-tested anonymous preview path. Entering a slug means you are updating an account-owned site and must save/test an API key first.
                  </p>
                </>
              )}
              {provider === "netlify" && (
                <>
                  <label htmlFor="input-publisher-site-id" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Netlify site ID</label>
                  <input
                    id="input-publisher-site-id"
                    type="text"
                    value={siteId}
                    onChange={event => setSiteId(event.target.value)}
                    placeholder="e.g. 12345678-abcd-..."
                  />
                </>
              )}
              {provider === "github_pages" && (
                <>
                  <label htmlFor="input-publisher-repo" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Repository</label>
                  <input
                    id="input-publisher-repo"
                    type="text"
                    value={repo}
                    onChange={event => setRepo(event.target.value)}
                    placeholder="owner/repo"
                  />
                  <label htmlFor="input-publisher-branch" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Branch</label>
                  <input
                    id="input-publisher-branch"
                    type="text"
                    value={branch}
                    onChange={event => setBranch(event.target.value)}
                    placeholder="gh-pages"
                  />
                  <label htmlFor="input-publisher-path-prefix" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Folder path</label>
                  <input
                    id="input-publisher-path-prefix"
                    type="text"
                    value={pathPrefix}
                    onChange={event => setPathPrefix(event.target.value)}
                    placeholder="leave blank for root, or use docs"
                  />
                </>
              )}
              {provider === "cloudflare_pages" && (
                <>
                  <label htmlFor="input-publisher-account-id" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Cloudflare account ID</label>
                  <input
                    id="input-publisher-account-id"
                    type="text"
                    value={accountId}
                    onChange={event => setAccountId(event.target.value)}
                    placeholder="account ID"
                  />
                  <label htmlFor="input-publisher-project-name" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Pages project name</label>
                  <input
                    id="input-publisher-project-name"
                    type="text"
                    value={siteId}
                    onChange={event => setSiteId(event.target.value)}
                    placeholder="your-pages-project"
                  />
                  <label htmlFor="input-publisher-cloudflare-branch" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Production branch</label>
                  <input
                    id="input-publisher-cloudflare-branch"
                    type="text"
                    value={branch}
                    onChange={event => setBranch(event.target.value)}
                    placeholder="main"
                  />
                </>
              )}
              <label htmlFor="input-publisher-project" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>Notes</label>
              <input
                id="input-publisher-project"
                type="text"
                value={projectHint}
                onChange={event => setProjectHint(event.target.value)}
                placeholder="optional internal note"
              />
              {providerUsesCredential && (
                <>
                  <label htmlFor="input-publisher-credential" style={{ fontWeight: 600, display: "block", marginTop: "0.9rem", marginBottom: "0.35rem" }}>{providerCredentialLabel()}</label>
                  <input
                    id="input-publisher-credential"
                    type="password"
                    value={credential}
                    onChange={event => setCredential(event.target.value)}
                    placeholder={providerCredentialPlaceholder}
                  />
                  <label style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginTop: "0.7rem" }}>
                    <input
                      type="checkbox"
                      checked={clearCredential}
                      onChange={event => setClearCredential(event.target.checked)}
                    />
                    Clear saved credential
                  </label>
                </>
              )}
              <div className="btn-group" style={{ marginTop: "0.75rem" }}>
                <button className="btn btn-secondary" type="button" onClick={handleSaveConnectorClick} disabled={loading}>
                  <CheckCircle size={16} />
                  Save connector
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onTestPublisherConnection(provider)} disabled={loading || connectorDisabled}>
                  <CheckCircle size={16} />
                  Test connection
                </button>
              </div>
              {publisherTestResult && (
                <p className="help-text">
                  {publisherTestResult.ok ? "Test passed: " : "Test needs attention: "}
                  {publisherTestResult.message}
                </p>
              )}
              <div className="btn-group" style={{ marginTop: "0.75rem" }}>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenExternalUrl(selectedProvider.url)}>
                  <ExternalLink size={16} />
                  Open {selectedProvider.label}
                </button>
                <button className="btn btn-secondary" type="button" onClick={() => onOpenLocalPath(artifactPath(publishResult.zip_path), "site-package.zip")}>
                  <FileArchive size={16} />
                  Open ZIP
                </button>
              </div>
            </div>
          )}

          <div className="card publish-history-card">
            <h3 className="card-title">Publish history</h3>
            {publishHistory.length === 0 ? (
              <p className="help-text">No publish runs recorded yet.</p>
            ) : (
              <div className="publish-history-table" role="table" aria-label="Publish history">
                <div className="publish-history-row publish-history-head" role="row">
                  <span>Issue</span>
                  <span>Provider</span>
                  <span>Articles</span>
                  <span>Generated</span>
                </div>
                {publishHistory.slice(0, 6).map(run => (
                  <div className="publish-history-row" role="row" key={`${run.id}-${run.generated_at}`}>
                    <span title={run.output_path}>{run.issue_id}</span>
                    <span>{run.provider.replace(/_/g, " ")}</span>
                    <span>{run.article_count}</span>
                    <span>{new Date(run.generated_at).toLocaleString()}</span>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
