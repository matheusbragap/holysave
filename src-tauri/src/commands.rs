//! Comandos `#[tauri::command]` → frontend `invoke`. Tabela em `docs/codigo-fonte/ponte-tauri-invoke.md`.

use crate::models::game::Game;
use crate::services::{filesystem, steam, steamgriddb};

use std::collections::HashMap;

// ============================================================================
// STEAM COMMANDS
// ============================================================================

/// Retorna o caminho de instalação do Steam no registro do Windows
#[tauri::command]
pub fn get_steam_path() -> Option<String> {
    steam::get_steam_path().map(|path| path.to_string_lossy().into_owned())
}

/// Varre todas as bibliotecas Steam e retorna lista de jogos instalados
#[tauri::command]
pub fn scan_steam_games() -> Vec<Game> {
    steam::scan_steam_games()
}

// ============================================================================
// FILESYSTEM COMMANDS
// ============================================================================

/// Pasta padrão para backups: `Documents/HolySave` do utilizador atual
#[tauri::command]
pub fn get_default_backup_directory() -> Result<String, String> {
    dirs::document_dir()
        .map(|p| p.join("HolySave").to_string_lossy().into_owned())
        .ok_or_else(|| "Não foi possível localizar a pasta Documentos.".to_owned())
}

/// Abre uma pasta no gerenciador padrão do sistema (Explorer no Windows)
#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    filesystem::open_folder(&path)
}

/// Deleta o save local de um jogo recursivamente
#[tauri::command]
pub fn delete_save(game: Game) -> Result<(), String> {
    let save_path = game
        .save_path
        .ok_or("Este jogo não tem caminho de save detectado")?;
    
    filesystem::delete_path(&save_path)
}

// ============================================================================
// STEAMGRIDDB COMMANDS
// ============================================================================

/// Recupera capas (grid vertical) via SteamGridDB
#[tauri::command]
pub async fn get_steamgriddb_covers(
    app_ids: Vec<String>,
) -> Result<HashMap<String, String>, String> {
    steamgriddb::get_grid_covers_by_steam_ids(&app_ids).await
}
