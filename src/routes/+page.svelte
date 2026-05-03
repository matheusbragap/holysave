<script lang="ts">
  /**
   * Página principal: biblioteca Steam, capas e filtros.
   * Processo: docs/codigo-fonte/frontend/rotas.md
   */
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import AppShell from "$lib/components/layout/AppShell.svelte";
  import PageHeader from "$lib/components/layout/PageHeader.svelte";
  import StatusBanner from "$lib/components/ui/StatusBanner.svelte";
  import GameList from "$lib/features/game-library/GameList.svelte";
  import { scanSteamGames } from "$lib/services/steam";
  import { getSteamGridCovers } from "$lib/services/steamgriddb";
  import type { Game } from "$lib/types/game";

  const currentRoute = $derived(
    $page.url.pathname.startsWith("/configuracoes") ? "Configuracoes" : "Biblioteca"
  );

  let games = $state<Game[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let viewMode = $state<"grid" | "list">("grid");
  let coverUrls = $state<Record<string, string>>({});
  async function loadGames() {
    isLoading = true;
    error = null;

    try {
      games = await scanSteamGames();
      void loadCoverUrls(games);
    } catch (err) {
      error = err instanceof Error ? err.message : "Erro ao carregar jogos.";
      games = [];
      coverUrls = {};
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    void loadGames();
  });

  async function loadCoverUrls(nextGames: Game[]) {
    const steamIds = nextGames
      .filter((game) => game.platform === "Steam")
      .map((game) => game.id);

    if (steamIds.length === 0) {
      coverUrls = {};
      return;
    }

    try {
      coverUrls = await getSteamGridCovers(steamIds);
    } catch {
      coverUrls = {};
    }
  }

</script>

<AppShell>
  <div slot="topbar">
    <PageHeader currentRoute={currentRoute} />
  </div>

  <div class="floating-bar">
    <div class="bar-row">
      <div class="quick-search">
        <label for="quick-search" class="sr-only">Pesquisa rápida</label>
        <div class="search-shell">
          <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <path
              d="M11 19a8 8 0 100-16 8 8 0 000 16zm7 2l-3.5-3.5"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
            />
          </svg>
          <input
            id="quick-search"
            type="search"
            placeholder="Nome, app id, pasta, backup..."
            aria-label="Pesquisar jogos"
          />
          <span class="search-hint">/ para focar</span>
        </div>
      </div>

      <div class="bar-actions">
        <details class="filter-dropdown">
          <summary class="filter-toggle" aria-label="Abrir filtros">
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M3 5h18l-6.5 7.2V19l-5 0v-6.8L3 5z" />
            </svg>
            <span class="filter-toggle-label">Filtros</span>
            <span class="filter-toggle-badge">0</span>
          </summary>

          <div class="filter-panel" aria-label="Filtros da biblioteca">
            <div class="filter-head">
              <div>
                <span class="filter-title">Filtros da biblioteca</span>
                <span class="filter-sub">Refine, ordene e encontre jogos em segundos.</span>
              </div>
              <div class="filter-actions">
                <button type="button" class="ghost-btn">Limpar tudo</button>
                <button type="button" class="ghost-btn">Salvar preset</button>
              </div>
            </div>

            <div class="filter-grid">
              <div class="filter-card">
                <label>Ordenar por</label>
                <div class="toggle-grid" role="group" aria-label="Ordenacao">
                  <button type="button" class="toggle-pill is-selected" aria-pressed="true">
                    <span class="toggle-label">Nome</span>
                    <span class="toggle-pair" aria-hidden="true">
                      <span class="toggle-option is-on">A-Z</span>
                      <span class="toggle-sep">/</span>
                      <span class="toggle-option">Z-A</span>
                    </span>
                  </button>
                  <button type="button" class="toggle-pill" aria-pressed="false">
                    <span class="toggle-label">Backup</span>
                    <span class="toggle-pair" aria-hidden="true">
                      <span class="toggle-option is-on">Recente</span>
                      <span class="toggle-sep">/</span>
                      <span class="toggle-option">Antigo</span>
                    </span>
                  </button>
                  <button type="button" class="toggle-pill" aria-pressed="false">
                    <span class="toggle-label">Save</span>
                    <span class="toggle-pair" aria-hidden="true">
                      <span class="toggle-option is-on">Maior</span>
                      <span class="toggle-sep">/</span>
                      <span class="toggle-option">Menor</span>
                    </span>
                  </button>
                  <button type="button" class="toggle-pill" aria-pressed="false">
                    <span class="toggle-label">Jogo</span>
                    <span class="toggle-pair" aria-hidden="true">
                      <span class="toggle-option is-on">Maior</span>
                      <span class="toggle-sep">/</span>
                      <span class="toggle-option">Menor</span>
                    </span>
                  </button>
                </div>
                <div class="toggle-hint">So um filtro ativo por vez. Clique para alternar.</div>
              </div>

              <div class="filter-card">
                <label>Backup</label>
                <div class="pill-row">
                  <button type="button" class="pill is-selected" aria-pressed="true">
                    Todos os jogos
                  </button>
                  <button type="button" class="pill">Apenas com backup</button>
                  <button type="button" class="pill">Ignorados</button>
                </div>
              </div>

              <div class="filter-card">
                <label>Tipo</label>
                <div class="pill-row">
                  <button type="button" class="pill is-selected" aria-pressed="true">
                    Jogos e ferramentas
                  </button>
                  <button type="button" class="pill">Somente jogos</button>
                  <button type="button" class="pill">Somente ferramentas</button>
                </div>
              </div>

              <div class="filter-card">
                <label>Plataforma</label>
                <div class="pill-row">
                  <button type="button" class="pill is-selected" aria-pressed="true">Steam</button>
                  <button type="button" class="pill">GOG</button>
                  <button type="button" class="pill">Epic Games</button>
                  <button type="button" class="pill">Manual</button>
                </div>
              </div>

              <div class="filter-card">
                <label>Status de backup</label>
                <div class="pill-row">
                  <button type="button" class="pill pill--good">Synced</button>
                  <button type="button" class="pill pill--warn">Pending</button>
                  <button type="button" class="pill pill--info">Uploading</button>
                  <button type="button" class="pill pill--danger">Error</button>
                </div>
              </div>
            </div>
          </div>
        </details>

        <button
          class="icon-btn view-btn"
          type="button"
          onclick={() => (viewMode = viewMode === "grid" ? "list" : "grid")}
          aria-pressed={viewMode === "grid"}
          title={viewMode === "grid" ? "Mudar para lista" : "Mudar para grade"}
          aria-label={viewMode === "grid" ? "Mudar para lista" : "Mudar para grade"}
        >
          {#if viewMode === "grid"}
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M4 6h2v2H4V6zm4 0h12v2H8V6zM4 11h2v2H4v-2zm4 0h12v2H8v-2zM4 16h2v2H4v-2zm4 0h12v2H8v-2z" />
            </svg>
          {:else}
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M4 4h7v7H4V4zm9 0h7v7h-7V4zM4 13h7v7H4v-7zm9 0h7v7h-7v-7z" />
            </svg>
          {/if}
        </button>
      </div>
    </div>

    <div class="bar-count">
      <span class="count-number">{isLoading ? "..." : games.length}</span>
      <span class="count-label">jogos</span>
    </div>
  </div>

  {#if isLoading}
    <StatusBanner>Carregando...</StatusBanner>
  {:else if error}
    <StatusBanner tone="error">{error}</StatusBanner>
  {:else}
    <GameList {games} viewMode={viewMode} coverUrls={coverUrls} />
  {/if}
</AppShell>

<style>
  .floating-bar {
    position: sticky;
    top: calc(var(--topbar-height) + 16px);
    z-index: 2;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
    padding: 10px 14px;
    margin-bottom: 20px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border);
    background: linear-gradient(135deg, var(--panel), var(--panel-strong));
    box-shadow: var(--shadow);
  }

  .bar-row {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 8px;
  }

  .bar-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    margin-left: auto;
  }

  .bar-count {
    display: flex;
    gap: 6px;
    align-items: baseline;
  }

  .count-number {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--accent-strong);
  }

  .count-label {
    font-size: 0.58rem;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.16em;
  }

  .quick-search {
    flex: 1;
    min-width: 280px;
    max-width: 100%;
  }

  .quick-search label {
    display: none;
  }

  .filter-dropdown {
    position: relative;
  }

  .filter-toggle {
    list-style: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    border-radius: 999px;
    border: 1px solid var(--border);
    padding: 6px 12px;
    background: #fff;
    color: var(--muted);
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    cursor: pointer;
    transition: border-color 0.2s ease, color 0.2s ease, box-shadow 0.2s ease;
  }

  .filter-toggle::-webkit-details-marker {
    display: none;
  }

  .filter-toggle svg {
    width: 16px;
    height: 16px;
  }

  .filter-toggle-label {
    font-weight: 600;
  }

  .filter-toggle-badge {
    padding: 2px 7px;
    border-radius: 999px;
    font-size: 0.58rem;
    background: rgba(224, 122, 63, 0.15);
    color: var(--accent-strong);
  }

  .filter-dropdown[open] .filter-toggle {
    border-color: rgba(224, 122, 63, 0.5);
    color: var(--accent-strong);
    box-shadow: 0 10px 18px rgba(224, 122, 63, 0.15);
    background: #fff7ea;
  }

  .filter-panel {
    position: absolute;
    top: calc(100% + 12px);
    right: 0;
    width: min(760px, 92vw);
    margin-top: 0;
    padding: 14px;
    border-radius: 18px;
    border: 1px solid rgba(226, 215, 195, 0.85);
    background:
      radial-gradient(circle at 10% 10%, rgba(255, 236, 198, 0.55), transparent 55%),
      radial-gradient(circle at 90% 0%, rgba(255, 250, 234, 0.8), transparent 60%),
      linear-gradient(140deg, #fff9f0, #ffffff);
    box-shadow: 0 18px 30px rgba(36, 24, 12, 0.08);
    z-index: 3;
  }

  .filter-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 12px;
  }

  .filter-title {
    display: block;
    font-family: "Fraunces", "Playfair Display", "Georgia", serif;
    font-size: 1rem;
    font-weight: 600;
    color: var(--ink);
  }

  .filter-sub {
    display: block;
    margin-top: 2px;
    font-size: 0.72rem;
    color: var(--muted);
    letter-spacing: 0.04em;
  }

  .filter-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .ghost-btn {
    border-radius: 999px;
    border: 1px dashed rgba(191, 171, 140, 0.7);
    background: rgba(255, 255, 255, 0.75);
    color: var(--muted);
    padding: 6px 12px;
    font-size: 0.7rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    cursor: pointer;
    transition: border-color 0.2s ease, color 0.2s ease;
  }

  .ghost-btn:hover {
    border-color: rgba(224, 122, 63, 0.5);
    color: var(--accent-strong);
  }

  .filter-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 12px;
  }

  .filter-card {
    border-radius: 14px;
    border: 1px solid rgba(228, 217, 198, 0.85);
    background: rgba(255, 255, 255, 0.7);
    padding: 12px;
  }

  .filter-card label {
    font-size: 0.62rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: rgba(90, 78, 62, 0.7);
    display: block;
    margin-bottom: 8px;
  }

  .quick-search {
    flex: 1;
    min-width: 280px;
    max-width: 100%;
  }

  .quick-search label {
    display: none;
  }

  .search-shell {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    border-radius: 12px;
    border: 1px solid rgba(224, 122, 63, 0.35);
    background: #fff;
    box-shadow: inset 0 0 0 1px rgba(255, 244, 224, 0.7);
  }

  .search-shell svg {
    width: 18px;
    height: 18px;
    color: rgba(144, 122, 92, 0.9);
  }

  .search-shell input {
    border: none;
    background: transparent;
    font: inherit;
    color: var(--ink);
    min-width: 0;
  }

  .search-shell input:focus {
    outline: none;
  }

  .search-hint {
    font-size: 0.65rem;
    color: rgba(128, 110, 88, 0.7);
    padding: 2px 8px;
    border-radius: 999px;
    background: rgba(255, 244, 220, 0.7);
    border: 1px solid rgba(224, 122, 63, 0.18);
  }

  .pill-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }

  .pill {
    border-radius: 999px;
    border: 1px solid rgba(210, 195, 170, 0.9);
    padding: 6px 12px;
    font-size: 0.72rem;
    background: rgba(255, 255, 255, 0.9);
    color: rgba(88, 74, 55, 0.85);
    cursor: pointer;
    transition: transform 0.15s ease, border-color 0.15s ease, color 0.15s ease;
  }

  .pill:hover {
    transform: translateY(-1px);
    border-color: rgba(224, 122, 63, 0.45);
    color: var(--accent-strong);
  }

  .pill.is-selected {
    border-color: rgba(224, 122, 63, 0.55);
    background: linear-gradient(135deg, #fff2dc, #fffdf7);
    color: var(--accent-strong);
    box-shadow: 0 6px 16px rgba(224, 122, 63, 0.12);
  }

  .toggle-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 10px;
  }

  .toggle-pill {
    border-radius: 14px;
    border: 1px solid rgba(210, 195, 170, 0.9);
    background: rgba(255, 255, 255, 0.92);
    padding: 10px 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    cursor: pointer;
    color: rgba(88, 74, 55, 0.85);
    transition: transform 0.15s ease, border-color 0.15s ease, color 0.15s ease;
  }

  .toggle-pill:hover {
    transform: translateY(-1px);
    border-color: rgba(224, 122, 63, 0.45);
    color: var(--accent-strong);
  }

  .toggle-pill.is-selected {
    border-color: rgba(224, 122, 63, 0.55);
    background: linear-gradient(135deg, #fff2dc, #fffdf7);
    color: var(--accent-strong);
    box-shadow: 0 8px 16px rgba(224, 122, 63, 0.15);
  }

  .toggle-label {
    font-size: 0.66rem;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    font-weight: 600;
  }

  .toggle-pair {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 2px 6px;
    border-radius: 999px;
    border: 1px solid rgba(226, 215, 195, 0.7);
    background: rgba(255, 255, 255, 0.75);
    font-size: 0.64rem;
  }

  .toggle-option {
    color: rgba(90, 78, 62, 0.7);
    font-weight: 500;
  }

  .toggle-option.is-on {
    color: var(--accent-strong);
    font-weight: 700;
  }

  .toggle-sep {
    color: rgba(130, 112, 86, 0.55);
  }

  .toggle-hint {
    margin-top: 8px;
    font-size: 0.68rem;
    color: var(--muted);
  }


  .pill--good {
    border-color: rgba(70, 160, 110, 0.5);
    color: #2e7d5b;
  }

  .pill--warn {
    border-color: rgba(222, 160, 76, 0.6);
    color: #a4652c;
  }

  .pill--info {
    border-color: rgba(92, 148, 214, 0.6);
    color: #2f5f9f;
  }

  .pill--danger {
    border-color: rgba(214, 92, 92, 0.6);
    color: #b43a3a;
  }

  .icon-btn {
    border: none;
    border-radius: 12px;
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    cursor: pointer;
  }

  .icon-btn svg {
    width: 18px;
    height: 18px;
  }

  .view-btn {
    border: 1px solid var(--border);
    background: #fff;
    color: var(--muted);
    transition: background 150ms, color 150ms, border-color 150ms;
  }

  .view-btn:hover {
    background: var(--panel-strong);
    color: var(--accent-strong);
    border-color: rgba(224, 122, 63, 0.4);
  }

  @media (max-width: 720px) {
    .floating-bar {
      position: static;
      align-items: flex-start;
    }

    .bar-row {
      flex-direction: column;
      align-items: stretch;
      width: 100%;
      gap: 8px;
    }

    .quick-search {
      min-width: auto;
    }

    .bar-actions {
      width: 100%;
      justify-content: flex-start;
      gap: 8px;
    }

    .filter-dropdown {
      flex: 1;
    }

    .filter-toggle {
      width: 100%;
      justify-content: space-between;
    }

    .filter-panel {
      position: static;
      width: 100%;
      margin-top: 12px;
    }

    .filter-head {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
