// core/migrations.rs
use rusqlite::Connection;
use std::error::Error;

const MIGRATIONS: &[(&str, &str)] = &[
    ("0001_init", include_str!("../../migrations/0001_init.sql")),
    (
        "0003_settings",
        include_str!("../../migrations/0003_settings.sql"),
    ),
];

pub fn run_migrations(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
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
