// src/components/PublishPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { PublishPanel } from "./PublishPanel";

const defaultPublisherProps = {
  publishHistory: [],
  publisherConfig: null,
  publisherProvider: "here_now",
  publisherTestResult: null,
  subscribers: [],
  subscriberEmail: "",
  subscriberName: "",
  onSubscriberEmailChange: vi.fn(),
  onSubscriberNameChange: vi.fn(),
  onPublishWithConnector: vi.fn(),
  onLoadPublisherConfig: vi.fn(),
  onSavePublisherConfig: vi.fn(),
  onTestPublisherConnection: vi.fn(),
  onAddSubscriber: vi.fn(),
  onDeleteSubscriber: vi.fn(),
  onImportSubscribersCsv: vi.fn(),
  onExportSubscribersCsv: vi.fn(),
  onExportIssueEmail: vi.fn(),
  onCopyPublishText: vi.fn(),
  onCopyPublishArtifact: vi.fn(),
  approvedDraftCount: 1,
  communityProfile: {
    site_title: "Test Publication",
    site_subtitle: "Local news and community information.",
    about_text: "About this publication.",
    ethics_text: "Editorial standards are set by the publisher.",
    how_we_report_text: "Stories are reviewed before publication.",
    organization_type: "single_person",
    footer_text: "",
    logo_url: "",
    accent_color: "#5a1818",
    layout_style: "classic",
    first_amendment_advisor_enabled: true,
    money_threshold: 50000,
    watchlist: [],
    city: "Longmont",
    state: "CO",
  },
};

