<script lang="ts">
  import type {Snippet} from 'svelte';

  interface Props {
    /** 卡片标题 */
    title?: string
    /** 是否可点击 */
    clickable?: boolean
    /** 是否禁用 */
    disabled?: boolean
    /** 额外的 CSS 类 */
    class?: string
    /** 点击事件处理 */
    onclick?: (event: MouseEvent) => void
    /** 自定义内容区域 */
    children: Snippet
  }

  let {
    title,
    clickable = false,
    disabled = false,
    class: className = '',
    onclick,
    children,
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  class="card {className}"
  class:clickable
  class:disabled
  role={clickable ? 'button' : undefined}
  tabindex={clickable ? 0 : -1}
  onclick={disabled ? undefined : onclick}
  onkeydown={(e) => {
    if (clickable && !disabled && (e.key === 'Enter' || e.key === ' ')) {
      e.preventDefault();
      onclick?.(new MouseEvent('click'));
    }
  }}
>
  {#if title}
    <h3 class="card-title">{title}</h3>
  {/if}
  <div class="card-content">
    {@render children()}
  </div>
</div>

<style>
  .card {
    background: var(--color-bg-secondary, #2a2a2a);
    border: 1px solid var(--color-border, #3a3a3a);
    border-radius: var(--radius-md, 8px);
    padding: 20px;
    transition: all 0.2s ease;
  }

  .card.clickable {
    cursor: pointer;
  }

  .card.clickable:hover {
    background: var(--color-bg-hover, #333333);
    border-color: var(--color-border-hover, #4a4a4a);
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .card.clickable:active {
    transform: translateY(0);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }

  .card.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .card-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 12px 0;
  }

  .card-content {
    color: var(--color-text-secondary, #b0b0b0);
    font-size: 14px;
  }
</style>