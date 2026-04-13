<script lang="ts">
  import {open} from '@tauri-apps/plugin-dialog';
  import Input from './Input.svelte';
  import Button from './Button.svelte';

  interface Props {
    /** 创建回调 */
    oncreate?: (data: { name: string; path: string }) => void
    /** 取消回调 */
    oncancel?: () => void
    /** 是否加载中 */
    loading?: boolean
  }

  let {
    oncreate,
    oncancel,
    loading = false,
  }: Props = $props();

  let vaultName = $state('');
  let vaultPath = $state('');
  let nameError = $state('');
  let pathError = $state('');

  /**
   * 选择目录
   */
  async function handleSelectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择仓库存储目录',
      });
      if (selected && typeof selected === 'string') {
        vaultPath = selected;
        pathError = '';
      }
    } catch (e) {
      console.error('选择目录失败:', e);
    }
  }

  /**
   * 验证表单
   */
  function validate(): boolean {
    let valid = true;

    if (!vaultName.trim()) {
      nameError = '请输入仓库名称';
      valid = false;
    } else {
      nameError = '';
    }

    if (!vaultPath.trim()) {
      pathError = '请选择仓库目录';
      valid = false;
    } else {
      pathError = '';
    }

    return valid;
  }

  /**
   * 处理创建
   */
  function handleCreate() {
    if (!validate()) return;

    oncreate?.({
      name: vaultName.trim(),
      path: vaultPath.trim(),
    });
  }

  /**
   * 处理返回
   */
  function handleCancel() {
    oncancel?.();
  }
</script>

<div class="create-vault-form">
  <div class="form-header">
    <h2 class="form-title">新建仓库</h2>
    <p class="form-description">选择仓库存储位置并命名</p>
  </div>

  <div class="form-body">
    <div class="form-field">
      <Input
        label="仓库名称"
        placeholder="输入仓库名称"
        bind:value={vaultName}
        error={nameError}
      />
    </div>

    <div class="form-field">
      <label class="field-label" for="vault-directory">仓库目录</label>
      <div class="path-input">
        <input
          id="vault-directory"
          class="path-input-field"
          type="text"
          placeholder="选择仓库目录"
          value={vaultPath}
          readonly
        />
        <Button variant="secondary" size="sm" onclick={handleSelectDirectory}>
          选择
        </Button>
      </div>
      {#if pathError}
        <span class="field-error">{pathError}</span>
      {/if}
    </div>
  </div>

  <div class="form-actions">
    <Button variant="secondary" onclick={handleCancel}>
      返回
    </Button>
    <Button variant="primary" onclick={handleCreate} {loading}>
      创建
    </Button>
  </div>
</div>

<style>
  .create-vault-form {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 32px;
    box-sizing: border-box;
  }

  .form-header {
    margin-bottom: 32px;
  }

  .form-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 8px 0;
  }

  .form-description {
    font-size: 14px;
    color: var(--color-text-secondary, #b0b0b0);
    margin: 0;
  }

  .form-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary, #f0f0f0);
  }

  .path-input {
    display: flex;
    gap: 8px;
  }

  .path-input-field {
    flex: 1;
    padding: 10px 12px;
    font-size: 14px;
    color: var(--color-text-primary, #f0f0f0);
    background: var(--color-bg-primary, #1a1a1a);
    border: 1px solid var(--color-border, #3a3a3a);
    border-radius: var(--radius-sm, 6px);
    outline: none;
    box-sizing: border-box;
  }

  .path-input-field::placeholder {
    color: var(--color-text-muted, #666666);
  }

  .field-error {
    font-size: 12px;
    color: var(--color-error, #ef4444);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 32px;
    padding-top: 24px;
    border-top: 1px solid var(--color-border, #3a3a3a);
  }
</style>