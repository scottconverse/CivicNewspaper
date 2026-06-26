ALTER TABLE daily_scan_leads ADD COLUMN why_flagged TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN source_name TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN source_type TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN priority TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN suggested_next_step TEXT;
