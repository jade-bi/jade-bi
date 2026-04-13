//! 仓库操作命令模块

use crate::contexts::app_state_manager::domain::vault_registry::VaultEntry;
use crate::contexts::app_state_manager::infrastructure::state_storage as app_state_storage;
use crate::contexts::shared::domain::vault::{S3Config, Vault, VaultType};
use crate::contexts::vault_manager::domain::validator::VaultValidator;
use std::path::Path;

/// 创建新本地仓库
#[tauri::command]
pub fn vault_manager_create_local(
    name: String,
    path: String,
    description: Option<String>,
) -> Result<Vault, String> {
    // 验证路径
    VaultValidator::validate_for_creation(&path)?;

    // 创建仓库实例
    let vault = Vault::new(name.clone(), path.clone(), description);

    // 保存到 VaultRepository
    // 注意：这里简化处理，直接保存元数据
    // 后续可以通过 VaultRepository 保存完整信息

    // 注册到 AppStateManager
    let entry = VaultEntry::from_vault(
        vault.metadata.id.clone(),
        name,
        path,
        VaultType::Local,
    );
    app_state_storage::upsert_vault_entry(&entry)?;

    Ok(vault)
}

/// 创建新 S3 仓库
#[tauri::command]
pub fn vault_manager_create_s3(
    name: String,
    s3_config: serde_json::Value,
    description: Option<String>,
) -> Result<Vault, String> {
    let config: S3Config = serde_json::from_value(s3_config)
        .map_err(|e| format!("解析 S3 配置失败: {}", e))?;

    // 创建仓库实例
    let vault = Vault::new_s3(name.clone(), serde_json::to_value(&config).unwrap());

    // 注册到 AppStateManager
    let entry = VaultEntry::from_vault(
        vault.metadata.id.clone(),
        name,
        vault.metadata.path.clone(),
        VaultType::S3,
    );
    app_state_storage::upsert_vault_entry(&entry)?;

    Ok(vault)
}

/// 打开现有仓库（职责单一，只负责打开，不自动创建）
#[tauri::command]
pub fn vault_manager_open(path: String) -> Result<Vault, String> {
    // 验证路径
    VaultValidator::validate_for_open(&path)?;

    // 尝试从 AppStateManager 注册表中查找
    let entries = app_state_storage::load_vault_registry()?;
    let entry = entries
        .iter()
        .find(|e| e.path == path)
        .cloned();

    match entry {
        Some(entry) => {
            // 找到了注册信息，构建完整 Vault
            let vault = Vault {
                metadata: crate::contexts::shared::domain::vault::VaultMetadata {
                    id: entry.id,
                    name: entry.name,
                    path: entry.path,
                    vault_type: entry.vault_type,
                    last_accessed_at: entry.last_accessed_at,
                    modified_at: entry.modified_at,
                    has_unsaved_changes: false,
                },
                created_at: entry.modified_at,
                description: None,
                s3_config: None,
            };

            // 更新访问时间
            let _ = app_state_storage::update_vault_access_time(&vault.metadata.id);

            Ok(vault)
        }
        None => {
            // 没有注册信息，检查目录是否为有效的仓库
            // 如果是，创建新的 Vault 实例（用于快速打开未注册的仓库）
            let path_buf = Path::new(&path);
            let name = path_buf
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "未知仓库".to_string());

            Ok(Vault::new(name, path, None))
        }
    }
}

/// 验证仓库路径
#[tauri::command]
pub fn vault_manager_validate_path(path: String) -> Result<bool, String> {
    VaultValidator::validate_path(&path)?;
    Ok(true)
}

/// 根据 ID 获取完整仓库信息（供防腐层调用）
#[tauri::command]
pub fn vault_manager_get_by_id(vault_id: String) -> Result<serde_json::Value, String> {
    // 从 AppStateManager 获取条目
    let entry = app_state_storage::get_vault_entry_by_id(&vault_id)?;

    match entry {
        Some(entry) => {
            let vault = Vault {
                metadata: crate::contexts::shared::domain::vault::VaultMetadata {
                    id: entry.id,
                    name: entry.name,
                    path: entry.path,
                    vault_type: entry.vault_type,
                    last_accessed_at: entry.last_accessed_at,
                    modified_at: entry.modified_at,
                    has_unsaved_changes: false,
                },
                created_at: entry.modified_at,
                description: None,
                s3_config: None,
            };

            Ok(serde_json::to_value(vault).unwrap_or(serde_json::Value::Null))
        }
        None => Ok(serde_json::Value::Null),
    }
}

/// 删除仓库（仅删除注册信息，不删除实际文件）
#[tauri::command]
pub fn vault_manager_remove(vault_id: String) -> Result<bool, String> {
    app_state_storage::remove_vault_entry(&vault_id)?;
    Ok(true)
}