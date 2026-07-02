// core/migrations.rs
use rusqlite::Connection;
use std::error::Error;

// IMPORTANT: the applied schema version is the *array position* (index + 1), NOT the
// numeric prefix in the filename. The filenames are non-contiguous (there is no `0002`),
// so `0003_settings` is applied as `user_version = 2`. When adding a migration, ALWAYS
// append to the end of this array - never insert in the middle, or every later migration's
// applied version shifts and existing databases will re-run or skip migrations. The
// debug-assert in `run_migrations` enforces that filename prefixes stay monotonically
// increasing so an out-of-order insertion fails loudly in debug/test builds.
const MIGRATIONS: &[(&str, &str)] = &[
    ("0001_init", include_str!("../../migrations/0001_init.sql")),
    (
        "0003_settings",
        include_str!("../../migrations/0003_settings.sql"),
    ),
    (
        "0004_source_tier",
        include_str!("../../migrations/0004_source_tier.sql"),
    ),
    (
        "0005_daily_scans",
        include_str!("../../migrations/0005_daily_scans.sql"),
    ),
    (
        "0006_daily_scan_lead_source_nullable",
        include_str!("../../migrations/0006_daily_scan_lead_source_nullable.sql"),
    ),
    (
        "0007_source_tier_check",
        include_str!("../../migrations/0007_source_tier_check.sql"),
    ),
    (
        "0008_draft_publish_gate",
        include_str!("../../migrations/0008_draft_publish_gate.sql"),
    ),
    (
        "0009_daily_scan_lead_context",
        include_str!("../../migrations/0009_daily_scan_lead_context.sql"),
    ),
    (
        "0010_publish_runs",
        include_str!("../../migrations/0010_publish_runs.sql"),
    ),
    (
        "0011_subscribers",
        include_str!("../../migrations/0011_subscribers.sql"),
    ),
    (
        "0012_civic_intelligence",
        include_str!("../../migrations/0012_civic_intelligence.sql"),
    ),
    (
        "0013_verification_queue",
        include_str!("../../migrations/0013_verification_queue.sql"),
    ),
    (
        "0014_beat_memory",
        include_str!("../../migrations/0014_beat_memory.sql"),
    ),
    (
        "0015_story_quality_metadata",
        include_str!("../../migrations/0015_story_quality_metadata.sql"),
    ),
    (
        "0016_recurrence_metadata",
        include_str!("../../migrations/0016_recurrence_metadata.sql"),
    ),
    (
        "0017_publish_decision_audit",
        include_str!("../../migrations/0017_publish_decision_audit.sql"),
    ),
    (
        "0018_evidence_source_identity",
        include_str!("../../migrations/0018_evidence_source_identity.sql"),
    ),
];

pub fn run_migrations(conn: &mut Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Guard the append-only invariant: filename numeric prefixes must be strictly
    // increasing. A mid-array insertion (which would silently shift applied versions)
    // makes this fail loudly in debug/test builds.
    debug_assert!(
        MIGRATIONS
            .windows(2)
            .all(|w| filename_ordinal(w[0].0) < filename_ordinal(w[1].0)),
        "migration filename prefixes must be strictly increasing; do not insert mid-array"
    );

    // Enforce foreign keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Get current user_version
    let current_version: i32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    for (idx, (name, sql)) in MIGRATIONS.iter().enumerate() {
        let version = (idx + 1) as i32;
        if version > current_version {
            println!(
                "Applying database migration: {} (version {})",
                name, version
            );

            // We use a transaction to apply the schema updates atomically
            let tx = conn.transaction()?;
            tx.execute_batch(sql)?;
            tx.execute(&format!("PRAGMA user_version = {};", version), [])?;
            tx.commit()?;
        }
    }

    Ok(())
}
#[allow(dead_code)]
pub fn get_current_version(conn: &Connection) -> Result<i32, rusqlite::Error> {
    conn.query_row("PRAGMA user_version;", [], |row| row.get(0))
}

pub fn get_expected_version() -> i32 {
    MIGRATIONS.len() as i32
}

/// Parse the leading numeric prefix of a migration name (e.g. "0003_settings" -> 3).
/// Used only by the debug-assert monotonicity guard, not for the applied version.
fn filename_ordinal(name: &str) -> u32 {
    name.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}
