<script lang="ts">
  import {Card} from '@jade-bi/ui-kit';

  interface Props {
    onclick?: () => void
  }

  let {onclick}: Props = $props();
</script>

<Card title="打开仓库" clickable={true} onclick={onclick}>
  <div class="card-inner">
    <div class="icon">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </div>
    <p class="description">打开已存在的本地仓库</p>
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