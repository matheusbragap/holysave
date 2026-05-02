mod services;
mod models;

use crate::models::game::Game;
use crate::services::steam;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_steam_path() -> Option<String> {
    steam::get_steam_path().map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
fn scan_steam_games() -> Vec<Game> {
    steam::scan_steam_games()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet, get_steam_path, scan_steam_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
