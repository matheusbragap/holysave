# Componentes UI (`src/lib/components/ui/`)

Atomicos partilhaveis **sem dominio especifico**.

---

## `StatusBanner.svelte`

**Papel**: bloco destacado (<slot /> default) para avisos, placeholders ou texto explicativo.

**Uso** (excerpt):

```svelte
<StatusBanner>
  Pagina em construcao; em breve opcoes reais aqui.
</StatusBanner>
```

**Estilo**: deve consumir apenas tokens de `:root`; se precisares de variantes (“aviso”, “erro”), considera nova prop opcional `--banner-tone` no futuro e documenta em [estilo-app-css.md](../../estilo-app-css.md).
