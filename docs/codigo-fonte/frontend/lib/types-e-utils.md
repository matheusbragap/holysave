# Tipos e utilitarios (`src/lib/types`, `src/lib/utils`)

---

## `types/game.ts`

Define o tipo **TypeScript** usado na maior parte dos componentes Svelte quando se fala num `Game`:

```typescript
export type Game = {
  id: string;
  name: string;
  platform: string;
  install_dir?: string;
  size_bytes?: number;
  save_path?: string | null;
};
```

### Comparacao com o modelo Rust

O backend usa `crate::models::game::Game` com campos extras (`Platform` enum tipado, `GameStatus`, `last_backup`, `checksum`, `is_ignored`, …).

- Em **runtime**, o JSON de `invoke` pode trazer todos esses campos.
- O tipo TS atual e um **subconjunto** util para compilacao onde so precisamos de subcampos frequentes (`id`, `name`, paths).
- Para contrato forte, evoluir para tipo gerado (tauri-codegen) ou alinhar interface TypeScript aos campos reais conforme [`../../models/game.md`](../../../models/game.md).

**Exemplo** de objeto minimo esperado pela UI:

```json
{
  "id": "620",
  "name": "Portal 2",
  "platform": "Steam",
  "install_dir": "C:\\\\Program Files (x86)\\\\Steam\\\\steamapps\\\\common\\\\Portal 2",
  "size_bytes": 12345678,
  "save_path": null
}
```

(Outros campos podem existir; o cliente ignora-os se nao tipados.)

---

## `utils/format-bytes.ts`

Função pura `formatBytes(value?: number): string | null`.

| Entrada | Saida exemplo |
| ------- | ------------- |
| `undefined` / `0` | `null` |
| `1500` | `1.5 KB` |
| `1073741824` | `1.0 GB` |

```typescript
import { formatBytes } from "$lib/utils/format-bytes";

const label = formatBytes(game.size_bytes) ?? "—";
```
