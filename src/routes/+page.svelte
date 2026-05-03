<script lang="ts">
  /**
   * Página principal: biblioteca Steam, capas, barra de destino de backup.
   * Processo: docs/codigo-fonte/frontend/rotas.md
   */
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import AppShell from "$lib/components/layout/AppShell.svelte";
  import PageHeader from "$lib/components/layout/PageHeader.svelte";
  import StatusBanner from "$lib/components/ui/StatusBanner.svelte";
  import GameList from "$lib/features/game-library/GameList.svelte";
  import { getDefaultBackupDirectory } from "$lib/services/backup-path";
  import { scanSteamGames } from "$lib/services/steam";
  import { getSteamGridCovers } from "$lib/services/steamgriddb";
  import type { Game } from "$lib/types/game";

  const BACKUP_DEST_STORAGE_KEY = "holysave.backupDestination";

  const currentRoute = $derived(
    $page.url.pathname.startsWith("/configuracoes") ? "Configuracoes" : "Biblioteca"
  );

  let games = $state<Game[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  /** Valor atual no campo (pode divergir do confirmado). */
  let backupPathDraft = $state("");
  /** Destino efectivo para backup aberto pasta; só muda ao confirmar. */
  let backupPathCommitted = $state("");
  /** Exemplo real com o utilizador atual (Rust). */
  let backupPathPlaceholder = $state(
    String.raw`C:\Users\...\Documents\HolySave`
  );
  let isBackingUp = $state(false);

  /** Só permite escrever após clicar no ícone de editar (o campo não abre edição só com foco/clique). */
  let backupPathEditing = $state(false);
  let backupPathInputRef = $state<HTMLInputElement | undefined>();
  /** Campo + ícones de editar / confirmar (cliques fora disso cancelam a edição). */
  let backupPathShellRef = $state<HTMLDivElement | undefined>();

  let backupPathDirty = $derived(
    backupPathDraft.trim() !== backupPathCommitted.trim()
  );
  let viewMode = $state<"grid" | "list">("grid");
  let coverUrls = $state<Record<string, string>>({});

  const canOpenBackupPath = $derived(
    backupPathDraft.trim().length > 0 ||
      backupPathPlaceholder.trim().length > 0
  );
  /** Rascunho preenchido ou vazio mas com pasta padrão resolvível (substitui vazio pela padrão ao confirmar). */
  const canApplyBackupPath = $derived(
    backupPathEditing &&
      backupPathDirty &&
      (backupPathDraft.trim().length > 0 ||
        backupPathPlaceholder.trim().length > 0)
  );
  const canBackup = $derived(
    !isLoading &&
      !isBackingUp &&
      !backupPathDirty &&
      backupPathCommitted.trim().length > 0 &&
      games.length > 0
  );
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

  async function initBackupDestination() {
    try {
      const systemDefault = await getDefaultBackupDirectory();
      backupPathPlaceholder = systemDefault;

      let initial = systemDefault;
      try {
        const stored = localStorage.getItem(BACKUP_DEST_STORAGE_KEY)?.trim();
        if (stored) initial = stored;
      } catch {
        /* private mode / quota */
      }

      backupPathCommitted = initial;
      backupPathDraft = initial;
    } catch {
      backupPathPlaceholder = String.raw`C:\Users\...\Documents\HolySave`;
    }
  }

  function confirmBackupDestination() {
    const trimmed = backupPathDraft.trim();
    const next = trimmed || backupPathPlaceholder.trim();
    if (!next) return;
    backupPathCommitted = next;
    backupPathDraft = next;
    try {
      localStorage.setItem(BACKUP_DEST_STORAGE_KEY, next);
    } catch {
      /* ignorar */
    }
  }

  function startEditingBackupDestination() {
    backupPathEditing = true;
    queueMicrotask(() => backupPathInputRef?.focus());
  }

  function applyBackupDestinationAndLeaveEditMode() {
    const fallback = backupPathPlaceholder.trim();
    if (!backupPathDraft.trim() && !fallback) return;
    confirmBackupDestination();
    backupPathEditing = false;
  }

  /** Reverte para o último diretório confirmado e sai do modo edição. */
  function cancelBackupPathEdit() {
    if (!backupPathEditing) return;
    backupPathDraft = backupPathCommitted;
    backupPathEditing = false;
  }

  function onBackupPathKeydown(e: KeyboardEvent) {
    if (!backupPathEditing) return;
    if (e.key === "Enter" && canApplyBackupPath) {
      e.preventDefault();
      applyBackupDestinationAndLeaveEditMode();
    }
    if (e.key === "Escape") {
      e.preventDefault();
      cancelBackupPathEdit();
    }
  }

  function onDocumentPointerDown(e: PointerEvent) {
    if (!backupPathEditing || !backupPathShellRef) return;
    const t = e.target;
    if (!(t instanceof Node)) return;
    if (backupPathShellRef.contains(t)) return;
    cancelBackupPathEdit();
  }

  onMount(() => {
    void loadGames();
    void initBackupDestination();
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

  async function handleOpenBackupFolder() {
    const trimmed =
      backupPathDraft.trim() || backupPathPlaceholder.trim();
    if (!trimmed) return;

    try {
      await invoke("open_folder", { path: trimmed });
    } catch {
      /* invoke failed */
    }
  }

  async function handleBackupNow() {
    if (!canBackup) return;

    isBackingUp = true;
    try {
      await Promise.resolve();
    } finally {
      isBackingUp = false;
    }
  }
</script>

<svelte:window onpointerdown={onDocumentPointerDown} />

<AppShell>
  <div slot="topbar">
    <PageHeader currentRoute={currentRoute} />
  </div>

  <div class="floating-bar">
    <div class="path-stack">
      <label for="backup-path">Destino do backup</label>
      <div class="path-row">
        <div
          bind:this={backupPathShellRef}
          class="path-input-shell"
          class:path-input-shell--dirty={backupPathDirty}
          class:path-input-shell--editing={backupPathEditing}
        >
          <input
            bind:this={backupPathInputRef}
            id="backup-path"
            type="text"
            class="path-field"
            class:path-field--locked={!backupPathEditing}
            spellcheck="false"
            autocomplete="off"
            readonly={!backupPathEditing}
            tabindex={backupPathEditing ? 0 : -1}
            placeholder={backupPathPlaceholder}
            bind:value={backupPathDraft}
            onkeydown={onBackupPathKeydown}
          />
          {#if !backupPathEditing}
            <button
              type="button"
              class="path-slot path-slot--edit"
              onclick={startEditingBackupDestination}
              title="Editar destino do backup"
              aria-label="Editar destino do backup"
            >
              <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M12 19h9M5.71 13.71l11-11a3 3 0 014.24 4.24l-11 11-6 2 2-6z"
                />
              </svg>
            </button>
          {:else if canApplyBackupPath}
            <button
              type="button"
              class="path-slot path-slot--apply"
              onclick={applyBackupDestinationAndLeaveEditMode}
              title="Confirmar pasta de destino"
              aria-label="Confirmar pasta de destino"
            >
              <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M6 13l4 4L18 9"
                />
              </svg>
            </button>
          {:else}
            <button
              type="button"
              class="path-slot path-slot--idle"
              onclick={() => cancelBackupPathEdit()}
              title="Fechar modo de edição (sem alterações)"
              aria-label="Fechar modo de edição sem alterações"
            >
              <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path
                  stroke="currentColor"
                  stroke-width="1.75"
                  stroke-linecap="round"
                  d="M6 13l4 4L18 9"
                />
              </svg>
            </button>
          {/if}
        </div>
        <div class="button-row">
          <button
            class="icon-btn open-path"
            type="button"
            onclick={handleOpenBackupFolder}
            disabled={!canOpenBackupPath}
            aria-label="Abrir pasta do backup"
            title="Abrir pasta do backup"
          >
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M3 6a2 2 0 012-2h5l2 2h7a2 2 0 012 2v2H3V6z" />
              <path d="M3 10h18l-2 9H5l-2-9z" />
            </svg>
          </button>
          <button
            class="icon-btn backup-btn"
            type="button"
            onclick={handleBackupNow}
            disabled={!canBackup}
            aria-label="Backup agora"
            title={backupPathDirty
              ? "Confirme o destino (✓ ao lado do caminho ou Enter) antes do backup"
              : !backupPathEditing && backupPathDraft.trim().length > 0
                ? "Backup agora"
                : "Configure e confirme o destino do backup primeiro"}
          >
            <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M12 3v10.2l3.4-3.4 1.4 1.4-5.8 5.8-5.8-5.8 1.4-1.4 3.4 3.4V3h2z" />
              <path d="M5 19h14v2H5z" />
            </svg>
          </button>
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
      <div class="count">
        <span class="count-number">{isLoading ? "..." : games.length}</span>
        <span class="count-label">jogos</span>
      </div>
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
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 14px;
    margin-bottom: 20px;
    border-radius: var(--radius-lg);
    border: 1px solid var(--border);
    background: linear-gradient(135deg, var(--panel), var(--panel-strong));
    box-shadow: var(--shadow);
  }

  .path-stack {
    display: grid;
    gap: 6px;
    flex: 1;
    min-width: min(520px, 100%);
  }

  .path-stack label {
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--muted);
  }

  .path-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    align-items: center;
  }

  .path-input-shell {
    flex: 1 1 320px;
    min-width: 220px;
    display: flex;
    align-items: center;
    position: relative;
    border-radius: 10px;
    border: 1px solid var(--border);
    background: #fff;
    transition:
      border-color 0.15s ease,
      box-shadow 0.15s ease;
  }

  .path-input-shell--dirty {
    border-color: rgba(224, 122, 63, 0.55);
    box-shadow: 0 0 0 2px rgba(224, 122, 63, 0.12);
  }

  .path-input-shell--editing:not(.path-input-shell--dirty) {
    border-color: rgba(224, 122, 63, 0.28);
  }

  .path-field {
    flex: 1 1 auto;
    min-width: 0;
    border: none;
    border-radius: 10px;
    padding: 7px 10px;
    padding-right: 40px;
    background: transparent;
    color: var(--ink);
    font: inherit;
  }

  .path-field:focus {
    outline: none;
  }

  .path-field--locked {
    cursor: default;
    color: var(--muted);
    text-overflow: ellipsis;
  }

  .path-field::placeholder {
    color: #9a917f;
    opacity: 1;
  }

  .path-slot {
    position: absolute;
    top: 50%;
    right: 6px;
    transform: translateY(-50%);
    width: 30px;
    height: 30px;
    border: none;
    border-radius: 8px;
    display: grid;
    place-items: center;
    cursor: pointer;
    color: var(--muted);
    background: transparent;
    transition:
      transform 0.12s ease,
      background 0.15s,
      color 0.15s,
      opacity 0.15s;
  }

  .path-slot:hover {
    background: rgba(0, 0, 0, 0.04);
    color: var(--accent-strong);
  }

  .path-slot svg {
    width: 18px;
    height: 18px;
    display: block;
  }

  .path-slot--edit:hover {
    color: var(--ink);
  }

  .path-slot--apply {
    color: rgba(106, 98, 82, 0.85);
    border: 1px solid rgba(226, 215, 195, 0.9);
    background: rgba(255, 255, 255, 0.7);
    transform: translateY(-50%) scale(0.98);
  }

  .path-slot--apply:hover {
    color: var(--ink);
    border-color: rgba(224, 122, 63, 0.35);
    background: rgba(255, 250, 240, 0.95);
  }

  .path-slot--idle {
    color: var(--muted);
    opacity: 0.72;
    border: none;
  }

  .path-slot--idle:hover {
    opacity: 1;
    color: var(--muted);
  }

  .path-slot--idle svg {
    width: 16px;
    height: 16px;
    opacity: 0.82;
  }

  .button-row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-left: auto;
  }

  .count {
    display: flex;
    gap: 8px;
    align-items: baseline;
  }

  .count-number {
    font-size: 1.2rem;
    font-weight: 600;
    color: var(--accent-strong);
  }

  .count-label {
    font-size: 0.65rem;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.18em;
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

  .open-path {
    border: 1px solid rgba(224, 122, 63, 0.35);
    background: #fff4dc;
    color: var(--accent-strong);
  }

  .backup-btn {
    color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent-strong));
    box-shadow: 0 12px 24px rgba(207, 90, 29, 0.25);
    transition: transform 0.15s ease, box-shadow 0.2s ease;
  }

  .icon-btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
    box-shadow: none;
  }

  .backup-btn:not(:disabled):hover {
    transform: translateY(-1px);
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

    .path-stack {
      min-width: 100%;
    }
  }
</style>
