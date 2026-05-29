// src/components/Modal.test.tsx
import { useState } from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { describe, test, expect, vi } from "vitest";
import { Modal } from "./Modal";

describe("Modal", () => {
  test("exposes dialog semantics wired to its heading", () => {
    render(
      <Modal labelledBy="t" onClose={vi.fn()}>
        <h3 id="t">My Title</h3>
        <button>One</button>
      </Modal>
    );
    const dialog = screen.getByRole("dialog");
    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(dialog).toHaveAttribute("aria-labelledby", "t");
    expect(dialog).toHaveAccessibleName("My Title");
  });

  test("focuses the first focusable element on open", () => {
    render(
      <Modal labelledBy="t" onClose={vi.fn()}>
        <h3 id="t">Title</h3>
        <button>First</button>
        <button>Second</button>
      </Modal>
    );
    expect(screen.getByRole("button", { name: "First" })).toHaveFocus();
  });

  test("Escape calls onClose", () => {
    const onClose = vi.fn();
    render(
      <Modal labelledBy="t" onClose={onClose}>
        <h3 id="t">Title</h3>
        <button>OK</button>
      </Modal>
    );
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  test("Tab from the last focusable wraps to the first", () => {
    render(
      <Modal labelledBy="t" onClose={vi.fn()}>
        <h3 id="t">Title</h3>
        <button>First</button>
        <button>Last</button>
      </Modal>
    );
    const first = screen.getByRole("button", { name: "First" });
    screen.getByRole("button", { name: "Last" }).focus();
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Tab" });
    expect(first).toHaveFocus();
  });

  test("Shift+Tab from the first focusable wraps to the last", () => {
    render(
      <Modal labelledBy="t" onClose={vi.fn()}>
        <h3 id="t">Title</h3>
        <button>First</button>
        <button>Last</button>
      </Modal>
    );
    const last = screen.getByRole("button", { name: "Last" });
    screen.getByRole("button", { name: "First" }).focus();
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Tab", shiftKey: true });
    expect(last).toHaveFocus();
  });

  test("restores focus to the trigger after closing", () => {
    const Harness = () => {
      const [open, setOpen] = useState(false);
      return (
        <>
          <button onClick={() => setOpen(true)}>Trigger</button>
          {open && (
            <Modal labelledBy="t" onClose={() => setOpen(false)}>
              <h3 id="t">Title</h3>
              <button>Inside</button>
            </Modal>
          )}
        </>
      );
    };
    render(<Harness />);
    const trigger = screen.getByRole("button", { name: "Trigger" });
    trigger.focus();
    fireEvent.click(trigger);
    expect(screen.getByRole("button", { name: "Inside" })).toHaveFocus();
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(trigger).toHaveFocus();
  });

  test("locks body scroll while open and restores it on close", () => {
    const Harness = () => {
      const [open, setOpen] = useState(true);
      return open ? (
        <Modal labelledBy="t" onClose={() => setOpen(false)}>
          <h3 id="t">Title</h3>
          <button>Close</button>
        </Modal>
      ) : null;
    };
    render(<Harness />);
    expect(document.body.style.overflow).toBe("hidden");
    fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(document.body.style.overflow).toBe("");
  });
});
