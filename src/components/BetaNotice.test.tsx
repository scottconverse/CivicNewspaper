// src/components/BetaNotice.test.tsx
// Issue #12 (frontend): the first-run beta notice renders on a clean launch,
// disappears when dismissed, and stays dismissed on the next mount (persisted).
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, beforeEach } from "vitest";
import { BetaNotice } from "./BetaNotice";

describe("BetaNotice (first-run beta notice)", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  test("renders the unsigned-beta / SmartScreen disclosure on first run", () => {
    render(<BetaNotice />);
    expect(screen.getByTestId("beta-notice")).toBeInTheDocument();
    expect(screen.getByText(/unsigned public beta/i)).toBeInTheDocument();
    expect(screen.getByText(/SmartScreen may warn/i)).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Report issues on GitHub/i })).toBeInTheDocument();
  });

  test("dismiss button hides the notice", () => {
    render(<BetaNotice />);
    fireEvent.click(screen.getByTestId("beta-notice-dismiss"));
    expect(screen.queryByTestId("beta-notice")).not.toBeInTheDocument();
  });

  test("stays dismissed across remounts (persisted)", () => {
    const { unmount } = render(<BetaNotice />);
    fireEvent.click(screen.getByTestId("beta-notice-dismiss"));
    unmount();

    render(<BetaNotice />);
    expect(screen.queryByTestId("beta-notice")).not.toBeInTheDocument();
  });
});
