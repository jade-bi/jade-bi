<script lang="ts">
  import type {Snippet} from 'svelte';
  import '@jade-bi/ui-kit/theme';

  interface Props {
    children: Snippet
  }

  let {children}: Props = $props();
</script>

<div class="app-container">
  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
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