use std::path::{Path, PathBuf};

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use serde::{Deserialize, Serialize};

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

pub struct SafeWriteDestination {
    root: Dir,
    root_path: PathBuf,
    relative: PathBuf,
    absolute: PathBuf,
}

impl SafeWriteDestination {
    pub fn path(&self) -> &Path {
        &self.absolute
    }

    pub fn write(&self, contents: impl AsRef<[u8]>) -> Result<(), String> {
        self.atomic_write(contents.as_ref(), false, false)
    }

    fn atomic_write(
        &self,
        contents: &[u8],
        fail_before_rename: bool,
        fail_cleanup: bool,
    ) -> Result<(), String> {
        if let Some(parent) = self.relative.parent() {
            self.root
                .create_dir_all(parent)
                .map_err(|e| e.to_string())?;
        }
        let parent = self.relative.parent().unwrap_or_else(|| Path::new(""));
        let temp = parent.join(format!(".civicdesk-write-{}.tmp", uuid::Uuid::new_v4()));
        let result = (|| {
            let mut file = self.root.create(&temp).map_err(|e| e.to_string())?;
            use std::io::Write;
            file.write_all(contents).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            drop(file);
            if fail_before_rename {
                return Err("injected atomic write failure".to_string());
            }
            self.root
                .rename(&temp, &self.root, &self.relative)
                .map_err(|e| e.to_string())
        })();
        if let Err(error) = result {
            if let Err(cleanup) = cleanup_capability_temp(&self.root, &temp, fail_cleanup) {
                return Err(format!("{error} Cleanup also failed: {cleanup}"));
            }
            return Err(error);
        }
        Ok(())
    }

    pub fn copy_from(&self, source: &Path) -> Result<(), String> {
        self.copy_from_internal(source, false, false)
    }

    fn copy_from_internal(
        &self,
        source: &Path,
        fail_before_rename: bool,
        fail_cleanup: bool,
    ) -> Result<(), String> {
        if let Some(parent) = self.relative.parent() {
            self.root
                .create_dir_all(parent)
                .map_err(|e| e.to_string())?;
        }
        let mut source = std::fs::File::open(source).map_err(|e| e.to_string())?;
        let parent = self.relative.parent().unwrap_or_else(|| Path::new(""));
        let temp = parent.join(format!(".civicdesk-copy-{}.tmp", uuid::Uuid::new_v4()));
        let result = (|| {
            let mut destination = self.root.create(&temp).map_err(|e| e.to_string())?;
            std::io::copy(&mut source, &mut destination).map_err(|e| e.to_string())?;
            destination.sync_all().map_err(|e| e.to_string())?;
            drop(destination);
            if fail_before_rename {
                return Err("injected atomic copy failure".to_string());
            }
            self.root
                .rename(&temp, &self.root, &self.relative)
                .map_err(|e| e.to_string())
        })();
        if let Err(error) = result {
            if let Err(cleanup) = cleanup_capability_temp(&self.root, &temp, fail_cleanup) {
                return Err(format!("{error} Cleanup also failed: {cleanup}"));
            }
            return Err(error);
        }
        Ok(())
    }

    pub fn prepare_directory(&self) -> Result<(), String> {
        self.root
            .create_dir_all(&self.relative)
            .map_err(|e| e.to_string())
    }

    pub fn snapshot_tree_to(&self, destination: &Path) -> Result<(), String> {
        let source = self
            .root
            .open_dir(&self.relative)
            .map_err(|e| e.to_string())?;
        let _ = std::fs::remove_dir_all(destination);
        std::fs::create_dir_all(destination).map_err(|e| e.to_string())?;
        copy_capability_tree_to_ambient(&source, destination)
    }

    #[allow(dead_code)]
    pub fn install_tree_from(&self, source: &Path, app_data: &Path) -> Result<(), String> {
        self.install_tree_from_with_commit(source, app_data, |_| Ok(()))
    }

    pub fn install_tree_from_with_commit<F>(
        &self,
        source: &Path,
        app_data: &Path,
        commit: F,
    ) -> Result<(), String>
    where
        F: FnOnce(&str) -> Result<(), String>,
    {
        self.install_tree_from_with_commit_internal(source, app_data, commit, false)
    }

