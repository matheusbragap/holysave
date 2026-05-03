# Indice por arquivo

Checklist de **todos os ficheiros de codigo** em `src/` e `src-tauri/src/`. Cada linha liga a uma explicacao mais longa no documento indicado.

---

## Frontend (`src/`)

| Arquivo | Responsabilidade |
| ------- | -------------------------------- |
| [`app.css`](../../src/app.css) | Tokens globais (`:root`), `box-sizing`, selecao global desativada. Doc: [frontend/estilo-app-css.md](./frontend/estilo-app-css.md) |
| [`routes/+layout.ts`](../../src/routes/+layout.ts) | `ssr = false` — modo SPA para Tauri. Doc: [frontend/rotas.md](./frontend/rotas.md) |
| [`routes/+layout.svelte`](../../src/routes/+layout.svelte) | Import global de `app.css` e `<slot />`. Doc: [frontend/rotas.md](./frontend/rotas.md) |
| [`routes/+page.svelte`](../../src/routes/+page.svelte) | Pagina principal: barra flutuante backup, lista de jogos, carregamento Steam/SteamGridDB. Doc: [frontend/rotas.md](./frontend/rotas.md) |
| [`routes/configuracoes/+page.svelte`](../../src/routes/configuracoes/+page.svelte) | Placeholder de configuracoes. Doc: [frontend/rotas.md](./frontend/rotas.md) |
| [`lib/types/game.ts`](../../src/lib/types/game.ts) | Tipo `Game` (UI). Doc: [frontend/lib/types-e-utils.md](./frontend/lib/types-e-utils.md) |
| [`lib/utils/format-bytes.ts`](../../src/lib/utils/format-bytes.ts) | Formatar tamanhos (B…TB). Doc: [frontend/lib/types-e-utils.md](./frontend/lib/types-e-utils.md) |
| [`lib/services/steam.ts`](../../src/lib/services/steam.ts) | `invoke` `scan_steam_games`. Doc: [frontend/lib/servicos.md](./frontend/lib/servicos.md) |
| [`lib/services/backup-path.ts`](../../src/lib/services/backup-path.ts) | `invoke` destino padrao. Doc: [frontend/lib/servicos.md](./frontend/lib/servicos.md) |
| [`lib/services/steamgriddb.ts`](../../src/lib/services/steamgriddb.ts) | `invoke` capas. Doc: [frontend/lib/servicos.md](./frontend/lib/servicos.md) |
| [`lib/components/layout/AppShell.svelte`](../../src/lib/components/layout/AppShell.svelte) | Shell: topbar slot, rail, conteudo. Doc: [frontend/lib/componentes/layout.md](./frontend/lib/componentes/layout.md) |
| [`lib/components/layout/PageHeader.svelte`](../../src/lib/components/layout/PageHeader.svelte) | Cabecalho com rota actual. Doc: [frontend/lib/componentes/layout.md](./frontend/lib/componentes/layout.md) |
| [`lib/components/layout/NavigationRail.svelte`](../../src/lib/components/layout/NavigationRail.svelte) | Navegacao `/` e `/configuracoes`. Doc: [frontend/lib/componentes/layout.md](./frontend/lib/componentes/layout.md) |
| [`lib/components/ui/StatusBanner.svelte`](../../src/lib/components/ui/StatusBanner.svelte) | Banner informativo slot. Doc: [frontend/lib/componentes/ui.md](./frontend/lib/componentes/ui.md) |
| [`lib/features/game-library/GameList.svelte`](../../src/lib/features/game-library/GameList.svelte) | Grelha/lista de jogos. Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |
| [`lib/features/game-library/GameDetailModal.svelte`](../../src/lib/features/game-library/GameDetailModal.svelte) | Modal detalhe + mocks + accoes. Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |
| [`lib/features/game-library/FolderTree.svelte`](../../src/lib/features/game-library/FolderTree.svelte) | Arvore recursiva UI (mock saves). Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |
| [`lib/features/game-library/game-tauri.ts`](../../src/lib/features/game-library/game-tauri.ts) | Wrapper `open_folder` / `delete_save` com boolean resultado. Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |
| [`lib/features/game-library/mock-game-status.ts`](../../src/lib/features/game-library/mock-game-status.ts) | Dados ilustrativos de estado/health. Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |
| [`lib/features/game-library/mock-save-tree.ts`](../../src/lib/features/game-library/mock-save-tree.ts) | Arvore ficticia de pastas. Doc: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md) |

---

## Backend Tauri (`src-tauri/src/`)

| Arquivo | Responsabilidade |
| ------- | -------------------------------- |
| [`main.rs`](../../src-tauri/src/main.rs) | `main` delega `holysave_lib::run()`. Doc: [backend/entrypoint-e-comandos.md](./backend/entrypoint-e-comandos.md) |
| [`lib.rs`](../../src-tauri/src/lib.rs) | Builder Tauri + lista de handlers. Doc: [backend/entrypoint-e-comandos.md](./backend/entrypoint-e-comandos.md) |
| [`commands.rs`](../../src-tauri/src/commands.rs) | Todos os `#[tauri::command]` publicos. Doc: [backend/entrypoint-e-comandos.md](./backend/entrypoint-e-comandos.md) |
| [`models.rs`](../../src-tauri/src/models.rs) | `pub mod game`. Doc: [backend/modelos.md](./backend/modelos.md) |
| [`models/game.rs`](../../src-tauri/src/models/game.rs) | Struct `Game`, `Platform`, `GameStatus`. Doc: [backend/modelos.md](./backend/modelos.md) |
| [`services.rs`](../../src-tauri/src/services.rs) | `pub mod steam, filesystem, steamgriddb`. Doc: [backend/README.md](./backend/README.md) |
| [`services/filesystem.rs`](../../src-tauri/src/services/filesystem.rs) | `open_folder`, `delete_path`. Doc: [backend/servicos/filesystem.md](./backend/servicos/filesystem.md) |
| [`services/steam/mod.rs`](../../src-tauri/src/services/steam/mod.rs) | Re-export publico do modulo Steam. Doc: [backend/servicos/steam.md](./backend/servicos/steam.md) |
| [`services/steam/path.rs`](../../src-tauri/src/services/steam/path.rs) | Registo Win + `libraryfolders.vdf`. Doc: [backend/servicos/steam.md](./backend/servicos/steam.md) |
| [`services/steam/scanner.rs`](../../src-tauri/src/services/steam/scanner.rs) | Leitura de `appmanifest_*.acf`. Doc: [backend/servicos/steam.md](./backend/servicos/steam.md) |
| [`services/steam/parser.rs`](../../src-tauri/src/services/steam/parser.rs) | Parser minimalista VDF (linhas entre aspas). Doc: [backend/servicos/steam.md](./backend/servicos/steam.md) |
| [`services/steamgriddb.rs`](../../src-tauri/src/services/steamgriddb.rs) | Cliente API capas. Doc: [backend/servicos/steamgriddb.md](./backend/servicos/steamgriddb.md) |
