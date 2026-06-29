CREATE TABLE IF NOT EXISTS beat_memory (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic_key TEXT NOT NULL UNIQUE,
    representative_title TEXT NOT NULL,
    source_url TEXT NOT NULL DEFAULT '',
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    seen_count INTEGER NOT NULL DEFAULT 1,
    last_scan_lead_id INTEGER REFERENCES daily_scan_leads(id) ON DELETE SET NULL,
    last_summary TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_beat_memory_last_seen ON beat_memory(last_seen_at DESC);
