# Backend (`src-tauri/src/`)

Codigo Rust embebido pela shell Tauri. Organizacao:

```text
src-tauri/src/
  main.rs          # ponto binario minimal
  lib.rs           # run() Builder + registos
  commands.rs      # comandosIPC publicos
  models.rs / models/game.rs
  services.rs + services/**/*.rs
```

## Documentos nesta pasta

| Ficheiro | Conteudo |
| -------- | -------- |
| [entrypoint-e-comandos.md](./entrypoint-e-comandos.md) | `main.rs`, `lib.rs`, `commands.rs`, `services.rs` aggregator |
| [modelos.md](./modelos.md) | Struct `Game` e enums relacionados |
| [servicos/steam.md](./servicos/steam.md) | Registo Steam, VDF e scanner |
| [servicos/filesystem.md](./servicos/filesystem.md) | Abrir explorer, apagar arvore |
| [servicos/steamgriddb.md](./servicos/steamgriddb.md) | Cliente REST capas |

## Dependencias Rust relevantes (`Cargo.toml`)

- `dirs` → `document_dir()` para backups default.
- `winreg` → leitura de `InstallPath` Steam (Windows).
- `serde`/`serde_json` → serializacao comandos ↔ WebView.

Ver [`../../ponte-tauri-invoke.md`](../ponte-tauri-invoke.md) para cruzamento com o frontend.
