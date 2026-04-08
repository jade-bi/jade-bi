# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目语言说明

**主要语言**: 中文（本项目所有注释、文档、提交信息均使用中文）

## 项目概览

JadeBi 是一个基于 Rust 和 Svelte 的块笔记桌面应用（类似 Notion），采用 pnpm monorepo 架构。项目已建立完整的类型系统、UI 组件库和 Tauri 桌面应用基础，支持类型安全的命令调用和插件扩展。

### 技术栈
- **前端**: Svelte 5.54.0 + TypeScript 5.9.3
- **桌面框架**: Tauri 2.10.1
- **包管理器**: pnpm 10.30.1
- **Node.js**: >= 18.0.0
- **语言**: TypeScript、Rust

### 当前状态
- ✅ 已建立 pnpm monorepo 基础结构
- ✅ 创建了类型安全的命令和事件系统（@jade-bi/api 包）
- ✅ 建立了完整的 UI 设计系统（@jade-bi/ui-kit 包）
- ✅ Tauri 桌面应用已创建（app）
- ✅ 路由系统已建立（/welcome 欢迎页、/vault 仓库页）
- ✅ WindowTitlebar 组件已添加
- ⏳ 插件系统类型基础已建立，具体实现待完善

`★ Insight ─────────────────────────────────────`
JadeBi 采用类型优先的架构设计：先定义严格的命令和事件类型系统，再基于此构建应用。这种设计确保了插件系统的类型安全性和可扩展性。类型系统通过泛型和条件类型实现编译时安全性。
`─────────────────────────────────────────────────`

## 项目结构说明

```
jadebi/
├── app/                   # Tauri 桌面应用（主应用）
│   ├── src/              # Svelte 前端代码
│   │   ├── lib/          # 共享库（组件、工具函数）
│   │   │   └── components/ # 可复用组件（从 routes/components 迁移）
│   │   └── routes/       # SvelteKit 路由
│   │       ├── +layout.svelte    # 应用布局容器
│   │       ├── +page.ts          # 根路径重定向（/ → /welcome）
│   │       ├── welcome/          # 欢迎页面
│   │       └── vault/            # 仓库编辑器页面
│   ├── src-tauri/        # Rust 后端代码
│   │   ├── Cargo.toml    # Rust 依赖配置
│   │   ├── tauri.conf.json # Tauri 应用配置
│   │   └── src/          # Rust 源代码
│   └── package.json      # 应用依赖配置
├── ui-kit/               # UI 组件库（@jade-bi/ui-kit）
│   ├── src/lib/          # Svelte 组件（Button, Input, Card 等）
│   │   └── components/   # 组件目录
│   │       └── WindowTitlebar.svelte # 窗口标题栏组件
│   └── static/           # 静态资源
├── packages/             # 共享包目录
│   └── jadebi-api/       # 类型安全的命令和事件系统（@jade-bi/api）
│       └── src/lib/      # 核心类型定义和命令调用器
├── package.json          # 根项目配置和脚本
├── pnpm-workspace.yaml   # monorepo 工作空间配置
└── CLAUDE.md            # 项目指南
```

### 包职责

| 包名 | 版本 | 职责 | 状态 |
|------|------|------|------|
| `@jade-bi/app` | 0.1.0 | Tauri 桌面应用（主应用入口） | ✅ 已创建 |
| `@jade-bi/api` | 0.0.1 | 类型安全的命令和事件系统，提供 Tauri 命令调用器 | ✅ 已创建 |
| `@jade-bi/ui-kit` | 0.0.1 | UI 组件库，包含完整的设计系统和组件 | ✅ 已创建 |

`★ Insight ─────────────────────────────────────`
项目采用三层次架构：应用层（app）、类型系统层（@jade-bi/api）、UI 层（@jade-bi/ui-kit）。类型系统通过泛型提供编译时类型安全，UI 组件库通过设计令牌实现主题一致性。Tauri 应用通过 `@jade-bi/api` 的类型安全调用器与 Rust 后端通信。
`─────────────────────────────────────────────────`

