<script lang="ts">
  import '$lib/theme/index.css';
  let theme = $state('light');

  function toggleTheme() {
    theme = theme === 'light' ? 'dark' : 'light';
    document.documentElement.setAttribute('data-theme', theme);
  }
</script>

<div class="demo">
  <header>
    <h1>JadeBi 主题系统</h1>
    <p>无头组件库的样式基础</p>
    <button onclick={toggleTheme}>
      切换到 {theme === 'light' ? '暗色' : '亮色'} 主题
    </button>
  </header>

  <main>
    <section>
      <h2>颜色系统</h2>
      <div class="color-grid">
        <div class="color-item" style="background: var(--jb-primary); color: var(--jb-on-primary)">
          Primary
        </div>
        <div class="color-item" style="background: var(--jb-secondary); color: var(--jb-on-secondary)">
          Secondary
        </div>
        <div class="color-item" style="background: var(--jb-tertiary); color: var(--jb-on-tertiary)">
          Tertiary
        </div>
        <div class="color-item" style="background: var(--jb-error); color: var(--jb-on-error)">
          Error
        </div>
        <div class="color-item" style="background: var(--jb-surface); color: var(--jb-on-surface)">
          Surface
        </div>
        <div class="color-item" style="background: var(--jb-background); color: var(--jb-on-background)">
          Background
        </div>
      </div>
    </section>

    <section>
      <h2>尺寸系统</h2>
      <div class="size-grid">
        {#each ['3xs', '2xs', 'xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl', '4xl'] as size}
          <div class="size-item">
            <span class="size-box" style="width: var(--jb-size-{size}); height: var(--jb-size-{size})"></span>
            <span class="size-label">{size}</span>
          </div>
        {/each}
      </div>
    </section>

    <section>
      <h2>排版系统</h2>
      <div class="type-sample">
        <p style="font: var(--jb-display1)">Display 1</p>
        <p style="font: var(--jb-display2)">Display 2</p>
        <p style="font: var(--jb-display3)">Display 3</p>
        <p style="font: var(--jb-headline)">Headline</p>
        <p style="font: var(--jb-title)">Title</p>
        <p style="font: var(--jb-label-large)">Label Large</p>
        <p style="font: var(--jb-body-large)">Body Large - 正文内容示例</p>
        <p style="font: var(--jb-body-medium)">Body Medium - 正文内容示例</p>
        <p style="font: var(--jb-body-small)">Body Small - 正文内容示例</p>
      </div>
    </section>

    <section>
      <h2>圆角系统</h2>
      <div class="radius-grid">
        {#each ['none', 'sm', 'base', 'md', 'lg', 'xl', '2xl', '3xl', 'full'] as radius}
          <div class="radius-item">
            <span class="radius-box" style="border-radius: var(--jb-radius-{radius})"></span>
            <span class="radius-label">{radius}</span>
          </div>
        {/each}
      </div>
    </section>

    <section>
      <h2>阴影系统</h2>
      <div class="shadow-grid">
        {#each ['xs', 'sm', 'base', 'md', 'lg', 'xl'] as shadow}
          <div class="shadow-item" style="box-shadow: var(--jb-shadow-{shadow})">
            {shadow}
          </div>
        {/each}
      </div>
    </section>

    <section>
      <h2>交互状态</h2>
      <div class="state-sample">
        <button class="demo-button">默认按钮</button>
        <button class="demo-button" data-jb-hover>悬停效果</button>
        <button class="demo-button" data-jb-focus>聚焦效果</button>
        <button class="demo-button" disabled>禁用状态</button>
      </div>
    </section>
  </main>
</div>

<style>
  .demo {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--jb-spacing-8);
  }

  header {
    margin-bottom: var(--jb-spacing-10);
    text-align: center;
  }

  header h1 {
    font: var(--jb-display2);
    color: var(--jb-on-background);
    margin-bottom: var(--jb-spacing-2);
  }

  header p {
    font: var(--jb-body-large);
    color: var(--jb-on-surface-variant);
    margin-bottom: var(--jb-spacing-4);
  }

  header button {
    padding: var(--jb-spacing-2) var(--jb-spacing-4);
    background: var(--jb-primary);
    color: var(--jb-on-primary);
    border-radius: var(--jb-radius-button);
    transition: all var(--jb-transition-normal);
  }

  header button:hover {
    background: var(--jb-primary-container);
    color: var(--jb-on-primary-container);
  }

  section {
    margin-bottom: var(--jb-space-10);
  }

  section h2 {
    font: var(--jb-headline);
    color: var(--jb-on-background);
    margin-bottom: var(--jb-spacing-4);
    padding-bottom: var(--jb-spacing-2);
    border-bottom: 1px solid var(--jb-outline-variant);
  }

  .color-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: var(--jb-spacing-3);
  }

  .color-item {
    padding: var(--jb-spacing-4);
    border-radius: var(--jb-radius-md);
    text-align: center;
    font-weight: var(--jb-font-weight-medium);
  }

  .size-grid {
    display: flex;
    flex-wrap: wrap;
    gap: var(--jb-spacing-4);
    align-items: flex-end;
  }

  .size-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--jb-spacing-1);
  }

  .size-box {
    background: var(--jb-primary);
    min-width: 8px;
    min-height: 8px;
  }

  .size-label {
    font-size: var(--jb-font-size-caption1);
    color: var(--jb-on-surface-variant);
  }

  .type-sample {
    display: flex;
    flex-direction: column;
    gap: var(--jb-spacing-2);
  }

  .type-sample p {
    color: var(--jb-on-background);
  }

  .radius-grid {
    display: flex;
    flex-wrap: wrap;
    gap: var(--jb-spacing-4);
    align-items: center;
  }

  .radius-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--jb-spacing-1);
  }

  .radius-box {
    width: 60px;
    height: 60px;
    background: var(--jb-primary);
  }

  .radius-label {
    font-size: var(--jb-font-size-caption1);
    color: var(--jb-on-surface-variant);
  }

  .shadow-grid {
    display: flex;
    flex-wrap: wrap;
    gap: var(--jb-spacing-4);
  }

  .shadow-item {
    width: 120px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--jb-surface);
    color: var(--jb-on-surface);
    border-radius: var(--jb-radius-md);
    font-size: var(--jb-font-size-caption1);
  }

  .state-sample {
    display: flex;
    flex-wrap: wrap;
    gap: var(--jb-spacing-3);
  }

  .demo-button {
    padding: var(--jb-spacing-2) var(--jb-spacing-4);
    background: var(--jb-primary);
    color: var(--jb-on-primary);
    border-radius: var(--jb-radius-button);
    transition: all var(--jb-transition-normal);
  }

  .demo-button:hover {
    opacity: var(--jb-hover-opacity);
  }

  .demo-button[data-jb-focus]:focus-visible {
    outline: var(--jb-focus-visible-outline);
    outline-offset: var(--jb-focus-visible-offset);
  }

  .demo-button:disabled {
    opacity: var(--jb-disabled-opacity);
    cursor: not-allowed;
  }
</style>
