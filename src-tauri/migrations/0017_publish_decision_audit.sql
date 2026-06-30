-- 0017_publish_decision_audit.sql
-- Append-only audit trail for publish-advancing editor decisions. The app must
-- never veto the editor's judgment, but the backend must durably record whether
-- the story was attested, whether guardrail warnings existed, and whether the
-- editor supplied an override note.
CREATE TABLE IF NOT EXISTS publish_decision_audits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    draft_id INTEGER NOT NULL,
    decision TEXT NOT NULL,
    attested INTEGER NOT NULL DEFAULT 0,
    guardrail_override_reason TEXT,
    guardrail_issue_count INTEGER NOT NULL DEFAULT 0,
    note TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (draft_id) REFERENCES drafts(id) ON DELETE CASCADE
);