    fn install_tree_from_with_commit_internal<F>(
        &self,
        source: &Path,
        app_data: &Path,
        commit: F,
        fail_postcommit_housekeeping: bool,
    ) -> Result<(), String>
    where
        F: FnOnce(&str) -> Result<(), String>,
    {
        let parent = self.relative.parent().unwrap_or_else(|| Path::new(""));
        let unique = uuid::Uuid::new_v4();
        let staging = parent.join(format!(".civicdesk-cap-staging-{unique}"));
        let rollback = parent.join(format!(".civicdesk-cap-rollback-{unique}"));
        copy_tree_into_capability(&self.root, source, &staging)?;
        let journal_dir = app_data.join("publication-journals");
        std::fs::create_dir_all(&journal_dir).map_err(|e| e.to_string())?;
        let journal_path = journal_dir.join(format!("{unique}.json"));
        let mut journal = CapabilitySwapJournal {
            id: unique.to_string(),
            root: self.root_path.clone(),
            destination: self.relative.clone(),
            staging: staging.clone(),
            rollback: rollback.clone(),
            phase: "prepared".to_string(),
        };
        write_capability_journal(&journal_path, &journal)?;
        let had_previous = self.root.metadata(&self.relative).is_ok();
        if had_previous {
            self.root
                .rename(&self.relative, &self.root, &rollback)
                .map_err(|e| e.to_string())?;
        }
        journal.phase = "old_moved".to_string();
        write_capability_journal(&journal_path, &journal)?;
        if let Err(error) = self.root.rename(&staging, &self.root, &self.relative) {
            if had_previous {
                let _ = self.root.rename(&rollback, &self.root, &self.relative);
            }
            let _ = self.root.remove_dir_all(&staging);
            return Err(error.to_string());
        }
        journal.phase = "new_installed".to_string();
        write_capability_journal(&journal_path, &journal)?;
        if let Err(error) = commit(&journal.id) {
            let _ = self.root.remove_dir_all(&self.relative);
            if had_previous {
                let _ = self.root.rename(&rollback, &self.root, &self.relative);
            }
            let _ = self.root.remove_dir_all(&staging);
            let _ = std::fs::remove_file(&journal_path);
            return Err(error);
        }
        journal.phase = "db_committed".to_string();
        if fail_postcommit_housekeeping {
            eprintln!("Publication committed; injected post-commit housekeeping failure. Recovery journal retained.");
            return Ok(());
        }
        if let Err(error) = write_capability_journal(&journal_path, &journal) {
            eprintln!("Publication committed, but its recovery journal could not be finalized: {error}. The journal and rollback are retained for startup recovery.");
            return Ok(());
        }
        if had_previous {
            if let Err(error) = self.root.remove_dir_all(&rollback) {
                eprintln!("Publication committed, but the previous tree could not be removed: {error}. The recovery journal is retained for startup cleanup.");
                return Ok(());
            }
        }
        if let Err(error) = std::fs::remove_file(&journal_path) {
            eprintln!("Publication committed, but its completed recovery journal could not be removed: {error}. Startup will retry cleanup.");
        }
        Ok(())
    }
}

fn cleanup_capability_temp(root: &Dir, temp: &Path, fail_cleanup: bool) -> Result<(), String> {
    if !fail_cleanup {
        match root.remove_file(temp) {
            Ok(()) => return Ok(()),
            Err(_error) if root.metadata(temp).is_err() => return Ok(()),
            Err(_) => {}
        }
    }
    let parent = temp.parent().unwrap_or_else(|| Path::new(""));
    let quarantine = parent.join(format!(
        ".civicdesk-cleanup-pending-{}",
        uuid::Uuid::new_v4()
    ));
    root.rename(temp, root, &quarantine)
        .map_err(|rename_error| {
            format!(
                "temporary file remains at `{}` and could not be quarantined: {rename_error}",
                temp.display()
            )
        })?;
    Err(format!(
        "temporary file was quarantined at `{}`",
        quarantine.display()
    ))
}

#[derive(Serialize, Deserialize)]
struct CapabilitySwapJournal {
    id: String,
    root: PathBuf,
    destination: PathBuf,
    staging: PathBuf,
    rollback: PathBuf,
    phase: String,
}

