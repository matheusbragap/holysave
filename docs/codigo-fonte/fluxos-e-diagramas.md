# Fluxos e diagramas

Diagramas conceiticos (Mermaid). Util para onboarding e revisoes de fluxo sem abrir todas as ferramentas.

---

## Arranque da aplicacao (desktop)

```mermaid
sequenceDiagram
  participant Main as main.rs
  participant Lib as lib.rs
  participant WebView as Frontend SvelteKit
  Main->>Lib: holysave_lib::run()
  Lib->>WebView: instancia WebView2 + SPA
  Note over WebView: SSR desativado (+layout.ts)
```

- `main.rs` apenas delega para a biblioteca (sem logica extra).
- `lib.rs` regista comandos IPC e plugins Tauri (`tauri_plugin_opener`).

---

## Carregar biblioteca na pagina inicial

```mermaid
sequenceDiagram
  participant PG as routes/+page.svelte
  participant ST as lib/services/steam.ts
  participant CMD as Rust scan_steam_games
  participant SC as services/steam/*

  PG->>ST: scanSteamGames()
  ST->>CMD: invoke("scan_steam_games")
  CMD->>SC: scan_steam_games()
  SC-->>CMD: Vec<Game>
  CMD-->>ST: JSON
  ST-->>PG: Game[]
  PG->>PG: loadCoverUrls(Steam apenas)
```

1. Ao montar, `loadGames()` chama Rust.
2. Rust le registo Steam, `libraryfolders.vdf` e cada `appmanifest_*.acf`.
3. A UI opcionalmente pede imagens SteamGridDB (outro invoke).

---

## Capas SteamGridDB

```mermaid
flowchart LR
  PG[+page filtros Steam ids]
  TS[steamgriddb.ts]
  R[get_steamgriddb_covers]
  HTTP[reqwest HTTPS]
  PG --> TS --> R --> HTTP
  HTTP -.-> PG
```

- Requer variavel de ambiente `STEAMGRIDDB_API_KEY` no **processo** do binario Rust.
- Se a chave faltar ou a rede falhar, a UI deve degradar (`coverUrls = {}`), como em `loadCoverUrls`.

---

## Destino de backup (localStorage + padrão do SO)

```mermaid
stateDiagram-v2
  [*] --> Idle: página carrega
  Idle --> Editing: clicar lápis
  Editing --> Committed: Enter / ✓ (path ou vazio→padrão)
  Editing --> Idle: Escape / clicar fora (reverte draft)
```

- **Confirmado** (`backupPathCommitted`) persiste em `localStorage` (`holysave.backupDestination`).
- **Rascunho** pode divergir; backup global e botoes relacionados ficam inconsistentes até confirmar (`backupPathDirty`).

Implementacao paso-a-paso: ver [frontend/rotas.md](./frontend/rotas.md).

---

## Modal de detalhe do jogo

```mermaid
flowchart TB
  GL[GameList]
  GM[GameDetailModal]
  M1[mock-game-status]
  M2[mock-save-tree]
  TAURI[game-tauri.ts invokes]

  GL -->|game selecionado| GM
  GM --> M1
  GM --> M2
  GM --> TAURI
```

- Parte do conteudo e **ilusorio** (`mock-*`) para UX enquanto o pipeline real de saves nao liga aos dados Rust.
- Acoes reais (`open_folder`, `delete_save`) passam por `game-tauri.ts`.

Detalhes: [frontend/lib/features/game-library.md](./frontend/lib/features/game-library.md).

---

## SPA + Tauri (SvelteKit)

```mermaid
flowchart LR
  subgraph Build ["npm run build"]
    SK[SvelteKit adapter-static]
  end
  subgraph Run ["Cargo / Tauri"]
    WV[WebView2 assets]
  end
  SK --> WV
```

- Configuracao: `routes/+layout.ts` exporta `ssr = false` (sem Node no servidor; so WebView).
