// src/components/PublishPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { PublishPanel } from "./PublishPanel";

const defaultPublisherProps = {
  publishHistory: [],
  publisherConfig: null,
  publisherProvider: "netlify",
  publisherTestResult: null,
  onPublishWithConnector: vi.fn(),
  onLoadPublisherConfig: vi.fn(),
  onSavePublisherConfig: vi.fn(),
  onTestPublisherConnection: vi.fn(),
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
    expect(screen.getByText("1")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /ZIP package/i }));
    expect(handleOpen).toHaveBeenCalledWith("C:\\my-site\\site-package.zip");
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
    fireEvent.change(screen.getByLabelText(/^Public URL$/i), {
      target: { value: "https://example.org/civic" },
    });
    fireEvent.change(screen.getByLabelText(/Deployment ID or note/i), {
      target: { value: "manual-pages" },
    });
    fireEvent.click(screen.getByRole("button", { name: /Save public URL/i }));

    expect(handleRecord).toHaveBeenCalledWith("github_pages", "https://example.org/civic", "manual-pages");
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
        publisherConfig={{
          provider: "netlify",
          display_name: "Town Netlify",
          site_url: "https://town.example",
          project_hint: "town-site",
          site_id: "site-123",
          has_credential: true,
        }}
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
    fireEvent.change(screen.getByLabelText(/Connector name/i), {
      target: { value: "Updated Netlify" },
    });
    fireEvent.click(screen.getByRole("button", { name: /Save connector/i }));
    expect(handleSave).toHaveBeenCalledWith(expect.objectContaining({
      provider: "netlify",
      display_name: "Updated Netlify",
      site_url: "https://town.example",
      project_hint: "town-site",
      site_id: "site-123",
      clear_credential: false,
    }));
    fireEvent.click(screen.getByRole("button", { name: /Test connection/i }));
    expect(handleTest).toHaveBeenCalledWith("netlify");
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
});
