//! Varredura `steamapps`: lê `appmanifest_*.acf` e monta uma lista de `Game`.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::models::game::{Game, GameStatus, Platform};

use super::parser::parse_appmanifest;
use super::path::get_library_folders;

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

fn is_appmanifest(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    file_name.starts_with("appmanifest_") && file_name.ends_with(".acf")
}
