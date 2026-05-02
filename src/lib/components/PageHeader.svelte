<script lang="ts">
  export let gameCount = 0;
  export let currentRoute = "Jogos";
  export let isLoading = false;
  export let showStats = true;
  export let showRefresh = true;
  export let onRefresh: () => void = () => {};
</script>

<header class="header">
  <div class="title">
    <span class="eyebrow">{currentRoute}</span>
    <h1>Holy Save</h1>
  </div>
  {#if showStats || showRefresh}
    <div class="actions">
      {#if showStats}
        <div class="count">
          <span class="count-number">{gameCount}</span>
          <span class="count-label">jogos</span>
        </div>
      {/if}
      {#if showRefresh}
        <button class="refresh" type="button" on:click={onRefresh} disabled={isLoading}>
          {isLoading ? "Atualizando..." : "Atualizar"}
        </button>
      {/if}
    </div>
  {/if}
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
    justify-content: space-between;
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


  .actions {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .count {
    display: grid;
    justify-items: end;
  }

  .count-number {
    font-size: 1.6rem;
    font-weight: 600;
    color: var(--accent-strong);
  }

  .count-label {
    font-size: 0.8rem;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.18em;
  }

  .refresh {
    border: 1px solid rgba(224, 122, 63, 0.35);
    background: #fff4dc;
    color: var(--accent-strong);
    border-radius: 999px;
    padding: 10px 18px;
    font-weight: 600;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    box-shadow: 0 8px 16px rgba(29, 18, 6, 0.12);
  }

  .refresh:disabled {
    cursor: progress;
    opacity: 0.7;
    box-shadow: none;
  }

  @media (max-width: 640px) {
    .header {
      flex-direction: column;
      align-items: flex-start;
    }

    .actions {
      width: 100%;
      justify-content: space-between;
    }
  }
</style>
