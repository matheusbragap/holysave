# Modelos (`models/`)

---

## `models.rs`

Unica linha de agregação:

```rust
pub mod game;
```

Ajuda ferramentas (rust-analyzer) e mantêm namespace limpo quando surgirem `platform_account`, `manifest`, etc.

---

## `models/game.rs`

Contem **fonte canonical** do domínio aplicacional em Rust.

### Struct `Game`

Campos destacados vs doc de dominio mais alto nivel ([../../models/game.md](../../models/game.md)):

| Campo | Nota tecnica Rust |
| ----- | ------------- |
| `id` | `String` porque manifestos já trazem AppID texto. |
| `platform` | Enum `Platform` (Steam já implementado pelo scanner atual). |
| `install_dir` | Caminho instalacao `steamapps/common/{installdir}`. |
| `save_path` | `None` inicialmente até integrar motor de Ludusavi / probing. |
| `status`, `checksum`, `last_backup`, `is_ignored` | Preparacao para estado real de backup — defaults no scanner inicial. |

### Enums `Platform` e `GameStatus`

Serde serializa enums **externamente** para JSON esperado pela UI como strings (`"Steam"`, `"Pending"`, `"Error"` com campo interno, etc.) conforme configuracao serde default.

#### Exemplo JSON minimo típico (scanner actual)

```json
{
  "id": "4000",
  "name": "Garry's Mod",
  "platform": "Steam",
  "size_bytes": 123456,
  "last_backup": null,
  "install_dir": "…\\\\steamapps\\\\common\\\\GarrysMod",
  "save_path": null,
  "status": "Pending",
  "checksum": null,
  "is_ignored": false
}
```

(Serializacao de `SystemTime` exige formato especifico serde; no futuro poderá ser UNIX ms.)

---

## Divergência com `$lib/types/game`

O cliente TypeScript hoje só declara alguns desses campos; adicioná-los no TS antes de novo UI trabalhar estado real evita mismatches silent.
