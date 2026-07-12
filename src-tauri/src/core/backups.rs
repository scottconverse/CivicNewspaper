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
    restore_backup_internal(db_conn_arc, backup_path, live_db_path, None)
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RestoreFault {
    Preserve,
    Open,
    Migration,
    Schema,
    RollbackRename,
    RollbackReopen,
    IdentityQuery,
    Checkpoint,
    Cleanup,
    StaleRecoveryCleanup,
    RollbackCopy,
    LiveRemove,
    SidecarCleanup,
    FallbackCopy,
    BothRollbackReopens,
    SuccessRollbackCleanup,
    StaleRollbackCleanupBeforePreserve,
}

#[cfg(test)]
pub(crate) fn restore_backup_with_fault(
    db_conn_arc: &DbConn,
    backup_path: &str,
    live_db_path: &str,
    fault: RestoreFault,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    restore_backup_internal(db_conn_arc, backup_path, live_db_path, Some(fault))
}

fn restore_live_file(
    conn: &mut Connection,
    live_db_path: &str,
    rollback_path: &str,
    fault: Option<RestoreFault>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(not(test))]
    let _ = fault;
    let recovery_path = format!("{rollback_path}.recovery");
    let emergency = super::db::open_conn(rollback_path).map_err(|error| {
        format!("Could not open the preserved original database for emergency recovery: {error}")
    })?;
    *conn = emergency;
    #[cfg(test)]
    if matches!(fault, Some(RestoreFault::StaleRecoveryCleanup)) {
        return Err("injected stale recovery cleanup failure; the active handle remains connected to the preserved original database".into());
    }
    if std::path::Path::new(&recovery_path).exists() {
        std::fs::remove_file(&recovery_path).map_err(|error| {
            format!(
                "Could not clear the stale rollback recovery artifact at `{recovery_path}`: {error}"
            )
        })?;
    }
    #[cfg(test)]
    if matches!(fault, Some(RestoreFault::RollbackCopy)) {
        return Err("injected rollback copy failure; the active handle remains connected to the preserved original database".into());
    }
    std::fs::copy(rollback_path, &recovery_path).map_err(|error| {
        format!("Could not copy the preserved original database for rollback installation: {error}")
    })?;

    let sidecar_quarantine = format!(
        "{live_db_path}.sidecars-cleanup-pending-{}",
        uuid::Uuid::new_v4()
    );
    let sidecars = [format!("{live_db_path}-wal"), format!("{live_db_path}-shm")];
    if sidecars
        .iter()
        .any(|path| std::path::Path::new(path).exists())
    {
        std::fs::create_dir(&sidecar_quarantine)?;
        for sidecar in &sidecars {
            if std::path::Path::new(sidecar).exists() {
                let name = std::path::Path::new(sidecar)
                    .file_name()
                    .ok_or("Invalid SQLite sidecar name")?;
                std::fs::rename(
                    sidecar,
                    std::path::Path::new(&sidecar_quarantine).join(name),
                )?;
            }
        }
        #[cfg(test)]
        if matches!(fault, Some(RestoreFault::SidecarCleanup)) {
            return Err(format!("injected sidecar cleanup failure; candidate sidecars are quarantined as a set at `{sidecar_quarantine}` and the active handle remains on the preserved original").into());
        }
    }
    if std::path::Path::new(live_db_path).exists() {
        #[cfg(test)]
        if matches!(fault, Some(RestoreFault::LiveRemove)) {
            return Err("injected live candidate removal failure; the active handle remains connected to the preserved original database".into());
        }
        std::fs::remove_file(live_db_path)?;
    }
    #[cfg(test)]
    let install_result = if matches!(
        fault,
        Some(RestoreFault::RollbackRename | RestoreFault::FallbackCopy)
    ) {
        Err(std::io::Error::other("injected rollback rename failure"))
    } else {
        std::fs::rename(&recovery_path, live_db_path)
    };
    #[cfg(not(test))]
    let install_result = std::fs::rename(&recovery_path, live_db_path);
    let install_error = install_result.err();
    if let Some(error) = &install_error {
        #[cfg(test)]
        if matches!(fault, Some(RestoreFault::FallbackCopy)) {
            return Err(format!("Rollback replacement failed: {error}. Injected fallback copy failure; the preserved original remains active at `{rollback_path}` and the recovery copy remains at `{recovery_path}`").into());
        }
        std::fs::copy(rollback_path, live_db_path).map_err(|fallback_error| {
            format!(
                "Rollback replacement failed: {error}. The original remains at `{rollback_path}`, but copying it back to the live path also failed: {fallback_error}"
            )
        })?;
    }

    #[cfg(test)]
    let open_result = if matches!(
        fault,
        Some(RestoreFault::RollbackReopen | RestoreFault::BothRollbackReopens)
    ) {
        Err("injected rollback reopen failure".into())
    } else {
        super::db::open_conn(live_db_path)
    };
    #[cfg(not(test))]
    let open_result = super::db::open_conn(live_db_path);
    let open_error = open_result.as_ref().err().map(ToString::to_string);
    let recovered = match open_result {
        Ok(recovered) => recovered,
        Err(_) => {
            #[cfg(test)]
            if matches!(fault, Some(RestoreFault::BothRollbackReopens)) {
                return Err(format!("Rollback database was restored at `{live_db_path}`, but both reopen attempts were injected to fail. The active handle remains connected to the preserved original at `{rollback_path}`").into());
            }
            super::db::open_conn(live_db_path).map_err(|fallback_error| {
            format!(
                "Rollback database was restored at `{live_db_path}`, but reopening it failed: {}. A retry also failed: {fallback_error}. The original recovery file remains at `{rollback_path}`",
                open_error.as_deref().unwrap_or("unknown reopen error")
            )
        })?
        }
    };
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .map_err(|error| {
            format!("Could not checkpoint the preserved rollback database before switching back to the live path: {error}")
        })?;
    *conn = recovered;
    #[cfg(test)]
    if matches!(fault, Some(RestoreFault::SuccessRollbackCleanup)) {
        let quarantine = format!("{rollback_path}.cleanup-pending-{}", uuid::Uuid::new_v4());
        std::fs::rename(rollback_path, &quarantine)?;
        return Err(format!("injected successful-restore rollback cleanup failure; the old snapshot was quarantined at `{quarantine}` and the restored live handle remains usable").into());
    }
    for artifact in [
        rollback_path.to_string(),
        format!("{rollback_path}-wal"),
        format!("{rollback_path}-shm"),
        recovery_path.clone(),
    ] {
        if std::path::Path::new(&artifact).exists() {
            std::fs::remove_file(&artifact).map_err(|error| {
                format!(
                    "The live database was recovered, but cleanup failed for `{artifact}`: {error}. The recovery artifact remains quarantined at that path"
                )
            })?;
        }
    }
    if std::path::Path::new(&sidecar_quarantine).exists() {
        std::fs::remove_dir_all(&sidecar_quarantine).map_err(|error| {
            format!("Live database recovery succeeded, but the candidate SQLite sidecar quarantine could not be removed: {error}. It remains at `{sidecar_quarantine}`")
        })?;
    }

    if let Some(error) = install_error {
        return Err(format!("Rollback replacement encountered an error but recovered by copying the preserved original: {error}").into());
    }
    if let Some(error) = open_error {
        return Err(format!(
            "Rollback reopen encountered an error but recovered on retry: {error}"
        )
        .into());
    }
    Ok(())
}