## 依赖管理

项目使用 pnpm catalog 统一管理共享依赖版本，确保整个工作空间中依赖版本的一致性。

### Catalog 配置

在 `pnpm-workspace.yaml` 中定义共享依赖版本，各子包通过 `catalog:` 引用：

```yaml
catalog:
  typescript: ^5.9.3
  vite: ^7.3.1
  svelte: ^5.54.0
  svelte-check: ^4.4.5
  "@sveltejs/kit": ^2.55.0
  "@sveltejs/vite-plugin-svelte": ^6.2.4
```

### 子包引用方式

各包的 `package.json` 中使用 `catalog:` 引用共享依赖：

```json
{
  "devDependencies": {
    "typescript": "catalog:",
    "vite": "catalog:",
    "svelte": "catalog:"
  }
}
```

### 依赖更新

运行 `pnpm update` 可自动更新 catalog 中的版本，所有引用 catalog 的包都会同步更新。

`★ Insight ─────────────────────────────────────`
pnpm catalog 的核心优势在于集中化版本管理。不同于传统的每个包独立声明版本，catalog 允许在 workspace 级别定义统一的版本策略。当某个依赖需要升级时，只需修改 `pnpm-workspace.yaml` 中的 catalog 定义，运行 `pnpm update` 即可一键更新所有子包，避免了版本碎片化问题。
`─────────────────────────────────────────────────`

## 开发命令

### 包管理
```bash
# 安装所有依赖
pnpm install

# 添加依赖到根项目
pnpm add -w <package>

# 添加依赖到特定工作空间包
pnpm add -F @jade-bi/app <package>      # 添加到桌面应用
pnpm add -F @jade-bi/api <package>      # 添加到 API 类型系统包
pnpm add -F @jade-bi/ui-kit <package>   # 添加到 UI 组件库
```

### 根项目脚本（已配置在 package.json）
根目录的 `package.json` 已配置以下脚本：
```bash
pnpm dev              # 启动桌面应用开发服务器
pnpm tauri:dev        # 启动 Tauri 开发模式（包含 Rust 后端）
pnpm build            # 构建桌面应用
pnpm api:build        # 构建 @jade-bi/api 包
pnpm ui-kit:dev       # 启动 UI 组件库开发服务器
pnpm ui-kit:build     # 构建 @jade-bi/ui-kit 包
```

### 常用命令示例
```bash
# 启动桌面应用开发服务器（前端）
pnpm dev

# 启动 Tauri 完整开发环境（前端 + Rust 后端）
pnpm tauri:dev

# 构建桌面应用
pnpm build

# 在特定包中运行开发服务器
pnpm --filter @jade-bi/app dev
pnpm --filter @jade-bi/api dev
pnpm --filter @jade-bi/ui-kit dev

# 构建特定包
pnpm --filter @jade-bi/app build
pnpm --filter @jade-bi/api build
pnpm --filter @jade-bi/ui-kit build

# 运行类型检查
pnpm --filter @jade-bi/app check
pnpm --filter @jade-bi/api check
pnpm --filter @jade-bi/ui-kit check

# 执行 Tauri 命令
pnpm --filter @jade-bi/app tauri <command>
```

`★ Insight ─────────────────────────────────────`
pnpm monorepo 的 `--filter` 标志允许精准控制包操作范围。`-w` 表示工作空间根目录，`-F` 表示特定包。Tauri 开发需要同时运行前端开发服务器和 Rust 后端，`tauri:dev` 脚本自动协调这一过程。
`─────────────────────────────────────────────────`

## 架构设计说明

### 类型系统设计
项目采用泛型类型系统来确保命令和事件的安全性，通过 `@jade-bi/api` 包提供：

