ALTER TABLE daily_scan_leads ADD COLUMN recurrence_count INTEGER;
ALTER TABLE daily_scan_leads ADD COLUMN recurrence_note TEXT;

ALTER TABLE leads ADD COLUMN recurrence_count INTEGER;
ALTER TABLE leads ADD COLUMN recurrence_note TEXT;
