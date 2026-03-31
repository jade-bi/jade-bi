# @jadebi/api

JadeBi 桌面应用的类型安全 Tauri 命令与事件系统。

## 概述

`@jadebi/api` 为 [JadeBi](https://github.com/fengyifan/jadebi) 桌面应用提供类型安全的 Tauri 命令和事件通信层。它采用高级 TypeScript 类型系统，确保前端与 Rust 后端之间的 IPC 通信具有完整的类型安全性，减少运行时错误并提升开发体验。

## 特性

- **类型安全的命令调用**: 使用 TypeScript 泛型确保命令名称、参数和返回值的类型安全
- **响应式状态管理**: 基于 Svelte 5 runes 系统构建的响应式调用器状态
- **事件类型系统**: 类型安全的事件定义和载荷类型推断
- **插件友好设计**: 通过 TypeScript 模块扩展支持插件系统
- **Tauri 原生集成**: 直接集成 Tauri IPC 系统，无额外依赖

## 安装

```bash
# 使用 pnpm（推荐）
pnpm add @jadebi/api

# 使用 npm
npm install @jadebi/api

# 使用 yarn
yarn add @jadebi/api
```

**前提条件**: 确保项目已安装 [Tauri](https://tauri.app/) 和 [Svelte 5](https://svelte.dev/)（或支持 Svelte 5 runes 的环境）。

## 快速开始

### 1. 定义命令类型

首先，在你的应用中定义具体的命令类型：

```typescript
// src/types/commands.ts
import type { Command } from '@jadebi/api';

// 定义应用特定的命令
export type AppCommands =
  | Command<'greet', { name: string }, string>
  | Command<'getConfig', void, { theme: string, language: string }>
  | Command<'saveNote', { id: string; content: string }, boolean>;

export type AppEmits =
  | { name: 'configUpdated'; payload: { theme: string } }
  | { name: 'noteSaved'; payload: { id: string } };
```

### 2. 扩展类型系统

扩展 `@jadebi/api` 的类型系统以包含你的命令：

```typescript
// src/types/api.d.ts
import type { AppCommands, AppEmits } from './commands';

declare module '@jadebi/api' {
  export type Commands = AppCommands;
  export type Emits = AppEmits;
}
```

### 3. 使用调用器

在 Svelte 组件中使用类型安全的命令调用器：

```svelte
<script lang="ts">
  import { invoker } from '@jadebi/api';

  // 创建类型安全的调用器实例
  const greetInvoker = invoker('greet');

  async function handleGreet() {
    try {
      // 类型安全的调用 - 参数和返回值类型自动推断
      const message = await greetInvoker.call({ name: 'World' });
      console.log(message); // 类型: string
    } catch (error) {
      console.error('调用失败:', error);
    }
  }
</script>

<button on:click={handleGreet} disabled={greetInvoker.isLoading()}>
  {#if greetInvoker.isLoading()}
    加载中...
  {:else if greetInvoker.isSuccess()}
    已问候: {greetInvoker.data}
  {:else if greetInvoker.isError()}
    错误: {greetInvoker.error}
  {:else}
    问候
  {/if}
</button>
```

## API 参考

### 核心类型

#### `Command<NAME, ARGS, RESULT>`

表示一个 Tauri 命令的类型定义。

```typescript
type Command<
  NAME extends string,
  ARGS extends Record<string, unknown> | void = void,
  RESULT = null
> = {
  name: NAME;
  args: ARGS;
  result: RESULT;
};
```

#### `Emit<NAME, PAYLOAD>`

表示一个事件发射的类型定义。

```typescript
type Emit<
  NAME extends string,
  PAYLOAD = unknown
> = {
  name: NAME;
  payload: PAYLOAD;
};
```

### 类型工具

#### `CommandName<CMD>`
提取命令类型中的名称。

#### `CommandArgs<CMD, NAME>`
提取指定命令名称的参数类型。

#### `CommandResult<CMD, NAME>`
提取指定命令名称的返回值类型。

#### `EmitName<EMIT>`
提取事件类型中的名称。

#### `EmitPayload<EMIT, NAME>`
提取指定事件名称的载荷类型。

### 调用器系统

#### `Invoker<NAME>`

类型安全的 Tauri 命令调用器接口。

```typescript
interface Invoker<NAME extends CommandNames> {
  readonly state: InvokerState;  // 'idle' | 'loading' | 'success' | 'error'
  readonly data: ResultOf<NAME> | undefined;
  readonly error: string | undefined;

  // 条件调用签名
  call: ArgsOf<NAME> extends void
    ? () => Promise<ResultOf<NAME>>
    : (args: ArgsOf<NAME>) => Promise<ResultOf<NAME>>;

  isSuccess(): this is Invoker<NAME, 'success', ArgsOf<NAME>, Exclude<ResultOf<NAME>, undefined>, undefined>;
  isLoading(): this is Invoker<NAME, 'loading', ArgsOf<NAME>, undefined, undefined>;
  isError(): this is Invoker<NAME, 'error', ArgsOf<NAME>, undefined, string>;
}
```

#### `invoker(commandName: NAME): Invoker<NAME>`

创建类型安全调用器实例的工厂函数。

```typescript
import { invoker } from '@jadebi/api';

// 创建调用器实例
const getUserInvoker = invoker('getUser');

// 类型安全的调用
const user = await getUserInvoker.call({ id: '123' }); // user: User 类型
```

## 插件开发

### 扩展命令系统

插件可以通过声明合并扩展命令系统：

```typescript
// 插件类型定义
declare module '@jadebi/api' {
  export interface Commands {
    'plugin.awesomeFeature': Command<'plugin.awesomeFeature', { data: string }, { success: boolean }>;
  }

  export interface Emits {
    'plugin.notification': { name: 'plugin.notification'; payload: { message: string } };
  }
}
```

### 在插件中使用

```typescript
// 插件实现
import { invoker } from '@jadebi/api';

export function useAwesomeFeature() {
  const invoker = invoker('plugin.awesomeFeature');

  return {
    invoke: invoker.call,
    state: invoker.state,
    data: invoker.data,
    error: invoker.error
  };
}
```

## 与 Tauri 后端集成

### Rust 后端命令定义

确保 Rust 后端的命令签名与 TypeScript 类型匹配：

```rust
// src/commands.rs
#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_config() -> Config {
    Config {
        theme: "dark".to_string(),
        language: "en".to_string(),
    }
}
```

### 事件发射

从 Rust 后端发射事件：

```rust
use tauri::Emitter;

app.emit("configUpdated", ConfigUpdatedEvent { theme: "light".to_string() }).unwrap();
```

## 高级用法

### 批量命令调用

```typescript
import { invoker } from '@jadebi/api';

async function initializeApp() {
  const configInvoker = invoker('getConfig');
  const userInvoker = invoker('getUser');

  // 并行调用多个命令
  const [config, user] = await Promise.all([
    configInvoker.call(),
    userInvoker.call({ id: 'current' })
  ]);

  return { config, user };
}
```

### 自定义错误处理

```typescript
const invoker = invoker('dangerousOperation');

try {
  const result = await invoker.call();
} catch (error) {
  // 类型安全的错误处理
  if (invoker.isError()) {
    console.error('调用失败:', invoker.error);
  }

  // 根据错误类型进行特定处理
  if (error instanceof NetworkError) {
    // 网络错误处理
  } else if (error instanceof ValidationError) {
    // 验证错误处理
  }
}
```

### 响应式状态绑定

```svelte
<script lang="ts">
  import { invoker } from '@jadebi/api';

  const dataInvoker = invoker('fetchData');

  // 自动响应状态变化
  $effect(() => {
    console.log('状态变化:', dataInvoker.state);

    if (dataInvoker.isSuccess()) {
      // 类型细化：data 不再是 undefined
      processData(dataInvoker.data);
    }
  });
</script>
```

## 开发指南

### 项目结构

```
jadebi-api/
├── src/
│   ├── lib/
│   │   ├── types/
│   │   │   ├── command.ts      # 命令类型定义
│   │   │   ├── emit.ts        # 事件类型定义
│   │   │   ├── io.ts          # 类型系统入口
│   │   │   └── index.ts       # 类型导出
│   │   └── invoker.svelte.ts  # 调用器实现
│   └── app.d.ts              # Svelte 类型定义
├── package.json
├── tsconfig.json
└── README.md
```

### 构建

```bash
# 开发模式
pnpm dev

# 生产构建
pnpm build

# 类型检查
pnpm check
```

### 测试

```bash
# 运行测试
pnpm test

# 开发模式运行测试
pnpm test:watch
```

## 常见问题

### 类型扩展不生效？

确保类型声明文件被 TypeScript 包含。检查 `tsconfig.json` 中的 `include` 或 `files` 配置。

### 调用器状态不更新？

调用器使用 Svelte 5 runes 系统。确保在支持 runes 的 Svelte 版本中使用。

### 命令名称类型错误？

确认命令名称在 `Commands` 类型中正确定义，并且类型声明文件已正确扩展。

### 与 Tauri 版本兼容性

`@jadebi/api` 设计为与 Tauri 2.x 版本兼容。如果使用 Tauri 1.x，可能需要调整 IPC 调用方式。

## 贡献

欢迎贡献！请参阅 [JadeBi 项目贡献指南](https://github.com/fengyifan/jadebi/blob/main/CONTRIBUTING.md)。

1.  Fork 项目
2.  创建特性分支 (`git checkout -b feature/amazing-feature`)
3.  提交更改 (`git commit -m 'feat(api): 添加 amazing feature'`)
4.  推送到分支 (`git push origin feature/amazing-feature`)
5.  开启 Pull Request

## 许可证

MIT © [Feng Yifan](https://github.com/fengyifan)

## 相关项目

- [JadeBi 桌面应用](https://github.com/fengyifan/jadebi) - 主桌面应用
- [@jadebi/tools](https://github.com/fengyifan/jadebi/tree/main/packages/tools) - 共享工具库
- [Tauri](https://tauri.app/) - 构建小型、快速、安全的桌面应用