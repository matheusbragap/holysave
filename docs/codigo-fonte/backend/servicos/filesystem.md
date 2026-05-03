# Servico filesystem (`services/filesystem.rs`)

Operacções de alto nivel que **delegam ao SO**:

---

## `open_folder(path: &str) -> Result<(), String>`

1. Normaliza tipo `Path`.
2. Garante **existência** antes de abrir (`Err` caso contrário).
3. Despacha comando por SO:

| SO | Binario |
| -- | ------- |
| Windows | `explorer` |
| macOS | `open` |
| Linux | `xdg-open` |

Usa apenas `spawn` (fire-and-forget) — nao espera processo terminar.

Erros são convertidos em `String` curta para surfaced na UI quando `invoke` propaga falha.

---

## `delete_path(path: &str) -> Result<(), String>`

1. Resolve existência.
2. Se directorio ⇒ `remove_dir_all` (recursivo).
3. Se ficheiro simples ⇒ `remove_file`.

**Atencao seguranca UX**: comandos destructive (`delete_save`) sempre devem ser precedidos por confirmacao forte no frontend porque aqui não ha lixeira/recuperação.

---

## Ligacao comandos ↔ UI

Comando `delete_save`:

```rust
pub fn delete_save(game: Game) -> Result<(), String> {
    let save_path = game.save_path.ok_or("Este jogo não tem caminho de save detectado")?;
    filesystem::delete_path(&save_path)
}
```

Se `save_path` vier `None` ( estado actual pos-scanner antes de probing ), o comando devolve erro textual – o TS captura como `invoke` falha (`game-tauri` retorna boolean false).
