CREATE TABLE daily_scan_runs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_date TEXT NOT NULL,
  city TEXT NOT NULL,
  state TEXT NOT NULL,
  model_used TEXT NOT NULL,
  prompt_id TEXT NOT NULL,
  raw_response TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE daily_scan_leads (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id INTEGER NOT NULL REFERENCES daily_scan_runs(id) ON DELETE CASCADE,
  rank INTEGER NOT NULL,
  tier TEXT NOT NULL,
  headline TEXT NOT NULL,
  details TEXT NOT NULL,
  source TEXT,
  url TEXT,
  confidence TEXT,
  action TEXT,
  beat TEXT
);
ALTER TABLE leads ADD COLUMN from_scan_lead_id INTEGER REFERENCES daily_scan_leads(id) ON DELETE SET NULL;
