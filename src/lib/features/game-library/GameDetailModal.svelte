<script lang="ts">
  /**
   * Detalhes de um jogo: dados ilustrativos (mocks) + ações reais (pastas/delete).
   * Doc: docs/codigo-fonte/frontend/lib/features/game-library.md
   */
  import type { Game } from "$lib/types/game";
  import { formatBytes } from "$lib/utils/format-bytes";
  import FolderTree from "./FolderTree.svelte";
  import {
    getMockAutoBackupActive,
    getMockGameHealth,
    getMockGameStatusLabel,
    getMockLastBackupAt,
  } from "./mock-game-status";
  import { buildMockSaveTree } from "./mock-save-tree";
  import { deleteGameSave, openFolderPath } from "./game-tauri";

  let {
    game,
    coverUrl = null,
    onClose,
  }: {
    game: Game;
    coverUrl?: string | null;
    onClose: () => void;
  } = $props();

  let dialogEl = $state<HTMLDialogElement | null>(null);

  const mockRoot = $derived(buildMockSaveTree(game));

  let mockHealth = $derived(getMockGameHealth(game));
  let mockStatusLabel = $derived(getMockGameStatusLabel(game));
  let mockLastBackup = $derived(getMockLastBackupAt(game));
  let mockAutoBackupOn = $derived(getMockAutoBackupActive(game));

  function savePathLabel(full: string | undefined | null, max = 96): string {
    if (!full?.trim()) return "(não disponível)";
    if (full.length <= max) return full;
    return `${full.slice(0, max - 1)}…`;
  }

  let canOpenSave = $derived(Boolean(game.save_path?.trim()));

  /** Só aparece como dica ao passar o mouse na chip da loja (plataforma). */
  let storeInstallHint = $derived(game.install_dir?.trim() || undefined);

  $effect(() => {
    const el = dialogEl;
    if (!el) return;
    queueMicrotask(() => {
      try {
        el.showModal();
      } catch {
        /* já em uso */
      }
    });
    return () => {
      try {
        el.close();
      } catch {
        /* ignorar */
      }
    };
  });

  function handleDialogClick(e: MouseEvent) {
    if ((e.target as Node) === dialogEl) dialogEl?.close();
  }

  async function onOpenSave() {
    await openFolderPath(game.save_path);
  }

  async function onOpenGame() {
    await openFolderPath(game.install_dir);
  }

  async function onRemoveSave() {
    if (!confirm(`Remover o save de ${game.name}? Esta ação não pode ser desfeita.`)) return;
    await deleteGameSave(game);
  }

  async function onUninstall() {
    if (!confirm(`Desinstalar ${game.name} pelo sistema? (integração futura)`)) return;
  }

  async function onBackupSave() {
    await Promise.resolve();
  }

  async function onRestoreSave() {
    await Promise.resolve();
  }
</script>

<dialog
  bind:this={dialogEl}
  class="game-detail-dialog"
  aria-labelledby="game-detail-title"
  onclick={handleDialogClick}
  onclose={onClose}
