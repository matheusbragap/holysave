use crate::models::game::Game;
use crate::services::steam;

#[tauri::command]
pub fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_steam_path() -> Option<String> {
	steam::get_steam_path().map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn scan_steam_games() -> Vec<Game> {
	steam::scan_steam_games()
}
