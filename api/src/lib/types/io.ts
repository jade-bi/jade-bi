import type {Command, CommandArgs, CommandName, CommandResult} from './command.ts';
import type {EmitName, EmitPayload} from './emit.ts';
import type {S3Config, Vault, VaultMetadata} from './entities/vault';

/**
 * 仓库相关命令
 */

// 创建仓库命令
export type Command_vault_create = Command<
  'vault:create',
  { name: string; path: string; description?: string },
  { success: boolean; vault: Vault }
>

// 打开仓库命令
export type Command_vault_open = Command<
  'vault:open',
  { path: string },
  { success: boolean; vault: Vault }
>

// 获取最近打开的仓库列表
export type Command_vault_get_recent = Command<
  'vault:get_recent',
  { limit?: number },
  { vaults: VaultMetadata[] }
>

// 获取当前打开的仓库
export type Command_vault_get_current = Command<
  'vault:get_current',
  void,
  { vault: Vault | null }
>

// 从 S3 实例化仓库命令
export type Command_vault_init_from_s3 = Command<
  'vault:init_from_s3',
  { name: string; s3Config: S3Config },
  { success: boolean; vault: Vault }
>

/**
 * 窗口控制命令
 */

// 最小化窗口
export type Command_window_minimize = Command<
  'window:minimize',
  void,
  { success: boolean }
>

// 最大化/还原窗口
export type Command_window_maximize = Command<
  'window:maximize',
  void,
  { success: boolean; isMaximized: boolean }
>

// 关闭窗口
export type Command_window_close = Command<
  'window:close',
  void,
  { success: boolean }
>

// 获取窗口最大化状态
export type Command_window_is_maximized = Command<
  'window:is_maximized',
  void,
  { isMaximized: boolean }
>

/**
 * 命令类型并集
 */
export type Commands =
  | Command_vault_create
  | Command_vault_open
  | Command_vault_get_recent
  | Command_vault_get_current
  | Command_vault_init_from_s3
  | Command_window_minimize
  | Command_window_maximize
  | Command_window_close
  | Command_window_is_maximized

export type CommandNames = CommandName<Commands>
export type ArgsOf<NAME extends CommandNames> = CommandArgs<Commands, NAME>
export type ResultOf<NAME extends CommandNames> = CommandResult<Commands, NAME>

/**
 * 事件类型
 */
export type Emits = unknown
export type EmitNames = EmitName<Emits>
export type PayloadOf<NAME extends EmitNames> = EmitPayload<Emits, NAME>