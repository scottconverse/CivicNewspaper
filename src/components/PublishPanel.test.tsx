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
        publishPath="C:\my-site"
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
        publishPath="C:\\my-site"
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
});
