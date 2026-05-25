# Implementation Report

## Pre-edit Fact-Forcing Gate

### File: `src-tauri/migrations/0004_source_tier.sql` & `0005_daily_scans.sql`
1. **Importers/Callers:** `src-tauri/src/core/migrations.rs` will include them via `include_str!`.
2. **Public API affected:** None directly. Modifies DB schema.
3. **Data schema touched:** 
   - `sources` table: Adds `tier TEXT DEFAULT 'community_signal'`.
   - New `daily_scan_runs` table: `id INTEGER PRIMARY KEY`, `started_at TEXT`, `completed_at TEXT`, `run_status TEXT`.
   - New `daily_scan_leads` table: `scan_id INTEGER`, `lead_id INTEGER`.
   - `leads` table: Adds `from_scan_lead_id INTEGER`.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/db.rs`
1. **Importers/Callers:** `tauri_cmds.rs`, `detectors.rs`, `daily_scan.rs`.
2. **Public API affected:** `Source` struct gets `tier` field; `Lead` struct gets `from_scan_lead_id`. New daily scan structs `DailyScanRun` and `DailyScanLead`. New CRUD methods.
3. **Data schema touched:** Same as migrations.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/migrations.rs`
1. **Importers/Callers:** `src-tauri/src/core/db.rs` `init_db`.
2. **Public API affected:** Schema migration ordering array `MIGRATIONS`.
3. **Data schema touched:** Registers `0004_source_tier` and `0005_daily_scans`.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/detectors.rs`
1. **Importers/Callers:** `tauri_cmds.rs` `run_detectors_cmd`.
2. **Public API affected:** None. Just updates `Source` initialization.
3. **Data schema touched:** N/A.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/prompts.rs` (New)
1. **Importers/Callers:** `src-tauri/src/core/mod.rs`, `src-tauri/src/tauri_cmds.rs`.
2. **Public API affected:** Exposes `list_prompts()` and `get_prompt(id: &str)`.
3. **Data schema touched:** N/A.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/daily_scan.rs` (New)
1. **Importers/Callers:** `src-tauri/src/core/mod.rs`, `src-tauri/src/tauri_cmds.rs`.
2. **Public API affected:** Exposes `run_daily_scan()`.
3. **Data schema touched:** Inserts rows into `daily_scan_runs` and `daily_scan_leads`.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

### File: `src-tauri/src/core/tests.rs`
1. **Importers/Callers:** `cargo test`.
2. **Public API affected:** Adds 6 new tests.
3. **Data schema touched:** N/A.
4. **Manifest Goal:** "CivicNews ships a real prompt library, a Daily Scan feature backed by the News Aggregator prompt, a one-click Plain Language rewrite, and an explicit Source Tier taxonomy on every source."

---
