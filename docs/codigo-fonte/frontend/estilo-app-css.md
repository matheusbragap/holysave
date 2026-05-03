# Estilo global (`src/app.css`)

## Camadas

| Bloco | Funcao |
| ----- | ------ |
| `:root` | Tokens de cor, sombra, radios, fonte base. Todas as páginas derivam destes nomes (`--bg`, `--ink`, `--accent`, …). |
| `*, *::before, *::after` | `box-sizing: border-box` + **selecção de texto global desactivada** (`user-select: none`, `-webkit-touch-callout: none`). Evita highlight acidental em desktop app. |
| `html, body` | `margin: 0` — evita faixa branca no WebView. |

## Exemplo: consumir tokens num componente Svelte

```svelte
<style>
  .card {
    background: var(--panel);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow);
    color: var(--ink);
  }
</style>
```

## Visual e escalabilidade

Ao introduzires novos accents (ex.: “sucesso”), **preferir** novo `--token` em `:root` a cores hard-coded no componente. Documenta novos tokens numa linha em `CHANGELOG` ou neste ficheiro.

## Implicacao UX

Com `user-select: none` global, texto de entrada no WebView também segue esta regra; se precisares de copiar texto noutros ecrãs futuros, isso será uma decisão de produto (excepção pontual ou `user-select` explicativo nessa zona apenas).
