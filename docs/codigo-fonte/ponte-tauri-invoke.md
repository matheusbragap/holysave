# Ponte Tauri (`invoke`)

Cada linha relaciona **nome do comando Tauri**, **implementacao Rust** e **consumidor TypeScript**.

| Comando invoke | Ficheiro Rust | Funcao Rust | Frontend (principal) |
| -------------- | ------------- | ----------- | --------------------- |
| `scan_steam_games` | `commands.rs` | `scan_steam_games` | `lib/services/steam.ts` (`scanSteamGames`) |
| `get_default_backup_directory` | `commands.rs` | `get_default_backup_directory` | `lib/services/backup-path.ts` |
| `open_folder` | `commands.rs` | `open_folder` | `routes/+page.svelte`, `game-tauri.ts` |
| `delete_save` | `commands.rs` | `delete_save` | `game-tauri.ts` |
| `get_steamgriddb_covers` | `commands.rs` | `get_steamgriddb_covers` | `lib/services/steamgriddb.ts` |

### Comando registado mas nao usado no frontend atual

| Comando | Nota |
| ------- | ------ |
| `get_steam_path` | Implementado para diagnostico/extension; pode ser chamado desde a UI ou novo codigo quando precisares do caminho bruto Steam. |

### Forma do payload

Por omissão, Tauri passa objetos camelCase aos argumentos em JS e serializa structs/enums serde para JSON.

**Game (Rust)** — estrutura completa descrita em [../models/game.md](../models/game.md).

O tipo `Game` em `src/lib/types/game.ts` e um **subconjunto** comum; o runtime pode incluir mais campos (ex.: `status`, `last_backup`) que o Rust envia — o TypeScript aceita extras em `invoke<Game[]>` mas o contrato estrito pode ser fortalecido com tipos gerados ou zod numa fase seguinte.

### Exemplo minimo `invoke`

```typescript
import { invoke } from "@tauri-apps/api/core";

const games = await invoke<Array<Record<string, unknown>>>("scan_steam_games");
```

```rust
// Em commands.rs o handler ja esta ligado em lib.rs
#[tauri::command]
pub fn scan_steam_games() -> Vec<Game> {
    steam::scan_steam_games()
}
```
