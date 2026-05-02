<script lang="ts">
  import type { Game } from "$lib/types/game";

  export let games: Game[] = [];

  let activeMenu: string | null = null;

  function toggleMenu(gameId: string) {
    activeMenu = activeMenu === gameId ? null : gameId;
  }

  function closeMenu() {
    activeMenu = null;
  }

  function handleReload(game: Game) {
    console.log('Recarregar', game);
  }

  function handleUninstall(game: Game) {
    console.log('Desinstalar jogo', game);
  }

  function handleOpenGameFolder(game: Game) {
    console.log('Abrir local do arquivo do jogo', game);
  }

  function handleOpenSaveFolder(game: Game) {
    console.log('Abrir local do arquivo do save', game);
  }

  function handleDeleteSave(game: Game) {
    console.log('Apagar save game', game);
  }

  function handleRestoreSave(game: Game) {
    console.log('Restaurar último save da nuvem', game);
  }

  function formatBytes(value?: number) {
    if (!value) return null;
    const units = ["B", "KB", "MB", "GB", "TB"];
    let size = value;
    let unitIndex = 0;
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex += 1;
    }
    return `${size.toFixed(size >= 100 ? 0 : 1)} ${units[unitIndex]}`;
  }
</script>

<svelte:window on:click={closeMenu} />

<div class="game-list">
  {#each games as game}
    <div class="game-card">
      <div class="info">
        <strong>{game.name}</strong>
        <div class="meta">
          <span class="chip">{game.platform}</span>
          {#if formatBytes(game.size_bytes)}
            <span class="size">{formatBytes(game.size_bytes)}</span>
          {/if}
        </div>
      </div>

      <div class="actions">
        <!-- Apagar save -->
        <button
          class="icon-btn danger"
          on:click|stopPropagation={() => handleDeleteSave(game)}
          title="Apagar save game"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d="M9 3v1H4v2h1v13a2 2 0 002 2h10a2 2 0 002-2V6h1V4h-5V3H9zm0 5h2v9H9V8zm4 0h2v9h-2V8z"/>
          </svg>
        </button>

        <!-- Restaurar save -->
        <button
          class="icon-btn restore"
          on:click|stopPropagation={() => handleRestoreSave(game)}
          title="Restaurar último save da nuvem"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d="M12 3v10.2l3.4-3.4 1.4 1.4-5.8 5.8-5.8-5.8 1.4-1.4 3.4 3.4V3h2z"/>
            <path d="M5 19h14v2H5z"/>
          </svg>
        </button>

        <!-- Três pontinhos -->
        <div class="menu-wrapper">
          <button
            class="icon-btn menu-trigger"
            class:active={activeMenu === game.id}
            on:click|stopPropagation={() => toggleMenu(game.id)}
            title="Mais opções"
          >
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <circle cx="12" cy="5" r="2"/>
              <circle cx="12" cy="12" r="2"/>
              <circle cx="12" cy="19" r="2"/>
            </svg>
          </button>

          {#if activeMenu === game.id}
            <div class="dropdown" on:click|stopPropagation>
              <button on:click={() => { handleReload(game); closeMenu(); }}>
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M17.65 6.35A7.958 7.958 0 0012 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0112 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
                Recarregar
              </button>
              <button on:click={() => { handleOpenGameFolder(game); closeMenu(); }}>
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/>
                </svg>
                Abrir pasta do jogo
              </button>
              <button on:click={() => { handleOpenSaveFolder(game); closeMenu(); }}>
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M17 3H5a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2V7l-4-4zm-5 16a3 3 0 110-6 3 3 0 010 6zm3-10H5V5h10v4z"/>
                </svg>
                Abrir pasta do save
              </button>
              <div class="divider"></div>
              <button class="danger" on:click={() => { handleUninstall(game); closeMenu(); }}>
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                </svg>
                Desinstalar jogo
              </button>
            </div>
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <p class="empty">Nenhum jogo encontrado ainda...</p>
  {/each}
</div>

<style>
  .game-list {
    max-width: 980px;
    margin: 0 auto;
    display: grid;
    gap: 12px;
  }

  .game-card {
    display: grid;
    grid-template-columns: 1fr auto;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: 0 12px 20px rgba(22, 16, 6, 0.08);
    animation: fadeUp 250ms ease-out;
  }

  .info {
    display: grid;
    gap: 6px;
  }

  strong {
    font-size: 1.05rem;
  }

  .meta {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--muted);
    font-size: 0.9rem;
  }

  .chip {
    padding: 4px 10px;
    border-radius: 999px;
    background: #fff4de;
    border: 1px solid rgba(224, 122, 63, 0.3);
    color: var(--accent-strong);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.65rem;
  }

  .size {
    color: var(--muted);
    font-size: 0.85rem;
  }

  /* Actions */
  .actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border-radius: var(--radius-sm, 6px);
    border: none;
    background: transparent;
    color: var(--muted);
    cursor: pointer;
    transition: background 150ms, color 150ms;
  }

  .icon-btn svg {
    width: 17px;
    height: 17px;
  }

  .icon-btn:hover {
    background: var(--panel-strong, rgba(0,0,0,0.06));
    color: var(--text);
  }

  .icon-btn.danger:hover {
    background: rgba(220, 38, 38, 0.1);
    color: #dc2626;
  }

  .icon-btn.restore:hover {
    background: rgba(37, 99, 235, 0.1);
    color: #2563eb;
  }

  .icon-btn.menu-trigger.active {
    background: var(--panel-strong, rgba(0,0,0,0.06));
    color: var(--text);
  }

  /* Dropdown */
  .menu-wrapper {
    position: relative;
  }

  .dropdown {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    min-width: 210px;
    background: var(--panel, #fff);
    border: 1px solid var(--border);
    border-radius: var(--radius-md, 10px);
    box-shadow: 0 8px 24px rgba(0,0,0,0.12);
    z-index: 100;
    padding: 4px;
    animation: dropIn 150ms ease-out;
  }

  .dropdown button {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: transparent;
    border-radius: var(--radius-sm, 6px);
    font-size: 0.9rem;
    color: var(--text);
    cursor: pointer;
    text-align: left;
    transition: background 120ms;
  }

  .dropdown button svg {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
    color: var(--muted);
  }

  .dropdown button:hover {
    background: var(--panel-strong, rgba(0,0,0,0.05));
  }

  .dropdown button.danger {
    color: #dc2626;
  }

  .dropdown button.danger svg {
    color: #dc2626;
  }

  .dropdown button.danger:hover {
    background: rgba(220, 38, 38, 0.08);
  }

  .divider {
    height: 1px;
    background: var(--border);
    margin: 4px 0;
  }

  /* Empty */
  .empty {
    max-width: 980px;
    margin: 0 auto;
    padding: 12px 16px;
    border-radius: var(--radius-md);
    background: var(--panel-strong);
    color: var(--muted);
    border: 1px solid var(--border);
  }

  /* Animations */
  @keyframes fadeUp {
    from { opacity: 0; transform: translateY(6px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  @keyframes dropIn {
    from { opacity: 0; transform: translateY(-4px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 640px) {
    .game-card {
      grid-template-columns: 1fr;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .game-card { animation: none; }
    .dropdown  { animation: none; }
  }
</style>