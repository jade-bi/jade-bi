//! 仓库查询服务

/// 获取当前打开的仓库
///
/// 注意：当前实现返回 null，因为应用状态管理已移至前端
/// 后续可以通过前端状态管理或 Tauri 状态管理实现
#[tauri::command]
pub fn vault_manager_get_current() -> Result<serde_json::Value, String> {
    // TODO: 实现从 Tauri 状态或前端获取当前仓库
    Ok(serde_json::Value::Null)
}