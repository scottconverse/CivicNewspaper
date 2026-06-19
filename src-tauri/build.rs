use std::path::Path;

fn main() {
    // B-1: the app bundles the Ollama sidecar as a Tauri `externalBin`
    // (`binaries/ollama-<target-triple>`). These binaries are NOT committed to
    // the repo — they are fetched by `scripts/fetch-ollama-binaries.sh`. If a
    // developer builds from a fresh checkout without running that script, the
    // Tauri bundler fails late with an opaque "resource path ... doesn't exist".
    // Detect the missing binary here and emit a CLEAR, actionable message up
    // front so the fix (run the fetch script) is obvious.
    check_ollama_sidecar_present();

    tauri_build::build();
}

fn check_ollama_sidecar_present() {
    // The target triple Tauri appends to the externalBin base name.
    let target = std::env::var("TARGET").unwrap_or_default();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    if target.is_empty() {
        // Can't determine the triple; let the bundler speak for itself.
        return;
    }

    let exe_suffix = if target.contains("windows") {
        ".exe"
    } else {
        ""
    };
    let expected = format!("binaries/ollama-{}{}", target, exe_suffix);
    let expected_path = Path::new(&manifest_dir).join(&expected);

    // Re-run this check if the binaries directory changes.
    println!("cargo:rerun-if-changed=binaries");

    if !expected_path.exists() {
        // A warning (not a hard error) keeps `cargo check`/`cargo test` usable
        // on machines where the sidecar isn't needed for the task at hand, while
        // still loudly telling the developer what to do. The Tauri bundler will
        // hard-fail later if the binary is genuinely required for a bundle.
        println!(
            "cargo:warning=Ollama sidecar binary not found at `{}`.",
            expected_path.display()
        );
        println!(
            "cargo:warning=Run `bash scripts/fetch-ollama-binaries.sh` from the repo root to download the bundled Ollama binaries before building or bundling the app."
        );
    }
}
