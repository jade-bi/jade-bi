//! 仓库注册表实体

use crate::contexts::shared::domain::vault::{VaultMetadata, VaultType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 仓库条目（用于全局注册表）
///
/// 仅存储仓库的基本注册信息，不包含具体内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
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

impl From<VaultMetadata> for VaultEntry {
    fn from(metadata: VaultMetadata) -> Self {
        VaultEntry {
            id: metadata.id,
            name: metadata.name,
            path: metadata.path,
            vault_type: metadata.vault_type,
            last_accessed_at: metadata.last_accessed_at,
            modified_at: metadata.modified_at,
            has_unsaved_changes: metadata.has_unsaved_changes,
        }
    }
}

impl From<&VaultMetadata> for VaultEntry {
    fn from(metadata: &VaultMetadata) -> Self {
        VaultEntry {
            id: metadata.id.clone(),
            name: metadata.name.clone(),
            path: metadata.path.clone(),
            vault_type: metadata.vault_type.clone(),
            last_accessed_at: metadata.last_accessed_at,
            modified_at: metadata.modified_at,
            has_unsaved_changes: metadata.has_unsaved_changes,
        }
    }
}

impl VaultEntry {
    /// 从完整 Vault 信息创建
    pub fn from_vault(
        id: String,
        name: String,
        path: String,
        vault_type: VaultType,
    ) -> Self {
        let now = Utc::now();
        VaultEntry {
            id,
            name,
            path,
            vault_type,
            last_accessed_at: now,
            modified_at: now,
            has_unsaved_changes: false,
        }
    }
}