// Config block: MAX_CONFIG_NAME_BYTES bounds profile names; default_dir() resolves
// %LOCALAPPDATA%\wallbang\configs with a portable fallback.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::constants::{CONFIG_EXTENSION, MAX_CONFIG_NAME_BYTES};
use crate::Settings;

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Filesystem-backed storage for named config profiles.
///
/// Profiles contain compact JSON but use the `.wb` extension so they are not
/// confused with ordinary JSON files.
#[derive(Clone, Debug)]
pub struct Storage {
    dir: PathBuf,
}

impl Storage {
    /// Creates a storage rooted at `dir` (the directory itself is not created yet).
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Creates the config directory, including missing parents.
    pub fn ensure_dir(&self) -> io::Result<()> {
        fs::create_dir_all(&self.dir)
    }

    /// Lists valid profile names (without the `.wb` extension), sorted.
    pub fn list(&self) -> io::Result<Vec<String>> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(err),
        };

        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some(CONFIG_EXTENSION) {
                continue;
            }
            if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                if Self::validate_name(stem) {
                    names.push(stem.to_owned());
                }
            }
        }
        names.sort();
        names.dedup();
        Ok(names)
    }

    /// True when a profile file exists on disk.
    pub fn exists(&self, name: &str) -> bool {
        self.path_for(name).map(|p| p.is_file()).unwrap_or(false)
    }

    /// Loads a profile; a missing file yields `Settings::default()`, broken JSON is an error.
    pub fn load(&self, name: &str) -> io::Result<Settings> {
        let path = self.path_for(name)?;
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Settings::default()),
            Err(err) => return Err(err),
        };
        serde_json::from_slice(&bytes).map_err(|err| io::Error::new(ErrorKind::InvalidData, err))
    }

    /// Atomically writes a compact JSON profile to `<dir>/<name>.wb`.
    pub fn save(&self, name: &str, settings: &Settings) -> io::Result<()> {
        let path = self.path_for(name)?;
        self.ensure_dir()?;
        let json =
            serde_json::to_vec(settings).map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

        let sequence = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let temp = self
            .dir
            .join(format!(
                ".{name}.{}.{}.{sequence}.tmp",
                CONFIG_EXTENSION,
                std::process::id()
            ));

        if let Err(err) = fs::write(&temp, &json) {
            let _ = fs::remove_file(&temp);
            return Err(err);
        }

        match fs::rename(&temp, &path) {
            Ok(()) => Ok(()),
            Err(err) => {
                // Some Windows configurations refuse to replace an existing file.
                if path.exists() {
                    fs::remove_file(&path)?;
                    let result = fs::rename(&temp, &path);
                    if result.is_err() {
                        let _ = fs::remove_file(&temp);
                    }
                    result
                } else {
                    let _ = fs::remove_file(&temp);
                    Err(err)
                }
            }
        }
    }

    /// Removes a profile file.
    pub fn remove(&self, name: &str) -> io::Result<()> {
        fs::remove_file(self.path_for(name)?)
    }

    /// Rejects empty names, `.`/`..`, illegal filename characters and oversized names.
    pub fn validate_name(name: &str) -> bool {
        if name.is_empty() || name.len() > MAX_CONFIG_NAME_BYTES {
            return false;
        }
        if name == "." || name == ".." {
            return false;
        }
        !name
            .chars()
            .any(|c| matches!(c, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'))
    }

    fn path_for(&self, name: &str) -> io::Result<PathBuf> {
        if !Self::validate_name(name) {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "invalid config profile name",
            ));
        }
        Ok(self.dir.join(format!("{name}.{CONFIG_EXTENSION}")))
    }
}

/// Platform-specific default config directory.
pub fn default_dir() -> PathBuf {
    match std::env::var_os("LOCALAPPDATA") {
        Some(local_app_data) if !local_app_data.is_empty() => {
            PathBuf::from(local_app_data).join("wallbang").join("configs")
        }
        _ => PathBuf::from("wallbang").join("configs"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rules() {
        assert!(Storage::validate_name("default"));
        assert!(!Storage::validate_name(""));
        assert!(!Storage::validate_name("."));
        assert!(!Storage::validate_name(".."));
        assert!(!Storage::validate_name("bad/name"));
        assert!(!Storage::validate_name("bad:name"));
        assert!(!Storage::validate_name(&"x".repeat(MAX_CONFIG_NAME_BYTES + 1)));
    }

    #[test]
    fn profiles_use_wb_extension() {
        let dir = std::env::temp_dir().join(format!(
            "wallbang_storage_test_{}_{}",
            std::process::id(),
            TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let storage = Storage::new(&dir);
        let mut settings = Settings::default();
        settings.esp.enabled = true;
        settings.misc.bhop = true;
        settings.misc.motion_blur.enabled = true;
        settings.ui.hit_logs = true;
        settings.esp.weapon = true;
        settings.esp.ammo = true;
        settings.visual.chams.enemy.enabled = true;

        storage.save("test", &settings).expect("save must work");
        assert!(dir.join("test.wb").is_file());
        assert!(storage.exists("test"));
        assert_eq!(storage.list().expect("list must work"), vec!["test".to_string()]);

        let loaded = storage.load("test").expect("load must work");
        assert!(loaded.esp.enabled);
        assert!(loaded.misc.bhop);
        assert!(loaded.misc.motion_blur.enabled);
        assert!(loaded.ui.hit_logs);
        assert!(loaded.esp.weapon);
        assert!(loaded.esp.ammo);
        assert!(loaded.visual.chams.enemy.enabled);

        let _ = fs::remove_dir_all(&dir);
    }
}