#### 命令类型 (`command.ts`)
```typescript
export type Command<
  NAME extends string,
  ARGS extends Record<string, unknown> | void = void,
  RESULT = null,
> = {
  name: NAME
  args: ARGS
  result: RESULT
}
```

#### 事件类型 (`emit.ts`)
```typescript
export type Emit<
  NAME extends string,
  PAYLOAD = unknown,
> = {
  name: NAME
  payload: PAYLOAD
}
```

#### 类型系统入口 (`io.ts`)
```typescript
export type Commands = unknown  // 将由具体应用填充
export type Emits = unknown     // 将由具体应用填充
```

`★ Insight ─────────────────────────────────────`
类型系统采用 "占位符" 设计：`Commands` 和 `Emits` 当前为 `unknown`，等待具体应用填充。这种设计允许类型系统提前定义，应用实现时再提供具体类型，实现了关注点分离。命令参数支持 `void` 表示无参数，返回值默认为 `null` 以匹配 Tauri 后端的 `Ok(())` 默认值。
`─────────────────────────────────────────────────`

### 命令调用器（Invoker）
`@jade-bi/api` 提供了类型安全的命令调用器，使用 Svelte 5 的 runes 系统实现响应式状态管理：

```typescript
import { invoker } from '@jade-bi/api/invoker';

// 创建类型安全的调用器实例
const saveNote = invoker('save_note');

// 调用命令（参数类型自动推断）
await saveNote.call({ title: 'Note', content: '...' });

// 响应式状态访问
$effect(() => {
  if (saveNote.isSuccess()) {
    console.log('Saved:', saveNote.data);
  }
  if (saveNote.isError()) {
    console.error('Error:', saveNote.error);
  }
});
```

### Tauri 集成架构
Tauri 桌面应用通过类型安全的 IPC 与 Rust 后端通信：

```
前端 (Svelte) <--[类型安全命令/事件]--> @jade-bi/api <--[Tauri IPC]--> Rust 后端 (app/src-tauri/src)
```

- **前端**: 使用 `invoker` 调用 Tauri 命令，通过 `@tauri-apps/api` 通信
- **类型系统**: `@jade-bi/api` 提供编译时类型检查，确保命令名称、参数和返回值的正确性
- **Rust 后端**: 在 `app/src-tauri/src` 中实现命令处理程序

### UI 设计系统
`@jade-bi/ui-kit` 包提供完整的组件库和设计令牌系统：

- **设计令牌**: 通过 CSS 变量定义颜色、间距、字体等设计属性
- **组件库**: Button、Input、Card、Form、Grid、Flex 等基础组件
- **主题系统**: 支持 light/dark/system 主题，通过 `data-theme` 属性切换
- **类型安全**: 所有组件都有完整的 TypeScript 类型定义
- **WindowTitlebar**: 窗口标题栏组件，用于桌面应用的窗口控制（最小化、最大化、关闭）

### 路由系统架构
项目采用 SvelteKit 的文件系统路由，主要路由如下：

- **根路径 `/`**: 通过 `+page.ts` 重定向到 `/welcome`
- **欢迎页 `/welcome`**: 全新欢迎页面，提供创建或打开仓库的选项
- **仓库页 `/vault`**: 仓库编辑器页面（stub 实现）

路由结构遵循 SvelteKit 最佳实践：
- `+layout.svelte`: 应用布局容器，负责导入主题和全局样式
- `+page.ts`: 根路径重定向逻辑
- 各页面组件自行包含 `WindowTitlebar` 组件

### 组件目录重构
组件从 `routes/components/` 移动到 `lib/components/`，这是 SvelteKit 推荐的最佳实践：
- `lib/components/`: 可复用组件目录
- `routes/`: 页面级组件和路由定义

### 插件系统架构
插件通过类型扩展机制添加新功能，保持类型安全：

```typescript
// 插件类型声明
declare module '@jade-bi/api' {
  export interface Commands {
    'plugin.specialAction': Command<'plugin.specialAction', { data: string }, { success: boolean }>
  }

  export interface Emits {
    'plugin.notification': Emit<'plugin.notification', { message: string }>
  }
}

// 插件实现
const pluginInvoker = invoker('plugin.specialAction');
```

