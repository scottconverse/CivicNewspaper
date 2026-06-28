fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        // The generated Tauri invoke handler is large enough that the bundled
        // Windows release exe can overflow the default stack during startup.
        // Give the desktop binary a normal larger GUI-app stack reserve.
        println!("cargo:rustc-link-arg-bin=civicnews=/STACK:16777216");
    }

    // The app installs/starts Ollama from first-run setup when a clean machine
    // does not already have a reachable runtime. Keep release bundles small and
    // verify runtime bootstrap in cleanroom E2E instead of requiring a bundled
    // multi-GB sidecar at build time.
    tauri_build::build();
}
