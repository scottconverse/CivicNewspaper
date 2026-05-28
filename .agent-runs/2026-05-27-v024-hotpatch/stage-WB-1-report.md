# Stage WB-1 Report — Delete or fix src-tauri/build.rs

We modified `src-tauri/build.rs` to remove process-tree walking and the `sysinfo` dependency. We kept a minimal `build.rs` calling `tauri_build::build();` because completely removing the file causes the Tauri compiler to fail with an `OUT_DIR env var is not set` compilation error.

This is correct because the Tauri build script generator requires a `build.rs` to generate context files at build time, and the `tauri_build::build()` command is idempotent.

Cited evidence:
- Tauri Build documentation: https://docs.rs/tauri-build/latest/tauri_build/fn.build.html