`★ Insight ─────────────────────────────────────`
项目采用三层架构：应用层（app）、类型系统层（@jade-bi/api）、UI 层（@jade-bi/ui-kit）。类型系统通过泛型和条件类型提供编译时安全性，UI 组件库通过设计令牌实现一致性，插件系统通过类型扩展实现可扩展性。这种分离确保了关注点分离和代码复用。
`─────────────────────────────────────────────────`

## 开发工作流程

### 环境设置
1. **Node.js**: 确保安装 Node.js >= 18.0.0
2. **pnpm**: 安装 pnpm 10.30.1 或更高版本（与 `packageManager` 字段一致）
3. **Rust**: 安装 Rust 工具链（为 Tauri 开发准备）
4. **IDE 推荐**: VS Code + TypeScript + Rust 扩展

### 开发建议
1. **类型安全优先**: 充分利用 TypeScript 的严格模式
2. **包边界清晰**: 保持包之间的职责分离
3. **插件友好设计**: 考虑扩展性，为插件系统留出接口
4. **Tauri 最佳实践**: 遵循 Tauri 的安全和性能指南

### 测试策略
- **单元测试**: 针对工具函数和业务逻辑
- **集成测试**: 测试包之间的交互
- **端到端测试**: 针对桌面应用功能

## 注意事项

### 已知问题
1. **早期开发阶段**: 核心架构已建立，但应用功能尚未完全实现
2. **Rust 后端基础**: Tauri 后端已创建，但命令处理程序需要具体实现
3. **测试覆盖不足**: 缺少完整的测试套件
4. **插件系统待实现**: 类型基础已建立，但具体插件机制待开发

### 架构决策
1. **Monorepo 选择**: 使用 pnpm workspace 管理多包项目
2. **类型优先设计**: 先定义类型系统，再实现具体功能
3. **插件系统设计**: 通过类型扩展实现插件功能
4. **Tauri 框架**: 选择 Tauri 而非 Electron，追求更好的性能和安全性

### 兼容性要求
- Node.js >= 18.0.0
- pnpm >= 10.0.0（推荐 10.30.1）
- TypeScript >= 5.9.3
- Svelte >= 5.54.0

## 贡献指南

### 代码规范
- **TypeScript**: 使用严格模式，避免 `any` 类型
- **命名约定**: 驼峰命名法，类型使用 PascalCase
- **文件组织**: 按功能模块组织，避免过大的文件
- **注释要求**: 公共 API 必须有 JSDoc 注释

### 提交消息格式
遵循 Conventional Commits 规范：
```
类型(范围): 描述

正文（可选）

脚注（可选）
```

**类型**:
- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或工具调整

**示例**:
```
feat(api): 添加类型安全命令调用器

- 实现 invoker 工厂函数
- 添加命令类型定义
- 提供响应式状态管理

Closes #123
```

### PR 流程
1. 从 `main` 分支创建特性分支
2. 开发完成后运行测试和 lint 检查
3. 提交符合规范的 commit 消息
4. 创建 Pull Request，描述变更内容和测试计划

---

## 项目记忆

### 2026-03-19 项目分析记录
- **项目状态**: 核心架构已建立，包含类型系统、UI 组件库和 Tauri 应用基础
- **技术栈确认**: Svelte 5.54.0 + Tauri 2.10.1 + TypeScript 5.9.3 + pnpm 10.30.1
- **架构特点**: 三层架构（应用层、类型系统层、UI 层），类型优先设计
- **包结构**: `app`（桌面应用）、`@jade-bi/api`（类型系统）、`@jade-bi/ui-kit`（UI 组件库）
- **开发建议**: 需要完善 Rust 后端命令实现和插件系统具体机制

