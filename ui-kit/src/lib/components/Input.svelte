<script lang="ts">
  interface Props {
    /** 标签文本 */
    label?: string
    /** 输入框类型 */
    type?: 'text' | 'password' | 'email' | 'number'
    /** 占位符文本 */
    placeholder?: string
    /** 输入值（双向绑定） */
    value?: string
    /** 错误信息 */
    error?: string
    /** 是否禁用 */
    disabled?: boolean
    /** 额外的 CSS 类 */
    class?: string
    /** 输入事件处理 */
    oninput?: (event: Event) => void
    /** 失焦事件处理 */
    onblur?: (event: FocusEvent) => void
  }

  let {
    label,
    type = 'text',
    placeholder = '',
    value = $bindable(''),
    error,
    disabled = false,
    class: className = '',
    oninput,
    onblur,
  }: Props = $props();

  const inputId = $derived(label ? label.replace(/\s+/g, '-').toLowerCase() : undefined);
</script>

<div class="input-wrapper {className}">
  {#if label}
    <label class="input-label" for={inputId}>{label}</label>
  {/if}
  <input
    class="input"
    class:error={!!error}
    {type}
    {placeholder}
    {disabled}
    bind:value
    oninput={oninput}
    onblur={onblur}
    id={inputId}
  />
  {#if error}
    <span class="input-error">{error}</span>
  {/if}
</div>

<style>
  .input-wrapper {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .input-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary, #f0f0f0);
  }

  .input {
    width: 100%;
    padding: 10px 12px;
    font-size: 14px;
    color: var(--color-text-primary, #f0f0f0);
    background: var(--color-bg-primary, #1a1a1a);
    border: 1px solid var(--color-border, #3a3a3a);
    border-radius: var(--radius-sm, 6px);
    outline: none;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
    box-sizing: border-box;
  }

  .input::placeholder {
    color: var(--color-text-muted, #666666);
  }

  .input:focus {
    border-color: var(--color-primary, #6366f1);
    box-shadow: 0 0 0 2px var(--color-primary-alpha, rgba(99, 102, 241, 0.2));
  }

  .input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .input.error {
    border-color: var(--color-error, #ef4444);
  }

  .input-error {
    font-size: 12px;
    color: var(--color-error, #ef4444);
  }
</style>