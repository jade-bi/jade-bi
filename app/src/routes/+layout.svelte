<script lang="ts">
  import {WindowTitlebar} from '@jade-bi/ui-kit';
  import {goto} from '$app/navigation';
  import {onMount} from 'svelte';
  import type {Snippet} from 'svelte';

  interface Props {
    children: Snippet
  }

  let {children}: Props = $props();

  // 检查当前是否有打开的仓库，决定是否跳转
  onMount(async () => {
    // TODO: 实现检查当前仓库的逻辑
    // 如果有打开的仓库，跳转到仓库页
    // const currentVault = await invoke('vault:get_current');
    // if (currentVault.vault) {
    //   goto('/pages/vault');
    // }
  });
</script>

<div class="app-container">
  <WindowTitlebar title="玉璧" />

  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  :global(html), :global(body) {
    margin: 0 !important;
    padding: 0 !important;
    height: 100% !important;
    width: 100% !important;
    overflow: hidden !important;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow: auto;
    background: var(--color-bg-primary, #1a1a1a);
  }
</style>