### 代码规范记录
- **类型安全**: 优先使用 TypeScript 严格模式
- **包管理**: 使用 pnpm workspace 进行 monorepo 管理
- **提交规范**: 遵循 Conventional Commits 格式
- **文档要求**: 公共 API 必须包含 JSDoc 注释

### 2026-03-31 目录结构重构
- **目录变更**: `jadebi-app/` → `app/`，`packages/jadebi-ui/` → `ui-kit/`
- **包名变更**: `@jadebi/api` → `@jade-bi/api`，`@jadebi/ui` → `@jade-bi/ui-kit`
- **更新文件**: CLAUDE.md、package.json、pnpm-workspace.yaml

## 项目目标规划（2026-03-31）

### 目标一：良好的编写体验，支持自定义插件扩展

**核心诉求**：提供流畅的内容创作体验，支持多种内容类型，允许通过插件扩展功能。

**插件 vs Skills 的职责边界**：

| 系统 | 职责 | 示例 |
|------|------|------|
| **插件** | 扩展功能 | 新增 Markdown 编辑器、Mermaid 图表、主题皮肤 |
| **Skills** | 让 AI 使用功能 | 告诉 AI 有哪些命令、参数含义、何时调用 |

关系：**插件提供能力，Skills 让 AI 理解和使用这些能力**。

**插件系统技术要点**：
- 基于 `@jade-bi/api` 的类型系统提供插件 API
- 插件声明式注册（manifest 格式）
- 沙箱化执行环境
- 热加载/卸载支持

---

### 目标二：深度 AI 集成，所有操作均可由 CLI 完成

**核心诉求**：AI 可以深度理解、检索、生成和操作用户的知识库，成为用户知识管理的智能助手。

**大模型三层能力**：

| 能力层 | 说明 | 典型应用场景 |
|--------|------|--------------|
| **检索** | 基于向量语义搜索找到相关知识 | 用户提问时，AI 检索相关笔记内容后回答 |
| **生成** | 基于已有文档编写新内容或修改 | 根据笔记大纲生成完整文章、续写内容、润色改写 |
| **执行** | 通过 CLI 调用完成具体操作 | 创建笔记、添加标签、整理文件夹、导出文档 |

**AI 与知识库交互流程示例**：

```
用户："帮我基于上次整理的 Rust 学习笔记，写一篇入门教程"
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│  AI 思考过程                                                │
│  1. 【检索】调用 jadebi-cli search --query "Rust 学习"       │
│     → 获取相关笔记列表                                       │
│  2. 【检索】读取具体笔记内容                                  │
│     → 获取详细内容                                           │
│  3. 【生成】基于内容撰写入门教程                              │
│     → 生成结构化教程内容                                     │
│  4. 【执行】创建新笔记                                       │
│     → jadebi-cli note create --title "Rust 入门教程"          │
└─────────────────────────────────────────────────────────────┘
        │
        ▼
用户：看到新创建的 "Rust 入门教程" 笔记，内容已填充
```

**核心设计原则**：

1. **AI 是用户的知识助手，不是替代品**
   - AI 帮助整理、检索、生成，但用户保持控制权
   - 所有 AI 操作都可被用户审查和修改

2. **知识库是核心，AI 是增强层**
   - 没有 AI，JadeBi 仍是完整的笔记应用
   - AI 只是让知识库更易用、更有价值

3. **透明可审计**
   - AI 的操作记录可追溯
   - 用户知道 AI 做了什么、基于什么做的

**CLI 定位**：

**给人类和 AI 共同使用**，设计为：
- 同一套命令，通过 `--format` 区分输出格式
- 人类模式：友好表格 + 交互提示（`JADEBI_CLI_MODE=human`）
- AI 模式：结构化 JSON + 无交互（`JADEBI_CLI_MODE=ai`）

**Skills 系统边界**：

**Skills 负责**：让 AI 使用功能
- 告诉 AI 有哪些命令可用
- 解释命令的参数和返回值
- 将自然语言意图转换为 CLI 命令序列
- 处理 AI 的操作结果反馈

