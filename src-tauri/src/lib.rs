//! Shell Tauri: regista comandos IPC e plugins. Índice: `docs/codigo-fonte/backend/`.

mod services;
mod models;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_steam_path,
            commands::scan_steam_games,
            commands::load_cached_steam_games,
            commands::get_steamgriddb_covers,
            commands::get_default_backup_directory,
            commands::open_folder,
            commands::delete_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
