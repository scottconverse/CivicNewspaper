ALTER TABLE daily_scan_leads ADD COLUMN story_type TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN what_changed TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN immediacy INTEGER;
ALTER TABLE daily_scan_leads ADD COLUMN impact INTEGER;
ALTER TABLE daily_scan_leads ADD COLUMN conflict INTEGER;
ALTER TABLE daily_scan_leads ADD COLUMN novelty INTEGER;
ALTER TABLE daily_scan_leads ADD COLUMN publishability_note TEXT;
ALTER TABLE daily_scan_leads ADD COLUMN disposition TEXT NOT NULL DEFAULT 'review';

ALTER TABLE leads ADD COLUMN story_type TEXT;
ALTER TABLE leads ADD COLUMN disposition TEXT NOT NULL DEFAULT 'review';
ALTER TABLE leads ADD COLUMN novelty_score INTEGER;
ALTER TABLE leads ADD COLUMN novelty_reason TEXT;

CREATE TABLE IF NOT EXISTS story_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL,
    description TEXT NOT NULL,
    prompt_guidance TEXT NOT NULL
);

INSERT OR IGNORE INTO story_templates (key, label, description, prompt_guidance) VALUES
('story', 'Reported story', 'A current, specific civic development with enough verified evidence for a reader-facing article.', 'Write a clean lede, explain why this matters now, use only cited evidence, and do not inflate weak facts.'),
('brief', 'Brief', 'A short item with one clear current fact, useful to readers but not enough for a full story.', 'Keep it short, precise, and evidence-bound. Avoid background filler.'),
('watch', 'Watch item', 'Something worth monitoring, but not yet a publishable story.', 'Explain what is known, what is missing, and what should be checked next.'),
('background', 'Background note', 'Evergreen or recurring information with no current news development.', 'Do not frame this as news. State what new fact would make it publishable.'),
('verification', 'Verification assignment', 'A lead that may matter but needs more checking before drafting.', 'List concrete verification steps and do not write reader-facing article copy yet.');
