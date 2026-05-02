use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

use crate::models::game::{Game, GameStatus, Platform};

pub fn get_steam_path() -> Option<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey("SOFTWARE\\Valve\\Steam")
        .or_else(|_| hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")) // fallback old steam
        .ok()
        .and_then(|key| {
            let path: String = key.get_value("InstallPath").ok()?;
            Some(PathBuf::from(path))
        })
}

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

fn parse_libraryfolders(contents: &str) -> Vec<PathBuf> {
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

fn looks_like_path(value: &str) -> bool {
    value.contains(":\\") || value.contains(":/") || value.starts_with("\\\\")
}

fn normalize_vdf_path(value: &str) -> String {
    value.replace("\\\\", "\\")
}

fn is_appmanifest(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    file_name.starts_with("appmanifest_") && file_name.ends_with(".acf")
}

struct AppManifest {
    app_id: String,
    name: String,
    install_dir: String,
    size_bytes: u64,
}

fn parse_appmanifest(contents: &str) -> Option<AppManifest> {
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