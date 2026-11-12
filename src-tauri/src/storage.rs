use crate::models::{Entry, Settings, Theme};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use tauri::{AppHandle, Manager};

/// Serialises every read-modify-write of `library.json`. Without it, a stats
/// update from a background "game closed" task can race with a save from the
/// UI and silently drop one of the two writes.
static LIBRARY_LOCK: Mutex<()> = Mutex::new(());

pub fn library_lock() -> MutexGuard<'static, ()> {
    LIBRARY_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn data_dir(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn media_dir(app: &AppHandle) -> PathBuf {
    data_dir(app).join("media")
}

pub fn themes_dir(app: &AppHandle) -> PathBuf {
    data_dir(app).join("themes")
}

pub fn ensure_dirs(app: &AppHandle) {
    let _ = fs::create_dir_all(data_dir(app));
    let _ = fs::create_dir_all(media_dir(app));
    let _ = fs::create_dir_all(themes_dir(app));
}

/// Reads JSON. If the file exists but cannot be parsed, it is moved aside to
/// `<name>.corrupt-<timestamp>` so the next save does not silently overwrite
/// the user's data with an empty default.
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    let data = fs::read_to_string(path).ok()?;
    match serde_json::from_str(&data) {
        Ok(v) => Some(v),
        Err(e) => {
            let stamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
            let mut backup = path.as_os_str().to_owned();
            backup.push(format!(".corrupt-{stamp}"));
            let _ = fs::copy(path, PathBuf::from(&backup));
            eprintln!("lumen: failed to parse {}: {e}; backed up", path.display());
            None
        }
    }
}

/// Writes to a temp file then renames over the target, so a crash or power
/// loss mid-write never leaves a truncated JSON file behind.
fn write_json<T: serde::Serialize + ?Sized>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, data).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

pub fn load_library(app: &AppHandle) -> Vec<Entry> {
    ensure_dirs(app);
    read_json(&data_dir(app).join("library.json")).unwrap_or_default()
}

pub fn save_library(app: &AppHandle, entries: &[Entry]) -> Result<(), String> {
    ensure_dirs(app);
    write_json(&data_dir(app).join("library.json"), entries)
}

/// Locked read-modify-write of the library.
pub fn update_library<R>(
    app: &AppHandle,
    f: impl FnOnce(&mut Vec<Entry>) -> R,
) -> Result<R, String> {
    let _g = library_lock();
    let mut entries = load_library(app);
    let r = f(&mut entries);
    save_library(app, &entries)?;
    Ok(r)
}

pub fn load_settings(app: &AppHandle) -> Settings {
    ensure_dirs(app);
    read_json(&data_dir(app).join("settings.json")).unwrap_or_default()
}

pub fn save_settings(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    ensure_dirs(app);
    write_json(&data_dir(app).join("settings.json"), settings)
}

fn theme_file(app: &AppHandle, name: &str) -> PathBuf {
    let safe = name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_");
    themes_dir(app).join(format!("{}.json", safe))
}

pub fn list_themes(app: &AppHandle) -> Vec<Theme> {
    ensure_dirs(app);
    let mut themes = Vec::new();
    if let Ok(entries) = fs::read_dir(themes_dir(app)) {
        for e in entries.flatten() {
            let path = e.path();
            let ext = path.extension().and_then(|s| s.to_str());
            if matches!(ext, Some("json") | Some("lumentheme")) {
                if let Some(t) = read_json::<Theme>(&path) {
                    themes.push(t);
                }
            }
        }
    }
    themes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    if themes.is_empty() {
        themes.push(Theme::default());
    }
    themes
}

pub fn save_theme(app: &AppHandle, theme: &Theme) -> Result<(), String> {
    ensure_dirs(app);
    write_json(&theme_file(app, &theme.name), theme)
}

pub fn delete_theme(app: &AppHandle, name: &str) -> Result<(), String> {
    let path = theme_file(app, name);
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}
