// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "windows")]
fn harden_webview2_startup() {
    const GPU_FLAGS: &str = "--disable-gpu --disable-gpu-compositing";

    let merged = match std::env::var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS") {
        Ok(existing) if existing.contains("--disable-gpu") => existing,
        Ok(existing) if !existing.trim().is_empty() => format!("{existing} {GPU_FLAGS}"),
        _ => GPU_FLAGS.to_string(),
    };

    std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", merged);
}

#[cfg(not(target_os = "windows"))]
fn harden_webview2_startup() {}

fn main() {
    harden_webview2_startup();

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let _ = std::env::set_current_dir(dir);
        }
    }
    tauri_app_lib::run()
}
