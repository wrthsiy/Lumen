use crate::models::{Entry, Settings, Theme};
use crate::storage;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_opener::OpenerExt;
use uuid::Uuid;

#[tauri::command]
pub fn list_entries(app: AppHandle) -> Vec<Entry> {
    let _g = storage::library_lock();
    storage::load_library(&app)
}

#[tauri::command]
pub fn save_entry(app: AppHandle, entry: Entry) -> Result<Entry, String> {
    storage::update_library(&app, |entries| {
        if let Some(existing) = entries.iter_mut().find(|e| e.id == entry.id) {
            // Stats are owned by the backend: never let a stale copy from the UI
            // overwrite playtime that was recorded while the editor was open.
            let stats = existing.stats.clone();
            *existing = entry.clone();
            existing.stats = stats;
            existing.clone()
        } else {
            entries.push(entry.clone());
            entry.clone()
        }
    })
}

#[tauri::command]
pub fn delete_entry(app: AppHandle, id: String) -> Result<(), String> {
    storage::update_library(&app, |entries| entries.retain(|e| e.id != id))?;
    // Remove extracted media (icons) for this entry; ignore errors.
    let media = storage::media_dir(&app).join(&id);
    if media.starts_with(storage::media_dir(&app)) && media.is_dir() {
        let _ = std::fs::remove_dir_all(media);
    }
    Ok(())
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Settings {
    storage::load_settings(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let old = storage::load_settings(&app);
    storage::save_settings(&app, &settings)?;
    if old.hotkey != settings.hotkey {
        crate::register_hotkey(&app, &settings.hotkey)?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_themes(app: AppHandle) -> Vec<Theme> {
    storage::list_themes(&app)
}

#[tauri::command]
pub fn save_theme(app: AppHandle, theme: Theme) -> Result<(), String> {
    storage::save_theme(&app, &theme)
}

#[tauri::command]
pub fn delete_theme(app: AppHandle, name: String) -> Result<(), String> {
    storage::delete_theme(&app, &name)
}

/// A `Command` that never flashes a console window on Windows.
fn hidden_command(program: &str) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

fn ps_quote(s: &str) -> String {
    s.replace('\'', "''")
}

fn resolve_lnk(path: &str) -> Option<String> {
    let script = format!(
        "$sh=New-Object -ComObject WScript.Shell; $sc=$sh.CreateShortcut('{}'); $sc.TargetPath",
        ps_quote(path)
    );
    let out = hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn extract_icon_ps(target: &str, output: &str) -> bool {
    let script = format!(
        "Add-Type -AssemblyName System.Drawing; $icon = [System.Drawing.Icon]::ExtractAssociatedIcon('{}'); if ($icon) {{ $bmp = $icon.ToBitmap(); $bmp.Save('{}', [System.Drawing.Imaging.ImageFormat]::Png); $icon.Dispose(); $bmp.Dispose(); }}",
        ps_quote(target),
        ps_quote(output)
    );
    hidden_command("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn lower_ext(p: &Path) -> Option<String> {
    p.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase())
}

#[tauri::command]
pub async fn create_entry_from_path(app: AppHandle, path: String) -> Result<Entry, String> {
    // PowerShell calls are slow (~1s each); keep them off the IPC thread.
    tauri::async_runtime::spawn_blocking(move || create_entry_blocking(&app, path))
        .await
        .map_err(|e| e.to_string())?
}

fn create_entry_blocking(app: &AppHandle, path: String) -> Result<Entry, String> {
    let p = PathBuf::from(&path);
    let resolved_target = if lower_ext(&p).as_deref() == Some("lnk") {
        resolve_lnk(&path).unwrap_or_else(|| path.clone())
    } else {
        path.clone()
    };
    let rp = PathBuf::from(&resolved_target);
    // Prefer the shortcut's own name ("Witcher 3" rather than "witcher3").
    let name_src = if lower_ext(&p).as_deref() == Some("lnk") { &p } else { &rp };
    let name = name_src
        .file_stem()
        .or_else(|| name_src.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("New entry")
        .to_string();
    let kind = if rp.is_dir() {
        "folder"
    } else {
        match lower_ext(&rp).as_deref() {
            Some("exe") => "exe",
            Some("url") => "url",
            _ => "file",
        }
    }
    .to_string();

    let id = Uuid::new_v4().to_string();
    storage::ensure_dirs(app);
    let media = storage::media_dir(app).join(&id);
    let _ = std::fs::create_dir_all(&media);
    let cover_path = media.join("cover.png");
    let have_icon = kind == "exe"
        && extract_icon_ps(&resolved_target, &cover_path.to_string_lossy())
        && cover_path.exists();

    let entry = Entry {
        id,
        name,
        kind,
        target: resolved_target,
        args: String::new(),
        work_dir: String::new(),
        track_process: None,
        cover: have_icon.then(|| cover_path.to_string_lossy().to_string()),
        background: None,
        video: None,
        accent: None,
        category: None,
        tags: Vec::new(),
        stats: crate::models::Stats::default(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    storage::update_library(app, |entries| entries.push(entry.clone()))?;
    Ok(entry)
}

#[tauri::command]
pub fn data_dir_path(app: AppHandle) -> String {
    storage::data_dir(&app).to_string_lossy().to_string()
}

fn hide_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

fn show_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn is_process_running(name: &str) -> bool {
    use sysinfo::{ProcessesToUpdate, System};
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    sys.processes()
        .values()
        .any(|p| p.name().to_string_lossy().eq_ignore_ascii_case(name))
}

/// Waits for `name` to appear (store launchers like Steam can take a while to
/// start the game), then waits for it to exit. Returns false if it never
/// appeared within `start_timeout`.
async fn wait_for_tracked_process(name: &str, start_timeout: Duration) -> bool {
    let deadline = Instant::now() + start_timeout;
    loop {
        let n = name.to_string();
        let running = tokio::task::spawn_blocking(move || is_process_running(&n))
            .await
            .unwrap_or(false);
        if running {
            break;
        }
        if Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let n = name.to_string();
        let running = tokio::task::spawn_blocking(move || is_process_running(&n))
            .await
            .unwrap_or(false);
        if !running {
            return true;
        }
    }
}

/// Splits an argument string, honouring double quotes:
/// `-w 1920 --profile "My Profile"` -> ["-w", "1920", "--profile", "My Profile"].
pub fn split_args(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut has_token = false;
    for c in s.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                has_token = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if has_token {
                    out.push(std::mem::take(&mut cur));
                    has_token = false;
                }
            }
            c => {
                cur.push(c);
                has_token = true;
            }
        }
    }
    if has_token {
        out.push(cur);
    }
    out
}

#[derive(Clone, Serialize)]
pub struct EntryClosed {
    pub id: String,
    /// Seconds of this session, 0 when it could not be measured.
    pub session_sec: u64,
}

/// Records playtime (if measured), restores the launcher and notifies the UI.
fn finish_session(app: &AppHandle, id: &str, session_sec: Option<u64>) {
    if let Some(sec) = session_sec {
        let _ = storage::update_library(app, |all| {
            if let Some(e) = all.iter_mut().find(|e| e.id == id) {
                e.stats.playtime_sec += sec;
                e.stats.last_played = Some(chrono::Utc::now().to_rfc3339());
            }
        });
    }
    show_window(app);
    let _ = app.emit(
        "entry-closed",
        EntryClosed { id: id.to_string(), session_sec: session_sec.unwrap_or(0) },
    );
}

/// For launches that hand off to another program (browser, Steam, Explorer):
/// track the configured process if any, otherwise bring the launcher back
/// after a short delay.
fn spawn_handoff_watch(app: AppHandle, id: String, track: Option<String>, fallback: Duration) {
    let start = Instant::now();
    tauri::async_runtime::spawn(async move {
        match track {
            Some(proc_name) => {
                let seen = wait_for_tracked_process(&proc_name, Duration::from_secs(120)).await;
                let secs = seen.then(|| start.elapsed().as_secs());
                finish_session(&app, &id, secs);
            }
            None => {
                tokio::time::sleep(fallback).await;
                finish_session(&app, &id, None);
            }
        }
    });
}

fn has_scheme(s: &str) -> bool {
    // "steam://...", "https://...", "com.epicgames.launcher://..." — but not "C:\..."
    s.find("://").is_some_and(|i| i > 1)
}

#[tauri::command]
pub async fn launch_entry(app: AppHandle, id: String) -> Result<(), String> {
    let entry = storage::update_library(&app, |all| {
        all.iter_mut().find(|e| e.id == id).map(|e| {
            e.stats.launches += 1;
            e.stats.last_played = Some(chrono::Utc::now().to_rfc3339());
            e.clone()
        })
    })?
    .ok_or_else(|| "entry not found".to_string())?;

    let track = entry.track_process.clone().filter(|s| !s.trim().is_empty());
    let target = entry.target.trim().to_string();
    if target.is_empty() {
        return Err("entry has no target".into());
    }

    match entry.kind.as_str() {
        // Opened through the shell (ShellExecute) rather than `cmd /C start`,
        // which would interpret `&`, `|`, `^` in URLs as command separators.
        "url" | "steam" | "epic" | "file" => {
            let res = if has_scheme(&target) {
                app.opener().open_url(&target, None::<&str>)
            } else {
                app.opener().open_path(&target, None::<&str>)
            };
            res.map_err(|e| e.to_string())?;
            hide_window(&app);
            let delay = if entry.kind == "file" || entry.kind == "url" { 3 } else { 4 };
            spawn_handoff_watch(app, id, track, Duration::from_secs(delay));
        }
        "uwp" => {
            hidden_command("explorer.exe")
                .arg(format!("shell:AppsFolder\\{}", target))
                .spawn()
                .map_err(|e| e.to_string())?;
            hide_window(&app);
            spawn_handoff_watch(app, id, track, Duration::from_secs(4));
        }
        "folder" => {
            Command::new("explorer.exe").arg(&target).spawn().map_err(|e| e.to_string())?;
            // Step aside for Explorer; the user returns via hotkey/tray.
            hide_window(&app);
            let _ = app.emit("entry-closed", EntryClosed { id, session_sec: 0 });
        }
        _ => {
            let mut cmd = Command::new(&target);
            cmd.args(split_args(&entry.args));
            // Many games load assets relative to their own folder, so default
            // the working directory to the executable's folder.
            let work_dir = if entry.work_dir.trim().is_empty() {
                Path::new(&target).parent().map(Path::to_path_buf)
            } else {
                Some(PathBuf::from(entry.work_dir.trim()))
            };
            if let Some(wd) = work_dir.filter(|p| p.is_dir()) {
                cmd.current_dir(wd);
            }
            let mut child = cmd.spawn().map_err(|e| format!("{}: {}", target, e))?;
            hide_window(&app);
            let start = Instant::now();
            tauri::async_runtime::spawn(async move {
                let _ = tokio::task::spawn_blocking(move || child.wait()).await;
                if let Some(proc_name) = track {
                    // Launcher stubs often exit immediately and spawn the real game.
                    wait_for_tracked_process(&proc_name, Duration::from_secs(60)).await;
                }
                finish_session(&app, &id, Some(start.elapsed().as_secs()));
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_args_handles_quotes() {
        assert_eq!(split_args(""), Vec::<String>::new());
        assert_eq!(split_args("  -a   -b "), vec!["-a", "-b"]);
        assert_eq!(
            split_args(r#"-w 1920 --profile "My Profile" -x"#),
            vec!["-w", "1920", "--profile", "My Profile", "-x"]
        );
        assert_eq!(split_args(r#"--name="" -v"#), vec!["--name=", "-v"]);
    }

    #[test]
    fn scheme_detection() {
        assert!(has_scheme("steam://rungameid/570"));
        assert!(has_scheme("https://example.com/?a=1&b=2"));
        assert!(!has_scheme(r"C:\Games\game.exe"));
        assert!(!has_scheme(r"D:\x.url"));
    }
}
