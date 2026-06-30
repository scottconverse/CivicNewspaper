import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { AiModelPanel } from "./AiModelPanel";

describe("AiModelPanel", () => {
  it("shows recovery actions when the local AI service is offline", () => {
    const retry = vi.fn();
    const openSystem = vi.fn();
    const installRuntime = vi.fn();

    render(
      <AiModelPanel
        ollamaOnline={false}
        systemRam={16}
        wizardModel="phi4-mini:latest"
        installedModels={[]}
        onWizardModelChange={vi.fn()}
        pullingModel={false}
        pullProgressText={[]}
        onInstallRuntime={installRuntime}
        onPullModel={vi.fn()}
        onRetryStatus={retry}
        onOpenSystem={openSystem}
      />
    );

    screen.getByRole("button", { name: "Install local AI runtime" }).click();
    screen.getByRole("button", { name: "Retry AI check" }).click();
    screen.getByRole("button", { name: "Open System Status" }).click();

    expect(installRuntime).toHaveBeenCalledTimes(1);
    expect(retry).toHaveBeenCalledTimes(1);
    expect(openSystem).toHaveBeenCalledTimes(1);
  });

  it("exposes model download progress semantically", () => {
    render(
      <AiModelPanel
        ollamaOnline={true}
        systemRam={16}
        wizardModel="phi4-mini:latest"
        installedModels={[]}
        onWizardModelChange={vi.fn()}
        pullingModel={true}
        pullProgressText={["downloading (42%)"]}
        onInstallRuntime={vi.fn()}
        onPullModel={vi.fn()}
        onRetryStatus={vi.fn()}
        onOpenSystem={vi.fn()}
      />
    );

    expect(screen.getByRole("progressbar", { name: "AI model download progress" })).toHaveAttribute("aria-valuenow", "42");
    expect(screen.getByRole("status")).toHaveTextContent("downloading");
  });
});
