use std::ffi::{OsStr, OsString};
use std::sync::{LazyLock, Mutex, MutexGuard};

static PROCESS_ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub(crate) struct TestEnv {
    _guard: MutexGuard<'static, ()>,
    originals: Vec<(OsString, Option<OsString>)>,
}

impl TestEnv {
    pub(crate) fn new() -> Self {
        Self {
            _guard: PROCESS_ENV_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
            originals: Vec::new(),
        }
    }

    fn remember(&mut self, key: &OsStr) {
        if !self.originals.iter().any(|(saved, _)| saved == key) {
            self.originals
                .push((key.to_os_string(), std::env::var_os(key)));
        }
    }

    pub(crate) fn set(&mut self, key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) {
        let key = key.as_ref();
        self.remember(key);
        std::env::set_var(key, value);
    }

    pub(crate) fn remove(&mut self, key: impl AsRef<OsStr>) {
        let key = key.as_ref();
        self.remember(key);
        std::env::remove_var(key);
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        for (key, value) in self.originals.drain(..).rev() {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

#[test]
fn test_env_restores_original_values() {
    const KEY: &str = "CIVICNEWS_TEST_ENV_GUARD";
    {
        let mut env = TestEnv::new();
        env.set(KEY, "during");
        assert_eq!(std::env::var(KEY).as_deref(), Ok("during"));
    }
    let _env = TestEnv::new();
    assert_eq!(std::env::var_os(KEY), None);
}

#[test]
fn test_env_restores_after_panic_and_recovers_poisoned_lock() {
    const KEY: &str = "CIVICNEWS_TEST_ENV_GUARD_PANIC";
    let result = std::panic::catch_unwind(|| {
        let mut env = TestEnv::new();
        env.set(KEY, "during");
        panic!("exercise panic-safe environment restoration");
    });
    assert!(result.is_err());

    let _env = TestEnv::new();
    assert_eq!(std::env::var_os(KEY), None);
}
