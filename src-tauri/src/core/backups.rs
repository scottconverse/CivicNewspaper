// core/backups.rs
use super::db::DbConn;
use rusqlite::Connection;
use std::error::Error;
use std::time::Duration;

pub fn save_backup(conn: &Connection, dest_path: &str) -> Result<(), Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
    println!("[debug] restore_backup: copying backup to temp path");
    // 1. Copy selected backup to a temporary file
    let temp_db_path = format!("{}.restore_temp", live_db_path);
    std::fs::copy(backup_path, &temp_db_path)?;

    println!("[debug] restore_backup: validating integrity of temp db");
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

        let temp_conn = Connection::open(&temp_db_path)?;
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
    }

    println!("[debug] restore_backup: acquiring database lock");
    // 3. Create rollback recovery snapshot of the live DB
    let rollback_backup_path = format!("{}.rollback_temp", live_db_path);

    // Open a lock on the connection mutex
    let mut conn = db_conn_arc
        .lock()
        .map_err(|_| "Failed to acquire database lock")?;

    println!("[debug] restore_backup: creating rollback snapshot");
    {
        let mut rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&conn, &mut rollback_conn)?;
        backup.run_to_completion(5, Duration::from_millis(10), None)?;
        drop(backup);
    }

    println!("[debug] restore_backup: replacing connection with open_in_memory");
    // 4. Temporarily point active connection to an in-memory database
    // This releases all open file handles on live_db_path, allowing us to rename files on Windows
    *conn = Connection::open_in_memory()?;

    println!("[debug] restore_backup: swapping files via std::fs::rename");
    // 5. Atomic Swap of files
    if let Err(e) = std::fs::rename(&temp_db_path, live_db_path) {
        println!(
            "[debug] restore_backup: Swap failed: {}. Restoring from rollback snapshot.",
            e
        );
        eprintln!("Swap failed: {}. Restoring from rollback snapshot.", e);

        // Rollback: copy rollback snapshot back to live_db_path
        let mut original_conn = Connection::open(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);

        *conn = original_conn;
        let _ = std::fs::remove_file(&temp_db_path);
        let _ = std::fs::remove_file(&rollback_backup_path);

        return Err(format!("Failed to rename restored database file: {}", e).into());
    }

    println!("[debug] restore_backup: connecting to the newly swapped live database");
    // 6. Connect to the newly swapped live database
    let mut new_conn = match Connection::open(live_db_path) {
        Ok(c) => c,
        Err(err) => {
            println!(
                "[debug] restore_backup: failed to open new database: {}. Restoring from rollback.",
                err
            );
            eprintln!(
                "Failed to open new database: {}. Restoring from rollback.",
                err
            );
            // Restore from rollback
            let mut original_conn = Connection::open(live_db_path)?;
            let rollback_conn = Connection::open(&rollback_backup_path)?;
            let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
            let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
            drop(backup);
            *conn = original_conn;
            let _ = std::fs::remove_file(&rollback_backup_path);
            return Err(format!("Failed to connect to restored database file: {}", err).into());
        }
    };

    new_conn.pragma_update(None, "journal_mode", "WAL")?;

    println!("[debug] restore_backup: running migrations on restored database");
    // 7. Run migrations on the restored database just in case it is older
    if let Err(migration_err) = super::migrations::run_migrations(&mut new_conn) {
        println!(
            "[debug] restore_backup: migrations failed: {}. Rolling back.",
            migration_err
        );
        eprintln!(
            "Failed to run migrations on restored database: {}. Rolling back.",
            migration_err
        );
        // Restore from rollback
        let mut original_conn = Connection::open(live_db_path)?;
        let rollback_conn = Connection::open(&rollback_backup_path)?;
        let backup = rusqlite::backup::Backup::new(&rollback_conn, &mut original_conn)?;
        let _ = backup.run_to_completion(5, Duration::from_millis(10), None);
        drop(backup);
        *conn = original_conn;
        let _ = std::fs::remove_file(&rollback_backup_path);
        return Err(format!("Migrations failed on restored database: {}", migration_err).into());
    }

    println!("[debug] restore_backup: replacing connection handle");
    // Successfully restored! Replace the connection handle
    *conn = new_conn;

    println!("[debug] restore_backup: cleaning up temp files");
    // Clean up temp files
    let _ = std::fs::remove_file(&rollback_backup_path);
    let _ = std::fs::remove_file(&temp_db_path);

    println!("[debug] restore_backup: done");
    Ok(())
}
