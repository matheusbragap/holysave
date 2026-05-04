/**
 * IPC para o comando Rust `scan_steam_games` (ver `docs/codigo-fonte/ponte-tauri-invoke.md`).
 */
import { invoke } from "@tauri-apps/api/core";
import type { Game } from "$lib/types/game";

export async function loadCachedSteamGames(): Promise<Game[]> {
  return invoke<Game[]>("load_cached_steam_games");
}

export async function scanSteamGames(): Promise<Game[]> {
  return invoke<Game[]>("scan_steam_games");
}
