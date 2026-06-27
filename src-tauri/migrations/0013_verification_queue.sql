CREATE TABLE IF NOT EXISTS verification_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dark_signal_id INTEGER REFERENCES dark_signals(id) ON DELETE CASCADE,
    observation_id INTEGER REFERENCES civic_observations(id) ON DELETE SET NULL,
    lead_id INTEGER REFERENCES leads(id) ON DELETE SET NULL,
    draft_id INTEGER REFERENCES drafts(id) ON DELETE SET NULL,
    entity_id INTEGER REFERENCES civic_entities(id) ON DELETE SET NULL,
    check_type TEXT NOT NULL CHECK (check_type IN (
        'source_reachability',
        'official_record_match',
        'entity_lookup',
        'story_decision',
        'evidence_gap'
    )),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    target_label TEXT NOT NULL DEFAULT '',
    target_url TEXT,
    status TEXT NOT NULL DEFAULT 'suggested' CHECK (status IN (
        'suggested',
        'auto_checked',
        'needs_human',
        'blocked',
        'resolved'
    )),
    effort_level TEXT NOT NULL CHECK (effort_level IN ('low', 'medium', 'high')),
    impact_level TEXT NOT NULL CHECK (impact_level IN ('low', 'medium', 'high')),
    rank_score REAL NOT NULL DEFAULT 0,
    result_summary TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(dark_signal_id, check_type, target_label)
);

CREATE INDEX IF NOT EXISTS idx_verification_tasks_status_rank ON verification_tasks(status, rank_score DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_verification_tasks_signal ON verification_tasks(dark_signal_id);
CREATE INDEX IF NOT EXISTS idx_verification_tasks_links ON verification_tasks(lead_id, draft_id, entity_id);
