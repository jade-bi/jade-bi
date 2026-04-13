//! 仓库注册表命令模块

use crate::contexts::app_state_manager::domain::vault_registry::VaultEntry;
use crate::contexts::app_state_manager::infrastructure::state_storage;
use crate::contexts::shared::domain::vault::VaultType;

/// 获取最近访问的仓库条目
#[tauri::command]
pub fn vault_registry_get_recent(limit: Option<usize>) -> Result<Vec<serde_json::Value>, String> {
    // 确保迁移旧数据
    let _ = state_storage::migrate_from_old_format();

    let entries = state_storage::load_vault_registry()?;

    // 按最后访问时间排序（最新的在前）
    let mut sorted: Vec<_> = entries;
    sorted.sort_by(|a, b| b.last_accessed_at.cmp(&a.last_accessed_at));

    // 限制数量
    let limit = limit.unwrap_or(10);
    let entries: Vec<serde_json::Value> = sorted
        .into_iter()
        .take(limit)
        .map(|e| serde_json::to_value(e).unwrap_or(serde_json::Value::Null))
        .collect();

    Ok(entries)
}

/// 更新仓库访问时间
#[tauri::command]
pub fn vault_registry_update_access_time(vault_id: String) -> Result<bool, String> {
    state_storage::update_vault_access_time(&vault_id)?;
    Ok(true)
}

/// 注册新仓库（仅记录基本注册信息）
#[tauri::command]
pub fn vault_registry_register(
    id: String,
    name: String,
    path: String,
    vault_type: VaultType,
) -> Result<serde_json::Value, String> {
    let entry = VaultEntry::from_vault(id, name, path, vault_type);
    state_storage::upsert_vault_entry(&entry)?;
    Ok(serde_json::to_value(entry).unwrap_or(serde_json::Value::Null))
}

/// 移除仓库注册
#[tauri::command]
pub fn vault_registry_unregister(vault_id: String) -> Result<bool, String> {
    state_storage::remove_vault_entry(&vault_id)?;
    Ok(true)
}

/// 查询仓库注册信息
#[tauri::command]
pub fn vault_registry_get_entry(vault_id: String) -> Result<serde_json::Value, String> {
    let entry = state_storage::get_vault_entry_by_id(&vault_id)?;
    match entry {
        Some(e) => Ok(serde_json::to_value(e).unwrap_or(serde_json::Value::Null)),
        None => Ok(serde_json::Value::Null),
    }
}