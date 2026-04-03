import type {Command, CommandArgs, CommandName, CommandResult} from './command.ts';
import type {EmitName, EmitPayload} from './emit.ts';
import type {S3Config, Vault, VaultMetadata} from './entities/vault';

/**
 * 命令类型并集
 */
export type Commands =
  // 仓库相关命令（与 Rust 端函数名保持一致，使用下划线分隔符）
  | Command<'vault_create', { name: string; path: string; description?: string }, { success: boolean; vault: Vault }>
  | Command<'vault_open', { path: string }, { success: boolean; vault: Vault }>
  | Command<'vault_get_recent', { limit?: number }, { vaults: VaultMetadata[] }>
  | Command<'vault_get_current', void, { vault: Vault | null }>
  | Command<'vault_init_from_s3', { name: string; s3Config: S3Config }, { success: boolean; vault: Vault }>
  // 窗口控制命令
  | Command<'window_minimize', void, { success: boolean }>
  | Command<'window_maximize', void, { success: boolean; isMaximized: boolean }>
  | Command<'window_close', void, { success: boolean }>
  | Command<'window_is_maximized', void, { isMaximized: boolean }>

export type CommandNames = CommandName<Commands>
export type ArgsOf<NAME extends CommandNames> = CommandArgs<Commands, NAME>
export type ResultOf<NAME extends CommandNames> = CommandResult<Commands, NAME>

/**
 * 事件类型
 */
export type Emits = unknown
export type EmitNames = EmitName<Emits>
export type PayloadOf<NAME extends EmitNames> = EmitPayload<Emits, NAME>