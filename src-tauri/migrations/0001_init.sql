-- 0001_init.sql
-- SQLite Database Schema for CivicNews

CREATE TABLE IF NOT EXISTS sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL, -- 'primary_record', 'official_comm', 'community_signal', 'media_lead'
    status TEXT NOT NULL DEFAULT 'online', -- 'online', 'offline'
    last_success_at TEXT,
    last_failed_at TEXT,
    last_scraped TEXT
);

CREATE TABLE IF NOT EXISTS evidence_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id INTEGER NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
    url TEXT,
    fetched_at TEXT NOT NULL,
    excerpt TEXT NOT NULL,
    content_hash TEXT NOT NULL UNIQUE,
    entities TEXT NOT NULL DEFAULT '[]' -- JSON array of strings
);

CREATE TABLE IF NOT EXISTS leads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    detector_name TEXT NOT NULL,
    why TEXT NOT NULL,
    confidence TEXT NOT NULL, -- 'low', 'med', 'high'
    risk_level TEXT NOT NULL DEFAULT 'low', -- 'low', 'med', 'high'
    confirmation_checklist TEXT NOT NULL DEFAULT '[]', -- JSON array of objects
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lead_evidence (
    lead_id INTEGER NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    evidence_id INTEGER NOT NULL REFERENCES evidence_items(id) ON DELETE CASCADE,
    PRIMARY KEY (lead_id, evidence_id)
);

CREATE TABLE IF NOT EXISTS drafts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lead_id INTEGER REFERENCES leads(id) ON DELETE SET NULL,
    format TEXT NOT NULL, -- 'brief', 'watch', 'explainer', 'investigation', 'opinion'
    title TEXT NOT NULL,
    content TEXT NOT NULL, -- Markdown with source citations
    status TEXT NOT NULL DEFAULT 'lead', -- 'lead', 'draft_generated', 'ready_to_review', 'needs_verification', 'hold', 'killed', 'published', 'corrected'
    verification_checklist TEXT NOT NULL DEFAULT '[]', -- JSON array
    missing_evidence_notes TEXT,
    correction_note TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS published_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    draft_id INTEGER NOT NULL REFERENCES drafts(id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    url TEXT NOT NULL,
    published_at TEXT NOT NULL,
    correction_history TEXT NOT NULL DEFAULT '[]' -- JSON array of corrections
);

CREATE TABLE IF NOT EXISTS paired_clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL,
    pairing_pin TEXT, -- 6-digit short-lived PIN
    pin_expires_at TEXT, -- Timestamp
    created_at TEXT NOT NULL,
    last_used_at TEXT,
    revoked INTEGER NOT NULL DEFAULT 0 -- 0=false, 1=true
);
