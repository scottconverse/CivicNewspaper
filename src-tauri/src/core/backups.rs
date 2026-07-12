// core/backups.rs
use super::db::DbConn;
use rusqlite::{Connection, OptionalExtension};
use std::error::Error;
use std::time::Duration;

const CIVIC_DESK_APPLICATION_ID: i64 = 1_128_879_683;

fn application_id(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("PRAGMA application_id;", [], |row| row.get(0))
}

fn schema_definitions(
    conn: &Connection,
) -> Result<Vec<(String, String, String, String)>, rusqlite::Error> {
    let mut statement = conn.prepare(
        "SELECT type, name, tbl_name, sql
         FROM sqlite_master
         WHERE name NOT LIKE 'sqlite_%' AND sql IS NOT NULL
         ORDER BY type, name",
    )?;
    let definitions = statement
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?
        .collect();
    definitions
}

fn validate_civic_schema(
    conn: &Connection,
    require_current_schema: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    const REQUIRED_TABLES: &[&str] = &[
        "sources",
        "evidence_items",
        "leads",
        "lead_evidence",
        "drafts",
        "published_posts",
        "paired_clients",
    ];
    const CURRENT_TABLES: &[&str] = &[
        "settings",
        "daily_scan_runs",
        "daily_scan_leads",
        "publish_runs",
        "subscribers",
        "civic_observations",
        "civic_entities",
        "civic_observation_entities",
        "source_performance_scores",
        "dark_signals",
        "verification_tasks",
        "beat_memory",
        "story_templates",
        "publish_decision_audits",
    ];
    for table in REQUIRED_TABLES.iter().chain(
        require_current_schema
            .then_some(CURRENT_TABLES)
            .into_iter()
            .flatten(),
    ) {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
            [table],
            |row| row.get(0),
        )?;
        if count != 1 {
            return Err(format!(
                "Selected file is not a Civic Desk database: required table `{table}` is missing."
            )
            .into());
        }
    }

    if require_current_schema {
        let mut canonical = Connection::open_in_memory()?;
        super::migrations::run_migrations(&mut canonical)?;
        if schema_definitions(conn)? != schema_definitions(&canonical)? {
            return Err(
                "Selected file is not a compatible Civic Desk database: its tables, indexes, views, or triggers do not match the current schema."
                    .into(),
            );
        }
    }

    for (table, column) in [
        ("sources", "url"),
        ("evidence_items", "source_id"),
        ("drafts", "content"),
        ("drafts", "status"),
        ("published_posts", "draft_id"),
    ] {
        let sql = format!("SELECT COUNT(*) FROM pragma_table_info('{table}') WHERE name = ?1");
        let count: i64 = conn.query_row(&sql, [column], |row| row.get(0))?;
        if count != 1 {
            return Err(format!(
                "Selected file is not a compatible Civic Desk database: `{table}.{column}` is missing."
            )
            .into());
        }
    }
    let foreign_key_violation: Option<String> = conn
        .query_row(
            "SELECT CAST(\"table\" AS TEXT) FROM pragma_foreign_key_check LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()?;
    if let Some(table) = foreign_key_violation {
        return Err(format!(
            "Selected Civic Desk database has a broken foreign-key reference in `{table}`."
        )
        .into());
    }
    Ok(())
}

pub fn save_backup(conn: &Connection, dest_path: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut dest_conn = Connection::open(dest_path)?;
    let backup = rusqlite::backup::Backup::new(conn, &mut dest_conn)?;
    // Run the backup to completion (batch size 5 pages, sleep 10ms between batches to keep responsive)
    backup.run_to_completion(5, Duration::from_millis(10), None)?;
    Ok(())
}

