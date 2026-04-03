<script lang="ts">
  import {getCurrentWindow} from '@tauri-apps/api/window';

  interface Props {
    /** 窗口标题 */
    title?: string
  }

  let {title = 'JadeBi'}: Props = $props();

  const appWindow = getCurrentWindow();
  let maximized = $state(false);

  async function minimize() {
    await appWindow.minimize();
  }

  async function toggleMaximize() {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
    maximized = !isMaximized;
  }

  async function close() {
    await appWindow.close();
  }

  async function startDragging() {
    await appWindow.startDragging();
  }

  async function handleDoubleClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('button')) {
      await toggleMaximize();
    }
  }

  // 初始化时获取窗口状态
  $effect(() => {
    appWindow.isMaximized().then(max => {
      maximized = max;
    });
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
  ondblclick={handleDoubleClick}
>
  <div class="titlebar-title">{title}</div>

  <div class="titlebar-controls">
    <button
      class="control-button minimize"
      onclick={minimize}
      aria-label="最小化"
    >
      <svg width="12" height="12" viewBox="0 0 12 12">
        <rect x="1" y="5.5" width="10" height="1" fill="currentColor" />
      </svg>
    </button>

    <button
      class="control-button maximize"
      onclick={toggleMaximize}
      aria-label={maximized ? '还原' : '最大化'}
    >
      {#if maximized}
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="2.5" y="4" width="6" height="5.5" fill="none" stroke="currentColor" stroke-width="1" />
          <path d="M4 4V2.5h6.5V9H9" fill="none" stroke="currentColor" stroke-width="1" />
        </svg>
      {:else}
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="1.5" y="1.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1" />
        </svg>
      {/if}
    </button>

    <button
      class="control-button close"
      onclick={close}
      aria-label="关闭"
    >
      <svg width="12" height="12" viewBox="0 0 12 12">
        <path d="M2 2L10 10M10 2L2 10" stroke="currentColor" stroke-width="1.2" />
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
    background: var(--color-bg-primary, #1a1a1a);
    color: var(--color-text-primary, #f0f0f0);
    -webkit-app-region: drag;
    user-select: none;
    padding: 0 8px;
  }

  .titlebar-title {
    font-size: 13px;
    font-weight: 500;
    opacity: 0.9;
  }

  .titlebar-controls {
    display: flex;
    -webkit-app-region: no-drag;
  }

  .control-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 46px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--color-text-primary, #f0f0f0);
    cursor: pointer;
    transition: background-color 0.15s ease;
  }

  .control-button:hover {
    background: var(--color-bg-hover, rgba(255, 255, 255, 0.1));
  }

  .control-button.close:hover {
    background: #e81123;
  }

  .control-button:active {
    background: var(--color-bg-active, rgba(255, 255, 255, 0.15));
  }

  .control-button.close:active {
    background: #bf0f1d;
  }
</style>