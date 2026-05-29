// src/components/Modal.tsx
import React, { useEffect, useRef } from "react";

const FOCUSABLE_SELECTOR =
  'a[href], button:not([disabled]), textarea:not([disabled]), input:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])';

interface ModalProps {
  // Called on Escape. The parent owns open/closed state and unmounts the modal.
  onClose: () => void;
  // id of the heading element inside `children`, wired to aria-labelledby.
  labelledBy: string;
  // Optional overlay id, preserved for existing test/query selectors.
  id?: string;
  // Extra classes appended to the base "modal-content".
  contentClassName?: string;
  contentStyle?: React.CSSProperties;
  children: React.ReactNode;
}

// Accessible dialog shell: owns role="dialog" + aria-modal + aria-labelledby,
// Esc-to-close, a Tab focus trap, body scroll-lock, initial focus on the first
// focusable child, and focus restoration to the trigger on unmount. Parents
// mount it only while open, so mount == open and unmount == close.
export const Modal: React.FC<ModalProps> = ({
  onClose,
  labelledBy,
  id,
  contentClassName,
  contentStyle,
  children,
}) => {
  const contentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const previouslyFocused = document.activeElement as HTMLElement | null;
    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = "hidden";

    const focusTarget =
      contentRef.current?.querySelector<HTMLElement>(FOCUSABLE_SELECTOR) ||
      contentRef.current;
    focusTarget?.focus();

    return () => {
      document.body.style.overflow = previousOverflow;
      previouslyFocused?.focus?.();
    };
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Escape") {
      e.stopPropagation();
      onClose();
      return;
    }
    if (e.key !== "Tab") return;

    const content = contentRef.current;
    if (!content) return;
    const focusables = Array.from(
      content.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)
    );
    if (focusables.length === 0) {
      e.preventDefault();
      return;
    }
    const first = focusables[0];
    const last = focusables[focusables.length - 1];
    const active = document.activeElement as HTMLElement;
    if (e.shiftKey && active === first) {
      e.preventDefault();
      last.focus();
    } else if (!e.shiftKey && active === last) {
      e.preventDefault();
      first.focus();
    }
  };

  return (
    <div
      className="modal-overlay"
      id={id}
      role="dialog"
      aria-modal="true"
      aria-labelledby={labelledBy}
      onKeyDown={handleKeyDown}
    >
      <div
        ref={contentRef}
        className={contentClassName ? `modal-content ${contentClassName}` : "modal-content"}
        style={contentStyle}
        tabIndex={-1}
      >
        {children}
      </div>
    </div>
  );
};
