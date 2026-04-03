/**
 * 仓库实体类型定义
 *
 * 定义了 JadeBi 应用中仓库相关的所有类型，包括：
 * - Vault: 完整的仓库信息
 * - VaultMetadata: 仓库元数据（用于列表展示）
 * - S3Config: S3 存储配置
 * - VaultType: 仓库类型枚举
 */

/**
 * 仓库类型枚举
 */
export type VaultType = 'local' | 's3'

/**
 * S3 存储配置
 */
export interface S3Config {
  /** S3 访问端点 */
  endpoint: string
  /** 访问密钥 ID */
  accessKeyId: string
  /** 秘密访问密钥 */
  secretAccessKey: string
  /** 存储桶名称 */
  bucket: string
  /** 区域 */
  region: string
  /** 路径前缀 */
  prefix?: string
}

/**
 * 仓库元数据
 *
 * 用于仓库列表展示，不包含敏感信息
 */
export interface VaultMetadata {
  /** 仓库唯一标识符 */
  id: string
  /** 仓库名称 */
  name: string
  /** 仓库路径（本地路径或 S3 路径） */
  path: string
  /** 仓库类型 */
  type: VaultType
  /** 最后访问时间 */
  lastAccessedAt: string
  /** 最后修改时间 */
  modifiedAt: string
  /** 是否有未保存的更改 */
  hasUnsavedChanges: boolean
}

/**
 * 完整的仓库信息
 *
 * 包含所有仓库数据，可用于仓库详情页和编辑
 */
export interface Vault extends VaultMetadata {
  /** 仓库创建时间 */
  createdAt: string
  /** 仓库描述 */
  description?: string
  /** S3 配置（仅 S3 类型仓库有此字段） */
  s3Config?: S3Config
}
