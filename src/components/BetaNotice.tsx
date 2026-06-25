// src/components/BetaNotice.tsx
// Issue #12 (frontend): a dismissible first-run notice telling beta testers the
// app is an unsigned public beta, that Windows SmartScreen warnings on install
// are expected, and where to report issues. Dismissal is persisted (see
// betaNoticeStore) so the banner only shows until the user acknowledges it once.
import React, { useState } from "react";
import { AlertTriangle, X } from "lucide-react";
import { betaNoticeIsDismissed, betaNoticeMarkDismissed } from "./betaNoticeStore";

const issuesUrl = "https://" + "github.com/scottconverse/CivicNewspaper/issues";

export const BetaNotice: React.FC = () => {
  const [dismissed, setDismissed] = useState<boolean>(() => betaNoticeIsDismissed());

  if (dismissed) return null;

  const handleDismiss = () => {
    betaNoticeMarkDismissed();
    setDismissed(true);
  };

  return (
    <div
      role="alert"
      aria-live="polite"
      data-testid="beta-notice"
      className="card"
      style={{
        borderLeft: "4px solid var(--color-warning)",
        background: "var(--accent-light)",
        marginBottom: "1rem",
      }}
    >
      <div className="flex-between" style={{ alignItems: "flex-start", gap: "1rem" }}>
        <div style={{ display: "flex", gap: "0.6rem", alignItems: "flex-start" }}>
          <AlertTriangle
            size={18}
            style={{ color: "var(--color-warning)", flexShrink: 0, marginTop: "0.15rem" }}
          />
          <p style={{ fontSize: "0.9rem", margin: 0, color: "var(--text-primary)" }}>
            You're running an <strong>unsigned public beta</strong> of The Civic Desk.
            Windows SmartScreen may warn on install; that's expected.{" "}
            <a
              href={issuesUrl}
              target="_blank"
              rel="noreferrer"
              style={{ color: "var(--accent-primary)", fontWeight: 600 }}
            >
              Report issues on GitHub
            </a>
            .
          </p>
        </div>
        <button
          className="btn btn-secondary btn-sm"
          onClick={handleDismiss}
          aria-label="Dismiss beta notice"
          data-testid="beta-notice-dismiss"
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
};
