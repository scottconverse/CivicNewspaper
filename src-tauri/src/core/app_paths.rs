use std::path::PathBuf;

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
        return Ok(path);
    }

    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn app_data_override_requires_absolute_path() {
        std::env::set_var(APP_DATA_OVERRIDE_ENV, "relative-profile");
        let result = app_data_override_dir();
        std::env::remove_var(APP_DATA_OVERRIDE_ENV);
        assert!(result.is_err());
    }

    #[test]
    fn app_data_override_creates_clean_profile_dir() {
        let root = tempdir().unwrap();
        let profile = root.path().join("clean-profile");
        std::env::set_var(APP_DATA_OVERRIDE_ENV, &profile);
        let result = app_data_override_dir().unwrap();
        std::env::remove_var(APP_DATA_OVERRIDE_ENV);
        assert_eq!(result.as_deref(), Some(profile.as_path()));
        assert!(profile.exists());
    }
}
