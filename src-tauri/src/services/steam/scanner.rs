//! Varredura `steamapps`: lê `appmanifest_*.acf` e monta uma lista de `Game`.

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread;

use crate::models::game::{Game, GameStatus, Platform};
use crate::services::db;

use super::parser::parse_appmanifest;
use super::path::get_library_folders;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Debug, Serialize)]
struct SteamScanUpdate {
    game: Game,
    valid: bool,
}

pub fn scan_steam_games(app: AppHandle) -> Vec<Game> {
    let games = collect_steam_games();
    sync_games_db(&games);

    let games_for_validation = games.clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        let Ok(mut conn) = db::open_connection() else {
            return;
        };

        for game in games_for_validation {
            let valid = is_probably_game(&mut conn, &game.id);
            if !valid {
                if let Ok(Some(path)) = db::get_image_path(&mut conn, &game.id) {
                    let _ = fs::remove_file(&path);
                }
            }

            let _ = app_handle.emit(
                "steam_game_scan_update",
                SteamScanUpdate {
                    game,
                    valid,
                },
            );
        }
    });

    games
}

fn collect_steam_games() -> Vec<Game> {
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

fn sync_games_db(games: &[Game]) {
    let Ok(mut conn) = db::open_connection() else {
        return;
    };

    let _ = db::upsert_games(&mut conn, games);

    let current_ids: HashSet<String> = games.iter().map(|game| game.id.clone()).collect();
    let Ok(missing_ids) = db::find_missing_game_ids(&mut conn, "Steam", &current_ids) else {
        return;
    };

    for id in missing_ids {
        if let Ok(Some(path)) = db::get_image_path(&mut conn, &id) {
            let _ = fs::remove_file(&path);
        }
        let _ = db::delete_game(&mut conn, &id);
    }
}

fn is_probably_game(conn: &mut rusqlite::Connection, app_id: &str) -> bool {
    if let Ok(Some(cached)) = db::get_game_validation(conn, app_id) {
        return cached;
    }

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
    {
        Ok(client) => client,
        Err(_) => return true,
    };

    let url = format!("https://store.steampowered.com/api/appdetails?appids={app_id}");
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => return true,
    };

    let value: serde_json::Value = match response.json() {
        Ok(value) => value,
        Err(_) => return true,
    };

    let Some(app_entry) = value.get(app_id) else {
        return false;
    };

    let Some(success) = app_entry.get("success").and_then(|value| value.as_bool()) else {
        return false;
    };

    if !success {
        let _ = db::set_game_validation(conn, app_id, false);
        return false;
    }

    let Some(categories) = app_entry
        .get("data")
        .and_then(|data| data.get("categories"))
        .and_then(|categories| categories.as_array())
    else {
        let _ = db::set_game_validation(conn, app_id, false);
        return false;
    };

    let valid = categories.iter().any(|category| {
        category
            .get("id")
            .and_then(|value| value.as_i64())
            .is_some_and(|id| matches!(id, 1 | 2 | 9))
    });

    let _ = db::set_game_validation(conn, app_id, valid);
    valid
}

fn is_appmanifest(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    file_name.starts_with("appmanifest_") && file_name.ends_with(".acf")
}
