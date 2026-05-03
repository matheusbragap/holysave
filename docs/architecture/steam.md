# Steam service

Este documento descreve, em detalhes, como o servico Steam detecta jogos instalados e monta a lista de jogos consumida pelo frontend.

## Objetivo

- Descobrir o caminho raiz da instalacao do Steam no Windows.
- Ler as bibliotecas adicionais (libraryfolders.vdf).
- Varer os manifests appmanifest_*.acf para detectar jogos instalados.
- Montar o modelo `Game` com dados minimos confiaveis.

## Estrutura de pastas e arquivos

```
src-tauri/src/services/steam/
  mod.rs
  path.rs
  parser.rs
  scanner.rs
```

Responsabilidades:

- `mod.rs`: expoe a API publica do modulo.
- `path.rs`: localiza o Steam e as bibliotecas.
- `parser.rs`: parse do VDF e do appmanifest.
- `scanner.rs`: varredura dos manifests e montagem do `Game`.

## Fluxo macro

```
scan_steam_games
  -> get_library_folders
      -> get_steam_path (registry)
      -> parse_libraryfolders (libraryfolders.vdf)
  -> for cada library_root
      -> is_appmanifest
      -> parse_appmanifest
      -> monta Game
```

## mod.rs: API publica

Apenas o que precisa ser consumido fora do modulo fica exposto. Isso evita acoplamento com funcoes internas e reduz superficie de mudanca.

```rust
mod parser;
mod path;
mod scanner;

pub use path::get_steam_path;
pub use scanner::scan_steam_games;
```

## path.rs: localizar Steam e bibliotecas

### get_steam_path

- Le o registro do Windows para encontrar o caminho de instalacao do Steam.
- Usa dois caminhos possiveis do registro para cobrir instalacoes 64-bit e fallback antigo.
- Retorna `Option<PathBuf>` para indicar ausencia quando o Steam nao esta instalado.

```rust
pub fn get_steam_path() -> Option<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey("SOFTWARE\\Valve\\Steam")
        .or_else(|_| hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam"))
        .ok()
        .and_then(|key| {
            let path: String = key.get_value("InstallPath").ok()?;
            Some(PathBuf::from(path))
        })
}
```

### get_library_folders

- Parte do caminho raiz retornado por `get_steam_path`.
- Adiciona o Steam root por default.
- Le `libraryfolders.vdf` e adiciona as bibliotecas adicionais.
- Remove duplicatas via `HashSet` e ordena o resultado para determinismo.

```rust
pub fn get_library_folders() -> Vec<PathBuf> {
    let Some(steam_root) = get_steam_path() else {
        return Vec::new();
    };

    let mut paths = HashSet::new();
    paths.insert(steam_root.clone());

    let vdf_path = steam_root.join("steamapps").join("libraryfolders.vdf");
    if let Ok(contents) = fs::read_to_string(&vdf_path) {
        for path in parse_libraryfolders(&contents) {
            paths.insert(path);
        }
    }

    let mut result: Vec<PathBuf> = paths.into_iter().collect();
    result.sort();
    result
}
```

## parser.rs: parse de VDF e manifests

### parse_libraryfolders

- Percorre cada linha do arquivo `libraryfolders.vdf`.
- Extrai tokens entre aspas com `quoted_tokens`.
- Considera chaves `path` e chaves numericas ("0", "1", ...).
- Normaliza caminhos com barras invertidas duplicadas.

```rust
pub(crate) fn parse_libraryfolders(contents: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for line in contents.lines() {
        let tokens = quoted_tokens(line);
        if tokens.len() != 2 {
            continue;
        }

        let key = tokens[0].trim().to_ascii_lowercase();
        let value = tokens[1].trim();

        if key == "path" || key.chars().all(|c| c.is_ascii_digit()) {
            if looks_like_path(value) {
                paths.push(PathBuf::from(normalize_vdf_path(value)));
            }
        }
    }

    paths
}
```

### parse_appmanifest

- Le chaves essenciais do `appmanifest_*.acf`:
  - `appid`
  - `name`
  - `installdir`
  - `sizeondisk`
- Retorna `None` se dados obrigatorios faltarem.

```rust
pub(crate) fn parse_appmanifest(contents: &str) -> Option<AppManifest> {
    let mut app_id = None;
    let mut name = None;
    let mut install_dir = None;
    let mut size_bytes = None;

    for line in contents.lines() {
        let tokens = quoted_tokens(line);
        if tokens.len() != 2 {
            continue;
        }

        let key = tokens[0].trim().to_ascii_lowercase();
        let value = tokens[1].trim();

        match key.as_str() {
            "appid" => app_id = Some(value.to_string()),
            "name" => name = Some(value.to_string()),
            "installdir" => install_dir = Some(value.to_string()),
            "sizeondisk" => size_bytes = value.parse::<u64>().ok(),
            _ => {}
        }
    }

    Some(AppManifest {
        app_id: app_id?,
        name: name?,
        install_dir: install_dir?,
        size_bytes: size_bytes.unwrap_or(0),
    })
}
```

