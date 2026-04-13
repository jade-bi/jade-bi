//! 仓库仓储

use crate::contexts::shared::domain::vault::Vault;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

/// 仓库仓储
///
/// 负责存储和管理完整的 Vault 信息
pub struct VaultRepository {
    /// 内存缓存
    cache: Mutex<HashMap<String, Vault>>,
}

impl VaultRepository {
    pub fn new() -> Self {
        VaultRepository {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// 保存仓库
    pub fn save(&self, vault: &Vault) -> Result<(), String> {
        // 保存到文件
        let path = self.get_vault_path(&vault.metadata.id)?;
        let metadata_dir = path.join(".jade-bi");

        if !metadata_dir.exists() {
            std::fs::create_dir_all(&metadata_dir)
                .map_err(|e| format!("创建元数据目录失败: {}", e))?;
        }

        let metadata_file = metadata_dir.join("metadata.json");
        let content = serde_json::to_string_pretty(vault)
            .map_err(|e| format!("序列化仓库元数据失败: {}", e))?;

        std::fs::write(&metadata_file, content)
            .map_err(|e| format!("写入仓库元数据失败: {}", e))?;

        // 更新缓存
        let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
        cache.insert(vault.metadata.id.clone(), vault.clone());

        Ok(())
    }

    /// 根据 ID 获取仓库
    pub fn get_by_id(&self, vault_id: &str) -> Result<Option<Vault>, String> {
        // 先从缓存获取
        {
            let cache = self.cache.lock().map_err(|e| e.to_string())?;
            if let Some(vault) = cache.get(vault_id) {
                return Ok(Some(vault.clone()));
            }
        }

        // 从文件加载
        let path = self.get_vault_path(vault_id)?;
        let metadata_file = path.join(".jade-bi").join("metadata.json");

        if !metadata_file.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&metadata_file)
            .map_err(|e| format!("读取仓库元数据失败: {}", e))?;

        let vault: Vault = serde_json::from_str(&content)
            .map_err(|e| format!("解析仓库元数据失败: {}", e))?;

        // 更新缓存
        {
            let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
            cache.insert(vault_id.to_string(), vault.clone());
        }

        Ok(Some(vault))
    }

    /// 根据路径获取仓库
    pub fn get_by_path(&self, path: &str) -> Result<Option<Vault>, String> {
        // 遍历缓存查找
        let cache = self.cache.lock().map_err(|e| e.to_string())?;
        for vault in cache.values() {
            if vault.metadata.path == path {
                return Ok(Some(vault.clone()));
            }
        }
        drop(cache);

        // 从文件加载（需要遍历查找，比较慢）
        // 这里简化处理，返回 None
        // 实际应该从索引文件或数据库查找
        Ok(None)
    }

    /// 删除仓库
    pub fn delete(&self, vault_id: &str) -> Result<(), String> {
        // 从缓存移除
        {
            let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
            cache.remove(vault_id);
        }

        // 注意：不删除实际的文件和目录，只删除元数据
        Ok(())
    }

    /// 获取仓库文件路径
    fn get_vault_path(&self, vault_id: &str) -> Result<PathBuf, String> {
        let data_dir = dirs::data_dir()
            .ok_or("无法获取应用数据目录")?
            .join("jade-bi")
            .join("vaults")
            .join(vault_id);

        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("创建仓库目录失败: {}", e))?;
        }

        Ok(data_dir)
    }
}

impl Default for VaultRepository {
    fn default() -> Self {
        Self::new()
    }
}