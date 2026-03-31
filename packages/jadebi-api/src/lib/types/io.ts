import type {CommandArgs, CommandName, CommandResult} from './command.ts';
import type {EmitName, EmitPayload} from './emit.ts';

// 在此定义具体的命令类型
export type Commands = unknown
export type CommandNames = CommandName<Commands>
export type ArgsOf<NAME extends CommandNames> = CommandArgs<Commands, NAME>
export type ResultOf<NAME extends CommandNames> = CommandResult<Commands, NAME>

export type Emits = unknown
export type EmitNames = EmitName<Emits>
export type PayloadOf<NAME extends EmitNames> = EmitPayload<Emits, NAME>
