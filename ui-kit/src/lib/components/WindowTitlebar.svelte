<script lang="ts">
  import {getCurrentWindow} from '@tauri-apps/api/window';
  import type {Snippet} from 'svelte';
  import {onMount} from 'svelte';

  // 组件属性接口
  interface Props {
    /** 窗口标题文字 */
    title?: string;
    /** 标题右侧的自定义内容 */
    afterTitle?: Snippet;
    /** 窗口控制按钮左侧的自定义内容 */
    beforeControls?: Snippet;
  }

  // 获取当前窗口实例
  const appWindow = getCurrentWindow();

  // 属性默认值
  let {title = 'JadeBi', afterTitle, beforeControls}: Props = $props();

  // 最大化状态
  let isMaximized = $state(false);

  // 事件监听器清理函数
  let unlisten: (() => void) | undefined;

  /** 最小化窗口 */
  async function minimize() {
    await appWindow.minimize();
  }

  /** 切换窗口最大化状态 */
  async function toggleMaximize() {
    const _isMaximized = await appWindow.isMaximized();
    if (_isMaximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
    isMaximized = !_isMaximized;
  }

  /** 关闭窗口 */
  async function close() {
    await appWindow.close();
  }

  /** 启动窗口拖拽 */
  async function startDragging() {
    await appWindow.startDragging();
  }

  /** 从系统获取当前窗口最大化状态并同步到组件状态 */
  async function updateMaximizedState() {
    try {
      isMaximized = await appWindow.isMaximized();
    } catch (e) {
      console.error('获取窗口状态失败:', e);
    }
  }

  // 组件挂载时初始化状态并设置事件监听
  onMount(() => {
    // 初始化获取窗口状态
    updateMaximizedState();

    // 监听窗口尺寸变化事件，保持按钮图标与实际状态同步
    appWindow.onResized(() => {
      updateMaximizedState();
    }).then(unsub => {
      unlisten = unsub;
    });

    // 组件卸载时清理事件监听器
    return () => {
      unlisten?.();
    };
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="titlebar"
  onmousedown={(e) => {
    if (!(e.target as HTMLElement).closest('button')) {
      startDragging();
    }
  }}
>
  <div class="titlebar-title">
    {title}
    {#if afterTitle}
      {@render afterTitle()}
    {/if}
  </div>

  <div class="titlebar-controls">
    {#if beforeControls}
      {@render beforeControls()}
    {/if}
    <button
      class="control-button minimize"
      onclick={minimize}
      aria-label="最小化"
    >
      <svg width="12" height="12" viewBox="0 0 12 12">
        <rect x="1" y="5.5" width="10" height="1" fill="currentColor"/>
      </svg>
    </button>

    <button
      class="control-button maximize"
      onclick={toggleMaximize}
      aria-label={isMaximized ? '还原' : '最大化'}
    >
      {#if isMaximized}
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="2.5" y="4" width="6" height="5.5" fill="none" stroke="currentColor" stroke-width="1"/>
          <path d="M4 4V2.5h6.5V9H9" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
      {:else}
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="1.5" y="1.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
      {/if}
    </button>

    <button
      class="control-button close"
      onclick={close}
      aria-label="关闭"
    >
      <svg width="12" height="12" viewBox="0 0 12 12">
        <path d="M2 2L10 10M10 2L2 10" stroke="currentColor" stroke-width="1.2"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 32px;
    background: var(--color-bg-primary, #1A1A1A);
    color: var(--color-text-primary, #F0F0F0);
    user-select: none;
    padding: 0 8px;
  }

  .titlebar-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 500;
    opacity: 0.9;
  }

  .titlebar-controls {
    display: flex;
  }

  .control-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 46px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--color-text-primary, #F0F0F0);
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .control-button:hover {
    background: var(--color-bg-hover, rgba(255, 255, 255, 0.1));
  }

  .control-button.close:hover {
    background: #E81123;
  }

  .control-button:active {
    background: var(--color-bg-active, rgba(255, 255, 255, 0.15));
  }

  .control-button.close:active {
    background: #BF0F1D;
  }
</style>
