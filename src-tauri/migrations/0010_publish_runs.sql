-- 0010_publish_runs.sql
-- Issue-level publish/export history. `published_posts` records per-article
-- output; this table records the package/issue build itself so the app can
-- show a durable publishing receipt after restart.
CREATE TABLE IF NOT EXISTS publish_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    issue_id TEXT NOT NULL UNIQUE,
    output_path TEXT NOT NULL,
    generated_files TEXT NOT NULL DEFAULT '[]',
    provider TEXT NOT NULL DEFAULT 'local_export',
    published_url TEXT,
    deployment_id TEXT,
    article_count INTEGER NOT NULL DEFAULT 0,
    skipped_count INTEGER NOT NULL DEFAULT 0,
    files_written INTEGER NOT NULL DEFAULT 0,
    generated_at TEXT NOT NULL
);
