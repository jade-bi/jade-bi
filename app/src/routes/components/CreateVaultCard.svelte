<script lang="ts">
  import {Card} from '@jade-bi/ui-kit';

  interface Props {
    onclick?: () => void
  }

  let {onclick}: Props = $props();
</script>

<Card title="新建仓库" clickable={true} onclick={onclick}>
  <div class="card-inner">
    <div class="icon">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M12 5v14M5 12h14" stroke-linecap="round" />
      </svg>
    </div>
    <p class="description">创建一个新的本地仓库</p>
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