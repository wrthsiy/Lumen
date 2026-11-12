mod commands;
mod models;
mod storage;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const DEFAULT_HOTKEY: &str = "Ctrl+Shift+L";

fn summon(app: &AppHandle, fullscreen: bool) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
        if fullscreen {
            let _ = w.set_fullscreen(true);
        }
    }
}

/// (Re)binds the global "bring Lumen back" hotkey, e.g. "Ctrl+Shift+L" or
/// "Alt+F1". On an invalid string the previous binding is restored.
pub fn register_hotkey(app: &AppHandle, hotkey: &str) -> Result<(), String> {
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();
    let bind = |hk: &str| {
        gs.on_shortcut(hk, |app, _sc, event| {
            if event.state == ShortcutState::Pressed {
                let fs = storage::load_settings(app).fullscreen;
                summon(app, fs);
            }
        })
    };
    match bind(hotkey.trim()) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = bind(DEFAULT_HOTKEY);
            Err(format!("invalid hotkey '{hotkey}': {e}"))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // A second launch focuses the running instance instead of opening another.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            summon(app, storage::load_settings(app).fullscreen);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let settings = storage::load_settings(app.handle());
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.set_fullscreen(settings.fullscreen);
            }

            if let Err(e) = register_hotkey(app.handle(), &settings.hotkey) {
                eprintln!("lumen: {e}");
            }

            let show_i = MenuItem::with_id(app, "show", "Show Lumen", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
            if let Some(icon) = app.default_window_icon().cloned() {
                let _ = TrayIconBuilder::new()
                    .icon(icon)
                    .tooltip("Lumen")
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "show" => summon(app, false),
                        "quit" => app.exit(0),
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            summon(tray.app_handle(), false);
                        }
                    })
                    .build(app);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_entries,
            commands::save_entry,
            commands::delete_entry,
            commands::get_settings,
            commands::save_settings,
            commands::list_themes,
            commands::save_theme,
            commands::delete_theme,
            commands::create_entry_from_path,
            commands::data_dir_path,
            commands::launch_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