fn restore_backup_internal(
    db_conn_arc: &DbConn,
    backup_path: &str,
    live_db_path: &str,
    fault: Option<RestoreFault>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Copy selected backup to a temporary file
    let temp_db_path = format!("{}.restore_temp", live_db_path);
    let outcome = (|| -> Result<(), Box<dyn Error + Send + Sync>> {
        std::fs::copy(backup_path, &temp_db_path)?;
        #[cfg(test)]
        if matches!(fault, Some(RestoreFault::Cleanup)) {
            return Err("injected restore operation failure before cleanup".into());
        }

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
            if database_application_id != 0 && database_application_id != CIVIC_DESK_APPLICATION_ID
            {
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

        #[cfg(test)]
        let checkpoint_result = if matches!(fault, Some(RestoreFault::Checkpoint)) {
            Err(rusqlite::Error::ExecuteReturnedResults)
        } else {
            conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        };
        #[cfg(not(test))]
        let checkpoint_result = conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);");
        checkpoint_result?;

        if std::path::Path::new(&rollback_backup_path).exists() {
            #[cfg(test)]
            if matches!(
                fault,
                Some(RestoreFault::StaleRollbackCleanupBeforePreserve)
            ) {
                return Err(
                    "injected stale rollback cleanup failure before the live handle was detached"
                        .into(),
                );
            }
            std::fs::remove_file(&rollback_backup_path).map_err(|error| {
                format!("Could not remove stale rollback snapshot before restore; the live database remains active and unchanged: {error}")
            })?;
        }

        // 4. Temporarily point active connection to an in-memory database. This releases
        // all handles before preserving the original file byte-for-byte for rollback.
        *conn = Connection::open_in_memory()?;
        #[cfg(test)]
        let preserve_result = if matches!(fault, Some(RestoreFault::Preserve)) {
            Err(std::io::Error::other(
                "injected live database preservation failure",
            ))
        } else {
            std::fs::rename(live_db_path, &rollback_backup_path)
        };
        #[cfg(not(test))]
        let preserve_result = std::fs::rename(live_db_path, &rollback_backup_path);
        if let Err(preserve_error) = preserve_result {
            let _ = std::fs::remove_file(&temp_db_path);
            let _ = std::fs::remove_file(&rollback_backup_path);
            let recovered = super::db::open_conn(live_db_path).map_err(|recovery_error| {
            format!(
                "Failed to preserve the live database for rollback: {preserve_error}. The unchanged live database also could not be reopened: {recovery_error}"
            )
        })?;
            *conn = recovered;
            return Err(format!(
                "Failed to preserve the live database for rollback: {preserve_error}"
            )
            .into());
        }

        // 5. Swap the validated candidate into the now-vacant live path.
        if let Err(e) = std::fs::rename(&temp_db_path, live_db_path) {
            eprintln!("Swap failed: {}. Restoring from rollback snapshot.", e);
            if let Err(rollback_error) =
                restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
            {
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!(
                "Failed to rename restored database file: {e}. Rollback recovery encountered: {rollback_error}"
            )
            .into());
            }
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(format!("Failed to rename restored database file: {}", e).into());
        }

        // 6. Connect to the newly swapped live database (WAL + foreign_keys via open_conn)
        #[cfg(test)]
        let open_result = if matches!(fault, Some(RestoreFault::Open)) {
            Err("injected post-swap open failure".into())
        } else {
            super::db::open_conn(live_db_path)
        };
        #[cfg(not(test))]
        let open_result = super::db::open_conn(live_db_path);
        let mut new_conn = match open_result {
            Ok(c) => c,
            Err(err) => {
                eprintln!(
                    "Failed to open new database: {}. Restoring from rollback.",
                    err
                );
                if let Err(rollback_error) =
                    restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
                {
                    let _ = std::fs::remove_file(&temp_db_path);
                    return Err(format!(
                    "Failed to connect to restored database file: {err}. Rollback recovery encountered: {rollback_error}"
                )
                .into());
                }
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!("Failed to connect to restored database file: {}", err).into());
            }
        };

        // WAL + foreign_keys are already set by db::open_conn above (C-2).

        // 7. Run migrations on the restored database just in case it is older
        #[cfg(test)]
        let migration_result = if matches!(fault, Some(RestoreFault::Migration)) {
            Err("injected post-swap migration failure".into())
        } else {
            super::migrations::run_migrations(&mut new_conn)
        };
        #[cfg(not(test))]
        let migration_result = super::migrations::run_migrations(&mut new_conn);
        if let Err(migration_err) = migration_result {
            eprintln!(
                "Failed to run migrations on restored database: {}. Rolling back.",
                migration_err
            );
            drop(new_conn);
            if let Err(rollback_error) =
                restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
            {
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!(
                "Migrations failed on restored database: {migration_err}. Rollback recovery encountered: {rollback_error}"
            )
            .into());
            }
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(
                format!("Migrations failed on restored database: {}", migration_err).into(),
            );
        }

        #[cfg(test)]
        let schema_result = if matches!(
            fault,
            Some(
                RestoreFault::Schema
                    | RestoreFault::RollbackRename
                    | RestoreFault::RollbackReopen
                    | RestoreFault::StaleRecoveryCleanup
                    | RestoreFault::RollbackCopy
                    | RestoreFault::LiveRemove
                    | RestoreFault::SidecarCleanup
                    | RestoreFault::FallbackCopy
                    | RestoreFault::BothRollbackReopens
            )
        ) {
            Err("injected post-swap schema failure".into())
        } else {
            validate_civic_schema(&new_conn, true)
        };
        #[cfg(not(test))]
        let schema_result = validate_civic_schema(&new_conn, true);
        if let Err(schema_err) = schema_result {
            eprintln!("Restored database failed the post-migration schema check: {schema_err}");
            drop(new_conn);
            if let Err(rollback_error) =
                restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
            {
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!(
                "Restored database is incompatible: {schema_err}. Rollback recovery encountered: {rollback_error}"
            )
            .into());
            }
            let _ = std::fs::remove_file(&temp_db_path);
            return Err(format!("Restored database is incompatible: {schema_err}").into());
        }

        #[cfg(test)]
        let identity_result = if matches!(fault, Some(RestoreFault::IdentityQuery)) {
            Err(rusqlite::Error::InvalidQuery)
        } else {
            application_id(&new_conn)
        };
        #[cfg(not(test))]
        let identity_result = application_id(&new_conn);
        let restored_application_id = match identity_result {
            Ok(value) => value,
            Err(identity_error) => {
                drop(new_conn);
                if let Err(rollback_error) =
                    restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
                {
                    return Err(format!(
                    "Could not read the restored database identity: {identity_error}. Rollback recovery encountered: {rollback_error}"
                )
                .into());
                }
                return Err(format!(
                "Could not read the restored database identity: {identity_error}. The original database was restored."
            )
            .into());
            }
        };
        if restored_application_id != CIVIC_DESK_APPLICATION_ID {
            drop(new_conn);
            if let Err(rollback_error) =
                restore_live_file(&mut conn, live_db_path, &rollback_backup_path, fault)
            {
                let _ = std::fs::remove_file(&temp_db_path);
                return Err(format!(
                "Restored database identity is incompatible. Rollback recovery encountered: {rollback_error}"
            )
            .into());
            }
            let _ = std::fs::remove_file(&temp_db_path);
            return Err("Restored database is incompatible: the Civic Desk application identity marker is missing after migration.".into());
        }

        // Successfully restored! Replace the connection handle
        *conn = new_conn;

        if std::path::Path::new(&rollback_backup_path).exists() {
            let quarantine = format!(
                "{rollback_backup_path}.cleanup-pending-{}",
                uuid::Uuid::new_v4()
            );
            #[cfg(test)]
            if matches!(fault, Some(RestoreFault::SuccessRollbackCleanup)) {
                std::fs::rename(&rollback_backup_path, &quarantine)?;
                eprintln!("Restore committed; injected rollback cleanup failure. The old snapshot was quarantined at `{quarantine}` and the restored live handle remains usable.");
                return Ok(());
            }
            if let Err(cleanup_error) = std::fs::remove_file(&rollback_backup_path) {
                std::fs::rename(&rollback_backup_path, &quarantine).map_err(|quarantine_error| {
                    format!("Restore succeeded, but the old snapshot could not be removed: {cleanup_error}, and quarantine also failed: {quarantine_error}. It remains at `{rollback_backup_path}`")
                })?;
                eprintln!("Restore committed, but the old snapshot could not be removed: {cleanup_error}. It was quarantined at `{quarantine}` and the restored live handle remains usable.");
                return Ok(());
            }
        }
        Ok(())
    })();

    let cleanup = cleanup_restore_temp(&temp_db_path, fault);
    match (outcome, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(cleanup_error)) => Err(cleanup_error),
        (Err(error), Err(cleanup_error)) => {
            Err(format!("{error} Cleanup also failed: {cleanup_error}").into())
        }
    }
}

fn cleanup_restore_temp(
    temp_path: &str,
    fault: Option<RestoreFault>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if !std::path::Path::new(temp_path).exists() {
        return Ok(());
    }
    let quarantine = format!("{temp_path}.cleanup-pending-{}", uuid::Uuid::new_v4());
    #[cfg(test)]
    if matches!(fault, Some(RestoreFault::Cleanup)) {
        std::fs::rename(temp_path, &quarantine)?;
        return Err(format!(
            "injected restore temp cleanup failure; the temporary copy was quarantined at `{quarantine}`"
        )
        .into());
    }
    #[cfg(not(test))]
    let _ = fault;
    match std::fs::remove_file(temp_path) {
        Ok(()) => Ok(()),
        Err(cleanup_error) => match std::fs::rename(temp_path, &quarantine) {
            Ok(()) => Err(format!(
                "Could not remove the restore temporary copy: {cleanup_error}. It was quarantined at `{quarantine}`"
            )
            .into()),
            Err(quarantine_error) => Err(format!(
                "Could not remove the restore temporary copy: {cleanup_error}. Could not quarantine it away from the next restore either: {quarantine_error}. It remains at `{temp_path}`"
            )
            .into()),
        },
    }
}