>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="modal-shell" onclick={(e) => e.stopPropagation()}>
    <div class="banner" class:banner--fallback={!coverUrl}>
      {#if coverUrl}
        <img class="banner-bg" src={coverUrl} alt="" />
      {/if}
      <div class="banner-scrim"></div>
      <div class="banner-body">
        <h2 id="game-detail-title">{game.name}</h2>
        <div class="banner-chips">
          {#if storeInstallHint}
            <span class="chip chip-store" title={storeInstallHint}>{game.platform}</span>
          {:else}
            <span class="chip">{game.platform}</span>
          {/if}
          {#if formatBytes(game.size_bytes)}
            <span class="chip chip-muted">{formatBytes(game.size_bytes)}</span>
          {/if}
          <span class="chip chip-muted">ID {game.id}</span>
        </div>
      </div>
    </div>

    <button type="button" class="close-x" aria-label="Fechar" onclick={() => dialogEl?.close()}>
      <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
        <path
          d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
        />
      </svg>
    </button>

    <div class="modal-body">
      <section class="stack">
        <dl class="save-path-block">
          <dt>Pasta do save</dt>
          <dd class="save-path-row">
            <span class="mono save-path-text">{savePathLabel(game.save_path)}</span>
            <button
              type="button"
              class="save-open-folder"
              aria-label="Abrir pasta do save"
              disabled={!canOpenSave}
              title={canOpenSave ? "Abrir pasta do save no Explorer" : "Caminho do save não disponível"}
              onclick={onOpenSave}
            >
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path
                  d="M17 3H5a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2V7l-4-4zm-5 16a3 3 0 110-6 3 3 0 010 6zm3-10H5V5h10v4z"
                />
              </svg>
            </button>
          </dd>
        </dl>
      </section>

      <section class="stack">
        <dl class="status-grid">
          <div>
            <dt>Último backup</dt>
            <dd class="status-strong">{mockLastBackup}</dd>
          </div>
          <div>
            <dt>Backup automático</dt>
            <dd>
              {#if mockAutoBackupOn}
                <span class="auto-flag on"><span class="dot"></span> Ativo</span>
              {:else}
                <span class="auto-flag off"><span class="dot"></span> Inativo</span>
              {/if}
            </dd>
          </div>
          <div>
            <dt>Status do jogo</dt>
            <dd>
              <span class="pill pill--{mockHealth}" title={mockStatusLabel}>{mockStatusLabel}</span>
            </dd>
          </div>
        </dl>
      </section>

      <section class="stack">
        <h3>Estrutura do save (ilustrativa)</h3>
        <div class="tree-box">
          <FolderTree node={mockRoot} />
        </div>
      </section>

      <section class="stack actions-section">
        <h3>Ações rápidas</h3>

        <div class="toolbar">
          <div class="toolbar-icons">
            <button
              type="button"
              class="icon-act"
              onclick={onOpenGame}
              title="Abrir pasta do jogo"
              aria-label="Abrir pasta do jogo"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path
                  d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"
                />
              </svg>
            </button>
            <button
              type="button"
              class="icon-act danger"
              onclick={onRemoveSave}
              title="Remover save"
              aria-label="Remover save"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path
                  d="M9 3v1H4v2h1v13a2 2 0 002 2h10a2 2 0 002-2V6h1V4h-5V3H9zm0 5h2v9H9V8zm4 0h2v9h-2V8z"
                />
              </svg>
            </button>
            <button
              type="button"
              class="icon-act muted-strong"
              onclick={onUninstall}
              title="Desinstalar jogo"
              aria-label="Desinstalar jogo"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path
                  d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"
                />
              </svg>
            </button>
          </div>

          <div class="toolbar-labeled">
            <button type="button" class="btn-labeled-primary" onclick={onBackupSave}>
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M12 4l-4 4h3v8h2V8h3l-4-4zm-7 14h14v2H5v-2z" />
              </svg>
              <span>Fazer backup do save</span>
            </button>
            <button type="button" class="btn-labeled-neutral" onclick={onRestoreSave}>
              <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path d="M12 16l4-4h-3V4h-2v8H8l4 4zm-7 6h14v-2H5v2z" />
              </svg>
              <span>Restaurar save</span>
            </button>
          </div>
        </div>
      </section>
    </div>
  </div>
</dialog>

<style>
  .game-detail-dialog {
    position: fixed;
    inset: 0;
    z-index: 200;
    width: 100%;
    max-width: none;
    height: 100%;
    max-height: none;
    margin: 0;
    padding: max(12px, 2.5vh) 12px;
    border: none;
    border-radius: 0;
    background: transparent;
    box-sizing: border-box;
    display: grid;
    place-items: center;
    overflow: hidden;
  }

  .game-detail-dialog::backdrop {
    background: rgba(22, 16, 8, 0.48);
    backdrop-filter: blur(3px);
  }

  .modal-shell {
    position: relative;
    display: flex;
    flex-direction: column;
    background: var(--panel);
    width: min(860px, 100%);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border);
    box-shadow: var(--shadow);
    overflow: hidden;
    max-height: min(920px, calc(100vh - 24px));
  }

  .banner {
    position: relative;
    flex-shrink: 0;
    min-height: 108px;
    background-color: var(--panel-strong);
    overflow: hidden;
  }

  .banner-bg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
  }

  .banner--fallback {
    background-image:
      radial-gradient(circle at 20% 30%, rgba(224, 122, 63, 0.35), transparent 45%),
      radial-gradient(circle at 80% 50%, rgba(207, 90, 29, 0.2), transparent 50%);
  }

  .banner-scrim {
    position: absolute;
    inset: 0;
    background: linear-gradient(180deg, rgba(20, 14, 8, 0.15) 0%, rgba(20, 14, 8, 0.78) 100%);
  }

  .banner-body {
    position: relative;
    z-index: 1;
    padding: 52px 18px 12px;
    color: #fff;
    text-shadow: 0 2px 10px rgba(0, 0, 0, 0.55);
  }

  .banner-body h2 {
    margin: 0 0 6px;
    font-size: clamp(1.2rem, 2.4vw, 1.55rem);
    font-weight: 600;
    line-height: 1.2;
  }

  .banner-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    padding: 4px 10px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.35);
    font-size: 0.68rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.09em;
  }

  .chip-muted {
    background: rgba(0, 0, 0, 0.25);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .chip-store {
    cursor: help;
  }

  .close-x {
    position: absolute;
    top: 12px;
    right: 12px;
    z-index: 2;
    width: 40px;
    height: 40px;
    border: none;
    border-radius: 12px;
    background: rgba(0, 0, 0, 0.4);
    color: #fff;
    cursor: pointer;
    display: grid;
    place-items: center;
    transition: background 0.15s;
  }

  .close-x:hover {
    background: rgba(0, 0, 0, 0.55);
  }

  .close-x svg {
    width: 20px;
    height: 20px;
  }

  .modal-body {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 12px 18px 14px;
    display: grid;
    gap: 12px;
    scrollbar-gutter: stable;
  }

  .stack h3 {
    margin: 0 0 6px;
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--muted);
    font-weight: 600;
  }

  .save-path-block {
    margin: 0;
    border-radius: var(--radius-md);
    border: 1px solid var(--border);
    background: var(--panel-strong);
    padding: 10px 12px;
  }

  .save-path-block dt {
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: var(--muted);
    margin: 0 0 6px;
    font-weight: 600;
  }

  .save-path-row {
    margin: 0;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
  }

  .save-path-text {
    flex: 1 1 12rem;
    min-width: 0;
    font-size: 0.82rem;
    color: var(--ink);
    line-height: 1.45;
  }

  .save-open-folder {
    flex-shrink: 0;
    width: 36px;
    height: 36px;
    display: grid;
    place-items: center;
    border-radius: var(--radius-sm, 10px);
    border: 1px solid rgba(224, 122, 63, 0.45);
    background: linear-gradient(145deg, #fff4e8, #fff);
    color: var(--accent-strong);
    cursor: pointer;
    transition:
      transform 0.12s ease,
      background 0.15s,
      opacity 0.15s,
      border-color 0.15s;
  }

  .save-open-folder svg {
    width: 18px;
    height: 18px;
  }

  .save-open-folder:hover:not(:disabled) {
    transform: translateY(-1px);
    background: linear-gradient(145deg, #ffeada, #fff4e8);
  }

  .save-open-folder:disabled {
    opacity: 0.45;
    cursor: not-allowed;
    transform: none;
    border-color: var(--border);
    background: var(--panel);
    color: var(--muted);
  }

  .mono {
    font-family: ui-monospace, "Cascadia Code", monospace;
    font-size: 0.8rem;
    word-break: break-all;
  }

  .hint {
    margin: 0 0 6px;
    font-size: 0.75rem;
    color: var(--muted);
  }

  .tree-box {
    border-radius: var(--radius-md);
    border: 1px solid var(--border);
    background: var(--panel-strong);
    padding: 8px 10px;
    max-height: 132px;
    overflow: auto;
  }

  .status-grid {
    margin: 0;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(176px, 1fr));
    gap: 8px;
  }

  .status-grid dt {
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--muted);
    margin-bottom: 4px;
    font-weight: 600;
  }

  .status-grid dd {
    margin: 0;
  }

  .status-strong {
    font-size: 0.86rem;
    font-weight: 600;
    color: var(--ink);
  }

  .pill {
    display: inline-block;
    max-width: 100%;
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 0.76rem;
    font-weight: 600;
    line-height: 1.35;
  }

  .pill--ok {
    background: rgba(34, 197, 94, 0.12);
    border: 1px solid rgba(34, 197, 94, 0.35);
    color: #15803d;
  }

  .pill--attention {
    background: rgba(251, 191, 36, 0.15);
    border: 1px solid rgba(245, 158, 11, 0.45);
    color: #b45309;
  }

  .pill--idle {
    background: var(--panel-strong);
    border: 1px solid var(--border);
    color: var(--muted);
  }

  .auto-flag {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--ink);
  }

  .auto-flag .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .auto-flag.on .dot {
    background: #22c55e;
    box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.25);
  }

  .auto-flag.off .dot {
    background: var(--muted);
    opacity: 0.85;
    box-shadow: none;
  }

  .toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px 12px;
  }

  .toolbar-icons {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
  }

  .icon-act {
    width: 38px;
    height: 38px;
    flex-shrink: 0;
    display: grid;
    place-items: center;
    border-radius: var(--radius-sm, 12px);
    border: 1px solid var(--border);
    background: #fff;
    color: var(--muted);
    cursor: pointer;
    transition:
      transform 0.12s ease,
      background 0.15s,
      border-color 0.15s,
      color 0.15s;
  }

  .icon-act svg {
    width: 18px;
    height: 18px;
  }

  .icon-act:hover {
    transform: translateY(-1px);
    background: var(--panel-strong);
    color: var(--ink);
    border-color: rgba(224, 122, 63, 0.35);
  }

  .icon-act.danger {
    border-color: rgba(220, 38, 38, 0.35);
    color: #b91c1c;
    background: #fff8f8;
  }

  .icon-act.danger:hover {
    background: #fdeaea;
    border-color: rgba(220, 38, 38, 0.5);
  }

  .icon-act.muted-strong {
    border-color: transparent;
    background: transparent;
    color: var(--muted);
  }

  .icon-act.muted-strong:hover {
    color: var(--ink);
    border-color: var(--border);
    background: var(--panel-strong);
  }

  .toolbar-labeled {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    flex: 1;
    justify-content: flex-start;
    min-width: min(280px, 100%);
  }

  .btn-labeled-primary,
  .btn-labeled-neutral {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: var(--radius-sm, 10px);
    font-size: 0.8rem;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    border: 1px solid transparent;
    transition:
      transform 0.12s ease,
      box-shadow 0.15s,
      border-color 0.15s,
      background 0.15s;
    flex: 1 1 148px;
    justify-content: center;
    min-width: 0;
    text-align: center;
    white-space: normal;
  }

  .btn-labeled-primary svg,
  .btn-labeled-neutral svg {
    width: 18px;
    height: 18px;
    flex-shrink: 0;
  }

  .btn-labeled-primary {
    background: linear-gradient(135deg, var(--accent), var(--accent-strong));
    color: #fff;
    box-shadow: 0 6px 14px rgba(207, 90, 29, 0.18);
    border-color: transparent;
  }

  .btn-labeled-primary:hover {
    transform: translateY(-1px);
  }

  .btn-labeled-neutral {
    background: #fff;
    color: var(--ink);
    border-color: var(--border);
  }

  .btn-labeled-neutral:hover {
    transform: translateY(-1px);
    border-color: rgba(224, 122, 63, 0.4);
    background: var(--panel-strong);
  }

  @media (max-width: 560px) {
    .banner-body {
      padding-top: 50px;
    }

    .toolbar {
      flex-direction: column;
      align-items: stretch;
    }

    .toolbar-labeled {
      flex-direction: column;
    }

    .btn-labeled-primary,
    .btn-labeled-neutral {
      flex-basis: auto;
      width: 100%;
    }
  }
</style>
