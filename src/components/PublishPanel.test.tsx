// src/components/PublishPanel.test.tsx
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { PublishPanel } from "./PublishPanel";

describe("PublishPanel Component Tests", () => {
  test("shows validation error on empty path when moving to step 2", () => {
    const handleStepChange = vi.fn();

    render(
      <PublishPanel
        publishPath=""
        publishResult={null}
        onPublishPathChange={vi.fn()}
        publishStep={1}
        onPublishStepChange={handleStepChange}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onChoosePublishPath={vi.fn()}
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
        onPublishPathChange={vi.fn()}
        publishStep={2}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={handlePublish}
        onOpenLocalPath={vi.fn()}
        onChoosePublishPath={vi.fn()}
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
        onPublishPathChange={vi.fn()}
        publishStep={1}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={vi.fn()}
        onChoosePublishPath={handleChoosePublishPath}
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
          output_dir: "C:/my-site",
          generated_at: "2026-06-27T00:00:00Z",
          article_count: 2,
          skipped_count: 1,
          files_written: 12,
          index_path: "index.html",
          rss_path: "feed.xml",
          newsletter_path: "newsletter.md",
          share_package_path: "share-package.md",
          manifest_path: "publish-manifest.json",
          zip_path: "site-package.zip",
          articles: [],
        }}
        onPublishPathChange={vi.fn()}
        publishStep={3}
        onPublishStepChange={vi.fn()}
        loading={false}
        onPublish={vi.fn()}
        onOpenLocalPath={handleOpen}
        onChoosePublishPath={vi.fn()}
      />
    );

    expect(screen.getByLabelText("Compile receipt")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();
    expect(screen.getByText("12")).toBeInTheDocument();
    expect(screen.getByText("1")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /ZIP package/i }));
    expect(handleOpen).toHaveBeenCalledWith("C:\\my-site\\site-package.zip");
  });
});
