mod contexts;
mod tray;

use contexts::vault::domain::entities::vault::Vault;
use tauri::Manager;

// ============ 仓库命令 ============

#[tauri::command]
fn vault_create(name: String, path: String, description: Option<String>) -> Result<serde_json::Value, String> {
    // Stub: 创建仓库
    let vault = Vault::new(name.clone(), path.clone(), description);
    Ok(serde_json::json!({
        "success": true,
        "vault": vault
    }))
}

#[tauri::command]
fn vault_open(path: String) -> Result<serde_json::Value, String> {
    // Stub: 打开仓库
    let vault = Vault::new(
        "示例仓库".to_string(),
        path,
        None,
    );
    Ok(serde_json::json!({
        "success": true,
        "vault": vault
    }))
}

#[tauri::command]
fn vault_get_recent(limit: Option<usize>) -> Result<serde_json::Value, String> {
    // Stub: 获取最近打开的仓库列表
    let _limit = limit.unwrap_or(10);
    let vaults: Vec<serde_json::Value> = vec![];
    Ok(serde_json::json!({
        "vaults": vaults
    }))
}

#[tauri::command]
fn vault_get_current() -> Result<serde_json::Value, String> {
    // Stub: 获取当前打开的仓库
    Ok(serde_json::json!({
        "vault": serde_json::Value::Null
    }))
}

#[tauri::command]
fn vault_init_from_s3(name: String, s3_config: serde_json::Value) -> Result<serde_json::Value, String> {
    // Stub: 从 S3 实例化仓库
    let vault = Vault::new_s3(name, s3_config);
    Ok(serde_json::json!({
        "success": true,
        "vault": vault
    }))
}

// ============ 窗口控制命令 ============

#[tauri::command]
fn window_minimize(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
fn window_maximize(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
        if is_maximized {
            window.unmaximize().map_err(|e| e.to_string())?;
        } else {
            window.maximize().map_err(|e| e.to_string())?;
        }
        Ok(serde_json::json!({ "success": true, "isMaximized": !is_maximized }))
    } else {
        Ok(serde_json::json!({ "success": false }))
    }
}

#[tauri::command]
fn window_close(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
fn window_is_maximized(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "isMaximized": is_maximized }))
    } else {
        Ok(serde_json::json!({ "isMaximized": false }))
    }
}

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
