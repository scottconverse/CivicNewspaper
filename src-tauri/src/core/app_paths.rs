use std::path::{Path, PathBuf};

use tauri::{AppHandle, Manager, Runtime};

pub const APP_DATA_OVERRIDE_ENV: &str = "CIVICNEWS_APP_DATA_DIR";

fn app_data_override_dir() -> Result<Option<PathBuf>, String> {
    if let Some(raw) = std::env::var_os(APP_DATA_OVERRIDE_ENV) {
        let path = PathBuf::from(raw);
        if !path.is_absolute() {
            return Err(format!(
                "{APP_DATA_OVERRIDE_ENV} must be an absolute path for clean-profile tests"
            ));
        }
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        return Ok(Some(path));
    }
    Ok(None)
}

pub fn app_data_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    if let Some(path) = app_data_override_dir()? {
        ensure_standard_app_dirs(&path)?;
        return Ok(path);
    }

    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    ensure_standard_app_dirs(&path)?;
    Ok(path)
}

pub fn ensure_standard_app_dirs(app_data: &Path) -> Result<(), String> {
    for relative in [
        ["sites", "default"].as_slice(),
        ["backups"].as_slice(),
        ["logs"].as_slice(),
    ] {
        let mut path = app_data.to_path_buf();
        for part in relative {
            path.push(part);
        }
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn is_standard_site_path(app_data: &std::path::Path, path: &std::path::Path) -> bool {
    let site_root = app_data.join("sites");
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
        || !path.starts_with(&site_root)
    {
        return false;
    }

    let Ok(canonical_root) = site_root.canonicalize() else {
        return false;
    };
    let Some(existing_ancestor) = path.ancestors().find(|ancestor| ancestor.exists()) else {
        return false;
    };
    existing_ancestor
        .canonicalize()
        .is_ok_and(|ancestor| ancestor.starts_with(canonical_root))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{LazyLock, Mutex};
    use tempfile::tempdir;

    static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn app_data_override_requires_absolute_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::set_var(APP_DATA_OVERRIDE_ENV, "relative-profile");
        let result = app_data_override_dir();
        std::env::remove_var(APP_DATA_OVERRIDE_ENV);
        assert!(result.is_err());
    }

    #[test]
    fn app_data_override_creates_clean_profile_dir() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = tempdir().unwrap();
        let profile = root.path().join("clean-profile");
        std::env::set_var(APP_DATA_OVERRIDE_ENV, &profile);
        let result = app_data_override_dir().unwrap();
        std::env::remove_var(APP_DATA_OVERRIDE_ENV);
        assert_eq!(result.as_deref(), Some(profile.as_path()));
        assert!(profile.exists());
    }

    #[test]
    fn app_data_dir_creates_standard_publish_and_support_dirs() {
        let _guard = ENV_LOCK.lock().unwrap();
        let root = tempdir().unwrap();
        let profile = root.path().join("clean-profile");
        std::env::set_var(APP_DATA_OVERRIDE_ENV, &profile);
        let result = app_data_override_dir().unwrap().unwrap();
        ensure_standard_app_dirs(&result).unwrap();
        std::env::remove_var(APP_DATA_OVERRIDE_ENV);

        assert!(profile.join("sites").join("default").is_dir());
        assert!(profile.join("backups").is_dir());
        assert!(profile.join("logs").is_dir());
    }

    #[test]
    fn standard_site_path_detection_is_scoped_to_sites_folder() {
        let root = tempdir().unwrap();
        let app_data = root.path().join("app-data");
        ensure_standard_app_dirs(&app_data).unwrap();
        assert!(is_standard_site_path(
            &app_data,
            &app_data.join("sites").join("default")
        ));
        assert!(is_standard_site_path(
            &app_data,
            &app_data.join("sites").join("default").join("issue-001")
        ));
        assert!(!is_standard_site_path(
            &app_data,
            &app_data.join("backups").join("default")
        ));
        assert!(!is_standard_site_path(
            &app_data,
            &app_data.join("sites-archive").join("new")
        ));
        assert!(!is_standard_site_path(
            &app_data,
            &app_data.join("sites").join("..").join("backups")
        ));
    }
}
