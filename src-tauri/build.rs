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
        // Emit a CLEAR, actionable warning naming the fix. NOTE: tauri_build's own
        // externalBin validation (in tauri_build::build(), called right after this)
        // HARD-FAILS any `cargo build`/`cargo test`/bundle when the sidecar is
        // missing — so `scripts/fetch-ollama-binaries.sh` is required even to run
        // `cargo test`. This check exists only to turn that late, opaque
        // "resource path ... doesn't exist" error into an up-front, fixable message.
        println!(
            "cargo:warning=Ollama sidecar binary not found at `{}`.",
            expected_path.display()
        );
        println!(
            "cargo:warning=Run `bash scripts/fetch-ollama-binaries.sh` from the repo root to download the bundled Ollama binaries before building or bundling the app."
        );
    }
}
