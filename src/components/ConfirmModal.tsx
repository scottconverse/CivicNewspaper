// src/components/ConfirmModal.tsx
import React from "react";
import { Modal } from "./Modal";

interface ConfirmModalProps {
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

// Styled, accessible replacement for native window.confirm(). Renders inside the
// shared Modal shell so destructive actions get a focus-trapped dialog with
// consequence-specific copy instead of a browser-chrome prompt.
export const ConfirmModal: React.FC<ConfirmModalProps> = ({
  title,
  message,
  confirmLabel = "Confirm",
  cancelLabel = "Cancel",
  danger = false,
  onConfirm,
  onCancel,
}) => {
  return (
    <Modal
      id="confirm-modal"
      labelledBy="confirm-modal-title"
      onClose={onCancel}
    >
      <h3 id="confirm-modal-title" style={{ marginTop: 0 }}>{title}</h3>
      <p className="help-text" style={{ marginTop: 0 }}>{message}</p>
      <div className="flex-between" style={{ marginTop: "1.5rem", justifyContent: "flex-end", gap: "0.5rem" }}>
        <button type="button" className="btn btn-secondary" onClick={onCancel}>
          {cancelLabel}
        </button>
        <button
          type="button"
          className={danger ? "btn btn-danger" : "btn btn-primary"}
          onClick={onConfirm}
          data-testid="confirm-modal-confirm"
        >
          {confirmLabel}
        </button>
      </div>
    </Modal>
  );
};
