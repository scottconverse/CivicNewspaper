# Stage 04 Version Bump Report

The application version has been bumped from `0.1.1` to `0.2.0` across the project files, and the auto-updater has been marked as inactive for this release.

## Version Bump Details
- **Cargo.toml**: Version bumped to `0.2.0` (dependency `civicnews` version updated in `Cargo.lock` by running `cargo update -p civicnews`).
- **package.json**: Version bumped to `0.2.0`.
- **tauri.conf.json**: Version bumped to `0.2.0`.

## Product Name Reconcile
- The repository name is `CivicNewspaper`. The `productName` in `tauri.conf.json` was previously `CivicNews`.
- Reconciled the name by changing `"productName": "CivicNewspaper"` in `tauri.conf.json`.

## Auto-Updater Status
- Auto-updater is explicitly **disabled** for `v0.2.0` by setting `plugins.updater.active = false` under `plugins.updater` in `tauri.conf.json`.
- The plugin dependency remains in place to avoid unnecessary churn.
- This configuration is tracked as **P5-002** in `carried-debt.md`.
