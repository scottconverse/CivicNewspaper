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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Panic Hook
            if let Ok(app_data) = app.path().app_data_dir() {
                let log_dir = app_data.join("logs");
                let log_path = log_dir.join("civicnews.log");
                std::panic::set_hook(Box::new(move |info| {
                    let msg = match info.payload().downcast_ref::<&'static str>() {
                        Some(s) => *s,
                        None => match info.payload().downcast_ref::<String>() {
                            Some(s) => &s[..],
                            None => "Box<dyn Any>",
                        },
                    };
                    let ts = chrono::Utc::now().to_rfc3339();
                    let backtrace = std::backtrace::Backtrace::force_capture();

                    if std::fs::create_dir_all(&log_dir).is_ok() {
                        let mut truncate = false;
                        if let Ok(metadata) = std::fs::metadata(&log_path) {
                            if metadata.len() >= 1_048_576 {
                                truncate = true;
                            }
                        }

                        let file_opts = std::fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(truncate)
                            .append(!truncate)
                            .open(&log_path);

                        if let Ok(mut file) = file_opts {
                            use std::io::Write;
                            let _ = writeln!(file, "[{}] PANIC: {}\n{}", ts, msg, backtrace);
                        }
                    }
                }));
            }

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

            // Start Ollama Sidecar
            let sidecar = Arc::new(crate::core::llm::OllamaSidecar::new());
            app.manage(sidecar.clone());
            if let Err(e) = sidecar.start(app.handle()) {
                eprintln!("Failed to start Ollama sidecar: {}", e);
            }

            // Register LlmClient state for Daily Scan / LLM features
            let llm_client: Arc<dyn crate::core::llm::LlmClient> = Arc::new(crate::core::llm::OllamaClient);
            app.manage(llm_client);

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
            pull_ollama_model,
            get_system_ram,
            discover_sources,
            ollama_health,
            ollama_pull_model,
            is_onboarding_complete,
            set_onboarding_complete,
            set_setting,
            export_diagnostics,
            list_prompts,
            get_prompt,
            run_daily_scan,
            plain_language_rewrite
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(sidecar) =
                    app_handle.try_state::<Arc<crate::core::llm::OllamaSidecar>>()
                {
                    let _ = sidecar.stop();
                }
            }
        });
}
