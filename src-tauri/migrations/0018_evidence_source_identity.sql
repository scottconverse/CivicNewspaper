-- 0018_evidence_source_identity.sql
-- Store the same excerpt when it appears from different sources or URLs.
-- `content_hash` remains the canonical text hash for change detection; duplicate
-- suppression is scoped to the same source and URL so corroborating notices are
-- preserved as separate evidence observations.
CREATE TEMP TABLE IF NOT EXISTS _lead_evidence_0018 AS
SELECT lead_id, evidence_id
FROM lead_evidence;

CREATE TEMP TABLE IF NOT EXISTS _civic_observations_evidence_0018 AS
SELECT id, evidence_id
FROM civic_observations
WHERE evidence_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS evidence_items_v2 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id INTEGER NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    url TEXT,
    fetched_at TEXT NOT NULL,
    excerpt TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    entities TEXT NOT NULL DEFAULT '[]'
);

INSERT OR IGNORE INTO evidence_items_v2 (id, source_id, url, fetched_at, excerpt, content_hash, entities)
SELECT id, source_id, url, fetched_at, excerpt, content_hash, entities
FROM evidence_items;

DROP TABLE evidence_items;
ALTER TABLE evidence_items_v2 RENAME TO evidence_items;

CREATE UNIQUE INDEX IF NOT EXISTS idx_evidence_source_url_content_hash
ON evidence_items(source_id, COALESCE(url, ''), content_hash);

INSERT OR IGNORE INTO lead_evidence (lead_id, evidence_id)
SELECT le.lead_id, le.evidence_id
FROM _lead_evidence_0018 le
WHERE EXISTS (SELECT 1 FROM leads l WHERE l.id = le.lead_id)
  AND EXISTS (SELECT 1 FROM evidence_items e WHERE e.id = le.evidence_id);

UPDATE civic_observations
SET evidence_id = (
    SELECT saved.evidence_id
    FROM _civic_observations_evidence_0018 saved
    WHERE saved.id = civic_observations.id
)
WHERE EXISTS (
    SELECT 1
    FROM _civic_observations_evidence_0018 saved
    JOIN evidence_items e ON e.id = saved.evidence_id
    WHERE saved.id = civic_observations.id
);

DROP TABLE IF EXISTS _lead_evidence_0018;
DROP TABLE IF EXISTS _civic_observations_evidence_0018;
