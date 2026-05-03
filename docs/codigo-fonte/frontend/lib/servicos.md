# Servicos (`src/lib/services/`)

Camada **fina**: cada ficheiro mapeia 1–N comandos Tauri através de `invoke`.

## Principio

Manter **`invoke` encapsulado** aqui permite:

1. Mudar payloads num unico lugar.
2. Substituir por mocks nos testes (futuro).
3. Ter documentacao tecnica correlacionavel com Rust ([`../../ponte-tauri-invoke.md`](../../ponte-tauri-invoke.md)).

---

## `steam.ts`

```typescript
import { invoke } from "@tauri-apps/api/core";
import type { Game } from "$lib/types/game";

export async function scanSteamGames(): Promise<Game[]> {
  return invoke<Game[]>("scan_steam_games");
}
```

**Fluxo**: UI chama esta funcao → Rust `scan_steam_games()` retorna todos os manifests lidos das bibliotecas Steam.

---

## `backup-path.ts`

```typescript
export async function getDefaultBackupDirectory(): Promise<string> {
  return invoke<string>("get_default_backup_directory");
}
```

Usado pela pagina inicial para inicializar `backupPathPlaceholder` quando nao há override em `localStorage`.

---

## `steamgriddb.ts`

```typescript
export async function getSteamGridCovers(
  appIds: string[]
): Promise<Record<string, string>> {
  if (appIds.length === 0) return {};
  return invoke<Record<string, string>>("get_steamgriddb_covers", { appIds });
}
```

Argumento **`appIds`**: deve corresponder aos `id` Steam (mesmos strings que aparecem no manifest).

**Ambiente Rust**: obrigatorio `STEAMGRIDDB_API_KEY` no processo; ver backend [servicos/steamgriddb.md](../../../backend/servicos/steamgriddb.md).

---

## Chamada desde a pagina

```typescript
// Exemplo típico
games = await scanSteamGames();

const urls = await getSteamGridCovers(
  games.filter((g) => g.platform === "Steam").map((g) => g.id)
);
```
