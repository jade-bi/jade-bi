mod contexts;
mod shared;

use contexts::app_state_manager::application::vault_registry_commands::{
    vault_registry_get_entry, vault_registry_get_recent, vault_registry_register,
    vault_registry_unregister, vault_registry_update_access_time,
};
use contexts::vault_manager::application::vault_commands::{
    vault_manager_create_local, vault_manager_create_s3, vault_manager_get_by_id,
    vault_manager_open, vault_manager_remove, vault_manager_validate_path,
};
use contexts::vault_manager::application::vault_queries::vault_manager_get_current;
use shared::commands::window::{
    window_close, window_is_maximized, window_maximize, window_minimize, window_open_vault,
};
use shared::system::tray;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            // VaultRegistry (AppStateManager)
            vault_registry_get_recent,
            vault_registry_update_access_time,
            vault_registry_register,
            vault_registry_unregister,
            vault_registry_get_entry,
            // VaultManager
            vault_manager_create_local,
            vault_manager_create_s3,
            vault_manager_open,
            vault_manager_validate_path,
            vault_manager_get_by_id,
            vault_manager_get_current,
            vault_manager_remove,
            // Window
            window_minimize,
            window_maximize,
            window_close,
            window_is_maximized,
            window_open_vault
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
