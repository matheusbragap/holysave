<script lang="ts">
  import { onMount } from "svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import GameList from "$lib/components/GameList.svelte";
  import StatusBanner from "$lib/components/StatusBanner.svelte";
  import NavigationRail from "$lib/components/NavigationRail.svelte";
  import { scanSteamGames } from "$lib/services/steam";
  import type { Game } from "$lib/types/game";

  let games = $state<Game[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  async function loadGames() {
    isLoading = true;
    error = null;

    try {
      games = await scanSteamGames();
    } catch (err) {
      error = err instanceof Error ? err.message : "Erro ao carregar jogos.";
      games = [];
    } finally {
      isLoading = false;
    }
  }

  onMount(loadGames);
</script>

<main class="shell">
  <div class="topbar">
    <PageHeader
      currentRoute="Jogos"
      gameCount={games.length}
      {isLoading}
      onRefresh={loadGames}
    />
  </div>

  <div class="rail-wrap">
    <NavigationRail />
  </div>

  <section class="content">
    {#if isLoading}
      <StatusBanner>Carregando...</StatusBanner>
    {:else if error}
      <StatusBanner tone="error">{error}</StatusBanner>
    {:else}
      <GameList {games} />
    {/if}
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
