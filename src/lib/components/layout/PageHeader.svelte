<script lang="ts">
  /** Título/rota atual na topbar (`currentRoute`). */
  let { currentRoute = "Jogos" }: { currentRoute?: string } = $props();
</script>

<header class="header">
  <div class="title">
    <span class="eyebrow">{currentRoute}</span>
    <h1>Holy Save</h1>
  </div>
</header>

<style>
  .header {
    width: 100%;
    margin: 0;
    padding: 0;
    background: transparent;
    border: none;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 18px;
    flex-wrap: wrap;
  }

  .title {
    display: grid;
    gap: 2px;
  }

  .eyebrow {
    text-transform: uppercase;
    letter-spacing: 0.2em;
    font-size: 0.68rem;
    color: var(--muted);
  }

  h1 {
    margin: 0;
    font-size: clamp(1.8rem, 3vw, 2.6rem);
  }
</style>
