mod contexts;
mod shared;

use contexts::vault::application::commands::{
    vault_create, vault_get_current, vault_get_recent, vault_init_from_s3, vault_open,
};
use shared::commands::window::{
    window_close, window_is_maximized, window_maximize, window_minimize,
};
use shared::system::tray;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 单实例检测到重复启动时，显示已有窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            // 初始化托盘
            tray::setup_tray(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            // 窗口关闭时最小化到托盘而不是退出
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            vault_create,
            vault_open,
            vault_get_recent,
            vault_get_current,
            vault_init_from_s3,
            window_minimize,
            window_maximize,
            window_close,
            window_is_maximized
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
