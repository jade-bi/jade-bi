//! 仓库注册表存储基础设施

use crate::contexts::app_state_manager::domain::vault_registry::VaultEntry;
use crate::contexts::shared::domain::vault::VaultMetadata;
use std::fs;
use std::path::PathBuf;

/// 获取应用数据目录
fn get_data_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir()
        .ok_or("无法获取应用数据目录")?
        .join("jade-bi");

    // 确保目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
    }

    Ok(data_dir)
}

/// 获取仓库注册表文件路径
fn get_vault_registry_file() -> Result<PathBuf, String> {
    Ok(get_data_dir()?.join("vault_registry.json"))
}

// ==================== 仓库注册表操作 ====================

/// 加载仓库注册表
pub fn load_vault_registry() -> Result<Vec<VaultEntry>, String> {
    let file_path = get_vault_registry_file()?;

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取仓库注册表失败: {}", e))?;

    let entries: Vec<VaultEntry> = serde_json::from_str(&content)
        .map_err(|e| format!("解析仓库注册表失败: {}", e))?;

    Ok(entries)
}

/// 保存仓库注册表
pub fn save_vault_registry(entries: &[VaultEntry]) -> Result<(), String> {
    let file_path = get_vault_registry_file()?;

    let content = serde_json::to_string_pretty(entries)
        .map_err(|e| format!("序列化仓库注册表失败: {}", e))?;

    fs::write(&file_path, content)
        .map_err(|e| format!("写入仓库注册表失败: {}", e))?;

    Ok(())
}

/// 根据 ID 获取仓库条目
pub fn get_vault_entry_by_id(vault_id: &str) -> Result<Option<VaultEntry>, String> {
    let entries = load_vault_registry()?;
    Ok(entries.into_iter().find(|e| e.id == vault_id))
}

/// 添加或更新仓库条目
pub fn upsert_vault_entry(entry: &VaultEntry) -> Result<(), String> {
    let mut entries = load_vault_registry()?;

    if let Some(existing) = entries.iter_mut().find(|e| e.id == entry.id) {
        *existing = entry.clone();
    } else {
        entries.push(entry.clone());
    }

    save_vault_registry(&entries)
}

/// 移除仓库条目
pub fn remove_vault_entry(vault_id: &str) -> Result<(), String> {
    let mut entries = load_vault_registry()?;
    entries.retain(|e| e.id != vault_id);
    save_vault_registry(&entries)
}

/// 更新仓库访问时间
pub fn update_vault_access_time(vault_id: &str) -> Result<(), String> {
    let mut entries = load_vault_registry()?;

    if let Some(entry) = entries.iter_mut().find(|e| e.id == vault_id) {
        entry.last_accessed_at = chrono::Utc::now();
        save_vault_registry(&entries)
    } else {
        Err("仓库不存在".to_string())
    }
}

// ==================== 兼容层（迁移旧数据） ====================

/// 从旧的 vaults.json 迁移数据到新的 vault_registry.json
pub fn migrate_from_old_format() -> Result<(), String> {
    let old_file = get_data_dir()?.join("vaults.json");

    if !old_file.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&old_file)
        .map_err(|e| format!("读取旧仓库列表失败: {}", e))?;

    let old_vaults: Vec<VaultMetadata> = serde_json::from_str(&content)
        .map_err(|e| format!("解析旧仓库列表失败: {}", e))?;

    let entries: Vec<VaultEntry> = old_vaults
        .into_iter()
        .map(VaultEntry::from)
        .collect();

    save_vault_registry(&entries)?;

    // 重命名旧文件以备份
    let backup_file = get_data_dir()?.join("vaults.json.bak");
    fs::rename(&old_file, &backup_file)
        .map_err(|e| format!("备份旧文件失败: {}", e))?;

    Ok(())
}