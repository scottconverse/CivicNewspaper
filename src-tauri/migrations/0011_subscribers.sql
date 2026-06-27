-- 0011_subscribers.sql
-- Local newsletter subscriber list for issue email exports.
CREATE TABLE IF NOT EXISTS subscribers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'unsubscribed')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
