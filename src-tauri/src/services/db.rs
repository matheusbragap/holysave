use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};

use crate::models::game::Game;
use crate::models::game::{GameStatus, Platform};

pub fn app_data_dir() -> Result<PathBuf, String> {
    let base = dirs::data_local_dir().ok_or("data_local_dir not available")?;
    let dir = base.join("HolySave");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn covers_dir() -> Result<PathBuf, String> {
    let dir = app_data_dir()?.join("covers");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn db_path() -> Result<PathBuf, String> {
    Ok(app_data_dir()?.join("holysave.sqlite3"))
}

pub fn open_connection() -> Result<Connection, String> {
    let path = db_path()?;
    let mut conn = Connection::open(path).map_err(|e| e.to_string())?;
    conn.pragma_update(None, "foreign_keys", "ON")
        .map_err(|e| e.to_string())?;
    init_schema(&mut conn)?;
    Ok(conn)
}

fn init_schema(conn: &mut Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            platform TEXT NOT NULL,
            install_dir TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            last_seen INTEGER NOT NULL,
            is_probably_game INTEGER,
            validated_at INTEGER
        );
        CREATE TABLE IF NOT EXISTS game_images (
            game_id TEXT PRIMARY KEY,
            image_path TEXT NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE
        );",
    )
    .map_err(|e| e.to_string())?;

    ensure_games_validation_columns(conn)?;

    Ok(())
}

fn ensure_games_validation_columns(conn: &mut Connection) -> Result<(), String> {
    let mut stmt = conn
        .prepare("PRAGMA table_info(games)")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| e.to_string())?;

    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(row.map_err(|e| e.to_string())?);
    }

    if !columns.contains("is_probably_game") {
        conn.execute("ALTER TABLE games ADD COLUMN is_probably_game INTEGER", [])
            .map_err(|e| e.to_string())?;
    }

    if !columns.contains("validated_at") {
        conn.execute("ALTER TABLE games ADD COLUMN validated_at INTEGER", [])
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn now_epoch_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn upsert_games(conn: &mut Connection, games: &[Game]) -> Result<(), String> {
    let now = now_epoch_seconds();

    for game in games {
        let platform = format!("{:?}", game.platform);
        let size_bytes = game.size_bytes as i64;
        conn.execute(
            "INSERT INTO games (id, name, platform, install_dir, size_bytes, last_seen)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
               name = excluded.name,
               platform = excluded.platform,
               install_dir = excluded.install_dir,
               size_bytes = excluded.size_bytes,
               last_seen = excluded.last_seen",
            params![
                game.id,
                game.name,
                platform,
                game.install_dir,
                size_bytes,
                now
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn get_game_validation(
    conn: &mut Connection,
    game_id: &str,
) -> Result<Option<bool>, String> {
    let cached = conn.query_row(
        "SELECT is_probably_game FROM games WHERE id = ?1",
        params![game_id],
        |row| row.get::<_, Option<i64>>(0),
    )
    .optional()
    .map_err(|e| e.to_string())?;

    Ok(cached.and_then(|value| value.map(|flag| flag != 0)))
}

pub fn set_game_validation(
    conn: &mut Connection,
    game_id: &str,
    is_probably_game: bool,
) -> Result<(), String> {
    let now = now_epoch_seconds();
    let value = if is_probably_game { 1_i64 } else { 0_i64 };

    conn.execute(
        "UPDATE games
         SET is_probably_game = ?2,
             validated_at = ?3
         WHERE id = ?1",
        params![game_id, value, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn find_missing_game_ids(
    conn: &mut Connection,
    platform: &str,
    current_ids: &HashSet<String>,
) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT id FROM games WHERE platform = ?1")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![platform], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?;

    let mut missing = Vec::new();
    for row in rows {
        let id = row.map_err(|e| e.to_string())?;
        if !current_ids.contains(&id) {
            missing.push(id);
        }
    }

    Ok(missing)
}

pub fn delete_game(conn: &mut Connection, game_id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM game_images WHERE game_id = ?1", params![game_id])
        .map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM games WHERE id = ?1", params![game_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_image_path(conn: &mut Connection, game_id: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT image_path FROM game_images WHERE game_id = ?1",
        params![game_id],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| e.to_string())
}

pub fn set_image_path(
    conn: &mut Connection,
    game_id: &str,
    image_path: &str,
) -> Result<(), String> {
    let now = now_epoch_seconds();
    conn.execute(
        "INSERT INTO game_images (game_id, image_path, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(game_id) DO UPDATE SET
           image_path = excluded.image_path,
           updated_at = excluded.updated_at",
        params![game_id, image_path, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn image_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn list_games_by_platform(conn: &mut Connection, platform: &str) -> Result<Vec<Game>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, platform, install_dir, size_bytes
             FROM games
             WHERE platform = ?1 AND is_probably_game = 1
             ORDER BY name COLLATE NOCASE ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![platform], |row| {
            let platform_text: String = row.get(2)?;
            let platform_value = match platform_text.as_str() {
                "Steam" => Platform::Steam,
                "Gog" => Platform::Gog,
                "Epic" => Platform::Epic,
                _ => Platform::Manual,
            };

            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
                platform: platform_value,
                size_bytes: row.get::<_, i64>(4)? as u64,
                last_backup: None,
                install_dir: row.get(3)?,
                save_path: None,
                status: GameStatus::Pending,
                checksum: None,
                is_ignored: false,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut games = Vec::new();
    for row in rows {
        games.push(row.map_err(|e| e.to_string())?);
    }

    Ok(games)
}
