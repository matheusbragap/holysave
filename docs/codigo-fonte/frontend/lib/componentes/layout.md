# Componentes de layout (`src/lib/components/layout/`)

Componentes estruturais reutilizados por qualquer pagina dentro do `AppShell`.

---

## `AppShell.svelte`

**Slots**:

| Slot | Posicao CSS | Uso típico |
| ---- | ----------- | ----------- |
| (default) | Area `content` | Corpo principal (lista, formulários). |
| `topbar` | Barra sticky superior | Titulo/rota (`PageHeader`). |

**Layout CSS**: CSS Grid (`grid-template-columns: 84px 1fr`, `rail` + `conteudo`).

Diagrama rápido:

```
+------------------+------------------------+
|     topbar (slot spanning full width)   |
+--------+--------------------------------+
| Rail   |  content (slot default)        |
| (nav)  |                                |
+--------+--------------------------------+
```

---

## `PageHeader.svelte`

**Prop**: `currentRoute` — texto legivel (“Biblioteca”, “Configuracoes”). Quem monta deve derivar ou fixar segundo a rota ativa (`+page.svelte` usa `$derived` sobre `$page.url`).

---

## `NavigationRail.svelte`

Links:

- **`/`**: biblioteca principal (icon rectangles).
- **`/configuracoes`**: rota de configuracoes (gear).

Usa **`$page` store** (`$page.url.pathname`) para estilizar `aria-current="page"` em quem esta activo.

**Acessibilidade**: `aria-label` em cada `a`; `aria-hidden` nos SVG decorative.
