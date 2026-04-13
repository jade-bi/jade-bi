//! 仓库验证器

use std::path::Path;

/// 仓库验证器
pub struct VaultValidator;

impl VaultValidator {
    /// 验证仓库路径
    pub fn validate_path(path: &str) -> Result<(), String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err("目录不存在".to_string());
        }

        if !path.is_dir() {
            return Err("选择的不是目录".to_string());
        }

        Ok(())
    }

    /// 验证仓库目录是否为空
    pub fn validate_empty(path: &str) -> Result<bool, String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err("目录不存在".to_string());
        }

        let is_empty = std::fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?
            .next()
            .is_none();

        Ok(is_empty)
    }

    /// 验证仓库目录可用于创建新仓库
    pub fn validate_for_creation(path: &str) -> Result<(), String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err("目录不存在".to_string());
        }

        if !path.is_dir() {
            return Err("选择的不是目录".to_string());
        }

        let is_empty = std::fs::read_dir(path)
            .map_err(|e| format!("读取目录失败: {}", e))?
            .next()
            .is_none();

        if !is_empty {
            return Err("目录非空，请选择一个空目录作为新仓库".to_string());
        }

        Ok(())
    }

    /// 验证仓库目录可用于打开现有仓库
    pub fn validate_for_open(path: &str) -> Result<(), String> {
        let path = Path::new(path);

        if !path.exists() {
            return Err("仓库目录不存在".to_string());
        }

        if !path.is_dir() {
            return Err("仓库路径不是目录".to_string());
        }

        Ok(())
    }
}