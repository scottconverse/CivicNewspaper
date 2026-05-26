ALTER TABLE sources ADD COLUMN tier TEXT DEFAULT 'community_signal';

UPDATE sources SET tier = 'official_record' WHERE type = 'primary_record' OR type = 'official_comm';
UPDATE sources SET tier = 'news_reporting' WHERE type = 'media_lead';