**Skills 实现方式**：
- 每个 Skill 是一个声明式描述文件
- 描述可用的 CLI 命令及其用途
- 提供自然语言到命令的映射示例

**CLI 技术实现要点**：
- CLI 基于 Rust 实现（性能、跨平台）
- 与桌面应用共享核心逻辑（通过 crate 复用）
- 支持 MCP（Model Context Protocol）协议供 AI 调用
- 命令自动发现机制

---

### 目标三：内置向量化引擎，支持文档向量化处理

**核心诉求**：文档可被向量化存储和检索，支持语义搜索，方便 AI 基于知识库内容进行问答。

**向量化引擎设计原则**：

**应用提供接入点，用户自选嵌入方案**：

1. **应用层职责**：
   - 文档分块和预处理
   - 向量存储和索引管理
   - 语义检索接口
   - 增量更新机制

2. **嵌入模型由用户配置**：
   - 本地 Ollama 模型（隐私优先）
   - 云端 API（OpenAI、智谱等）
   - 自定义本地模型路径

**技术实现要点**：

**向量存储方案**：
- 优先使用 SQLite + sqlite-vec 扩展（轻量、无额外依赖）
- 备选：LanceDB（专为 AI 设计，支持复杂查询）
- 大规模场景可配置外部向量数据库

**嵌入模型接入**：
- 统一接口定义：`EmbeddingProvider`
- 内置 OllamaProvider、OpenAIProvider
- 支持用户自定义 Provider

**文档处理流程**：
1. 文档变更检测（文件系统监听或手动触发）
2. 文档解析（Markdown、PDF、HTML 等）
3. 文本分块（滑动窗口、语义分块等策略）
4. 调用嵌入模型生成向量
5. 存储到向量数据库
6. 更新索引元数据

---

### 三个目标的协作关系

```
┌─────────────────────────────────────────────────────────────┐
│                      用户交互层                              │
│         (桌面应用 GUI / CLI / AI 助手)                       │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│   目标一      │    │   目标二      │    │   目标三      │
│  插件系统     │◄──►│  CLI + Skills │◄──►│ 向量化引擎   │
│  （扩展功能） │    │ （AI 集成）   │    │（语义能力）  │
└───────────────┘    └───────────────┘    └───────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    核心基础设施                              │
│    (@jade-bi/api 类型系统 / 存储层 / 配置管理 / 权限系统)        │
└─────────────────────────────────────────────────────────────┘
```

**协作场景示例**：

**场景 1：用户想让 AI 基于知识库回答问题**
1. **目标三（向量化引擎）**：将用户的历史笔记向量化存储
2. **目标二（CLI + Skills）**：AI 通过 CLI 调用语义搜索，找到相关笔记
3. **AI 生成回答**：基于检索到的内容回答用户问题

**场景 2：用户安装了一个思维导图插件**
1. **目标一（插件系统）**：用户安装思维导图插件，新增思维导图编辑器
2. **目标二（Skills）**：AI 了解这个新插件的能力，可以帮用户创建思维导图
3. **目标三（向量化）**：思维导图内容也可以被向量化，用于语义搜索

---

## 风险与挑战

### 技术风险
| 风险点 | 影响 | 缓解措施 |
|--------|------|----------|
| 向量存储性能瓶颈 | 大规模文档时查询缓慢 | 支持分片、索引优化、外部向量数据库 |
| CLI 与 GUI 状态同步 | 两者状态不一致导致混乱 | 统一数据源、事件驱动更新、状态版本控制 |
| 插件安全性 | 恶意插件窃取数据或破坏系统 | 沙箱化、权限声明、签名验证 |

### 产品风险
| 风险点 | 影响 | 缓解措施 |
|--------|------|----------|
| 功能过于复杂 | 用户学习成本高，望而却步 | 渐进式功能暴露、优秀的引导流程、文档 |
| 与现有产品差异化不足 | 用户没有迁移动力 | 专注 AI 原生体验、本地优先、开放生态 |

