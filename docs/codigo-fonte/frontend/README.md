# Frontend (`src/`)

Estrutura SvelteKit pensada para desktop (Tauri):

- **Rotas**: `src/routes/` — paginas e layouts.
- **Biblioteca partilhada**: `src/lib/` — componentes, features, servicos, tipos.

## Arvore conceptual

```text
src/
  app.css              # estilo global
  routes/
    +layout.ts         # ssr = false
    +layout.svelte     # import app.css
    +page.svelte       # biblioteca + backup bar
    configuracoes/+page.svelte
  lib/
    components/        # layout shell + UI atomica
    features/        # dominios (ex.: biblioteca de jogos)
    services/        # invokes Tauri (finos)
    types/           # contratos TS
    utils/           # funcoes puras
```

## Guias por area

| Doc | Conteudo |
| ----- | -------- |
| [rotas.md](./rotas.md) | Layouts, pagina principal, SPA |
| [estilo-app-css.md](./estilo-app-css.md) | Variaveis CSS e regras globais |
| [lib/types-e-utils.md](./lib/types-e-utils.md) | `game.ts`, `format-bytes.ts` |
| [lib/servicos.md](./lib/servicos.md) | Camada `invoke` |
| [lib/componentes/layout.md](./lib/componentes/layout.md) | AppShell, PageHeader, NavigationRail |
| [lib/componentes/ui.md](./lib/componentes/ui.md) | StatusBanner |
| [lib/features/game-library.md](./lib/features/game-library.md) | Lista, modal, mocks, `game-tauri` |

## Estado (Svelte 5)

A UI usa **`$state` / `$derived` / `$effect`** onde aplicavel (ex.: `+page.svelte`, modais). Ao adicionar novo estado partilhado entre rotas, considera store em ficheiro `.ts` sob `lib/` e documenta no `README` da feature.
