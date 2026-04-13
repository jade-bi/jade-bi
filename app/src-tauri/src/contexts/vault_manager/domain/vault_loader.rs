//! 仓库加载器

use crate::contexts::shared::domain::vault::Vault;
use std::path::Path;

/// 仓库加载器
pub struct VaultLoader;

impl VaultLoader {
    /// 打开现有仓库
    pub fn open(path: &str) -> Result<Vault, String> {
        let path_buf = Path::new(path);

        // 检查目录是否存在
        if !path_buf.exists() {
            return Err("仓库目录不存在".to_string());
        }

        if !path_buf.is_dir() {
            return Err("仓库路径不是目录".to_string());
        }

        // 尝试从存储中加载完整 Vault 信息
        // 注意：这里我们需要从 VaultRepository 获取完整信息
        // 为了简单起见，我们先检查 .jade-bi 目录中的 metadata.json
        let metadata_path = path_buf.join(".jade-bi").join("metadata.json");

        if metadata_path.exists() {
            let content = std::fs::read_to_string(&metadata_path)
                .map_err(|e| format!("读取仓库元数据失败: {}", e))?;
            let vault: Vault = serde_json::from_str(&content)
                .map_err(|e| format!("解析仓库元数据失败: {}", e))?;
            Ok(vault)
        } else {
            // 如果没有元数据文件，创建一个基本的 Vault 实例
            // 这用于快速打开尚未完全初始化的仓库
            let name = path_buf
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "未知仓库".to_string());

            Ok(Vault::new(name, path.to_string(), None))
        }
    }

    /// 从 ID 加载仓库（供防腐层调用）
    pub fn load_by_id(vault_id: &str) -> Result<Option<Vault>, String> {
        // 这里我们需要从某个存储中查找
        // 暂时从 AppStateManager 的注册表中获取路径，然后加载
        // 这是一个临时实现，后续可以改进
        Ok(None)
    }
}