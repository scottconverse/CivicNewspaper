CREATE TABLE IF NOT EXISTS civic_observations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    observation_type TEXT NOT NULL CHECK (observation_type IN (
        'source_fetched',
        'document_changed',
        'agenda_item_found',
        'video_posted',
        'entity_detected',
        'social_signal_found'
    )),
    source_id INTEGER REFERENCES sources(id) ON DELETE SET NULL,
    evidence_id INTEGER REFERENCES evidence_items(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    url TEXT,
    observed_at TEXT NOT NULL,
    content_hash TEXT,
    previous_hash TEXT,
    diff_summary TEXT,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    tier TEXT NOT NULL DEFAULT 'official_record' CHECK (tier IN ('official_record', 'news_reporting', 'community_signal'))
);

CREATE TABLE IF NOT EXISTS civic_entities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL CHECK (entity_type IN (
        'person',
        'company',
        'parcel',
        'address',
        'vendor',
        'agency',
        'money',
        'unknown'
    )),
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL,
    mention_count INTEGER NOT NULL DEFAULT 1,
    UNIQUE(entity_type, normalized_name)
);

CREATE TABLE IF NOT EXISTS civic_observation_entities (
    observation_id INTEGER NOT NULL REFERENCES civic_observations(id) ON DELETE CASCADE,
    entity_id INTEGER NOT NULL REFERENCES civic_entities(id) ON DELETE CASCADE,
    context TEXT,
    PRIMARY KEY (observation_id, entity_id)
);

CREATE TABLE IF NOT EXISTS source_performance_scores (
    source_id INTEGER PRIMARY KEY REFERENCES sources(id) ON DELETE CASCADE,
    fetch_successes INTEGER NOT NULL DEFAULT 0,
    fetch_failures INTEGER NOT NULL DEFAULT 0,
    new_items INTEGER NOT NULL DEFAULT 0,
    changed_items INTEGER NOT NULL DEFAULT 0,
    entity_hits INTEGER NOT NULL DEFAULT 0,
    dark_signal_hits INTEGER NOT NULL DEFAULT 0,
    reliability_score REAL NOT NULL DEFAULT 0,
    usefulness_score REAL NOT NULL DEFAULT 0,
    last_fetch_at TEXT,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dark_signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    observation_id INTEGER REFERENCES civic_observations(id) ON DELETE SET NULL,
    source_id INTEGER REFERENCES sources(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    summary TEXT NOT NULL,
    origin TEXT NOT NULL,
    risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
    rank_score REAL NOT NULL,
    tier TEXT NOT NULL DEFAULT 'community_signal',
    evidence_policy TEXT NOT NULL DEFAULT 'editor_review_only',
    why_it_matters TEXT NOT NULL,
    verification_path TEXT NOT NULL,
    publication_status TEXT NOT NULL DEFAULT 'review' CHECK (publication_status IN ('review', 'verifying', 'ready_for_story', 'dismissed', 'published')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_civic_observations_type_time ON civic_observations(observation_type, observed_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_observations_source_time ON civic_observations(source_id, observed_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_entities_name ON civic_entities(normalized_name);
CREATE INDEX IF NOT EXISTS idx_dark_signals_rank ON dark_signals(publication_status, rank_score DESC, created_at DESC);
