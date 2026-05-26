CREATE TABLE daily_scan_leads_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    source_id INTEGER,
    original_url TEXT NOT NULL,
    FOREIGN KEY(scan_id) REFERENCES daily_scan_runs(id),
    FOREIGN KEY(source_id) REFERENCES sources(id)
);

INSERT INTO daily_scan_leads_new SELECT * FROM daily_scan_leads;
DROP TABLE daily_scan_leads;
ALTER TABLE daily_scan_leads_new RENAME TO daily_scan_leads;
