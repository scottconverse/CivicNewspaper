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

pub fn validate_write_destination(
    allowed_roots: &[PathBuf],
    requested: &Path,
) -> Result<PathBuf, String> {
    if !requested.is_absolute()
        || requested
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(
            "Write destination must be an absolute path inside an allowed directory.".to_string(),
        );
    }

    let canonical_roots = allowed_roots
        .iter()
        .map(|root| {
            root.canonicalize()
                .map_err(|e| format!("Invalid allowed directory `{}`: {e}", root.display()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let existing_ancestor = requested
        .ancestors()
        .find(|ancestor| ancestor.exists())
        .ok_or_else(|| "Write destination has no existing parent directory.".to_string())?;
    let canonical_ancestor = existing_ancestor
        .canonicalize()
        .map_err(|e| format!("Invalid write destination: {e}"))?;

    if !canonical_roots
        .iter()
        .any(|root| canonical_ancestor.starts_with(root))
    {
        return Err("Write destination is outside allowed directories.".to_string());
    }

    let suffix = requested
        .strip_prefix(existing_ancestor)
        .map_err(|_| "Invalid write destination.".to_string())?;
    Ok(canonical_ancestor.join(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::TestEnv;
    use tempfile::tempdir;

    #[cfg(unix)]
    fn symlink_dir(target: &Path, link: &Path) {
        std::os::unix::fs::symlink(target, link).unwrap();
    }

    #[cfg(windows)]
    fn symlink_dir(target: &Path, link: &Path) {
        let status = std::process::Command::new("cmd")
            .args(["/d", "/c", "mklink", "/J"])
            .arg(link)
            .arg(target)
            .status()
            .unwrap();
        assert!(status.success(), "failed to create test junction");
    }

    #[test]
    fn app_data_override_requires_absolute_path() {
        let mut env = TestEnv::new();
        env.set(APP_DATA_OVERRIDE_ENV, "relative-profile");
        let result = app_data_override_dir();
        assert!(result.is_err());
    }

    #[test]
    fn app_data_override_creates_clean_profile_dir() {
        let mut env = TestEnv::new();
        let root = tempdir().unwrap();
        let profile = root.path().join("clean-profile");
        env.set(APP_DATA_OVERRIDE_ENV, &profile);
        let result = app_data_override_dir().unwrap();
        assert_eq!(result.as_deref(), Some(profile.as_path()));
        assert!(profile.exists());
    }

    #[test]
    fn app_data_dir_creates_standard_publish_and_support_dirs() {
        let mut env = TestEnv::new();
        let root = tempdir().unwrap();
        let profile = root.path().join("clean-profile");
        env.set(APP_DATA_OVERRIDE_ENV, &profile);
        let result = app_data_override_dir().unwrap().unwrap();
        ensure_standard_app_dirs(&result).unwrap();

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

    #[test]
    fn write_destination_accepts_allowed_roots_and_nonexistent_children() {
        let root = tempdir().unwrap();
        let app_data = root.path().join("app-data");
        let downloads = root.path().join("downloads");
        std::fs::create_dir_all(&app_data).unwrap();
        std::fs::create_dir_all(&downloads).unwrap();

        let file = downloads.join("exports").join("subscribers.csv");
        let directory = app_data.join("sites").join("next-issue");

        assert_eq!(
            validate_write_destination(&[app_data.clone(), downloads.clone()], &file).unwrap(),
            downloads
                .canonicalize()
                .unwrap()
                .join("exports")
                .join("subscribers.csv")
        );
        assert_eq!(
            validate_write_destination(&[app_data, downloads], &directory).unwrap(),
            directory
                .ancestors()
                .find(|ancestor| ancestor.exists())
                .unwrap()
                .canonicalize()
                .unwrap()
                .join("sites")
                .join("next-issue")
        );
    }

    #[test]
    fn write_destination_rejects_sibling_and_traversal_escapes() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let sibling = root.path().join("downloads-archive");
        std::fs::create_dir_all(&allowed).unwrap();
        std::fs::create_dir_all(&sibling).unwrap();

        assert!(validate_write_destination(
            std::slice::from_ref(&allowed),
            &sibling.join("export.csv")
        )
        .is_err());
        assert!(validate_write_destination(
            std::slice::from_ref(&allowed),
            &allowed.join("..").join("outside").join("export.csv")
        )
        .is_err());
        assert!(validate_write_destination(&[allowed], Path::new("relative.csv")).is_err());
    }

    #[test]
    fn write_destination_rejects_symlink_escape() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let outside = root.path().join("outside");
        std::fs::create_dir_all(&allowed).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        let escape = allowed.join("escape");
        symlink_dir(&outside, &escape);

        assert!(validate_write_destination(&[allowed], &escape.join("export.csv")).is_err());
    }
}
