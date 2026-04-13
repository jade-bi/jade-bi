<script lang="ts">
  import {invoke} from '@tauri-apps/api/core';
  import {WindowTitlebar, CreateVaultForm} from '@jade-bi/ui-kit';
  import type {VaultMetadata} from '@jade-bi/api';
  import CreateVaultCard from '$lib/components/CreateVaultCard.svelte';
  import OpenVaultCard from '$lib/components/OpenVaultCard.svelte';
  import S3VaultCard from '$lib/components/S3VaultCard.svelte';
  import VaultList from '$lib/components/VaultList.svelte';

  // 视图状态
  type ViewState = 'welcome' | 'create-vault';
  let currentView = $state<ViewState>('welcome');

  // 仓库列表
  let recentVaults = $state<VaultMetadata[]>([]);
  let loading = $state(false);
  let error = $state('');

  // 加载仓库列表
  async function loadRecentVaults() {
    loading = true;
    error = '';
    try {
      const result = await invoke<VaultMetadata[]>('vault_registry_get_recent', {limit: 10});
      recentVaults = result;
    } catch (e) {
      error = String(e);
      console.error('加载仓库列表失败:', e);
    } finally {
      loading = false;
    }
  }

  // 组件挂载时加载仓库列表
  $effect(() => {
    loadRecentVaults();
  });

  /**
   * 处理新建仓库
   */
  function handleCreateVault() {
    currentView = 'create-vault';
  }

  /**
   * 处理打开仓库
   */
  async function handleOpenVault() {
    // TODO: 实现打开仓库逻辑（调用系统文件选择器）
    console.log('打开仓库');
  }

  /**
   * 处理 S3 实例化
   */
  function handleS3Vault() {
    // TODO: 实现 S3 实例化逻辑（打开 S3 配置对话框）
    console.log('S3 实例化');
  }

  /**
   * 处理选择仓库
   */
  async function handleSelectVault(vault: VaultMetadata) {
    try {
      // 更新访问时间
      await invoke('vault_registry_update_access_time', {vaultId: vault.id});

      // 获取完整仓库信息并在新窗口中打开
      const vaultData = await invoke<any>('vault_manager_get_by_id', {vaultId: vault.id});
      if (vaultData) {
        await invoke('window_open_vault', {
          vaultPath: vaultData.path,
          vaultName: vaultData.name,
        });
      }
    } catch (e) {
      console.error('打开仓库失败:', e);
    }
  }

  /**
   * 处理创建仓库表单提交
   */
  async function handleCreateFormSubmit(data: {name: string; path: string}) {
    try {
      // 调用后端创建仓库
      await invoke('vault_manager_create_local', {
        name: data.name,
        path: data.path,
        description: null,
      });

      // 重新加载仓库列表
      await loadRecentVaults();

      // 返回欢迎页
      currentView = 'welcome';
    } catch (e) {
      console.error('创建仓库失败:', e);
      throw e; // 重新抛出以便表单显示错误
    }
  }

  /**
   * 处理创建表单取消
   */
  function handleCreateFormCancel() {
    currentView = 'welcome';
  }
</script>

<div class="welcome-container">
  <WindowTitlebar title="玉璧" />

  {#if currentView === 'welcome'}
    <div class="welcome-page">
      <aside class="sidebar">
        <div class="sidebar-content">
          <VaultList
            vaults={recentVaults}
            onselect={handleSelectVault}
            {loading}
          />
        </div>
      </aside>

      <main class="main">
        <div class="hero">
          <h1 class="title">欢迎使用玉璧</h1>
          <p class="subtitle">选择以下选项开始使用</p>
        </div>

        <div class="cards">
          <CreateVaultCard onclick={handleCreateVault} />
          <OpenVaultCard onclick={handleOpenVault} />
          <S3VaultCard onclick={handleS3Vault} />
        </div>
      </main>
    </div>
  {:else}
    <CreateVaultForm
      oncreate={handleCreateFormSubmit}
      oncancel={handleCreateFormCancel}
    />
  {/if}
</div>

<style>
  .welcome-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }
  .welcome-page {
    display: flex;
    height: 100%;
    min-height: 0;
  }

  .sidebar {
    width: 300px;
    flex-shrink: 0;
    border-right: 1px solid var(--color-border, #3a3a3a);
    background: var(--color-bg-primary, #1a1a1a);
  }

  .sidebar-content {
    padding: 24px;
    height: 100%;
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 48px;
    gap: 32px;
    min-height: 0;
  }

  .hero {
    text-align: center;
  }

  .title {
    font-size: 28px;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 8px 0;
  }

  .subtitle {
    font-size: 16px;
    color: var(--color-text-secondary, #b0b0b0);
    margin: 0;
  }

  .cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
    width: 100%;
    max-width: 900px;
  }

  @media (max-width: 900px) {
    .cards {
      grid-template-columns: 1fr;
      max-width: 320px;
    }
  }
</style>
