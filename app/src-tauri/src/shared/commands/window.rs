//! 窗口命令模块

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

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

/// 在新窗口中打开仓库
#[tauri::command]
pub fn window_open_vault(
    app: tauri::AppHandle,
    vault_path: String,
    vault_name: String,
) -> Result<serde_json::Value, String> {
    // 使用 vault_path 生成唯一窗口标签
    let window_label = format!("vault-{}", vault_path.replace(['/', '\\', ':'], "-"));

    // 如果窗口已存在，则聚焦并返回
    if let Some(window) = app.get_webview_window(&window_label) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(serde_json::json!({ "success": true, "existing": true }));
    }

    // 创建新窗口
    let url = format!("/vault?path={}", urlencoding::encode(&vault_path));

    WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::App(url.into()),
    )
    .title(format!("玉璧 - {}", vault_name))
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .center()
    .build()
    .map_err(|e| format!("创建窗口失败: {}", e))?;

    Ok(serde_json::json!({ "success": true, "existing": false }))
}
