fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        // The generated Tauri invoke handler is large enough that the bundled
        // Windows release exe can overflow the default stack during startup.
        // Give the desktop binary a normal larger GUI-app stack reserve.
        println!("cargo:rustc-link-arg-bin=civicnews=/STACK:16777216");

        // tauri-plugin-dialog links TaskDialogIndirect. Cargo's Windows test
        // harness does not inherit the Tauri application manifest, so without
        // this dependency Windows activates Common Controls v5.82 and the test
        // executable exits before discovery with STATUS_ENTRYPOINT_NOT_FOUND.
        println!(
            "cargo:rustc-link-arg=/MANIFESTDEPENDENCY:type='win32' name='Microsoft.Windows.Common-Controls' version='6.0.0.0' processorArchitecture='*' publicKeyToken='6595b64144ccf1df' language='*'"
        );
    }

    // The app installs/starts Ollama from first-run setup when a clean machine
    // does not already have a reachable runtime. Keep release bundles small and
    // verify runtime bootstrap in cleanroom E2E instead of requiring a bundled
    // multi-GB sidecar at build time.
    tauri_build::build();
}
