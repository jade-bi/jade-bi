//! 仓库存储基础设施

use crate::contexts::vault::domain::vault::VaultMetadata;
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

/// 获取仓库元数据文件路径
fn get_vaults_file() -> Result<PathBuf, String> {
    Ok(get_data_dir()?.join("vaults.json"))
}

/// 加载所有仓库元数据
pub fn load_vaults() -> Result<Vec<VaultMetadata>, String> {
    let file_path = get_vaults_file()?;

    if !file_path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取仓库列表失败: {}", e))?;

    let vaults: Vec<VaultMetadata> = serde_json::from_str(&content)
        .map_err(|e| format!("解析仓库列表失败: {}", e))?;

    Ok(vaults)
}

/// 保存仓库元数据列表
pub fn save_vaults(vaults: &[VaultMetadata]) -> Result<(), String> {
    let file_path = get_vaults_file()?;

    let content = serde_json::to_string_pretty(vaults)
        .map_err(|e| format!("序列化仓库列表失败: {}", e))?;

    fs::write(&file_path, content)
        .map_err(|e| format!("写入仓库列表失败: {}", e))?;

    Ok(())
}

/// 添加或更新仓库
pub fn upsert_vault(vault: &VaultMetadata) -> Result<(), String> {
    let mut vaults = load_vaults()?;

    // 查找并更新或添加
    if let Some(existing) = vaults.iter_mut().find(|v| v.id == vault.id) {
        *existing = vault.clone();
    } else {
        vaults.push(vault.clone());
    }

    save_vaults(&vaults)
}

/// 创建仓库目录（或使用已有空目录）
pub fn create_vault_directory(path: &str) -> Result<(), String> {
    let path = PathBuf::from(path);

    // 目录必须存在
    if !path.exists() {
        return Err("目录不存在".to_string());
    }

    // 检查是否为目录
    if !path.is_dir() {
        return Err("选择的不是目录".to_string());
    }

    // 检查目录是否为空（允许空目录作为新仓库）
    let is_empty = std::fs::read_dir(&path)
        .map_err(|e| format!("读取目录失败: {}", e))?
        .next()
        .is_none();

    if !is_empty {
        return Err("目录非空，请选择一个空目录作为新仓库".to_string());
    }

    Ok(())
}

/// 检查仓库目录是否存在
pub fn vault_directory_exists(path: &str) -> bool {
    PathBuf::from(path).exists() && PathBuf::from(path).is_dir()
}