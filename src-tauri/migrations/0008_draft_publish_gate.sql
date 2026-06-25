-- 0008_draft_publish_gate.sql
-- GG-B2 / GG-C1 (editorial trust boundary). Adds the publish-gate columns to
-- `drafts`:
--   attested_by / attested_at  -- the human who confirmed they verified the
--                                 draft against its cited evidence before it was
--                                 approved for publishing (GG-C1).
--   guardrail_override_reason  -- an explicit, logged reason for publishing a
--                                 draft despite error-severity guardrail issues
--                                 (GG-B2).
-- All three are nullable with no default, so existing rows and the existing
-- Draft SELECT/INSERT statements (which do not reference these columns) are
-- unaffected.
ALTER TABLE drafts ADD COLUMN attested_by TEXT;
ALTER TABLE drafts ADD COLUMN attested_at TEXT;
ALTER TABLE drafts ADD COLUMN guardrail_override_reason TEXT;
