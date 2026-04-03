//! 仓库命令模块

use crate::contexts::vault::domain::vault::Vault;

/// 创建仓库
#[tauri::command]
pub fn vault_create(name: String, path: String, description: Option<String>) -> Result<serde_json::Value, String> {
    let vault = Vault::new(name.clone(), path.clone(), description);
    Ok(serde_json::json!({
        "success": true,
        "vault": vault
    }))
}

/// 打开仓库
#[tauri::command]
pub fn vault_open(path: String) -> Result<serde_json::Value, String> {
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

/// 获取最近打开的仓库列表
#[tauri::command]
pub fn vault_get_recent(limit: Option<usize>) -> Result<serde_json::Value, String> {
    let _limit = limit.unwrap_or(10);
    let vaults: Vec<serde_json::Value> = vec![];
    Ok(serde_json::json!({
        "vaults": vaults
    }))
}

/// 获取当前打开的仓库
#[tauri::command]
pub fn vault_get_current() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "vault": serde_json::Value::Null
    }))
}

/// 从 S3 实例化仓库
#[tauri::command]
pub fn vault_init_from_s3(name: String, s3_config: serde_json::Value) -> Result<serde_json::Value, String> {
    let vault = Vault::new_s3(name, s3_config);
    Ok(serde_json::json!({
        "success": true,
        "vault": vault
    }))
}