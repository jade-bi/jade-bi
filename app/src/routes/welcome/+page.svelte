<script lang="ts">
  import {goto} from '$app/navigation';
  import {WindowTitlebar} from '@jade-bi/ui-kit';
  import type {VaultMetadata} from '@jade-bi/api';
  import CreateVaultCard from '$lib/components/CreateVaultCard.svelte';
  import OpenVaultCard from '$lib/components/OpenVaultCard.svelte';
  import S3VaultCard from '$lib/components/S3VaultCard.svelte';
  import VaultList from '$lib/components/VaultList.svelte';

  // 模拟的仓库列表数据（后续将从后端获取）
  let recentVaults = $state<VaultMetadata[]>([
    {
      id: '1',
      name: '我的笔记',
      path: '/home/viktor/notes/my-notes',
      type: 'local',
      lastAccessedAt: new Date().toISOString(),
      modifiedAt: new Date().toISOString(),
      hasUnsavedChanges: false,
    },
  ]);

  /**
   * 处理新建仓库
   */
  function handleCreateVault() {
    // TODO: 实现新建仓库逻辑
    console.log('创建新仓库');
  }

  /**
   * 处理打开仓库
   */
  function handleOpenVault() {
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
  function handleSelectVault(vault: VaultMetadata) {
    // TODO: 调用后端打开仓库
    console.log('打开仓库:', vault.name);
    // 跳转到仓库页
    goto('/vault');
  }
</script>

<div class="welcome-container">
  <WindowTitlebar title="玉璧" />

  <div class="welcome-page">
  <aside class="sidebar">
    <div class="sidebar-content">
      <VaultList vaults={recentVaults} onselect={handleSelectVault} />
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