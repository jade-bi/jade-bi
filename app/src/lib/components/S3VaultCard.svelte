<script lang="ts">
  import {Card} from '@jade-bi/ui-kit';

  interface Props {
    onclick?: () => void
  }

  let {onclick}: Props = $props();
</script>

<Card title="S3 存储" clickable={true} onclick={onclick}>
  <div class="card-inner">
    <div class="icon">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" stroke-linecap="round" stroke-linejoin="round" />
        <path d="M3.27 6.96L12 12.01l8.73-5.05M12 22.08V12" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </div>
    <p class="description">从 S3 兼容存储实例化仓库</p>
  </div>
</Card>

<style>
  .card-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 12px;
  }

  .icon {
    color: var(--color-accent, #4a9eff);
    opacity: 0.8;
  }

  .description {
    margin: 0;
    font-size: 14px;
    color: var(--color-text-secondary, #b0b0b0);
  }
</style>