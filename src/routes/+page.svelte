<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import AppShell from "$lib/components/AppShell.svelte";
  import PageHeader from "$lib/components/PageHeader.svelte";
  import GameList from "$lib/components/GameList.svelte";
  import StatusBanner from "$lib/components/StatusBanner.svelte";
  import { scanSteamGames } from "$lib/services/steam";
  import type { Game } from "$lib/types/game";

  const currentRoute = $derived(
    $page.url.pathname.startsWith("/configuracoes") ? "Configuracoes" : "Biblioteca"
  );

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

<AppShell>
  <div slot="topbar">
    <PageHeader
      currentRoute={currentRoute}
      gameCount={games.length}
      {isLoading}
      onRefresh={loadGames}
    />
  </div>

  {#if isLoading}
    <StatusBanner>Carregando...</StatusBanner>
  {:else if error}
    <StatusBanner tone="error">{error}</StatusBanner>
  {:else}
    <GameList {games} />
  {/if}
</AppShell>
