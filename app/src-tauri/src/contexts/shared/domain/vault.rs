//! 仓库实体（共享领域）

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 仓库类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VaultType {
    Local,
    S3,
}

/// S3 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// 仓库元数据（用于全局注册表）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub vault_type: VaultType,
    #[serde(rename = "lastAccessedAt")]
    pub last_accessed_at: DateTime<Utc>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: DateTime<Utc>,
    #[serde(rename = "hasUnsavedChanges")]
    pub has_unsaved_changes: bool,
}

/// 完整的仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    #[serde(flatten)]
    pub metadata: VaultMetadata,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
}

impl Vault {
    /// 创建新的本地仓库
    pub fn new(name: String, path: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Vault {
            metadata: VaultMetadata {
                id: Uuid::new_v4().to_string(),
                name,
                path,
                vault_type: VaultType::Local,
                last_accessed_at: now,
                modified_at: now,
                has_unsaved_changes: false,
            },
            created_at: now,
            description,
            s3_config: None,
        }
    }

    /// 创建新的 S3 仓库
    pub fn new_s3(name: String, s3_config: serde_json::Value) -> Self {
        let now = Utc::now();
        let config: S3Config = serde_json::from_value(s3_config).unwrap_or(S3Config {
            endpoint: "".to_string(),
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            bucket: "".to_string(),
            region: "us-east-1".to_string(),
            prefix: None,
        });
        let path = format!("s3://{}/{}", config.bucket, config.prefix.as_deref().unwrap_or(""));
        Vault {
            metadata: VaultMetadata {
                id: Uuid::new_v4().to_string(),
                name,
                path,
                vault_type: VaultType::S3,
                last_accessed_at: now,
                modified_at: now,
                has_unsaved_changes: false,
            },
            created_at: now,
            description: None,
            s3_config: Some(config),
        }
    }
}