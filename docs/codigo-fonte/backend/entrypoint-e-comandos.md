# Entrypoint, biblioteca e comandos

---

## `main.rs`

Snippet conceptual:

```rust
fn main() {
    holysave_lib::run()
}
```

- `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`: suprime consola CMD em release Windows.
- Nao contenha negocio — mantem compilacao rápida e separacao clara.

---

## `lib.rs`

Monta o **`tauri::Builder`**:

1. Plugins (`tauri_plugin_opener`).
2. `invoke_handler!` agrupando todas as funcoes públicas marcadas em `commands.rs`:

```rust
tauri::generate_handler![
    commands::get_steam_path,
    commands::scan_steam_games,
    commands::get_steamgriddb_covers,
    commands::get_default_backup_directory,
    commands::open_folder,
    commands::delete_save
]
```

3. Arranque com `generate_context!()`.

Ao adicionar comando novo:

1. Implementar helper em `services/…`.
2. Encapsular camada limpa numa funcao marcada por `#[tauri::command]` em `commands.rs`.
3. Inserir símbolo no array macro em `lib.rs`.
4. Atualizar [../ponte-tauri-invoke.md](../ponte-tauri-invoke.md).

---

## `commands.rs`

Cada comando documenta semanticamente antes da funcao (** /// ** doc-comments).

Grupos actuais:

| Grupo | Comandos |
| ----- | -------- |
| Steam | `get_steam_path`, `scan_steam_games` |
| FS / backup | `get_default_backup_directory`, `open_folder`, `delete_save` |
| Remix | `get_steamgriddb_covers` async |

Todos retornam tipos serde-friendly para espelharmos no frontend.

Erros são `Result<_, String>` onde precisamos de mensagens legivel no UI.

---

## `services.rs`

```rust
pub mod steam;
pub mod filesystem;
pub mod steamgriddb;
```

Unico papel: expor subsistemas. Logica volumosa vive sempre num submodulo `services/*/`.
