<script lang="ts">
  import type {Snippet} from 'svelte';

  interface Props {
    /** 按钮变体 */
    variant?: 'primary' | 'secondary' | 'ghost'
    /** 按钮尺寸 */
    size?: 'sm' | 'md' | 'lg'
    /** 是否禁用 */
    disabled?: boolean
    /** 是否显示加载状态 */
    loading?: boolean
    /** 额外的 CSS 类 */
    class?: string
    /** 点击事件处理 */
    onclick?: (event: MouseEvent) => void
    /** 自定义内容 */
    children: Snippet
  }

  let {
    variant = 'primary',
    size = 'md',
    disabled = false,
    loading = false,
    class: className = '',
    onclick,
    children,
  }: Props = $props();

  const isDisabled = $derived(disabled || loading);
</script>

<button
  class="button {variant} {size} {className}"
  class:loading
  disabled={isDisabled}
  onclick={isDisabled ? undefined : onclick}
>
  {#if loading}
    <span class="spinner"></span>
  {/if}
  <span class="content" class:loading>
    {@render children()}
  </span>
</button>

<style>
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-weight: 500;
    border-radius: var(--radius-sm, 6px);
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid transparent;
    white-space: nowrap;
  }

  .button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* 尺寸 */
  .sm {
    padding: 6px 12px;
    font-size: 13px;
  }

  .md {
    padding: 10px 16px;
    font-size: 14px;
  }

  .lg {
    padding: 12px 24px;
    font-size: 16px;
  }

  /* 变体 */
  .primary {
    background: var(--color-primary, #6366f1);
    color: white;
  }

  .primary:hover:not(:disabled) {
    background: var(--color-primary-hover, #5558e3);
  }

  .primary:active:not(:disabled) {
    background: var(--color-primary-active, #4849d4);
  }

  .secondary {
    background: var(--color-bg-secondary, #2a2a2a);
    color: var(--color-text-primary, #f0f0f0);
    border-color: var(--color-border, #3a3a3a);
  }

  .secondary:hover:not(:disabled) {
    background: var(--color-bg-hover, #333333);
    border-color: var(--color-border-hover, #4a4a4a);
  }

  .ghost {
    background: transparent;
    color: var(--color-text-secondary, #b0b0b0);
  }

  .ghost:hover:not(:disabled) {
    background: var(--color-bg-hover, #333333);
    color: var(--color-text-primary, #f0f0f0);
  }

  /* 加载状态 */
  .loading .content {
    opacity: 0.7;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid transparent;
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>