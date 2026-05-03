<script lang="ts">
  /** Configuracoes: destino do backup. */
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AppShell from "$lib/components/layout/AppShell.svelte";
  import PageHeader from "$lib/components/layout/PageHeader.svelte";
  import { getDefaultBackupDirectory } from "$lib/services/backup-path";

  const BACKUP_DEST_STORAGE_KEY = "holysave.backupDestination";

  /** Valor atual no campo (pode divergir do confirmado). */
  let backupPathDraft = $state("");
  /** Destino efetivo para backup; só muda ao confirmar. */
  let backupPathCommitted = $state("");
  /** Exemplo real com o utilizador atual (Rust). */
  let backupPathPlaceholder = $state(
    String.raw`C:\Users\...\Documents\HolySave`
  );

  /** Só permite escrever após clicar no ícone de editar (o campo não abre edição só com foco/clique). */
  let backupPathEditing = $state(false);
  let backupPathInputRef = $state<HTMLInputElement | undefined>();
  /** Campo + ícones de editar / confirmar (cliques fora disso cancelam a edição). */
  let backupPathShellRef = $state<HTMLDivElement | undefined>();

  let backupPathDirty = $derived(
    backupPathDraft.trim() !== backupPathCommitted.trim()
  );

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
    void initBackupDestination();
  });

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
</script>

<svelte:window onpointerdown={onDocumentPointerDown} />

<AppShell>
  <div slot="topbar">
    <PageHeader currentRoute="Configuracoes" />
  </div>

  <section class="settings-shell">
    <div class="settings-card">
      <div class="card-header">
        <div>
          <span class="card-kicker">Backups</span>
          <h2>Destino do backup</h2>
          <p>
            Defina a pasta principal para armazenar todos os saves. Essa pasta sera usada
            como base para a nuvem e restauracoes.
          </p>
        </div>
        <span class="card-chip">Preferencia global</span>
      </div>

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
        </div>
        <div class="path-note">
          Use o lapis para editar. Enter confirma, Esc descarta.
        </div>
      </div>

      <div class="settings-actions">
        <button
          class="ghost-btn"
          type="button"
          onclick={handleOpenBackupFolder}
          disabled={!canOpenBackupPath}
        >
          Abrir pasta
        </button>
        <button
          class="primary-btn"
          type="button"
          onclick={applyBackupDestinationAndLeaveEditMode}
          disabled={!canApplyBackupPath}
        >
          Salvar destino
        </button>
      </div>
    </div>
  </section>
</AppShell>

<style>
  .settings-shell {
    display: grid;
    gap: 18px;
  }

  .settings-card {
    padding: 18px;
    border-radius: 18px;
    border: 1px solid rgba(226, 215, 195, 0.85);
    background:
      radial-gradient(circle at 10% 10%, rgba(255, 236, 198, 0.55), transparent 55%),
      linear-gradient(140deg, #fff9f0, #ffffff);
    box-shadow: 0 16px 26px rgba(36, 24, 12, 0.08);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
    margin-bottom: 16px;
  }

  .card-kicker {
    display: inline-block;
    font-size: 0.62rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: var(--muted);
  }

  .card-header h2 {
    margin: 6px 0 6px;
    font-family: "Fraunces", "Playfair Display", "Georgia", serif;
    font-size: 1.2rem;
    color: var(--ink);
  }

  .card-header p {
    margin: 0;
    color: var(--muted);
    max-width: 56ch;
  }

  .card-chip {
    align-self: flex-start;
    padding: 6px 12px;
    border-radius: 999px;
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    background: rgba(255, 236, 198, 0.6);
    color: rgba(120, 92, 54, 0.9);
    border: 1px solid rgba(224, 122, 63, 0.3);
  }

  .path-stack {
    display: grid;
    gap: 6px;
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
    padding: 8px 12px;
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

  .path-note {
    font-size: 0.7rem;
    color: var(--muted);
  }

  .settings-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 16px;
  }

  .ghost-btn {
    border-radius: 999px;
    border: 1px dashed rgba(191, 171, 140, 0.7);
    background: rgba(255, 255, 255, 0.75);
    color: var(--muted);
    padding: 8px 14px;
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

  .primary-btn {
    border-radius: 999px;
    border: none;
    padding: 8px 18px;
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #fff;
    background: linear-gradient(135deg, var(--accent), var(--accent-strong));
    cursor: pointer;
    box-shadow: 0 12px 22px rgba(207, 90, 29, 0.25);
    transition: transform 0.15s ease, box-shadow 0.2s ease;
  }

  .primary-btn:disabled,
  .ghost-btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
    box-shadow: none;
  }

  .primary-btn:not(:disabled):hover {
    transform: translateY(-1px);
  }

  @media (max-width: 720px) {
    .card-header {
      flex-direction: column;
      align-items: flex-start;
    }

    .card-chip {
      align-self: flex-start;
    }
  }
</style>
