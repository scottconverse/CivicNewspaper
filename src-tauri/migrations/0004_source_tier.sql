ALTER TABLE sources ADD COLUMN tier TEXT NOT NULL DEFAULT 'official_record'
  CHECK (tier IN ('official_record', 'news_reporting', 'community_signal'));

-- Backfill: any existing source with type='media_lead' becomes news_reporting.
UPDATE sources SET tier = 'news_reporting' WHERE type = 'media_lead';