fn write_capability_journal(path: &Path, journal: &CapabilitySwapJournal) -> Result<(), String> {
    let temp = path.with_extension("tmp");
    let mut file = std::fs::File::create(&temp).map_err(|e| e.to_string())?;
    use std::io::Write;
    file.write_all(
        serde_json::to_string(journal)
            .map_err(|e| e.to_string())?
            .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    std::fs::rename(temp, path).map_err(|e| e.to_string())
}

pub fn recover_capability_publications(
    allowed_roots: &[PathBuf],
    app_data: &Path,
    is_committed: impl Fn(&str) -> bool,
) -> Result<(), String> {
    let journal_dir = app_data.join("publication-journals");
    if !journal_dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(&journal_dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().and_then(|v| v.to_str()) != Some("json") {
            continue;
        }
        let journal: CapabilitySwapJournal =
            serde_json::from_str(&std::fs::read_to_string(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let destination_path = journal.root.join(&journal.destination);
        let destination = open_safe_write_destination(allowed_roots, &destination_path)?;
        if destination.root_path != journal.root
            || journal.staging.parent() != journal.destination.parent()
            || journal.rollback.parent() != journal.destination.parent()
            || !journal.staging.file_name().is_some_and(|name| {
                name.to_string_lossy()
                    .starts_with(".civicdesk-cap-staging-")
            })
            || !journal.rollback.file_name().is_some_and(|name| {
                name.to_string_lossy()
                    .starts_with(".civicdesk-cap-rollback-")
            })
        {
            return Err("Invalid capability publication journal".to_string());
        }
        let committed = is_committed(&journal.id);
        if !committed && destination.root.metadata(&journal.rollback).is_ok() {
            let _ = destination.root.remove_dir_all(&journal.destination);
            destination
                .root
                .rename(&journal.rollback, &destination.root, &journal.destination)
                .map_err(|e| e.to_string())?;
        }
        if committed && destination.root.metadata(&journal.destination).is_ok()
            || !committed && destination.root.metadata(&journal.destination).is_ok()
        {
            let _ = destination.root.remove_dir_all(&journal.rollback);
            let _ = destination.root.remove_dir_all(&journal.staging);
            std::fs::remove_file(path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn copy_tree_into_capability(root: &Dir, source: &Path, destination: &Path) -> Result<(), String> {
    root.create_dir_all(destination)
        .map_err(|e| e.to_string())?;
    for entry in std::fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let target = destination.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_tree_into_capability(root, &entry.path(), &target)?;
        } else {
            let mut input = std::fs::File::open(entry.path()).map_err(|e| e.to_string())?;
            let mut output = root.create(&target).map_err(|e| e.to_string())?;
            std::io::copy(&mut input, &mut output).map_err(|e| e.to_string())?;
            output.sync_all().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn copy_capability_tree_to_ambient(source: &Dir, destination: &Path) -> Result<(), String> {
    for entry in source.entries().map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let target = destination.join(&name);
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            let child = source.open_dir(&name).map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
            copy_capability_tree_to_ambient(&child, &target)?;
        } else {
            let mut input = source.open(&name).map_err(|e| e.to_string())?;
            let mut output = std::fs::File::create(&target).map_err(|e| e.to_string())?;
            std::io::copy(&mut input, &mut output).map_err(|e| e.to_string())?;
            output.sync_all().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn open_safe_write_destination(
    allowed_roots: &[PathBuf],
    requested: &Path,
) -> Result<SafeWriteDestination, String> {
    let absolute = validate_write_destination(allowed_roots, requested)?;
    for allowed_root in allowed_roots {
        let canonical_root = allowed_root.canonicalize().map_err(|e| {
            format!(
                "Invalid allowed directory `{}`: {e}",
                allowed_root.display()
            )
        })?;
        let Ok(relative) = absolute.strip_prefix(&canonical_root) else {
            continue;
        };
        let root = Dir::open_ambient_dir(&canonical_root, ambient_authority())
            .map_err(|e| format!("Could not securely open allowed directory: {e}"))?;
        return Ok(SafeWriteDestination {
            root,
            root_path: canonical_root,
            relative: relative.to_path_buf(),
            absolute,
        });
    }
    Err("Write destination is outside allowed directories.".to_string())
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

    #[test]
    fn opened_write_destination_rejects_junction_swapped_after_validation() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let child = allowed.join("exports");
        let outside = root.path().join("outside");
        std::fs::create_dir_all(&child).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &child.join("data.csv"))
                .unwrap();

        std::fs::remove_dir(&child).unwrap();
        symlink_dir(&outside, &child);

        assert!(destination.write("private data").is_err());
        assert!(!outside.join("data.csv").exists());
    }

    #[test]
    fn atomic_file_write_failure_preserves_previous_good_file_and_cleans_temp() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        std::fs::create_dir_all(&allowed).unwrap();
        let path = allowed.join("subscribers.csv");
        std::fs::write(&path, "previous good").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &path).unwrap();

        assert!(destination
            .atomic_write(b"partial replacement", true, false)
            .is_err());

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "previous good");
        assert!(std::fs::read_dir(allowed).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".civicdesk-write-")));
    }

    #[test]
    fn atomic_file_write_replaces_existing_file_on_windows() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        std::fs::create_dir_all(&allowed).unwrap();
        let path = allowed.join("subscribers.csv");
        std::fs::write(&path, "old").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &path).unwrap();

        destination.write("new").unwrap();

        assert_eq!(std::fs::read_to_string(path).unwrap(), "new");
    }

    #[test]
    fn atomic_file_copy_failure_preserves_previous_good_file_and_cleans_temp() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        std::fs::create_dir_all(&allowed).unwrap();
        let path = allowed.join("backup.sqlite");
        let source = root.path().join("candidate.sqlite");
        std::fs::write(&path, "previous backup").unwrap();
        std::fs::write(&source, "partial replacement").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &path).unwrap();

        assert!(destination
            .copy_from_internal(&source, true, false)
            .is_err());

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "previous backup");
        assert!(std::fs::read_dir(allowed).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".civicdesk-copy-")));
    }

    #[test]
    fn atomic_write_cleanup_failure_is_reported_and_uniquely_quarantined() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        std::fs::create_dir_all(&allowed).unwrap();
        let path = allowed.join("export.csv");
        std::fs::write(&path, "old").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &path).unwrap();

        let error = destination
            .atomic_write(b"sensitive partial", true, true)
            .unwrap_err();

        assert!(error.contains("quarantined"));
        assert_eq!(std::fs::read_to_string(path).unwrap(), "old");
        let quarantines = std::fs::read_dir(allowed)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".civicdesk-cleanup-pending-")
            })
            .count();
        assert_eq!(quarantines, 1);
    }

    #[test]
    fn atomic_copy_cleanup_failure_is_reported_and_uniquely_quarantined() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        std::fs::create_dir_all(&allowed).unwrap();
        let path = allowed.join("backup.sqlite");
        let source = root.path().join("candidate.sqlite");
        std::fs::write(&path, "old backup").unwrap();
        std::fs::write(&source, "sensitive partial").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &path).unwrap();

        let error = destination
            .copy_from_internal(&source, true, true)
            .unwrap_err();

        assert!(error.contains("quarantined"));
        assert_eq!(std::fs::read_to_string(path).unwrap(), "old backup");
        assert_eq!(
            std::fs::read_dir(allowed)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".civicdesk-cleanup-pending-"))
                .count(),
            1
        );
    }

    #[test]
    fn capability_tree_install_never_follows_swapped_destination_junction() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let destination_path = allowed.join("site");
        let outside = root.path().join("outside");
        let candidate = root.path().join("candidate");
        std::fs::create_dir_all(&destination_path).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::create_dir_all(&candidate).unwrap();
        std::fs::write(outside.join("sentinel.txt"), "outside").unwrap();
        std::fs::write(candidate.join("index.html"), "candidate").unwrap();
        let destination = open_safe_write_destination(&[allowed], &destination_path).unwrap();
        std::fs::remove_dir(&destination_path).unwrap();
        symlink_dir(&outside, &destination_path);

        assert!(destination
            .install_tree_from(&candidate, &root.path().join("app-data"))
            .is_err());
        assert_eq!(
            std::fs::read_to_string(outside.join("sentinel.txt")).unwrap(),
            "outside"
        );
        assert!(!outside.join("index.html").exists());
    }

    #[test]
    fn capability_tree_install_replaces_site_and_cleans_journal() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let destination_path = allowed.join("site");
        let candidate = root.path().join("candidate");
        let app_data = root.path().join("app-data");
        std::fs::create_dir_all(&destination_path).unwrap();
        std::fs::create_dir_all(&candidate).unwrap();
        std::fs::write(destination_path.join("index.html"), "old").unwrap();
        std::fs::write(candidate.join("index.html"), "new").unwrap();
        let destination = open_safe_write_destination(&[allowed], &destination_path).unwrap();

        destination
            .install_tree_from(&candidate, &app_data)
            .unwrap();

        assert_eq!(
            std::fs::read_to_string(destination_path.join("index.html")).unwrap(),
            "new"
        );
        assert!(std::fs::read_dir(app_data.join("publication-journals"))
            .unwrap()
            .next()
            .is_none());
    }

    #[test]
    fn capability_tree_commit_failure_restores_previous_site() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let destination_path = allowed.join("site");
        let candidate = root.path().join("candidate");
        let app_data = root.path().join("app-data");
        std::fs::create_dir_all(&destination_path).unwrap();
        std::fs::create_dir_all(&candidate).unwrap();
        std::fs::write(destination_path.join("index.html"), "old").unwrap();
        std::fs::write(candidate.join("index.html"), "new").unwrap();
        let destination = open_safe_write_destination(&[allowed], &destination_path).unwrap();

        let result = destination.install_tree_from_with_commit(&candidate, &app_data, |_| {
            Err("injected database release failure".to_string())
        });

        assert!(result.unwrap_err().contains("database release"));
        assert_eq!(
            std::fs::read_to_string(destination_path.join("index.html")).unwrap(),
            "old"
        );
    }

    #[test]
    fn postcommit_housekeeping_failure_is_success_with_recovery_journal_retained() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let destination_path = allowed.join("site");
        let candidate = root.path().join("candidate");
        let app_data = root.path().join("app-data");
        std::fs::create_dir_all(&destination_path).unwrap();
        std::fs::create_dir_all(&candidate).unwrap();
        std::fs::write(destination_path.join("index.html"), "old").unwrap();
        std::fs::write(candidate.join("index.html"), "new").unwrap();
        let destination =
            open_safe_write_destination(std::slice::from_ref(&allowed), &destination_path).unwrap();

        let result = destination.install_tree_from_with_commit_internal(
            &candidate,
            &app_data,
            |_| Ok(()),
            true,
        );

        assert!(
            result.is_ok(),
            "durably committed publish must not request retry"
        );
        assert_eq!(
            std::fs::read_to_string(destination_path.join("index.html")).unwrap(),
            "new"
        );
        assert!(std::fs::read_dir(app_data.join("publication-journals"))
            .unwrap()
            .next()
            .is_some());
        assert!(std::fs::read_dir(allowed)
            .unwrap()
            .filter_map(Result::ok)
            .any(|entry| entry
                .file_name()
                .to_string_lossy()
                .starts_with(".civicdesk-cap-rollback-")));
    }

    #[test]
    fn capability_publication_restart_recovers_old_moved_phase() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let app_data = root.path().join("app-data");
        let destination = allowed.join("site");
        std::fs::create_dir_all(&destination).unwrap();
        std::fs::create_dir_all(&app_data).unwrap();
        std::fs::write(destination.join("index.html"), "last good").unwrap();
        let rollback = PathBuf::from(".civicdesk-cap-rollback-restart");
        let staging = PathBuf::from(".civicdesk-cap-staging-restart");
        let root_dir = Dir::open_ambient_dir(&allowed, ambient_authority()).unwrap();
        root_dir.rename("site", &root_dir, &rollback).unwrap();
        root_dir.create_dir_all(&staging).unwrap();
        let journal_dir = app_data.join("publication-journals");
        std::fs::create_dir_all(&journal_dir).unwrap();
        write_capability_journal(
            &journal_dir.join("restart.json"),
            &CapabilitySwapJournal {
                id: "restart".to_string(),
                root: allowed.canonicalize().unwrap(),
                destination: PathBuf::from("site"),
                staging,
                rollback,
                phase: "old_moved".to_string(),
            },
        )
        .unwrap();

        recover_capability_publications(&[allowed], &app_data, |_| false).unwrap();

        assert_eq!(
            std::fs::read_to_string(destination.join("index.html")).unwrap(),
            "last good"
        );
        assert!(std::fs::read_dir(journal_dir).unwrap().next().is_none());
    }

    #[test]
    fn capability_publication_restart_keeps_new_tree_only_with_db_commit_marker() {
        let root = tempdir().unwrap();
        let allowed = root.path().join("downloads");
        let app_data = root.path().join("app-data");
        let destination = allowed.join("site");
        std::fs::create_dir_all(&destination).unwrap();
        std::fs::create_dir_all(&app_data).unwrap();
        std::fs::write(destination.join("index.html"), "new").unwrap();
        let rollback = PathBuf::from(".civicdesk-cap-rollback-committed");
        let staging = PathBuf::from(".civicdesk-cap-staging-committed");
        let root_dir = Dir::open_ambient_dir(&allowed, ambient_authority()).unwrap();
        root_dir.create_dir_all(&rollback).unwrap();
        std::fs::write(allowed.join(&rollback).join("index.html"), "old").unwrap();
        let journal_dir = app_data.join("publication-journals");
        std::fs::create_dir_all(&journal_dir).unwrap();
        write_capability_journal(
            &journal_dir.join("committed.json"),
            &CapabilitySwapJournal {
                id: "committed".to_string(),
                root: allowed.canonicalize().unwrap(),
                destination: PathBuf::from("site"),
                staging,
                rollback,
                phase: "new_installed".to_string(),
            },
        )
        .unwrap();

        recover_capability_publications(&[allowed], &app_data, |id| id == "committed").unwrap();

        assert_eq!(
            std::fs::read_to_string(destination.join("index.html")).unwrap(),
            "new"
        );
        assert!(std::fs::read_dir(journal_dir).unwrap().next().is_none());
    }
}
