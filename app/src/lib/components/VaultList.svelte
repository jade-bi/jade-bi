<script lang="ts">
  import type {VaultMetadata} from '@jade-bi/api';

  interface Props {
    /** 仓库列表 */
    vaults?: VaultMetadata[]
    /** 选中事件处理 */
    onselect?: (vault: VaultMetadata) => void
  }

  let {vaults = [], onselect}: Props = $props();

  /**
   * 格式化日期
   */
  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<div class="vault-list">
  <h3 class="list-title">最近打开</h3>

  {#if vaults.length === 0}
    <div class="empty-state">
      <p>暂无最近打开的仓库</p>
    </div>
  {:else}
    <ul class="vaults">
      {#each vaults as vault (vault.id)}
        <li>
          <button
            class="vault-item"
            onclick={() => onselect?.(vault)}
          >
            <div class="vault-icon">
              {#if vault.type === 's3'}
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
                </svg>
              {:else}
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                </svg>
              {/if}
            </div>
            <div class="vault-info">
              <span class="vault-name">{vault.name}</span>
              <span class="vault-path">{vault.path}</span>
            </div>
            <div class="vault-date">
              {formatDate(vault.lastAccessedAt)}
            </div>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .vault-list {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .list-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-secondary, #b0b0b0);
    margin: 0 0 16px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    color: var(--color-text-muted, #666);
    font-size: 14px;
  }

  .vaults {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .vault-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 12px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-md, 8px);
    cursor: pointer;
    text-align: left;
    color: var(--color-text-primary, #f0f0f0);
    transition: all 0.15s ease;
  }

  .vault-item:hover {
    background: var(--color-bg-hover, rgba(255, 255, 255, 0.05));
    border-color: var(--color-border, #3a3a3a);
  }

  .vault-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: var(--color-bg-secondary, #2a2a2a);
    border-radius: var(--radius-sm, 4px);
    color: var(--color-accent, #4a9eff);
  }

  .vault-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .vault-name {
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .vault-path {
    font-size: 12px;
    color: var(--color-text-muted, #666);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .vault-date {
    font-size: 12px;
    color: var(--color-text-muted, #666);
    white-space: nowrap;
  }
</style>