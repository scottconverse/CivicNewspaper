CREATE TABLE IF NOT EXISTS daily_scan_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    run_status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS daily_scan_leads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    source_id INTEGER NOT NULL,
    original_url TEXT NOT NULL,
    FOREIGN KEY(scan_id) REFERENCES daily_scan_runs(id),
    FOREIGN KEY(source_id) REFERENCES sources(id)
);

ALTER TABLE leads ADD COLUMN from_scan_lead_id INTEGER;
