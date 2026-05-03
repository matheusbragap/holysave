//! Cliente HTTP SteamGridDB (capas verticais). Requer `STEAMGRIDDB_API_KEY`.

use std::collections::HashMap;

use reqwest::Client;
use serde_json::Value;

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

        best.insert(app_id.clone(), url.to_string());
    }

    Ok(best)
}
