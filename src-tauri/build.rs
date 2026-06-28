fn main() {
    // The app installs/starts Ollama from first-run setup when a clean machine
    // does not already have a reachable runtime. Keep release bundles small and
    // verify runtime bootstrap in cleanroom E2E instead of requiring a bundled
    // multi-GB sidecar at build time.
    tauri_build::build();
}
