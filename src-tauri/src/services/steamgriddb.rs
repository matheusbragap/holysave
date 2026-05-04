//! Cliente HTTP SteamGridDB (capas verticais). Requer `STEAMGRIDDB_API_KEY`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use reqwest::Client;
use serde_json::Value;

use crate::services::db;

const BASE_URL: &str = "https://www.steamgriddb.com/api/v2";
const DIMENSIONS: &str = "600x900";
const TYPES: &str = "static";
const LIMIT: &str = "1";

pub async fn get_grid_covers_by_steam_ids(
    app_ids: &[String],
) -> Result<HashMap<String, String>, String> {
    if app_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let api_key = std::env::var("STEAMGRIDDB_API_KEY")
        .map_err(|_| "STEAMGRIDDB_API_KEY nao configurada".to_string())?;

    let client = Client::new();
    let mut best: HashMap<String, String> = HashMap::new();

    for app_id in app_ids {
        if let Ok(mut conn) = db::open_connection() {
            if let Ok(Some(path)) = db::get_image_path(&mut conn, app_id) {
                if db::image_exists(&path) {
                    best.insert(app_id.clone(), path);
                    continue;
                }
            }
        }

        let url = format!(
            "{}/grids/steam/{}?dimensions={}&types={}&nsfw=false&humor=false&epilepsy=false&limit={}",
            BASE_URL, app_id, DIMENSIONS, TYPES, LIMIT
        );

        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| format!("Erro ao chamar SteamGridDB: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            continue;
        }

        let payload: Value = response
            .json()
            .await
            .map_err(|e| format!("Erro ao ler resposta da SteamGridDB: {}", e))?;

        let data = payload
            .get("data")
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default();

        let Some(first) = data.first() else {
            continue;
        };

        let url = first.get("url").and_then(|value| value.as_str()).unwrap_or("");
        if url.is_empty() {
            continue;
        }

        let file_path = cover_path(app_id, url)?;
        let bytes = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Erro ao baixar capa: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Erro ao ler bytes da capa: {}", e))?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        fs::write(&file_path, &bytes).map_err(|e| e.to_string())?;

        let path_string = file_path.to_string_lossy().into_owned();
        if let Ok(mut conn) = db::open_connection() {
            let _ = db::set_image_path(&mut conn, app_id, &path_string);
        }

        best.insert(app_id.clone(), path_string);
    }

    Ok(best)
}

fn cover_path(app_id: &str, url: &str) -> Result<PathBuf, String> {
    let ext = cover_extension(url);
    let dir = db::covers_dir()?;
    Ok(dir.join(format!("{}.{}", app_id, ext)))
}

fn cover_extension(url: &str) -> &str {
    let lower = url.to_lowercase();
    if lower.ends_with(".png") {
        "png"
    } else if lower.ends_with(".jpeg") || lower.ends_with(".jpg") {
        "jpg"
    } else {
        "jpg"
    }
}