### quoted_tokens

- Varre uma linha e retorna apenas o texto entre aspas.
- Evita interpretar estrutura hierarquica do VDF. Para os dados que precisamos, isso e suficiente.

```rust
fn quoted_tokens(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        if in_quotes {
            if ch == '"' {
                tokens.push(current);
                current = String::new();
                in_quotes = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' {
            in_quotes = true;
        }
    }

    tokens
}
```

### looks_like_path e normalize_vdf_path

- `looks_like_path` filtra tokens que realmente parecem caminho.
- `normalize_vdf_path` remove barras duplicadas.

```rust
fn looks_like_path(value: &str) -> bool {
    value.contains(":\\") || value.contains(":/") || value.starts_with("\\\\")
}

fn normalize_vdf_path(value: &str) -> String {
    value.replace("\\\\", "\\")
}
```

## scanner.rs: varredura de jogos

### is_appmanifest

- Filtra arquivos que seguem o padrao `appmanifest_*.acf`.

```rust
fn is_appmanifest(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    file_name.starts_with("appmanifest_") && file_name.ends_with(".acf")
}
```

### scan_steam_games

Fluxo principal:

1. Carrega as bibliotecas Steam.
2. Lista `steamapps` de cada biblioteca.
3. Abre e parseia os manifests.
4. Monta objetos `Game` com dados basicos.

```rust
pub fn scan_steam_games() -> Vec<Game> {
    let mut games = Vec::new();
    let mut seen = HashSet::new();

    for library_root in get_library_folders() {
        let steamapps = library_root.join("steamapps");
        let Ok(entries) = fs::read_dir(&steamapps) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !is_appmanifest(&path) {
                continue;
            }

            let Ok(contents) = fs::read_to_string(&path) else {
                continue;
            };

            let Some(manifest) = parse_appmanifest(&contents) else {
                continue;
            };

            if seen.contains(&manifest.app_id) {
                continue;
            }

            let install_dir = steamapps
                .join("common")
                .join(&manifest.install_dir)
                .to_string_lossy()
                .into_owned();

            games.push(Game {
                id: manifest.app_id.clone(),
                name: manifest.name,
                platform: Platform::Steam,
                last_backup: None,
                install_dir,
                save_path: None,
                status: GameStatus::Pending,
                size_bytes: manifest.size_bytes,
                checksum: None,
                is_ignored: false,
            });

            seen.insert(manifest.app_id);
        }
    }

    games
}
```

## Estrutura dos arquivos do Steam

### Exemplo de libraryfolders.vdf

```vdf
"libraryfolders"
{
    "0"
    {
        "path" "D:\\SteamLibrary"
        "label" ""
        "contentid" "1234567890123456789"
        "totalsize" "0"
        "update_clean_bytes_tally" "0"
        "time_last_update" "0"
    }
}
```

### Exemplo de appmanifest_*.acf

```vdf
"AppState"
{
    "appid" "570"
    "name" "Dota 2"
    "installdir" "dota 2 beta"
    "sizeondisk" "4567890123"
}
```

## Mapeamento para o modelo Game

O `Game` produzido pela varredura Steam eh o ponto de partida. Campos importantes:

- `id`: o `appid` do manifest.
- `name`: nome do jogo.
- `install_dir`: caminho completo para `steamapps/common/<installdir>`.
- `save_path`: `None` ate haver deteccao via Ludusavi ou outro mapa.
- `status`: inicializado como `Pending`.
- `size_bytes`: `sizeondisk` do manifest.

Exemplo do objeto que sobe para o frontend:

```json
{
  "id": "570",
  "name": "Dota 2",
  "platform": "Steam",
  "last_backup": null,
  "install_dir": "D:\\SteamLibrary\\steamapps\\common\\dota 2 beta",
  "save_path": null,
  "status": "Pending",
  "size_bytes": 4567890123,
  "checksum": null,
  "is_ignored": false
}
```

## Tolerancia a falhas

O servico eh tolerante a dados parciais:

- Se o registro nao tiver o caminho do Steam, retorna lista vazia.
- Se uma biblioteca falhar, o scan continua nas demais.
- Se um manifest estiver corrompido, ele eh ignorado.
- Duplicatas sao filtradas usando `HashSet`.

## Como usar no backend

O comando exposto para o frontend usa o servico diretamente:

```rust
#[tauri::command]
pub fn scan_steam_games() -> Vec<Game> {
    steam::scan_steam_games()
}
```

## Ponto de extensao

- Detalhar `save_path` via Ludusavi (manifesto YAML).
- Cruzar bibliotecas com outros launchers.
- Enriquecer `Game` com tempo de jogo e icones do Steam.
