//! 仓库创建器

use crate::contexts::shared::domain::vault::{S3Config, Vault, VaultType};
use std::path::Path;

/// 创建本地仓库参数
pub struct CreateLocalVaultParams {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
}

/// 创建 S3 仓库参数
pub struct CreateS3VaultParams {
    pub name: String,
    pub s3_config: S3Config,
    pub description: Option<String>,
}

/// 仓库创建器 trait
pub trait VaultCreator {
    /// 创建仓库
    fn create(&self, params: CreateLocalVaultParams) -> Result<Vault, String>;
}

/// 本地仓库创建器
pub struct LocalVaultCreator;

impl VaultCreator for LocalVaultCreator {
    fn create(&self, params: CreateLocalVaultParams) -> Result<Vault, String> {
        let path = Path::new(&params.path);

        // 验证路径存在且为空
        if !path.exists() {
            return Err("目录不存在".to_string());
        }

        if !path.is_dir() {
            return Err("选择的不是目录".to_string());
        }

        // 检查目录是否为空（允许空目录作为新仓库）
        let is_empty = std::fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?
            .next()
            .is_none();

        if !is_empty {
            return Err("目录非空，请选择一个空目录作为新仓库".to_string());
        }

        Ok(Vault::new(params.name, params.path, params.description))
    }
}

/// S3 仓库创建器
pub struct S3VaultCreator;

impl VaultCreator for S3VaultCreator {
    fn create(&self, params: CreateLocalVaultParams) -> Result<Vault, String> {
        // S3 仓库创建需要不同的参数结构
        Err("请使用 create_s3 方法创建 S3 仓库".to_string())
    }
}

impl S3VaultCreator {
    /// 创建 S3 仓库
    pub fn create_s3(&self, params: CreateS3VaultParams) -> Result<Vault, String> {
        let s3_config_json = serde_json::to_value(&params.s3_config)
            .map_err(|e| format!("序列化 S3 配置失败: {}", e))?;
        Ok(Vault::new_s3(params.name, s3_config_json))
    }
}