// src/lib.rs
mod core;
mod tauri_cmds;

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_cmds::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let sidecar = Arc::new(crate::core::llm::OllamaSidecar::new());
            app.manage(sidecar.clone());

            // Panic Hook
            if let Ok(app_data) = app.path().app_data_dir() {
                let log_dir = app_data.join("logs");
                let log_path = log_dir.join("civicnews.log");
                let sidecar_clone = sidecar.clone();
                std::panic::set_hook(Box::new(move |info| {
                    // ENG-Min2: best-effort, non-blocking stop so the hook can
                    // never deadlock on the sidecar mutex during a panic.
                    sidecar_clone.stop_best_effort();
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

            // 4. Start Axum server in a background tokio task for browser pairing.
            //    ENG-Min5: a bind failure (e.g. port 12053 already in use) must
            //    not be swallowed to stderr only — emit a Tauri event so the
            //    pairing UI can explain that browser pairing is unavailable.
            let server_app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = crate::core::server::start_server(db_conn).await {
                    eprintln!("HTTP Axum Server failed to start: {}", e);
                    use tauri::Emitter;
                    let _ = server_app_handle.emit(
                        "http-server-error",
                        format!(
                            "The local pairing server could not start ({}). Browser pairing will be unavailable — another app may be using port 12053.",
                            e
                        ),
                    );
                }
            });

            // Start Ollama Sidecar
            if let Err(e) = sidecar.start(app.handle()) {
                eprintln!("Failed to start Ollama sidecar: {}", e);
            }

            // Register LlmClient state for Daily Scan / LLM features
            app.manage(std::sync::Arc::new(crate::core::llm::OllamaClient)
                as std::sync::Arc<dyn crate::core::llm::LlmClient>);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_sources,
            add_source,
            delete_source,
            generate_pairing_pin,
            list_paired_clients,
            get_browser_extension_path,
            open_local_path,
            open_external_url,
            extract_source_import_text,
            revoke_pairing,
            list_daily_scan_leads,
            get_community_profile,
            save_community_profile,
            ingest,
            get_queue,
            get_evidence,
            save_draft,
            delete_draft,
            story_decision,
            attest_draft,
            get_guardrail_terms,
            set_guardrail_terms,
            generate_draft,
            llm_task,
            guardrails_check,
            publish,
            record_publish_destination,
            save_publisher_config,
            get_publisher_config,
            test_publisher_connection,
            publish_with_connector,
            list_publish_history,
            list_subscribers,
            add_subscriber,
            delete_subscriber,
            import_subscribers_csv,
            export_subscribers_csv,
            read_publish_artifact,
            export_issue_email,
            register_correction,
            backup_save,
            backup_restore,
            check_ollama,
            pull_ollama_model,
            cancel_ollama_pull,
            get_system_ram,
            discover_sources,
            ollama_health,
            is_onboarding_complete,
            set_onboarding_complete,
            set_setting,
            get_setting,
            export_diagnostics,
            list_prompts,
            get_prompt,
            run_daily_scan,
            plain_language_rewrite
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::Exit
            | tauri::RunEvent::WindowEvent {
                event: tauri::WindowEvent::CloseRequested { .. },
                ..
            } => {
                if let Some(sidecar) =
                    app_handle.try_state::<Arc<crate::core::llm::OllamaSidecar>>()
                {
                    let _ = sidecar.stop();
                }
            }
            _ => {}
        });
}
