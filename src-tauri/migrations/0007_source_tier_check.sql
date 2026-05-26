CREATE TABLE sources_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'online',
    last_success_at TEXT,
    last_failed_at TEXT,
    last_scraped TEXT,
    tier TEXT NOT NULL CHECK(tier IN ('official_record', 'news_reporting', 'community_signal')) DEFAULT 'community_signal'
);

INSERT INTO sources_new SELECT id, name, url, type, status, last_success_at, last_failed_at, last_scraped, tier FROM sources;
DROP TABLE sources;
ALTER TABLE sources_new RENAME TO sources;
