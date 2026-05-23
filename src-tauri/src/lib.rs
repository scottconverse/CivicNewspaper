// src/lib.rs
mod core;
mod tauri_cmds;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_cmds::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // 1. Get database path inside app data folder
            let db_path =
                crate::core::db::get_app_db_path(app.handle()).map_err(|e| e.to_string())?;
            let db_path_str = db_path.to_str().ok_or("Invalid DB path")?;

            // 2. Initialize database and run migrations
            let conn = crate::core::db::init_db(db_path_str).map_err(|e| e.to_string())?;
            let db_conn = Arc::new(Mutex::new(conn));

            // 3. Manage database connection state in Tauri
            app.manage(db_conn.clone());

            // 4. Start Axum server in a background tokio task for browser pairing
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::core::server::start_server(db_conn).await {
                    eprintln!("HTTP Axum Server failed to start: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_sources,
            add_source,
            delete_source,
            generate_pairing_pin,
            list_paired_clients,
            revoke_pairing,
            get_community_profile,
            save_community_profile,
            ingest,
            get_queue,
            get_evidence,
            save_draft,
            delete_draft,
            story_decision,
            generate_draft,
            llm_task,
            guardrails_check,
            publish,
            register_correction,
            backup_save,
            backup_restore,
            check_ollama,
            pull_model,
            get_system_ram,
            discover_sources
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