---

## 下一步行动建议

### Phase 1：基础设施完善（1-2 周）
- [ ] 完善 CLI 基础架构（Rust crate 结构）
- [ ] 设计并实现 Skills 系统的声明式描述格式
- [ ] 确定向量存储方案（推荐 SQLite + sqlite-vec）

### Phase 2：核心能力实现（2-4 周）
- [ ] 实现向量化引擎核心（分块、存储、检索）
- [ ] 实现 CLI 核心命令（note、search、config）
- [ ] 实现 Skills 注册和发现机制

### Phase 3：集成与优化（2-3 周）
- [ ] CLI 与桌面应用状态同步
- [ ] AI 通过 MCP 协议调用 CLI
- [ ] 性能优化（大规模文档测试）

### Phase 4：插件生态启动（持续）
- [ ] 插件 SDK 完善
- [ ] 示例插件开发（思维导图、流程图等）
- [ ] 插件市场（可选）

---

## 项目记忆

### 2026-03-19 项目分析记录
- **项目状态**: 核心架构已建立，包含类型系统、UI 组件库和 Tauri 应用基础
- **技术栈确认**: Svelte 5.54.0 + Tauri 2.10.1 + TypeScript 5.9.3 + pnpm 10.30.1
- **架构特点**: 三层架构（应用层、类型系统层、UI 层），类型优先设计
- **包结构**: `app`（桌面应用）、`@jade-bi/api`（类型系统）、`@jade-bi/ui-kit`（UI 组件库）
- **开发建议**: 需要完善 Rust 后端命令实现和插件系统具体机制

### 代码规范记录
- **类型安全**: 优先使用 TypeScript 严格模式
- **包管理**: 使用 pnpm workspace 进行 monorepo 管理
- **提交规范**: 遵循 Conventional Commits 格式
- **文档要求**: 公共 API 必须包含 JSDoc 注释

### 2026-03-31 目录结构重构
- **目录变更**: `jadebi-app/` → `app/`，`packages/jadebi-ui/` → `ui-kit/`
- **包名变更**: `@jadebi/api` → `@jade-bi/api`，`@jadebi/ui` → `@jade-bi/ui-kit`
- **更新文件**: CLAUDE.md、package.json、pnpm-workspace.yaml

### 2026-03-31 项目目标规划确立
- **目标一**: 良好的编写体验，支持自定义插件扩展
  - 插件负责扩展功能，Skills 负责让 AI 使用功能
  - 两者关系：插件提供能力，Skills 让 AI 理解和使用这些能力
- **目标二**: 深度 AI 集成，所有操作均可由 CLI 完成
  - CLI 给人类和 AI 共同使用，同一套命令，不同输出格式
  - 人类模式：友好表格 + 交互提示
  - AI 模式：结构化 JSON + 无交互
- **目标三**: 内置向量化引擎，支持文档向量化处理
  - 应用提供接入点，用户自选嵌入方案
  - 应用层负责：分块、存储、检索、增量更新
  - 嵌入模型：本地 Ollama 或云端 API（用户配置）

### 2026-04-08 路由系统重构
- **路由结构**: 新增 `/welcome`（欢迎页）和 `/vault`（仓库页）路由
- **根路径重定向**: `/` 通过 `+page.ts` 重定向到 `/welcome`
- **组件目录重构**: 组件从 `routes/components/` 移动到 `lib/components/`
- **Layout 简化**: `+layout.svelte` 简化为应用容器，导入 `@jade-bi/ui-kit/theme`
- **WindowTitlebar**: 新增窗口标题栏组件，移至各页面组件内部
- **API 类型**: `@jade-bi/api` 新增 `VaultMetadata` 类型

---

*本文件由 Claude Code 基于实际代码库分析生成，将随项目发展更新。*

**最后更新**: 2026-04-08
