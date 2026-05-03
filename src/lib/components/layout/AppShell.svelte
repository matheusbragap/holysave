<script lang="ts">
  /** Encaixe desktop: rail + topbar (`docs/codigo-fonte/frontend/lib/componentes/layout.md`). */

  import NavigationRail from "$lib/components/layout/NavigationRail.svelte";
</script>

<main class="shell">
  <div class="topbar">
    <slot name="topbar" />
  </div>

  <div class="rail-wrap">
    <NavigationRail />
  </div>

  <section class="content">
    <slot />
  </section>
</main>

<style>
  .shell {
    --topbar-height: 86px;
    min-height: 100vh;
    display: grid;
    grid-template-columns: 84px minmax(0, 1fr);
    grid-template-rows: var(--topbar-height) 1fr;
    background: var(--bg);
  }

  .topbar {
    grid-column: 1 / -1;
    grid-row: 1;
    position: sticky;
    top: 0;
    z-index: 3;
    height: var(--topbar-height);
    padding: 12px 32px;
    background: var(--rail);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
  }

  .rail-wrap {
    grid-column: 1;
    grid-row: 2;
    position: sticky;
    top: var(--topbar-height);
    height: calc(100vh - var(--topbar-height));
    align-self: start;
  }

  .content {
    grid-column: 2;
    grid-row: 2;
    --content-padding: 32px;
    padding: 24px var(--content-padding) 80px;
  }

  @media (max-width: 720px) {
    .shell {
      --topbar-height: 94px;
      grid-template-columns: 64px minmax(0, 1fr);
    }

    .topbar {
      padding: 16px 18px;
    }

    .content {
      --content-padding: 18px;
      padding: 20px var(--content-padding) 80px;
    }
  }
</style>