describe("PublishPanel Component Tests", () => {
  test("shows validation error on empty path when moving to step 2", () => {
    const handleStepChange = vi.fn();

    render(
      <PublishPanel
        publishPath=""
        publishResult={null}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={1}
        onPublishStepChange={handleStepChange}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    const nextBtn = screen.getByRole("button", { name: /Review compile checklist/i });
    fireEvent.click(nextBtn);

    // Expect validation error to appear and no state change to be triggered
    expect(screen.getByTestId("validation-error")).toBeInTheDocument();
    expect(screen.getByText("Output path cannot be empty.")).toBeInTheDocument();
    expect(handleStepChange).not.toHaveBeenCalled();
  });

  test("fires the compile action on valid path at step 2", () => {
    const handlePublish = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={null}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={2}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={handlePublish}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    const compileBtn = screen.getByRole("button", { name: /Compile site/i });
    fireEvent.click(compileBtn);

    // Verify error is empty and action is triggered
    expect(screen.queryByTestId("validation-error")).not.toBeInTheDocument();
    expect(handlePublish).toHaveBeenCalled();
  });

  test("requires a real publication name before compile", () => {
    const handlePublish = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={null}
        {...defaultPublisherProps}
        communityProfile={{
          ...defaultPublisherProps.communityProfile,
          site_title: "My Local Publication",
        }}
        onPublishPathChange={vi.fn()}
        publishStep={2}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={handlePublish}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: /Compile site/i }));

    expect(screen.getByTestId("validation-error")).toHaveTextContent(
      "Choose and save a real publication name before compiling or publishing."
    );
    expect(handlePublish).not.toHaveBeenCalled();
  });

  test("browse calls the folder picker handler", () => {
    const handleChoosePublishPath = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={null}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={1}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={handleChoosePublishPath}
        onRecordPublishDestination={vi.fn()}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: /Browse/i }));
    expect(handleChoosePublishPath).toHaveBeenCalledTimes(1);
  });

  test("shows compile receipt artifacts after publish succeeds", () => {
    const handleOpen = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 2,
          skipped_count: 1,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={handleOpen}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByLabelText("Compile receipt")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();
    expect(screen.getByText("12")).toBeInTheDocument();
    expect(screen.getAllByText("1").length).toBeGreaterThanOrEqual(1);

    fireEvent.click(screen.getByRole("button", { name: /ZIP package/i }));
    expect(handleOpen).toHaveBeenCalledWith("C:\\my-site\\site-package.zip", "site-package.zip");
  });

  test("does not mark host publish or resident sharing complete until a public URL is saved", () => {
    const publishResult = {
      issue_id: "issue-20260627-000000",
      output_dir: "C:/my-site",
      generated_at: "2026-06-27T00:00:00Z",
      provider: "local_export",
      published_url: null,
      deployment_id: null,
      article_count: 1,
      skipped_count: 0,
      files_written: 12,
      generated_files: [],
      index_path: "index.html",
      rss_path: "feed.xml",
      newsletter_path: "newsletter.md",
      substack_path: "substack.md",
      share_package_path: "share-package.md",
      facebook_post_path: "facebook-post.txt",
      subreddit_post_path: "subreddit-post.md",
      nextdoor_post_path: "nextdoor-post.txt",
      short_link_blurb_path: "short-link-blurb.txt",
      manifest_path: "publish-manifest.json",
      zip_path: "site-package.zip",
      articles: [],
    };

    const { rerender } = render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={publishResult}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByText("Publish to a host").parentElement).toHaveTextContent("pending");
    expect(screen.getByText("Share with residents").parentElement).toHaveTextContent("pending");

    rerender(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{ ...publishResult, published_url: "https://example.org/civic" }}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByText("Publish to a host").parentElement).toHaveTextContent("live URL");
    expect(screen.getByText("Share with residents").parentElement).toHaveTextContent("posts");
  });

  test("records a public publish destination after compile", () => {
    const handleRecord = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={handleRecord}
      />
    );

    fireEvent.change(screen.getByLabelText(/Provider/i), { target: { value: "github_pages" } });
    expect(screen.getByLabelText(/Folder path/i)).toHaveAttribute("placeholder", "leave blank for root, or use docs");
    fireEvent.change(screen.getByLabelText(/^Public URL$/i), {
      target: { value: "https://example.org/civic" },
    });
    fireEvent.change(screen.getByLabelText(/Deployment ID or note/i), {
      target: { value: "manual-pages" },
    });
    fireEvent.click(screen.getByRole("button", { name: /Save public URL/i }));

    expect(handleRecord).toHaveBeenCalledWith("github_pages", "https://example.org/civic", "manual-pages");
  });

  test("blocks saving a public URL when publication name is still starter text", () => {
    const handleRecord = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        communityProfile={{
          ...defaultPublisherProps.communityProfile,
          site_title: "My Local Publication",
        }}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={handleRecord}
      />
    );

    fireEvent.change(screen.getByLabelText(/^Public URL$/i), {
      target: { value: "https://example.org/civic" },
    });
    fireEvent.click(screen.getByRole("button", { name: /Save public URL/i }));

    expect(screen.getByTestId("validation-error")).toHaveTextContent(
      "Choose and save a real publication name before compiling or publishing."
    );
    expect(handleRecord).not.toHaveBeenCalled();
  });

  test("allows anonymous here.now publish before connector test but gates credential connectors", () => {
    const handlePublishWithConnector = vi.fn();
    const publishResult = {
      issue_id: "issue-20260627-000000",
      output_dir: "C:/my-site",
      generated_at: "2026-06-27T00:00:00Z",
      provider: "local_export",
      published_url: null,
      deployment_id: null,
      article_count: 1,
      skipped_count: 0,
      files_written: 12,
      generated_files: [],
      index_path: "index.html",
      rss_path: "feed.xml",
      newsletter_path: "newsletter.md",
      substack_path: "substack.md",
      share_package_path: "share-package.md",
      facebook_post_path: "facebook-post.txt",
      subreddit_post_path: "subreddit-post.md",
      nextdoor_post_path: "nextdoor-post.txt",
      short_link_blurb_path: "short-link-blurb.txt",
      manifest_path: "publish-manifest.json",
      zip_path: "site-package.zip",
      articles: [],
    };

    const { rerender } = render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={publishResult}
        {...defaultPublisherProps}
        onPublishWithConnector={handlePublishWithConnector}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByRole("button", { name: /Publish to here.now/i })).toBeEnabled();
    expect(screen.getByText(/temporary anonymous preview/i)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /Publish to here.now/i }));
    expect(handlePublishWithConnector).toHaveBeenCalledWith("here_now", "", "");

    fireEvent.change(screen.getByLabelText(/here.now slug/i), {
      target: { value: "town-civic-paper" },
    });
    expect(screen.getByRole("button", { name: /Publish to here.now/i })).toBeDisabled();
    expect(screen.getByText(/account-owned here.now target/i)).toBeInTheDocument();

    rerender(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={publishResult}
        {...defaultPublisherProps}
        publisherConfig={{
          provider: "here_now",
          display_name: "Town here.now",
          site_url: null,
          project_hint: null,
          site_id: "town-civic-paper",
          account_id: null,
          repo: null,
          branch: null,
          path_prefix: null,
          username: null,
          has_credential: true,
        }}
        publisherTestResult={{
          provider: "here_now",
          ok: true,
          message: "here.now accepted the saved API key.",
          credential_checked: true,
        }}
        onPublishWithConnector={handlePublishWithConnector}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByRole("button", { name: /Publish to here.now/i })).toBeEnabled();

    fireEvent.change(screen.getByLabelText(/Provider/i), { target: { value: "netlify" } });
    expect(screen.getByRole("button", { name: /Publish with connector/i })).toBeDisabled();
    expect(screen.getByText(/Test the selected connector before publishing/i)).toBeInTheDocument();
  });

  test("Cloudflare is assisted/manual in the beta, not an active API connector", () => {
    const handlePublishWithConnector = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        publisherProvider="cloudflare_pages"
        onPublishWithConnector={handlePublishWithConnector}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByRole("button", { name: /Publish with connector/i })).toBeDisabled();
    expect(screen.getByRole("button", { name: /Test connection/i })).toBeDisabled();
    expect(screen.getByText(/Cloudflare API publishing is disabled in this public beta/i)).toBeInTheDocument();
  });

  test("blocks compile when no approved stories are ready", () => {
    const handlePublish = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={null}
        {...defaultPublisherProps}
        approvedDraftCount={0}
        onPublishPathChange={vi.fn()}
        publishStep={2}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={handlePublish}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByLabelText(/Approved story readiness/i)).toHaveTextContent("0 approved stories");
    expect(screen.getByRole("button", { name: /Compile site/i })).toBeDisabled();
    expect(handlePublish).not.toHaveBeenCalled();
  });

  test("saves connector config and tests connection", () => {
    const handleSave = vi.fn();
    const handleTest = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        publisherConfig={null}
        publisherTestResult={{
          provider: "netlify",
          ok: true,
          message: "Netlify accepted the site ID and API token.",
          credential_checked: true,
        }}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
        onSavePublisherConfig={handleSave}
        onTestPublisherConnection={handleTest}
      />
    );

    expect(screen.getByText(/Test passed:/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/Connector setup guide/i)).toHaveTextContent("24-hour preview");
    expect(screen.getByLabelText(/Connector setup guide/i)).toHaveTextContent("existing here.now slug");
    fireEvent.change(screen.getByLabelText(/here.now slug/i), {
      target: { value: "town-civic-paper" },
    });
    fireEvent.change(screen.getByLabelText(/here.now API key/i), {
      target: { value: "hnk_test" },
    });
    fireEvent.change(screen.getByLabelText(/Connector name/i), {
      target: { value: "Town here.now" },
    });
    fireEvent.click(screen.getByRole("button", { name: /Save connector/i }));
    expect(handleSave).toHaveBeenCalledWith(expect.objectContaining({
      provider: "here_now",
      display_name: "Town here.now",
      site_id: "town-civic-paper",
      credential: "hnk_test",
      clear_credential: false,
    }));
    fireEvent.click(screen.getByRole("button", { name: /Test connection/i }));
    expect(handleTest).toHaveBeenCalledWith("here_now");
  });

  test("shows provider-specific setup guidance when switching providers", () => {
    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        {...defaultPublisherProps}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    fireEvent.change(screen.getByLabelText(/Provider/i), { target: { value: "wordpress" } });

    expect(screen.getByLabelText(/Connector setup guide/i)).toHaveTextContent("WordPress application password");
    expect(screen.getByLabelText(/Connector setup guide/i)).toHaveTextContent("publish pages");
  });

  test("renders publish history", () => {
    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={null}
        {...defaultPublisherProps}
        publishHistory={[{
          id: 1,
          issue_id: "issue-20260627-000000",
          output_path: "C:/my-site",
          generated_files: "[]",
          provider: "github_pages",
          published_url: "https://example.org/civic",
          deployment_id: "pages-42",
          article_count: 3,
          skipped_count: 0,
          files_written: 12,
          generated_at: "2026-06-27T00:00:00Z",
        }]}
        onPublishPathChange={vi.fn()}
        publishStep={1}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    expect(screen.getByRole("table", { name: /Publish history/i })).toBeInTheDocument();
    expect(screen.getByText("issue-20260627-000000")).toBeInTheDocument();
    expect(screen.getByText("github pages")).toBeInTheDocument();
  });

  test("supports Substack copy actions and subscriber controls", () => {
    const copyText = vi.fn();
    const copyArtifact = vi.fn();
    const addSubscriber = vi.fn();
    const deleteSubscriber = vi.fn();

    render(
      <PublishPanel
        publishPath={"C:\\my-site"}
        publishResult={{
          issue_id: "issue-20260627-000000",
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          provider: "local_export",
          published_url: null,
          deployment_id: null,
          article_count: 1,
          skipped_count: 0,
          files_written: 12,
          generated_files: [],
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          substack_path: "substack.md",
          share_package_path: "share-package.md",
          facebook_post_path: "facebook-post.txt",
          subreddit_post_path: "subreddit-post.md",
          nextdoor_post_path: "nextdoor-post.txt",
          short_link_blurb_path: "short-link-blurb.txt",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [{
            title: "Council weighs budget change",
            format: "watch",
            relative_path: "articles/council.html",
            updated_at: "2026-06-27T00:00:00Z",
          }],
        }}
        {...defaultPublisherProps}
        subscribers={[{
          id: 7,
          email: "reader@example.com",
          name: "Reader One",
          status: "active",
          created_at: "2026-06-27T00:00:00Z",
          updated_at: "2026-06-27T00:00:00Z",
        }]}
        subscriberEmail="new@example.com"
        subscriberName="New Reader"
        onAddSubscriber={addSubscriber}
        onDeleteSubscriber={deleteSubscriber}
        onCopyPublishText={copyText}
        onCopyPublishArtifact={copyArtifact}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onOpenExternalUrl={vi.fn()}
        onChoosePublishPath={vi.fn()}
        onRecordPublishDestination={vi.fn()}
      />
    );

    fireEvent.click(screen.getByRole("button", { name: /Copy headline/i }));
    expect(copyText).toHaveBeenCalledWith("Substack headline", "Council weighs budget change");
    fireEvent.click(screen.getByRole("button", { name: /Copy post body/i }));
    expect(copyArtifact).toHaveBeenCalledWith("Substack body", "substack.md");
    fireEvent.click(screen.getByRole("button", { name: /^Add$/i }));
    expect(addSubscriber).toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: /Remove reader@example.com/i }));
    expect(deleteSubscriber).toHaveBeenCalledWith(7);
  });
});
