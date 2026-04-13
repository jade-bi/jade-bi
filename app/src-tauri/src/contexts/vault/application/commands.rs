//! 仓库命令模块

use crate::contexts::vault::domain::vault::Vault;
use crate::contexts::vault::infrastructure::{
    create_vault_directory, load_vaults, save_vaults, upsert_vault, vault_directory_exists,
};

/// 创建仓库
#[tauri::command]
pub fn vault_create(name: String, path: String, description: Option<String>) -> Result<Vault, String> {
    // 检查目录是否已存在
    if vault_directory_exists(&path) {
        return Err("仓库目录已存在，请选择其他位置".to_string());
    }

    // 创建仓库目录
    create_vault_directory(&path)?;

    // 创建仓库实例
    let vault = Vault::new(name.clone(), path.clone(), description);

    // 保存到存储
    upsert_vault(&vault.metadata)?;

    Ok(vault)
}

/// 打开仓库
#[tauri::command]
pub fn vault_open(path: String) -> Result<Vault, String> {
    // 检查目录是否存在
    if !vault_directory_exists(&path) {
        return Err("仓库目录不存在".to_string());
    }

    // 尝试从存储中查找
    let vaults = load_vaults()?;
    let vault = vaults
        .iter()
        .find(|v| v.path == path)
        .cloned()
        .map(|metadata| Vault {
            metadata,
            created_at: chrono::Utc::now(),
            description: None,
            s3_config: None,
        })
        .unwrap_or_else(|| {
            // 如果不存在，则创建新实例（用于快速打开）
            Vault::new(
                std::path::Path::new(&path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "未知仓库".to_string()),
                path,
                None,
            )
        });

    Ok(vault)
}

/// 获取最近打开的仓库列表
#[tauri::command]
pub fn vault_get_recent(limit: Option<usize>) -> Result<Vec<serde_json::Value>, String> {
    let vaults = load_vaults()?;

    // 按最后访问时间排序（最新的在前）
    let mut sorted: Vec<_> = vaults;
    sorted.sort_by(|a, b| b.last_accessed_at.cmp(&a.last_accessed_at));

    // 限制数量
    let limit = limit.unwrap_or(10);
    let vaults: Vec<serde_json::Value> = sorted
        .into_iter()
        .take(limit)
        .map(|v| serde_json::to_value(v).unwrap_or(serde_json::Value::Null))
        .collect();

    Ok(vaults)
}

/// 获取当前打开的仓库
#[tauri::command]
pub fn vault_get_current() -> Result<serde_json::Value, String> {
    Ok(serde_json::Value::Null)
}

/// 从 S3 实例化仓库
#[tauri::command]
pub fn vault_init_from_s3(name: String, s3_config: serde_json::Value) -> Result<Vault, String> {
    let vault = Vault::new_s3(name, s3_config);

    // 保存到存储
    upsert_vault(&vault.metadata)?;

    Ok(vault)
}

/// 删除仓库（仅删除元数据，不删除实际文件）
#[tauri::command]
pub fn vault_remove(path: String) -> Result<bool, String> {
    let mut vaults = load_vaults()?;

    let original_len = vaults.len();
    vaults.retain(|v| v.path != path);

    if vaults.len() == original_len {
        return Err("仓库不存在".to_string());
    }

    save_vaults(&vaults)?;
    Ok(true)
}

/// 更新仓库的最后访问时间
#[tauri::command]
pub fn vault_update_access_time(path: String) -> Result<bool, String> {
    let mut vaults = load_vaults()?;

    if let Some(vault) = vaults.iter_mut().find(|v| v.path == path) {
        vault.last_accessed_at = chrono::Utc::now();
        save_vaults(&vaults)?;
        Ok(true)
    } else {
        Err("仓库不存在".to_string())
    }
}
