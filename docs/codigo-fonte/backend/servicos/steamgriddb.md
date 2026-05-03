# SteamGridDB (`services/steamgriddb.rs`)

Cliente HTTP minimalista usando `reqwest` **async**.

---

## Fluxo por AppID

Para cada Steam `app_id`:

1. Construir URL grids:

```
GET https://www.steamgriddb.com/api/v2/grids/steam/{app_id}
    ?dimensions=600x900&types=static&nsfw=false&humor=false&epilepsy=false&limit=1
```

2. Header `Authorization: Bearer {API_KEY}`.
3. Se status HTTP != OK → ignorar esse id (silent skip).
4. Parse JSON dinamico (`serde_json::Value`) → campo `data[0].url` se existir.
5. Inserir em `HashMap<app_id, url>`.

---

## Configuracao

**Obrigatorio**: variavel de ambiente `STEAMGRIDDB_API_KEY` visivel ao processo Rust.

Exemplo arranque dev (pseudo):

```powershell
$env:STEAMGRIDDB_API_KEY="sua-chave"; cargo tauri dev
```

Sem chave → `Err("STEAMGRIDDB_API_KEY nao configurada")` quando o comando async executa (UI pode engolir e mostrar capas omitidas).

---

## Performance / limites

Implementacao atual faz **requests sequenciais** num loop (`for app_id`). Para bibliotecas muito grandes, considerar um pool limitado ou batch API se disponível.

---

## Exemplo resultado

```rust
Ok(HashMap::from([
 ("620".into(), "https://cdn.grid…/grid_123.webp".into()),
]))
```