pub fn restore_backup(
    db_conn_arc: &DbConn,
    backup_path: &str,
    live_db_path: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Copy selected backup to a temporary file
    let temp_db_path = format!("{}.restore_temp", live_db_path);
    std::fs::copy(backup_path, &temp_db_path)?;

    // 2. Open temporary DB and validate integrity & version
    {
        // Validate SQLite magic header first to prevent SQLite from hanging on raw garbage files
        let mut header = [0u8; 16];
        use std::io::Read;
        let is_valid_sqlite = if let Ok(mut f) = std::fs::File::open(&temp_db_path) {
            f.read_exact(&mut header).is_ok() && &header == b"SQLite format 3\0"
        } else {
            false
        };

        if !is_valid_sqlite {
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Database file is not a valid SQLite database.".into());
        }

        let mut temp_conn = Connection::open(&temp_db_path)?;
        let integrity: String =
            temp_conn.query_row("PRAGMA integrity_check;", [], |row| row.get(0))?;
        if integrity != "ok" {
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Database integrity check failed. Backup file is corrupt.".into());
        }

        let version: i32 = temp_conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;
        let expected = super::migrations::get_expected_version();
        if version > expected {
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(format!(
                "Backup database version (v{}) is newer than the app version (v{}). Please update the app.", 
                version, expected
            ).into());
        }
        let database_application_id = application_id(&temp_conn)?;
        if database_application_id != 0 && database_application_id != CIVIC_DESK_APPLICATION_ID {
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Selected file is not a Civic Desk database: its application identity belongs to another application.".into());
        }
        if version == expected && database_application_id != CIVIC_DESK_APPLICATION_ID {
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Selected file is not a Civic Desk database: the current database identity marker is missing.".into());
        }
        if let Err(schema_err) = validate_civic_schema(&temp_conn, version == expected) {
            drop(temp_conn);
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(schema_err);
        }
        if version < expected {
            if let Err(migration_err) = super::migrations::run_migrations(&mut temp_conn) {
                drop(temp_conn);
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!(
                    "Selected Civic Desk backup could not be upgraded safely: {migration_err}"
                )
                .into());
            }
        }
        if application_id(&temp_conn)? != CIVIC_DESK_APPLICATION_ID {
            drop(temp_conn);
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Selected file is not a Civic Desk database: the application identity marker is missing after migration.".into());
        }
        if let Err(schema_err) = validate_civic_schema(&temp_conn, true) {
            drop(temp_conn);
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(schema_err);
        }
    }

    // 3. Create rollback recovery snapshot of the live DB
    let rollback_backup_path = format!("{}.rollback_temp", live_db_path);

    // Open a lock on the connection mutex
    let mut conn = db_conn_arc
        .lock()
        .map_err(|_| "Failed to acquire database lock")?;

    {
        let mut rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&conn, &mut rollback_conn)?;
        backup.run_to_completion(5, Duration::from_millis(10), None)?;
        drop(backup);
    }

    // 4. Temporarily point active connection to an in-memory database
    // This releases all open file handles on live_db_path, allowing us to rename files on Windows
    *conn = Connection::open_in_memory()?;

    // 5. Atomic Swap of files
    if let Err(e) = std::fs::rename(&temp_db_path, live_db_path) {
        eprintln!("Swap failed: {}. Restoring from rollback snapshot.", e);

        // Rollback: copy rollback snapshot back to live_db_path
        let mut original_conn = super::db::open_conn(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);

        *conn = original_conn;
        let _ = std::fs::remove_file(&temp_db_path);
        let _ = std::fs::remove_file(&rollback_backup_path);

        return Err(format!("Failed to rename restored database file: {}", e).into());
    }

    // 6. Connect to the newly swapped live database (WAL + foreign_keys via open_conn)
    let mut new_conn = match super::db::open_conn(live_db_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!(
                "Failed to open new database: {}. Restoring from rollback.",
                err
            );
            // Restore from rollback
            let mut original_conn = super::db::open_conn(live_db_path)?;
            let rollback_conn = Connection::open(&rollback_backup_path)?;
            let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
            let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
            drop(backup);
            *conn = original_conn;
            let _ = std::fs::remove_file(&rollback_backup_path);
            return Err(format!("Failed to connect to restored database file: {}", err).into());
        }
    };

    // WAL + foreign_keys are already set by db::open_conn above (C-2).

    // 7. Run migrations on the restored database just in case it is older
    if let Err(migration_err) = super::migrations::run_migrations(&mut new_conn) {
        eprintln!(
            "Failed to run migrations on restored database: {}. Rolling back.",
            migration_err
        );
        // Restore from rollback
        let mut original_conn = super::db::open_conn(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);
        *conn = original_conn;
        let _ = std::fs::remove_file(&rollback_backup_path);
        return Err(format!("Migrations failed on restored database: {}", migration_err).into());
    }

    if let Err(schema_err) = validate_civic_schema(&new_conn, true) {
        eprintln!("Restored database failed the post-migration schema check: {schema_err}");
        let mut original_conn = super::db::open_conn(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);
        *conn = original_conn;
        let _ = std::fs::remove_file(&rollback_backup_path);
        return Err(format!("Restored database is incompatible: {schema_err}").into());
    }

    if application_id(&new_conn)? != CIVIC_DESK_APPLICATION_ID {
        let mut original_conn = super::db::open_conn(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);
        *conn = original_conn;
        let _ = std::fs::remove_file(&rollback_backup_path);
        return Err("Restored database is incompatible: the Civic Desk application identity marker is missing after migration.".into());
    }

    // Successfully restored! Replace the connection handle
    *conn = new_conn;

    // Clean up temp files
    let _ = std::fs::remove_file(&rollback_backup_path);
    let _ = std::fs::remove_file(&temp_db_path);

    Ok(())
}
