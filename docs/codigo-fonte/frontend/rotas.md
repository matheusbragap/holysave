# Rotas (`src/routes/`)

## `+layout.ts`

**Papel**: desactiva SSR — o binario Tauri serve um bundle estatico **sem** servidor Node.

```typescript
export const ssr = false;
```

- Referencia oficial: [SPA SvelteKit](https://svelte.dev/docs/kit/single-page-apps) e [Tauri + SvelteKit](https://v2.tauri.app/start/frontend/sveltekit/).

---

## `+layout.svelte`

**Papel**:

1. Import unico de `../app.css` (tokens e reset global).
2. `<slot />` — conteudo das rotas filhas.

Nao ha logica de navegacao aqui; o layout e puramente estrutural.

---

## `+page.svelte` (Biblioteca)

**Papel**: ecra principal do Holy Save — carregar jogos Steam, capas, configurar destino de backup e hospedar `GameList`.

### Processo (ordem)

1. **`onMount`** dispara em paralelo `loadGames()` e `initBackupDestination()`.
2. **`loadGames`**: `scanSteamGames()` → preenche `games`; em seguida `loadCoverUrls` filtra apenas `platform === "Steam"`.
3. **`initBackupDestination`**: `getDefaultBackupDirectory()` preenche placeholder e merge com `localStorage` (`holysave.backupDestination`).
4. **Barra de backup**:
   - `backupPathDraft` vs `backupPathCommitted` controla `backupPathDirty`.
   - Edicao: campo `readonly` até icon de lapis; foco programatico ao abrir.
   - **Confirmar**: Enter ou botao ✓ chama `confirmBackupDestination()` (vazio → pasta padrao via `backupPathPlaceholder`).
   - **Cancelar**: Escape ou `pointerdown` fora de `backupPathShellRef` repoe `backupPathDraft` ao commit.
5. **Botoes laterais**: `open_folder` sobre rascunho ou fallback padrao; backup agora placeholder (`handleBackupNow`).

### Exemplo derivado (gatilhos UI)

```typescript
const canBackup = $derived(
  !isLoading &&
    !isBackingUp &&
    !backupPathDirty &&
    backupPathCommitted.trim().length > 0 &&
    games.length > 0
);
```

### Ligacao a componentes

- `AppShell` + `PageHeader` para moldura.
- `GameList` recebe lista, erros, modo grelha/lista, `coverUrls`.

---

## `configuracoes/+page.svelte`

**Papel**: placeholder de futuras preferencias (mantem `AppShell` + `StatusBanner` consistente com a biblioteca).

**Extensibilidade**: quando adicionares forms, reutiliza tokens de `app.css` e o padrao slot do `AppShell`.
