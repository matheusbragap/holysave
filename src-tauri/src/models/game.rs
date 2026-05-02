use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: String, // app id
    pub name: String,
    pub platform: Platform,

    pub last_backup: Option<SystemTime>,
    pub install_dir: String,       // dir game
    pub save_path: Option<String>, // local save game (pode nao ter sido encontrado)

    pub status: GameStatus,
    pub size_bytes: u64,
    pub checksum: Option<String>, // pode nao ter sido criado ainda
    pub is_ignored: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Platform {
    Steam,
    Gog,
    Epic,
    Manual,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameStatus {
    Synced,
    Pending,
    Uploading,
    Error(String),
}
