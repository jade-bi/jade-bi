//! 窗口命令模块

use tauri::Manager;

/// 最小化窗口
#[tauri::command]
pub fn window_minimize(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(serde_json::json!({ "success": true }))
}

/// 最大化/还原窗口
#[tauri::command]
pub fn window_maximize(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
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

/// 关闭窗口
#[tauri::command]
pub fn window_close(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(serde_json::json!({ "success": true }))
}

/// 获取窗口是否最大化
#[tauri::command]
pub fn window_is_maximized(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
        Ok(serde_json::json!({ "isMaximized": is_maximized }))
    } else {
        Ok(serde_json::json!({ "isMaximized": false }))
    }
}